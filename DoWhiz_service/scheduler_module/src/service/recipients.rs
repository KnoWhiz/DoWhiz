use crate::user_store::extract_emails;

pub(super) fn replyable_recipients(raw: &str) -> Vec<String> {
    split_recipients(raw)
        .into_iter()
        .filter(|recipient| contains_replyable_address(recipient))
        .collect()
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

fn contains_replyable_address(value: &str) -> bool {
    let emails = extract_emails(value);
    if emails.is_empty() {
        return false;
    }
    emails.iter().any(|address| !is_no_reply_address(address))
}

// Only local-part markers; avoid domain-based filtering.
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
    fn replyable_recipients_filters_no_reply_addresses() {
        let raw = "No Reply <noreply@example.com>, Real <user@example.com>";
        let recipients = replyable_recipients(raw);
        assert_eq!(recipients, vec!["Real <user@example.com>"]);
    }

    #[test]
    fn replyable_recipients_returns_empty_when_only_no_reply() {
        let raw = "No Reply <no-reply@example.com>";
        let recipients = replyable_recipients(raw);
        assert!(recipients.is_empty());
    }

    #[test]
    fn replyable_recipients_keeps_quoted_display_name_commas() {
        let raw =
            "\"Zoom Video Communications, Inc\" <reply@example.com>, Other <other@example.com>";
        let recipients = replyable_recipients(raw);
        assert_eq!(
            recipients,
            vec![
                "\"Zoom Video Communications, Inc\" <reply@example.com>",
                "Other <other@example.com>"
            ]
        );
    }

    #[test]
    fn no_reply_detection_matches_common_variants() {
        assert!(is_no_reply_address("noreply@example.com"));
        assert!(is_no_reply_address("no-reply@example.com"));
        assert!(is_no_reply_address("do-not-reply@example.com"));
        assert!(!is_no_reply_address("reply@example.com"));
    }

    #[test]
    fn no_reply_detection_requires_exact_local_part() {
        assert!(!is_no_reply_address("noreplying@example.com"));
        assert!(!is_no_reply_address("reply-noreply@example.com"));
        assert!(!is_no_reply_address("no-reply-bot@example.com"));
    }

    #[test]
    fn no_reply_detection_ignores_domain_markers() {
        assert!(!is_no_reply_address("notifications@github.com"));
        assert!(!is_no_reply_address("octocat@users.noreply.github.com"));
    }
}
