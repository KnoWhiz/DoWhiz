use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine as _;
use mime_guess::MimeGuess;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GmailError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("missing refresh token; set GMAIL_REFRESH_TOKEN or add refresh_token to the token file")]
    MissingRefreshToken,
    #[error("invalid credentials file: {0}")]
    InvalidCredentials(String),
    #[error("Gmail API error {status}: {body}")]
    ApiError { status: u16, body: String },
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Gmail API does not support scheduled send; use ScheduleMode::LocalDelay or schedule outside Gmail")]
    ScheduleNotSupported,
}

#[derive(Debug, Clone)]
pub enum SendTiming {
    Immediate,
    Delay(Duration),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleMode {
    LocalDelay,
    Gmail,
}

impl Default for ScheduleMode {
    fn default() -> Self {
        ScheduleMode::LocalDelay
    }
}

#[derive(Debug, Clone)]
pub struct GmailSenderConfig {
    pub credentials_path: PathBuf,
    pub token_path: PathBuf,
    pub from_email: String,
    pub user_id: String,
    pub base_url: String,
    pub application_name: String,
    pub timeout: Duration,
    pub access_token_override: Option<String>,
    pub schedule_mode: ScheduleMode,
}

impl Default for GmailSenderConfig {
    fn default() -> Self {
        let credentials_path = PathBuf::from(".gmail_client_secret.json");
        let token_path = credentials_path.clone();
        Self {
            credentials_path,
            token_path,
            from_email: "agent@dowhiz.com".to_string(),
            user_id: "me".to_string(),
            base_url: "https://gmail.googleapis.com/gmail/v1".to_string(),
            application_name: "DoWhiz MVP Gmail Sender".to_string(),
            timeout: Duration::from_secs(30),
            access_token_override: None,
            schedule_mode: ScheduleMode::LocalDelay,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SendResult {
    pub to: String,
    pub message_id: String,
    pub thread_id: Option<String>,
}

pub struct GmailSender {
    config: GmailSenderConfig,
    client: Client,
}

impl GmailSender {
    pub fn new(config: GmailSenderConfig) -> Result<Self, GmailError> {
        if config.from_email.trim().is_empty() {
            return Err(GmailError::InvalidConfig("from_email cannot be empty".to_string()));
        }
        let client = Client::builder()
            .user_agent(config.application_name.clone())
            .timeout(config.timeout)
            .build()?;
        Ok(Self { config, client })
    }

    pub async fn send_email(
        &self,
        subject: &str,
        html_path: impl AsRef<Path>,
        attachments_dir: impl AsRef<Path>,
        to_addresses: &[String],
        timing: SendTiming,
    ) -> Result<Vec<SendResult>, GmailError> {
        if to_addresses.is_empty() {
            return Err(GmailError::InvalidConfig("to_addresses cannot be empty".to_string()));
        }

        let html = fs::read_to_string(html_path)?;
        let attachments = load_attachments(attachments_dir.as_ref())?;

        if let SendTiming::Delay(delay) = timing {
            if !delay.is_zero() {
                match self.config.schedule_mode {
                    ScheduleMode::LocalDelay => tokio::time::sleep(delay).await,
                    ScheduleMode::Gmail => return Err(GmailError::ScheduleNotSupported),
                }
            }
        }

        let access_token = self.get_access_token().await?;

        let mut results = Vec::with_capacity(to_addresses.len());
        for to in to_addresses {
            let mime = build_mime_message(&self.config.from_email, to, subject, &html, &attachments);
            let raw = URL_SAFE_NO_PAD.encode(mime.as_bytes());
            let response = self.send_raw_message(&access_token, &raw).await?;
            results.push(SendResult {
                to: to.clone(),
                message_id: response.id,
                thread_id: response.thread_id,
            });
        }

        Ok(results)
    }

    async fn send_raw_message(&self, access_token: &str, raw: &str) -> Result<SendMessageResponse, GmailError> {
        let url = format!(
            "{}/users/{}/messages/send",
            self.config.base_url, self.config.user_id
        );
        let response = self
            .client
            .post(url)
            .bearer_auth(access_token)
            .json(&json!({"raw": raw}))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_else(|_| "<failed to read body>".to_string());
            return Err(GmailError::ApiError { status, body });
        }

        Ok(response.json().await?)
    }

    async fn get_access_token(&self) -> Result<String, GmailError> {
        if let Some(token) = &self.config.access_token_override {
            return Ok(token.clone());
        }

        let credentials = load_credentials(&self.config.credentials_path)?;
        let mut token_store = StoredToken::load_or_default(&self.config.token_path)?;

        if token_store.access_token_valid() {
            return Ok(token_store.access_token.clone().unwrap());
        }

        let refresh_token = match token_store.refresh_token.clone() {
            Some(token) => token,
            None => std::env::var("GMAIL_REFRESH_TOKEN").ok().unwrap_or_default(),
        };

        if refresh_token.is_empty() {
            return Err(GmailError::MissingRefreshToken);
        }

        let token_response = refresh_access_token(&self.client, &credentials, &refresh_token).await?;
        token_store.refresh_token = Some(refresh_token);
        token_store.access_token = Some(token_response.access_token.clone());
        let expires_at = token_response
            .expires_in
            .unwrap_or(3600)
            .saturating_sub(60);
        token_store.expires_at = Some(unix_timestamp() + expires_at);
        token_store.save(&self.config.token_path)?;

        Ok(token_response.access_token)
    }
}

#[derive(Debug, Deserialize)]
struct CredentialsFile {
    web: Option<Credentials>,
    installed: Option<Credentials>,
}

#[derive(Debug, Deserialize, Clone)]
struct Credentials {
    client_id: String,
    client_secret: String,
    token_uri: String,
}

fn load_credentials(path: &Path) -> Result<Credentials, GmailError> {
    let contents = fs::read_to_string(path)?;
    let file: CredentialsFile = serde_json::from_str(&contents)?;
    if let Some(web) = file.web {
        return Ok(web);
    }
    if let Some(installed) = file.installed {
        return Ok(installed);
    }
    Err(GmailError::InvalidCredentials(
        "expected top-level 'web' or 'installed' credentials".to_string(),
    ))
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

async fn refresh_access_token(
    client: &Client,
    credentials: &Credentials,
    refresh_token: &str,
) -> Result<TokenResponse, GmailError> {
    let response = client
        .post(&credentials.token_uri)
        .form(&[
            ("client_id", credentials.client_id.as_str()),
            ("client_secret", credentials.client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "<failed to read body>".to_string());
        return Err(GmailError::ApiError { status, body });
    }

    let mut token: TokenResponse = response.json().await?;
    if token.refresh_token.is_none() {
        token.refresh_token = Some(refresh_token.to_string());
    }
    Ok(token)
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct StoredToken {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<u64>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

impl StoredToken {
    fn load_or_default(path: &Path) -> Result<Self, GmailError> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    fn save(&self, path: &Path) -> Result<(), GmailError> {
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    fn access_token_valid(&self) -> bool {
        match (&self.access_token, self.expires_at) {
            (Some(token), Some(expires_at)) if !token.is_empty() => unix_timestamp() < expires_at,
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SendMessageResponse {
    id: String,
    #[serde(rename = "threadId")]
    thread_id: Option<String>,
}

#[derive(Debug, Clone)]
struct Attachment {
    filename: String,
    mime_type: String,
    data: Vec<u8>,
}

fn load_attachments(dir: &Path) -> Result<Vec<Attachment>, GmailError> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    if !dir.is_dir() {
        return Err(GmailError::InvalidConfig(format!(
            "attachments_dir is not a directory: {}",
            dir.display()
        )));
    }

    let mut attachments = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect();
    entries.sort_by_key(|entry| entry.as_ref().ok().map(|e| e.file_name()));

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let filename = entry.file_name().to_string_lossy().to_string();
        let data = fs::read(&path)?;
        let mime_type = MimeGuess::from_path(&path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();
        attachments.push(Attachment {
            filename,
            mime_type,
            data,
        });
    }

    Ok(attachments)
}

fn build_mime_message(
    from: &str,
    to: &str,
    subject: &str,
    html_body: &str,
    attachments: &[Attachment],
) -> String {
    let boundary: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();
    let boundary = format!("==BOUNDARY_{}", boundary);

    let mut message = String::new();
    message.push_str(&format!("From: {}\r\n", from));
    message.push_str(&format!("To: {}\r\n", to));
    message.push_str(&format!("Subject: {}\r\n", subject));
    message.push_str("MIME-Version: 1.0\r\n");
    message.push_str(&format!(
        "Content-Type: multipart/mixed; boundary=\"{}\"\r\n",
        boundary
    ));
    message.push_str("\r\n");

    message.push_str(&format!("--{}\r\n", boundary));
    message.push_str("Content-Type: text/html; charset=\"UTF-8\"\r\n");
    message.push_str("Content-Transfer-Encoding: base64\r\n");
    message.push_str("\r\n");
    message.push_str(&wrap_base64(&STANDARD.encode(html_body.as_bytes())));
    message.push_str("\r\n");

    for attachment in attachments {
        message.push_str(&format!("--{}\r\n", boundary));
        message.push_str(&format!(
            "Content-Type: {}; name=\"{}\"\r\n",
            attachment.mime_type, attachment.filename
        ));
        message.push_str(&format!(
            "Content-Disposition: attachment; filename=\"{}\"\r\n",
            attachment.filename
        ));
        message.push_str("Content-Transfer-Encoding: base64\r\n");
        message.push_str("\r\n");
        message.push_str(&wrap_base64(&STANDARD.encode(&attachment.data)));
        message.push_str("\r\n");
    }

    message.push_str(&format!("--{}--\r\n", boundary));
    message
}

fn wrap_base64(data: &str) -> String {
    data.as_bytes()
        .chunks(76)
        .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\r\n")
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs()
}
