//! Shared Google Drive Comments API client.
//!
//! The Comments API is the same for Docs, Sheets, and Slides.

use std::collections::HashSet;

use tracing::{error, info};

use crate::channel::AdapterError;
use crate::google_auth::GoogleAuth;

use super::models::{
    ActionableComment, CommentReply, CommentsListResponse, DriveFile, FilesListResponse,
    GoogleComment,
};
use super::types::{GOOGLE_DOCS_MIME, GOOGLE_SHEETS_MIME, GOOGLE_SLIDES_MIME};

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

        let client = reqwest::blocking::Client::new();

        let query = format!(
            "mimeType='{}' or mimeType='{}' or mimeType='{}'",
            GOOGLE_DOCS_MIME, GOOGLE_SHEETS_MIME, GOOGLE_SLIDES_MIME
        );

        let url = format!(
            "https://www.googleapis.com/drive/v3/files?q={}&fields=files(id,name,mimeType,owners)",
            urlencoding::encode(&query)
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to list files: {} - {}", status, body);
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let files_response: FilesListResponse = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        Ok(files_response.files.unwrap_or_default())
    }

    /// List comments on a specific file.
    pub fn list_comments(&self, file_id: &str) -> Result<Vec<GoogleComment>, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments?fields=comments(id,content,htmlContent,resolved,author,createdTime,modifiedTime,replies,anchor,quotedFileContent)",
            file_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to list comments for {}: {} - {}", file_id, status, body);
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let comments_response: CommentsListResponse = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        Ok(comments_response.comments.unwrap_or_default())
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

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments/{}/replies?fields=id,content,createdTime,author",
            file_id, comment_id
        );

        let payload = serde_json::json!({ "content": reply_content });

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to reply to comment {} on {}: {} - {}",
                comment_id, file_id, status, body
            );
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        let reply: CommentReply = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        info!(
            "Posted reply {} to comment {} on file {}",
            reply.id, comment_id, file_id
        );

        Ok(reply)
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

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType={}",
            file_id,
            urlencoding::encode(export_mime_type)
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to export file {}: {} - {}", file_id, status, body);
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        response
            .text()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
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
            let is_from_employee = comment
                .author
                .as_ref()
                .and_then(|a| a.email_address.as_ref())
                .map(|e| employee_emails.contains(e))
                .unwrap_or(false);

            if !is_from_employee && mention_checker(&comment.content) {
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

                let is_from_employee = reply
                    .author
                    .as_ref()
                    .and_then(|a| a.email_address.as_ref())
                    .map(|e| employee_emails.contains(e))
                    .unwrap_or(false);

                if is_from_employee {
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
