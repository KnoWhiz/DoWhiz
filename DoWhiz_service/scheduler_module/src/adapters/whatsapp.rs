//! WhatsApp adapter for inbound and outbound messages via Meta Cloud API.
//!
//! This module provides adapters for handling WhatsApp messages:
//! - `WhatsAppInboundAdapter`: Parses WhatsApp webhook payloads
//! - `WhatsAppOutboundAdapter`: Sends messages via Meta Graph API

use serde::{Deserialize, Serialize};

use crate::channel::{
    AdapterError, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
    OutboundAdapter, OutboundMessage, SendResult,
};

/// Adapter for parsing WhatsApp webhook payloads.
#[derive(Debug, Clone, Default)]
pub struct WhatsAppInboundAdapter;

impl WhatsAppInboundAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl InboundAdapter for WhatsAppInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        let webhook: WhatsAppWebhook = serde_json::from_slice(raw_payload)
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        // Navigate the nested structure to get the message
        let entry = webhook
            .entry
            .first()
            .ok_or_else(|| AdapterError::ParseError("no entry in webhook".to_string()))?;

        let change = entry
            .changes
            .first()
            .ok_or_else(|| AdapterError::ParseError("no changes in entry".to_string()))?;

        let message = change
            .value
            .messages
            .as_ref()
            .and_then(|msgs| msgs.first())
            .ok_or_else(|| AdapterError::ParseError("no messages in webhook".to_string()))?;

        // Extract sender info from contacts if available
        let sender_name = change
            .value
            .contacts
            .as_ref()
            .and_then(|contacts| contacts.first())
            .and_then(|c| c.profile.as_ref())
            .map(|p| p.name.clone());

        let sender = message.from.clone();

        // Extract text content
        let text_body = message
            .text
            .as_ref()
            .map(|t| t.body.clone())
            .or_else(|| message.button.as_ref().map(|b| b.text.clone()))
            .or_else(|| message.interactive.as_ref().and_then(|i| {
                i.button_reply.as_ref().map(|b| b.title.clone())
                    .or_else(|| i.list_reply.as_ref().map(|l| l.title.clone()))
            }));

        // Use phone number as thread identifier
        let thread_id = format!("whatsapp:{}", sender);

        Ok(InboundMessage {
            channel: Channel::WhatsApp,
            sender: sender.clone(),
            sender_name,
            recipient: change.value.metadata.display_phone_number.clone().unwrap_or_default(),
            subject: None,
            text_body,
            html_body: None,
            thread_id,
            message_id: Some(message.id.clone()),
            attachments: Vec::new(), // TODO: handle media attachments
            reply_to: vec![sender.clone()],
            raw_payload: raw_payload.to_vec(),
            metadata: ChannelMetadata {
                whatsapp_phone_number: Some(sender),
                ..Default::default()
            },
        })
    }

    fn channel(&self) -> Channel {
        Channel::WhatsApp
    }
}

/// Adapter for sending messages via WhatsApp Cloud API.
#[derive(Debug, Clone)]
pub struct WhatsAppOutboundAdapter {
    /// WhatsApp access token
    pub access_token: String,
    /// Phone number ID (the bot's phone)
    pub phone_number_id: String,
}

impl WhatsAppOutboundAdapter {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        Self {
            access_token,
            phone_number_id,
        }
    }

    fn api_url(&self) -> String {
        format!(
            "https://graph.facebook.com/v17.0/{}/messages",
            self.phone_number_id
        )
    }
}

impl OutboundAdapter for WhatsAppOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        let to = message
            .metadata
            .whatsapp_phone_number
            .clone()
            .or_else(|| message.to.first().cloned())
            .ok_or(AdapterError::ConfigError(
                "no phone number specified for WhatsApp message".to_string(),
            ))?;

        let text = if message.text_body.is_empty() {
            message.html_body.clone()
        } else {
            message.text_body.clone()
        };

        let request = WhatsAppSendMessageRequest {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to,
            message_type: "text".to_string(),
            text: WhatsAppTextContent { body: text },
        };

        let url = self.api_url();
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .map_err(|e| AdapterError::SendError(e.to_string()))?;

        if status.is_success() {
            let message_id = body["messages"][0]["id"]
                .as_str()
                .unwrap_or("")
                .to_string();
            Ok(SendResult {
                success: true,
                message_id,
                submitted_at: chrono::Utc::now().to_rfc3339(),
                error: None,
            })
        } else {
            let error_msg = body["error"]["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();
            Ok(SendResult {
                success: false,
                message_id: String::new(),
                submitted_at: String::new(),
                error: Some(error_msg),
            })
        }
    }

    fn channel(&self) -> Channel {
        Channel::WhatsApp
    }
}

// ============================================================================
// WhatsApp Webhook types (Inbound)
// ============================================================================

/// Root webhook payload from WhatsApp
#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppWebhook {
    pub object: String,
    pub entry: Vec<WhatsAppEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppEntry {
    pub id: String,
    pub changes: Vec<WhatsAppChange>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppChange {
    pub value: WhatsAppValue,
    pub field: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppValue {
    pub messaging_product: String,
    pub metadata: WhatsAppMetadata,
    #[serde(default)]
    pub contacts: Option<Vec<WhatsAppContact>>,
    #[serde(default)]
    pub messages: Option<Vec<WhatsAppMessage>>,
    #[serde(default)]
    pub statuses: Option<Vec<WhatsAppStatus>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppMetadata {
    pub display_phone_number: Option<String>,
    pub phone_number_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppContact {
    pub wa_id: String,
    pub profile: Option<WhatsAppProfile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppProfile {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppMessage {
    pub id: String,
    pub from: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(default)]
    pub text: Option<WhatsAppText>,
    #[serde(default)]
    pub button: Option<WhatsAppButton>,
    #[serde(default)]
    pub interactive: Option<WhatsAppInteractive>,
    #[serde(default)]
    pub image: Option<WhatsAppMedia>,
    #[serde(default)]
    pub audio: Option<WhatsAppMedia>,
    #[serde(default)]
    pub document: Option<WhatsAppMedia>,
    #[serde(default)]
    pub video: Option<WhatsAppMedia>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppText {
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppButton {
    pub text: String,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppInteractive {
    #[serde(rename = "type")]
    pub interactive_type: Option<String>,
    pub button_reply: Option<WhatsAppButtonReply>,
    pub list_reply: Option<WhatsAppListReply>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppMedia {
    pub id: String,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WhatsAppStatus {
    pub id: String,
    pub status: String,
    pub timestamp: String,
    pub recipient_id: String,
}

// ============================================================================
// WhatsApp Send types (Outbound)
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct WhatsAppSendMessageRequest {
    messaging_product: String,
    recipient_type: String,
    to: String,
    #[serde(rename = "type")]
    message_type: String,
    text: WhatsAppTextContent,
}

#[derive(Debug, Clone, Serialize)]
struct WhatsAppTextContent {
    body: String,
}

// ============================================================================
// Helper functions
// ============================================================================

/// Send a quick response via WhatsApp (async version).
pub async fn send_quick_whatsapp_response(
    access_token: &str,
    phone_number_id: &str,
    to: &str,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://graph.facebook.com/v17.0/{}/messages",
        phone_number_id
    );

    let request = serde_json::json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": to,
        "type": "text",
        "text": {
            "body": response_text
        }
    });

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("WhatsApp API returned {}: {}", status, body).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_text_message() {
        let payload = r#"{
            "object": "whatsapp_business_account",
            "entry": [{
                "id": "123456789",
                "changes": [{
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "15551234567",
                            "phone_number_id": "987654321"
                        },
                        "contacts": [{
                            "wa_id": "14155551234",
                            "profile": {
                                "name": "Dylan Tang"
                            }
                        }],
                        "messages": [{
                            "id": "wamid.abc123",
                            "from": "14155551234",
                            "timestamp": "1234567890",
                            "type": "text",
                            "text": {
                                "body": "Hello from WhatsApp!"
                            }
                        }]
                    },
                    "field": "messages"
                }]
            }]
        }"#;

        let adapter = WhatsAppInboundAdapter::new();
        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.channel, Channel::WhatsApp);
        assert_eq!(message.sender, "14155551234");
        assert_eq!(message.sender_name, Some("Dylan Tang".to_string()));
        assert_eq!(message.text_body, Some("Hello from WhatsApp!".to_string()));
        assert_eq!(message.metadata.whatsapp_phone_number, Some("14155551234".to_string()));
    }

    #[test]
    fn ignore_status_updates() {
        let payload = r#"{
            "object": "whatsapp_business_account",
            "entry": [{
                "id": "123456789",
                "changes": [{
                    "value": {
                        "messaging_product": "whatsapp",
                        "metadata": {
                            "display_phone_number": "15551234567",
                            "phone_number_id": "987654321"
                        },
                        "statuses": [{
                            "id": "wamid.abc123",
                            "status": "delivered",
                            "timestamp": "1234567890",
                            "recipient_id": "14155551234"
                        }]
                    },
                    "field": "messages"
                }]
            }]
        }"#;

        let adapter = WhatsAppInboundAdapter::new();
        let result = adapter.parse(payload.as_bytes());
        assert!(result.is_err()); // Status updates should fail parsing as messages
    }
}
