//! Outbound adapter for Google Sheets.

use tracing::{error, info};

use crate::channel::{AdapterError, Channel, OutboundAdapter, OutboundMessage, SendResult};
use crate::google_auth::GoogleAuth;

use super::super::google_common::{CommentReply, GoogleCommentsClient};
use super::super::google_docs::contains_employee_mention;
use std::collections::HashSet;

/// Adapter for posting replies to Google Sheets comments and editing spreadsheets.
#[derive(Debug, Clone)]
pub struct GoogleSheetsOutboundAdapter {
    auth: GoogleAuth,
    comments_client: GoogleCommentsClient,
}

impl GoogleSheetsOutboundAdapter {
    pub fn new(auth: GoogleAuth) -> Self {
        let comments_client =
            GoogleCommentsClient::new(auth.clone(), HashSet::new(), contains_employee_mention);
        Self {
            auth,
            comments_client,
        }
    }

    /// Post a reply to a comment.
    pub fn reply_to_comment(
        &self,
        spreadsheet_id: &str,
        comment_id: &str,
        reply_content: &str,
    ) -> Result<CommentReply, AdapterError> {
        self.comments_client
            .reply_to_comment(spreadsheet_id, comment_id, reply_content)
    }

    /// Read spreadsheet content as CSV.
    pub fn read_spreadsheet_content(&self, spreadsheet_id: &str) -> Result<String, AdapterError> {
        self.comments_client
            .export_file_content(spreadsheet_id, "text/csv")
    }

    /// Update cell values in a spreadsheet.
    pub fn update_values(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: Vec<Vec<serde_json::Value>>,
    ) -> Result<(), AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=USER_ENTERED",
            spreadsheet_id,
            urlencoding::encode(range)
        );

        let payload = serde_json::json!({
            "values": values
        });

        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to update spreadsheet {}: {} - {}",
                spreadsheet_id, status, body
            );
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        info!(
            "Updated values in spreadsheet {} at {}",
            spreadsheet_id, range
        );
        Ok(())
    }

    /// Batch update spreadsheet (for formatting, adding sheets, etc.).
    pub fn batch_update(
        &self,
        spreadsheet_id: &str,
        requests: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}:batchUpdate",
            spreadsheet_id
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
            error!(
                "Failed to batch update spreadsheet {}: {} - {}",
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

    /// Append rows to a spreadsheet.
    pub fn append_rows(
        &self,
        spreadsheet_id: &str,
        range: &str,
        values: Vec<Vec<serde_json::Value>>,
    ) -> Result<(), AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption=USER_ENTERED&insertDataOption=INSERT_ROWS",
            spreadsheet_id,
            urlencoding::encode(range)
        );

        let payload = serde_json::json!({
            "values": values
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
                "Failed to append rows to spreadsheet {}: {} - {}",
                spreadsheet_id, status, body
            );
            return Err(AdapterError::SendError(format!(
                "HTTP {}: {}",
                status, body
            )));
        }

        info!(
            "Appended rows to spreadsheet {} at {}",
            spreadsheet_id, range
        );
        Ok(())
    }

    /// Get spreadsheet metadata (sheet names, properties, etc.).
    pub fn get_spreadsheet_metadata(
        &self,
        spreadsheet_id: &str,
    ) -> Result<serde_json::Value, AdapterError> {
        let access_token = self
            .auth
            .get_access_token()
            .map_err(|e| AdapterError::ConfigError(e.to_string()))?;

        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}?fields=properties,sheets.properties",
            spreadsheet_id
        );

        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            error!(
                "Failed to get spreadsheet metadata {}: {} - {}",
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

impl OutboundAdapter for GoogleSheetsOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let spreadsheet_id = message
            .metadata
            .google_sheets_spreadsheet_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing spreadsheet ID".to_string()))?;

        let comment_id = message
            .metadata
            .google_sheets_comment_id
            .as_ref()
            .ok_or_else(|| AdapterError::ConfigError("Missing comment ID".to_string()))?;

        let reply_content = if !message.text_body.is_empty() {
            &message.text_body
        } else {
            &message.html_body
        };

        let reply = self.reply_to_comment(spreadsheet_id, comment_id, reply_content)?;

        Ok(SendResult {
            success: true,
            message_id: reply.id,
            submitted_at: reply
                .created_time
                .unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            error: None,
        })
    }

    fn channel(&self) -> Channel {
        Channel::GoogleSheets
    }
}
