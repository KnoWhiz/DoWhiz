use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub(crate) struct ReplyContext {
    pub(crate) subject: String,
    pub(crate) in_reply_to: Option<String>,
    pub(crate) references: Option<String>,
    pub(crate) from: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PostmarkInboundLite {
    #[serde(rename = "Subject")]
    subject: Option<String>,
    #[serde(rename = "StrippedTextReply")]
    stripped_text_reply: Option<String>,
    #[serde(rename = "TextBody")]
    text_body: Option<String>,
    #[serde(rename = "To")]
    #[allow(dead_code)]
    to: Option<String>,
    #[serde(rename = "Cc")]
    #[allow(dead_code)]
    cc: Option<String>,
    #[serde(rename = "Bcc")]
    #[allow(dead_code)]
    bcc: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    message_id: Option<String>,
    #[serde(rename = "Headers")]
    headers: Option<Vec<PostmarkHeaderLite>>,
}

#[derive(Debug, Deserialize)]
struct PostmarkHeaderLite {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

impl PostmarkInboundLite {
    fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.as_ref().and_then(|headers| {
            headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case(name))
                .map(|header| header.value.as_str())
        })
    }

    fn header_message_id(&self) -> Option<&str> {
        self.header_value("message-id")
    }

    fn reply_subject(&self) -> String {
        if let Some(subject) = normalized_original_subject(self.subject.as_deref().unwrap_or("")) {
            return ensure_reply_prefix(&subject);
        }

        self.stripped_text_reply
            .as_deref()
            .and_then(normalize_reply_subject_hint)
            .or_else(|| {
                self.text_body
                    .as_deref()
                    .and_then(normalize_reply_subject_hint)
            })
            .map(|summary| ensure_reply_prefix(&summary))
            .unwrap_or_else(|| reply_subject(""))
    }
}

#[derive(Debug, Deserialize)]
struct DiscordMetaLite {
    channel: Option<String>,
    message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SlackMetaLite {
    channel: Option<String>,
    thread_id: Option<String>,
}

pub(crate) fn load_reply_context(workspace_dir: &Path) -> ReplyContext {
    let incoming_dir = workspace_dir.join("incoming_email");

    // Discord: always reply to the current inbound message.
    if let Some(message_id) = latest_discord_message_id(&incoming_dir) {
        return ReplyContext {
            subject: "Discord reply".to_string(),
            in_reply_to: Some(message_id),
            references: None,
            from: None,
        };
    }

    // Slack: reply in the same thread.
    if let Some(thread_ts) = latest_slack_thread_id(&incoming_dir) {
        return ReplyContext {
            subject: "Slack reply".to_string(),
            in_reply_to: Some(thread_ts),
            references: None,
            from: None,
        };
    }

    // Try Google Docs metadata first
    let gdocs_metadata_path = incoming_dir.join("google_docs_metadata.json");
    if let Ok(content) = fs::read_to_string(&gdocs_metadata_path) {
        if let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&content) {
            let document_id = metadata
                .get("document_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let comment_id = metadata
                .get("comment_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let document_name = metadata
                .get("document_name")
                .and_then(|v| v.as_str())
                .unwrap_or("Document");

            if !document_id.is_empty() && !comment_id.is_empty() {
                // For Google Docs, in_reply_to format is "document_id:comment_id"
                let in_reply_to = format!("{}:{}", document_id, comment_id);
                return ReplyContext {
                    subject: format!("Re: Comment on {}", document_name),
                    in_reply_to: Some(in_reply_to),
                    references: None,
                    from: None,
                };
            }
        }
    }

    // Fall back to Postmark (email) payload
    let payload_path = incoming_dir.join("postmark_payload.json");
    let payload = fs::read_to_string(&payload_path)
        .ok()
        .and_then(|content| serde_json::from_str::<PostmarkInboundLite>(&content).ok());

    if let Some(payload) = payload {
        let subject = payload.reply_subject();
        let (in_reply_to, references) = reply_headers(&payload);
        ReplyContext {
            subject,
            in_reply_to,
            references,
            from: None,
        }
    } else {
        ReplyContext {
            subject: reply_subject(""),
            in_reply_to: None,
            references: None,
            from: None,
        }
    }
}

fn latest_discord_message_id(incoming_dir: &Path) -> Option<String> {
    let entries = fs::read_dir(incoming_dir).ok()?;
    let mut meta_files = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.ends_with("_discord_meta.json"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    meta_files.sort();
    let latest = meta_files.last()?;
    let content = fs::read_to_string(latest).ok()?;
    let meta = serde_json::from_str::<DiscordMetaLite>(&content).ok()?;
    if meta
        .channel
        .as_deref()
        .map(|channel| channel.eq_ignore_ascii_case("discord"))
        .unwrap_or(false)
    {
        return meta
            .message_id
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }
    None
}

fn latest_slack_thread_id(incoming_dir: &Path) -> Option<String> {
    let entries = fs::read_dir(incoming_dir).ok()?;
    let mut meta_files = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.ends_with("_slack_meta.json"))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    meta_files.sort();
    let latest = meta_files.last()?;
    let content = fs::read_to_string(latest).ok()?;
    let meta = serde_json::from_str::<SlackMetaLite>(&content).ok()?;
    if meta
        .channel
        .as_deref()
        .map(|channel| channel.eq_ignore_ascii_case("slack"))
        .unwrap_or(false)
    {
        return meta
            .thread_id
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
    }
    None
}

const REPLY_SUBJECT_FALLBACK: &str = "Your request";
const REPLY_SUBJECT_HINT_MAX_CHARS: usize = 72;

fn reply_subject(original: &str) -> String {
    normalized_original_subject(original)
        .map(|subject| ensure_reply_prefix(&subject))
        .unwrap_or_else(|| format!("Re: {}", REPLY_SUBJECT_FALLBACK))
}

fn ensure_reply_prefix(subject: &str) -> String {
    let trimmed = subject.trim();
    if trimmed.to_ascii_lowercase().starts_with("re:") {
        trimmed.to_string()
    } else {
        format!("Re: {}", trimmed)
    }
}

fn normalized_original_subject(original: &str) -> Option<String> {
    let trimmed = original.trim();
    if trimmed.is_empty() || is_placeholder_subject(trimmed) {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn normalize_reply_subject_hint(raw: &str) -> Option<String> {
    let mut body_started = false;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            body_started = true;
            continue;
        }

        if !body_started && is_reply_header_line(trimmed) {
            continue;
        }

        return clean_reply_subject_hint(trimmed);
    }

    None
}

fn is_reply_header_line(line: &str) -> bool {
    let lowered = line.to_ascii_lowercase();
    lowered.starts_with("from:")
        || lowered.starts_with("date:")
        || lowered.starts_with("to:")
        || lowered.starts_with("cc:")
        || lowered.starts_with("bcc:")
        || lowered.starts_with("subject:")
}

fn clean_reply_subject_hint(line: &str) -> Option<String> {
    let compact = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.is_empty() || is_placeholder_subject(&compact) {
        return None;
    }
    Some(truncate_reply_subject_hint(
        &compact,
        REPLY_SUBJECT_HINT_MAX_CHARS,
    ))
}

fn truncate_reply_subject_hint(value: &str, max_chars: usize) -> String {
    let mut chars = value.chars();
    let mut output = String::new();

    for _ in 0..max_chars {
        match chars.next() {
            Some(ch) => output.push(ch),
            None => return output,
        }
    }

    if chars.next().is_some() {
        output.push_str("...");
    }

    output
}

fn is_placeholder_subject(subject: &str) -> bool {
    let mut remainder = subject.trim();
    loop {
        let lowered = remainder.to_ascii_lowercase();
        if lowered.starts_with("re:") {
            remainder = remainder[3..].trim_start();
            continue;
        }
        if lowered.starts_with("fw:") {
            remainder = remainder[3..].trim_start();
            continue;
        }
        if lowered.starts_with("fwd:") {
            remainder = remainder[4..].trim_start();
            continue;
        }
        break;
    }

    remainder
        .trim()
        .trim_matches(|ch: char| matches!(ch, '(' | ')' | '[' | ']'))
        .trim()
        .eq_ignore_ascii_case("no subject")
}

fn reply_headers(payload: &PostmarkInboundLite) -> (Option<String>, Option<String>) {
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());

    let mut references = payload
        .header_value("References")
        .or_else(|| payload.header_value("In-Reply-To"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    if let Some(ref msg_id) = message_id {
        references = match references {
            Some(existing) => {
                if references_contains(&existing, msg_id) {
                    Some(existing)
                } else {
                    Some(format!("{existing} {msg_id}"))
                }
            }
            None => Some(msg_id.clone()),
        };
    }

    (message_id, references)
}

fn references_contains(references: &str, message_id: &str) -> bool {
    references
        .split_whitespace()
        .any(|entry| entry == message_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_reply_context_prefers_latest_discord_message_id() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");

        fs::write(
            incoming_dir.join("00001_discord_meta.json"),
            r#"{"channel":"discord","message_id":"1001"}"#,
        )
        .expect("write first");
        fs::write(
            incoming_dir.join("00002_discord_meta.json"),
            r#"{"channel":"discord","message_id":"1002"}"#,
        )
        .expect("write second");

        let context = load_reply_context(temp.path());
        assert_eq!(context.in_reply_to.as_deref(), Some("1002"));
    }

    #[test]
    fn load_reply_context_falls_back_to_email_headers() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("postmark_payload.json"),
            r#"{
                "Subject": "Hello",
                "MessageID": "<msg-123>",
                "Headers": [{"Name":"References","Value":"<msg-122>"}]
            }"#,
        )
        .expect("payload");

        let context = load_reply_context(temp.path());
        assert_eq!(context.subject, "Re: Hello");
        assert_eq!(context.in_reply_to.as_deref(), Some("<msg-123>"));
        assert_eq!(context.references.as_deref(), Some("<msg-122> <msg-123>"));
    }

    #[test]
    fn load_reply_context_prefers_latest_slack_thread_id() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");

        fs::write(
            incoming_dir.join("00001_slack_meta.json"),
            r#"{"channel":"slack","thread_id":"1700000000.001"}"#,
        )
        .expect("write first");
        fs::write(
            incoming_dir.join("00002_slack_meta.json"),
            r#"{"channel":"slack","thread_id":"1700000000.002"}"#,
        )
        .expect("write second");

        let context = load_reply_context(temp.path());
        assert_eq!(context.subject, "Slack reply");
        assert_eq!(context.in_reply_to.as_deref(), Some("1700000000.002"));
    }

    #[test]
    fn load_reply_context_uses_stripped_reply_when_subject_missing() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("postmark_payload.json"),
            r#"{
                "Subject": "",
                "StrippedTextReply": "Need help with payroll export\n\n> quoted message",
                "MessageID": "<msg-456>"
            }"#,
        )
        .expect("payload");

        let context = load_reply_context(temp.path());
        assert_eq!(context.subject, "Re: Need help with payroll export");
    }

    #[test]
    fn load_reply_context_treats_no_subject_placeholder_as_missing() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("postmark_payload.json"),
            r#"{
                "Subject": "Re: (no subject)",
                "TextBody": "From: sender@example.com\nSubject: \n\nCan you resend the invoice?\nThanks",
                "MessageID": "<msg-789>"
            }"#,
        )
        .expect("payload");

        let context = load_reply_context(temp.path());
        assert_eq!(context.subject, "Re: Can you resend the invoice?");
    }

    #[test]
    fn load_reply_context_uses_generic_subject_when_email_has_no_subject_or_body() {
        let temp = TempDir::new().expect("tempdir");
        let incoming_dir = temp.path().join("incoming_email");
        fs::create_dir_all(&incoming_dir).expect("incoming_email");
        fs::write(
            incoming_dir.join("postmark_payload.json"),
            r#"{
                "Subject": " ",
                "TextBody": "\n  ",
                "MessageID": "<msg-999>"
            }"#,
        )
        .expect("payload");

        let context = load_reply_context(temp.path());
        assert_eq!(context.subject, "Re: Your request");
    }
}
