use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use mime_guess::MimeGuess;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SendEmailParams {
    pub subject: String,
    pub html_path: PathBuf,
    pub attachments_dir: PathBuf,
    pub from: Option<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub in_reply_to: Option<String>,
    pub references: Option<String>,
    /// Reply-To address - where replies should be sent
    /// If set, this overrides the default reply behavior
    pub reply_to: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostmarkSendResponse {
    pub error_code: i64,
    pub message: String,
    #[serde(rename = "MessageID", alias = "MessageId")]
    pub message_id: String,
    pub submitted_at: String,
    pub to: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SendEmailError {
    #[error("missing environment variable: {0}")]
    MissingEnv(&'static str),
    #[error("missing from address")]
    MissingFrom,
    #[error("missing recipient in To list")]
    MissingRecipient,
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("postmark request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("postmark returned error: {0}")]
    Postmark(String),
    #[error("failed to parse json: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PostmarkSendRequest {
    from: String,
    to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
    subject: String,
    text_body: String,
    html_body: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    headers: Vec<PostmarkHeader>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<PostmarkAttachment>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PostmarkAttachment {
    name: String,
    content: String,
    content_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PostmarkHeader {
    name: String,
    value: String,
}

pub fn send_email(params: &SendEmailParams) -> Result<PostmarkSendResponse, SendEmailError> {
    dotenvy::dotenv().ok();

    let token = env::var("POSTMARK_SERVER_TOKEN")
        .map_err(|_| SendEmailError::MissingEnv("POSTMARK_SERVER_TOKEN"))?;
    if token.trim().is_empty() {
        return Err(SendEmailError::MissingEnv("POSTMARK_SERVER_TOKEN"));
    }
    let from = params
        .from
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .ok_or(SendEmailError::MissingFrom)?;

    let to = join_recipients(&params.to).ok_or(SendEmailError::MissingRecipient)?;
    let cc = join_recipients(&params.cc);
    let mut bcc_list = params.bcc.clone();
    if !bcc_list
        .iter()
        .any(|addr| addr.trim().eq_ignore_ascii_case(&from))
    {
        bcc_list.push(from.clone());
    }
    let bcc = join_recipients(&bcc_list);

    let html_body = fs::read_to_string(&params.html_path)?;
    let mut text_body = strip_html_tags(&html_body);
    if text_body.trim().is_empty() {
        text_body = "(no content)".to_string();
    }

    let attachments = load_attachments(&params.attachments_dir)?;

    let mut headers = Vec::new();
    if let Some(value) = clean_header_value(&params.in_reply_to) {
        headers.push(PostmarkHeader {
            name: "In-Reply-To".to_string(),
            value,
        });
    }
    if let Some(value) = clean_header_value(&params.references) {
        headers.push(PostmarkHeader {
            name: "References".to_string(),
            value,
        });
    }

    let reply_to = clean_header_value(&params.reply_to);

    let payload = PostmarkSendRequest {
        from,
        to,
        cc,
        bcc,
        reply_to,
        subject: params.subject.clone(),
        text_body,
        html_body,
        headers,
        attachments,
    };

    let api_base = env::var("POSTMARK_API_BASE_URL")
        .unwrap_or_else(|_| "https://api.postmarkapp.com".to_string());
    let url = format!("{}/email", api_base.trim_end_matches('/'));

    let client = reqwest::blocking::Client::new();
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
            "status {}: {}",
            status, body
        )));
    }

    Ok(serde_json::from_str(&body)?)
}

fn join_recipients(list: &[String]) -> Option<String> {
    let mut cleaned = Vec::new();
    for entry in list {
        let trimmed = entry.trim();
        if !trimmed.is_empty() {
            let sanitized = sanitize_recipient(trimmed);
            if !sanitized.is_empty() {
                cleaned.push(sanitized);
            }
        }
    }
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned.join(", "))
    }
}

fn sanitize_recipient(value: &str) -> String {
    if has_unbalanced_quotes(value) {
        if let Some(email) = extract_email_address(value) {
            return email;
        }
    }
    value.to_string()
}

fn has_unbalanced_quotes(value: &str) -> bool {
    value.chars().filter(|ch| *ch == '"').count() % 2 == 1
}

fn extract_email_address(value: &str) -> Option<String> {
    if let Some(start) = value.find('<') {
        let remainder = &value[start + 1..];
        if let Some(end) = remainder.find('>') {
            return normalize_email(&remainder[..end]);
        }
    }
    for token in value.split(|ch| matches!(ch, ',' | ';' | ' ' | '\t' | '\n' | '\r')) {
        if let Some(email) = normalize_email(token) {
            return Some(email);
        }
    }
    None
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
    Some(format!("{}@{}", local, domain))
}

fn clean_header_value(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(|trimmed| trimmed.to_string())
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

fn ascii_safe_attachment_name(path: &Path, used_names: &mut HashSet<String>) -> String {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    let mut base = sanitize_ascii_attachment_stem(stem);
    if base.is_empty() {
        base = "attachment".to_string();
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(sanitize_ascii_attachment_extension)
        .filter(|value| !value.is_empty());

    uniquify_attachment_name(base, extension.as_deref(), used_names)
}

fn sanitize_ascii_attachment_stem(value: &str) -> String {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            current.push(ch);
        } else if !current.is_empty() {
            tokens.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    let mut deduped = Vec::new();
    for token in tokens {
        let duplicate = deduped
            .last()
            .map(|last: &String| last.eq_ignore_ascii_case(&token))
            .unwrap_or(false);
        if !duplicate {
            deduped.push(token);
        }
    }

    deduped.join("_")
}

fn sanitize_ascii_attachment_extension(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

fn uniquify_attachment_name(
    base: String,
    extension: Option<&str>,
    used_names: &mut HashSet<String>,
) -> String {
    let mut suffix = 1;

    loop {
        let stem = if suffix == 1 {
            base.clone()
        } else {
            format!("{}_{}", base, suffix)
        };
        let candidate = match extension {
            Some(ext) if !ext.is_empty() => format!("{}.{}", stem, ext),
            _ => stem,
        };
        if used_names.insert(candidate.to_ascii_lowercase()) {
            return candidate;
        }
        suffix += 1;
    }
}

fn load_attachments(dir: &Path) -> Result<Vec<PostmarkAttachment>, std::io::Error> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut attachments = Vec::new();
    let mut used_names = HashSet::new();
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let content = fs::read(&path)?;
        let mime = MimeGuess::from_path(&path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();
        let attachment = PostmarkAttachment {
            name: ascii_safe_attachment_name(&path, &mut used_names),
            content: BASE64_STANDARD.encode(content),
            content_type: mime,
        };
        attachments.push(attachment);
    }

    Ok(attachments)
}
