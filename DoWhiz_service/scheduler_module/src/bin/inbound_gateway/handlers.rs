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
use scheduler_module::adapters::wechat::WeChatInboundAdapter;
use scheduler_module::adapters::whatsapp::WhatsAppInboundAdapter;
use scheduler_module::channel::{Channel, ChannelMetadata, InboundAdapter, InboundMessage};
use scheduler_module::ingestion::{IngestionEnvelope, IngestionPayload};
use scheduler_module::ingestion_queue::IngestionQueue;
use scheduler_module::raw_payload_store::{self, RawPayloadStoreError};
use scheduler_module::user_store::extract_emails;

use super::routes::{build_dedupe_key, normalize_email, normalize_phone_number, resolve_route};
use super::state::{find_service_address, GatewayState, RouteDecision, RouteKey, RouteTarget};
use super::verify::{
    verify_bluebubbles, verify_postmark, verify_slack, verify_twilio, verify_wechat,
    verify_whatsapp_subscription,
};

/// Request payload for creating a workspace brief document
#[derive(Debug, Deserialize)]
pub(super) struct CreateWorkspaceBriefRequest {
    pub founder_name: String,
    pub founder_email: String,
    pub venture_name: Option<String>,
    pub thesis: Option<String>,
    pub stage: Option<String>,
    pub goals: Vec<String>,
    pub current_assets: Option<Vec<String>>,
    pub plan_horizon_days: Option<i32>,
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Create90DayPlanRequest {
    pub founder_name: String,
    pub founder_email: String,
    pub venture_name: Option<String>,
    pub thesis: Option<String>,
    pub stage: Option<String>,
    pub goals: Vec<String>,
    pub current_assets: Option<Vec<String>>,
    pub plan_horizon_days: Option<i32>,
    pub account_id: Option<Uuid>,
}

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
        Err(e) => {
            let body_preview = String::from_utf8_lossy(&body[..body.len().min(500)]);
            warn!("gateway failed to parse postmark payload: {} - body preview: {}", e, body_preview);
            return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_json"})));
        }
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
        let body_preview = String::from_utf8_lossy(&body[..body.len().min(1000)]);
        info!(
            "gateway no service address found in postmark payload: to={:?}, cc={:?}, bcc={:?}, original_recipient={:?}, from={:?}, subject={:?}, body_preview={}",
            payload.to,
            payload.cc,
            payload.bcc,
            payload.original_recipient,
            payload.from,
            payload.subject,
            body_preview
        );
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

    let Some(route) = resolve_slack_route(api_app_id, &state) else {
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

fn route_decision_from_target(target: RouteTarget, state: &GatewayState) -> RouteDecision {
    let tenant_id = target
        .tenant_id
        .clone()
        .or_else(|| state.config.defaults.tenant_id.clone())
        .unwrap_or_else(|| "default".to_string());
    RouteDecision {
        tenant_id,
        employee_id: target.employee_id,
    }
}

fn resolve_employee_id_by_slack_app_id(api_app_id: &str, state: &GatewayState) -> Option<String> {
    let app_id = api_app_id.trim();
    if app_id.is_empty() {
        return None;
    }

    for employee in &state.employee_directory.employees {
        let env_key = format!(
            "{}_SLACK_APP_ID",
            employee.id.to_uppercase().replace('-', "_")
        );
        let matched = std::env::var(&env_key)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .map(|value| value == app_id)
            .unwrap_or(false);
        if matched {
            return Some(employee.id.clone());
        }
    }

    let default_matched = std::env::var("SLACK_APP_ID")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(|value| value == app_id)
        .unwrap_or(false);
    if default_matched {
        return state
            .config
            .defaults
            .employee_id
            .clone()
            .or_else(|| state.employee_directory.default_employee_id.clone());
    }

    None
}

fn resolve_slack_route(api_app_id: &str, state: &GatewayState) -> Option<RouteDecision> {
    // 1) Exact route in gateway config (api_app_id specific) has highest precedence.
    let explicit_key = RouteKey {
        channel: Channel::Slack,
        key: api_app_id.to_string(),
    };
    if let Some(target) = state.config.routes.get(&explicit_key).cloned() {
        return Some(route_decision_from_target(target, state));
    }

    // 2) Env-based app-id mapping (e.g. BOILED_EGG_SLACK_APP_ID) to avoid wildcard misrouting.
    if let Some(employee_id) = resolve_employee_id_by_slack_app_id(api_app_id, state) {
        return Some(RouteDecision {
            tenant_id: state
                .config
                .defaults
                .tenant_id
                .clone()
                .unwrap_or_else(|| "default".to_string()),
            employee_id,
        });
    }

    // 3) Fallback to existing wildcard/default route behavior.
    resolve_route(Channel::Slack, api_app_id, state)
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

/// Query parameters for WeChat webhook verification
#[derive(Debug, Deserialize)]
pub(super) struct WeChatVerifyParams {
    /// WeChat sends "signature" for URL verification, "msg_signature" for encrypted messages
    #[serde(alias = "msg_signature")]
    pub signature: Option<String>,
    pub timestamp: Option<String>,
    pub nonce: Option<String>,
    pub echostr: Option<String>,
}

/// Handle WeChat webhook verification (GET request)
pub(super) async fn verify_wechat_webhook(
    Query(params): Query<WeChatVerifyParams>,
) -> impl IntoResponse {
    info!(
        "wechat verification request: signature={:?} timestamp={:?} nonce={:?} echostr_len={:?}",
        params.signature.as_deref(),
        params.timestamp.as_deref(),
        params.nonce.as_deref(),
        params.echostr.as_ref().map(|s| s.len())
    );

    match verify_wechat(
        params.signature.as_deref(),
        params.timestamp.as_deref(),
        params.nonce.as_deref(),
        params.echostr.as_deref(),
    ) {
        Ok(echostr) => {
            info!("wechat verification succeeded, returning echostr len={}", echostr.len());
            (StatusCode::OK, echostr)
        }
        Err(reason) => {
            warn!("wechat webhook verification failed: {}", reason);
            (StatusCode::FORBIDDEN, reason.to_string())
        }
    }
}

/// Handle WeChat inbound messages (POST request)
pub(super) async fn ingest_wechat(
    State(state): State<Arc<GatewayState>>,
    body: Bytes,
) -> impl IntoResponse {
    let adapter = WeChatInboundAdapter::new();
    let message = match adapter.parse(&body) {
        Ok(message) => message,
        Err(err) => {
            debug!("gateway ignoring wechat event: {}", err);
            return (StatusCode::OK, Json(json!({"status": "ignored"})));
        }
    };

    let user_id = message
        .metadata
        .wechat_user_id
        .clone()
        .unwrap_or_else(|| message.sender.clone());

    let Some(route) = resolve_route(Channel::WeChat, &user_id, &state) else {
        info!("gateway no route for wechat user_id={}", user_id);
        return (StatusCode::OK, Json(json!({"status": "no_route"})));
    };

    let external_message_id = message.message_id.clone();
    let envelope =
        match build_envelope(route, Channel::WeChat, external_message_id, &message, &body).await {
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

const NO_REPLY_MARKERS: [&str; 5] = [
    "noreply",
    "no-reply",
    "do-not-reply",
    "mailer-daemon",
    "postmaster",
];

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
        match raw_payload_store::upload_raw_payload_azure(
            envelope_id,
            received_at,
            &stored_payload_bytes,
        )
        .await
        {
            Ok(payload_ref) => Some(payload_ref),
            Err(err) => {
                // Non-blocking: log error but continue without raw payload archival
                tracing::error!("failed to upload raw payload: {}", err);
                None
            }
        }
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
        account_id: None,
    })
}

/// Handle workspace brief creation request
/// POST /api/workspace/create-brief
pub(super) async fn create_workspace_brief(
    State(state): State<Arc<GatewayState>>,
    Json(request): Json<CreateWorkspaceBriefRequest>,
) -> impl IntoResponse {
    info!(
        "workspace brief request: founder={} email={}",
        request.founder_name, request.founder_email
    );

    // Determine employee to route to (default to oliver)
    let employee_id = state
        .config
        .defaults
        .employee_id
        .clone()
        .unwrap_or_else(|| "oliver".to_string());

    let tenant_id = state
        .config
        .defaults
        .tenant_id
        .clone()
        .unwrap_or_else(|| "default".to_string());

    // Build the prompt for Codex
    let venture_name = request.venture_name.as_deref().unwrap_or("Startup");
    let thesis = request.thesis.as_deref().unwrap_or("Building something great");
    let stage = request.stage.as_deref().unwrap_or("idea");
    let horizon = request.plan_horizon_days.unwrap_or(30);
    let goals_text = if request.goals.is_empty() {
        "- Define initial goals".to_string()
    } else {
        request.goals.iter().map(|g| format!("- {}", g)).collect::<Vec<_>>().join("\n")
    };
    let assets_text = request
        .current_assets
        .as_ref()
        .filter(|a| !a.is_empty())
        .map(|a| a.iter().map(|x| format!("- {}", x)).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|| "- None listed yet".to_string());

    let prompt = format!(
        r#"Create a Startup Workspace Brief Google Doc for this founder and share it with them.

## Founder Information
- **Name:** {founder_name}
- **Email:** {founder_email}
- **Venture Name:** {venture_name}
- **Thesis:** {thesis}
- **Stage:** {stage}
- **Planning Horizon:** {horizon} days

## Goals (30-90 days)
{goals_text}

## Current Assets
{assets_text}

## Instructions
1. Create a new Google Doc titled "Startup Workspace Brief - {venture_name}"
2. Add the following sections with professional formatting:
   - Executive Summary (synthesize the thesis and stage)
   - Founder Profile
   - 30-90 Day Goals (expand on each goal with suggested milestones)
   - Current Assets & Resources
   - Recommended Next Steps
   - Key Metrics to Track
3. Share the document with {founder_email} as a writer
4. Send an email to {founder_email} with the document link and a brief introduction

Use the google-docs skill to create and share the document."#,
        founder_name = request.founder_name,
        founder_email = request.founder_email,
        venture_name = venture_name,
        thesis = thesis,
        stage = stage,
        horizon = horizon,
        goals_text = goals_text,
        assets_text = assets_text,
    );

    // Build InboundMessage with the prompt using Email channel
    let message_id = format!("<workspace-brief-{}@dowhiz.com>", Uuid::new_v4());
    let recipient_email = state
        .employee_directory
        .employees
        .iter()
        .find(|e| e.id == employee_id)
        .and_then(|e| e.address_set.iter().next())
        .cloned()
        .unwrap_or_else(|| format!("{}@dowhiz.com", employee_id));
    let subject = format!("Create Workspace Brief for {}", venture_name);

    let message = InboundMessage {
        channel: Channel::Email,
        sender: request.founder_email.clone(),
        sender_name: Some(request.founder_name.clone()),
        recipient: recipient_email.clone(),
        subject: Some(subject.clone()),
        text_body: Some(prompt.clone()),
        html_body: None,
        thread_id: message_id.clone(),
        message_id: Some(message_id.clone()),
        attachments: Vec::new(),
        reply_to: vec![request.founder_email.clone()],
        raw_payload: Vec::new(),
        metadata: ChannelMetadata::default(),
    };

    // Build synthetic Postmark-style email payload
    let email_payload = json!({
        "From": format!("{} <{}>", request.founder_name, request.founder_email),
        "To": recipient_email,
        "ReplyTo": request.founder_email,
        "Subject": subject,
        "TextBody": prompt,
        "MessageID": message_id
    });
    let raw_payload = serde_json::to_vec(&email_payload).unwrap_or_default();

    let route = RouteDecision {
        tenant_id,
        employee_id,
    };

    let external_message_id = message.message_id.clone();
    let mut envelope = match build_envelope(
        route,
        Channel::Email,
        external_message_id,
        &message,
        &raw_payload,
    )
    .await
    {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("failed to build workspace brief envelope: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "envelope_build_failed", "error": err.to_string()})),
            );
        }
    };

    envelope.account_id = request.account_id;

    let task_id = envelope.envelope_id.to_string();
    let result = enqueue_envelope(state.queue.clone(), envelope).await;

    // Augment response with task_id for potential polling
    match result {
        (StatusCode::OK, Json(mut body)) => {
            if let Some(obj) = body.as_object_mut() {
                obj.insert("task_id".to_string(), json!(task_id));
            }
            (StatusCode::OK, Json(body))
        }
        other => other,
    }
}

pub(super) async fn create_90_day_plan(
    State(state): State<Arc<GatewayState>>,
    Json(request): Json<Create90DayPlanRequest>,
) -> impl IntoResponse {
    info!(
        "90-day plan request: founder={} email={}",
        request.founder_name, request.founder_email
    );

    let employee_id = state
        .config
        .defaults
        .employee_id
        .clone()
        .unwrap_or_else(|| "oliver".to_string());

    let tenant_id = state
        .config
        .defaults
        .tenant_id
        .clone()
        .unwrap_or_else(|| "default".to_string());

    let venture_name = request.venture_name.as_deref().unwrap_or("Startup");
    let thesis = request.thesis.as_deref().unwrap_or("Building something great");
    let stage = request.stage.as_deref().unwrap_or("idea");
    let horizon = request.plan_horizon_days.unwrap_or(90);
    let goals_text = if request.goals.is_empty() {
        "- Define initial goals".to_string()
    } else {
        request.goals.iter().map(|g| format!("- {}", g)).collect::<Vec<_>>().join("\n")
    };
    let assets_text = request
        .current_assets
        .as_ref()
        .filter(|a| !a.is_empty())
        .map(|a| a.iter().map(|x| format!("- {}", x)).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|| "- None listed yet".to_string());

    let prompt = format!(
        r#"Create a professional, well-formatted {horizon}-Day Action Plan Google Doc for this founder and share it with them.

## Founder Information
- **Name:** {founder_name}
- **Email:** {founder_email}
- **Venture Name:** {venture_name}
- **Thesis:** {thesis}
- **Stage:** {stage}

## Goals
{goals_text}

## Current Assets
{assets_text}

## Instructions
1. Create a new Google Doc titled "{horizon}-Day Action Plan - {venture_name}"
2. Use professional formatting throughout:
   - Clear heading hierarchy (Title, H1, H2, H3)
   - Consistent spacing and indentation
   - Bullet points and numbered lists where appropriate
   - Bold text for key terms and deadlines
   - Tables for weekly breakdowns if helpful
3. Structure the document with:
   - Executive Summary (1 paragraph synthesizing the goals and timeline)
   - Week-by-week breakdown for {horizon} days ({weeks} weeks total)
   - Each week should have 2-3 concrete, actionable tasks derived from the goals
   - Milestones section marking key checkpoints at day 30, 60, and 90
   - Success Metrics (specific, measurable criteria for each goal)
   - Resources Needed (based on current assets and identified gaps)
4. Share the document with {founder_email} as a writer
5. Send an email to {founder_email} with the document link

Use the google-docs skill to create and share the document."#,
        founder_name = request.founder_name,
        founder_email = request.founder_email,
        venture_name = venture_name,
        thesis = thesis,
        stage = stage,
        horizon = horizon,
        weeks = horizon / 7,
        goals_text = goals_text,
        assets_text = assets_text,
    );

    let message_id = format!("<90-day-plan-{}@dowhiz.com>", Uuid::new_v4());
    let recipient_email = state
        .employee_directory
        .employees
        .iter()
        .find(|e| e.id == employee_id)
        .and_then(|e| e.address_set.iter().next())
        .cloned()
        .unwrap_or_else(|| format!("{}@dowhiz.com", employee_id));
    let subject = format!("Create {}-Day Action Plan for {}", horizon, venture_name);

    let message = InboundMessage {
        channel: Channel::Email,
        sender: request.founder_email.clone(),
        sender_name: Some(request.founder_name.clone()),
        recipient: recipient_email.clone(),
        subject: Some(subject.clone()),
        text_body: Some(prompt.clone()),
        html_body: None,
        thread_id: message_id.clone(),
        message_id: Some(message_id.clone()),
        attachments: Vec::new(),
        reply_to: vec![request.founder_email.clone()],
        raw_payload: Vec::new(),
        metadata: ChannelMetadata::default(),
    };

    let email_payload = json!({
        "From": format!("{} <{}>", request.founder_name, request.founder_email),
        "To": recipient_email,
        "ReplyTo": request.founder_email,
        "Subject": subject,
        "TextBody": prompt,
        "MessageID": message_id
    });
    let raw_payload = serde_json::to_vec(&email_payload).unwrap_or_default();

    let route = RouteDecision {
        tenant_id,
        employee_id,
    };

    let external_message_id = message.message_id.clone();
    let mut envelope = match build_envelope(
        route,
        Channel::Email,
        external_message_id,
        &message,
        &raw_payload,
    )
    .await
    {
        Ok(envelope) => envelope,
        Err(err) => {
            error!("failed to build 90-day plan envelope: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "envelope_build_failed", "error": err.to_string()})),
            );
        }
    };

    envelope.account_id = request.account_id;

    let task_id = envelope.envelope_id.to_string();
    let result = enqueue_envelope(state.queue.clone(), envelope).await;

    match result {
        (StatusCode::OK, Json(mut body)) => {
            if let Some(obj) = body.as_object_mut() {
                obj.insert("task_id".to_string(), json!(task_id));
            }
            (StatusCode::OK, Json(body))
        }
        other => other,
    }
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
    fn payload_contains_no_reply_marker_detects_mailer_daemon() {
        let payload: PostmarkInboundPayload =
            serde_json::from_str(r#"{"From":"mailer-daemon@googlemail.com"}"#).expect("payload");
        assert!(payload_contains_no_reply_marker(&payload));
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

    #[test]
    fn create_workspace_brief_request_parses_full_payload() {
        let json = r#"{
            "founder_name": "Dylan Tang",
            "founder_email": "dylan@example.com",
            "venture_name": "Acme Labs",
            "thesis": "Building AI tools for productivity",
            "stage": "mvp",
            "goals": ["Launch MVP", "Get 3 pilot customers", "Raise seed round"],
            "current_assets": ["Landing page", "Figma mockups"],
            "plan_horizon_days": 60
        }"#;

        let request: CreateWorkspaceBriefRequest =
            serde_json::from_str(json).expect("should parse");

        assert_eq!(request.founder_name, "Dylan Tang");
        assert_eq!(request.founder_email, "dylan@example.com");
        assert_eq!(request.venture_name.as_deref(), Some("Acme Labs"));
        assert_eq!(request.thesis.as_deref(), Some("Building AI tools for productivity"));
        assert_eq!(request.stage.as_deref(), Some("mvp"));
        assert_eq!(request.goals.len(), 3);
        assert_eq!(request.goals[0], "Launch MVP");
        assert_eq!(request.current_assets.as_ref().map(|a| a.len()), Some(2));
        assert_eq!(request.plan_horizon_days, Some(60));
    }

    #[test]
    fn create_workspace_brief_request_parses_minimal_payload() {
        let json = r#"{
            "founder_name": "Jane Doe",
            "founder_email": "jane@example.com",
            "goals": []
        }"#;

        let request: CreateWorkspaceBriefRequest =
            serde_json::from_str(json).expect("should parse");

        assert_eq!(request.founder_name, "Jane Doe");
        assert_eq!(request.founder_email, "jane@example.com");
        assert!(request.venture_name.is_none());
        assert!(request.thesis.is_none());
        assert!(request.stage.is_none());
        assert!(request.goals.is_empty());
        assert!(request.current_assets.is_none());
        assert!(request.plan_horizon_days.is_none());
    }

    #[test]
    fn create_workspace_brief_request_rejects_missing_required_fields() {
        // Missing founder_email
        let json = r#"{"founder_name": "Test", "goals": []}"#;
        let result: Result<CreateWorkspaceBriefRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Missing founder_name
        let json = r#"{"founder_email": "test@example.com", "goals": []}"#;
        let result: Result<CreateWorkspaceBriefRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Missing goals
        let json = r#"{"founder_name": "Test", "founder_email": "test@example.com"}"#;
        let result: Result<CreateWorkspaceBriefRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn create_90_day_plan_request_parses_full_payload() {
        let json = r#"{
            "founder_name": "Dylan Tang",
            "founder_email": "dylan@example.com",
            "venture_name": "Acme Labs",
            "thesis": "Building AI tools for productivity",
            "stage": "mvp",
            "goals": ["Launch MVP", "Get 3 pilot customers", "Raise seed round"],
            "current_assets": ["Landing page", "Figma mockups"],
            "plan_horizon_days": 90
        }"#;

        let request: Create90DayPlanRequest =
            serde_json::from_str(json).expect("should parse");

        assert_eq!(request.founder_name, "Dylan Tang");
        assert_eq!(request.founder_email, "dylan@example.com");
        assert_eq!(request.venture_name.as_deref(), Some("Acme Labs"));
        assert_eq!(request.thesis.as_deref(), Some("Building AI tools for productivity"));
        assert_eq!(request.stage.as_deref(), Some("mvp"));
        assert_eq!(request.goals.len(), 3);
        assert_eq!(request.goals[0], "Launch MVP");
        assert_eq!(request.current_assets.as_ref().map(|a| a.len()), Some(2));
        assert_eq!(request.plan_horizon_days, Some(90));
    }

    #[test]
    fn create_90_day_plan_request_parses_minimal_payload() {
        let json = r#"{
            "founder_name": "Jane Doe",
            "founder_email": "jane@example.com",
            "goals": []
        }"#;

        let request: Create90DayPlanRequest =
            serde_json::from_str(json).expect("should parse");

        assert_eq!(request.founder_name, "Jane Doe");
        assert_eq!(request.founder_email, "jane@example.com");
        assert!(request.venture_name.is_none());
        assert!(request.thesis.is_none());
        assert!(request.stage.is_none());
        assert!(request.goals.is_empty());
        assert!(request.current_assets.is_none());
        assert!(request.plan_horizon_days.is_none());
    }

    #[test]
    fn create_90_day_plan_request_rejects_missing_required_fields() {
        let json = r#"{"founder_name": "Test", "goals": []}"#;
        let result: Result<Create90DayPlanRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#"{"founder_email": "test@example.com", "goals": []}"#;
        let result: Result<Create90DayPlanRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());

        let json = r#"{"founder_name": "Test", "founder_email": "test@example.com"}"#;
        let result: Result<Create90DayPlanRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
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
        account_id: None,
    })
}
