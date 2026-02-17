//! Discord adapter for inbound and outbound messages.
//!
//! This module provides adapters for handling messages via Discord:
//! - `DiscordInboundAdapter`: Converts serenity Message events to InboundMessage
//! - `DiscordOutboundAdapter`: Sends messages via Discord REST API

use std::collections::HashSet;
use std::env;

use crate::channel::{
    AdapterError, Attachment, Channel, ChannelMetadata, InboundMessage, OutboundAdapter,
    OutboundMessage, SendResult,
};

/// Adapter for converting Discord Gateway events to normalized messages.
///
/// Unlike Slack which uses HTTP webhooks, Discord messages arrive via WebSocket
/// through serenity's EventHandler. This adapter provides methods to convert
/// serenity types to our normalized InboundMessage format.
#[derive(Debug, Clone, Default)]
pub struct DiscordInboundAdapter {
    /// Bot user IDs that this adapter handles (messages from these are ignored)
    pub bot_user_ids: HashSet<u64>,
}

impl DiscordInboundAdapter {
    pub fn new(bot_user_ids: HashSet<u64>) -> Self {
        Self { bot_user_ids }
    }

    /// Check if the sender is a bot that should be ignored.
    pub fn is_bot_message(&self, author_id: u64, is_bot: bool) -> bool {
        // Ignore bot messages
        if is_bot {
            return true;
        }
        // Ignore messages from our own bot user
        if self.bot_user_ids.contains(&author_id) {
            return true;
        }
        false
    }

    /// Convert a serenity Message to a normalized InboundMessage.
    ///
    /// This is called from the serenity EventHandler when MESSAGE_CREATE events
    /// are received via the Gateway WebSocket.
    pub fn from_serenity_message(
        &self,
        message: &serenity::model::channel::Message,
    ) -> Result<InboundMessage, AdapterError> {
        // Ignore bot messages
        if self.is_bot_message(message.author.id.get(), message.author.bot) {
            return Err(AdapterError::ParseError("ignoring bot message".to_string()));
        }

        let guild_id = message.guild_id.map(|id| id.get());
        let channel_id = message.channel_id.get();
        let author_id = message.author.id.get();

        // Thread ID: use referenced message ID if replying, otherwise message ID
        // Discord doesn't have explicit threads like Slack's thread_ts
        let thread_id = message
            .referenced_message
            .as_ref()
            .map(|m| m.id.get().to_string())
            .unwrap_or_else(|| message.id.get().to_string());

        // Parse attachments
        let attachments = message
            .attachments
            .iter()
            .map(|a| Attachment {
                name: a.filename.clone(),
                content_type: a.content_type.clone().unwrap_or_default(),
                content: String::new(), // Discord attachments need to be fetched via URL
            })
            .collect();

        // Serialize original message for archival
        let raw_payload = serde_json::to_vec(&DiscordMessagePayload {
            id: message.id.get(),
            channel_id,
            guild_id,
            author_id,
            author_name: message.author.name.clone(),
            content: message.content.clone(),
            timestamp: message.timestamp.to_string(),
        })
        .unwrap_or_default();

        Ok(InboundMessage {
            channel: Channel::Discord,
            sender: author_id.to_string(),
            sender_name: Some(message.author.name.clone()),
            recipient: channel_id.to_string(),
            subject: None, // Discord doesn't have subjects
            text_body: Some(message.content.clone()),
            html_body: None,
            thread_id,
            message_id: Some(message.id.get().to_string()),
            attachments,
            reply_to: vec![channel_id.to_string()],
            raw_payload,
            metadata: ChannelMetadata {
                discord_guild_id: guild_id,
                discord_channel_id: Some(channel_id),
                ..Default::default()
            },
        })
    }
}

/// Adapter for sending messages via Discord REST API.
#[derive(Debug, Clone)]
pub struct DiscordOutboundAdapter {
    /// Discord Bot token
    pub bot_token: String,
}

impl DiscordOutboundAdapter {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }
}

impl OutboundAdapter for DiscordOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let channel_id = message
            .metadata
            .discord_channel_id
            .or_else(|| message.to.first().and_then(|s| s.parse().ok()))
            .ok_or(AdapterError::ConfigError(
                "no channel specified for Discord message".to_string(),
            ))?;

        let request = DiscordCreateMessageRequest {
            content: if message.text_body.is_empty() {
                message.html_body.clone() // Fallback to html_body if text is empty
            } else {
                message.text_body.clone()
            },
            message_reference: message.thread_id.as_ref().and_then(|tid| {
                tid.parse::<u64>()
                    .ok()
                    .map(|id| DiscordMessageReference { message_id: id })
            }),
        };

        // Send via Discord REST API
        let api_base = env::var("DISCORD_API_BASE_URL")
            .unwrap_or_else(|_| "https://discord.com/api/v10".to_string());
        let url = format!(
            "{}/channels/{}/messages",
            api_base.trim_end_matches('/'),
            channel_id
        );

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(url)
            .header("Authorization", format!("Bot {}", self.bot_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if response.status().is_success() {
            let api_response: DiscordMessageResponse = response
                .json()
                .map_err(|e| AdapterError::SendError(e.to_string()))?;

            Ok(SendResult {
                success: true,
                message_id: api_response.id,
                submitted_at: api_response.timestamp,
                error: None,
            })
        } else {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "unknown error".to_string());
            Ok(SendResult {
                success: false,
                message_id: String::new(),
                submitted_at: String::new(),
                error: Some(error_text),
            })
        }
    }

    fn channel(&self) -> Channel {
        Channel::Discord
    }
}

// ============================================================================
// Discord-specific types
// ============================================================================

/// Simplified Discord message payload for archival.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscordMessagePayload {
    pub id: u64,
    pub channel_id: u64,
    pub guild_id: Option<u64>,
    pub author_id: u64,
    pub author_name: String,
    pub content: String,
    pub timestamp: String,
}

/// Request body for creating a Discord message.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscordCreateMessageRequest {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<DiscordMessageReference>,
}

/// Message reference for replies.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscordMessageReference {
    pub message_id: u64,
}

/// Response from Discord when creating a message.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DiscordMessageResponse {
    pub id: String,
    pub timestamp: String,
    pub channel_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bot_user_id_filtered() {
        let mut bot_ids = HashSet::new();
        bot_ids.insert(123456789u64);
        let adapter = DiscordInboundAdapter::new(bot_ids);

        assert!(adapter.is_bot_message(123456789, false));
        assert!(!adapter.is_bot_message(987654321, false));
    }

    #[test]
    fn bot_flag_filtered() {
        let adapter = DiscordInboundAdapter::default();

        assert!(adapter.is_bot_message(123456789, true));
        assert!(!adapter.is_bot_message(123456789, false));
    }

    #[test]
    fn create_message_request_serializes() {
        let request = DiscordCreateMessageRequest {
            content: "Hello, Discord!".to_string(),
            message_reference: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Hello, Discord!"));
        assert!(!json.contains("message_reference"));
    }

    #[test]
    fn create_message_request_with_reference() {
        let request = DiscordCreateMessageRequest {
            content: "This is a reply".to_string(),
            message_reference: Some(DiscordMessageReference {
                message_id: 123456789012345678,
            }),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("message_reference"));
        assert!(json.contains("123456789012345678"));
    }

    #[test]
    fn message_response_deserializes() {
        let json = r#"{
            "id": "123456789012345678",
            "timestamp": "2024-01-15T12:00:00.000Z",
            "channel_id": "987654321098765432"
        }"#;

        let response: DiscordMessageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "123456789012345678");
        assert_eq!(response.channel_id, "987654321098765432");
    }
}
