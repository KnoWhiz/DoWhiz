use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use serde_json::json;
use sha1::Sha1;
use sha2::Sha256;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use scheduler_module::adapters::bluebubbles::BlueBubblesInboundAdapter;
use scheduler_module::adapters::discord::DiscordInboundAdapter;
use scheduler_module::adapters::postmark::PostmarkInboundPayload;
use scheduler_module::adapters::slack::{is_url_verification, SlackChallengeResponse, SlackInboundAdapter};
use scheduler_module::adapters::telegram::TelegramInboundAdapter;
use scheduler_module::channel::{Channel, ChannelMetadata, InboundAdapter, InboundMessage};
use scheduler_module::employee_config::{load_employee_directory, EmployeeDirectory};
use scheduler_module::google_auth::GoogleAuthConfig;
use scheduler_module::google_docs_poller::GoogleDocsPollerConfig;
use scheduler_module::ingestion::{encode_raw_payload, IngestionEnvelope, IngestionPayload};
use scheduler_module::ingestion_queue::IngestionQueue;
use scheduler_module::mailbox;

#[derive(Debug, Deserialize, Default)]
struct GatewayConfigFile {
    #[serde(default)]
    server: GatewayServerConfig,
    #[serde(default)]
    storage: GatewayStorageConfig,
    #[serde(default)]
    defaults: GatewayDefaultsConfig,
    #[serde(default)]
    routes: Vec<GatewayRouteConfig>,
}

#[derive(Debug, Deserialize, Default)]
struct GatewayServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Deserialize, Default)]
struct GatewayStorageConfig {
    db_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct GatewayDefaultsConfig {
    tenant_id: Option<String>,
    employee_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct GatewayRouteConfig {
    channel: String,
    key: String,
    employee_id: String,
    tenant_id: Option<String>,
}

#[derive(Clone)]
struct GatewayConfig {
    db_path: PathBuf,
    defaults: GatewayDefaultsConfig,
    routes: HashMap<RouteKey, RouteTarget>,
    channel_defaults: HashMap<Channel, RouteTarget>,
}

#[derive(Clone)]
struct GatewayState {
    config: GatewayConfig,
    employee_directory: EmployeeDirectory,
    address_to_employee: HashMap<String, String>,
    queue: Arc<IngestionQueue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RouteKey {
    channel: Channel,
    key: String,
}

#[derive(Debug, Clone)]
struct RouteTarget {
    tenant_id: Option<String>,
    employee_id: String,
}

#[derive(Debug, Clone)]
struct RouteDecision {
    tenant_id: String,
    employee_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().with_target(false).init();
    dotenvy::dotenv().ok();

    let config_path = resolve_gateway_config_path()?;
    let config_file = load_gateway_config(&config_path)?;

    let employee_config_path = resolve_employee_config_path();
    let employee_directory = load_employee_directory(&employee_config_path)?;
    let address_to_employee = build_address_map(&employee_directory);

    let host = env::var("GATEWAY_HOST")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            config_file
                .server
                .host
                .unwrap_or_else(|| "0.0.0.0".to_string())
        });
    let port = env::var("GATEWAY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or_else(|| config_file.server.port.unwrap_or(9100));

    let db_path = env::var("INGESTION_DB_PATH")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            config_file
                .storage
                .db_path
                .unwrap_or_else(default_ingestion_db_path)
        });

    let (routes, channel_defaults) = normalize_routes(&config_file.routes)?;

    let queue = Arc::new(IngestionQueue::new(&db_path)?);

    let state = Arc::new(GatewayState {
        config: GatewayConfig {
            db_path,
            defaults: config_file.defaults,
            routes,
            channel_defaults,
        },
        employee_directory,
        address_to_employee,
        queue,
    });

    info!(
        "ingestion gateway config path={}, host={}, port={}, db_path={}",
        config_path.display(),
        host,
        port,
        state.config.db_path.display()
    );

    spawn_discord_gateway(state.clone()).await;
    spawn_google_docs_poller(state.clone());

    let max_body_bytes = env::var("GATEWAY_MAX_BODY_BYTES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(25 * 1024 * 1024);

    let app = Router::new()
        .route("/health", get(health))
        .route("/postmark/inbound", post(ingest_postmark))
        .route("/slack/events", post(ingest_slack))
        .route("/bluebubbles/webhook", post(ingest_bluebubbles))
        .route("/telegram/webhook", post(ingest_telegram))
        .route("/sms/twilio", post(ingest_sms))
        .with_state(state)
        .layer(DefaultBodyLimit::max(max_body_bytes));

    let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("ingestion gateway listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn ingest_postmark(
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

    let envelope = build_envelope(route, Channel::Email, external_message_id, &message, &body);
    enqueue_envelope(state.queue.clone(), envelope).await
}

async fn ingest_slack(
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

    let envelope = build_envelope(route, Channel::Slack, event_id, &message, &body);
    enqueue_envelope(state.queue.clone(), envelope).await
}

async fn ingest_bluebubbles(
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
    let envelope = build_envelope(route, Channel::BlueBubbles, external_message_id, &message, &body);
    enqueue_envelope(state.queue.clone(), envelope).await
}

async fn ingest_sms(
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

    let envelope = build_envelope(route, Channel::Sms, message_sid, &message, &body);
    enqueue_envelope(state.queue.clone(), envelope).await
}

async fn ingest_telegram(
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
    let envelope = build_envelope(route, Channel::Telegram, external_message_id, &message, &body);
    enqueue_envelope(state.queue.clone(), envelope).await
}

async fn enqueue_envelope(queue: Arc<IngestionQueue>, envelope: IngestionEnvelope) -> (StatusCode, Json<serde_json::Value>) {
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

fn build_envelope(
    route: RouteDecision,
    channel: Channel,
    external_message_id: Option<String>,
    message: &InboundMessage,
    raw_payload: &[u8],
) -> IngestionEnvelope {
    let dedupe_key = build_dedupe_key(
        &route.tenant_id,
        &route.employee_id,
        channel,
        external_message_id.as_deref(),
        raw_payload,
    );
    IngestionEnvelope {
        envelope_id: Uuid::new_v4(),
        received_at: Utc::now(),
        tenant_id: Some(route.tenant_id),
        employee_id: route.employee_id,
        channel,
        external_message_id,
        dedupe_key,
        payload: IngestionPayload::from_inbound(message),
        raw_payload_b64: encode_raw_payload(raw_payload),
    }
}

fn resolve_route(channel: Channel, route_key: &str, state: &GatewayState) -> Option<RouteDecision> {
    let normalized_key = normalize_route_key(channel, route_key);
    let key = RouteKey {
        channel,
        key: normalized_key.clone(),
    };

    let target = state.config.routes.get(&key).cloned().or_else(|| {
        state
            .config
            .channel_defaults
            .get(&channel)
            .cloned()
            .or_else(|| {
                if channel == Channel::Email {
                    state
                        .address_to_employee
                        .get(&normalized_key)
                        .map(|employee_id| RouteTarget {
                            employee_id: employee_id.clone(),
                            tenant_id: state.config.defaults.tenant_id.clone(),
                        })
                } else {
                    None
                }
            })
            .or_else(|| {
                state
                    .config
                    .defaults
                    .employee_id
                    .as_ref()
                    .map(|employee_id| RouteTarget {
                        employee_id: employee_id.clone(),
                        tenant_id: state.config.defaults.tenant_id.clone(),
                    })
            })
    })?;

    let tenant_id = target
        .tenant_id
        .clone()
        .or_else(|| state.config.defaults.tenant_id.clone())
        .unwrap_or_else(|| "default".to_string());

    Some(RouteDecision {
        tenant_id,
        employee_id: target.employee_id,
    })
}

fn build_dedupe_key(
    tenant_id: &str,
    employee_id: &str,
    channel: Channel,
    external_message_id: Option<&str>,
    raw_payload: &[u8],
) -> String {
    let base = if let Some(id) = external_message_id {
        id.to_string()
    } else if !raw_payload.is_empty() {
        format!("{:x}", md5::compute(raw_payload))
    } else {
        Uuid::new_v4().to_string()
    };
    format!("{}:{}:{}:{}", tenant_id, employee_id, channel, base)
}

fn normalize_routes(
    routes: &[GatewayRouteConfig],
) -> Result<(HashMap<RouteKey, RouteTarget>, HashMap<Channel, RouteTarget>), String> {
    let mut map = HashMap::new();
    let mut defaults = HashMap::new();

    for route in routes {
        let channel: Channel = route
            .channel
            .parse()
            .map_err(|err| format!("invalid route channel {}: {}", route.channel, err))?;
        let key = normalize_route_key(channel, route.key.trim());
        if key.is_empty() {
            return Err("route key cannot be empty".to_string());
        }
        let target = RouteTarget {
            tenant_id: route.tenant_id.clone(),
            employee_id: route.employee_id.clone(),
        };
        if key == "*" {
            defaults.insert(channel, target);
        } else {
            let route_key = RouteKey {
                channel,
                key,
            };
            map.insert(route_key, target);
        }
    }

    Ok((map, defaults))
}

fn normalize_route_key(channel: Channel, key: &str) -> String {
    let trimmed = key.trim();
    if trimmed == "*" {
        return "*".to_string();
    }
    match channel {
        Channel::Email => normalize_email(trimmed),
        Channel::Sms => normalize_phone_number(trimmed),
        _ => trimmed.to_string(),
    }
}

fn normalize_email(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn normalize_phone_number(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '+')
        .collect::<String>()
}

fn resolve_gateway_config_path() -> Result<PathBuf, String> {
    if let Ok(path) = env::var("GATEWAY_CONFIG_PATH") {
        if !path.trim().is_empty() {
            return Ok(PathBuf::from(path));
        }
    }

    let cwd = env::current_dir().map_err(|err| err.to_string())?;
    let direct = cwd.join("gateway.toml");
    if direct.exists() {
        return Ok(direct);
    }

    let nested = cwd.join("DoWhiz_service").join("gateway.toml");
    if nested.exists() {
        return Ok(nested);
    }

    Err("GATEWAY_CONFIG_PATH not set and gateway.toml not found".to_string())
}

fn resolve_employee_config_path() -> PathBuf {
    if let Ok(path) = env::var("EMPLOYEE_CONFIG_PATH") {
        if !path.trim().is_empty() {
            return PathBuf::from(path);
        }
    }

    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let direct = cwd.join("employee.toml");
    if direct.exists() {
        return direct;
    }
    let nested = cwd.join("DoWhiz_service").join("employee.toml");
    if nested.exists() {
        return nested;
    }

    PathBuf::from("DoWhiz_service/employee.toml")
}

fn load_gateway_config(path: &Path) -> Result<GatewayConfigFile, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read gateway config: {}", err))?;
    toml::from_str::<GatewayConfigFile>(&content)
        .map_err(|err| format!("failed to parse gateway config: {}", err))
}

fn default_ingestion_db_path() -> PathBuf {
    let base = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(".dowhiz")
        .join("DoWhiz")
        .join("gateway")
        .join("state")
        .join("ingestion.db")
}

fn build_address_map(directory: &EmployeeDirectory) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for employee in &directory.employees {
        for address in &employee.address_set {
            if let Some(prev) = map.insert(address.clone(), employee.id.clone()) {
                warn!(
                    "gateway duplicate address mapping: {} ({} -> {})",
                    address, prev, employee.id
                );
            }
        }
    }
    map
}

fn find_service_address(
    payload: &PostmarkInboundPayload,
    service_addresses: &HashSet<String>,
) -> Option<String> {
    let candidates = collect_service_address_candidates(payload);
    let mailbox = mailbox::select_inbound_service_mailbox(&candidates, service_addresses);
    mailbox.map(|value| value.address)
}

fn collect_service_address_candidates(payload: &PostmarkInboundPayload) -> Vec<Option<&str>> {
    let mut candidates = Vec::new();
    if let Some(value) = payload.to.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.cc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.bcc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(list) = payload.to_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.cc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.bcc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    for header in [
        "X-Original-To",
        "Delivered-To",
        "Envelope-To",
        "X-Envelope-To",
        "X-Forwarded-To",
        "X-Original-Recipient",
        "Original-Recipient",
    ] {
        for value in payload.header_values(header) {
            candidates.push(Some(value));
        }
    }
    candidates
}

fn verify_slack(headers: &HeaderMap, body: &[u8]) -> Result<(), &'static str> {
    let secret = env::var("SLACK_SIGNING_SECRET").ok();
    let Some(secret) = secret.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let signature = headers
        .get("x-slack-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_signature")?;
    let timestamp = headers
        .get("x-slack-request-timestamp")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_timestamp")?;
    let timestamp_value: i64 = timestamp.parse().map_err(|_| "invalid_timestamp")?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs() as i64;
    if (now - timestamp_value).abs() > 60 * 5 {
        return Err("stale_timestamp");
    }

    let base = format!("v0:{}:{}", timestamp, String::from_utf8_lossy(body));
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| "bad_secret")?;
    mac.update(base.as_bytes());
    let expected = format!("v0={}", hex::encode(mac.finalize().into_bytes()));
    if expected != signature {
        return Err("invalid_signature");
    }
    Ok(())
}

fn verify_postmark(headers: &HeaderMap) -> Result<(), &'static str> {
    let token = env::var("POSTMARK_INBOUND_TOKEN").ok();
    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let header = headers
        .get("x-postmark-token")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_token")?;
    if header != token {
        return Err("invalid_token");
    }
    Ok(())
}

fn verify_bluebubbles(headers: &HeaderMap) -> Result<(), &'static str> {
    let token = env::var("BLUEBUBBLES_WEBHOOK_TOKEN").ok();
    let Some(token) = token.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };
    let header = headers
        .get("x-bluebubbles-token")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_token")?;
    if header != token {
        return Err("invalid_token");
    }
    Ok(())
}

fn verify_twilio(headers: &HeaderMap, body: &[u8]) -> Result<(), &'static str> {
    let token = env::var("TWILIO_AUTH_TOKEN").ok();
    let url = env::var("TWILIO_WEBHOOK_URL").ok();
    let (Some(token), Some(url)) = (token, url) else {
        return Ok(());
    };
    if token.trim().is_empty() || url.trim().is_empty() {
        return Ok(());
    }
    let signature = headers
        .get("x-twilio-signature")
        .and_then(|value| value.to_str().ok())
        .ok_or("missing_signature")?;

    let params: HashMap<String, String> =
        serde_urlencoded::from_bytes(body).map_err(|_| "bad_form")?;
    let mut keys: Vec<_> = params.keys().cloned().collect();
    keys.sort();
    let mut data = url.clone();
    for key in keys {
        if let Some(value) = params.get(&key) {
            data.push_str(&key);
            data.push_str(value);
        }
    }

    let mut mac = Hmac::<Sha1>::new_from_slice(token.as_bytes()).map_err(|_| "bad_secret")?;
    mac.update(data.as_bytes());
    let expected = base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    if expected != signature {
        return Err("invalid_signature");
    }
    Ok(())
}

async fn spawn_discord_gateway(state: Arc<GatewayState>) {
    let token = match env::var("DISCORD_BOT_TOKEN") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return,
    };

    let bot_user_id = env::var("DISCORD_BOT_USER_ID")
        .ok()
        .and_then(|value| value.parse::<u64>().ok());

    let mut bot_user_ids = HashSet::new();
    if let Some(id) = bot_user_id {
        bot_user_ids.insert(id);
    }

    let handler_state = DiscordIngressState {
        state: state.clone(),
        adapter: DiscordInboundAdapter::new(bot_user_ids.clone()),
        bot_user_ids,
    };

    tokio::spawn(async move {
        if let Err(err) = run_discord_gateway(token, handler_state).await {
            error!("discord gateway error: {}", err);
        }
    });
}

struct DiscordIngressState {
    state: Arc<GatewayState>,
    adapter: DiscordInboundAdapter,
    bot_user_ids: HashSet<u64>,
}

struct DiscordIngressHandler {
    inner: Arc<DiscordIngressState>,
}

#[serenity::async_trait]
impl serenity::all::EventHandler for DiscordIngressHandler {
    async fn ready(&self, _ctx: serenity::all::Context, ready: serenity::all::Ready) {
        info!("Discord bot connected as {}", ready.user.name);
    }

    async fn message(&self, _ctx: serenity::all::Context, msg: serenity::all::Message) {
        let inbound = match self.inner.adapter.from_serenity_message(&msg) {
            Ok(message) => message,
            Err(err) => {
                if !err.to_string().contains("ignoring bot") {
                    warn!("gateway discord parse error: {}", err);
                }
                return;
            }
        };

        let is_mention = msg
            .mentions
            .iter()
            .any(|u| self.inner.bot_user_ids.contains(&u.id.get()));
        let is_reply_to_bot = msg
            .referenced_message
            .as_ref()
            .map(|ref_msg| self.inner.bot_user_ids.contains(&ref_msg.author.id.get()))
            .unwrap_or(false);

        if !is_mention && !is_reply_to_bot {
            return;
        }

        let guild_id = inbound
            .metadata
            .discord_guild_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "dm".to_string());

        let route_key = guild_id.clone();
        let Some(route) = resolve_route(Channel::Discord, &route_key, &self.inner.state) else {
            info!("gateway no route for discord guild_id={}", route_key);
            return;
        };

        let external_message_id = inbound.message_id.clone();
        let envelope = build_envelope(
            route,
            Channel::Discord,
            external_message_id,
            &inbound,
            &inbound.raw_payload,
        );

        match self.inner.state.queue.enqueue(&envelope) {
            Ok(result) => {
                if result.inserted {
                    info!("gateway enqueued discord message {}", envelope.envelope_id);
                } else {
                    info!("gateway duplicate discord message {}", envelope.dedupe_key);
                }
            }
            Err(err) => {
                error!("gateway discord enqueue error: {}", err);
            }
        }
    }
}

async fn run_discord_gateway(
    token: String,
    state: DiscordIngressState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let intents = serenity::all::GatewayIntents::GUILD_MESSAGES
        | serenity::all::GatewayIntents::DIRECT_MESSAGES
        | serenity::all::GatewayIntents::MESSAGE_CONTENT;

    let handler = DiscordIngressHandler {
        inner: Arc::new(state),
    };

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    info!("Starting Discord Gateway client...");
    client.start().await?;
    Ok(())
}

fn spawn_google_docs_poller(state: Arc<GatewayState>) {
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
            let envelope = build_envelope(
                route,
                Channel::GoogleDocs,
                external_message_id,
                &message,
                &raw_payload,
            );

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
