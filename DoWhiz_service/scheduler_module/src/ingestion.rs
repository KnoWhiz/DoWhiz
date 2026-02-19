use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::warn;
use uuid::Uuid;

use crate::channel::{Attachment, Channel, ChannelMetadata, InboundMessage};
use crate::raw_payload_store;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionEnvelope {
    pub envelope_id: Uuid,
    pub received_at: DateTime<Utc>,
    pub tenant_id: Option<String>,
    pub employee_id: String,
    pub channel: Channel,
    pub external_message_id: Option<String>,
    pub dedupe_key: String,
    pub payload: IngestionPayload,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_payload_ref: Option<String>,
}

impl IngestionEnvelope {
    pub fn raw_payload_bytes(&self) -> Vec<u8> {
        if let Some(ref payload_ref) = self.raw_payload_ref {
            match raw_payload_store::download_raw_payload(payload_ref) {
                Ok(bytes) => bytes,
                Err(err) => {
                    warn!("failed to download raw payload {}: {}", payload_ref, err);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }

    pub fn to_inbound_message(&self) -> InboundMessage {
        self.payload
            .to_inbound_message(self.channel, self.raw_payload_bytes())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionPayload {
    pub sender: String,
    #[serde(default)]
    pub sender_name: Option<String>,
    pub recipient: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub text_body: Option<String>,
    #[serde(default)]
    pub html_body: Option<String>,
    pub thread_id: String,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub reply_to: Vec<String>,
    #[serde(default)]
    pub metadata: ChannelMetadata,
}

impl IngestionPayload {
    pub fn from_inbound(message: &InboundMessage) -> Self {
        Self {
            sender: message.sender.clone(),
            sender_name: message.sender_name.clone(),
            recipient: message.recipient.clone(),
            subject: message.subject.clone(),
            text_body: message.text_body.clone(),
            html_body: message.html_body.clone(),
            thread_id: message.thread_id.clone(),
            message_id: message.message_id.clone(),
            attachments: message.attachments.clone(),
            reply_to: message.reply_to.clone(),
            metadata: message.metadata.clone(),
        }
    }

    pub fn to_inbound_message(&self, channel: Channel, raw_payload: Vec<u8>) -> InboundMessage {
        InboundMessage {
            channel,
            sender: self.sender.clone(),
            sender_name: self.sender_name.clone(),
            recipient: self.recipient.clone(),
            subject: self.subject.clone(),
            text_body: self.text_body.clone(),
            html_body: self.html_body.clone(),
            thread_id: self.thread_id.clone(),
            message_id: self.message_id.clone(),
            attachments: self.attachments.clone(),
            reply_to: self.reply_to.clone(),
            raw_payload,
            metadata: self.metadata.clone(),
        }
    }
}
