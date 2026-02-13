//! Slack adapter for inbound and outbound messages.
//!
//! This module provides adapters for handling messages via Slack:
//! - `SlackInboundAdapter`: Parses Slack event payloads
//! - `SlackOutboundAdapter`: Sends messages via Slack Web API

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;

use crate::channel::{
    AdapterError, Attachment, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
    OutboundAdapter, OutboundMessage, SendResult,
};

/// Adapter for parsing Slack event webhook payloads.
#[derive(Debug, Clone, Default)]
pub struct SlackInboundAdapter {
    /// Bot user IDs that this adapter handles (messages from these are ignored)
    pub bot_user_ids: HashSet<String>,
}

impl SlackInboundAdapter {
    pub fn new(bot_user_ids: HashSet<String>) -> Self {
        Self { bot_user_ids }
    }

    /// Check if the sender is a bot that should be ignored.
    pub fn is_bot_message(&self, event: &SlackMessageEvent) -> bool {
        // Ignore bot messages
        if event.bot_id.is_some() {
            return true;
        }
        // Ignore messages from our own bot user
        if let Some(ref user) = event.user {
            if self.bot_user_ids.contains(user) {
                return true;
            }
        }
        false
    }

    /// Extract thread ID from a Slack message event.
    pub fn extract_thread_id(&self, event: &SlackMessageEvent) -> String {
        // Use thread_ts if present, otherwise use ts (message timestamp) as thread ID
        event
            .thread_ts
            .clone()
            .unwrap_or_else(|| event.ts.clone())
    }
}

impl InboundAdapter for SlackInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        let wrapper: SlackEventWrapper =
            serde_json::from_slice(raw_payload).map_err(|e| AdapterError::ParseError(e.to_string()))?;

        // Handle URL verification challenge
        if wrapper.event_type == "url_verification" {
            return Err(AdapterError::ParseError(
                "url_verification should be handled separately".to_string(),
            ));
        }

        // Only handle event_callback with message events
        if wrapper.event_type != "event_callback" {
            return Err(AdapterError::ParseError(format!(
                "unsupported event type: {}",
                wrapper.event_type
            )));
        }

        let event = wrapper
            .event
            .ok_or(AdapterError::MissingField("event"))?;

        // Only handle message events (not subtypes like message_changed, etc.)
        if event.event_type != "message" {
            return Err(AdapterError::ParseError(format!(
                "unsupported event.type: {}",
                event.event_type
            )));
        }

        // Ignore message subtypes (edits, deletes, etc.) - only process new messages
        if event.subtype.is_some() {
            return Err(AdapterError::ParseError(
                "ignoring message with subtype".to_string(),
            ));
        }

        let sender = event
            .user
            .clone()
            .ok_or(AdapterError::MissingField("user"))?;

        let channel_id = event
            .channel
            .clone()
            .ok_or(AdapterError::MissingField("channel"))?;

        let thread_id = self.extract_thread_id(&event);

        // Parse files as attachments
        let attachments = event
            .files
            .as_ref()
            .map(|files| {
                files
                    .iter()
                    .map(|f| Attachment {
                        name: f.name.clone().unwrap_or_default(),
                        content_type: f.mimetype.clone().unwrap_or_default(),
                        content: String::new(), // Slack files need to be fetched separately
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(InboundMessage {
            channel: Channel::Slack,
            sender,
            sender_name: None, // Would need users.info API call to get display name
            recipient: channel_id.clone(),
            subject: None, // Slack doesn't have subjects
            text_body: event.text.clone(),
            html_body: None,
            thread_id,
            message_id: Some(event.ts.clone()),
            attachments,
            reply_to: vec![channel_id.clone()],
            raw_payload: raw_payload.to_vec(),
            metadata: ChannelMetadata {
                slack_channel_id: Some(channel_id),
                slack_team_id: wrapper.team_id,
                ..Default::default()
            },
        })
    }

    fn channel(&self) -> Channel {
        Channel::Slack
    }
}

/// Adapter for sending messages via Slack Web API.
#[derive(Debug, Clone)]
pub struct SlackOutboundAdapter {
    /// Slack Bot OAuth token
    pub bot_token: String,
}

impl SlackOutboundAdapter {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }
}

impl OutboundAdapter for SlackOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let channel = message
            .metadata
            .slack_channel_id
            .as_ref()
            .or(message.to.first())
            .ok_or(AdapterError::ConfigError(
                "no channel specified for Slack message".to_string(),
            ))?;

        let request = SlackPostMessageRequest {
            channel: channel.clone(),
            text: if message.text_body.is_empty() {
                message.html_body.clone() // Fallback to html_body if text is empty
            } else {
                message.text_body.clone()
            },
            thread_ts: message.thread_id.clone(),
            mrkdwn: Some(true),
        };

        // Send via Slack API
        let api_base = env::var("SLACK_API_BASE_URL")
            .unwrap_or_else(|_| "https://slack.com/api".to_string());
        let url = format!("{}/chat.postMessage", api_base.trim_end_matches('/'));
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        let api_response: SlackApiResponse = response
            .json()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if api_response.ok {
            Ok(SendResult {
                success: true,
                message_id: api_response.ts.unwrap_or_default(),
                submitted_at: chrono::Utc::now().to_rfc3339(),
                error: None,
            })
        } else {
            Ok(SendResult {
                success: false,
                message_id: String::new(),
                submitted_at: String::new(),
                error: api_response.error,
            })
        }
    }

    fn channel(&self) -> Channel {
        Channel::Slack
    }
}

// ============================================================================
// Slack-specific types
// ============================================================================

/// Wrapper for all Slack event payloads.
#[derive(Debug, Clone, Deserialize)]
pub struct SlackEventWrapper {
    #[serde(rename = "type")]
    pub event_type: String,
    /// Challenge string for URL verification
    pub challenge: Option<String>,
    /// Token for verification (deprecated, use signing secret instead)
    pub token: Option<String>,
    /// Team ID
    pub team_id: Option<String>,
    /// API App ID
    pub api_app_id: Option<String>,
    /// The actual event payload (for event_callback type)
    pub event: Option<SlackMessageEvent>,
    /// Event ID for deduplication
    pub event_id: Option<String>,
    /// Event time
    pub event_time: Option<i64>,
}

/// URL verification challenge request from Slack.
#[derive(Debug, Clone, Deserialize)]
pub struct SlackUrlVerification {
    #[serde(rename = "type")]
    pub event_type: String,
    pub challenge: String,
    pub token: String,
}

/// URL verification challenge response.
#[derive(Debug, Clone, Serialize)]
pub struct SlackChallengeResponse {
    pub challenge: String,
}

/// Slack message event payload.
#[derive(Debug, Clone, Deserialize)]
pub struct SlackMessageEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    /// Message subtype (e.g., "message_changed", "message_deleted")
    pub subtype: Option<String>,
    /// Channel ID where the message was sent
    pub channel: Option<String>,
    /// User ID who sent the message
    pub user: Option<String>,
    /// Message text
    pub text: Option<String>,
    /// Message timestamp (also serves as message ID)
    pub ts: String,
    /// Thread timestamp (if message is in a thread)
    pub thread_ts: Option<String>,
    /// Bot ID if message is from a bot
    pub bot_id: Option<String>,
    /// App ID if message is from an app
    pub app_id: Option<String>,
    /// File attachments
    pub files: Option<Vec<SlackFile>>,
    /// Channel type (channel, group, im, mpim)
    pub channel_type: Option<String>,
    /// Event timestamp
    pub event_ts: Option<String>,
}

/// Slack file attachment.
#[derive(Debug, Clone, Deserialize)]
pub struct SlackFile {
    pub id: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub mimetype: Option<String>,
    pub filetype: Option<String>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub size: Option<i64>,
}

/// Request body for chat.postMessage API.
#[derive(Debug, Clone, Serialize)]
pub struct SlackPostMessageRequest {
    pub channel: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrkdwn: Option<bool>,
}

/// Response from Slack API.
#[derive(Debug, Clone, Deserialize)]
pub struct SlackApiResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub ts: Option<String>,
    pub channel: Option<String>,
    pub message: Option<serde_json::Value>,
}

// ============================================================================
// Helper functions
// ============================================================================

/// Check if a payload is a URL verification challenge.
pub fn is_url_verification(payload: &[u8]) -> Option<SlackUrlVerification> {
    let wrapper: SlackEventWrapper = serde_json::from_slice(payload).ok()?;
    if wrapper.event_type == "url_verification" {
        Some(SlackUrlVerification {
            event_type: wrapper.event_type,
            challenge: wrapper.challenge?,
            token: wrapper.token.unwrap_or_default(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_verification_challenge() {
        let payload = r#"{
            "type": "url_verification",
            "challenge": "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P",
            "token": "Jhj5dZrVaK7ZwHHjRyZWjbDl"
        }"#;

        let verification = is_url_verification(payload.as_bytes());
        assert!(verification.is_some());
        let v = verification.unwrap();
        assert_eq!(v.challenge, "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P");
    }

    #[test]
    fn parse_message_event() {
        let payload = r#"{
            "type": "event_callback",
            "team_id": "T123ABC456",
            "api_app_id": "A123ABC456",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "Hello, world!",
                "ts": "1355517536.000001"
            },
            "event_id": "Ev123ABC456",
            "event_time": 1234567890
        }"#;

        let adapter = SlackInboundAdapter::default();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.channel, Channel::Slack);
        assert_eq!(message.sender, "U123ABC456");
        assert_eq!(message.text_body, Some("Hello, world!".to_string()));
        assert_eq!(message.metadata.slack_channel_id, Some("C123ABC456".to_string()));
        assert_eq!(message.metadata.slack_team_id, Some("T123ABC456".to_string()));
    }

    #[test]
    fn parse_threaded_message() {
        let payload = r#"{
            "type": "event_callback",
            "team_id": "T123ABC456",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "Reply in thread",
                "ts": "1355517536.000002",
                "thread_ts": "1355517536.000001"
            },
            "event_id": "Ev123ABC456"
        }"#;

        let adapter = SlackInboundAdapter::default();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.thread_id, "1355517536.000001");
    }

    #[test]
    fn ignore_bot_messages() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "Bot message",
                "ts": "1355517536.000001",
                "bot_id": "B123ABC456"
            }
        }"#;

        let wrapper: SlackEventWrapper = serde_json::from_slice(payload.as_bytes()).unwrap();
        let event = wrapper.event.unwrap();
        let adapter = SlackInboundAdapter::default();
        assert!(adapter.is_bot_message(&event));
    }

    #[test]
    fn ignore_message_subtypes() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "subtype": "message_changed",
                "channel": "C123ABC456",
                "ts": "1355517536.000001"
            }
        }"#;

        let adapter = SlackInboundAdapter::default();
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn parse_message_with_files() {
        let payload = r#"{
            "type": "event_callback",
            "team_id": "T123ABC456",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "Here's a file",
                "ts": "1355517536.000001",
                "files": [
                    {
                        "id": "F123ABC456",
                        "name": "report.pdf",
                        "mimetype": "application/pdf",
                        "size": 12345
                    }
                ]
            },
            "event_id": "Ev123ABC456"
        }"#;

        let adapter = SlackInboundAdapter::default();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.attachments.len(), 1);
        assert_eq!(message.attachments[0].name, "report.pdf");
        assert_eq!(message.attachments[0].content_type, "application/pdf");
    }

    #[test]
    fn missing_user_field_errors() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "text": "No user field",
                "ts": "1355517536.000001"
            }
        }"#;

        let adapter = SlackInboundAdapter::default();
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn missing_channel_field_errors() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "user": "U123ABC456",
                "text": "No channel field",
                "ts": "1355517536.000001"
            }
        }"#;

        let adapter = SlackInboundAdapter::default();
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn own_bot_user_id_filtered() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "UBOT123",
                "text": "Message from our bot",
                "ts": "1355517536.000001"
            }
        }"#;

        let mut bot_ids = HashSet::new();
        bot_ids.insert("UBOT123".to_string());
        let adapter = SlackInboundAdapter::new(bot_ids);

        let wrapper: SlackEventWrapper = serde_json::from_slice(payload.as_bytes()).unwrap();
        let event = wrapper.event.unwrap();
        assert!(adapter.is_bot_message(&event));
    }

    #[test]
    fn non_bot_user_not_filtered() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "Normal user message",
                "ts": "1355517536.000001"
            }
        }"#;

        let mut bot_ids = HashSet::new();
        bot_ids.insert("UBOT123".to_string());
        let adapter = SlackInboundAdapter::new(bot_ids);

        let wrapper: SlackEventWrapper = serde_json::from_slice(payload.as_bytes()).unwrap();
        let event = wrapper.event.unwrap();
        assert!(!adapter.is_bot_message(&event));
    }

    #[test]
    fn thread_id_uses_ts_when_no_thread_ts() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "New thread starter",
                "ts": "1355517536.000001"
            }
        }"#;

        let adapter = SlackInboundAdapter::default();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        // When no thread_ts, use ts as thread_id
        assert_eq!(message.thread_id, "1355517536.000001");
    }

    #[test]
    fn unsupported_event_type_errors() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "app_mention",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "Hey bot!",
                "ts": "1355517536.000001"
            }
        }"#;

        let adapter = SlackInboundAdapter::default();
        let result = adapter.parse(payload.as_bytes());
        // app_mention is not "message" type, should error
        assert!(result.is_err());
    }

    #[test]
    fn empty_text_message_parses() {
        let payload = r#"{
            "type": "event_callback",
            "event": {
                "type": "message",
                "channel": "C123ABC456",
                "user": "U123ABC456",
                "text": "",
                "ts": "1355517536.000001"
            }
        }"#;

        let adapter = SlackInboundAdapter::default();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.text_body, Some("".to_string()));
    }
}
