//! HTML parsing utilities for Notion pages.
//!
//! This module extracts structured data from Notion's HTML,
//! including notifications, page content, and comment threads.

use scraper::{Html, Selector};
use tracing::debug;

use super::models::{CommentInThread, NotionNotification, NotionPageContext};
use super::NotionError;

/// Parse notifications from the Notion notifications page HTML.
///
/// This extracts @mentions and other notifications from the HTML content
/// of the notifications page.
pub fn parse_notifications(html: &str) -> Result<Vec<NotionNotification>, NotionError> {
    let document = Html::parse_document(html);
    let mut notifications = Vec::new();

    // Notion's notification structure (selectors may need adjustment based on actual DOM)
    // These are approximations and may need to be updated when testing against real Notion
    let notification_selectors = [
        ".notion-notifications-list > div",
        "[data-notification-id]",
        ".notion-scroller [role='button']",
    ];

    for selector_str in &notification_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(notification) = parse_notification_element(&element) {
                    // Avoid duplicates
                    if !notifications.iter().any(|n: &NotionNotification| n.id == notification.id) {
                        notifications.push(notification);
                    }
                }
            }
        }
    }

    debug!("Parsed {} notifications from HTML", notifications.len());
    Ok(notifications)
}

/// Parse a single notification element.
fn parse_notification_element(element: &scraper::ElementRef) -> Option<NotionNotification> {
    // Extract notification ID from data attribute or generate from content
    let id = element
        .value()
        .attr("data-notification-id")
        .map(|s| s.to_string())
        .or_else(|| {
            // Fallback: generate ID from element content hash
            let content = element.text().collect::<String>();
            if content.is_empty() {
                None
            } else {
                Some(format!("notif_{:x}", md5_hash(&content)))
            }
        })?;

    // Extract text content
    let text_content = element.text().collect::<String>();
    let text_content = text_content.trim();

    if text_content.is_empty() {
        return None;
    }

    // Determine notification type
    let notification_type = if text_content.to_lowercase().contains("mentioned") {
        "mention"
    } else if text_content.to_lowercase().contains("commented") {
        "comment"
    } else if text_content.to_lowercase().contains("invited") {
        "invite"
    } else {
        "other"
    };

    // Extract URL from any link in the element
    let url = element
        .select(&Selector::parse("a[href]").ok()?)
        .next()
        .and_then(|a| a.value().attr("href"))
        .map(|href| {
            if href.starts_with('/') {
                format!("https://www.notion.so{}", href)
            } else {
                href.to_string()
            }
        })
        .unwrap_or_default();

    // Extract page ID from URL
    let page_id = extract_page_id_from_url(&url).unwrap_or_default();

    // Check if notification is read (usually indicated by styling)
    let is_read = element
        .value()
        .attr("class")
        .map(|c| c.contains("read") || c.contains("seen"))
        .unwrap_or(false);

    // Extract actor name (usually the first strong/bold element)
    let actor_name = element
        .select(&Selector::parse("strong, b, .notion-user-name").ok()?)
        .next()
        .map(|e| e.text().collect::<String>());

    Some(NotionNotification {
        id,
        notification_type: notification_type.to_string(),
        workspace_id: None,
        workspace_name: None,
        page_id,
        block_id: None,
        actor_id: None,
        actor_name,
        preview_text: Some(text_content.to_string()),
        url,
        created_at: None,
        is_read,
    })
}

/// Parse page content for context extraction.
pub fn parse_page_content(
    html: &str,
    page_id: &str,
    url: &str,
) -> Result<NotionPageContext, NotionError> {
    let document = Html::parse_document(html);

    // Extract page title
    let title = extract_page_title(&document).unwrap_or_else(|| "Untitled".to_string());

    // Extract text content
    let content_text = extract_page_text(&document);

    // Extract comment thread if present
    let comment_thread = extract_comment_thread(&document);

    Ok(NotionPageContext {
        title,
        page_id: page_id.to_string(),
        url: url.to_string(),
        content_text,
        parent_page_id: None,
        database_id: None,
        comment_thread,
    })
}

/// Extract the page title from Notion HTML.
fn extract_page_title(document: &Html) -> Option<String> {
    // Try various selectors for the page title
    let title_selectors = [
        ".notion-page-block .notion-title",
        "[data-content-editable-leaf][data-block-id] > div:first-child",
        "h1",
        ".notion-header-block",
        "title",
    ];

    for selector_str in &title_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let text = element.text().collect::<String>();
                let text = text.trim();
                if !text.is_empty() && text.len() < 500 {
                    return Some(text.to_string());
                }
            }
        }
    }

    None
}

/// Extract text content from the page.
fn extract_page_text(document: &Html) -> String {
    let mut text_parts = Vec::new();

    // Select content blocks
    let content_selectors = [
        ".notion-page-content [data-block-id]",
        ".notion-text-block",
        ".notion-bulleted_list-block",
        ".notion-numbered_list-block",
        ".notion-to_do-block",
        ".notion-code-block",
        "p",
    ];

    for selector_str in &content_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                let text = element.text().collect::<String>();
                let text = text.trim();
                if !text.is_empty() {
                    text_parts.push(text.to_string());
                }
            }
        }
    }

    // Deduplicate and join
    let mut seen = std::collections::HashSet::new();
    text_parts
        .into_iter()
        .filter(|t| seen.insert(t.clone()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Extract comment thread from the page.
fn extract_comment_thread(document: &Html) -> Vec<CommentInThread> {
    let mut comments = Vec::new();

    // Comment selectors (may need adjustment)
    let comment_selectors = [
        ".notion-comment",
        "[data-comment-id]",
        ".notion-discussion-thread .notion-comment-block",
    ];

    for selector_str in &comment_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                // Extract author
                let author_name = element
                    .select(&Selector::parse(".notion-user-name, .comment-author, strong").ok().unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_else(|| "Unknown".to_string());

                // Extract comment text
                let text = element
                    .select(&Selector::parse(".comment-text, .notion-comment-content, p").ok().unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_else(|| element.text().collect::<String>());

                let text = text.trim();
                if !text.is_empty() {
                    comments.push(CommentInThread {
                        author_name: author_name.trim().to_string(),
                        author_id: None,
                        text: text.to_string(),
                        created_at: None,
                    });
                }
            }
        }
    }

    comments
}

/// Extract page ID from a Notion URL.
fn extract_page_id_from_url(url: &str) -> Option<String> {
    // Notion URLs can be in formats like:
    // - https://www.notion.so/workspace/Page-Title-abc123def456
    // - https://www.notion.so/abc123def456
    // - https://notion.so/workspace/Page-Title-abc123def456?v=...

    let url = url.trim_end_matches('/');

    // Try to extract the last segment that looks like an ID
    if let Some(last_segment) = url.split('/').last() {
        // Remove query params
        let segment = last_segment.split('?').next().unwrap_or(last_segment);

        // Notion IDs are 32 hex chars, sometimes with dashes
        // They can be at the end of the page title
        if let Some(id_part) = segment.split('-').last() {
            if id_part.len() == 32 && id_part.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(id_part.to_string());
            }
        }

        // Or the whole segment could be the ID
        let clean_segment = segment.replace('-', "");
        if clean_segment.len() == 32 && clean_segment.chars().all(|c| c.is_ascii_hexdigit()) {
            return Some(clean_segment);
        }
    }

    None
}

/// Simple hash function for generating IDs.
fn md5_hash(input: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_page_id_from_url() {
        assert_eq!(
            extract_page_id_from_url("https://www.notion.so/workspace/My-Page-abc123def456789012345678901234ab"),
            Some("abc123def456789012345678901234ab".to_string())
        );

        assert_eq!(
            extract_page_id_from_url("https://www.notion.so/abc123def456789012345678901234ab"),
            Some("abc123def456789012345678901234ab".to_string())
        );
    }
}
