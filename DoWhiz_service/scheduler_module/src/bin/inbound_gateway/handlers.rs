use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use chrono::Utc;
use serde_json::json;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use scheduler_module::adapters::bluebubbles::BlueBubblesInboundAdapter;
use scheduler_module::adapters::postmark::PostmarkInboundPayload;
use scheduler_module::adapters::slack::{is_url_verification, SlackChallengeResponse, SlackInboundAdapter};
use scheduler_module::adapters::telegram::TelegramInboundAdapter;
use scheduler_module::adapters::whatsapp::WhatsAppInboundAdapter;
use scheduler_module::channel::{Channel, ChannelMetadata, InboundAdapter, InboundMessage};
use scheduler_module::ingestion::{IngestionEnvelope, IngestionPayload};
use scheduler_module::ingestion_queue::IngestionQueue;
use scheduler_module::raw_payload_store::{self, RawPayloadStoreError};

use super::routes::{build_dedupe_key, normalize_email, normalize_phone_number, resolve_route};
use super::state::{find_service_address, GatewayState, RouteDecision};
use super::verify::{verify_bluebubbles, verify_postmark, verify_slack, verify_twilio, verify_whatsapp_subscription};

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
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_json"})))
        }
    };

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
            return (StatusCode::BAD_REQUEST, Json(json!({"status": "parse_error"})));
        }
    };

    let external_message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let envelope = match build_envelope(route, Channel::Email, external_message_id, &message, &body)
        .await
    {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (StatusCode::BAD_GATEWAY, Json(json!({"status": "payload_store_failed"})));
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

    let wrapper: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(value) => value,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_json"})))
        }
    };

    let team_id = wrapper
        .get("team_id")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    if team_id.is_empty() {
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    }

    let event_id = wrapper
        .get("event_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());

    let Some(route) = resolve_route(Channel::Slack, team_id, &state) else {
        info!("gateway no route for slack team_id={}", team_id);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let adapter = SlackInboundAdapter::new(HashSet::new());
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
            return (StatusCode::BAD_GATEWAY, Json(json!({"status": "payload_store_failed"})));
        }
    };
    enqueue_envelope(state.queue.clone(), envelope).await
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
    let envelope = match build_envelope(route, Channel::BlueBubbles, external_message_id, &message, &body).await {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (StatusCode::BAD_GATEWAY, Json(json!({"status": "payload_store_failed"})));
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
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_form"})))
        }
    };

    let from = params.get("From").cloned().unwrap_or_default();
    let to = params.get("To").cloned().unwrap_or_default();
    let body_text = params.get("Body").cloned().unwrap_or_default();
    let message_sid = params.get("MessageSid").cloned();

    if from.is_empty() || to.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"status": "missing_fields"})));
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
            return (StatusCode::BAD_GATEWAY, Json(json!({"status": "payload_store_failed"})));
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
    let envelope = match build_envelope(route, Channel::Telegram, external_message_id, &message, &body).await {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("gateway failed to store raw payload: {}", err);
            return (StatusCode::BAD_GATEWAY, Json(json!({"status": "payload_store_failed"})));
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
        info!("gateway no route for whatsapp phone_number={}", phone_number);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let external_message_id = message.message_id.clone();
    let envelope = build_envelope(route, Channel::WhatsApp, external_message_id, &message, &body);
    enqueue_envelope(state.queue.clone(), envelope).await
}

pub(super) async fn enqueue_envelope(
    queue: Arc<dyn IngestionQueue>,
    envelope: IngestionEnvelope,
) -> (StatusCode, Json<serde_json::Value>) {
    match queue.enqueue(&envelope) {
        Ok(result) => {
            if result.inserted {
                (StatusCode::OK, Json(json!({"status": "accepted"})))
            } else {
                (StatusCode::OK, Json(json!({"status": "duplicate"})))
            }
        }
        Err(err) => {
            error!("gateway enqueue error: {}", err);
            (StatusCode::BAD_GATEWAY, Json(json!({"status": "enqueue_failed"})))
        }
    }
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
    let dedupe_key = build_dedupe_key(
        &route.tenant_id,
        &route.employee_id,
        channel,
        external_message_id.as_deref(),
        raw_payload,
    );
    let raw_payload_ref = if raw_payload.is_empty() {
        None
    } else {
        Some(raw_payload_store::upload_raw_payload(envelope_id, received_at, raw_payload).await?)
    };
    Ok(IngestionEnvelope {
        envelope_id,
        received_at,
        tenant_id: Some(route.tenant_id),
        employee_id: route.employee_id,
        channel,
        external_message_id,
        dedupe_key,
        payload: IngestionPayload::from_inbound(message),
        raw_payload_ref,
    })
}
