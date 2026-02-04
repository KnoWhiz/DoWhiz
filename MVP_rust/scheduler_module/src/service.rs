use axum::body::Bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;
use tracing::{error, info};

use crate::{ModuleExecutor, RunTaskTask, Scheduler, SendEmailTask, TaskKind};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub workspace_root: PathBuf,
    pub scheduler_state_path: PathBuf,
    pub processed_ids_path: PathBuf,
    pub codex_model: String,
    pub codex_disabled: bool,
    pub scheduler_poll_interval: Duration,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, BoxError> {
        dotenvy::dotenv().ok();

        let host = env::var("RUST_SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("RUST_SERVICE_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(9001);

        let workspace_root = resolve_path(env::var("WORKSPACE_ROOT").unwrap_or_else(|_| {
            ".workspace/run_task/workspaces".to_string()
        }))?;
        let scheduler_state_path = resolve_path(env::var("SCHEDULER_STATE_PATH").unwrap_or_else(
            |_| ".workspace/run_task/state/tasks.db".to_string(),
        ))?;
        let processed_ids_path =
            resolve_path(env::var("PROCESSED_IDS_PATH").unwrap_or_else(|_| {
                ".workspace/run_task/state/postmark_processed_ids.txt".to_string()
            }))?;
        let codex_model =
            env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.2-codex".to_string());
        let codex_disabled = env_flag("CODEX_DISABLED", false);
        let scheduler_poll_interval = env::var("SCHEDULER_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(1));

        Ok(Self {
            host,
            port,
            workspace_root,
            scheduler_state_path,
            processed_ids_path,
            codex_model,
            codex_disabled,
            scheduler_poll_interval,
        })
    }
}

#[derive(Clone)]
struct AppState {
    config: Arc<ServiceConfig>,
    dedupe_store: Arc<AsyncMutex<ProcessedMessageStore>>,
    scheduler: Arc<Mutex<Scheduler<ModuleExecutor>>>,
}

pub async fn run_server(
    config: ServiceConfig,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> Result<(), BoxError> {
    let config = Arc::new(config);
    let dedupe_store = ProcessedMessageStore::load(&config.processed_ids_path)?;
    let scheduler = Arc::new(Mutex::new(Scheduler::load(
        &config.scheduler_state_path,
        ModuleExecutor::default(),
    )?));
    let scheduler_stop = Arc::new(AtomicBool::new(false));
    let scheduler_poll_interval = config.scheduler_poll_interval;
    {
        let scheduler = scheduler.clone();
        let scheduler_stop = scheduler_stop.clone();
        thread::spawn(move || {
            while !scheduler_stop.load(Ordering::Relaxed) {
                let tick_result = {
                    let mut scheduler = match scheduler.lock() {
                        Ok(guard) => guard,
                        Err(err) => {
                            error!("scheduler lock poisoned: {}", err);
                            break;
                        }
                    };
                    scheduler.tick()
                };
                if let Err(err) = tick_result {
                    error!("scheduler tick failed: {}", err);
                }
                thread::sleep(scheduler_poll_interval);
            }
        });
    }
    let state = AppState {
        config: config.clone(),
        dedupe_store: Arc::new(AsyncMutex::new(dedupe_store)),
        scheduler,
    };

    let host: IpAddr = config
        .host
        .parse()
        .map_err(|_| format!("invalid host: {}", config.host))?;
    let addr = SocketAddr::new(host, config.port);
    info!("Rust email service listening on {}", addr);

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/postmark/inbound", post(postmark_inbound))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;
    scheduler_stop.store(true, Ordering::Relaxed);
    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn postmark_inbound(
    State(state): State<AppState>,
    body: Bytes,
) -> impl IntoResponse {
    let payload: PostmarkInbound = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"status": "bad_json"})),
            );
        }
    };

    info!("postmark inbound payload received");

    let message_ids = extract_message_ids(&payload, &body);
    let is_new = {
        let mut store = state.dedupe_store.lock().await;
        match store.mark_if_new(&message_ids) {
            Ok(value) => value,
            Err(err) => {
                error!("dedupe store error: {err}");
                true
            }
        }
    };

    if !is_new {
        return (StatusCode::OK, Json(json!({"status": "duplicate"})));
    }

    let config = state.config.clone();
    let scheduler = state.scheduler.clone();
    let payload_clone = payload.clone();
    let body_bytes = body.to_vec();
    tokio::task::spawn_blocking(move || {
        if let Err(err) = process_payload(&config, &scheduler, &payload_clone, &body_bytes) {
            error!("failed to process inbound payload: {err}");
        }
    });

    (StatusCode::OK, Json(json!({"status": "accepted"})))
}

fn process_payload(
    config: &ServiceConfig,
    scheduler: &Arc<Mutex<Scheduler<ModuleExecutor>>>,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    info!("processing inbound payload into workspace");

    let sender = payload.from.as_deref().unwrap_or("").trim();
    if is_blacklisted_sender(sender) {
        info!("skipping blacklisted sender: {}", sender);
        return Ok(());
    }
    let workspace = create_workspace(config, payload, raw_payload)?;
    info!("workspace created at {}", workspace.display());

    let reply_target = payload
        .reply_to
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| payload.from.as_deref())
        .unwrap_or("");
    let to_list = split_recipients(reply_target);
    if to_list.is_empty() {
        return Err("missing reply recipient".into());
    }

    let subject = payload.subject.clone().unwrap_or_default();
    let reply_subject = reply_subject(&subject);

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: config.codex_model.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: to_list.clone(),
    };

    let send_task = SendEmailTask {
        subject: reply_subject,
        html_path: workspace.join("reply_email_draft.html"),
        attachments_dir: workspace.join("reply_email_attachments"),
        to: to_list,
        cc: Vec::new(),
        bcc: Vec::new(),
    };

    {
        let mut scheduler = scheduler
            .lock()
            .map_err(|_| "scheduler lock poisoned")?;
        scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
        scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendEmail(send_task))?;
    }
    info!("scheduler tasks enqueued");

    Ok(())
}

fn is_blacklisted_sender(sender: &str) -> bool {
    if sender.is_empty() {
        return false;
    }
    let mut matched = false;
    let addresses = extract_emails(sender);
    for address in addresses {
        if is_blacklisted_address(&address) {
            matched = true;
            break;
        }
    }
    matched
}

fn is_blacklisted_address(address: &str) -> bool {
    matches!(
        address,
        "agent@dowhiz.com" | "oliver@dowhiz.com"
    )
}

fn extract_emails(raw: &str) -> Vec<String> {
    let mut emails = Vec::new();
    let mut seen = HashSet::new();

    let mut remainder = raw;
    while let Some(start) = remainder.find('<') {
        let after_start = &remainder[start + 1..];
        if let Some(end) = after_start.find('>') {
            let inside = &after_start[..end];
            if let Some(email) = normalize_email(inside) {
                if seen.insert(email.clone()) {
                    emails.push(email);
                }
            }
            remainder = &after_start[end + 1..];
        } else {
            break;
        }
    }

    for token in raw.split(|ch| matches!(ch, ',' | ';' | ' ' | '\t' | '\n' | '\r')) {
        if let Some(email) = normalize_email(token) {
            if seen.insert(email.clone()) {
                emails.push(email);
            }
        }
    }

    emails
}

fn normalize_email(raw: &str) -> Option<String> {
    let mut value = raw.trim();
    if value.is_empty() {
        return None;
    }
    if let Some(stripped) = value.strip_prefix("mailto:") {
        value = stripped.trim();
    }
    value = value.trim_matches(|ch: char| matches!(ch, '<' | '>' | '"' | '\'' | ',' | ';'));
    if !value.contains('@') {
        return None;
    }

    let mut parts = value.splitn(2, '@');
    let local = parts.next().unwrap_or("").trim();
    let domain = parts.next().unwrap_or("").trim();
    if local.is_empty() || domain.is_empty() {
        return None;
    }
    let local = local.split('+').next().unwrap_or(local);

    Some(format!(
        "{}@{}",
        local.to_ascii_lowercase(),
        domain.to_ascii_lowercase()
    ))
}

fn create_workspace(
    config: &ServiceConfig,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
) -> Result<PathBuf, BoxError> {
    fs::create_dir_all(&config.workspace_root)?;

    let fallback = format!("email_{}", Utc::now().timestamp());
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .unwrap_or("");
    let base = sanitize_token(message_id, &fallback);
    let workspace = create_unique_dir(&config.workspace_root, &base)?;

    let incoming_email = workspace.join("incoming_email");
    let incoming_attachments = workspace.join("incoming_attachments");
    let memory = workspace.join("memory");
    let references = workspace.join("references");

    fs::create_dir_all(&incoming_email)?;
    fs::create_dir_all(&incoming_attachments)?;
    fs::create_dir_all(&memory)?;
    fs::create_dir_all(&references)?;

    fs::write(incoming_email.join("postmark_payload.json"), raw_payload)?;
    let email_text = render_email_text(payload);
    fs::write(incoming_email.join("email.txt"), email_text)?;

    if let Some(attachments) = payload.attachments.as_ref() {
        for attachment in attachments {
            let name = sanitize_token(&attachment.name, "attachment");
            let target = incoming_attachments.join(name);
            let data = BASE64_STANDARD
                .decode(attachment.content.as_bytes())
                .unwrap_or_default();
            fs::write(target, data)?;
        }
    }

    Ok(workspace)
}

fn create_unique_dir(root: &Path, base: &str) -> Result<PathBuf, io::Error> {
    let mut candidate = root.join(base);
    if !candidate.exists() {
        fs::create_dir_all(&candidate)?;
        return Ok(candidate);
    }
    for idx in 1..1000 {
        let name = format!("{}_{}", base, idx);
        candidate = root.join(name);
        if !candidate.exists() {
            fs::create_dir_all(&candidate)?;
            return Ok(candidate);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "failed to create unique workspace directory",
    ))
}

fn render_email_text(payload: &PostmarkInbound) -> String {
    let subject = payload.subject.as_deref().unwrap_or("");
    let from = payload.from.as_deref().unwrap_or("");
    let to = payload.to.as_deref().unwrap_or("");
    let cc = payload.cc.as_deref().unwrap_or("");
    let bcc = payload.bcc.as_deref().unwrap_or("");
    let body = payload
        .text_body
        .as_deref()
        .or(payload.stripped_text_reply.as_deref())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.to_string())
        .or_else(|| payload.html_body.as_deref().map(strip_html_tags))
        .unwrap_or_default();

    let attachment_names = payload
        .attachments
        .as_ref()
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    let content_type = item.content_type.trim();
                    if content_type.is_empty() {
                        item.name.clone()
                    } else {
                        format!("{} ({})", item.name, content_type)
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let attachment_line = if attachment_names.is_empty() {
        "(none)".to_string()
    } else {
        attachment_names.join(", ")
    };

    format!(
        "From: {from}\nTo: {to}\nCc: {cc}\nBcc: {bcc}\nSubject: {subject}\nAttachments: {attachment_line}\n\n{body}\n"
    )
}

fn strip_html_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let trimmed = value.trim().trim_start_matches('<').trim_end_matches('>');
    let mut out = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    let cleaned = out.trim_matches(&['.', '_', '-'][..]);
    if cleaned.is_empty() {
        fallback.to_string()
    } else {
        cleaned.to_string()
    }
}

fn split_recipients(value: &str) -> Vec<String> {
    value
        .split(|ch| ch == ',' || ch == ';')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}

fn reply_subject(original: &str) -> String {
    let trimmed = original.trim();
    if trimmed.is_empty() {
        "Re: (no subject)".to_string()
    } else if trimmed.to_lowercase().starts_with("re:") {
        trimmed.to_string()
    } else {
        format!("Re: {}", trimmed)
    }
}

fn env_flag(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(
            value.trim().to_lowercase().as_str(),
            "1" | "true" | "yes" | "y"
        ),
        Err(_) => default,
    }
}

fn resolve_path(raw: String) -> Result<PathBuf, io::Error> {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Ok(path)
    } else {
        let cwd = env::current_dir()?;
        Ok(cwd.join(path))
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkInbound {
    #[serde(rename = "From")]
    from: Option<String>,
    #[serde(rename = "To")]
    to: Option<String>,
    #[serde(rename = "Cc")]
    cc: Option<String>,
    #[serde(rename = "Bcc")]
    bcc: Option<String>,
    #[serde(rename = "ReplyTo")]
    reply_to: Option<String>,
    #[serde(rename = "Subject")]
    subject: Option<String>,
    #[serde(rename = "TextBody")]
    text_body: Option<String>,
    #[serde(rename = "StrippedTextReply")]
    stripped_text_reply: Option<String>,
    #[serde(rename = "HtmlBody")]
    html_body: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    message_id: Option<String>,
    #[serde(rename = "Headers")]
    headers: Option<Vec<PostmarkHeader>>,
    #[serde(rename = "Attachments")]
    attachments: Option<Vec<PostmarkAttachment>>,
}

impl PostmarkInbound {
    fn header_message_id(&self) -> Option<&str> {
        self.headers.as_ref().and_then(|headers| {
            headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case("message-id"))
                .map(|header| header.value.as_str())
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkHeader {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkAttachment {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Content")]
    content: String,
    #[serde(rename = "ContentType")]
    content_type: String,
}

fn extract_message_ids(payload: &PostmarkInbound, raw_payload: &[u8]) -> Vec<String> {
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    if let Some(header_id) = payload
        .header_message_id()
        .and_then(normalize_message_id)
    {
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

struct ProcessedMessageStore {
    path: PathBuf,
    seen: HashSet<String>,
}

impl ProcessedMessageStore {
    fn load(path: &Path) -> Result<Self, io::Error> {
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

    fn mark_if_new(&mut self, ids: &[String]) -> Result<bool, io::Error> {
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
