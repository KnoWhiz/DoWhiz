use std::env;
use std::sync::Arc;
use std::time::Duration;

use scheduler_module::channel::Channel;
use scheduler_module::google_auth::GoogleAuthConfig;
use scheduler_module::google_docs_poller::GoogleDocsPollerConfig;
use tracing::{error, info, warn};

use super::handlers::build_envelope_blocking;
use super::routes::resolve_route;
use super::state::GatewayState;

pub(super) fn spawn_google_docs_poller(state: Arc<GatewayState>) {
    let enabled = env::var("GOOGLE_DOCS_ENABLED")
        .ok()
        .map(|value| value.to_lowercase() == "true")
        .unwrap_or(false);
    if !enabled {
        return;
    }

    let google_auth_config = GoogleAuthConfig::from_env();
    if !google_auth_config.is_valid() {
        warn!("Google Docs enabled but OAuth credentials not configured");
        return;
    }

    let poller_config = GoogleDocsPollerConfig::from_env();
    let poll_interval = poller_config.poll_interval_secs;

    std::thread::spawn(move || {
        match scheduler_module::google_docs_poller::GoogleDocsPoller::new(poller_config) {
            Ok(poller) => loop {
                match poll_google_docs_comments(&poller, &state) {
                    Ok(count) => {
                        if count > 0 {
                            info!("Google Docs polling enqueued {} items", count);
                        }
                    }
                    Err(err) => {
                        error!("Google Docs polling error: {}", err);
                    }
                }
                std::thread::sleep(Duration::from_secs(poll_interval));
            },
            Err(err) => {
                error!("Failed to create Google Docs poller: {}", err);
            }
        }
    });
}

fn poll_google_docs_comments(
    poller: &scheduler_module::google_docs_poller::GoogleDocsPoller,
    state: &GatewayState,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    use scheduler_module::adapters::google_docs::GoogleDocsInboundAdapter;

    let adapter = GoogleDocsInboundAdapter::new(
        poller.auth().clone(),
        poller.config().employee_emails.clone(),
    );

    let documents = adapter.list_shared_documents()?;
    let mut tasks_created = 0usize;

    for doc in documents {
        let doc_name = doc.name.as_deref().unwrap_or("Untitled");
        let owner_email = doc
            .owners
            .as_ref()
            .and_then(|owners| owners.first())
            .and_then(|o| o.email_address.as_deref());

        poller
            .store()
            .register_document(&doc.id, doc.name.as_deref(), owner_email)?;

        let comments = match adapter.list_comments(&doc.id) {
            Ok(c) => c,
            Err(err) => {
                warn!("Failed to list comments for '{}': {}", doc_name, err);
                continue;
            }
        };

        let processed = poller.store().get_processed_ids(&doc.id)?;
        let actionable_items = adapter.filter_actionable_comments(&comments, &processed);

        for actionable in actionable_items {
            let message = adapter.actionable_to_inbound_message(&doc.id, doc_name, &actionable);
            let route_key = doc.id.clone();
            let Some(route) = resolve_route(Channel::GoogleDocs, &route_key, state) else {
                info!("gateway no route for google docs doc_id={}", route_key);
                continue;
            };

            let external_message_id = Some(actionable.tracking_id.clone());
            let raw_payload = serde_json::to_vec(&actionable).unwrap_or_default();
            let envelope = match build_envelope_blocking(
                route,
                Channel::GoogleDocs,
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
                        poller
                            .store()
                            .mark_processed_id(&doc.id, &actionable.tracking_id)?;
                        tasks_created += 1;
                    }
                }
                Err(err) => {
                    error!("gateway gdocs enqueue error: {}", err);
                }
            }
        }

        poller.store().update_last_checked(&doc.id)?;
    }

    Ok(tasks_created)
}
