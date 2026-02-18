use std::collections::HashSet;

use tracing::{error, info};

use crate::channel::{
    AdapterError, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
};
use crate::google_auth::GoogleAuth;

use super::mentions::contains_employee_mention;
use super::models::{
    ActionableComment, CommentsListResponse, DriveFile, FilesListResponse, GoogleDocsComment,
};

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
                let is_from_employee = comment
                    .author
                    .as_ref()
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
                    let is_from_employee = reply
                        .author
                        .as_ref()
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
        let sender = actionable
            .triggering_author()
            .and_then(|a| a.email_address.clone())
            .unwrap_or_else(|| "unknown@unknown.com".to_string());

        let sender_name = actionable
            .triggering_author()
            .and_then(|a| a.display_name.clone());

        // Build text body with context
        let mut text_body = actionable.triggering_content().to_string();

        // Add context about the conversation thread
        if let Some(ref reply) = actionable.triggering_reply {
            // This is a reply - include the original comment for context
            let original_content = &actionable.comment.content;
            let original_author = actionable
                .comment
                .author
                .as_ref()
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
            html_body: actionable
                .triggering_reply
                .as_ref()
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
