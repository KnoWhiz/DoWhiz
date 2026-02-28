use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use scheduler_module::google_auth::GoogleAuthConfig;
use scheduler_module::google_drive_changes::GoogleDriveChangesConfig;
use scheduler_module::google_workspace_poller::{
    GoogleWorkspacePoller, GoogleWorkspacePollerConfig, WorkspaceFileType,
};
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use super::handlers::build_envelope_blocking;
use super::routes::resolve_route;
use super::state::GatewayState;

pub(super) fn spawn_google_workspace_poller(state: Arc<GatewayState>) {
    let docs_enabled = env::var("GOOGLE_DOCS_ENABLED")
        .ok()
        .map(|value| value.to_lowercase() == "true" || value == "1")
        .unwrap_or(false);

    let sheets_enabled = env::var("GOOGLE_SHEETS_ENABLED")
        .ok()
        .map(|value| value.to_lowercase() == "true" || value == "1")
        .unwrap_or(false);

    let slides_enabled = env::var("GOOGLE_SLIDES_ENABLED")
        .ok()
        .map(|value| value.to_lowercase() == "true" || value == "1")
        .unwrap_or(false);

    if !docs_enabled && !sheets_enabled && !slides_enabled {
        return;
    }

    let google_auth_config = GoogleAuthConfig::from_env();
    if !google_auth_config.is_valid() {
        warn!("Google Workspace enabled but OAuth credentials not configured");
        return;
    }

    let poller_config = GoogleWorkspacePollerConfig::from_env();
    let poll_interval = poller_config.poll_interval_secs;
    let push_enabled = GoogleDriveChangesConfig::from_env().is_valid();

    info!(
        "Starting Google Workspace poller: docs={}, sheets={}, slides={}, interval={}s, push_notifications={}",
        docs_enabled, sheets_enabled, slides_enabled, poll_interval, push_enabled
    );

    // Spawn separate threads for Docs, Sheets and Slides to poll in parallel
    // This reduces latency significantly when multiple types are enabled

    if docs_enabled {
        let state_docs = Arc::clone(&state);
        let config_docs = poller_config.clone();
        let change_rx = state
            .drive_change_notifier
            .as_ref()
            .map(|tx| tx.subscribe());
        std::thread::spawn(move || {
            run_workspace_poller(
                state_docs,
                config_docs,
                WorkspaceFileType::Docs,
                poll_interval,
                change_rx,
            );
        });
    }

    if sheets_enabled {
        let state_sheets = Arc::clone(&state);
        let config_sheets = poller_config.clone();
        let change_rx = state
            .drive_change_notifier
            .as_ref()
            .map(|tx| tx.subscribe());
        std::thread::spawn(move || {
            run_workspace_poller(
                state_sheets,
                config_sheets,
                WorkspaceFileType::Sheets,
                poll_interval,
                change_rx,
            );
        });
    }

    if slides_enabled {
        let state_slides = Arc::clone(&state);
        let config_slides = poller_config.clone();
        // Slides does not support push notifications (Google API limitation)
        // so we don't pass a change receiver - it will use polling only
        std::thread::spawn(move || {
            run_workspace_poller(
                state_slides,
                config_slides,
                WorkspaceFileType::Slides,
                poll_interval,
                None, // No push notifications for Slides
            );
        });
    }
}

/// Main loop for a workspace poller thread.
/// Polls at regular intervals, but also responds immediately to push notifications.
fn run_workspace_poller(
    state: Arc<GatewayState>,
    config: GoogleWorkspacePollerConfig,
    file_type: WorkspaceFileType,
    poll_interval: u64,
    mut change_rx: Option<broadcast::Receiver<String>>,
) {
    let poller = match GoogleWorkspacePoller::new(config) {
        Ok(p) => p,
        Err(err) => {
            error!(
                "Failed to create {} poller: {}",
                file_type.display_name(),
                err
            );
            return;
        }
    };

    info!("{} poller thread started", file_type.display_name());

    // Track which files we're monitoring (for push notifications)
    let mut monitored_files: HashSet<String> = HashSet::new();
    // Track files where watch channel registration failed (to avoid spamming retries)
    let mut failed_watch_files: HashSet<String> = HashSet::new();

    loop {
        // Regular polling
        match poll_workspace_comments(&poller, &state, file_type) {
            Ok(count) => {
                if count > 0 {
                    info!(
                        "{} polling enqueued {} items",
                        file_type.display_name(),
                        count
                    );
                }
            }
            Err(err) => {
                error!("{} polling error: {}", file_type.display_name(), err);
            }
        }

        // Register watch channels for new files (if push notifications enabled and supported)
        // Note: Google Slides does NOT support files.watch API (returns 403)
        if file_type.supports_push_notifications() {
            if let Some(ref manager) = state.drive_changes_manager {
                if let Ok(files) = poller.list_files(file_type) {
                    for file in files {
                        // Skip files that are already monitored or previously failed
                        if monitored_files.contains(&file.id)
                            || failed_watch_files.contains(&file.id)
                        {
                            continue;
                        }
                        match manager.watch_file(&file.id) {
                            Ok(_) => {
                                info!(
                                    "Registered watch channel for {} file: {} ({})",
                                    file_type.display_name(),
                                    file.name.as_deref().unwrap_or("unknown"),
                                    file.id
                                );
                                monitored_files.insert(file.id.clone());
                            }
                            Err(e) => {
                                // Log once and don't retry until restart
                                warn!(
                                    "Failed to register watch for {} file {} ({}): {} - will not retry",
                                    file_type.display_name(),
                                    file.name.as_deref().unwrap_or("unknown"),
                                    file.id,
                                    e
                                );
                                failed_watch_files.insert(file.id.clone());
                            }
                        }
                    }

                    // Renew expiring channels
                    if let Err(e) = manager.renew_expiring_channels() {
                        warn!("Failed to renew watch channels: {}", e);
                    }
                }
            }
        }

        // Wait for next poll interval, but also listen for push notifications
        if let Some(ref mut rx) = change_rx {
            // Use tokio runtime to handle async receive with timeout
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build();

            if let Ok(rt) = rt {
                let timeout = Duration::from_secs(poll_interval);
                let result = rt.block_on(async { tokio::time::timeout(timeout, rx.recv()).await });

                match result {
                    Ok(Ok(file_id)) => {
                        // Immediate poll triggered by push notification
                        info!(
                            "{} immediate poll triggered for file {}",
                            file_type.display_name(),
                            file_id
                        );
                        if let Err(e) = poll_single_file(&poller, &state, file_type, &file_id) {
                            warn!("Immediate poll for {} failed: {}", file_id, e);
                        }
                    }
                    Ok(Err(_)) => {
                        // Channel closed, fall back to regular polling
                        debug!("Push notification channel closed");
                    }
                    Err(_) => {
                        // Timeout - normal poll interval elapsed
                    }
                }
            } else {
                // Fallback to simple sleep
                std::thread::sleep(Duration::from_secs(poll_interval));
            }
        } else {
            // No push notifications, use simple sleep
            std::thread::sleep(Duration::from_secs(poll_interval));
        }
    }
}

/// Poll a single file immediately (triggered by push notification).
fn poll_single_file(
    poller: &GoogleWorkspacePoller,
    state: &GatewayState,
    file_type: WorkspaceFileType,
    file_id: &str,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let results = poller.poll_single_file(file_id, file_type)?;

    let channel = file_type.channel();
    let mut tasks_created = 0usize;

    for (file, actionable_items) in results {
        let file_name = file.name.as_deref().unwrap_or("Untitled");

        for actionable in actionable_items {
            let message = poller.actionable_to_inbound_message(&file, &actionable, file_type);
            let route_key = file.id.clone();

            let Some(route) = resolve_route(channel.clone(), &route_key, state) else {
                info!(
                    "gateway no route for {} file_id={}",
                    file_type.display_name(),
                    route_key
                );
                continue;
            };

            let external_message_id = Some(actionable.tracking_id.clone());
            let raw_payload = serde_json::to_vec(&actionable).unwrap_or_default();

            let envelope = match build_envelope_blocking(
                route,
                channel.clone(),
                external_message_id,
                &message,
                &raw_payload,
            ) {
                Ok(envelope) => envelope,
                Err(err) => {
                    error!("gateway failed to store raw payload: {}", err);
                    continue;
                }
            };

            match state.queue.enqueue(&envelope) {
                Ok(result) => {
                    if result.inserted {
                        poller.store().mark_processed_id(
                            &file.id,
                            &actionable.tracking_id,
                            file_type,
                        )?;
                        tasks_created += 1;
                        info!(
                            "Created task for {} comment {} on {} ({}) [push notification]",
                            file_type.display_name(),
                            actionable.tracking_id,
                            file_name,
                            file.id
                        );
                    }
                }
                Err(err) => {
                    error!("gateway {} enqueue error: {}", file_type.name(), err);
                }
            }
        }

        poller.store().update_last_checked(&file.id)?;
    }

    Ok(tasks_created)
}

fn poll_workspace_comments(
    poller: &GoogleWorkspacePoller,
    state: &GatewayState,
    file_type: WorkspaceFileType,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let results = match file_type {
        WorkspaceFileType::Docs => poller.poll_docs()?,
        WorkspaceFileType::Sheets => poller.poll_sheets()?,
        WorkspaceFileType::Slides => poller.poll_slides()?,
    };

    let mut tasks_created = 0usize;
    let channel = file_type.channel();

    for (file, actionable_items) in results {
        let file_name = file.name.as_deref().unwrap_or("Untitled");

        for actionable in actionable_items {
            let message = poller.actionable_to_inbound_message(&file, &actionable, file_type);
            let route_key = file.id.clone();

            let Some(route) = resolve_route(channel.clone(), &route_key, state) else {
                info!(
                    "gateway no route for {} file_id={}",
                    file_type.display_name(),
                    route_key
                );
                continue;
            };

            let external_message_id = Some(actionable.tracking_id.clone());
            let raw_payload = serde_json::to_vec(&actionable).unwrap_or_default();

            let envelope = match build_envelope_blocking(
                route,
                channel.clone(),
                external_message_id,
                &message,
                &raw_payload,
            ) {
                Ok(envelope) => envelope,
                Err(err) => {
                    error!("gateway failed to store raw payload: {}", err);
                    continue;
                }
            };

            match state.queue.enqueue(&envelope) {
                Ok(result) => {
                    if result.inserted {
                        poller.store().mark_processed_id(
                            &file.id,
                            &actionable.tracking_id,
                            file_type,
                        )?;
                        tasks_created += 1;
                        info!(
                            "Created task for {} comment {} on {} ({})",
                            file_type.display_name(),
                            actionable.tracking_id,
                            file_name,
                            file.id
                        );
                    }
                }
                Err(err) => {
                    error!("gateway {} enqueue error: {}", file_type.name(), err);
                }
            }
        }

        poller.store().update_last_checked(&file.id)?;
    }

    Ok(tasks_created)
}
