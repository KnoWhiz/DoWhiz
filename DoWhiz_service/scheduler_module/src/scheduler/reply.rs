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
        let subject_raw = payload.subject.as_deref().unwrap_or("");
        let subject = reply_subject(subject_raw);
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

fn reply_subject(original: &str) -> String {
    let trimmed = original.trim();
    if trimmed.is_empty() {
        "Re: (no subject)".to_string()
    } else if trimmed.to_lowercase().starts_with("re:") {
        trimmed.to_string()
    } else {
        format!("Re: {}", trimmed)
    }
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
}
