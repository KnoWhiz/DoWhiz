//! Shared Google Drive Comments API client.
//!
//! The Comments API is the same for Docs, Sheets, and Slides.

use std::collections::HashSet;
use std::time::Duration;

use tracing::{error, info, warn};

use crate::channel::AdapterError;
use crate::google_auth::GoogleAuth;

use super::models::{
    ActionableComment, CommentReply, CommentsListResponse, DriveFile, FilesListResponse,
    GoogleComment,
};
use super::types::{GOOGLE_DOCS_MIME, GOOGLE_SHEETS_MIME, GOOGLE_SLIDES_MIME};

/// Default timeout for Google API requests (30 seconds).
const API_TIMEOUT_SECS: u64 = 30;

/// Maximum retry attempts for transient failures.
const MAX_RETRIES: u32 = 3;

/// Initial backoff delay for retries.
const INITIAL_BACKOFF_MS: u64 = 500;

/// Create an HTTP client with appropriate timeout settings.
fn create_http_client() -> Result<reqwest::blocking::Client, AdapterError> {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(API_TIMEOUT_SECS))
        .connect_timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| AdapterError::ConfigError(format!("Failed to create HTTP client: {}", e)))
}

/// Check if an error is retryable (network issues, 5xx errors).
fn is_retryable_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request()
}

/// Check if an HTTP status code is retryable.
fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS
}

/// Client for Google Drive Comments API operations.
/// Works with Docs, Sheets, and Slides.
#[derive(Debug, Clone)]
pub struct GoogleCommentsClient {
    auth: GoogleAuth,
    employee_emails: HashSet<String>,
    mention_checker: fn(&str) -> bool,
}

impl GoogleCommentsClient {
    pub fn new(
        auth: GoogleAuth,
        employee_emails: HashSet<String>,
        mention_checker: fn(&str) -> bool,
    ) -> Self {
        Self {
            auth,
            employee_emails,
            mention_checker,
        }
    }

    /// List all Google Workspace files shared with the authenticated user.
    pub fn list_shared_files(&self) -> Result<Vec<DriveFile>, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = create_http_client()?;

        let query = format!(
            "mimeType='{}' or mimeType='{}' or mimeType='{}'",
            GOOGLE_DOCS_MIME, GOOGLE_SHEETS_MIME, GOOGLE_SLIDES_MIME
        );

        let url = format!(
            "https://www.googleapis.com/drive/v3/files?q={}&fields=files(id,name,mimeType,owners)",
            urlencoding::encode(&query)
        );

        // Retry logic for transient failures
        let mut last_error = None;
        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(INITIAL_BACKOFF_MS * 2u64.pow(attempt - 1));
                warn!("Retrying list_shared_files (attempt {}/{}), backoff {:?}", attempt + 1, MAX_RETRIES, backoff);
                std::thread::sleep(backoff);
            }

            match client
                .get(&url)
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let files_response: FilesListResponse = response
                            .json()
                            .map_err(|e| AdapterError::ParseError(e.to_string()))?;
                        return Ok(files_response.files.unwrap_or_default());
                    }

                    let status = response.status();
                    if is_retryable_status(status) && attempt < MAX_RETRIES - 1 {
                        let body = response.text().unwrap_or_default();
                        warn!("Retryable HTTP error listing files: {} - {}", status, body);
                        last_error = Some(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                        continue;
                    }

                    let body = response.text().unwrap_or_default();
                    error!("Failed to list files: {} - {}", status, body);
                    return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                }
                Err(e) => {
                    if is_retryable_error(&e) && attempt < MAX_RETRIES - 1 {
                        warn!("Retryable network error listing files: {}", e);
                        last_error = Some(AdapterError::SendError(e.to_string()));
                        continue;
                    }
                    return Err(AdapterError::SendError(e.to_string()));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AdapterError::SendError("Max retries exceeded".to_string())))
    }

    /// List comments on a specific file.
    pub fn list_comments(&self, file_id: &str) -> Result<Vec<GoogleComment>, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = create_http_client()?;

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments?fields=comments(id,content,htmlContent,resolved,author,createdTime,modifiedTime,replies,anchor,quotedFileContent)",
            file_id
        );

        // Retry logic for transient failures
        let mut last_error = None;
        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(INITIAL_BACKOFF_MS * 2u64.pow(attempt - 1));
                warn!("Retrying list_comments for {} (attempt {}/{})", file_id, attempt + 1, MAX_RETRIES);
                std::thread::sleep(backoff);
            }

            match client
                .get(&url)
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let comments_response: CommentsListResponse = response
                            .json()
                            .map_err(|e| AdapterError::ParseError(e.to_string()))?;
                        return Ok(comments_response.comments.unwrap_or_default());
                    }

                    let status = response.status();
                    if is_retryable_status(status) && attempt < MAX_RETRIES - 1 {
                        let body = response.text().unwrap_or_default();
                        warn!("Retryable HTTP error listing comments for {}: {} - {}", file_id, status, body);
                        last_error = Some(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                        continue;
                    }

                    let body = response.text().unwrap_or_default();
                    error!("Failed to list comments for {}: {} - {}", file_id, status, body);
                    return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                }
                Err(e) => {
                    if is_retryable_error(&e) && attempt < MAX_RETRIES - 1 {
                        warn!("Retryable network error listing comments for {}: {}", file_id, e);
                        last_error = Some(AdapterError::SendError(e.to_string()));
                        continue;
                    }
                    return Err(AdapterError::SendError(e.to_string()));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AdapterError::SendError("Max retries exceeded".to_string())))
    }

    /// Post a reply to a comment.
    pub fn reply_to_comment(
        &self,
        file_id: &str,
        comment_id: &str,
        reply_content: &str,
    ) -> Result<CommentReply, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = create_http_client()?;

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments/{}/replies?fields=id,content,createdTime,author",
            file_id, comment_id
        );

        let payload = serde_json::json!({ "content": reply_content });

        // Retry logic for transient failures
        let mut last_error = None;
        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(INITIAL_BACKOFF_MS * 2u64.pow(attempt - 1));
                warn!("Retrying reply_to_comment {} on {} (attempt {}/{})", comment_id, file_id, attempt + 1, MAX_RETRIES);
                std::thread::sleep(backoff);
            }

            match client
                .post(&url)
                .header("Authorization", format!("Bearer {}", access_token))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        let reply: CommentReply = response
                            .json()
                            .map_err(|e| AdapterError::ParseError(e.to_string()))?;
                        info!(
                            "Posted reply {} to comment {} on file {}",
                            reply.id, comment_id, file_id
                        );
                        return Ok(reply);
                    }

                    let status = response.status();
                    if is_retryable_status(status) && attempt < MAX_RETRIES - 1 {
                        let body = response.text().unwrap_or_default();
                        warn!("Retryable HTTP error replying to comment {} on {}: {} - {}", comment_id, file_id, status, body);
                        last_error = Some(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                        continue;
                    }

                    let body = response.text().unwrap_or_default();
                    error!("Failed to reply to comment {} on {}: {} - {}", comment_id, file_id, status, body);
                    return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                }
                Err(e) => {
                    if is_retryable_error(&e) && attempt < MAX_RETRIES - 1 {
                        warn!("Retryable network error replying to comment {} on {}: {}", comment_id, file_id, e);
                        last_error = Some(AdapterError::SendError(e.to_string()));
                        continue;
                    }
                    return Err(AdapterError::SendError(e.to_string()));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AdapterError::SendError("Max retries exceeded".to_string())))
    }

    /// Filter comments to find actionable ones.
    pub fn filter_actionable_comments(
        &self,
        comments: &[GoogleComment],
        processed_ids: &HashSet<String>,
    ) -> Vec<ActionableComment> {
        filter_actionable_comments(
            comments,
            processed_ids,
            &self.employee_emails,
            self.mention_checker,
        )
    }

    /// Export file content as text.
    pub fn export_file_content(
        &self,
        file_id: &str,
        export_mime_type: &str,
    ) -> Result<String, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = create_http_client()?;

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType={}",
            file_id,
            urlencoding::encode(export_mime_type)
        );

        // Retry logic for transient failures
        let mut last_error = None;
        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let backoff = Duration::from_millis(INITIAL_BACKOFF_MS * 2u64.pow(attempt - 1));
                warn!("Retrying export_file_content for {} (attempt {}/{})", file_id, attempt + 1, MAX_RETRIES);
                std::thread::sleep(backoff);
            }

            match client
                .get(&url)
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        return response
                            .text()
                            .map_err(|e| AdapterError::ParseError(e.to_string()));
                    }

                    let status = response.status();
                    if is_retryable_status(status) && attempt < MAX_RETRIES - 1 {
                        let body = response.text().unwrap_or_default();
                        warn!("Retryable HTTP error exporting file {}: {} - {}", file_id, status, body);
                        last_error = Some(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                        continue;
                    }

                    let body = response.text().unwrap_or_default();
                    error!("Failed to export file {}: {} - {}", file_id, status, body);
                    return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
                }
                Err(e) => {
                    if is_retryable_error(&e) && attempt < MAX_RETRIES - 1 {
                        warn!("Retryable network error exporting file {}: {}", file_id, e);
                        last_error = Some(AdapterError::SendError(e.to_string()));
                        continue;
                    }
                    return Err(AdapterError::SendError(e.to_string()));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AdapterError::SendError("Max retries exceeded".to_string())))
    }
}

/// Filter comments to find actionable ones.
pub fn filter_actionable_comments(
    comments: &[GoogleComment],
    processed_ids: &HashSet<String>,
    employee_emails: &HashSet<String>,
    mention_checker: fn(&str) -> bool,
) -> Vec<ActionableComment> {
    let mut actionable = Vec::new();

    for comment in comments {
        if comment.resolved == Some(true) {
            continue;
        }

        let comment_tracking_id = format!("comment:{}", comment.id);
        if !processed_ids.contains(&comment_tracking_id) {
            // Skip comments from our own accounts.
            // Check both the `me` field (authenticated user) and email address.
            let is_from_self = comment
                .author
                .as_ref()
                .map(|a| a.me)
                .unwrap_or(false);
            let is_from_employee = comment
                .author
                .as_ref()
                .and_then(|a| a.email_address.as_ref())
                .map(|e| employee_emails.contains(e))
                .unwrap_or(false);

            if !is_from_self && !is_from_employee && mention_checker(&comment.content) {
                let comment_preview = comment.content.chars().take(50).collect::<String>();
                info!("Found actionable comment: '{}'", comment_preview);
                actionable.push(ActionableComment::from_comment(comment.clone()));
                continue;
            }
        }

        if let Some(ref replies) = comment.replies {
            for reply in replies {
                let reply_tracking_id = format!("comment:{}:reply:{}", comment.id, reply.id);

                if processed_ids.contains(&reply_tracking_id) {
                    continue;
                }

                // Skip replies from our own accounts.
                // Check both the `me` field (authenticated user) and email address.
                let is_from_self = reply
                    .author
                    .as_ref()
                    .map(|a| a.me)
                    .unwrap_or(false);
                let is_from_employee = reply
                    .author
                    .as_ref()
                    .and_then(|a| a.email_address.as_ref())
                    .map(|e| employee_emails.contains(e))
                    .unwrap_or(false);

                if is_from_self || is_from_employee {
                    continue;
                }

                if mention_checker(&reply.content) {
                    let reply_preview = reply.content.chars().take(50).collect::<String>();
                    info!("Found actionable reply: '{}'", reply_preview);
                    actionable.push(ActionableComment::from_reply(
                        comment.clone(),
                        reply.clone(),
                    ));
                }
            }
        }
    }

    actionable
}
