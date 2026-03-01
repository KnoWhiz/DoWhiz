use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use tracing::{error, info};
use uuid::Uuid;

use crate::raw_payload_store;

const DEFAULT_ADMIN_EMAIL: &str = "admin@dowhiz.com";
const DEFAULT_FROM_EMAIL: &str = "noreply@dowhiz.com";
const DEFAULT_POSTMARK_API_BASE: &str = "https://api.postmarkapp.com";
const DEFAULT_MESSAGE_STREAM: &str = "outbound";
const DEFAULT_MAX_UPLOAD_BYTES: usize = 10 * 1024 * 1024;
const MAX_TEXT_FIELD_LEN: usize = 20_000;
const MAX_FILES: usize = 400;

#[derive(Clone)]
pub struct AgentMarketState {
    admin_email: String,
    from_email: String,
    postmark_token: Option<String>,
    postmark_api_base: String,
    message_stream: String,
    max_upload_bytes: usize,
}

impl AgentMarketState {
    pub fn from_env() -> Self {
        let max_upload_bytes = std::env::var("AGENT_MARKET_MAX_UPLOAD_BYTES")
            .ok()
            .and_then(|raw| raw.trim().parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_MAX_UPLOAD_BYTES);

        Self {
            admin_email: read_env_trimmed("AGENT_MARKET_ADMIN_EMAIL")
                .unwrap_or_else(|| DEFAULT_ADMIN_EMAIL.to_string()),
            from_email: read_env_trimmed("AGENT_MARKET_FROM_EMAIL")
                .or_else(|| read_env_trimmed("POSTMARK_FROM_EMAIL"))
                .unwrap_or_else(|| DEFAULT_FROM_EMAIL.to_string()),
            postmark_token: read_env_trimmed("POSTMARK_SERVER_TOKEN"),
            postmark_api_base: read_env_trimmed("POSTMARK_API_BASE")
                .unwrap_or_else(|| DEFAULT_POSTMARK_API_BASE.to_string()),
            message_stream: read_env_trimmed("POSTMARK_MESSAGE_STREAM")
                .unwrap_or_else(|| DEFAULT_MESSAGE_STREAM.to_string()),
            max_upload_bytes,
        }
    }
}

#[derive(Debug, Serialize)]
struct AgentMarketAcceptedResponse {
    status: &'static str,
    request_id: String,
    eta_hours: u16,
}

#[derive(Debug, Serialize)]
struct AgentMarketErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct EnvKeyEntry {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
enum UploadCategory {
    #[serde(rename = "skills")]
    Skills,
    #[serde(rename = "private_data")]
    PrivateData,
}

impl UploadCategory {
    fn from_field_name(field_name: &str) -> Option<Self> {
        match field_name {
            "skills_files" => Some(Self::Skills),
            "private_data_files" => Some(Self::PrivateData),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Skills => "skills",
            Self::PrivateData => "private_data",
        }
    }
}

#[derive(Debug)]
struct UploadedFile {
    category: UploadCategory,
    relative_path: String,
    content_type: String,
    size_bytes: usize,
    sha256: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Serialize)]
struct StoredUploadFile {
    category: UploadCategory,
    relative_path: String,
    content_type: String,
    size_bytes: usize,
    sha256: String,
    content_base64: String,
}

#[derive(Debug, Serialize)]
struct StoredOperatorPayload {
    full_name: String,
    work_email: String,
    team_name: String,
}

#[derive(Debug, Serialize)]
struct StoredDeploymentPayload {
    agent_name: String,
    azure_region: String,
    wallet_id: String,
    contact_channel: String,
    workspace_paths: String,
    use_case: String,
}

#[derive(Debug, Serialize)]
struct StoredSubmissionPayload {
    schema_version: &'static str,
    request_id: String,
    submitted_at: String,
    operator: StoredOperatorPayload,
    deployment: StoredDeploymentPayload,
    env_keys: Vec<EnvKeyEntry>,
    files: Vec<StoredUploadFile>,
}

#[derive(Debug, Serialize)]
struct EmailManifest {
    schema_version: &'static str,
    request_id: String,
    submitted_at: String,
    operator: StoredOperatorPayload,
    deployment: StoredDeploymentPayload,
    env_key_names: Vec<String>,
    upload_summary: UploadSummary,
    storage_reference: String,
}

#[derive(Debug, Serialize)]
struct UploadSummary {
    total_file_count: usize,
    total_size_bytes: usize,
    skills_file_count: usize,
    private_data_file_count: usize,
    files: Vec<UploadFileSummary>,
}

#[derive(Debug, Serialize)]
struct UploadFileSummary {
    category: UploadCategory,
    relative_path: String,
    size_bytes: usize,
    sha256: String,
    content_type: String,
}

type AgentMarketResult<T> = Result<T, (StatusCode, Json<AgentMarketErrorResponse>)>;

pub fn agent_market_router(state: AgentMarketState) -> Router {
    Router::new()
        .route(
            "/api/agent-market/deploy",
            post(submit_agent_market_request),
        )
        .with_state(state)
}

async fn submit_agent_market_request(
    State(state): State<AgentMarketState>,
    mut multipart: Multipart,
) -> AgentMarketResult<(StatusCode, Json<AgentMarketAcceptedResponse>)> {
    let mut text_fields: HashMap<String, String> = HashMap::new();
    let mut uploaded_files: Vec<UploadedFile> = Vec::new();
    let mut total_upload_bytes = 0usize;

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        error!("agent market multipart parse failed: {}", err);
        bad_request("Invalid upload payload.")
    })? {
        let field_name = field.name().unwrap_or("").to_string();
        if field_name.is_empty() {
            continue;
        }

        if let Some(category) = UploadCategory::from_field_name(&field_name) {
            if uploaded_files.len() >= MAX_FILES {
                return Err(bad_request("Too many uploaded files in one request."));
            }

            let file_name = field.file_name().unwrap_or("upload.bin");
            let relative_path = normalize_relative_path(file_name);
            let content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();

            let bytes = field.bytes().await.map_err(|err| {
                error!("agent market file read failed: {}", err);
                bad_request("Failed to read one of the uploaded files.")
            })?;

            if bytes.is_empty() {
                continue;
            }

            total_upload_bytes = total_upload_bytes.saturating_add(bytes.len());
            if total_upload_bytes > state.max_upload_bytes {
                return Err(bad_request(format!(
                    "Total upload size exceeds {} bytes.",
                    state.max_upload_bytes
                )));
            }

            let bytes_vec = bytes.to_vec();
            let sha256 = sha256_hex(&bytes_vec);
            uploaded_files.push(UploadedFile {
                category,
                relative_path,
                content_type,
                size_bytes: bytes_vec.len(),
                sha256,
                bytes: bytes_vec,
            });
            continue;
        }

        let value = field.text().await.map_err(|err| {
            error!("agent market text field read failed: {}", err);
            bad_request("Invalid form field in request.")
        })?;
        if value.len() > MAX_TEXT_FIELD_LEN {
            return Err(bad_request(format!("Field '{}' is too long.", field_name)));
        }
        text_fields.insert(field_name, value.trim().to_string());
    }

    let full_name = required_text_field(&text_fields, "full_name")?;
    let work_email = required_text_field(&text_fields, "work_email")?;
    let team_name = optional_text_field(&text_fields, "team_name");
    let azure_region = required_text_field(&text_fields, "azure_region")?;
    let agent_name = required_text_field(&text_fields, "agent_name")?;
    let wallet_id = optional_text_field(&text_fields, "wallet_id");
    let contact_channel = required_text_field(&text_fields, "contact_channel")?;
    let workspace_paths = required_text_field(&text_fields, "workspace_paths")?;
    let use_case = required_text_field(&text_fields, "use_case")?;
    let env_keys_raw = required_text_field(&text_fields, "env_keys_json")?;

    if !looks_like_email(&work_email) {
        return Err(bad_request("Work email is invalid."));
    }

    let mut env_keys: Vec<EnvKeyEntry> = serde_json::from_str(&env_keys_raw).map_err(|_| {
        bad_request("Environment keys format is invalid. Please retry the submission.")
    })?;
    normalize_and_validate_env_keys(&mut env_keys)?;

    let skills_count = uploaded_files
        .iter()
        .filter(|file| matches!(file.category, UploadCategory::Skills))
        .count();
    let private_data_count = uploaded_files
        .iter()
        .filter(|file| matches!(file.category, UploadCategory::PrivateData))
        .count();
    if skills_count == 0 {
        return Err(bad_request("At least one skills file is required."));
    }
    if private_data_count == 0 {
        return Err(bad_request("At least one private data file is required."));
    }

    let request_uuid = Uuid::new_v4();
    let request_id = format!("am-{}", request_uuid.simple());
    let submitted_at = Utc::now();
    let submitted_at_iso = submitted_at.to_rfc3339();

    let operator = StoredOperatorPayload {
        full_name,
        work_email,
        team_name,
    };
    let deployment = StoredDeploymentPayload {
        agent_name,
        azure_region,
        wallet_id,
        contact_channel,
        workspace_paths,
        use_case,
    };

    let stored_files = uploaded_files
        .iter()
        .map(|file| StoredUploadFile {
            category: file.category,
            relative_path: file.relative_path.clone(),
            content_type: file.content_type.clone(),
            size_bytes: file.size_bytes,
            sha256: file.sha256.clone(),
            content_base64: BASE64_STANDARD.encode(&file.bytes),
        })
        .collect::<Vec<_>>();

    let stored_payload = StoredSubmissionPayload {
        schema_version: "agent_market_deploy.v1",
        request_id: request_id.clone(),
        submitted_at: submitted_at_iso.clone(),
        operator: StoredOperatorPayload {
            full_name: operator.full_name.clone(),
            work_email: operator.work_email.clone(),
            team_name: operator.team_name.clone(),
        },
        deployment: StoredDeploymentPayload {
            agent_name: deployment.agent_name.clone(),
            azure_region: deployment.azure_region.clone(),
            wallet_id: deployment.wallet_id.clone(),
            contact_channel: deployment.contact_channel.clone(),
            workspace_paths: deployment.workspace_paths.clone(),
            use_case: deployment.use_case.clone(),
        },
        env_keys: env_keys.clone(),
        files: stored_files,
    };

    let stored_payload_json = serde_json::to_vec(&stored_payload).map_err(|err| {
        error!("agent market payload serialize failed: {}", err);
        internal_error("Unable to process deployment request.")
    })?;

    let storage_reference =
        raw_payload_store::upload_raw_payload(request_uuid, submitted_at, &stored_payload_json)
            .await
            .map_err(|err| {
                error!("agent market payload upload failed: {}", err);
                internal_error(
                    "Deployment intake is temporarily unavailable. Please retry in a few minutes.",
                )
            })?;

    let upload_summary = UploadSummary {
        total_file_count: uploaded_files.len(),
        total_size_bytes: total_upload_bytes,
        skills_file_count: skills_count,
        private_data_file_count: private_data_count,
        files: uploaded_files
            .iter()
            .map(|file| UploadFileSummary {
                category: file.category,
                relative_path: file.relative_path.clone(),
                size_bytes: file.size_bytes,
                sha256: file.sha256.clone(),
                content_type: file.content_type.clone(),
            })
            .collect(),
    };

    let manifest = EmailManifest {
        schema_version: "agent_market_manifest.v1",
        request_id: request_id.clone(),
        submitted_at: submitted_at_iso.clone(),
        operator: StoredOperatorPayload {
            full_name: operator.full_name.clone(),
            work_email: operator.work_email.clone(),
            team_name: operator.team_name.clone(),
        },
        deployment: StoredDeploymentPayload {
            agent_name: deployment.agent_name.clone(),
            azure_region: deployment.azure_region.clone(),
            wallet_id: deployment.wallet_id.clone(),
            contact_channel: deployment.contact_channel.clone(),
            workspace_paths: deployment.workspace_paths.clone(),
            use_case: deployment.use_case.clone(),
        },
        env_key_names: env_keys.iter().map(|entry| entry.key.clone()).collect(),
        upload_summary,
        storage_reference: storage_reference.clone(),
    };
    let manifest_json = serde_json::to_vec_pretty(&manifest).map_err(|err| {
        error!("agent market manifest serialize failed: {}", err);
        internal_error("Unable to process deployment request.")
    })?;

    let subject = format!(
        "[Agent Market Deploy Request] {} ({})",
        deployment.agent_name, request_id
    );
    let text_body = build_admin_text_body(&manifest);
    let html_body = build_admin_html_body(&manifest);

    send_admin_email(
        &state,
        &subject,
        &text_body,
        &html_body,
        &manifest_json,
        &request_id,
    )
    .await
    .map_err(|err| {
        error!("agent market admin notify failed: {}", err);
        internal_error("Deployment intake is temporarily unavailable. Please retry.")
    })?;

    info!(
        "agent market request accepted request_id={}, reference={}, file_count={}, total_bytes={}",
        request_id,
        storage_reference,
        uploaded_files.len(),
        total_upload_bytes
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(AgentMarketAcceptedResponse {
            status: "accepted",
            request_id,
            eta_hours: 24,
        }),
    ))
}

fn read_env_trimmed(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn required_text_field(fields: &HashMap<String, String>, key: &str) -> AgentMarketResult<String> {
    match fields
        .get(key)
        .map(|value| value.trim())
        .filter(|v| !v.is_empty())
    {
        Some(value) => Ok(value.to_string()),
        None => Err(bad_request(format!("Missing required field '{}'.", key))),
    }
}

fn optional_text_field(fields: &HashMap<String, String>, key: &str) -> String {
    fields
        .get(key)
        .map(|value| value.trim().to_string())
        .unwrap_or_default()
}

fn normalize_and_validate_env_keys(env_keys: &mut [EnvKeyEntry]) -> AgentMarketResult<()> {
    if env_keys.is_empty() {
        return Err(bad_request("At least one environment key is required."));
    }

    let mut seen = HashSet::new();
    for entry in env_keys.iter_mut() {
        entry.key = entry.key.trim().to_uppercase();
        entry.value = entry.value.trim().to_string();

        if !is_valid_env_key(&entry.key) {
            return Err(bad_request(format!(
                "Invalid environment key '{}'.",
                entry.key
            )));
        }
        if entry.value.is_empty() {
            return Err(bad_request(format!(
                "Environment key '{}' has an empty value.",
                entry.key
            )));
        }
        if !seen.insert(entry.key.clone()) {
            return Err(bad_request(format!(
                "Duplicate environment key '{}'.",
                entry.key
            )));
        }
    }
    Ok(())
}

fn is_valid_env_key(key: &str) -> bool {
    let mut chars = key.chars();
    match chars.next() {
        Some(first) if first.is_ascii_uppercase() => {}
        _ => return false,
    }
    chars.all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_')
}

fn looks_like_email(value: &str) -> bool {
    let trimmed = value.trim();
    let mut parts = trimmed.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();
    parts.next().is_none()
        && !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !trimmed.contains(' ')
}

fn normalize_relative_path(input: &str) -> String {
    let normalized = input.replace('\\', "/");
    let mut segments = Vec::new();
    for segment in normalized.split('/') {
        let cleaned = segment.trim();
        if cleaned.is_empty() || cleaned == "." {
            continue;
        }
        if cleaned == ".." {
            let _ = segments.pop();
            continue;
        }
        let safe = cleaned
            .chars()
            .filter(|ch| !ch.is_control())
            .collect::<String>();
        if safe.is_empty() {
            continue;
        }
        segments.push(safe);
    }

    if segments.is_empty() {
        "upload.bin".to_string()
    } else {
        segments.join("/")
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn build_admin_text_body(manifest: &EmailManifest) -> String {
    format!(
        "Agent Market deploy request\n\nRequest ID: {}\nSubmitted at: {}\nOperator: {} <{}>\nTeam: {}\nAgent name: {}\nAzure region: {}\nWallet: {}\nContact channel: {}\nWorkspace paths: {}\n\nEnv key names: {}\nFile count: {}\nTotal bytes: {}\nStorage reference: {}\n\nThis request payload (including env values + uploaded files) is stored in raw payload storage under the reference above.",
        manifest.request_id,
        manifest.submitted_at,
        manifest.operator.full_name,
        manifest.operator.work_email,
        if manifest.operator.team_name.is_empty() {
            "N/A"
        } else {
            &manifest.operator.team_name
        },
        manifest.deployment.agent_name,
        manifest.deployment.azure_region,
        if manifest.deployment.wallet_id.is_empty() {
            "N/A"
        } else {
            &manifest.deployment.wallet_id
        },
        manifest.deployment.contact_channel,
        manifest.deployment.workspace_paths,
        manifest.env_key_names.join(", "),
        manifest.upload_summary.total_file_count,
        manifest.upload_summary.total_size_bytes,
        manifest.storage_reference
    )
}

fn build_admin_html_body(manifest: &EmailManifest) -> String {
    let env_keys = if manifest.env_key_names.is_empty() {
        "N/A".to_string()
    } else {
        manifest
            .env_key_names
            .iter()
            .map(|entry| escape_html(entry))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let mut files_html = String::new();
    for file in &manifest.upload_summary.files {
        files_html.push_str(&format!(
            "<li><strong>{}</strong> · {} · {} bytes · {}</li>",
            file.category.as_str(),
            escape_html(&file.relative_path),
            file.size_bytes,
            escape_html(&file.content_type)
        ));
    }

    format!(
        r#"<!doctype html>
<html>
  <body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; padding: 24px; background: #f5f7fb; color: #0f172a;">
    <div style="max-width: 760px; margin: 0 auto; background: #ffffff; border: 1px solid #dbe2ec; border-radius: 12px; padding: 24px;">
      <h2 style="margin-top: 0;">Agent Market Deploy Request</h2>
      <p><strong>Request ID:</strong> {request_id}</p>
      <p><strong>Submitted at:</strong> {submitted_at}</p>
      <p><strong>Operator:</strong> {operator_name} &lt;{operator_email}&gt;</p>
      <p><strong>Team:</strong> {team_name}</p>
      <p><strong>Agent:</strong> {agent_name}</p>
      <p><strong>Azure region:</strong> {azure_region}</p>
      <p><strong>Wallet:</strong> {wallet}</p>
      <p><strong>Contact channel:</strong> {contact_channel}</p>
      <p><strong>Workspace paths:</strong> {workspace_paths}</p>
      <p><strong>Env key names:</strong> {env_keys}</p>
      <p><strong>Total files:</strong> {file_count} ({total_bytes} bytes)</p>
      <p><strong>Storage reference:</strong> <code>{storage_reference}</code></p>
      <h3>Uploaded file summary</h3>
      <ul>{files_html}</ul>
      <p style="margin-top: 18px; color: #475569;">
        The full request payload (including env values and file contents) is stored in raw payload storage.
      </p>
    </div>
  </body>
</html>"#,
        request_id = escape_html(&manifest.request_id),
        submitted_at = escape_html(&manifest.submitted_at),
        operator_name = escape_html(&manifest.operator.full_name),
        operator_email = escape_html(&manifest.operator.work_email),
        team_name = if manifest.operator.team_name.is_empty() {
            "N/A".to_string()
        } else {
            escape_html(&manifest.operator.team_name)
        },
        agent_name = escape_html(&manifest.deployment.agent_name),
        azure_region = escape_html(&manifest.deployment.azure_region),
        wallet = if manifest.deployment.wallet_id.is_empty() {
            "N/A".to_string()
        } else {
            escape_html(&manifest.deployment.wallet_id)
        },
        contact_channel = escape_html(&manifest.deployment.contact_channel),
        workspace_paths = escape_html(&manifest.deployment.workspace_paths),
        env_keys = env_keys,
        file_count = manifest.upload_summary.total_file_count,
        total_bytes = manifest.upload_summary.total_size_bytes,
        storage_reference = escape_html(&manifest.storage_reference),
        files_html = files_html
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

async fn send_admin_email(
    state: &AgentMarketState,
    subject: &str,
    text_body: &str,
    html_body: &str,
    manifest_bytes: &[u8],
    request_id: &str,
) -> Result<(), String> {
    let postmark_token = state
        .postmark_token
        .clone()
        .ok_or_else(|| "POSTMARK_SERVER_TOKEN is not configured".to_string())?;
    let endpoint = format!("{}/email", state.postmark_api_base.trim_end_matches('/'));

    let attachment_name = format!("agent-market-{}-manifest.json", request_id);
    let payload = serde_json::json!({
        "From": state.from_email,
        "To": state.admin_email,
        "Subject": subject,
        "HtmlBody": html_body,
        "TextBody": text_body,
        "MessageStream": state.message_stream,
        "Attachments": [{
            "Name": attachment_name,
            "Content": BASE64_STANDARD.encode(manifest_bytes),
            "ContentType": "application/json"
        }]
    });

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .header("X-Postmark-Server-Token", postmark_token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|err| format!("postmark request failed: {}", err))?;

    if response.status().is_success() {
        return Ok(());
    }

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    Err(format!(
        "postmark send failed (status {}): {}",
        status, body
    ))
}

fn bad_request(message: impl Into<String>) -> (StatusCode, Json<AgentMarketErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(AgentMarketErrorResponse {
            error: message.into(),
        }),
    )
}

fn internal_error(message: impl Into<String>) -> (StatusCode, Json<AgentMarketErrorResponse>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(AgentMarketErrorResponse {
            error: message.into(),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        is_valid_env_key, looks_like_email, normalize_and_validate_env_keys, normalize_relative_path,
        EnvKeyEntry,
    };

    #[test]
    fn env_key_validation_accepts_uppercase_tokens() {
        assert!(is_valid_env_key("OPENAI_API_KEY"));
        assert!(is_valid_env_key("X1"));
        assert!(is_valid_env_key("A_B_C_2"));
    }

    #[test]
    fn env_key_validation_rejects_invalid_tokens() {
        assert!(!is_valid_env_key("openai_key"));
        assert!(!is_valid_env_key("1OPENAI"));
        assert!(!is_valid_env_key("OPENAI-KEY"));
        assert!(!is_valid_env_key(""));
    }

    #[test]
    fn relative_path_normalization_strips_traversal_segments() {
        assert_eq!(
            normalize_relative_path("../secret/../../foo/bar.txt"),
            "foo/bar.txt"
        );
        assert_eq!(
            normalize_relative_path(r"folder\sub\file.json"),
            "folder/sub/file.json"
        );
        assert_eq!(normalize_relative_path(""), "upload.bin");
    }

    #[test]
    fn env_key_normalization_trims_and_uppercases() {
        let mut entries = vec![EnvKeyEntry {
            key: "  openai_api_key  ".to_string(),
            value: "  sk-123  ".to_string(),
        }];

        let result = normalize_and_validate_env_keys(&mut entries);
        assert!(result.is_ok());
        assert_eq!(entries[0].key, "OPENAI_API_KEY");
        assert_eq!(entries[0].value, "sk-123");
    }

    #[test]
    fn env_key_validation_rejects_duplicate_after_normalization() {
        let mut entries = vec![
            EnvKeyEntry {
                key: "openai_api_key".to_string(),
                value: "value-1".to_string(),
            },
            EnvKeyEntry {
                key: " OPENAI_API_KEY ".to_string(),
                value: "value-2".to_string(),
            },
        ];

        let result = normalize_and_validate_env_keys(&mut entries);
        assert!(result.is_err());
    }

    #[test]
    fn email_validation_accepts_reasonable_formats() {
        assert!(looks_like_email("person@dowhiz.com"));
        assert!(looks_like_email("first.last@team.example"));
    }

    #[test]
    fn email_validation_rejects_invalid_formats() {
        assert!(!looks_like_email("no-at-symbol"));
        assert!(!looks_like_email("missing-domain@"));
        assert!(!looks_like_email("@missing-local.com"));
        assert!(!looks_like_email("bad domain@dowhiz.com"));
    }
}
