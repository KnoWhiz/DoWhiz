use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PostmarkInbound {
    #[serde(rename = "From")]
    pub(super) from: Option<String>,
    #[serde(rename = "To")]
    #[allow(dead_code)]
    pub(super) to: Option<String>,
    #[serde(rename = "Cc")]
    #[allow(dead_code)]
    pub(super) cc: Option<String>,
    #[serde(rename = "Bcc")]
    #[allow(dead_code)]
    pub(super) bcc: Option<String>,
    #[serde(rename = "ToFull")]
    #[allow(dead_code)]
    pub(super) to_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "CcFull")]
    #[allow(dead_code)]
    pub(super) cc_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "BccFull")]
    #[allow(dead_code)]
    pub(super) bcc_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "ReplyTo")]
    pub(super) reply_to: Option<String>,
    #[serde(rename = "Subject")]
    pub(super) subject: Option<String>,
    #[serde(rename = "TextBody")]
    pub(super) text_body: Option<String>,
    #[serde(rename = "StrippedTextReply")]
    pub(super) stripped_text_reply: Option<String>,
    #[serde(rename = "HtmlBody")]
    pub(super) html_body: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    pub(super) message_id: Option<String>,
    #[serde(rename = "Headers")]
    pub(super) headers: Option<Vec<PostmarkHeader>>,
    #[serde(rename = "Attachments")]
    pub(super) attachments: Option<Vec<PostmarkAttachment>>,
}

impl PostmarkInbound {
    pub(super) fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.as_ref().and_then(|headers| {
            headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case(name))
                .map(|header| header.value.as_str())
        })
    }

    pub(super) fn header_message_id(&self) -> Option<&str> {
        self.header_value("Message-ID")
    }

    pub(super) fn header_values(&self, name: &str) -> Vec<&str> {
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
pub(super) struct PostmarkRecipient {
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Name")]
    #[allow(dead_code)]
    name: Option<String>,
    #[serde(rename = "MailboxHash")]
    #[allow(dead_code)]
    mailbox_hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PostmarkHeader {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct PostmarkAttachment {
    #[serde(rename = "Name")]
    pub(super) name: String,
    #[serde(rename = "Content")]
    pub(super) content: String,
    #[serde(rename = "ContentType")]
    #[allow(dead_code)]
    pub(super) content_type: String,
}

pub(super) fn normalize_message_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches(|ch| matches!(ch, '<' | '>'));
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

pub(super) fn collect_service_address_candidates(payload: &PostmarkInbound) -> Vec<Option<&str>> {
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
