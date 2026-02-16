use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderMap, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use serde::Deserialize;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use scheduler_module::adapters::postmark::PostmarkInboundPayload;
use scheduler_module::employee_config::{load_employee_directory, EmployeeDirectory};
use scheduler_module::mailbox;

#[derive(Debug, Deserialize, Default)]
struct GatewayConfigFile {
    #[serde(default)]
    server: GatewayServerConfig,
    #[serde(default)]
    dedupe: GatewayDedupeConfig,
    #[serde(default)]
    targets: HashMap<String, String>,
    #[serde(default)]
    slack: SlackRouteConfig,
}

#[derive(Debug, Deserialize, Default)]
struct GatewayServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Deserialize, Default)]
struct GatewayDedupeConfig {
    path: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Default)]
struct SlackRouteConfig {
    default_employee_id: Option<String>,
    #[serde(default)]
    team_to_employee: HashMap<String, String>,
}

#[derive(Clone)]
struct GatewayConfig {
    host: String,
    port: u16,
    dedupe_path: PathBuf,
    targets: HashMap<String, String>,
    slack_default_employee_id: Option<String>,
    slack_team_map: HashMap<String, String>,
}

#[derive(Clone)]
struct GatewayState {
    client: reqwest::Client,
    config: GatewayConfig,
    employee_directory: EmployeeDirectory,
    address_to_employee: HashMap<String, String>,
    dedupe_store: Arc<Mutex<ProcessedMessageStore>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChannelKind {
    Email,
    Slack,
    Other,
}

#[derive(Debug, Clone)]
struct RouteDecision {
    employee_id: String,
    dedupe_keys: Vec<String>,
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

    let dedupe_path = env::var("GATEWAY_DEDUPE_PATH")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| config_file.dedupe.path.unwrap_or_else(default_dedupe_path));

    let targets = normalize_targets(&config_file.targets)?;
    if targets.is_empty() {
        return Err("gateway config must include at least one target".into());
    }

    let dedupe_store = ProcessedMessageStore::load(&dedupe_path)?;

    let state = Arc::new(GatewayState {
        client: reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()?,
        config: GatewayConfig {
            host: host.clone(),
            port,
            dedupe_path,
            targets,
            slack_default_employee_id: config_file.slack.default_employee_id,
            slack_team_map: config_file.slack.team_to_employee,
        },
        employee_directory,
        address_to_employee,
        dedupe_store: Arc::new(Mutex::new(dedupe_store)),
    });

    info!(
        "inbound gateway config path={}, host={}, port={}, dedupe_path={}",
        config_path.display(),
        host,
        port,
        state.config.dedupe_path.display()
    );

    let app = Router::new()
        .route("/health", any(health))
        .fallback(any(forward_request))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    info!("inbound gateway listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn forward_request(
    State(state): State<Arc<GatewayState>>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let path = uri
        .path_and_query()
        .map(|value| value.as_str())
        .unwrap_or(uri.path());

    let channel = channel_from_path(path);
    if channel == ChannelKind::Other {
        warn!("gateway unsupported path: {}", path);
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"status":"unsupported_path"})),
        )
            .into_response();
    }

    let route = match resolve_route(channel, &state, &body) {
        Ok(Some(route)) => route,
        Ok(None) => {
            return (StatusCode::OK, Json(json!({"status":"no_route"}))).into_response();
        }
        Err(err) => {
            warn!("gateway route error: {}", err);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"status":"bad_request"})),
            )
                .into_response();
        }
    };

    if !route.dedupe_keys.is_empty() {
        let mut store = state.dedupe_store.lock().await;
        match store.mark_if_new(&route.dedupe_keys) {
            Ok(true) => {}
            Ok(false) => {
                return (StatusCode::OK, Json(json!({"status":"duplicate"}))).into_response();
            }
            Err(err) => {
                warn!("gateway dedupe error: {}", err);
            }
        }
    }

    let target_base = match state.config.targets.get(&route.employee_id) {
        Some(url) => url,
        None => {
            warn!(
                "gateway missing target for employee_id={}",
                route.employee_id
            );
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status":"missing_target"})),
            )
                .into_response();
        }
    };

    let target_url = format!("{}{}", target_base, path);

    info!(
        "gateway forwarding channel={:?} employee_id={} target={} method={}",
        channel, route.employee_id, target_url, method
    );

    forward_to_target(&state.client, method, &target_url, headers, body).await
}

async fn forward_to_target(
    client: &reqwest::Client,
    method: Method,
    url: &str,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let req_method = match reqwest::Method::from_bytes(method.as_str().as_bytes()) {
        Ok(method) => method,
        Err(_) => {
            warn!("gateway unsupported method {}", method);
            return (
                StatusCode::METHOD_NOT_ALLOWED,
                Json(json!({"status":"bad_method"})),
            )
                .into_response();
        }
    };

    let mut request = client.request(req_method, url).body(body);
    for (name, value) in headers.iter() {
        if should_skip_header(name) {
            continue;
        }
        let header_name = match reqwest::header::HeaderName::from_bytes(name.as_str().as_bytes()) {
            Ok(name) => name,
            Err(_) => continue,
        };
        let header_value = match reqwest::header::HeaderValue::from_bytes(value.as_bytes()) {
            Ok(value) => value,
            Err(_) => continue,
        };
        request = request.header(header_name, header_value);
    }

    match request.send().await {
        Ok(response) => {
            let status =
                StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(|value| value.to_string());
            let body = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(err) => {
                    warn!("gateway response read failed: {}", err);
                    return (
                        StatusCode::BAD_GATEWAY,
                        Json(json!({"status":"bad_gateway"})),
                    )
                        .into_response();
                }
            };

            let mut builder = Response::builder().status(status);
            if let Some(content_type) = content_type {
                if let Ok(value) = header::HeaderValue::from_str(&content_type) {
                    builder = builder.header(header::CONTENT_TYPE, value);
                }
            }

            match builder.body(axum::body::Body::from(body)) {
                Ok(resp) => resp,
                Err(err) => {
                    error!("gateway response build failed: {}", err);
                    StatusCode::BAD_GATEWAY.into_response()
                }
            }
        }
        Err(err) => {
            warn!("gateway forward failed: {}", err);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"status":"bad_gateway"})),
            )
                .into_response()
        }
    }
}

fn resolve_route(
    channel: ChannelKind,
    state: &GatewayState,
    body: &[u8],
) -> Result<Option<RouteDecision>, String> {
    match channel {
        ChannelKind::Email => resolve_email_route(state, body),
        ChannelKind::Slack => resolve_slack_route(state, body),
        ChannelKind::Other => Ok(None),
    }
}

fn resolve_email_route(state: &GatewayState, body: &[u8]) -> Result<Option<RouteDecision>, String> {
    let payload: PostmarkInboundPayload =
        serde_json::from_slice(body).map_err(|err| format!("invalid postmark payload: {}", err))?;

    let address = find_service_address(&payload, &state.employee_directory.service_addresses);
    let Some(address) = address else {
        info!("gateway no service address found in postmark payload");
        return Ok(None);
    };
    let normalized = address.to_ascii_lowercase();
    let employee_id = match state.address_to_employee.get(&normalized) {
        Some(id) => id.clone(),
        None => {
            info!("gateway no employee mapped for address={}", normalized);
            return Ok(None);
        }
    };

    let message_ids = extract_message_ids(&payload, body);
    let dedupe_keys = message_ids
        .into_iter()
        .map(|id| format!("email:{}:{}", employee_id, id))
        .collect();

    Ok(Some(RouteDecision {
        employee_id,
        dedupe_keys,
    }))
}

fn resolve_slack_route(state: &GatewayState, body: &[u8]) -> Result<Option<RouteDecision>, String> {
    let wrapper: serde_json::Value =
        serde_json::from_slice(body).map_err(|err| format!("invalid slack payload: {}", err))?;
    let team_id = wrapper
        .get("team_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());
    let event_id = wrapper
        .get("event_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());

    let employee_id = team_id
        .as_ref()
        .and_then(|id| state.config.slack_team_map.get(id))
        .cloned()
        .or_else(|| state.config.slack_default_employee_id.clone());

    let Some(employee_id) = employee_id else {
        info!("gateway no slack route configured");
        return Ok(None);
    };

    let dedupe_keys = event_id
        .into_iter()
        .map(|id| format!("slack:{}:{}", employee_id, id))
        .collect();

    Ok(Some(RouteDecision {
        employee_id,
        dedupe_keys,
    }))
}

fn channel_from_path(path: &str) -> ChannelKind {
    if path.starts_with("/postmark/inbound") {
        ChannelKind::Email
    } else if path.starts_with("/slack/") {
        ChannelKind::Slack
    } else {
        ChannelKind::Other
    }
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
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read gateway config: {}", err))?;
    toml::from_str::<GatewayConfigFile>(&content)
        .map_err(|err| format!("failed to parse gateway config: {}", err))
}

fn normalize_targets(raw: &HashMap<String, String>) -> Result<HashMap<String, String>, String> {
    let mut targets = HashMap::new();
    for (employee_id, url) in raw {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err(format!("gateway target for {} is empty", employee_id));
        }
        let normalized = trimmed.trim_end_matches('/').to_string();
        targets.insert(employee_id.clone(), normalized);
    }
    Ok(targets)
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

fn extract_message_ids(payload: &PostmarkInboundPayload, raw_payload: &[u8]) -> Vec<String> {
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    if let Some(header_id) = payload.header_message_id().and_then(normalize_message_id) {
        if seen.insert(header_id.clone()) {
            ids.push(header_id);
        }
    }
    if let Some(message_id) = payload
        .message_id
        .as_ref()
        .and_then(|value| normalize_message_id(value))
    {
        if seen.insert(message_id.clone()) {
            ids.push(message_id);
        }
    }
    let fallback = format!("{:x}", md5::compute(raw_payload));
    if seen.insert(fallback.clone()) {
        ids.push(fallback);
    }
    ids
}

fn normalize_message_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches(|ch| matches!(ch, '<' | '>'));
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

fn should_skip_header(name: &header::HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection" | "host" | "content-length"
    )
}

fn default_dedupe_path() -> PathBuf {
    let base = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join(".dowhiz")
        .join("DoWhiz")
        .join("gateway")
        .join("state")
        .join("processed_ids.txt")
}

struct ProcessedMessageStore {
    path: PathBuf,
    seen: HashSet<String>,
}

impl ProcessedMessageStore {
    fn load(path: &Path) -> Result<Self, std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut seen = HashSet::new();
        if path.exists() {
            for raw in fs::read_to_string(path)?.lines() {
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

        let mut handle = fs::OpenOptions::new()
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

#[derive(serde::Serialize)]
struct Json<T>(T);

impl<T> IntoResponse for Json<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self.0) {
            Ok(body) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                body,
            )
                .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
