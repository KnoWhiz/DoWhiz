use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::thread;

use tracing::{error, warn};

use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::ingestion::IngestionEnvelope;
use crate::ingestion_queue::IngestionQueue;
use crate::message_router::MessageRouter;
use crate::slack_store::SlackStore;
use crate::user_store::UserStore;

use super::config::ServiceConfig;
use super::email::{process_inbound_payload, PostmarkInbound};
use super::inbound::{
    process_bluebubbles_event, process_discord_inbound_message, process_google_docs_message,
    process_slack_event, process_sms_message, process_telegram_event, try_quick_response_bluebubbles,
    try_quick_response_discord, try_quick_response_slack, try_quick_response_telegram,
};
use super::BoxError;

pub(super) fn spawn_ingestion_consumer(
    config: std::sync::Arc<ServiceConfig>,
    queue: std::sync::Arc<IngestionQueue>,
    user_store: std::sync::Arc<UserStore>,
    index_store: std::sync::Arc<IndexStore>,
    slack_store: std::sync::Arc<SlackStore>,
    message_router: std::sync::Arc<MessageRouter>,
) -> Result<(), BoxError> {
    let poll_interval = config.ingestion_poll_interval;
    let employee_id = config.employee_id.clone();
    let dedupe_path = config.ingestion_dedupe_path.clone();
    let runtime = tokio::runtime::Handle::current();

    thread::spawn(move || {
        let mut dedupe_store = match ProcessedMessageStore::load(&dedupe_path) {
            Ok(store) => store,
            Err(err) => {
                error!("ingestion dedupe store load failed: {}", err);
                return;
            }
        };

        loop {
            match queue.claim_next(&employee_id) {
                Ok(Some(item)) => {
                    let is_new = match dedupe_store.mark_if_new(&[item.envelope.dedupe_key.clone()])
                    {
                        Ok(value) => value,
                        Err(err) => {
                            error!("ingestion dedupe store error: {}", err);
                            true
                        }
                    };

                    if !is_new {
                        if let Err(err) = queue.mark_done(&item.id) {
                            warn!("failed to mark duplicate envelope done: {}", err);
                        }
                        continue;
                    }

                    match process_ingestion_envelope(
                        &config,
                        &user_store,
                        &index_store,
                        &slack_store,
                        &message_router,
                        &runtime,
                        &item.envelope,
                    ) {
                        Ok(_) => {
                            if let Err(err) = queue.mark_done(&item.id) {
                                warn!("failed to mark envelope done: {}", err);
                            }
                        }
                        Err(err) => {
                            if let Err(mark_err) =
                                queue.mark_failed(&item.id, &err.to_string())
                            {
                                warn!("failed to mark envelope failed: {}", mark_err);
                            }
                        }
                    }
                }
                Ok(None) => {
                    thread::sleep(poll_interval);
                }
                Err(err) => {
                    warn!("ingestion queue claim error: {}", err);
                    thread::sleep(poll_interval);
                }
            }
        }
    });

    Ok(())
}

fn process_ingestion_envelope(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    slack_store: &SlackStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    envelope: &IngestionEnvelope,
) -> Result<(), BoxError> {
    match envelope.channel {
        Channel::Email => {
            let raw_payload = envelope.raw_payload_bytes();
            let payload: PostmarkInbound = serde_json::from_slice(&raw_payload)?;
            process_inbound_payload(config, user_store, index_store, &payload, &raw_payload)
        }
        Channel::Slack => {
            let message = envelope.to_inbound_message();
            if try_quick_response_slack(
                config,
                user_store,
                slack_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            if raw_payload.is_empty() {
                return Err("missing slack raw payload".into());
            }
            process_slack_event(config, user_store, index_store, slack_store, &raw_payload)
        }
        Channel::BlueBubbles => {
            let message = envelope.to_inbound_message();
            if try_quick_response_bluebubbles(
                config,
                user_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            if raw_payload.is_empty() {
                return Err("missing bluebubbles raw payload".into());
            }
            process_bluebubbles_event(config, user_store, index_store, &raw_payload)
        }
        Channel::Discord => {
            let message = envelope.to_inbound_message();
            if try_quick_response_discord(
                config,
                user_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_discord_inbound_message(config, index_store, &message, &raw_payload)
        }
        Channel::Sms => {
            let message = envelope.to_inbound_message();
            let raw_payload = envelope.raw_payload_bytes();
            process_sms_message(config, user_store, index_store, &message, &raw_payload)
        }
        Channel::GoogleDocs => {
            let message = envelope.to_inbound_message();
            let raw_payload = envelope.raw_payload_bytes();
            process_google_docs_message(config, user_store, index_store, &message, &raw_payload)
        }
        Channel::Telegram => {
            let message = envelope.to_inbound_message();
            if try_quick_response_telegram(
                config,
                user_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_telegram_event(config, user_store, index_store, &message, &raw_payload)
        }
    }
}

struct ProcessedMessageStore {
    path: PathBuf,
    seen: HashSet<String>,
}

impl ProcessedMessageStore {
    fn load(path: &Path) -> Result<Self, std::io::Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut seen = HashSet::new();
        if path.exists() {
            for raw in std::fs::read_to_string(path)?.lines() {
                let line = raw.trim();
                if !line.is_empty() {
                    seen.insert(line.to_string());
                }
            }
        }
        Ok(Self {
            path: path.to_path_buf(),
            seen,
        })
    }

    fn mark_if_new(&mut self, ids: &[String]) -> Result<bool, std::io::Error> {
        let candidates: Vec<_> = ids
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .collect();
        if candidates.is_empty() {
            return Ok(true);
        }

        if candidates.iter().any(|value| self.seen.contains(*value)) {
            return Ok(false);
        }

        let mut handle = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        for value in candidates {
            self.seen.insert(value.to_string());
            use std::io::Write;
            writeln!(handle, "{}", value)?;
        }
        Ok(true)
    }
}
