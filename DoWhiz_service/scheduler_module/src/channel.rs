//! Channel abstraction for multi-platform messaging support.
//!
//! This module provides a unified interface for handling inbound and outbound
//! messages across different messaging platforms (email via Postmark, Slack, etc.).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Supported messaging channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    /// Email via Postmark
    Email,
    /// Slack (future)
    Slack,
    /// Telegram (future)
    Telegram,
}

impl Default for Channel {
    fn default() -> Self {
        Channel::Email
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Email => write!(f, "email"),
            Channel::Slack => write!(f, "slack"),
            Channel::Telegram => write!(f, "telegram"),
        }
    }
}

impl std::str::FromStr for Channel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "email" => Ok(Channel::Email),
            "slack" => Ok(Channel::Slack),
            "telegram" => Ok(Channel::Telegram),
            _ => Err(format!("unknown channel: {}", s)),
        }
    }
}

/// Normalized inbound message from any channel.
///
/// This struct provides a common representation for messages received from
/// any supported platform, abstracting away platform-specific details.
#[derive(Debug, Clone)]
pub struct InboundMessage {
    /// The channel this message came from
    pub channel: Channel,
    /// Sender identifier (email address, Slack user ID, etc.)
    pub sender: String,
    /// Sender display name (optional)
    pub sender_name: Option<String>,
    /// Recipient identifier (service address, bot ID, etc.)
    pub recipient: String,
    /// Message subject (email) or empty for platforms without subjects
    pub subject: Option<String>,
    /// Plain text body
    pub text_body: Option<String>,
    /// HTML body (email) or formatted text
    pub html_body: Option<String>,
    /// Thread identifier for grouping related messages
    pub thread_id: String,
    /// Unique message identifier from the source platform
    pub message_id: Option<String>,
    /// Attachments
    pub attachments: Vec<Attachment>,
    /// Reply-to address/ID (who to reply to)
    pub reply_to: Vec<String>,
    /// Raw payload bytes for archival
    pub raw_payload: Vec<u8>,
    /// Platform-specific metadata
    pub metadata: ChannelMetadata,
}

/// Attachment from any channel.
#[derive(Debug, Clone)]
pub struct Attachment {
    /// Filename
    pub name: String,
    /// MIME content type
    pub content_type: String,
    /// Base64-encoded content
    pub content: String,
}

/// Platform-specific metadata that doesn't fit in the common fields.
#[derive(Debug, Clone, Default)]
pub struct ChannelMetadata {
    /// Email-specific: In-Reply-To header
    pub in_reply_to: Option<String>,
    /// Email-specific: References header
    pub references: Option<String>,
    /// Slack-specific: Channel ID
    pub slack_channel_id: Option<String>,
    /// Slack-specific: Team ID
    pub slack_team_id: Option<String>,
    /// Telegram-specific: Chat ID
    pub telegram_chat_id: Option<i64>,
}

/// Normalized outbound message to any channel.
///
/// This struct provides a common representation for messages to be sent to
/// any supported platform.
#[derive(Debug, Clone)]
pub struct OutboundMessage {
    /// The channel to send this message to
    pub channel: Channel,
    /// Sender identifier (from address, bot name, etc.)
    pub from: Option<String>,
    /// Primary recipients
    pub to: Vec<String>,
    /// CC recipients (email only)
    pub cc: Vec<String>,
    /// BCC recipients (email only)
    pub bcc: Vec<String>,
    /// Message subject (email) or empty for platforms without subjects
    pub subject: String,
    /// Plain text body
    pub text_body: String,
    /// HTML body (email) or formatted text
    pub html_body: String,
    /// Path to HTML file (for file-based content)
    pub html_path: Option<PathBuf>,
    /// Directory containing attachments
    pub attachments_dir: Option<PathBuf>,
    /// Thread identifier for threading replies
    pub thread_id: Option<String>,
    /// Platform-specific metadata
    pub metadata: ChannelMetadata,
}

/// Result of sending an outbound message.
#[derive(Debug, Clone)]
pub struct SendResult {
    /// Whether the send was successful
    pub success: bool,
    /// Message ID assigned by the platform
    pub message_id: String,
    /// Timestamp when the message was submitted
    pub submitted_at: String,
    /// Error message if failed
    pub error: Option<String>,
}

/// Trait for parsing platform-specific inbound payloads into normalized messages.
pub trait InboundAdapter {
    /// Parse a raw payload into a normalized InboundMessage.
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError>;

    /// Get the channel this adapter handles.
    fn channel(&self) -> Channel;
}

/// Trait for sending normalized outbound messages to a specific platform.
pub trait OutboundAdapter {
    /// Send an outbound message to the platform.
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError>;

    /// Get the channel this adapter handles.
    fn channel(&self) -> Channel;
}

/// Errors that can occur during adapter operations.
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("failed to parse payload: {0}")]
    ParseError(String),
    #[error("missing required field: {0}")]
    MissingField(&'static str),
    #[error("send failed: {0}")]
    SendError(String),
    #[error("configuration error: {0}")]
    ConfigError(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
}
