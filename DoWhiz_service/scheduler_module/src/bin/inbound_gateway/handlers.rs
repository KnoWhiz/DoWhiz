use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use scheduler_module::adapters::bluebubbles::BlueBubblesInboundAdapter;
use scheduler_module::adapters::postmark::PostmarkInboundPayload;
use scheduler_module::adapters::slack::{
    is_url_verification, SlackChallengeResponse, SlackEventWrapper, SlackInboundAdapter,
};
use scheduler_module::adapters::telegram::TelegramInboundAdapter;
use scheduler_module::adapters::whatsapp::WhatsAppInboundAdapter;
use scheduler_module::channel::{Channel, ChannelMetadata, InboundAdapter, InboundMessage};
use scheduler_module::ingestion::{IngestionEnvelope, IngestionPayload};
use scheduler_module::ingestion_queue::IngestionQueue;
use scheduler_module::raw_payload_store::{self, RawPayloadStoreError};
use scheduler_module::user_store::extract_emails;

use super::routes::{build_dedupe_key, normalize_email, normalize_phone_number, resolve_route};
use super::state::{find_service_address, GatewayState, RouteDecision};
use super::verify::{
    verify_bluebubbles, verify_postmark, verify_slack, verify_twilio, verify_whatsapp_subscription,
};

pub(super) async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub(super) async fn ingest_postmark(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    if let Err(reason) = verify_postmark(&headers) {
        return (StatusCode::UNAUTHORIZED, Json(json!({"status": reason})));
    }

    let payload: PostmarkInboundPayload = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_json"}))),
    };

    if payload_contains_no_reply_marker(&payload) {
        info!(
            "gateway ignoring no-reply postmark inbound from={}",
            payload.from.as_deref().unwrap_or("")
        );
        return (StatusCode::OK, Json(json!({"status": "ignored_no_reply"})));
    }

    let address = find_service_address(&payload, &state.employee_directory.service_addresses);
    let Some(address) = address else {
        info!("gateway no service address found in postmark payload");
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let route_key = normalize_email(&address);
    let Some(route) = resolve_route(Channel::Email, &route_key, &state) else {
        info!("gateway no route for email address={}", route_key);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let adapter = scheduler_module::adapters::postmark::PostmarkInboundAdapter::new(
        state.employee_directory.service_addresses.clone(),
    );
    let message = match adapter.parse(&body) {
        Ok(message) => message,
        Err(err) => {
            warn!("gateway failed to parse postmark payload: {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"status": "parse_error"})),
            );
        }
    };

    let external_message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let envelope =
        match build_envelope(route, Channel::Email, external_message_id, &message, &body).await {
            Ok(envelope) => envelope,
            Err(err) => {
                error!("gateway failed to store raw payload: {}", err);
                return (
                    StatusCode::BAD_GATEWAY,
                    Json(json!({"status": "payload_store_failed"})),
                );
            }
        };
    enqueue_envelope(state.queue.clone(), envelope).await
}

pub(super) async fn ingest_slack(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    if let Some(verification) = is_url_verification(&body) {
        let response = SlackChallengeResponse {
            challenge: verification.challenge,
        };
        return (StatusCode::OK, Json(json!(response)));
    }

    if let Err(reason) = verify_slack(&headers, &body) {
        return (StatusCode::UNAUTHORIZED, Json(json!({"status": reason})));
    }

    let wrapper: SlackEventWrapper = match serde_json::from_slice(&body) {
        Ok(value) => value,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_json"}))),
    };

    // Extract api_app_id for routing (each Slack app has unique app_id)
    let api_app_id = wrapper.api_app_id.as_deref().unwrap_or("");
    if api_app_id.is_empty() {
        info!("gateway no api_app_id in slack payload");
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    }

    let event_id = wrapper.event_id.clone();

    let Some(route) = resolve_route(Channel::Slack, api_app_id, &state) else {
        info!("gateway no route for slack api_app_id={}", api_app_id);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    info!(
        "gateway slack routing: api_app_id={} -> employee_id={}",
        api_app_id, route.employee_id
    );

    let bot_user_id = resolve_slack_bot_user_id_for_employee(&route.employee_id);
    if !should_enqueue_slack_message(&wrapper, bot_user_id.as_deref()) {
        info!(
            "gateway ignoring slack event for employee={} api_app_id={} (not dm/app_mention/mention)",
            route.employee_id, api_app_id
        );
        return (StatusCode::OK, Json(json!({"status": "ignored"})));
    }

    let mut bot_user_ids = HashSet::new();
    if let Some(id) = bot_user_id {
        bot_user_ids.insert(id);
    }
    let adapter = SlackInboundAdapter::new(bot_user_ids);
    let message = match adapter.parse(&body) {
        Ok(message) => message,
        Err(err) => {
            warn!("gateway failed to parse slack payload: {}", err);
            return (StatusCode::OK, Json(json!({"status": "ignored"})));
        }
    };

    let envelope = match build_envelope(route, Channel::Slack, event_id, &message, &body).await {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "payload_store_failed"})),
            );
        }
    };
    enqueue_envelope(state.queue.clone(), envelope).await
}

fn resolve_slack_bot_user_id_for_employee(employee_id: &str) -> Option<String> {
    let employee_env = employee_id.to_uppercase().replace('-', "_");
    let employee_key = format!("{}_SLACK_BOT_USER_ID", employee_env);

    std::env::var(&employee_key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::env::var("SLACK_BOT_USER_ID")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
}

fn should_enqueue_slack_message(wrapper: &SlackEventWrapper, bot_user_id: Option<&str>) -> bool {
    let Some(event) = wrapper.event.as_ref() else {
        return false;
    };
    if event.subtype.is_some() {
        return false;
    }

    match event.event_type.as_str() {
        "app_mention" => true,
        "message" => {
            if matches!(event.channel_type.as_deref(), Some("im") | Some("mpim")) {
                return true;
            }
            let Some(bot_user_id) = bot_user_id else {
                return false;
            };
            let mention = format!("<@{}>", bot_user_id.trim());
            event
                .text
                .as_deref()
                .map(|text| text.contains(&mention))
                .unwrap_or(false)
        }
        _ => false,
    }
}

pub(super) async fn ingest_bluebubbles(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    if let Err(reason) = verify_bluebubbles(&headers) {
        return (StatusCode::UNAUTHORIZED, Json(json!({"status": reason})));
    }

    let adapter = BlueBubblesInboundAdapter::new();
    let message = match adapter.parse(&body) {
        Ok(message) => message,
        Err(err) => {
            debug!("gateway ignoring bluebubbles event: {}", err);
            return (StatusCode::OK, Json(json!({"status": "ignored"})));
        }
    };

    let chat_guid = message
        .metadata
        .bluebubbles_chat_guid
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    let Some(route) = resolve_route(Channel::BlueBubbles, &chat_guid, &state) else {
        info!("gateway no route for bluebubbles chat_guid={}", chat_guid);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let external_message_id = message.message_id.clone();
    let envelope = match build_envelope(
        route,
        Channel::BlueBubbles,
        external_message_id,
        &message,
        &body,
    )
    .await
    {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "payload_store_failed"})),
            );
        }
    };
    enqueue_envelope(state.queue.clone(), envelope).await
}

pub(super) async fn ingest_sms(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    if let Err(reason) = verify_twilio(&headers, &body) {
        return (StatusCode::UNAUTHORIZED, Json(json!({"status": reason})));
    }

    let params: HashMap<String, String> = match serde_urlencoded::from_bytes(&body) {
        Ok(values) => values,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_form"}))),
    };

    let from = params.get("From").cloned().unwrap_or_default();
    let to = params.get("To").cloned().unwrap_or_default();
    let body_text = params.get("Body").cloned().unwrap_or_default();
    let message_sid = params.get("MessageSid").cloned();

    if from.is_empty() || to.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"status": "missing_fields"})),
        );
    }

    let route_key = normalize_phone_number(&to);
    let Some(route) = resolve_route(Channel::Sms, &route_key, &state) else {
        info!("gateway no route for sms to={}", route_key);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let message = InboundMessage {
        channel: Channel::Sms,
        sender: from.clone(),
        sender_name: None,
        recipient: to.clone(),
        subject: None,
        text_body: Some(body_text),
        html_body: None,
        thread_id: format!("sms:{}:{}", route_key, normalize_phone_number(&from)),
        message_id: message_sid.clone(),
        attachments: Vec::new(),
        reply_to: vec![from.clone()],
        raw_payload: body.to_vec(),
        metadata: ChannelMetadata {
            sms_from: Some(from.clone()),
            sms_to: Some(to.clone()),
            ..Default::default()
        },
    };

    let envelope = match build_envelope(route, Channel::Sms, message_sid, &message, &body).await {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "payload_store_failed"})),
            );
        }
    };
    enqueue_envelope(state.queue.clone(), envelope).await
}

pub(super) async fn ingest_telegram(
    State(state): State<Arc<GatewayState>>,
    body: Bytes,
) -> impl IntoResponse {
    let adapter = TelegramInboundAdapter::new();
    let message = match adapter.parse(&body) {
        Ok(message) => message,
        Err(err) => {
            debug!("gateway ignoring telegram event: {}", err);
            return (StatusCode::OK, Json(json!({"status": "ignored"})));
        }
    };

    let chat_id = message
        .metadata
        .telegram_chat_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let Some(route) = resolve_route(Channel::Telegram, &chat_id, &state) else {
        info!("gateway no route for telegram chat_id={}", chat_id);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let external_message_id = message.message_id.clone();
    let envelope = match build_envelope(
        route,
        Channel::Telegram,
        external_message_id,
        &message,
        &body,
    )
    .await
    {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "payload_store_failed"})),
            );
        }
    };
    enqueue_envelope(state.queue.clone(), envelope).await
}

/// Query parameters for WhatsApp webhook verification
#[derive(Debug, Deserialize)]
pub(super) struct WhatsAppVerifyParams {
    #[serde(rename = "hub.mode")]
    pub hub_mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub hub_verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub hub_challenge: Option<String>,
}

/// Handle WhatsApp webhook verification (GET request)
pub(super) async fn verify_whatsapp_webhook(
    Query(params): Query<WhatsAppVerifyParams>,
) -> impl IntoResponse {
    match verify_whatsapp_subscription(
        params.hub_mode.as_deref(),
        params.hub_verify_token.as_deref(),
        params.hub_challenge.as_deref(),
    ) {
        Ok(challenge) => (StatusCode::OK, challenge),
        Err(reason) => {
            info!("whatsapp webhook verification failed: {}", reason);
            (StatusCode::FORBIDDEN, reason.to_string())
        }
    }
}

/// Handle WhatsApp inbound messages (POST request)
pub(super) async fn ingest_whatsapp(
    State(state): State<Arc<GatewayState>>,
    body: Bytes,
) -> impl IntoResponse {
    let adapter = WhatsAppInboundAdapter::new();
    let message = match adapter.parse(&body) {
        Ok(message) => message,
        Err(err) => {
            debug!("gateway ignoring whatsapp event: {}", err);
            return (StatusCode::OK, Json(json!({"status": "ignored"})));
        }
    };

    let phone_number = message
        .metadata
        .whatsapp_phone_number
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    let Some(route) = resolve_route(Channel::WhatsApp, &phone_number, &state) else {
        info!(
            "gateway no route for whatsapp phone_number={}",
            phone_number
        );
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let external_message_id = message.message_id.clone();
    let envelope = match build_envelope(
        route,
        Channel::WhatsApp,
        external_message_id,
        &message,
        &body,
    )
    .await
    {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "payload_store_failed"})),
            );
        }
    };
    enqueue_envelope(state.queue.clone(), envelope).await
}

pub(super) async fn enqueue_envelope(
    queue: Arc<dyn IngestionQueue>,
    envelope: IngestionEnvelope,
) -> (StatusCode, Json<serde_json::Value>) {
    let result = tokio::task::spawn_blocking(move || queue.enqueue(&envelope)).await;
    match result {
        Ok(Ok(result)) => {
            if result.inserted {
                (StatusCode::OK, Json(json!({"status": "accepted"})))
            } else {
                (StatusCode::OK, Json(json!({"status": "duplicate"})))
            }
        }
        Ok(Err(err)) => {
            error!("gateway enqueue error: {}", err);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "enqueue_failed"})),
            )
        }
        Err(err) => {
            error!("gateway enqueue join error: {}", err);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status": "enqueue_failed"})),
            )
        }
    }
}

const NO_REPLY_MARKERS: [&str; 3] = ["noreply", "no-reply", "do-not-reply"];

fn payload_contains_no_reply_marker(payload: &PostmarkInboundPayload) -> bool {
    let candidates = [payload.from.as_deref(), payload.reply_to.as_deref()];
    candidates
        .into_iter()
        .flatten()
        .any(contains_no_reply_marker)
}

fn contains_no_reply_marker(value: &str) -> bool {
    let emails = extract_emails(value);
    if emails.is_empty() {
        return false;
    }
    emails.into_iter().any(|email| {
        let normalized = email.trim().to_ascii_lowercase();
        NO_REPLY_MARKERS
            .iter()
            .any(|marker| normalized.contains(marker))
    })
}

fn build_queue_payload(channel: Channel, message: &InboundMessage) -> IngestionPayload {
    let mut payload = IngestionPayload::from_inbound(message);
    if channel == Channel::Email {
        // Keep queue envelopes small; email attachment bytes are loaded from raw payload/blob.
        payload.attachments.clear();
    }
    payload
}

async fn rewrite_email_payload_attachments_to_blob_refs(
    envelope_id: Uuid,
    received_at: chrono::DateTime<chrono::Utc>,
    raw_payload: &[u8],
) -> Vec<u8> {
    let mut payload_json: serde_json::Value = match serde_json::from_slice(raw_payload) {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "gateway failed to parse email raw payload for attachment offload: {}",
                err
            );
            return raw_payload.to_vec();
        }
    };
    let Some(attachments) = payload_json
        .get_mut("Attachments")
        .and_then(serde_json::Value::as_array_mut)
    else {
        return raw_payload.to_vec();
    };

    for (index, attachment) in attachments.iter_mut().enumerate() {
        let Some(obj) = attachment.as_object_mut() else {
            continue;
        };
        let content = obj
            .get("Content")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        if content.is_empty() {
            continue;
        }
        let file_name = obj
            .get("Name")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("attachment");

        let decoded = match BASE64_STANDARD.decode(content.as_bytes()) {
            Ok(bytes) => bytes,
            Err(err) => {
                warn!(
                    "gateway failed to decode email attachment '{}' for blob offload: {}",
                    file_name, err
                );
                continue;
            }
        };
        if decoded.is_empty() {
            continue;
        }

        match raw_payload_store::upload_attachment_azure(
            envelope_id,
            received_at,
            index,
            file_name,
            &decoded,
        )
        .await
        {
            Ok(storage_ref) => {
                obj.insert(
                    "StorageRef".to_string(),
                    serde_json::Value::String(storage_ref),
                );
                obj.insert(
                    "Content".to_string(),
                    serde_json::Value::String(String::new()),
                );
                obj.insert(
                    "ContentLength".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(decoded.len())),
                );
            }
            Err(err) => {
                warn!(
                    "gateway failed to upload email attachment '{}' to blob: {}",
                    file_name, err
                );
            }
        }
    }

    serde_json::to_vec(&payload_json).unwrap_or_else(|err| {
        warn!(
            "gateway failed to serialize rewritten email raw payload; using original: {}",
            err
        );
        raw_payload.to_vec()
    })
}

fn rewrite_email_payload_attachments_to_blob_refs_blocking(
    envelope_id: Uuid,
    received_at: chrono::DateTime<chrono::Utc>,
    raw_payload: &[u8],
) -> Vec<u8> {
    let mut payload_json: serde_json::Value = match serde_json::from_slice(raw_payload) {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "gateway failed to parse email raw payload for attachment offload: {}",
                err
            );
            return raw_payload.to_vec();
        }
    };
    let Some(attachments) = payload_json
        .get_mut("Attachments")
        .and_then(serde_json::Value::as_array_mut)
    else {
        return raw_payload.to_vec();
    };

    for (index, attachment) in attachments.iter_mut().enumerate() {
        let Some(obj) = attachment.as_object_mut() else {
            continue;
        };
        let content = obj
            .get("Content")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        if content.is_empty() {
            continue;
        }
        let file_name = obj
            .get("Name")
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("attachment");

        let decoded = match BASE64_STANDARD.decode(content.as_bytes()) {
            Ok(bytes) => bytes,
            Err(err) => {
                warn!(
                    "gateway failed to decode email attachment '{}' for blob offload: {}",
                    file_name, err
                );
                continue;
            }
        };
        if decoded.is_empty() {
            continue;
        }

        match raw_payload_store::upload_attachment_azure_blocking(
            envelope_id,
            received_at,
            index,
            file_name,
            &decoded,
        ) {
            Ok(storage_ref) => {
                obj.insert(
                    "StorageRef".to_string(),
                    serde_json::Value::String(storage_ref),
                );
                obj.insert(
                    "Content".to_string(),
                    serde_json::Value::String(String::new()),
                );
                obj.insert(
                    "ContentLength".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(decoded.len())),
                );
            }
            Err(err) => {
                warn!(
                    "gateway failed to upload email attachment '{}' to blob: {}",
                    file_name, err
                );
            }
        }
    }

    serde_json::to_vec(&payload_json).unwrap_or_else(|err| {
        warn!(
            "gateway failed to serialize rewritten email raw payload; using original: {}",
            err
        );
        raw_payload.to_vec()
    })
}

pub(super) async fn build_envelope(
    route: RouteDecision,
    channel: Channel,
    external_message_id: Option<String>,
    message: &InboundMessage,
    raw_payload: &[u8],
) -> Result<IngestionEnvelope, RawPayloadStoreError> {
    let envelope_id = Uuid::new_v4();
    let received_at = Utc::now();
    let queue_payload = build_queue_payload(channel, message);
    let dedupe_key = build_dedupe_key(
        &route.tenant_id,
        &route.employee_id,
        channel,
        external_message_id.as_deref(),
        raw_payload,
    );
    let stored_payload_bytes = if channel == Channel::Email {
        rewrite_email_payload_attachments_to_blob_refs(envelope_id, received_at, raw_payload).await
    } else {
        raw_payload.to_vec()
    };
    let raw_payload_ref = if raw_payload.is_empty() {
        None
    } else {
        Some(
            raw_payload_store::upload_raw_payload_azure(
                envelope_id,
                received_at,
                &stored_payload_bytes,
            )
            .await?,
        )
    };
    Ok(IngestionEnvelope {
        envelope_id,
        received_at,
        tenant_id: Some(route.tenant_id),
        employee_id: route.employee_id,
        channel,
        external_message_id,
        dedupe_key,
        payload: queue_payload,
        raw_payload_ref,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use scheduler_module::adapters::slack::{SlackEventWrapper, SlackMessageEvent};
    use scheduler_module::channel::Attachment;

    #[test]
    fn payload_contains_no_reply_marker_detects_from() {
        let payload: PostmarkInboundPayload =
            serde_json::from_str(r#"{"From":"noreply@example.com"}"#).expect("payload");
        assert!(payload_contains_no_reply_marker(&payload));
    }

    #[test]
    fn payload_contains_no_reply_marker_detects_reply_to() {
        let payload: PostmarkInboundPayload =
            serde_json::from_str(r#"{"From":"user@example.com","ReplyTo":"no-reply@x.com"}"#)
                .expect("payload");
        assert!(payload_contains_no_reply_marker(&payload));
    }

    #[test]
    fn payload_contains_no_reply_marker_allows_normal_sender() {
        let payload: PostmarkInboundPayload =
            serde_json::from_str(r#"{"From":"user@example.com"}"#).expect("payload");
        assert!(!payload_contains_no_reply_marker(&payload));
    }

    #[test]
    fn build_queue_payload_clears_email_attachments() {
        let message = InboundMessage {
            channel: Channel::Email,
            sender: "a@example.com".to_string(),
            sender_name: None,
            recipient: "svc@example.com".to_string(),
            subject: Some("s".to_string()),
            text_body: Some("t".to_string()),
            html_body: None,
            thread_id: "thread-1".to_string(),
            message_id: Some("m1".to_string()),
            attachments: vec![Attachment {
                name: "a.txt".to_string(),
                content_type: "text/plain".to_string(),
                content: "Zm9v".to_string(),
            }],
            reply_to: vec!["a@example.com".to_string()],
            raw_payload: br#"{}"#.to_vec(),
            metadata: ChannelMetadata::default(),
        };
        let payload = build_queue_payload(Channel::Email, &message);
        assert!(payload.attachments.is_empty());
    }

    #[test]
    fn build_queue_payload_keeps_non_email_attachments() {
        let message = InboundMessage {
            channel: Channel::Slack,
            sender: "U123".to_string(),
            sender_name: None,
            recipient: "C456".to_string(),
            subject: None,
            text_body: Some("hello".to_string()),
            html_body: None,
            thread_id: "thread-1".to_string(),
            message_id: Some("m1".to_string()),
            attachments: vec![Attachment {
                name: "file.pdf".to_string(),
                content_type: "application/pdf".to_string(),
                content: "placeholder".to_string(),
            }],
            reply_to: vec!["C456".to_string()],
            raw_payload: br#"{}"#.to_vec(),
            metadata: ChannelMetadata::default(),
        };
        let payload = build_queue_payload(Channel::Slack, &message);
        assert_eq!(payload.attachments.len(), 1);
    }

    #[test]
    fn should_enqueue_slack_message_accepts_app_mention() {
        let wrapper = SlackEventWrapper {
            event_type: "event_callback".to_string(),
            challenge: None,
            token: None,
            team_id: Some("T1".to_string()),
            api_app_id: Some("A1".to_string()),
            event: Some(SlackMessageEvent {
                event_type: "app_mention".to_string(),
                subtype: None,
                channel: Some("C1".to_string()),
                user: Some("U1".to_string()),
                text: Some("<@B1> hi".to_string()),
                ts: "1.01".to_string(),
                thread_ts: None,
                bot_id: None,
                app_id: None,
                files: None,
                channel_type: Some("channel".to_string()),
                event_ts: None,
            }),
            event_id: Some("Ev1".to_string()),
            event_time: None,
        };

        assert!(should_enqueue_slack_message(&wrapper, Some("B1")));
    }

    #[test]
    fn should_enqueue_slack_message_rejects_channel_message_without_bot_mention() {
        let wrapper = SlackEventWrapper {
            event_type: "event_callback".to_string(),
            challenge: None,
            token: None,
            team_id: Some("T1".to_string()),
            api_app_id: Some("A1".to_string()),
            event: Some(SlackMessageEvent {
                event_type: "message".to_string(),
                subtype: None,
                channel: Some("C1".to_string()),
                user: Some("U1".to_string()),
                text: Some("hello world".to_string()),
                ts: "1.02".to_string(),
                thread_ts: None,
                bot_id: None,
                app_id: None,
                files: None,
                channel_type: Some("channel".to_string()),
                event_ts: None,
            }),
            event_id: Some("Ev2".to_string()),
            event_time: None,
        };

        assert!(!should_enqueue_slack_message(&wrapper, Some("B1")));
    }

    #[test]
    fn should_enqueue_slack_message_accepts_dm_message() {
        let wrapper = SlackEventWrapper {
            event_type: "event_callback".to_string(),
            challenge: None,
            token: None,
            team_id: Some("T1".to_string()),
            api_app_id: Some("A1".to_string()),
            event: Some(SlackMessageEvent {
                event_type: "message".to_string(),
                subtype: None,
                channel: Some("D1".to_string()),
                user: Some("U1".to_string()),
                text: Some("hello in dm".to_string()),
                ts: "1.03".to_string(),
                thread_ts: None,
                bot_id: None,
                app_id: None,
                files: None,
                channel_type: Some("im".to_string()),
                event_ts: None,
            }),
            event_id: Some("Ev3".to_string()),
            event_time: None,
        };

        assert!(should_enqueue_slack_message(&wrapper, None));
    }
}

pub(super) fn build_envelope_blocking(
    route: RouteDecision,
    channel: Channel,
    external_message_id: Option<String>,
    message: &InboundMessage,
    raw_payload: &[u8],
) -> Result<IngestionEnvelope, RawPayloadStoreError> {
    let envelope_id = Uuid::new_v4();
    let received_at = Utc::now();
    let queue_payload = build_queue_payload(channel, message);
    let dedupe_key = build_dedupe_key(
        &route.tenant_id,
        &route.employee_id,
        channel,
        external_message_id.as_deref(),
        raw_payload,
    );
    let stored_payload_bytes = if channel == Channel::Email {
        rewrite_email_payload_attachments_to_blob_refs_blocking(
            envelope_id,
            received_at,
            raw_payload,
        )
    } else {
        raw_payload.to_vec()
    };
    let raw_payload_ref = if raw_payload.is_empty() {
        None
    } else {
        Some(raw_payload_store::upload_raw_payload_azure_blocking(
            envelope_id,
            received_at,
            &stored_payload_bytes,
        )?)
    };
    Ok(IngestionEnvelope {
        envelope_id,
        received_at,
        tenant_id: Some(route.tenant_id),
        employee_id: route.employee_id,
        channel,
        external_message_id,
        dedupe_key,
        payload: queue_payload,
        raw_payload_ref,
    })
}
