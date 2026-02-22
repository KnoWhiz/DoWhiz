use std::env;
use std::sync::Arc;
use std::time::Duration;

use scheduler_module::google_auth::GoogleAuthConfig;
use scheduler_module::google_workspace_poller::{
    GoogleWorkspacePoller, GoogleWorkspacePollerConfig, WorkspaceFileType,
};
use tracing::{error, info, warn};

use super::handlers::build_envelope_blocking;
use super::routes::resolve_route;
use super::state::GatewayState;

pub(super) fn spawn_google_workspace_poller(state: Arc<GatewayState>) {
    let sheets_enabled = env::var("GOOGLE_SHEETS_ENABLED")
        .ok()
        .map(|value| value.to_lowercase() == "true" || value == "1")
        .unwrap_or(false);

    let slides_enabled = env::var("GOOGLE_SLIDES_ENABLED")
        .ok()
        .map(|value| value.to_lowercase() == "true" || value == "1")
        .unwrap_or(false);

    if !sheets_enabled && !slides_enabled {
        return;
    }

    let google_auth_config = GoogleAuthConfig::from_env();
    if !google_auth_config.is_valid() {
        warn!("Google Workspace enabled but OAuth credentials not configured");
        return;
    }

    let poller_config = GoogleWorkspacePollerConfig::from_env();
    let poll_interval = poller_config.poll_interval_secs;

    info!(
        "Starting Google Workspace poller: sheets={}, slides={}",
        sheets_enabled, slides_enabled
    );

    std::thread::spawn(move || {
        match GoogleWorkspacePoller::new(poller_config) {
            Ok(poller) => loop {
                // Poll Sheets
                if sheets_enabled {
                    match poll_workspace_comments(&poller, &state, WorkspaceFileType::Sheets) {
                        Ok(count) => {
                            if count > 0 {
                                info!("Google Sheets polling enqueued {} items", count);
                            }
                        }
                        Err(err) => {
                            error!("Google Sheets polling error: {}", err);
                        }
                    }
                }

                // Poll Slides
                if slides_enabled {
                    match poll_workspace_comments(&poller, &state, WorkspaceFileType::Slides) {
                        Ok(count) => {
                            if count > 0 {
                                info!("Google Slides polling enqueued {} items", count);
                            }
                        }
                        Err(err) => {
                            error!("Google Slides polling error: {}", err);
                        }
                    }
                }

                std::thread::sleep(Duration::from_secs(poll_interval));
            },
            Err(err) => {
                error!("Failed to create Google Workspace poller: {}", err);
            }
        }
    });
}

fn poll_workspace_comments(
    poller: &GoogleWorkspacePoller,
    state: &GatewayState,
    file_type: WorkspaceFileType,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let results = match file_type {
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
