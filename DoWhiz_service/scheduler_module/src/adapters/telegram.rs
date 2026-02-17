//! Telegram adapter for inbound and outbound messages via Bot API.
//!
//! This module provides adapters for handling Telegram messages:
//! - `TelegramInboundAdapter`: Parses Telegram webhook payloads
//! - `TelegramOutboundAdapter`: Sends messages via Telegram Bot API

use serde::{Deserialize, Serialize};

use crate::channel::{
    AdapterError, Attachment, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
    OutboundAdapter, OutboundMessage, SendResult,
};

/// Adapter for parsing Telegram webhook payloads.
#[derive(Debug, Clone, Default)]
pub struct TelegramInboundAdapter {
    /// Bot's own user ID to filter out messages from self
    pub bot_user_id: Option<i64>,
}

impl TelegramInboundAdapter {
    pub fn new() -> Self {
        Self { bot_user_id: None }
    }

    pub fn with_bot_id(bot_user_id: i64) -> Self {
        Self {
            bot_user_id: Some(bot_user_id),
        }
    }

    /// Check if the message is from the bot itself.
    pub fn is_from_bot(&self, message: &TelegramMessage) -> bool {
        if let Some(bot_id) = self.bot_user_id {
            if let Some(ref from) = message.from {
                return from.id == bot_id;
            }
        }
        false
    }
}

impl InboundAdapter for TelegramInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        let update: TelegramUpdate = serde_json::from_slice(raw_payload)
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        // Handle regular messages
        let message = update
            .message
            .or(update.edited_message)
            .ok_or_else(|| AdapterError::ParseError("no message in update".to_string()))?;

        // Ignore bot's own messages
        if self.is_from_bot(&message) {
            return Err(AdapterError::ParseError(
                "ignoring message from bot itself".to_string(),
            ));
        }

        // Extract sender info
        let from = message
            .from
            .as_ref()
            .ok_or_else(|| AdapterError::ParseError("no sender in message".to_string()))?;

        let sender = from.id.to_string();
        let sender_name = from
            .first_name
            .clone()
            .map(|first| {
                if let Some(ref last) = from.last_name {
                    format!("{} {}", first, last)
                } else {
                    first
                }
            })
            .or_else(|| from.username.clone());

        // Extract text content
        let text_body = message.text.clone().or_else(|| message.caption.clone());

        // Use chat ID as thread identifier
        let chat_id = message.chat.id;
        let thread_id = chat_id.to_string();

        // Parse photo attachments (simplified - just metadata)
        let attachments: Vec<Attachment> = message
            .photo
            .as_ref()
            .map(|photos| {
                // Get the largest photo (last in array)
                photos
                    .last()
                    .map(|p| Attachment {
                        name: format!("photo_{}.jpg", p.file_id),
                        content_type: "image/jpeg".to_string(),
                        content: p.file_id.clone(), // Store file_id for later retrieval
                    })
                    .into_iter()
                    .collect()
            })
            .unwrap_or_default();

        // Add document attachments
        let mut all_attachments = attachments;
        if let Some(ref doc) = message.document {
            all_attachments.push(Attachment {
                name: doc.file_name.clone().unwrap_or_else(|| "document".to_string()),
                content_type: doc.mime_type.clone().unwrap_or_else(|| "application/octet-stream".to_string()),
                content: doc.file_id.clone(),
            });
        }

        Ok(InboundMessage {
            channel: Channel::Telegram,
            sender: sender.clone(),
            sender_name,
            recipient: "telegram_bot".to_string(),
            subject: None, // Telegram doesn't have subjects
            text_body,
            html_body: None,
            thread_id,
            message_id: Some(message.message_id.to_string()),
            attachments: all_attachments,
            reply_to: vec![sender],
            raw_payload: raw_payload.to_vec(),
            metadata: ChannelMetadata {
                telegram_chat_id: Some(chat_id),
                ..Default::default()
            },
        })
    }

    fn channel(&self) -> Channel {
        Channel::Telegram
    }
}

/// Adapter for sending messages via Telegram Bot API.
#[derive(Debug, Clone)]
pub struct TelegramOutboundAdapter {
    /// Telegram Bot API token
    pub bot_token: String,
}

impl TelegramOutboundAdapter {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }

    fn api_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.bot_token, method)
    }
}

impl OutboundAdapter for TelegramOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let chat_id = message
            .metadata
            .telegram_chat_id
            .or_else(|| message.to.first().and_then(|s| s.parse::<i64>().ok()))
            .ok_or(AdapterError::ConfigError(
                "no chat_id specified for Telegram message".to_string(),
            ))?;

        let text = if message.text_body.is_empty() {
            message.html_body.clone()
        } else {
            message.text_body.clone()
        };

        let request = TelegramSendMessageRequest {
            chat_id,
            text,
            parse_mode: Some("HTML".to_string()),
            reply_to_message_id: message
                .thread_id
                .as_ref()
                .and_then(|s| s.parse::<i64>().ok()),
        };

        // Send via Telegram Bot API
        let url = self.api_url("sendMessage");
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .json(&request)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        let api_response: TelegramApiResponse = response
            .json()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if api_response.ok {
            Ok(SendResult {
                success: true,
                message_id: api_response
                    .result
                    .as_ref()
                    .map(|r| r.message_id.to_string())
                    .unwrap_or_default(),
                submitted_at: chrono::Utc::now().to_rfc3339(),
                error: None,
            })
        } else {
            Ok(SendResult {
                success: false,
                message_id: String::new(),
                submitted_at: String::new(),
                error: Some(api_response.description.unwrap_or_else(|| "Unknown error".to_string())),
            })
        }
    }

    fn channel(&self) -> Channel {
        Channel::Telegram
    }
}

// ============================================================================
// Telegram-specific types
// ============================================================================

/// Webhook update from Telegram.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramUpdate {
    /// Unique update identifier
    pub update_id: i64,
    /// New incoming message
    pub message: Option<TelegramMessage>,
    /// Edited message
    pub edited_message: Option<TelegramMessage>,
}

/// Message from Telegram.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramMessage {
    /// Unique message identifier
    pub message_id: i64,
    /// Sender of the message
    pub from: Option<TelegramUser>,
    /// Chat the message belongs to
    pub chat: TelegramChat,
    /// Date the message was sent (Unix timestamp)
    pub date: i64,
    /// Text content of the message
    pub text: Option<String>,
    /// Caption for media messages
    pub caption: Option<String>,
    /// Photos attached to the message
    pub photo: Option<Vec<TelegramPhotoSize>>,
    /// Document attached to the message
    pub document: Option<TelegramDocument>,
    /// Original message for replies
    pub reply_to_message: Option<Box<TelegramMessage>>,
}

/// Telegram user.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramUser {
    /// Unique identifier
    pub id: i64,
    /// Whether the user is a bot
    pub is_bot: bool,
    /// User's first name
    pub first_name: Option<String>,
    /// User's last name
    pub last_name: Option<String>,
    /// User's username
    pub username: Option<String>,
}

/// Telegram chat.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramChat {
    /// Unique identifier
    pub id: i64,
    /// Type of chat: "private", "group", "supergroup", or "channel"
    #[serde(rename = "type")]
    pub chat_type: String,
    /// Title for groups/channels
    pub title: Option<String>,
    /// Username for private chats
    pub username: Option<String>,
    /// First name for private chats
    pub first_name: Option<String>,
    /// Last name for private chats
    pub last_name: Option<String>,
}

/// Photo size from Telegram.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramPhotoSize {
    /// Identifier for this file
    pub file_id: String,
    /// Unique identifier for this file
    pub file_unique_id: String,
    /// Photo width
    pub width: i32,
    /// Photo height
    pub height: i32,
    /// File size
    pub file_size: Option<i64>,
}

/// Document from Telegram.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramDocument {
    /// Identifier for this file
    pub file_id: String,
    /// Unique identifier for this file
    pub file_unique_id: String,
    /// Original filename
    pub file_name: Option<String>,
    /// MIME type
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<i64>,
}

/// Request body for sendMessage API.
#[derive(Debug, Clone, Serialize)]
pub struct TelegramSendMessageRequest {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
}

/// Response from Telegram Bot API.
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramApiResponse {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<TelegramMessage>,
}

// ============================================================================
// Helper functions
// ============================================================================

/// Send a quick response via Telegram for locally-handled queries (async version).
pub async fn send_quick_telegram_response(
    bot_token: &str,
    chat_id: i64,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let request = TelegramSendMessageRequest {
        chat_id,
        text: response_text.to_string(),
        parse_mode: None, // Use plain text for quick responses
        reply_to_message_id: None,
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
        return Err(format!("Telegram API returned {}: {}", status, body).into());
    }

    let api_response: TelegramApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if !api_response.ok {
        return Err(format!(
            "Telegram API error: {}",
            api_response.description.unwrap_or_else(|| "Unknown error".to_string())
        )
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_message() {
        let payload = r#"{
            "update_id": 123456789,
            "message": {
                "message_id": 1,
                "from": {
                    "id": 12345,
                    "is_bot": false,
                    "first_name": "Dylan",
                    "last_name": "Tang",
                    "username": "dylantang"
                },
                "chat": {
                    "id": 12345,
                    "type": "private",
                    "first_name": "Dylan"
                },
                "date": 1234567890,
                "text": "Hello from Telegram!"
            }
        }"#;

        let adapter = TelegramInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.channel, Channel::Telegram);
        assert_eq!(message.sender, "12345");
        assert_eq!(message.sender_name, Some("Dylan Tang".to_string()));
        assert_eq!(message.text_body, Some("Hello from Telegram!".to_string()));
        assert_eq!(message.metadata.telegram_chat_id, Some(12345));
    }

    #[test]
    fn parse_group_message() {
        let payload = r#"{
            "update_id": 123456790,
            "message": {
                "message_id": 2,
                "from": {
                    "id": 12345,
                    "is_bot": false,
                    "first_name": "Dylan"
                },
                "chat": {
                    "id": -100123456789,
                    "type": "supergroup",
                    "title": "Test Group"
                },
                "date": 1234567891,
                "text": "Hello group!"
            }
        }"#;

        let adapter = TelegramInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.metadata.telegram_chat_id, Some(-100123456789));
        assert_eq!(message.thread_id, "-100123456789");
    }

    #[test]
    fn ignore_bot_messages() {
        let payload = r#"{
            "update_id": 123456791,
            "message": {
                "message_id": 3,
                "from": {
                    "id": 99999,
                    "is_bot": true,
                    "first_name": "MyBot"
                },
                "chat": {
                    "id": 12345,
                    "type": "private"
                },
                "date": 1234567892,
                "text": "Bot message"
            }
        }"#;

        let adapter = TelegramInboundAdapter::with_bot_id(99999);
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn parse_message_with_photo() {
        let payload = r#"{
            "update_id": 123456792,
            "message": {
                "message_id": 4,
                "from": {
                    "id": 12345,
                    "is_bot": false,
                    "first_name": "Dylan"
                },
                "chat": {
                    "id": 12345,
                    "type": "private"
                },
                "date": 1234567893,
                "caption": "Check out this photo!",
                "photo": [
                    {
                        "file_id": "small_photo_id",
                        "file_unique_id": "small_unique",
                        "width": 90,
                        "height": 90
                    },
                    {
                        "file_id": "large_photo_id",
                        "file_unique_id": "large_unique",
                        "width": 800,
                        "height": 600
                    }
                ]
            }
        }"#;

        let adapter = TelegramInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.text_body, Some("Check out this photo!".to_string()));
        assert_eq!(message.attachments.len(), 1);
        assert!(message.attachments[0].name.contains("large_photo_id"));
    }

    #[test]
    fn parse_edited_message() {
        let payload = r#"{
            "update_id": 123456793,
            "edited_message": {
                "message_id": 5,
                "from": {
                    "id": 12345,
                    "is_bot": false,
                    "first_name": "Dylan"
                },
                "chat": {
                    "id": 12345,
                    "type": "private"
                },
                "date": 1234567894,
                "text": "Edited message"
            }
        }"#;

        let adapter = TelegramInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.text_body, Some("Edited message".to_string()));
    }
}
