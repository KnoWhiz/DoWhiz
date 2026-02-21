//! Postmark email adapter for inbound and outbound messages.
//!
//! This module provides adapters for handling email via Postmark:
//! - `PostmarkInboundAdapter`: Parses Postmark webhook payloads
//! - `PostmarkOutboundAdapter`: Sends emails via Postmark API

use serde::Deserialize;
use std::collections::HashSet;

use crate::channel::{
    AdapterError, Attachment, Channel, ChannelMetadata, InboundAdapter, InboundMessage,
    OutboundAdapter, OutboundMessage, SendResult,
};
use crate::user_store::extract_emails;

/// Adapter for parsing Postmark inbound webhook payloads.
#[derive(Debug, Clone, Default)]
pub struct PostmarkInboundAdapter {
    /// Service addresses that this employee handles
    pub service_addresses: HashSet<String>,
}

impl PostmarkInboundAdapter {
    pub fn new(service_addresses: HashSet<String>) -> Self {
        Self { service_addresses }
    }

    /// Extract the thread key from a Postmark payload.
    /// Used for grouping related messages.
    pub fn extract_thread_key(&self, payload: &PostmarkInboundPayload, raw: &[u8]) -> String {
        // Try References header first
        if let Some(value) = payload.header_value("References") {
            if let Some(id) = extract_first_message_id(value) {
                return id;
            }
        }
        // Try In-Reply-To header
        if let Some(value) = payload.header_value("In-Reply-To") {
            if let Some(id) = extract_first_message_id(value) {
                return id;
            }
        }
        // Try Message-ID
        if let Some(id) = payload
            .header_message_id()
            .or(payload.message_id.as_deref())
            .and_then(normalize_message_id)
        {
            return id;
        }
        // Fall back to hash of raw payload
        format!("{:x}", md5::compute(raw))
    }

    /// Check if a sender is blacklisted (e.g., service addresses).
    pub fn is_blacklisted_sender(&self, sender: &str) -> bool {
        if sender.is_empty() {
            return false;
        }
        let addresses = extract_emails(sender);
        addresses
            .iter()
            .any(|addr| self.service_addresses.contains(&addr.to_ascii_lowercase()))
    }

    /// Find the service address that was targeted in this inbound message.
    pub fn find_service_address(&self, payload: &PostmarkInboundPayload) -> Option<String> {
        let candidates = collect_service_address_candidates(payload);
        for candidate in candidates.into_iter().flatten() {
            let emails = extract_emails(candidate);
            for email in emails {
                let lower = email.to_ascii_lowercase();
                if self.service_addresses.contains(&lower) {
                    return Some(email);
                }
            }
        }
        None
    }
}

impl InboundAdapter for PostmarkInboundAdapter {
    fn parse(&self, raw_payload: &[u8]) -> Result<InboundMessage, AdapterError> {
        let payload: PostmarkInboundPayload = serde_json::from_slice(raw_payload)
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;

        let sender = payload
            .from
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(AdapterError::MissingField("From"))?
            .to_string();

        let sender_name = extract_display_name(&sender);
        let sender_email = extract_emails(&sender)
            .into_iter()
            .next()
            .unwrap_or(sender.clone());

        // Get recipient (service address)
        let recipient = self.find_service_address(&payload).unwrap_or_default();

        // Build thread ID
        let thread_id = self.extract_thread_key(&payload, raw_payload);

        // Get message ID
        let message_id = payload
            .header_message_id()
            .or(payload.message_id.as_deref())
            .map(|s| s.trim().to_string());

        // Build reply-to list
        let reply_to_raw = payload.reply_to.as_deref().unwrap_or("");
        let from_raw = payload.from.as_deref().unwrap_or("");
        let mut reply_to = replyable_recipients(reply_to_raw);
        if reply_to.is_empty() {
            reply_to = replyable_recipients(from_raw);
        }

        // Parse attachments
        let attachments = payload
            .attachments
            .as_ref()
            .map(|list| {
                list.iter()
                    .map(|a| Attachment {
                        name: a.name.clone(),
                        content_type: a.content_type.clone(),
                        content: a.content.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Build metadata
        let in_reply_to = payload.header_value("In-Reply-To").map(|s| s.to_string());
        let references = payload.header_value("References").map(|s| s.to_string());

        Ok(InboundMessage {
            channel: Channel::Email,
            sender: sender_email,
            sender_name,
            recipient,
            subject: payload.subject.clone(),
            text_body: payload
                .text_body
                .clone()
                .or(payload.stripped_text_reply.clone()),
            html_body: payload.html_body.clone(),
            thread_id,
            message_id,
            attachments,
            reply_to,
            raw_payload: raw_payload.to_vec(),
            metadata: ChannelMetadata {
                in_reply_to,
                references,
                ..Default::default()
            },
        })
    }

    fn channel(&self) -> Channel {
        Channel::Email
    }
}

/// Adapter for sending emails via Postmark API.
#[derive(Debug, Clone, Default)]
pub struct PostmarkOutboundAdapter;

impl OutboundAdapter for PostmarkOutboundAdapter {
    fn send(&self, message: &OutboundMessage) -> Result<SendResult, AdapterError> {
        // Build SendEmailParams from OutboundMessage
        // Note: html_body from message is not used directly - send_emails_module reads from html_path
        let params = send_emails_module::SendEmailParams {
            subject: message.subject.clone(),
            html_path: message.html_path.clone().unwrap_or_default(),
            attachments_dir: message.attachments_dir.clone().unwrap_or_default(),
            from: message.from.clone(),
            to: message.to.clone(),
            cc: message.cc.clone(),
            bcc: message.bcc.clone(),
            in_reply_to: message.metadata.in_reply_to.clone(),
            references: message.metadata.references.clone(),
            reply_to: message.metadata.reply_to_header.clone(),
        };

        // For now, if html_path is provided, use the existing send_email function
        // Otherwise, we'd need to write the html_body to a temp file
        if message.html_path.is_some() {
            match send_emails_module::send_email(&params) {
                Ok(response) => Ok(SendResult {
                    success: true,
                    message_id: response.message_id,
                    submitted_at: response.submitted_at,
                    error: None,
                }),
                Err(e) => Ok(SendResult {
                    success: false,
                    message_id: String::new(),
                    submitted_at: String::new(),
                    error: Some(e.to_string()),
                }),
            }
        } else {
            // Would need to write html_body to temp file or modify send_emails_module
            // For now, return error if no html_path
            Err(AdapterError::ConfigError(
                "html_path required for Postmark adapter".to_string(),
            ))
        }
    }

    fn channel(&self) -> Channel {
        Channel::Email
    }
}

// ============================================================================
// Postmark-specific types (moved from service.rs)
// ============================================================================

/// Postmark inbound webhook payload structure.
#[derive(Debug, Clone, Deserialize)]
pub struct PostmarkInboundPayload {
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "To")]
    pub to: Option<String>,
    #[serde(rename = "Cc")]
    pub cc: Option<String>,
    #[serde(rename = "Bcc")]
    pub bcc: Option<String>,
    #[serde(rename = "ToFull")]
    pub to_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "CcFull")]
    pub cc_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "BccFull")]
    pub bcc_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "ReplyTo")]
    pub reply_to: Option<String>,
    #[serde(rename = "Subject")]
    pub subject: Option<String>,
    #[serde(rename = "TextBody")]
    pub text_body: Option<String>,
    #[serde(rename = "StrippedTextReply")]
    pub stripped_text_reply: Option<String>,
    #[serde(rename = "HtmlBody")]
    pub html_body: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    pub message_id: Option<String>,
    #[serde(rename = "Headers")]
    pub headers: Option<Vec<PostmarkHeader>>,
    #[serde(rename = "Attachments")]
    pub attachments: Option<Vec<PostmarkAttachment>>,
}

impl PostmarkInboundPayload {
    pub fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.as_ref().and_then(|headers| {
            headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case(name))
                .map(|header| header.value.as_str())
        })
    }

    pub fn header_message_id(&self) -> Option<&str> {
        self.header_value("Message-ID")
    }

    pub fn header_values(&self, name: &str) -> Vec<&str> {
        self.headers
            .as_ref()
            .map(|headers| {
                headers
                    .iter()
                    .filter(|header| header.name.eq_ignore_ascii_case(name))
                    .map(|header| header.value.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostmarkRecipient {
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "MailboxHash")]
    pub mailbox_hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostmarkHeader {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostmarkAttachment {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "ContentType")]
    pub content_type: String,
}

// ============================================================================
// Helper functions (moved/adapted from service.rs)
// ============================================================================

fn extract_first_message_id(value: &str) -> Option<String> {
    for token in value.split(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | ',' | ';')) {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(id) = normalize_message_id(trimmed) {
            return Some(id);
        }
    }
    None
}

fn normalize_message_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches(|ch| matches!(ch, '<' | '>'));
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

fn extract_display_name(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if let Some(bracket_pos) = trimmed.find('<') {
        let name = trimmed[..bracket_pos].trim();
        let name = name.trim_matches('"');
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    None
}

fn collect_service_address_candidates(payload: &PostmarkInboundPayload) -> Vec<Option<&str>> {
    let mut candidates = Vec::new();
    if let Some(value) = payload.to.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.cc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.bcc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(list) = payload.to_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.cc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.bcc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    for header in [
        "X-Original-To",
        "Delivered-To",
        "Envelope-To",
        "X-Envelope-To",
        "X-Forwarded-To",
        "X-Original-Recipient",
        "Original-Recipient",
    ] {
        for value in payload.header_values(header) {
            candidates.push(Some(value));
        }
    }
    candidates
}

fn split_recipients(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for ch in value.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
                escaped = true;
                current.push(ch);
            }
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' | ';' if !in_quotes => {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    out.push(trimmed.to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    let trimmed = current.trim();
    if !trimmed.is_empty() {
        out.push(trimmed.to_string());
    }

    out
}

fn replyable_recipients(raw: &str) -> Vec<String> {
    split_recipients(raw)
        .into_iter()
        .filter(|recipient| contains_replyable_address(recipient))
        .collect()
}

fn contains_replyable_address(value: &str) -> bool {
    let emails = extract_emails(value);
    if emails.is_empty() {
        return false;
    }
    emails.iter().any(|address| !is_no_reply_address(address))
}

const NO_REPLY_LOCAL_PARTS: [&str; 3] = ["noreply", "no-reply", "do-not-reply"];

fn is_no_reply_address(address: &str) -> bool {
    let normalized = address.trim().to_ascii_lowercase();
    let local = normalized.split('@').next().unwrap_or("");
    if local.is_empty() {
        return false;
    }
    NO_REPLY_LOCAL_PARTS.iter().any(|marker| local == *marker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_postmark_payload() {
        let payload = r#"{
            "From": "Test User <test@example.com>",
            "To": "service@example.com",
            "Subject": "Hello",
            "TextBody": "Test message",
            "MessageID": "msg-123"
        }"#;

        let mut addresses = HashSet::new();
        addresses.insert("service@example.com".to_string());
        let adapter = PostmarkInboundAdapter::new(addresses);

        let message = adapter.parse(payload.as_bytes()).unwrap();

        assert_eq!(message.channel, Channel::Email);
        assert_eq!(message.sender, "test@example.com");
        assert_eq!(message.sender_name, Some("Test User".to_string()));
        assert_eq!(message.subject, Some("Hello".to_string()));
        assert_eq!(message.text_body, Some("Test message".to_string()));
    }

    #[test]
    fn extract_thread_key_from_references() {
        let payload = r#"{
            "From": "test@example.com",
            "Headers": [
                {"Name": "References", "Value": "<original-msg-id@example.com>"}
            ]
        }"#;

        let adapter = PostmarkInboundAdapter::default();
        let parsed: PostmarkInboundPayload = serde_json::from_str(payload).unwrap();
        let thread_key = adapter.extract_thread_key(&parsed, payload.as_bytes());

        assert_eq!(thread_key, "original-msg-id@example.com");
    }

    #[test]
    fn replyable_recipients_filters_noreply() {
        let result = replyable_recipients("noreply@example.com, user@example.com");
        assert_eq!(result, vec!["user@example.com"]);
    }
}
