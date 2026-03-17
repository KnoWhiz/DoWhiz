use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use serde_json::json;
use tracing::{info, warn};

use crate::account_store::AccountStore;
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
    process_bluebubbles_event, process_discord_inbound_message, process_google_workspace_message,
    process_notion_message, process_slack_event, process_sms_message, process_telegram_event,
    process_wechat_event, process_whatsapp_event, try_quick_response_bluebubbles,
    try_quick_response_discord, try_quick_response_google_workspace, try_quick_response_slack,
    try_quick_response_telegram, try_quick_response_wechat, try_quick_response_whatsapp,
};
use super::BoxError;

pub(super) struct IngestionControl {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl IngestionControl {
    pub(super) fn stop_and_join(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub(super) fn spawn_ingestion_consumer(
    config: std::sync::Arc<ServiceConfig>,
    queue: std::sync::Arc<dyn IngestionQueue>,
    user_store: std::sync::Arc<UserStore>,
    index_store: std::sync::Arc<IndexStore>,
    slack_store: std::sync::Arc<SlackStore>,
    message_router: std::sync::Arc<MessageRouter>,
    account_store: std::sync::Arc<AccountStore>,
) -> Result<IngestionControl, BoxError> {
    let poll_interval = config.ingestion_poll_interval;
    let employee_id = config.employee_id.clone();
    let runtime = tokio::runtime::Handle::current();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();

    let handle = thread::spawn(move || loop {
        if stop_thread.load(Ordering::Relaxed) {
            break;
        }
        match queue.claim_next(&employee_id) {
            Ok(Some(item)) => {
                info!(
                    "ingestion claimed envelope for employee={} channel={:?}",
                    employee_id, item.envelope.channel
                );
                match process_ingestion_envelope(
                    &config,
                    &user_store,
                    &index_store,
                    &slack_store,
                    &message_router,
                    &account_store,
                    &runtime,
                    &item.envelope,
                ) {
                    Ok(_) => {
                        info!(
                            "ingestion processed successfully for employee={}",
                            employee_id
                        );
                        if let Err(err) = queue.mark_done(&item.id) {
                            warn!("failed to mark envelope done: {}", err);
                        }
                    }
                    Err(err) => {
                        warn!(
                            "ingestion processing failed for employee={}: {}",
                            employee_id, err
                        );
                        if let Err(mark_err) = queue.mark_failed(&item.id, &err.to_string()) {
                            warn!("failed to mark envelope failed: {}", mark_err);
                        }
                    }
                }
            }
            Ok(None) => {
                if stop_thread.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(poll_interval);
            }
            Err(err) => {
                if stop_thread.load(Ordering::Relaxed) {
                    break;
                }
                warn!("ingestion queue claim error: {}", err);
                thread::sleep(poll_interval);
            }
        }
    });

    Ok(IngestionControl {
        stop,
        handle: Some(handle),
    })
}

fn process_ingestion_envelope(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    slack_store: &SlackStore,
    message_router: &MessageRouter,
    account_store: &AccountStore,
    runtime: &tokio::runtime::Handle,
    envelope: &IngestionEnvelope,
) -> Result<(), BoxError> {
    match envelope.channel {
        Channel::Email => {
            let sender = envelope.payload.sender.trim();
            if is_blacklisted_email_sender(sender, &config.employee_directory.service_addresses) {
                info!("skipping blacklisted sender: {}", sender);
                return Ok(());
            }
            let (payload, raw_payload) = resolve_email_payload(envelope)?;
            let subject = payload.subject.as_deref().unwrap_or("");
            if is_human_approval_gate_subject(subject) {
                info!(
                    "skipping human approval gate reply from ingestion workflow: subject={}",
                    subject
                );
                return Ok(());
            }
            process_inbound_payload(
                config,
                user_store,
                index_store,
                account_store,
                &payload,
                &raw_payload,
                envelope.account_id,
            )
        }
        Channel::Slack => {
            info!("processing slack envelope, trying quick response first");
            let message = envelope.to_inbound_message();
            if try_quick_response_slack(
                config,
                user_store,
                slack_store,
                message_router,
                runtime,
                &message,
            )? {
                info!("slack quick response succeeded");
                return Ok(());
            }
            info!("slack quick response returned false, proceeding to full pipeline");
            let raw_payload = envelope.raw_payload_bytes();
            if raw_payload.is_empty() {
                return Err("missing slack raw payload".into());
            }
            process_slack_event(
                config,
                user_store,
                index_store,
                slack_store,
                account_store,
                &raw_payload,
            )
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
            let raw_payload = envelope.raw_payload_bytes();
            if try_quick_response_discord(
                config,
                user_store,
                message_router,
                runtime,
                &message,
                &raw_payload,
            )? {
                return Ok(());
            }
            process_discord_inbound_message(
                config,
                user_store,
                index_store,
                account_store,
                &message,
                &raw_payload,
            )
        }
        Channel::Sms => {
            let message = envelope.to_inbound_message();
            let raw_payload = envelope.raw_payload_bytes();
            process_sms_message(config, user_store, index_store, &message, &raw_payload)
        }
        Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => {
            let message = envelope.to_inbound_message();
            if try_quick_response_google_workspace(
                config,
                user_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_google_workspace_message(
                config,
                user_store,
                index_store,
                account_store,
                &message,
                &raw_payload,
            )
        }
        Channel::Telegram => {
            let message = envelope.to_inbound_message();
            if try_quick_response_telegram(config, user_store, message_router, runtime, &message)? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_telegram_event(config, user_store, index_store, &message, &raw_payload)
        }
        Channel::WhatsApp => {
            let message = envelope.to_inbound_message();
            if try_quick_response_whatsapp(config, user_store, message_router, runtime, &message)? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_whatsapp_event(config, user_store, index_store, &message, &raw_payload)
        }
Channel::Notion => {
            // Process Notion comments via API
            let message = envelope.to_inbound_message();
            let raw_payload = envelope.raw_payload_bytes();
            process_notion_message(
                config,
                user_store,
                index_store,
                account_store,
                &message,
                &raw_payload,
            )
        }
        Channel::WeChat => {
            let message = envelope.to_inbound_message();
            if try_quick_response_wechat(config, user_store, message_router, runtime, &message)? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_wechat_event(config, user_store, index_store, &message, &raw_payload)
        }
    }
}

fn is_blacklisted_email_sender(
    sender: &str,
    service_addresses: &std::collections::HashSet<String>,
) -> bool {
    super::email::is_blacklisted_sender(sender, service_addresses)
}

fn is_human_approval_gate_subject(subject: &str) -> bool {
    let normalized = subject.trim();
    if normalized.is_empty() {
        return false;
    }

    let lowered = normalized.to_ascii_lowercase();
    if lowered.starts_with("[hag:") {
        return true;
    }
    if let Some(rest) = lowered.strip_prefix("re:") {
        if rest.trim_start().starts_with("[hag:") {
            return true;
        }
    }
    false
}

fn resolve_email_payload(
    envelope: &IngestionEnvelope,
) -> Result<(PostmarkInbound, Vec<u8>), BoxError> {
    let raw_payload = envelope.raw_payload_bytes();
    if !raw_payload.is_empty() {
        let parsed: PostmarkInbound = serde_json::from_slice(&raw_payload)?;
        return Ok((parsed, raw_payload));
    }

    let reply_to = if envelope.payload.reply_to.is_empty() {
        None
    } else {
        Some(envelope.payload.reply_to.join(", "))
    };
    let mut headers = Vec::new();
    if let Some(value) = envelope.payload.metadata.in_reply_to.as_ref() {
        if !value.trim().is_empty() {
            headers.push(json!({ "Name": "In-Reply-To", "Value": value }));
        }
    }
    if let Some(value) = envelope.payload.metadata.references.as_ref() {
        if !value.trim().is_empty() {
            headers.push(json!({ "Name": "References", "Value": value }));
        }
    }
    if let Some(value) = envelope.payload.message_id.as_ref() {
        if !value.trim().is_empty() {
            headers.push(json!({ "Name": "Message-ID", "Value": value }));
        }
    }

    let attachments = envelope
        .payload
        .attachments
        .iter()
        .map(|attachment| {
            json!({
                "Name": attachment.name,
                "Content": attachment.content,
                "ContentType": attachment.content_type,
            })
        })
        .collect::<Vec<_>>();

    let synthetic = json!({
        "From": envelope.payload.sender,
        "To": envelope.payload.recipient,
        "ReplyTo": reply_to,
        "Subject": envelope.payload.subject,
        "TextBody": envelope.payload.text_body,
        "HtmlBody": envelope.payload.html_body,
        "MessageID": envelope.payload.message_id,
        "Headers": headers,
        "Attachments": attachments,
    });
    let synthetic_raw = serde_json::to_vec(&synthetic)?;
    let payload: PostmarkInbound = serde_json::from_slice(&synthetic_raw)?;
    Ok((payload, synthetic_raw))
}

#[cfg(test)]
mod tests {
    use super::{
        is_blacklisted_email_sender, is_human_approval_gate_subject, resolve_email_payload,
    };
    use crate::channel::{Attachment, Channel, ChannelMetadata};
    use crate::ingestion::{IngestionEnvelope, IngestionPayload};
    use chrono::Utc;
    use std::collections::HashSet;
    use uuid::Uuid;

    #[test]
    fn resolve_email_payload_builds_fallback_from_ingestion_payload() {
        let envelope = IngestionEnvelope {
            envelope_id: Uuid::new_v4(),
            received_at: Utc::now(),
            tenant_id: None,
            employee_id: "little_bear".to_string(),
            channel: Channel::Email,
            external_message_id: Some("msg-1".to_string()),
            dedupe_key: "dedupe-1".to_string(),
            payload: IngestionPayload {
                sender: "tester@example.com".to_string(),
                sender_name: Some("Tester".to_string()),
                recipient: "oliver@dowhiz.com".to_string(),
                subject: Some("hello".to_string()),
                text_body: Some("plain".to_string()),
                html_body: Some("<p>html</p>".to_string()),
                thread_id: "thread-1".to_string(),
                message_id: Some("<message-1@example.com>".to_string()),
                attachments: vec![Attachment {
                    name: "a.txt".to_string(),
                    content_type: "text/plain".to_string(),
                    content: "YQ==".to_string(),
                }],
                reply_to: vec!["reply@example.com".to_string()],
                metadata: ChannelMetadata {
                    in_reply_to: Some("<in-reply-to@example.com>".to_string()),
                    references: Some("<ref@example.com>".to_string()),
                    ..ChannelMetadata::default()
                },
            },
            raw_payload_ref: None,
            account_id: None,
        };

        let (payload, raw) =
            resolve_email_payload(&envelope).expect("fallback payload should be built");

        assert!(!raw.is_empty());
        assert_eq!(payload.from.as_deref(), Some("tester@example.com"));
        assert_eq!(payload.reply_to.as_deref(), Some("reply@example.com"));
        assert_eq!(payload.subject.as_deref(), Some("hello"));
        assert_eq!(payload.text_body.as_deref(), Some("plain"));
        assert_eq!(
            payload.message_id.as_deref(),
            Some("<message-1@example.com>")
        );
        assert_eq!(payload.attachments.as_ref().map(|v| v.len()), Some(1));
    }

    #[test]
    fn blacklisted_email_sender_detects_service_address() {
        let mut service_addresses = HashSet::new();
        service_addresses.insert("dowhiz@deep-tutor.com".to_string());
        assert!(is_blacklisted_email_sender(
            "DoWhiz <dowhiz@deep-tutor.com>",
            &service_addresses
        ));
    }

    #[test]
    fn blacklisted_email_sender_allows_external_sender() {
        let mut service_addresses = HashSet::new();
        service_addresses.insert("dowhiz@deep-tutor.com".to_string());
        assert!(!is_blacklisted_email_sender(
            "user@example.com",
            &service_addresses
        ));
    }

    #[test]
    fn human_approval_gate_subject_detection_matches_hag_threads() {
        assert!(is_human_approval_gate_subject(
            "[HAG:49d7368d-95a6-4c6c-91cc-8c30a4583c35] 2FA approval needed"
        ));
        assert!(is_human_approval_gate_subject(
            "Re: [HAG:49d7368d-95a6-4c6c-91cc-8c30a4583c35] 2FA approval needed"
        ));
        assert!(is_human_approval_gate_subject(
            "re:    [hag:49d7368d-95a6-4c6c-91cc-8c30a4583c35] 2fa approval needed"
        ));
        assert!(!is_human_approval_gate_subject("Re: Project update"));
        assert!(!is_human_approval_gate_subject(""));
    }

    #[test]
    fn ingestion_envelope_serializes_with_account_id() {
        let account_id = Uuid::new_v4();
        let envelope = IngestionEnvelope {
            envelope_id: Uuid::new_v4(),
            received_at: Utc::now(),
            tenant_id: Some("test".to_string()),
            employee_id: "little_bear".to_string(),
            channel: Channel::Email,
            external_message_id: None,
            dedupe_key: "dedupe".to_string(),
            payload: IngestionPayload {
                sender: "sender@example.com".to_string(),
                sender_name: None,
                recipient: "oliver@dowhiz.com".to_string(),
                subject: None,
                text_body: Some("test".to_string()),
                html_body: None,
                thread_id: "thread".to_string(),
                message_id: None,
                attachments: vec![],
                reply_to: vec![],
                metadata: ChannelMetadata::default(),
            },
            raw_payload_ref: None,
            account_id: Some(account_id),
        };

        let json = serde_json::to_string(&envelope).expect("serialize");
        assert!(json.contains(&account_id.to_string()));

        let deserialized: IngestionEnvelope = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deserialized.account_id, Some(account_id));
    }

    #[test]
    fn ingestion_envelope_deserializes_without_account_id() {
        let json = r#"{
            "envelope_id": "00000000-0000-0000-0000-000000000001",
            "received_at": "2026-03-17T00:00:00Z",
            "tenant_id": "test",
            "employee_id": "little_bear",
            "channel": "email",
            "external_message_id": null,
            "dedupe_key": "dedupe",
            "payload": {
                "sender": "sender@example.com",
                "recipient": "oliver@dowhiz.com",
                "thread_id": "thread"
            },
            "raw_payload_ref": null
        }"#;

        let envelope: IngestionEnvelope = serde_json::from_str(json).expect("deserialize");
        assert_eq!(envelope.account_id, None);
    }
}
