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
    /// Slack
    Slack,
    /// Discord
    Discord,
    /// SMS (e.g., Twilio)
    Sms,
    /// Telegram
    Telegram,
    /// WhatsApp via Meta Cloud API
    WhatsApp,
    /// Google Docs collaboration via comments
    GoogleDocs,
    /// Google Sheets collaboration via comments
    GoogleSheets,
    /// Google Slides collaboration via comments
    GoogleSlides,
    /// iMessage via BlueBubbles bridge
    BlueBubbles,
    /// WeChat Work (企业微信) via qyapi
    WeChat,
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
            Channel::Discord => write!(f, "discord"),
            Channel::Sms => write!(f, "sms"),
            Channel::Telegram => write!(f, "telegram"),
            Channel::WhatsApp => write!(f, "whatsapp"),
            Channel::GoogleDocs => write!(f, "google_docs"),
            Channel::GoogleSheets => write!(f, "google_sheets"),
            Channel::GoogleSlides => write!(f, "google_slides"),
            Channel::BlueBubbles => write!(f, "bluebubbles"),
            Channel::WeChat => write!(f, "wechat"),
        }
    }
}

impl std::str::FromStr for Channel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "email" => Ok(Channel::Email),
            "slack" => Ok(Channel::Slack),
            "discord" => Ok(Channel::Discord),
            "sms" => Ok(Channel::Sms),
            "telegram" => Ok(Channel::Telegram),
            "whatsapp" => Ok(Channel::WhatsApp),
            "google_docs" | "googledocs" => Ok(Channel::GoogleDocs),
            "google_sheets" | "googlesheets" => Ok(Channel::GoogleSheets),
            "google_slides" | "googleslides" => Ok(Channel::GoogleSlides),
            "bluebubbles" | "imessage" => Ok(Channel::BlueBubbles),
            "wechat" | "weixin" => Ok(Channel::WeChat),
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Filename
    pub name: String,
    /// MIME content type
    pub content_type: String,
    /// Base64-encoded content
    pub content: String,
}

/// Platform-specific metadata that doesn't fit in the common fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelMetadata {
    /// Email-specific: In-Reply-To header
    pub in_reply_to: Option<String>,
    /// Email-specific: References header
    pub references: Option<String>,
    /// Email-specific: Reply-To header (where replies should be sent)
    pub reply_to_header: Option<String>,
    /// Slack-specific: Channel ID
    pub slack_channel_id: Option<String>,
    /// Slack-specific: Team ID
    pub slack_team_id: Option<String>,
    /// Discord-specific: Guild (server) ID
    pub discord_guild_id: Option<u64>,
    /// Discord-specific: Channel ID
    pub discord_channel_id: Option<u64>,
    /// Discord-specific: Current inbound message ID
    pub discord_message_id: Option<String>,
    /// Discord-specific: Quoted/referenced message ID (if this message is a reply)
    pub discord_referenced_message_id: Option<String>,
    /// Telegram-specific: Chat ID
    pub telegram_chat_id: Option<i64>,
    /// WhatsApp-specific: Phone number (sender's phone)
    pub whatsapp_phone_number: Option<String>,
    /// SMS-specific: From phone number
    pub sms_from: Option<String>,
    /// SMS-specific: To phone number
    pub sms_to: Option<String>,
    /// Google Docs-specific: Document ID
    pub google_docs_document_id: Option<String>,
    /// Google Docs-specific: Comment ID to reply to
    pub google_docs_comment_id: Option<String>,
    /// Google Docs-specific: Document name/title
    pub google_docs_document_name: Option<String>,
    /// Google Sheets-specific: Spreadsheet ID
    pub google_sheets_spreadsheet_id: Option<String>,
    /// Google Sheets-specific: Comment ID to reply to
    pub google_sheets_comment_id: Option<String>,
    /// Google Sheets-specific: Spreadsheet name/title
    pub google_sheets_spreadsheet_name: Option<String>,
    /// Google Sheets-specific: Sheet name (tab) where comment is located
    pub google_sheets_sheet_name: Option<String>,
    /// Google Slides-specific: Presentation ID
    pub google_slides_presentation_id: Option<String>,
    /// Google Slides-specific: Comment ID to reply to
    pub google_slides_comment_id: Option<String>,
    /// Google Slides-specific: Presentation name/title
    pub google_slides_presentation_name: Option<String>,
    /// Google Slides-specific: Slide number where comment is located
    pub google_slides_slide_number: Option<i32>,
    /// Google Docs-specific: Document owner's email (for account linking fallback)
    pub google_docs_owner_email: Option<String>,
    /// Google Sheets-specific: Spreadsheet owner's email (for account linking fallback)
    pub google_sheets_owner_email: Option<String>,
    /// Google Slides-specific: Presentation owner's email (for account linking fallback)
    pub google_slides_owner_email: Option<String>,
    /// BlueBubbles-specific: Chat GUID (e.g., "iMessage;-;+1234567890")
    pub bluebubbles_chat_guid: Option<String>,
    /// WeChat Work-specific: Corp ID (企业ID)
    pub wechat_corp_id: Option<String>,
    /// WeChat Work-specific: User ID (用户ID)
    pub wechat_user_id: Option<String>,
    /// WeChat Work-specific: Agent ID (应用ID)
    pub wechat_agent_id: Option<String>,

    // =========================================================================
    // Multi-channel collaboration support
    // =========================================================================
    /// Collaboration session ID linking multiple messages across channels.
    /// When set, this message is part of a collaboration session.
    pub collaboration_session_id: Option<String>,

    /// Artifacts extracted from this message (Google Docs, GitHub PRs, etc.).
    /// Used for linking messages to collaboration sessions.
    pub extracted_artifacts: Option<Vec<ExtractedArtifactRef>>,
}

/// Reference to an extracted artifact (lightweight version for metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedArtifactRef {
    /// Type of artifact (e.g., "google_docs", "github_pr", "notion")
    pub artifact_type: String,
    /// External ID of the artifact
    pub artifact_id: String,
    /// Full URL to the artifact
    pub url: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Channel Enum Tests ====================

    #[test]
    fn channel_display_wechat() {
        assert_eq!(Channel::WeChat.to_string(), "wechat");
    }

    #[test]
    fn channel_display_all() {
        assert_eq!(Channel::Email.to_string(), "email");
        assert_eq!(Channel::Slack.to_string(), "slack");
        assert_eq!(Channel::Discord.to_string(), "discord");
        assert_eq!(Channel::Sms.to_string(), "sms");
        assert_eq!(Channel::Telegram.to_string(), "telegram");
        assert_eq!(Channel::WhatsApp.to_string(), "whatsapp");
        assert_eq!(Channel::GoogleDocs.to_string(), "google_docs");
        assert_eq!(Channel::GoogleSheets.to_string(), "google_sheets");
        assert_eq!(Channel::GoogleSlides.to_string(), "google_slides");
        assert_eq!(Channel::BlueBubbles.to_string(), "bluebubbles");
        assert_eq!(Channel::WeChat.to_string(), "wechat");
    }

    #[test]
    fn channel_from_str_wechat() {
        assert_eq!("wechat".parse::<Channel>().unwrap(), Channel::WeChat);
        assert_eq!("WeChat".parse::<Channel>().unwrap(), Channel::WeChat);
        assert_eq!("WECHAT".parse::<Channel>().unwrap(), Channel::WeChat);
        assert_eq!("weixin".parse::<Channel>().unwrap(), Channel::WeChat);
        assert_eq!("Weixin".parse::<Channel>().unwrap(), Channel::WeChat);
    }

    #[test]
    fn channel_from_str_all_variants() {
        assert_eq!("email".parse::<Channel>().unwrap(), Channel::Email);
        assert_eq!("slack".parse::<Channel>().unwrap(), Channel::Slack);
        assert_eq!("discord".parse::<Channel>().unwrap(), Channel::Discord);
        assert_eq!("sms".parse::<Channel>().unwrap(), Channel::Sms);
        assert_eq!("telegram".parse::<Channel>().unwrap(), Channel::Telegram);
        assert_eq!("whatsapp".parse::<Channel>().unwrap(), Channel::WhatsApp);
        assert_eq!("google_docs".parse::<Channel>().unwrap(), Channel::GoogleDocs);
        assert_eq!("googledocs".parse::<Channel>().unwrap(), Channel::GoogleDocs);
        assert_eq!("google_sheets".parse::<Channel>().unwrap(), Channel::GoogleSheets);
        assert_eq!("google_slides".parse::<Channel>().unwrap(), Channel::GoogleSlides);
        assert_eq!("bluebubbles".parse::<Channel>().unwrap(), Channel::BlueBubbles);
        assert_eq!("imessage".parse::<Channel>().unwrap(), Channel::BlueBubbles);
    }

    #[test]
    fn channel_from_str_unknown_fails() {
        assert!("unknown".parse::<Channel>().is_err());
        assert!("facebook".parse::<Channel>().is_err());
        assert!("".parse::<Channel>().is_err());
    }

    #[test]
    fn channel_serde_roundtrip_wechat() {
        let channel = Channel::WeChat;
        let json = serde_json::to_string(&channel).unwrap();
        assert_eq!(json, "\"we_chat\""); // snake_case serialization
        let parsed: Channel = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Channel::WeChat);
    }

    #[test]
    fn channel_serde_roundtrip_all() {
        let channels = vec![
            Channel::Email,
            Channel::Slack,
            Channel::Discord,
            Channel::Sms,
            Channel::Telegram,
            Channel::WhatsApp,
            Channel::GoogleDocs,
            Channel::GoogleSheets,
            Channel::GoogleSlides,
            Channel::BlueBubbles,
            Channel::WeChat,
        ];
        for channel in channels {
            let json = serde_json::to_string(&channel).unwrap();
            let parsed: Channel = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, channel, "roundtrip failed for {:?}", channel);
        }
    }

    #[test]
    fn channel_default_is_email() {
        assert_eq!(Channel::default(), Channel::Email);
    }

    // ==================== ChannelMetadata WeChat Fields Tests ====================

    #[test]
    fn channel_metadata_wechat_fields_default() {
        let meta = ChannelMetadata::default();
        assert!(meta.wechat_corp_id.is_none());
        assert!(meta.wechat_user_id.is_none());
        assert!(meta.wechat_agent_id.is_none());
    }

    #[test]
    fn channel_metadata_wechat_fields_set() {
        let meta = ChannelMetadata {
            wechat_corp_id: Some("ww1234567890".to_string()),
            wechat_user_id: Some("zhangsan".to_string()),
            wechat_agent_id: Some("1000002".to_string()),
            ..Default::default()
        };
        assert_eq!(meta.wechat_corp_id.as_deref(), Some("ww1234567890"));
        assert_eq!(meta.wechat_user_id.as_deref(), Some("zhangsan"));
        assert_eq!(meta.wechat_agent_id.as_deref(), Some("1000002"));
    }

    #[test]
    fn channel_metadata_serde_wechat() {
        let meta = ChannelMetadata {
            wechat_corp_id: Some("corp123".to_string()),
            wechat_user_id: Some("user456".to_string()),
            wechat_agent_id: Some("789".to_string()),
            ..Default::default()
        };
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("wechat_corp_id"));
        assert!(json.contains("corp123"));

        let parsed: ChannelMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.wechat_corp_id, meta.wechat_corp_id);
        assert_eq!(parsed.wechat_user_id, meta.wechat_user_id);
        assert_eq!(parsed.wechat_agent_id, meta.wechat_agent_id);
    }
}
