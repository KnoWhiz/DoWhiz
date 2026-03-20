//! Notion email notification detector.
//!
//! This module detects and parses email notifications from Notion.
//! Notion sends emails from `notify@mail.notion.so` when:
//! - Someone @mentions you in a comment
//! - Someone mentions you in a page
//! - Someone comments on a page you're subscribed to
//!
//! By detecting these emails, we can trigger Notion-related tasks without
//! needing to poll the Notion inbox via browser automation.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use flate2::read::ZlibDecoder;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::LazyLock;
use tracing::{debug, warn};

/// Detected Notion email notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionEmailNotification {
    /// Type of notification
    pub notification_type: NotionNotificationType,
    /// Person who triggered the notification (e.g., who mentioned you)
    pub actor_name: Option<String>,
    /// Notion page URL extracted from the email
    pub page_url: Option<String>,
    /// Notion page ID extracted from the URL
    pub page_id: Option<String>,
    /// Workspace ID (space_id) extracted from tracking URL metadata
    pub workspace_id: Option<String>,
    /// Workspace name (if detectable)
    pub workspace_name: Option<String>,
    /// Page title (from email subject or body)
    pub page_title: Option<String>,
    /// Preview of the comment/mention text
    pub comment_preview: Option<String>,
    /// Direct link to the comment (if available)
    pub comment_url: Option<String>,
    /// Original email subject
    pub subject: String,
}

/// Types of Notion notifications we can detect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotionNotificationType {
    /// @mention in a comment
    CommentMention,
    /// Mention in page content
    PageMention,
    /// Reply to your comment
    CommentReply,
    /// New comment on a page you follow
    PageComment,
    /// Generic Notion notification
    Other,
}

/// Sender domains that indicate a Notion notification email.
const NOTION_SENDER_DOMAINS: &[&str] = &[
    "mail.notion.so",
    "notion.so",
    "makenotion.com",
];

/// Regex patterns for extracting Notion information from emails.
/// Matches page URLs like:
/// - https://notion.so/workspace/Page-Title-abc123...
/// - https://www.notion.so/Page-Title-abc123...
/// Page URLs are identified by having a 32-char hex UUID suffix.
static NOTION_URL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    // Match page URLs with UUID suffix (32 hex chars)
    // The UUID is required to distinguish pages from static assets
    // Note: We'll filter out non-page paths in the extraction function
    Regex::new(r"https://(?:www\.)?notion\.so/([a-zA-Z0-9_/-]+[a-f0-9]{32})")
        .expect("valid regex")
});

/// Non-page paths to filter out (static assets, API, etc.)
const NON_PAGE_PATHS: &[&str] = &["images/", "api/", "fonts/", "assets/", "icons/", "static/"];

static NOTION_SITE_URL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://([a-zA-Z0-9_-]+)\.notion\.site/([a-zA-Z0-9_-]+(?:-[a-f0-9]{32})?)")
        .expect("valid regex")
});

/// Pattern to detect comment URLs with discussion parameter
static NOTION_COMMENT_URL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://(?:www\.)?notion\.so/[^\s]*[?&]d=([a-f0-9-]+)")
        .expect("valid regex")
});

/// Pattern to detect Notion tracking URLs (e.g., https://mg.mail.notion.so/c/eJx...)
/// These URLs contain base64url-encoded, zlib-compressed metadata including space_id
static NOTION_TRACKING_URL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"https://mg\.mail\.notion\.so/c/([a-zA-Z0-9_-]+)"#).expect("valid regex")
});

/// Pattern to extract actor name from "X mentioned you" style text (English)
static MENTIONED_YOU_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"([A-Za-z][A-Za-z\s]+?)\s+(?:mentioned you|replied to|commented on|commented in)")
        .expect("valid regex")
});

/// Pattern to extract actor name from Chinese subject patterns
/// e.g., "Liu Xintong 在 Dowhiz testing 发表了评论"
/// Note: Uses [^\s] to match any non-whitespace char (including Unicode)
static CHINESE_ACTOR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([^\s]+(?:\s+[^\s]+)*?)\s+在\s+[^\s]+(?:\s+[^\s]+)*\s+(?:发表了评论|评论了|中?提及了您|@了您|回复了)")
        .expect("valid regex")
});

/// Pattern to extract page title from subject like "Re: [Page Title]" or "Comment on Page Title"
static PAGE_TITLE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:Comment on|Mention in|Reply to|Re:)\s*[:\[]?\s*(.+?)(?:\]|$)")
        .expect("valid regex")
});

/// Check if an email sender is from Notion.
pub fn is_notion_sender(sender: &str) -> bool {
    let sender_lower = sender.to_lowercase();
    NOTION_SENDER_DOMAINS.iter().any(|domain| sender_lower.contains(domain))
}

/// Detect if an email is a Notion notification and parse its contents.
///
/// Returns `Some(NotionEmailNotification)` if the email is from Notion,
/// `None` otherwise.
pub fn detect_notion_email(
    sender: &str,
    subject: &str,
    text_body: Option<&str>,
    html_body: Option<&str>,
) -> Option<NotionEmailNotification> {
    if !is_notion_sender(sender) {
        return None;
    }

    debug!("detected Notion sender: {}", sender);

    let subject_lower = subject.to_lowercase();
    let combined_text = format!(
        "{}\n{}\n{}",
        subject,
        text_body.unwrap_or(""),
        html_body.unwrap_or("")
    );

    // Determine notification type (supports both English and Chinese patterns)
    let notification_type = detect_notification_type(subject, &subject_lower);

    // Extract actor name (try English pattern first, then Chinese)
    let actor_name = MENTIONED_YOU_PATTERN
        .captures(&combined_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
        .or_else(|| {
            CHINESE_ACTOR_PATTERN
                .captures(subject)
                .and_then(|cap| cap.get(1))
                .map(|m| m.as_str().trim().to_string())
        });

    // Extract page URL and ID - first try direct URLs, then tracking URLs
    let (mut page_url, mut page_id) = extract_notion_page_info(&combined_text);

    // If no direct URL found, try to decode tracking URL
    if page_url.is_none() {
        if let Some(decoded_url) = decode_tracking_url_page_url(&combined_text) {
            debug!("Decoded page URL from tracking URL: {}", decoded_url);
            // Re-extract page info from the decoded URL
            let (decoded_page_url, decoded_page_id) = extract_notion_page_info(&decoded_url);
            page_url = decoded_page_url.or(Some(decoded_url));
            page_id = decoded_page_id.or(page_id);
        }
    }

    // Extract workspace_id from tracking URL metadata (most reliable method)
    let workspace_id = decode_tracking_url_workspace_id(&combined_text);
    if workspace_id.is_some() {
        debug!("Extracted workspace_id from tracking URL: {:?}", workspace_id);
    }

    // Extract comment URL if present
    let comment_url = NOTION_COMMENT_URL_PATTERN
        .captures(&combined_text)
        .map(|cap| cap.get(0).unwrap().as_str().to_string());

    // Extract page title from subject
    let page_title = PAGE_TITLE_PATTERN
        .captures(subject)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().trim().to_string())
        .or_else(|| extract_page_title_from_body(&combined_text));

    // Extract comment preview from text body
    let comment_preview = extract_comment_preview(text_body);

    // Extract workspace name from URL or email (fallback if workspace_id not found)
    let workspace_name = extract_workspace_name(&combined_text);

    Some(NotionEmailNotification {
        notification_type,
        actor_name,
        page_url,
        page_id,
        workspace_id,
        workspace_name,
        page_title,
        comment_preview,
        comment_url,
        subject: subject.to_string(),
    })
}

/// Extract Notion page URL and ID from text.
fn extract_notion_page_info(text: &str) -> (Option<String>, Option<String>) {
    // Try notion.so URLs - find all matches and filter out non-page paths
    for cap in NOTION_URL_PATTERN.captures_iter(text) {
        let url_str = cap.get(0).map(|m| m.as_str()).unwrap_or("");
        let path = cap.get(1).map(|m| m.as_str()).unwrap_or("");

        // Skip non-page paths (images, api, fonts, etc.)
        if NON_PAGE_PATHS.iter().any(|p| path.starts_with(p)) {
            continue;
        }

        let url = Some(url_str.to_string());
        let page_id = Some({
            // Extract just the UUID part (last 32 chars)
            if path.len() >= 32 {
                path[path.len() - 32..].to_string()
            } else {
                path.to_string()
            }
        });
        return (url, page_id);
    }

    // Try notion.site URLs
    if let Some(cap) = NOTION_SITE_URL_PATTERN.captures(text) {
        let url = cap.get(0).map(|m| m.as_str().to_string());
        let page_id = cap.get(2).map(|m| {
            let id = m.as_str();
            if id.len() > 32 && id.contains('-') {
                id.rsplit('-').next().unwrap_or(id).to_string()
            } else {
                id.to_string()
            }
        });
        return (url, page_id);
    }

    (None, None)
}

/// Extract page title from email body.
fn extract_page_title_from_body(text: &str) -> Option<String> {
    // Look for common patterns in Notion emails
    let patterns = [
        // "commented on Page Title"
        Regex::new(r"commented on\s+(.+?)(?:\n|$)").ok()?,
        // "mentioned you in Page Title"
        Regex::new(r"mentioned you in\s+(.+?)(?:\n|$)").ok()?,
    ];

    for pattern in patterns {
        if let Some(cap) = pattern.captures(text) {
            if let Some(title) = cap.get(1) {
                let title_str = title.as_str().trim();
                // Clean up the title
                if !title_str.is_empty() && title_str.len() < 200 {
                    return Some(title_str.to_string());
                }
            }
        }
    }

    None
}

/// Extract comment preview from text body.
fn extract_comment_preview(text_body: Option<&str>) -> Option<String> {
    let text = text_body?;

    // Look for quoted text or the main message body
    // Notion emails typically have the comment text after a blank line
    let lines: Vec<&str> = text.lines().collect();

    // Skip header lines and find the comment content
    let mut found_blank = false;
    let mut preview_lines = Vec::new();

    for line in lines {
        let trimmed = line.trim();

        // Skip common header/footer patterns
        if trimmed.starts_with("View in Notion")
            || trimmed.starts_with("Unsubscribe")
            || trimmed.starts_with("--")
            || trimmed.contains("notion.so")
            || trimmed.is_empty()
        {
            if !preview_lines.is_empty() {
                break;
            }
            found_blank = trimmed.is_empty();
            continue;
        }

        // Collect lines after the first blank (likely the comment content)
        if found_blank {
            preview_lines.push(trimmed);
            // Limit preview length
            if preview_lines.len() >= 5 {
                break;
            }
        }
    }

    if preview_lines.is_empty() {
        return None;
    }

    let preview = preview_lines.join("\n");
    // Truncate if too long
    if preview.len() > 500 {
        Some(format!("{}...", &preview[..500]))
    } else {
        Some(preview)
    }
}

/// Detect notification type from subject line.
/// Supports both English and Chinese patterns.
fn detect_notification_type(subject: &str, subject_lower: &str) -> NotionNotificationType {
    // English patterns
    if subject_lower.contains("mentioned you") {
        if subject_lower.contains("comment") {
            return NotionNotificationType::CommentMention;
        } else {
            return NotionNotificationType::PageMention;
        }
    }
    if subject_lower.contains("replied") {
        return NotionNotificationType::CommentReply;
    }
    if subject_lower.contains("commented") || subject_lower.contains("comment on") {
        return NotionNotificationType::PageComment;
    }

    // Chinese patterns:
    // - "X 在 Y 发表了评论" (X commented on Y)
    // - "X 在 Y 中提及了您" (X mentioned you in Y)
    // - "X 回复了您的评论" (X replied to your comment)
    if subject.contains("发表了评论") || subject.contains("评论了") {
        return NotionNotificationType::PageComment;
    }
    if subject.contains("提及了您") || subject.contains("@了您") || subject.contains("提到了你") {
        // Check if it's in a comment context
        if subject.contains("评论") {
            return NotionNotificationType::CommentMention;
        }
        return NotionNotificationType::PageMention;
    }
    if subject.contains("回复了") {
        return NotionNotificationType::CommentReply;
    }

    NotionNotificationType::Other
}

/// Extract workspace name from email content.
fn extract_workspace_name(text: &str) -> Option<String> {
    // Try to extract from notion.site URL
    if let Some(cap) = NOTION_SITE_URL_PATTERN.captures(text) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }

    // Try to extract from notion.so URL path
    // e.g., https://www.notion.so/workspacename/Page-Title-abc123
    let workspace_pattern = Regex::new(r"notion\.so/([a-zA-Z0-9_-]+)/[a-zA-Z0-9_-]").ok()?;
    if let Some(cap) = workspace_pattern.captures(text) {
        let workspace = cap.get(1)?.as_str();
        // Filter out common non-workspace paths
        if workspace != "www" && workspace.len() > 2 {
            return Some(workspace.to_string());
        }
    }

    None
}

/// Decode Notion tracking URL and extract workspace_id (space_id) from metadata.
///
/// Notion emails use tracking URLs like `https://mg.mail.notion.so/c/eJx...`
/// These contain base64url-encoded, zlib-compressed data with metadata including:
/// - `l`: the actual Notion page URL
/// - `metadata`: JSON with `space_id` (workspace_id)
///
/// Returns the space_id if successfully decoded.
fn decode_tracking_url_workspace_id(text: &str) -> Option<String> {
    // Find all tracking URLs and try to decode each
    for cap in NOTION_TRACKING_URL_PATTERN.captures_iter(text) {
        let encoded = cap.get(1)?.as_str();

        if let Some(space_id) = decode_single_tracking_url(encoded) {
            return Some(space_id);
        }
    }
    None
}

/// Decode a single tracking URL payload and extract space_id.
fn decode_single_tracking_url(encoded: &str) -> Option<String> {
    // Convert base64url to standard base64
    let encoded_std: String = encoded.chars().map(|c| match c {
        '-' => '+',
        '_' => '/',
        c => c,
    }).collect();

    // Add padding if needed
    let padding_needed = (4 - encoded_std.len() % 4) % 4;
    let encoded_padded = format!("{}{}", encoded_std, "=".repeat(padding_needed));

    // Decode base64
    let decoded_bytes = match URL_SAFE_NO_PAD.decode(&encoded_std) {
        Ok(bytes) => bytes,
        Err(_) => {
            // Try with padding
            match base64::engine::general_purpose::STANDARD.decode(&encoded_padded) {
                Ok(bytes) => bytes,
                Err(e) => {
                    warn!("Failed to decode tracking URL base64: {}", e);
                    return None;
                }
            }
        }
    };

    // Decompress with zlib
    let mut decoder = ZlibDecoder::new(&decoded_bytes[..]);
    let mut decompressed = String::new();
    if let Err(e) = decoder.read_to_string(&mut decompressed) {
        warn!("Failed to decompress tracking URL: {}", e);
        return None;
    }

    debug!("Decoded tracking URL payload: {}", decompressed);

    // Parse URL-encoded query string to find metadata
    // Format: key=value&key2=value2...
    // We're looking for: metadata={"space_id":"..."}
    for param in decompressed.split('&') {
        if param.starts_with("metadata=") {
            let metadata_encoded = &param[9..]; // Skip "metadata="
            // URL decode the metadata JSON
            if let Ok(metadata_json) = urlencoding::decode(metadata_encoded) {
                // Parse JSON to extract space_id
                if let Ok(metadata) = serde_json::from_str::<serde_json::Value>(&metadata_json) {
                    if let Some(space_id) = metadata.get("space_id").and_then(|v| v.as_str()) {
                        debug!("Extracted workspace_id (space_id) from tracking URL: {}", space_id);
                        return Some(space_id.to_string());
                    }
                }
            }
        }
    }

    None
}

/// Extract actual Notion page URL from tracking URL payload.
/// Returns the decoded page URL (the `l` parameter).
pub fn decode_tracking_url_page_url(text: &str) -> Option<String> {
    for cap in NOTION_TRACKING_URL_PATTERN.captures_iter(text) {
        let encoded = cap.get(1)?.as_str();

        if let Some(page_url) = decode_single_tracking_url_page(encoded) {
            return Some(page_url);
        }
    }
    None
}

/// Decode a single tracking URL payload and extract the actual page URL.
fn decode_single_tracking_url_page(encoded: &str) -> Option<String> {
    // Convert base64url to standard base64
    let encoded_std: String = encoded.chars().map(|c| match c {
        '-' => '+',
        '_' => '/',
        c => c,
    }).collect();

    let padding_needed = (4 - encoded_std.len() % 4) % 4;
    let encoded_padded = format!("{}{}", encoded_std, "=".repeat(padding_needed));

    let decoded_bytes = match URL_SAFE_NO_PAD.decode(&encoded_std) {
        Ok(bytes) => bytes,
        Err(_) => {
            match base64::engine::general_purpose::STANDARD.decode(&encoded_padded) {
                Ok(bytes) => bytes,
                Err(_) => return None,
            }
        }
    };

    let mut decoder = ZlibDecoder::new(&decoded_bytes[..]);
    let mut decompressed = String::new();
    if decoder.read_to_string(&mut decompressed).is_err() {
        return None;
    }

    // Find the `l` parameter (actual page URL)
    for param in decompressed.split('&') {
        if param.starts_with("l=") {
            let url_encoded = &param[2..];
            if let Ok(url) = urlencoding::decode(url_encoded) {
                return Some(url.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_notion_sender() {
        assert!(is_notion_sender("notify@mail.notion.so"));
        assert!(is_notion_sender("Notion <notify@mail.notion.so>"));
        assert!(is_notion_sender("updates@notion.so"));
        assert!(!is_notion_sender("user@example.com"));
        assert!(!is_notion_sender("notion-fake@attacker.com"));
    }

    #[test]
    fn test_detect_comment_mention() {
        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "Alice mentioned you in a comment on Project Notes",
            Some("Alice mentioned you:\n\n@Oliver can you review this section?\n\nView in Notion: https://notion.so/workspace/Project-Notes-abcdef01234567890123456789012345"),
            None,
        );

        assert!(notification.is_some());
        let n = notification.unwrap();
        assert_eq!(n.notification_type, NotionNotificationType::CommentMention);
        assert_eq!(n.actor_name, Some("Alice".to_string()));
        assert!(n.page_url.is_some());
    }

    #[test]
    fn test_detect_page_mention() {
        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "Bob mentioned you in Weekly Update",
            Some("Bob mentioned you in a page.\n\nView: https://notion.so/team/Weekly-Update-abcdef01234567890123456789012345"),
            None,
        );

        assert!(notification.is_some());
        let n = notification.unwrap();
        assert_eq!(n.notification_type, NotionNotificationType::PageMention);
    }

    #[test]
    fn test_extract_page_id_from_url() {
        let text = "Check this page: https://www.notion.so/myworkspace/My-Page-Title-abc123def456789012345678901234567890";
        let (url, page_id) = extract_notion_page_info(text);

        assert!(url.is_some());
        assert!(page_id.is_some());
    }

    #[test]
    fn test_non_notion_email() {
        let notification = detect_notion_email(
            "user@example.com",
            "Meeting Notes",
            Some("Here are the notes from today's meeting."),
            None,
        );

        assert!(notification.is_none());
    }

    #[test]
    fn test_comment_reply_detection() {
        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "Carol replied to your comment",
            Some("Carol replied to your comment on Design Doc.\n\nThanks for the feedback!"),
            None,
        );

        assert!(notification.is_some());
        let n = notification.unwrap();
        assert_eq!(n.notification_type, NotionNotificationType::CommentReply);
    }

    #[test]
    fn test_chinese_comment_detection() {
        // Test Chinese subject: "X 在 Y 发表了评论"
        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "Liu Xintong 在 Dowhiz testing 发表了评论",
            Some("Liu Xintong 在 Dowhiz testing 发表了评论\n\n@proto please check this"),
            None,
        );

        assert!(notification.is_some());
        let n = notification.unwrap();
        assert_eq!(n.notification_type, NotionNotificationType::PageComment);
        assert_eq!(n.actor_name, Some("Liu Xintong".to_string()));
    }

    #[test]
    fn test_chinese_mention_detection() {
        // Test Chinese subject: "X 在 Y 中提及了您"
        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "张三 在 项目计划 中提及了您",
            Some("张三 在 项目计划 中提及了您"),
            None,
        );

        assert!(notification.is_some());
        let n = notification.unwrap();
        assert_eq!(n.notification_type, NotionNotificationType::PageMention);
        assert_eq!(n.actor_name, Some("张三".to_string()));
    }

    #[test]
    fn test_url_excludes_image_paths() {
        // Test that image URLs are not extracted as page URLs
        let text = "Check this: https://www.notion.so/images/logo-for-slack-integration.png\nReal page: https://notion.so/workspace/My-Page-abc123def456789012345678901234567890";
        let (url, page_id) = extract_notion_page_info(text);

        assert!(url.is_some());
        let url_str = url.unwrap();
        // Should NOT match the images path
        assert!(!url_str.contains("/images/"));
        // Should match the actual page URL
        assert!(url_str.contains("My-Page"));
        assert!(page_id.is_some());
    }

    #[test]
    fn test_decode_tracking_url_workspace_id() {
        // Real tracking URL payload from Notion email (base64url + zlib compressed)
        // Contains: metadata={"space_id":"2be6a52cd8a0812a86840003b0ffbf46"}
        let encoded = "eJxMkE2OIyEMhU8Du0RgqoBesJhRKcu5QsmA6SClfhRMop7Tj5KOJr30-yzre85BueEDtKSgnTNOa-O1pAXrZe695iDcb-EmeQ44mOjGaJMpEb0lTUYNGjylXMgRyRpAgVVGO6WNBX0krW0sxRc_-BEwiUE97h7Xjeu2HtsmL-HMvDdhfgk4CTjd7_c3FXCatvu5_j0wNa7r58HoD4sjpOxReYUuGjcWyBkSAqLOwpxWYaZv-723swD7CDBxvVX-mtO2LLTyK6Zced7xk2aufCEBdr81YSYlF2LMyPhsD9B2TDTXLACeqgCR3iIa0Fs_KKVMVKXEMtjHopvkNeRnATGoTLQfuPN2PaZtkRy-JVuP_LWTMNNLjfJ_9gKN1vxn41pqwsdnZF9bjy1da6QgYPwxzv16ETDKW4B_AQAA__9RipZd";

        let text = format!("https://mg.mail.notion.so/c/{}", encoded);
        let workspace_id = decode_tracking_url_workspace_id(&text);

        assert!(workspace_id.is_some(), "Should decode workspace_id from tracking URL");
        assert_eq!(workspace_id.unwrap(), "2be6a52cd8a0812a86840003b0ffbf46");
    }

    #[test]
    fn test_decode_tracking_url_page_url() {
        // Same tracking URL payload
        let encoded = "eJxMkE2OIyEMhU8Du0RgqoBesJhRKcu5QsmA6SClfhRMop7Tj5KOJr30-yzre85BueEDtKSgnTNOa-O1pAXrZe695iDcb-EmeQ44mOjGaJMpEb0lTUYNGjylXMgRyRpAgVVGO6WNBX0krW0sxRc_-BEwiUE97h7Xjeu2HtsmL-HMvDdhfgk4CTjd7_c3FXCatvu5_j0wNa7r58HoD4sjpOxReYUuGjcWyBkSAqLOwpxWYaZv-723swD7CDBxvVX-mtO2LLTyK6Zced7xk2aufCEBdr81YSYlF2LMyPhsD9B2TDTXLACeqgCR3iIa0Fs_KKVMVKXEMtjHopvkNeRnATGoTLQfuPN2PaZtkRy-JVuP_LWTMNNLjfJ_9gKN1vxn41pqwsdnZF9bjy1da6QgYPwxzv16ETDKW4B_AQAA__9RipZd";

        let text = format!("https://mg.mail.notion.so/c/{}", encoded);
        let page_url = decode_tracking_url_page_url(&text);

        assert!(page_url.is_some(), "Should decode page URL from tracking URL");
        let url = page_url.unwrap();
        assert!(url.contains("notion.so"), "Page URL should be a notion.so URL");
        assert!(url.contains("Dowhiz-testing"), "Page URL should contain page title");
    }

    #[test]
    fn test_commented_in_pattern() {
        // Notion uses "commented in" not "commented on"
        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "Oliver commented in Dowhiz_webhook test",
            Some("Oliver commented in Dowhiz_webhook test\n\nHi there!"),
            None,
        );

        assert!(notification.is_some());
        let n = notification.unwrap();
        assert_eq!(n.notification_type, NotionNotificationType::PageComment);
        assert_eq!(n.actor_name, Some("Oliver".to_string()));
    }

    #[test]
    fn test_detect_notion_email_with_tracking_url() {
        // Test that detect_notion_email extracts workspace_id from tracking URLs
        let html_body = r#"<a href="https://mg.mail.notion.so/c/eJxMkE2OIyEMhU8Du0RgqoBesJhRKcu5QsmA6SClfhRMop7Tj5KOJr30-yzre85BueEDtKSgnTNOa-O1pAXrZe695iDcb-EmeQ44mOjGaJMpEb0lTUYNGjylXMgRyRpAgVVGO6WNBX0krW0sxRc_-BEwiUE97h7Xjeu2HtsmL-HMvDdhfgk4CTjd7_c3FXCatvu5_j0wNa7r58HoD4sjpOxReYUuGjcWyBkSAqLOwpxWYaZv-723swD7CDBxvVX-mtO2LLTyK6Zced7xk2aufCEBdr81YSYlF2LMyPhsD9B2TDTXLACeqgCR3iIa0Fs_KKVMVKXEMtjHopvkNeRnATGoTLQfuPN2PaZtkRy-JVuP_LWTMNNLjfJ_9gKN1vxn41pqwsdnZF9bjy1da6QgYPwxzv16ETDKW4B_AQAA__9RipZd">Click</a>"#;

        let notification = detect_notion_email(
            "notify@mail.notion.so",
            "Liu Xintong 在 Dowhiz testing 发表了评论",
            Some("@Proto-DoWhiz populate the page"),
            Some(html_body),
        );

        assert!(notification.is_some());
        let n = notification.unwrap();

        // Should have extracted workspace_id from tracking URL
        assert_eq!(n.workspace_id, Some("2be6a52cd8a0812a86840003b0ffbf46".to_string()));

        // Should also have decoded the page URL
        assert!(n.page_url.is_some());
        let page_url = n.page_url.unwrap();
        assert!(page_url.contains("notion.so"));
    }
}
