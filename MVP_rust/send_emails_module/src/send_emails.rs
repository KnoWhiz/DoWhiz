use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub const DEFAULT_POSTMARK_API_BASE: &str = "https://api.postmarkapp.com";
const DEFAULT_FROM_ADDRESS: &str = "oliver@dowhiz.com";

#[derive(Debug, Clone)]
pub struct SendEmailRequest<'a> {
    pub html_path: &'a Path,
    pub attachments_dir: &'a Path,
    pub to: &'a [String],
    pub cc: &'a [String],
    pub bcc: &'a [String],
    pub subject: Option<&'a str>,
    pub from: Option<&'a str>,
    pub postmark_token: Option<&'a str>,
    pub api_base_url: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct SendEmailResponse {
    pub message_id: String,
    pub submitted_at: Option<String>,
    pub to: String,
    pub message: String,
}

#[derive(Debug)]
pub enum SendEmailError {
    MissingPostmarkToken,
    MissingRecipients,
    InvalidAttachmentsDir(PathBuf),
    Io(std::io::Error),
    Http(reqwest::Error),
    Postmark(String),
    InvalidResponse(String),
}

impl fmt::Display for SendEmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendEmailError::MissingPostmarkToken => write!(f, "POSTMARK_SERVER_TOKEN not set"),
            SendEmailError::MissingRecipients => write!(f, "At least one recipient is required"),
            SendEmailError::InvalidAttachmentsDir(path) => {
                write!(f, "Attachments path is not a directory: {}", path.display())
            }
            SendEmailError::Io(err) => write!(f, "IO error: {err}"),
            SendEmailError::Http(err) => write!(f, "HTTP error: {err}"),
            SendEmailError::Postmark(msg) => write!(f, "Postmark error: {msg}"),
            SendEmailError::InvalidResponse(msg) => write!(f, "Invalid response: {msg}"),
        }
    }
}

impl std::error::Error for SendEmailError {}

impl From<std::io::Error> for SendEmailError {
    fn from(err: std::io::Error) -> Self {
        SendEmailError::Io(err)
    }
}

impl From<reqwest::Error> for SendEmailError {
    fn from(err: reqwest::Error) -> Self {
        SendEmailError::Http(err)
    }
}

#[derive(Debug, Deserialize)]
struct PostmarkSendResponse {
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "SubmittedAt")]
    submitted_at: Option<String>,
    #[serde(rename = "MessageID")]
    message_id: String,
    #[serde(rename = "ErrorCode")]
    error_code: i32,
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Debug, Serialize)]
struct PostmarkAttachment {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Content")]
    content: String,
    #[serde(rename = "ContentType")]
    content_type: String,
}

#[derive(Debug, Serialize)]
struct PostmarkEmailPayload<'a> {
    #[serde(rename = "From")]
    from: &'a str,
    #[serde(rename = "To")]
    to: &'a str,
    #[serde(rename = "Cc", skip_serializing_if = "Option::is_none")]
    cc: Option<&'a str>,
    #[serde(rename = "Bcc", skip_serializing_if = "Option::is_none")]
    bcc: Option<&'a str>,
    #[serde(rename = "Subject")]
    subject: &'a str,
    #[serde(rename = "HtmlBody")]
    html_body: &'a str,
    #[serde(rename = "Attachments", skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<PostmarkAttachment>,
}

pub fn send_email(request: SendEmailRequest<'_>) -> Result<SendEmailResponse, SendEmailError> {
    let token = match request.postmark_token {
        Some(token) if !token.trim().is_empty() => token.to_string(),
        _ => env::var("POSTMARK_SERVER_TOKEN").unwrap_or_default(),
    };
    if token.trim().is_empty() {
        return Err(SendEmailError::MissingPostmarkToken);
    }

    let from = match request.from {
        Some(from) if !from.trim().is_empty() => from.to_string(),
        _ => env::var("OUTBOUND_FROM").unwrap_or_else(|_| DEFAULT_FROM_ADDRESS.to_string()),
    };

    let to_list = normalize_recipients(request.to);
    if to_list.is_empty() {
        return Err(SendEmailError::MissingRecipients);
    }
    let cc_list = normalize_recipients(request.cc);
    let bcc_list = normalize_recipients(request.bcc);

    let subject = request.subject.unwrap_or("").trim();
    let subject = if subject.is_empty() { "(no subject)" } else { subject };

    let html_body_raw = fs::read_to_string(request.html_path)?;
    let html_body = if html_body_raw.trim().is_empty() {
        "<p>(no content)</p>".to_string()
    } else {
        html_body_raw
    };

    let attachments = collect_attachments(request.attachments_dir)?;

    let to_joined = to_list.join(", ");
    let cc_joined = join_optional(cc_list);
    let bcc_joined = join_optional(bcc_list);

    let payload = PostmarkEmailPayload {
        from: from.as_str(),
        to: to_joined.as_str(),
        cc: cc_joined.as_deref(),
        bcc: bcc_joined.as_deref(),
        subject,
        html_body: html_body.as_str(),
        attachments,
    };

    let api_base = request.api_base_url.unwrap_or(DEFAULT_POSTMARK_API_BASE);
    let url = format!("{}/email", api_base.trim_end_matches('/'));

    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;
    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("X-Postmark-Server-Token", token)
        .json(&payload)
        .send()?;

    let status = response.status();
    let body = response.text()?;
    if !status.is_success() {
        return Err(SendEmailError::Postmark(format!(
            "HTTP {}: {}",
            status.as_u16(),
            body
        )));
    }

    let decoded: PostmarkSendResponse = serde_json::from_str(&body)
        .map_err(|err| SendEmailError::InvalidResponse(format!("{err}: {body}")))?;

    if decoded.error_code != 0 {
        return Err(SendEmailError::Postmark(decoded.message));
    }

    Ok(SendEmailResponse {
        message_id: decoded.message_id,
        submitted_at: decoded.submitted_at,
        to: decoded.to,
        message: decoded.message,
    })
}

fn normalize_recipients(recipients: &[String]) -> Vec<String> {
    recipients
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect()
}

fn join_optional(values: Vec<String>) -> Option<String> {
    if values.is_empty() {
        None
    } else {
        Some(values.join(", "))
    }
}

fn collect_attachments(dir: &Path) -> Result<Vec<PostmarkAttachment>, SendEmailError> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    if !dir.is_dir() {
        return Err(SendEmailError::InvalidAttachmentsDir(dir.to_path_buf()));
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let mut attachments = Vec::new();
    for path in paths {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("attachment")
            .to_string();
        let payload = fs::read(&path)?;
        let content_type = mime_guess::from_path(&path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();
        let content = general_purpose::STANDARD.encode(payload);

        attachments.push(PostmarkAttachment {
            name,
            content,
            content_type,
        });
    }

    Ok(attachments)
}
