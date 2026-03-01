use regex::Regex;
use serde_json::Value;
use std::sync::OnceLock;

use crate::user_store::extract_emails;

const GITHUB_NOTIFICATIONS_ADDRESS: &str = "notifications@github.com";

pub(crate) fn extract_github_sender_login_from_postmark_payload(
    raw_payload: &[u8],
) -> Option<String> {
    let payload: Value = serde_json::from_slice(raw_payload).ok()?;
    extract_github_sender_login_from_value(&payload)
}

pub(crate) fn is_github_notifications_postmark_payload(raw_payload: &[u8]) -> bool {
    let payload: Value = match serde_json::from_slice(raw_payload) {
        Ok(value) => value,
        Err(_) => return false,
    };
    is_github_notifications_payload(&payload)
}

fn extract_github_sender_login_from_value(payload: &Value) -> Option<String> {
    if !is_github_notifications_payload(payload) {
        return None;
    }

    if let Some(login) = extract_github_sender_from_headers(payload) {
        return Some(login);
    }

    for field in ["StrippedTextReply", "TextBody", "HtmlBody"] {
        if let Some(body) = payload.get(field).and_then(Value::as_str) {
            if let Some(login) = extract_github_sender_from_text(body) {
                return Some(login);
            }
        }
    }

    None
}

fn is_github_notifications_payload(payload: &Value) -> bool {
    let from = payload
        .get("From")
        .or_else(|| payload.get("from"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    is_github_notifications_sender(from)
}

fn is_github_notifications_sender(from: &str) -> bool {
    extract_emails(from)
        .into_iter()
        .any(|email| email.eq_ignore_ascii_case(GITHUB_NOTIFICATIONS_ADDRESS))
}

fn extract_github_sender_from_headers(payload: &Value) -> Option<String> {
    let headers = payload.get("Headers").and_then(Value::as_array)?;
    for header in headers {
        let name = header
            .get("Name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !name.eq_ignore_ascii_case("X-GitHub-Sender") {
            continue;
        }
        let value = header
            .get("Value")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if let Some(login) = normalize_github_login(value) {
            return Some(login);
        }
    }
    None
}

fn extract_github_sender_from_text(text: &str) -> Option<String> {
    if text.trim().is_empty() {
        return None;
    }

    if let Some(captures) = github_activity_line_regex().captures(text) {
        if let Some(login) = captures.get(1).map(|m| m.as_str()) {
            if let Some(login) = normalize_github_login(login) {
                return Some(login);
            }
        }
    }

    if let Some(captures) = github_activity_html_regex().captures(text) {
        if let Some(login) = captures.get(1).map(|m| m.as_str()) {
            if let Some(login) = normalize_github_login(login) {
                return Some(login);
            }
        }
    }

    None
}

fn normalize_github_login(raw: &str) -> Option<String> {
    let trimmed = raw
        .trim()
        .trim_matches(|ch: char| matches!(ch, '"' | '\'' | '<' | '>' | '`'));
    if trimmed.is_empty() {
        return None;
    }
    if !github_login_regex().is_match(trimmed) {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

fn github_login_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(r"^[A-Za-z0-9](?:[A-Za-z0-9-]{0,38})(?:\[bot\])?$")
            .expect("valid github login regex")
    })
}

fn github_activity_line_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(
            r"(?im)^\s*([A-Za-z0-9](?:[A-Za-z0-9-]{0,38})(?:\[bot\])?)\s+(?:left a comment|created an issue|opened a pull request|opened an issue|closed an issue|reopened an issue|reviewed|requested a review)\b",
        )
        .expect("valid github activity line regex")
    })
}

fn github_activity_html_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| {
        Regex::new(
            r#"(?i)<strong>\s*([A-Za-z0-9](?:[A-Za-z0-9-]{0,38})(?:\[bot\])?)\s*</strong>\s*(?:left a comment|created an issue|opened a pull request|opened an issue)\b"#,
        )
        .expect("valid github activity html regex")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_sender_from_github_header() {
        let payload = br#"{
            "From": "Bingran You <notifications@github.com>",
            "Headers": [{"Name":"X-GitHub-Sender","Value":"bingran-you"}],
            "TextBody": "something else"
        }"#;
        let sender = extract_github_sender_login_from_postmark_payload(payload);
        assert_eq!(sender, Some("bingran-you".to_string()));
    }

    #[test]
    fn falls_back_to_text_activity_line() {
        let payload = br#"{
            "From": "notifications@github.com",
            "TextBody": "bingran-you left a comment (KnoWhiz/DoWhiz#568)"
        }"#;
        let sender = extract_github_sender_login_from_postmark_payload(payload);
        assert_eq!(sender, Some("bingran-you".to_string()));
    }

    #[test]
    fn falls_back_to_html_activity_line() {
        let payload = br#"{
            "From": "notifications@github.com",
            "HtmlBody": "<div><strong>bingran-you</strong> left a comment (KnoWhiz/DoWhiz#568)</div>"
        }"#;
        let sender = extract_github_sender_login_from_postmark_payload(payload);
        assert_eq!(sender, Some("bingran-you".to_string()));
    }

    #[test]
    fn returns_none_for_non_github_sender() {
        let payload = br#"{
            "From": "Alice <alice@example.com>",
            "Headers": [{"Name":"X-GitHub-Sender","Value":"bingran-you"}],
            "TextBody": "bingran-you left a comment"
        }"#;
        let sender = extract_github_sender_login_from_postmark_payload(payload);
        assert_eq!(sender, None);
    }

    #[test]
    fn normalizes_header_sender_case() {
        let payload = br#"{
            "From": "notifications@github.com",
            "Headers": [{"Name":"X-GitHub-Sender","Value":"Bingran-You"}]
        }"#;
        let sender = extract_github_sender_login_from_postmark_payload(payload);
        assert_eq!(sender, Some("bingran-you".to_string()));
    }

    #[test]
    fn rejects_invalid_sender_tokens() {
        assert_eq!(normalize_github_login("bingran_you"), None);
        assert_eq!(normalize_github_login("bingran you"), None);
        assert_eq!(normalize_github_login(""), None);
    }

    #[test]
    fn detects_github_notification_sender_from_postmark_payload() {
        let payload = br#"{
            "From": "Bingran You <notifications@github.com>"
        }"#;
        assert!(is_github_notifications_postmark_payload(payload));
    }

    #[test]
    fn detects_github_notification_sender_with_lowercase_from_key() {
        let payload = br#"{
            "from": "notifications@github.com"
        }"#;
        assert!(is_github_notifications_postmark_payload(payload));
    }
}
