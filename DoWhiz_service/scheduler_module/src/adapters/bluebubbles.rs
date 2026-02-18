//! BlueBubbles adapter for inbound and outbound iMessages.
//!
//! This module provides adapters for handling iMessages via BlueBubbles:
//! - `BlueBubblesInboundAdapter`: Parses BlueBubbles webhook payloads
//! - `BlueBubblesOutboundAdapter`: Sends messages via BlueBubbles REST API

use serde::{Deserialize, Serialize};

use crate::channel::{
    AdapterError, Attachment, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
    OutboundAdapter, OutboundMessage, SendResult,
};

/// Adapter for parsing BlueBubbles webhook payloads.
#[derive(Debug, Clone, Default)]
pub struct BlueBubblesInboundAdapter;

impl BlueBubblesInboundAdapter {
    pub fn new() -> Self {
        Self
    }

    /// Check if the message is from ourselves (outgoing message).
    pub fn is_from_me(&self, message: &BlueBubblesMessage) -> bool {
        message.is_from_me.unwrap_or(false)
    }
}

impl InboundAdapter for BlueBubblesInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        let wrapper: BlueBubblesWebhook = serde_json::from_slice(raw_payload)
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        // Only handle new-message events
        if wrapper.event_type != "new-message" {
            return Err(AdapterError::ParseError(format!(
                "unsupported event type: {}",
                wrapper.event_type
            )));
        }

        let message = wrapper.data;

        // Ignore outgoing messages (messages we sent)
        if self.is_from_me(&message) {
            return Err(AdapterError::ParseError(
                "ignoring outgoing message (is_from_me=true)".to_string(),
            ));
        }

        // Extract sender from handle
        let sender = message
            .handle
            .as_ref()
            .map(|h| h.address.clone())
            .unwrap_or_else(|| "unknown".to_string());

        // Extract chat GUID for replies
        let chat_guid = message
            .chats
            .as_ref()
            .and_then(|chats| chats.first())
            .map(|c| c.guid.clone());

        // Use message GUID as thread ID
        let thread_id = chat_guid.clone().unwrap_or_else(|| message.guid.clone());

        // Parse attachments
        let attachments = message
            .attachments
            .as_ref()
            .map(|atts| {
                atts.iter()
                    .map(|a| Attachment {
                        name: a.transfer_name.clone().unwrap_or_default(),
                        content_type: a.mime_type.clone().unwrap_or_default(),
                        content: String::new(), // BlueBubbles files need to be fetched separately
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(InboundMessage {
            channel: Channel::BlueBubbles,
            sender: sender.clone(),
            sender_name: message.handle.as_ref().and_then(|h| h.contact_name.clone()),
            recipient: "bluebubbles".to_string(),
            subject: None, // iMessage doesn't have subjects
            text_body: message.text.clone(),
            html_body: None,
            thread_id,
            message_id: Some(message.guid.clone()),
            attachments,
            reply_to: vec![sender],
            raw_payload: raw_payload.to_vec(),
            metadata: ChannelMetadata {
                bluebubbles_chat_guid: chat_guid,
                ..Default::default()
            },
        })
    }

    fn channel(&self) -> Channel {
        Channel::BlueBubbles
    }
}

/// Adapter for sending messages via BlueBubbles REST API.
#[derive(Debug, Clone)]
pub struct BlueBubblesOutboundAdapter {
    /// BlueBubbles server URL
    pub server_url: String,
    /// BlueBubbles server password
    pub password: String,
}

impl BlueBubblesOutboundAdapter {
    pub fn new(server_url: String, password: String) -> Self {
        Self {
            server_url,
            password,
        }
    }
}

impl OutboundAdapter for BlueBubblesOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let chat_guid = message
            .metadata
            .bluebubbles_chat_guid
            .as_ref()
            .or(message.to.first())
            .ok_or(AdapterError::ConfigError(
                "no chat GUID specified for BlueBubbles message".to_string(),
            ))?;

        let text = if message.text_body.is_empty() {
            message.html_body.clone()
        } else {
            message.text_body.clone()
        };

        let request = BlueBubblesSendRequest {
            chat_guid: chat_guid.clone(),
            message: text,
            method: Some("apple-script".to_string()),
            temp_guid: Some(uuid::Uuid::new_v4().to_string()),
        };

        // Send via BlueBubbles API
        let url = format!(
            "{}/api/v1/message/text?password={}",
            self.server_url.trim_end_matches('/'),
            self.password
        );
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        let api_response: BlueBubblesApiResponse = response
            .json()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if api_response.status == 200 {
            Ok(SendResult {
                success: true,
                message_id: api_response
                    .data
                    .as_ref()
                    .map(|d| d.guid.clone())
                    .unwrap_or_default(),
                submitted_at: chrono::Utc::now().to_rfc3339(),
                error: None,
            })
        } else {
            Ok(SendResult {
                success: false,
                message_id: String::new(),
                submitted_at: String::new(),
                error: Some(
                    api_response
                        .message
                        .unwrap_or_else(|| "Unknown error".to_string()),
                ),
            })
        }
    }

    fn channel(&self) -> Channel {
        Channel::BlueBubbles
    }
}

// ============================================================================
// BlueBubbles-specific types
// ============================================================================

/// Webhook payload from BlueBubbles server.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesWebhook {
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: BlueBubblesMessage,
}

/// Message data from BlueBubbles webhook.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesMessage {
    /// Unique message identifier
    pub guid: String,
    /// Message text content
    pub text: Option<String>,
    /// Whether this message was sent by us
    #[serde(rename = "isFromMe")]
    pub is_from_me: Option<bool>,
    /// Sender handle information
    pub handle: Option<BlueBubblesHandle>,
    /// Associated chats
    pub chats: Option<Vec<BlueBubblesChat>>,
    /// File attachments
    pub attachments: Option<Vec<BlueBubblesAttachment>>,
    /// Message timestamp
    #[serde(rename = "dateCreated")]
    pub date_created: Option<i64>,
}

/// Handle (contact) information.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesHandle {
    /// Phone number or email address
    pub address: String,
    /// Contact name if available
    #[serde(rename = "displayName")]
    pub contact_name: Option<String>,
}

/// Chat information.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesChat {
    /// Chat GUID (e.g., "iMessage;-;+1234567890")
    pub guid: String,
    /// Chat identifier (phone number or email)
    #[serde(rename = "chatIdentifier")]
    pub chat_identifier: Option<String>,
    /// Display name for group chats
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

/// Attachment information.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesAttachment {
    /// Attachment GUID
    pub guid: Option<String>,
    /// File name
    #[serde(rename = "transferName")]
    pub transfer_name: Option<String>,
    /// MIME type
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    /// File size in bytes
    #[serde(rename = "totalBytes")]
    pub total_bytes: Option<i64>,
}

/// Request body for sending a message via BlueBubbles.
#[derive(Debug, Clone, Serialize)]
pub struct BlueBubblesSendRequest {
    #[serde(rename = "chatGuid")]
    pub chat_guid: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Temporary GUID required when using AppleScript method
    #[serde(rename = "tempGuid", skip_serializing_if = "Option::is_none")]
    pub temp_guid: Option<String>,
}

/// Response from BlueBubbles API.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesApiResponse {
    pub status: i32,
    pub message: Option<String>,
    pub data: Option<BlueBubblesSendResponseData>,
}

/// Data from successful send response.
#[derive(Debug, Clone, Deserialize)]
pub struct BlueBubblesSendResponseData {
    pub guid: String,
}

// ============================================================================
// Helper functions
// ============================================================================

/// Send a quick response via BlueBubbles for locally-handled queries (async version).
pub async fn send_quick_bluebubbles_response(
    server_url: &str,
    password: &str,
    chat_guid: &str,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}/api/v1/message/text?password={}",
        server_url.trim_end_matches('/'),
        password
    );

    let request = BlueBubblesSendRequest {
        chat_guid: chat_guid.to_string(),
        message: response_text.to_string(),
        method: Some("apple-script".to_string()),
        temp_guid: Some(uuid::Uuid::new_v4().to_string()),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("BlueBubbles API returned {}: {}", status, body).into());
    }

    let api_response: BlueBubblesApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if api_response.status != 200 {
        return Err(format!(
            "BlueBubbles API error: {}",
            api_response
                .message
                .unwrap_or_else(|| "Unknown error".to_string())
        )
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_new_message_webhook() {
        let payload = r#"{
            "type": "new-message",
            "data": {
                "guid": "message-guid-123",
                "text": "Hello from iMessage!",
                "isFromMe": false,
                "handle": {
                    "address": "+1234567890",
                    "displayName": "John Doe"
                },
                "chats": [
                    {
                        "guid": "iMessage;-;+1234567890",
                        "chatIdentifier": "+1234567890"
                    }
                ]
            }
        }"#;

        let adapter = BlueBubblesInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.channel, Channel::BlueBubbles);
        assert_eq!(message.sender, "+1234567890");
        assert_eq!(message.sender_name, Some("John Doe".to_string()));
        assert_eq!(message.text_body, Some("Hello from iMessage!".to_string()));
        assert_eq!(
            message.metadata.bluebubbles_chat_guid,
            Some("iMessage;-;+1234567890".to_string())
        );
    }

    #[test]
    fn ignore_outgoing_messages() {
        let payload = r#"{
            "type": "new-message",
            "data": {
                "guid": "message-guid-456",
                "text": "My outgoing message",
                "isFromMe": true,
                "handle": {
                    "address": "+1234567890"
                },
                "chats": [
                    {
                        "guid": "iMessage;-;+1234567890"
                    }
                ]
            }
        }"#;

        let adapter = BlueBubblesInboundAdapter::new();
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn ignore_non_message_events() {
        let payload = r#"{
            "type": "typing-indicator",
            "data": {
                "guid": "chat-guid-123"
            }
        }"#;

        let adapter = BlueBubblesInboundAdapter::new();
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn parse_message_with_attachments() {
        let payload = r#"{
            "type": "new-message",
            "data": {
                "guid": "message-guid-789",
                "text": "Check out this photo",
                "isFromMe": false,
                "handle": {
                    "address": "+1234567890"
                },
                "chats": [
                    {
                        "guid": "iMessage;-;+1234567890"
                    }
                ],
                "attachments": [
                    {
                        "guid": "att-guid-001",
                        "transferName": "photo.jpg",
                        "mimeType": "image/jpeg",
                        "totalBytes": 12345
                    }
                ]
            }
        }"#;

        let adapter = BlueBubblesInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.attachments.len(), 1);
        assert_eq!(message.attachments[0].name, "photo.jpg");
        assert_eq!(message.attachments[0].content_type, "image/jpeg");
    }

    #[test]
    fn parse_group_chat_message() {
        let payload = r#"{
            "type": "new-message",
            "data": {
                "guid": "message-guid-group",
                "text": "Hello group!",
                "isFromMe": false,
                "handle": {
                    "address": "+1234567890",
                    "displayName": "Alice"
                },
                "chats": [
                    {
                        "guid": "iMessage;+;chat123456",
                        "displayName": "Family Group"
                    }
                ]
            }
        }"#;

        let adapter = BlueBubblesInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.sender, "+1234567890");
        assert_eq!(
            message.metadata.bluebubbles_chat_guid,
            Some("iMessage;+;chat123456".to_string())
        );
    }
}
