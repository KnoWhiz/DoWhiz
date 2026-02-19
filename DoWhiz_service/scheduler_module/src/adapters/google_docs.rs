//! Google Docs adapter for collaborative document editing via comments.
//!
//! This module provides adapters for handling messages via Google Docs comments:
//! - `GoogleDocsInboundAdapter`: Polls for comments mentioning the employee
//! - `GoogleDocsOutboundAdapter`: Posts replies and applies edits to documents

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::LazyLock;
use tracing::{error, info};

use crate::channel::{
    AdapterError, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
    OutboundAdapter, OutboundMessage, SendResult,
};
use crate::google_auth::GoogleAuth;

/// Patterns to match employee mentions in comments.
/// Supports all DoWhiz employees: oliver, maggie, proto, devin, etc.
static EMPLOYEE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        // Oliver (little_bear)
        Regex::new(r"(?i)\b@?oliver\b").unwrap(),
        Regex::new(r"(?i)oliver@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\blittle[_\s-]?bear\b").unwrap(),
        // Maggie (mini_mouse)
        Regex::new(r"(?i)\b@?maggie\b").unwrap(),
        Regex::new(r"(?i)maggie@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\bmini[_\s-]?mouse\b").unwrap(),
        // Proto / Boiled-Egg (boiled_egg) - for local testing
        Regex::new(r"(?i)\b@?proto\b").unwrap(),
        Regex::new(r"(?i)proto@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\bboiled[_\s-]?egg\b").unwrap(),
        // Devin / Sticky-Octopus (sticky_octopus)
        Regex::new(r"(?i)\b@?devin\b").unwrap(),
        Regex::new(r"(?i)devin@dowhiz\.com").unwrap(),
        Regex::new(r"(?i)\bsticky[_\s-]?octopus\b").unwrap(),
        Regex::new(r"(?i)coder@dowhiz\.com").unwrap(),
    ]
});

/// Check if text contains an employee mention.
pub fn contains_employee_mention(text: &str) -> bool {
    EMPLOYEE_PATTERNS.iter().any(|pattern| pattern.is_match(text))
}

/// Extract the employee name from a mention.
/// Returns the canonical display name for the employee.
pub fn extract_employee_name(text: &str) -> Option<&'static str> {
    let text_lower = text.to_lowercase();
    // Oliver (little_bear)
    if text_lower.contains("oliver") || text_lower.contains("little_bear") || text_lower.contains("little bear") || text_lower.contains("little-bear") {
        Some("Oliver")
    // Maggie (mini_mouse)
    } else if text_lower.contains("maggie") || text_lower.contains("mini_mouse") || text_lower.contains("mini mouse") || text_lower.contains("mini-mouse") {
        Some("Maggie")
    // Proto / Boiled-Egg (boiled_egg) - for local testing
    } else if text_lower.contains("proto") || text_lower.contains("boiled_egg") || text_lower.contains("boiled egg") || text_lower.contains("boiled-egg") {
        Some("Proto")
    // Devin / Sticky-Octopus (sticky_octopus)
    } else if text_lower.contains("devin") || text_lower.contains("sticky_octopus") || text_lower.contains("sticky octopus") || text_lower.contains("sticky-octopus") || text_lower.contains("coder") {
        Some("Devin")
    } else {
        None
    }
}

/// Google Docs comment from the Drive API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDocsComment {
    /// The comment ID
    pub id: String,
    /// Plain text content of the comment
    pub content: String,
    /// HTML content of the comment (if available)
    #[serde(rename = "htmlContent")]
    pub html_content: Option<String>,
    /// Whether the comment is resolved
    pub resolved: Option<bool>,
    /// Author information
    pub author: Option<CommentAuthor>,
    /// Creation time (RFC3339)
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// Modification time (RFC3339)
    #[serde(rename = "modifiedTime")]
    pub modified_time: Option<String>,
    /// Replies to this comment
    pub replies: Option<Vec<CommentReply>>,
    /// Anchor information (position in document)
    pub anchor: Option<String>,
    /// Quoted text from the document
    #[serde(rename = "quotedFileContent")]
    pub quoted_file_content: Option<QuotedFileContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAuthor {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "photoLink")]
    pub photo_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    pub id: String,
    pub content: String,
    #[serde(rename = "htmlContent")]
    pub html_content: Option<String>,
    pub author: Option<CommentAuthor>,
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotedFileContent {
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub value: Option<String>,
}

/// Represents an actionable item from Google Docs (either a comment or a reply).
/// This structure helps track whether a parent comment or a specific reply triggered the action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionableComment {
    /// The parent comment
    pub comment: GoogleDocsComment,
    /// If this is a reply, the specific reply that triggered the action
    pub triggering_reply: Option<CommentReply>,
    /// The unique ID for tracking (either "comment:{id}" or "comment:{id}:reply:{reply_id}")
    pub tracking_id: String,
}

impl ActionableComment {
    /// Create an actionable item for a parent comment.
    pub fn from_comment(comment: GoogleDocsComment) -> Self {
        let tracking_id = format!("comment:{}", comment.id);
        Self {
            comment,
            triggering_reply: None,
            tracking_id,
        }
    }

    /// Create an actionable item for a reply.
    pub fn from_reply(comment: GoogleDocsComment, reply: CommentReply) -> Self {
        let tracking_id = format!("comment:{}:reply:{}", comment.id, reply.id);
        Self {
            triggering_reply: Some(reply),
            comment,
            tracking_id,
        }
    }

    /// Get the content that triggered this action (either comment content or reply content).
    pub fn triggering_content(&self) -> &str {
        self.triggering_reply
            .as_ref()
            .map(|r| r.content.as_str())
            .unwrap_or(&self.comment.content)
    }

    /// Get the author of the triggering content.
    pub fn triggering_author(&self) -> Option<&CommentAuthor> {
        self.triggering_reply
            .as_ref()
            .and_then(|r| r.author.as_ref())
            .or(self.comment.author.as_ref())
    }
}

/// Response from Google Drive API comments.list
#[derive(Debug, Clone, Deserialize)]
pub struct CommentsListResponse {
    pub comments: Option<Vec<GoogleDocsComment>>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

/// Google Drive file metadata
#[derive(Debug, Clone, Deserialize)]
pub struct DriveFile {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub owners: Option<Vec<DriveFileOwner>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DriveFileOwner {
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

/// Response from Google Drive API files.list
#[derive(Debug, Clone, Deserialize)]
pub struct FilesListResponse {
    pub files: Option<Vec<DriveFile>>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

/// Adapter for polling Google Docs comments.
#[derive(Debug, Clone)]
pub struct GoogleDocsInboundAdapter {
    /// Google authentication
    auth: GoogleAuth,
    /// Employee email addresses to ignore (our own comments)
    employee_emails: HashSet<String>,
}

impl GoogleDocsInboundAdapter {
    pub fn new(auth: GoogleAuth, employee_emails: HashSet<String>) -> Self {
        Self {
            auth,
            employee_emails,
        }
    }

    /// List all documents shared with the authenticated user.
    pub fn list_shared_documents(&self) -> Result<Vec<DriveFile>, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Query for documents where user has access (shared with them)
        // Filter for Google Docs and Google Sheets
        let query = "mimeType='application/vnd.google-apps.document' or mimeType='application/vnd.google-apps.spreadsheet' or mimeType='application/vnd.google-apps.presentation'";

        let url = format!(
            "https://www.googleapis.com/drive/v3/files?q={}&fields=files(id,name,mimeType,owners)",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to list documents: {} - {}", status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let files_response: FilesListResponse = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        Ok(files_response.files.unwrap_or_default())
    }

    /// List comments on a specific document.
    pub fn list_comments(&self, document_id: &str) -> Result<Vec<GoogleDocsComment>, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments?fields=comments(id,content,htmlContent,resolved,author,createdTime,modifiedTime,replies,anchor,quotedFileContent)",
            document_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to list comments for {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let comments_response: CommentsListResponse = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        Ok(comments_response.comments.unwrap_or_default())
    }

    /// Filter comments to find ones that mention an employee and haven't been processed.
    /// Returns ActionableComment items that include tracking IDs for both comments and replies.
    pub fn filter_actionable_comments(
        &self,
        comments: &[GoogleDocsComment],
        processed_ids: &HashSet<String>,
    ) -> Vec<ActionableComment> {
        let mut actionable = Vec::new();

        for comment in comments {
            // Skip resolved comments entirely
            if comment.resolved == Some(true) {
                continue;
            }

            // Check the parent comment (if not already processed)
            let comment_tracking_id = format!("comment:{}", comment.id);
            if !processed_ids.contains(&comment_tracking_id) {
                // Skip comments from our own employee accounts
                let is_from_employee = comment.author.as_ref()
                    .and_then(|a| a.email_address.as_ref())
                    .map(|e| self.employee_emails.contains(e))
                    .unwrap_or(false);

                if !is_from_employee && contains_employee_mention(&comment.content) {
                    let comment_preview = comment.content.chars().take(50).collect::<String>();
                    info!("Found actionable comment: '{}'", comment_preview);
                    actionable.push(ActionableComment::from_comment(comment.clone()));
                    continue; // Don't check replies if parent comment itself is actionable
                }
            }

            // Check replies (even if parent comment was processed)
            if let Some(ref replies) = comment.replies {
                for reply in replies {
                    let reply_tracking_id = format!("comment:{}:reply:{}", comment.id, reply.id);

                    // Skip already processed replies
                    if processed_ids.contains(&reply_tracking_id) {
                        continue;
                    }

                    // Skip replies from our own accounts
                    let is_from_employee = reply.author.as_ref()
                        .and_then(|a| a.email_address.as_ref())
                        .map(|e| self.employee_emails.contains(e))
                        .unwrap_or(false);

                    if is_from_employee {
                        continue;
                    }

                    // Check if reply mentions an employee
                    if contains_employee_mention(&reply.content) {
                        let reply_preview = reply.content.chars().take(50).collect::<String>();
                        info!("Found actionable reply: '{}'", reply_preview);
                        actionable.push(ActionableComment::from_reply(comment.clone(), reply.clone()));
                    }
                }
            }
        }

        actionable
    }

    /// Convert a Google Docs comment to an InboundMessage.
    /// This method is kept for backward compatibility.
    pub fn comment_to_inbound_message(
        &self,
        document_id: &str,
        document_name: &str,
        comment: &GoogleDocsComment,
    ) -> InboundMessage {
        let actionable = ActionableComment::from_comment(comment.clone());
        self.actionable_to_inbound_message(document_id, document_name, &actionable)
    }

    /// Convert an ActionableComment to an InboundMessage.
    /// Handles both parent comments and replies correctly.
    pub fn actionable_to_inbound_message(
        &self,
        document_id: &str,
        document_name: &str,
        actionable: &ActionableComment,
    ) -> InboundMessage {
        // Get sender from the triggering item (reply or parent comment)
        let sender = actionable.triggering_author()
            .and_then(|a| a.email_address.clone())
            .unwrap_or_else(|| "unknown@unknown.com".to_string());

        let sender_name = actionable.triggering_author()
            .and_then(|a| a.display_name.clone());

        // Build text body with context
        let mut text_body = actionable.triggering_content().to_string();

        // Add context about the conversation thread
        if let Some(ref reply) = actionable.triggering_reply {
            // This is a reply - include the original comment for context
            let original_content = &actionable.comment.content;
            let original_author = actionable.comment.author.as_ref()
                .and_then(|a| a.display_name.as_deref())
                .unwrap_or("Someone");

            text_body = format!(
                "Original comment by {}: \"{}\"\n\nReply: {}",
                original_author, original_content, reply.content
            );
        }

        // Add quoted content if available (from parent comment)
        if let Some(ref quoted) = actionable.comment.quoted_file_content {
            if let Some(ref value) = quoted.value {
                text_body = format!("Quoted text from document: \"{}\"\n\n{}", value, text_body);
            }
        }

        // Thread ID is document_id + comment_id (always use parent comment ID)
        let thread_id = format!("{}:{}", document_id, actionable.comment.id);

        let reply_to = vec![sender.clone()];

        // Message ID includes reply ID if this is a reply
        let message_id = if let Some(ref reply) = actionable.triggering_reply {
            format!("{}:{}", actionable.comment.id, reply.id)
        } else {
            actionable.comment.id.clone()
        };

        InboundMessage {
            channel: Channel::GoogleDocs,
            sender,
            sender_name,
            recipient: "oliver@dowhiz.com".to_string(), // TODO: Make configurable
            subject: Some(format!("Comment on: {}", document_name)),
            text_body: Some(text_body),
            html_body: actionable.triggering_reply.as_ref()
                .and_then(|r| r.html_content.clone())
                .or_else(|| actionable.comment.html_content.clone()),
            thread_id,
            message_id: Some(message_id),
            attachments: vec![],
            reply_to,
            raw_payload: serde_json::to_vec(&actionable.comment).unwrap_or_default(),
            metadata: ChannelMetadata {
                google_docs_document_id: Some(document_id.to_string()),
                google_docs_comment_id: Some(actionable.comment.id.clone()),
                google_docs_document_name: Some(document_name.to_string()),
                ..Default::default()
            },
        }
    }

    /// Read document content as plain text for agent context.
    /// This fetches the full document content so the agent can understand and edit it.
    pub fn read_document_content(&self, document_id: &str) -> Result<String, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Export document as plain text
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType=text/plain",
            document_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to read document {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        response
            .text()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }
}

impl InboundAdapter for GoogleDocsInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        // This adapter is poll-based, so parse is used to convert a comment to InboundMessage
        let _comment: GoogleDocsComment =
            serde_json::from_slice(raw_payload).map_err(|e| AdapterError::ParseError(e.to_string()))?;

        // We need document info which isn't in the raw payload
        // This method is less useful for poll-based adapters
        Err(AdapterError::ParseError(
            "GoogleDocsInboundAdapter is poll-based; use comment_to_inbound_message instead".to_string(),
        ))
    }

    fn channel(&self) -> Channel {
        Channel::GoogleDocs
    }
}

/// Adapter for posting replies to Google Docs comments.
#[derive(Debug, Clone)]
pub struct GoogleDocsOutboundAdapter {
    /// Google authentication
    auth: GoogleAuth,
}

impl GoogleDocsOutboundAdapter {
    pub fn new(auth: GoogleAuth) -> Self {
        Self { auth }
    }

    /// Post a reply to a comment.
    pub fn reply_to_comment(
        &self,
        document_id: &str,
        comment_id: &str,
        reply_content: &str,
    ) -> Result<CommentReply, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Google Drive API v3 requires 'fields' parameter to specify response fields
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/comments/{}/replies?fields=id,content,createdTime,author",
            document_id, comment_id
        );

        let payload = serde_json::json!({
            "content": reply_content
        });

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
                comment_id, document_id, status, body
            );
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        let reply: CommentReply = response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        info!(
            "Posted reply {} to comment {} on document {}",
            reply.id, comment_id, document_id
        );

        Ok(reply)
    }

    /// Read document content (for context when processing comments).
    pub fn read_document_content(&self, document_id: &str) -> Result<String, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Export document as plain text
        let url = format!(
            "https://www.googleapis.com/drive/v3/files/{}/export?mimeType=text/plain",
            document_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to read document {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        response
            .text()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }

    /// Apply an edit to the document (direct edit, not suggestion mode).
    /// Note: Google Docs API does not support creating suggestions programmatically.
    pub fn apply_document_edit(
        &self,
        document_id: &str,
        requests: Vec<serde_json::Value>,
    ) -> Result<(), AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://docs.googleapis.com/v1/documents/{}:batchUpdate",
            document_id
        );

        let payload = serde_json::json!({
            "requests": requests
        });

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
            error!("Failed to apply edit to {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        info!("Applied edit to document {}", document_id);
        Ok(())
    }

    /// Get document structure to find text positions.
    /// Returns the document body content with start/end indices.
    pub fn get_document_structure(
        &self,
        document_id: &str,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://docs.googleapis.com/v1/documents/{}",
            document_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!("Failed to get document structure {}: {} - {}", document_id, status, body);
            return Err(AdapterError::SendError(format!("HTTP {}: {}", status, body)));
        }

        response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }

    /// Find text in document and return its start and end indices.
    /// Returns (start_index, end_index) or None if not found.
    pub fn find_text_position(
        &self,
        document_id: &str,
        search_text: &str,
    ) -> Result<Option<(i64, i64)>, AdapterError> {
        let doc = self.get_document_structure(document_id)?;

        // Extract body content
        let body = doc.get("body").and_then(|b| b.get("content"));
        if body.is_none() {
            return Ok(None);
        }

        let content = body.unwrap().as_array().ok_or_else(|| {
            AdapterError::ParseError("Invalid document structure".to_string())
        })?;

        // Build full text and track positions
        let mut full_text = String::new();
        let mut text_positions: Vec<(usize, i64)> = Vec::new(); // (string_pos, doc_index)

        for element in content {
            if let Some(paragraph) = element.get("paragraph") {
                if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(text_run) = elem.get("textRun") {
                            if let Some(content_text) = text_run.get("content").and_then(|c| c.as_str()) {
                                let start_idx = elem.get("startIndex").and_then(|i| i.as_i64()).unwrap_or(0);
                                text_positions.push((full_text.len(), start_idx));
                                full_text.push_str(content_text);
                            }
                        }
                    }
                }
            }
        }

        // Find the search text in full_text
        if let Some(string_pos) = full_text.find(search_text) {
            // Convert string position to document index
            let mut doc_start_idx = 0i64;
            for (str_pos, doc_idx) in &text_positions {
                if *str_pos <= string_pos {
                    doc_start_idx = *doc_idx + (string_pos - str_pos) as i64;
                }
            }
            let doc_end_idx = doc_start_idx + search_text.len() as i64;
            return Ok(Some((doc_start_idx, doc_end_idx)));
        }

        Ok(None)
    }

    /// Mark text for deletion with red color and strikethrough.
    /// Used in suggesting mode to show text that will be removed.
    pub fn mark_deletion(
        &self,
        document_id: &str,
        text_to_mark: &str,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, text_to_mark)?;

        let (start_idx, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Text not found in document: '{}'", text_to_mark))
        })?;

        // Apply red color and strikethrough
        let requests = vec![
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 1.0,
                                    "green": 0.0,
                                    "blue": 0.0
                                }
                            }
                        },
                        "strikethrough": true
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Marked deletion '{}' at indices {}-{}", text_to_mark, start_idx, end_idx);
        Ok(())
    }

    /// Insert new text with blue color (suggesting mode).
    /// The text is inserted after the specified anchor text.
    pub fn insert_suggestion(
        &self,
        document_id: &str,
        after_text: &str,
        new_text: &str,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, after_text)?;

        let (_, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Anchor text not found: '{}'", after_text))
        })?;

        // Insert text and make it blue (explicitly remove strikethrough in case anchor has it)
        let requests = vec![
            serde_json::json!({
                "insertText": {
                    "location": {
                        "index": end_idx
                    },
                    "text": new_text
                }
            }),
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": end_idx,
                        "endIndex": end_idx + new_text.chars().count() as i64
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 0.0,
                                    "green": 0.0,
                                    "blue": 1.0
                                }
                            }
                        },
                        "strikethrough": false
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Inserted suggestion '{}' after '{}'", new_text, after_text);
        Ok(())
    }

    /// Replace text with revision marks (suggesting mode).
    /// Old text gets red + strikethrough, new text gets blue.
    pub fn suggest_replace(
        &self,
        document_id: &str,
        old_text: &str,
        new_text: &str,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, old_text)?;

        let (start_idx, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Text to replace not found: '{}'", old_text))
        })?;

        // First, mark old text as deleted (red + strikethrough)
        // Then insert new text (blue) right after the old text
        let requests = vec![
            // Mark old text as deleted
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 1.0,
                                    "green": 0.0,
                                    "blue": 0.0
                                }
                            }
                        },
                        "strikethrough": true
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            }),
            // Insert new text right after old text
            serde_json::json!({
                "insertText": {
                    "location": {
                        "index": end_idx
                    },
                    "text": new_text
                }
            }),
            // Make new text blue (and explicitly remove strikethrough since it may inherit from previous text)
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": end_idx,
                        "endIndex": end_idx + new_text.chars().count() as i64
                    },
                    "textStyle": {
                        "foregroundColor": {
                            "color": {
                                "rgbColor": {
                                    "red": 0.0,
                                    "green": 0.0,
                                    "blue": 1.0
                                }
                            }
                        },
                        "strikethrough": false
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Suggested replacement: '{}' -> '{}'", old_text, new_text);
        Ok(())
    }

    /// Apply all suggestions in the document.
    /// Deletes all red strikethrough text and converts blue text to black.
    pub fn apply_suggestions(&self, document_id: &str) -> Result<(), AdapterError> {
        let doc = self.get_document_structure(document_id)?;

        let body = doc.get("body").and_then(|b| b.get("content"));
        if body.is_none() {
            return Ok(());
        }

        let content = body.unwrap().as_array().ok_or_else(|| {
            AdapterError::ParseError("Invalid document structure".to_string())
        })?;

        // Collect ranges to delete (red strikethrough) and ranges to normalize (blue)
        let mut ranges_to_delete: Vec<(i64, i64)> = Vec::new();
        let mut ranges_to_normalize: Vec<(i64, i64)> = Vec::new();

        for element in content {
            if let Some(paragraph) = element.get("paragraph") {
                if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(text_run) = elem.get("textRun") {
                            let start_idx = elem.get("startIndex").and_then(|i| i.as_i64()).unwrap_or(0);
                            let end_idx = elem.get("endIndex").and_then(|i| i.as_i64()).unwrap_or(0);

                            if let Some(text_style) = text_run.get("textStyle") {
                                // Check for red strikethrough (deletion markers)
                                let is_strikethrough = text_style.get("strikethrough")
                                    .and_then(|s| s.as_bool())
                                    .unwrap_or(false);
                                let is_red = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r > 0.8 && g < 0.2 && b < 0.2 // Check if red
                                    })
                                    .unwrap_or(false);

                                if is_strikethrough && is_red {
                                    ranges_to_delete.push((start_idx, end_idx));
                                    continue;
                                }

                                // Check for blue text (addition markers)
                                let is_blue = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r < 0.2 && g < 0.2 && b > 0.8 // Check if blue
                                    })
                                    .unwrap_or(false);

                                if is_blue {
                                    ranges_to_normalize.push((start_idx, end_idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Build requests: delete red text (in reverse order) then normalize blue text
        let mut requests: Vec<serde_json::Value> = Vec::new();

        // Sort ranges_to_delete in reverse order (to avoid index shifting issues)
        let mut sorted_delete = ranges_to_delete.clone();
        sorted_delete.sort_by(|a, b| b.0.cmp(&a.0));

        for (start_idx, end_idx) in sorted_delete {
            requests.push(serde_json::json!({
                "deleteContentRange": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    }
                }
            }));
        }

        // Normalize blue text to black (remove color)
        let ranges_to_normalize_len = ranges_to_normalize.len();
        for (start_idx, end_idx) in ranges_to_normalize {
            // Adjust indices based on deletions that occurred before this range
            let mut adjusted_start = start_idx;
            let mut adjusted_end = end_idx;
            for (del_start, del_end) in &ranges_to_delete {
                if *del_end <= start_idx {
                    let deleted_length = del_end - del_start;
                    adjusted_start -= deleted_length;
                    adjusted_end -= deleted_length;
                }
            }

            requests.push(serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": adjusted_start,
                        "endIndex": adjusted_end
                    },
                    "textStyle": {
                        "foregroundColor": {}  // Reset to default (black)
                    },
                    "fields": "foregroundColor"
                }
            }));
        }

        if !requests.is_empty() {
            self.apply_document_edit(document_id, requests)?;
            info!("Applied suggestions: deleted {} ranges, normalized {} ranges",
                  ranges_to_delete.len(), ranges_to_normalize_len);
        }

        Ok(())
    }

    /// Discard all suggestions in the document.
    /// Removes blue text and restores red strikethrough text to normal.
    pub fn discard_suggestions(&self, document_id: &str) -> Result<(), AdapterError> {
        let doc = self.get_document_structure(document_id)?;

        let body = doc.get("body").and_then(|b| b.get("content"));
        if body.is_none() {
            return Ok(());
        }

        let content = body.unwrap().as_array().ok_or_else(|| {
            AdapterError::ParseError("Invalid document structure".to_string())
        })?;

        // Collect ranges to delete (blue text) and ranges to restore (red strikethrough)
        let mut ranges_to_delete: Vec<(i64, i64)> = Vec::new();
        let mut ranges_to_restore: Vec<(i64, i64)> = Vec::new();

        for element in content {
            if let Some(paragraph) = element.get("paragraph") {
                if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                    for elem in elements {
                        if let Some(text_run) = elem.get("textRun") {
                            let start_idx = elem.get("startIndex").and_then(|i| i.as_i64()).unwrap_or(0);
                            let end_idx = elem.get("endIndex").and_then(|i| i.as_i64()).unwrap_or(0);

                            if let Some(text_style) = text_run.get("textStyle") {
                                // Check for blue text (to be deleted)
                                let is_blue = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r < 0.2 && g < 0.2 && b > 0.8
                                    })
                                    .unwrap_or(false);

                                if is_blue {
                                    ranges_to_delete.push((start_idx, end_idx));
                                    continue;
                                }

                                // Check for red strikethrough (to be restored)
                                let is_strikethrough = text_style.get("strikethrough")
                                    .and_then(|s| s.as_bool())
                                    .unwrap_or(false);
                                let is_red = text_style.get("foregroundColor")
                                    .and_then(|fc| fc.get("color"))
                                    .and_then(|c| c.get("rgbColor"))
                                    .map(|rgb| {
                                        let r = rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let g = rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        let b = rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                        r > 0.8 && g < 0.2 && b < 0.2
                                    })
                                    .unwrap_or(false);

                                if is_strikethrough && is_red {
                                    ranges_to_restore.push((start_idx, end_idx));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Build requests: delete blue text (in reverse order) then restore red text
        let mut requests: Vec<serde_json::Value> = Vec::new();

        // Sort ranges_to_delete in reverse order
        let mut sorted_delete = ranges_to_delete.clone();
        sorted_delete.sort_by(|a, b| b.0.cmp(&a.0));

        for (start_idx, end_idx) in sorted_delete {
            requests.push(serde_json::json!({
                "deleteContentRange": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    }
                }
            }));
        }

        // Restore red text to normal (remove color and strikethrough)
        let ranges_to_restore_len = ranges_to_restore.len();
        for (start_idx, end_idx) in ranges_to_restore {
            // Adjust indices based on deletions
            let mut adjusted_start = start_idx;
            let mut adjusted_end = end_idx;
            for (del_start, del_end) in &ranges_to_delete {
                if *del_end <= start_idx {
                    let deleted_length = del_end - del_start;
                    adjusted_start -= deleted_length;
                    adjusted_end -= deleted_length;
                }
            }

            requests.push(serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": adjusted_start,
                        "endIndex": adjusted_end
                    },
                    "textStyle": {
                        "foregroundColor": {},  // Reset to default
                        "strikethrough": false
                    },
                    "fields": "foregroundColor,strikethrough"
                }
            }));
        }

        if !requests.is_empty() {
            self.apply_document_edit(document_id, requests)?;
            info!("Discarded suggestions: deleted {} ranges, restored {} ranges",
                  ranges_to_delete.len(), ranges_to_restore_len);
        }

        Ok(())
    }

    /// Get existing styles from the document, useful for maintaining consistent formatting.
    /// Returns a summary of styles found for different heading levels and body text.
    pub fn get_document_styles(&self, document_id: &str) -> Result<DocumentStyles, AdapterError> {
        let doc = self.get_document_structure(document_id)?;
        let mut styles = DocumentStyles::default();

        // Get named styles (heading styles defined in the document)
        if let Some(named_styles) = doc.get("namedStyles").and_then(|ns| ns.get("styles")).and_then(|s| s.as_array()) {
            for style in named_styles {
                if let Some(name) = style.get("namedStyleType").and_then(|n| n.as_str()) {
                    let text_style = style.get("textStyle");
                    let paragraph_style = style.get("paragraphStyle");

                    let style_info = TextStyleInfo {
                        foreground_color: text_style
                            .and_then(|ts| ts.get("foregroundColor"))
                            .and_then(|fc| fc.get("color"))
                            .and_then(|c| c.get("rgbColor"))
                            .map(|rgb| {
                                let r = (rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0) * 255.0) as u8;
                                let g = (rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0) * 255.0) as u8;
                                let b = (rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0) * 255.0) as u8;
                                format!("#{:02X}{:02X}{:02X}", r, g, b)
                            }),
                        font_family: text_style
                            .and_then(|ts| ts.get("weightedFontFamily"))
                            .and_then(|wf| wf.get("fontFamily"))
                            .and_then(|f| f.as_str())
                            .map(|s| s.to_string()),
                        font_size: text_style
                            .and_then(|ts| ts.get("fontSize"))
                            .and_then(|fs| fs.get("magnitude"))
                            .and_then(|m| m.as_f64()),
                        bold: text_style
                            .and_then(|ts| ts.get("bold"))
                            .and_then(|b| b.as_bool()),
                        italic: text_style
                            .and_then(|ts| ts.get("italic"))
                            .and_then(|i| i.as_bool()),
                        alignment: paragraph_style
                            .and_then(|ps| ps.get("alignment"))
                            .and_then(|a| a.as_str())
                            .map(|s| s.to_string()),
                    };

                    match name {
                        "HEADING_1" => styles.heading_1 = Some(style_info),
                        "HEADING_2" => styles.heading_2 = Some(style_info),
                        "HEADING_3" => styles.heading_3 = Some(style_info),
                        "HEADING_4" => styles.heading_4 = Some(style_info),
                        "HEADING_5" => styles.heading_5 = Some(style_info),
                        "HEADING_6" => styles.heading_6 = Some(style_info),
                        "NORMAL_TEXT" => styles.normal_text = Some(style_info),
                        "TITLE" => styles.title = Some(style_info),
                        "SUBTITLE" => styles.subtitle = Some(style_info),
                        _ => {}
                    }
                }
            }
        }

        // Also scan document body for actual styles used (in case they differ from named styles)
        if let Some(body) = doc.get("body").and_then(|b| b.get("content")).and_then(|c| c.as_array()) {
            for element in body {
                if let Some(paragraph) = element.get("paragraph") {
                    let para_style = paragraph.get("paragraphStyle");
                    let named_style = para_style
                        .and_then(|ps| ps.get("namedStyleType"))
                        .and_then(|n| n.as_str());

                    // Get sample text and its style
                    if let Some(elements) = paragraph.get("elements").and_then(|e| e.as_array()) {
                        for elem in elements {
                            if let Some(text_run) = elem.get("textRun") {
                                if let Some(content) = text_run.get("content").and_then(|c| c.as_str()) {
                                    let content_trimmed = content.trim();
                                    if !content_trimmed.is_empty() && content_trimmed.len() > 1 {
                                        if let Some(text_style) = text_run.get("textStyle") {
                                            let style_info = TextStyleInfo {
                                                foreground_color: text_style
                                                    .get("foregroundColor")
                                                    .and_then(|fc| fc.get("color"))
                                                    .and_then(|c| c.get("rgbColor"))
                                                    .map(|rgb| {
                                                        let r = (rgb.get("red").and_then(|v| v.as_f64()).unwrap_or(0.0) * 255.0) as u8;
                                                        let g = (rgb.get("green").and_then(|v| v.as_f64()).unwrap_or(0.0) * 255.0) as u8;
                                                        let b = (rgb.get("blue").and_then(|v| v.as_f64()).unwrap_or(0.0) * 255.0) as u8;
                                                        format!("#{:02X}{:02X}{:02X}", r, g, b)
                                                    }),
                                                font_family: text_style
                                                    .get("weightedFontFamily")
                                                    .and_then(|wf| wf.get("fontFamily"))
                                                    .and_then(|f| f.as_str())
                                                    .map(|s| s.to_string()),
                                                font_size: text_style
                                                    .get("fontSize")
                                                    .and_then(|fs| fs.get("magnitude"))
                                                    .and_then(|m| m.as_f64()),
                                                bold: text_style.get("bold").and_then(|b| b.as_bool()),
                                                italic: text_style.get("italic").and_then(|i| i.as_bool()),
                                                alignment: None,
                                            };

                                            // Store sample for each heading type
                                            match named_style {
                                                Some("HEADING_1") if styles.heading_1_sample.is_none() => {
                                                    styles.heading_1_sample = Some((content_trimmed.to_string(), style_info));
                                                }
                                                Some("HEADING_2") if styles.heading_2_sample.is_none() => {
                                                    styles.heading_2_sample = Some((content_trimmed.to_string(), style_info));
                                                }
                                                Some("HEADING_3") if styles.heading_3_sample.is_none() => {
                                                    styles.heading_3_sample = Some((content_trimmed.to_string(), style_info));
                                                }
                                                _ => {}
                                            }
                                        }
                                        break; // Only need first text run per paragraph
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(styles)
    }

    /// Set text style for specified text in the document.
    /// Supports color (hex like "#FF0000"), font family, font size, bold, italic.
    pub fn set_text_style(
        &self,
        document_id: &str,
        text_to_style: &str,
        color: Option<&str>,
        font_family: Option<&str>,
        font_size: Option<f64>,
        bold: Option<bool>,
        italic: Option<bool>,
    ) -> Result<(), AdapterError> {
        let position = self.find_text_position(document_id, text_to_style)?;

        let (start_idx, end_idx) = position.ok_or_else(|| {
            AdapterError::SendError(format!("Text not found in document: '{}'", text_to_style))
        })?;

        let mut text_style = serde_json::Map::new();
        let mut fields = Vec::new();

        // Parse hex color like "#FF0000" or "FF0000"
        if let Some(color_str) = color {
            let hex = color_str.trim_start_matches('#');
            if hex.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    text_style.insert("foregroundColor".to_string(), serde_json::json!({
                        "color": {
                            "rgbColor": {
                                "red": r as f64 / 255.0,
                                "green": g as f64 / 255.0,
                                "blue": b as f64 / 255.0
                            }
                        }
                    }));
                    fields.push("foregroundColor");
                }
            }
        }

        if let Some(font) = font_family {
            text_style.insert("weightedFontFamily".to_string(), serde_json::json!({
                "fontFamily": font,
                "weight": 400
            }));
            fields.push("weightedFontFamily");
        }

        if let Some(size) = font_size {
            text_style.insert("fontSize".to_string(), serde_json::json!({
                "magnitude": size,
                "unit": "PT"
            }));
            fields.push("fontSize");
        }

        if let Some(b) = bold {
            text_style.insert("bold".to_string(), serde_json::json!(b));
            fields.push("bold");
        }

        if let Some(i) = italic {
            text_style.insert("italic".to_string(), serde_json::json!(i));
            fields.push("italic");
        }

        if fields.is_empty() {
            return Err(AdapterError::ConfigError("No style properties specified".to_string()));
        }

        let requests = vec![
            serde_json::json!({
                "updateTextStyle": {
                    "range": {
                        "startIndex": start_idx,
                        "endIndex": end_idx
                    },
                    "textStyle": text_style,
                    "fields": fields.join(",")
                }
            })
        ];

        self.apply_document_edit(document_id, requests)?;
        info!("Applied style to '{}' at indices {}-{}: fields={:?}",
              text_to_style, start_idx, end_idx, fields);
        Ok(())
    }
}

/// Style information for a text element
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct TextStyleInfo {
    pub foreground_color: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub alignment: Option<String>,
}

/// Document styles summary
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct DocumentStyles {
    pub title: Option<TextStyleInfo>,
    pub subtitle: Option<TextStyleInfo>,
    pub heading_1: Option<TextStyleInfo>,
    pub heading_2: Option<TextStyleInfo>,
    pub heading_3: Option<TextStyleInfo>,
    pub heading_4: Option<TextStyleInfo>,
    pub heading_5: Option<TextStyleInfo>,
    pub heading_6: Option<TextStyleInfo>,
    pub normal_text: Option<TextStyleInfo>,
    // Samples of actual styled text found in the document
    pub heading_1_sample: Option<(String, TextStyleInfo)>,
    pub heading_2_sample: Option<(String, TextStyleInfo)>,
    pub heading_3_sample: Option<(String, TextStyleInfo)>,
}

impl OutboundAdapter for GoogleDocsOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let document_id = message
            .metadata
            .google_docs_document_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing document ID".to_string()))?;

        let comment_id = message
            .metadata
            .google_docs_comment_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing comment ID".to_string()))?;

        // Use text_body as the reply content
        let reply_content = if !message.text_body.is_empty() {
            &message.text_body
        } else {
            &message.html_body
        };

        let reply = self.reply_to_comment(document_id, comment_id, reply_content)?;

        Ok(SendResult {
            success: true,
            message_id: reply.id,
            submitted_at: reply.created_time.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            error: None,
        })
    }

    fn channel(&self) -> Channel {
        Channel::GoogleDocs
    }
}

/// Format a proposal for document edit as a comment reply.
pub fn format_edit_proposal(
    original_text: &str,
    proposed_text: &str,
    explanation: Option<&str>,
) -> String {
    let mut reply = String::new();

    reply.push_str("Here's my suggested edit:\n\n");
    reply.push_str("**Original:**\n");
    reply.push_str(&format!("\"{}\"", original_text));
    reply.push_str("\n\n");
    reply.push_str("**Suggested:**\n");
    reply.push_str(&format!("\"{}\"", proposed_text));

    if let Some(exp) = explanation {
        reply.push_str("\n\n");
        reply.push_str("**Reason:** ");
        reply.push_str(exp);
    }

    reply.push_str("\n\nReply \"apply\" to confirm this edit, or let me know if you'd like any changes.");

    reply
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_mention_detection() {
        assert!(contains_employee_mention("Hey oliver can you help?"));
        assert!(contains_employee_mention("@Oliver please review"));
        assert!(contains_employee_mention("Oliver, check this"));
        assert!(contains_employee_mention("Contact oliver@dowhiz.com"));
        assert!(contains_employee_mention("little_bear please fix"));
        assert!(contains_employee_mention("little bear help me"));
        assert!(contains_employee_mention("OLIVER look at this"));

        assert!(contains_employee_mention("Hey maggie can you help?"));
        assert!(contains_employee_mention("mini_mouse please check"));

        assert!(!contains_employee_mention("Hey John can you help?"));
        assert!(!contains_employee_mention("This is a regular comment"));
    }

    #[test]
    fn test_extract_employee_name() {
        assert_eq!(extract_employee_name("Hey oliver"), Some("Oliver"));
        assert_eq!(extract_employee_name("@Oliver please"), Some("Oliver"));
        assert_eq!(extract_employee_name("little_bear help"), Some("Oliver"));
        assert_eq!(extract_employee_name("maggie check"), Some("Maggie"));
        assert_eq!(extract_employee_name("mini mouse help"), Some("Maggie"));
        assert_eq!(extract_employee_name("John help"), None);
    }

    #[test]
    fn test_format_edit_proposal() {
        let proposal = format_edit_proposal(
            "The quick brown fox",
            "The swift brown fox",
            Some("'Swift' is more descriptive"),
        );

        assert!(proposal.contains("**Original:**"));
        assert!(proposal.contains("The quick brown fox"));
        assert!(proposal.contains("**Suggested:**"));
        assert!(proposal.contains("The swift brown fox"));
        assert!(proposal.contains("**Reason:**"));
        assert!(proposal.contains("apply"));
    }
}
