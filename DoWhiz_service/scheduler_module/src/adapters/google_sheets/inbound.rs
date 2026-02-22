//! Inbound adapter for Google Sheets comments.

use std::collections::HashSet;

use tracing::error;

use crate::channel::{AdapterError, Channel, ChannelMetadata, InboundAdapter, InboundMessage};
use crate::google_auth::GoogleAuth;

use super::super::google_common::{
    ActionableComment, GoogleComment, GoogleCommentsClient, GoogleFileType,
};
use super::super::google_docs::contains_employee_mention;

/// Adapter for polling Google Sheets comments.
#[derive(Debug, Clone)]
pub struct GoogleSheetsInboundAdapter {
    comments_client: GoogleCommentsClient,
    auth: GoogleAuth,
}

impl GoogleSheetsInboundAdapter {
    pub fn new(auth: GoogleAuth, employee_emails: HashSet<String>) -> Self {
        let comments_client =
            GoogleCommentsClient::new(auth.clone(), employee_emails, contains_employee_mention);
        Self {
            comments_client,
            auth,
        }
    }

    /// List all spreadsheets shared with the authenticated user.
    pub fn list_shared_spreadsheets(
        &self,
    ) -> Result<Vec<super::super::google_common::DriveFile>, AdapterError> {
        self.comments_client
            .list_shared_files()
            .map(|files| {
                files
                    .into_iter()
                    .filter(|f| f.file_type() == GoogleFileType::Sheets)
                    .collect()
            })
    }

    /// List comments on a specific spreadsheet.
    pub fn list_comments(
        &self,
        spreadsheet_id: &str,
    ) -> Result<Vec<GoogleComment>, AdapterError> {
        self.comments_client.list_comments(spreadsheet_id)
    }

    /// Filter comments to find actionable ones.
    pub fn filter_actionable_comments(
        &self,
        comments: &[GoogleComment],
        processed_ids: &HashSet<String>,
    ) -> Vec<ActionableComment> {
        self.comments_client
            .filter_actionable_comments(comments, processed_ids)
    }

    /// Convert an ActionableComment to an InboundMessage.
    pub fn actionable_to_inbound_message(
        &self,
        spreadsheet_id: &str,
        spreadsheet_name: &str,
        actionable: &ActionableComment,
    ) -> InboundMessage {
        let sender = actionable
            .triggering_author()
            .and_then(|a| a.email_address.clone())
            .unwrap_or_else(|| "unknown@unknown.com".to_string());

        let sender_name = actionable
            .triggering_author()
            .and_then(|a| a.display_name.clone());

        let mut text_body = actionable.triggering_content().to_string();

        if let Some(ref reply) = actionable.triggering_reply {
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

        if let Some(ref quoted) = actionable.comment.quoted_file_content {
            if let Some(ref value) = quoted.value {
                text_body = format!("Quoted content: \"{}\"\n\n{}", value, text_body);
            }
        }

        let thread_id = format!("{}:{}", spreadsheet_id, actionable.comment.id);
        let reply_to = vec![sender.clone()];

        let message_id = if let Some(ref reply) = actionable.triggering_reply {
            format!("{}:{}", actionable.comment.id, reply.id)
        } else {
            actionable.comment.id.clone()
        };

        InboundMessage {
            channel: Channel::GoogleSheets,
            sender,
            sender_name,
            recipient: "oliver@dowhiz.com".to_string(),
            subject: Some(format!("Comment on: {}", spreadsheet_name)),
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
                google_sheets_spreadsheet_id: Some(spreadsheet_id.to_string()),
                google_sheets_comment_id: Some(actionable.comment.id.clone()),
                google_sheets_spreadsheet_name: Some(spreadsheet_name.to_string()),
                ..Default::default()
            },
        }
    }

    /// Read spreadsheet content as CSV for agent context.
    pub fn read_spreadsheet_content(
        &self,
        spreadsheet_id: &str,
    ) -> Result<String, AdapterError> {
        self.comments_client
            .export_file_content(spreadsheet_id, "text/csv")
    }

    /// Read spreadsheet data using Sheets API (structured JSON).
    pub fn read_spreadsheet_values(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        // Use raw URL - the ! character is valid in URL paths
        let base_url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
            spreadsheet_id,
            range
        );

        let response = client
            .get(&base_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to read spreadsheet {}: {} - {}",
                spreadsheet_id, status, body
            );
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        response
            .json()
            .map_err(|e| AdapterError::ParseError(e.to_string()))
    }
}

impl InboundAdapter for GoogleSheetsInboundAdapter {
    fn parse(&self, _raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        Err(AdapterError::ParseError(
            "GoogleSheetsInboundAdapter is poll-based; use actionable_to_inbound_message instead"
                .to_string(),
        ))
    }

    fn channel(&self) -> Channel {
        Channel::GoogleSheets
    }
}
