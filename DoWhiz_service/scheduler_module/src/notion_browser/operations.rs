//! Hybrid Notion operations layer.
//!
//! Provides a unified interface for Notion operations that:
//! - Prefers API when OAuth token is available (faster, more reliable)
//! - Falls back to browser automation when no API access
//!
//! This allows gradual migration from browser-only to API-first architecture.

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use super::api_client::{NotionApiClient, NotionApiError, NotionComment, NotionPage, PageContent};
use super::browser::NotionBrowser;
use super::NotionError;

/// Hybrid operations layer for Notion.
///
/// Automatically chooses between API and browser based on availability.
pub struct NotionOperations {
    api_client: NotionApiClient,
    browser: Arc<Mutex<NotionBrowser>>,
}

/// Result of reading a page, from either API or browser.
#[derive(Debug, Clone)]
pub struct PageReadResult {
    pub title: String,
    pub content: String,
    pub url: String,
    /// Whether the result came from API (true) or browser (false)
    pub from_api: bool,
}

impl NotionOperations {
    /// Create a new operations layer.
    pub fn new(api_client: NotionApiClient, browser: NotionBrowser) -> Self {
        Self {
            api_client,
            browser: Arc::new(Mutex::new(browser)),
        }
    }

    /// Read page content - API first, browser fallback.
    pub async fn read_page(
        &self,
        workspace_id: &str,
        page_id: &str,
    ) -> Result<PageReadResult, NotionError> {
        // Try API first if we have access
        if self.api_client.has_access(workspace_id) {
            match self.api_client.get_page_content(workspace_id, page_id) {
                Ok(content) => {
                    info!("Read page {} via API", page_id);
                    let text_content = blocks_to_text(&content);
                    return Ok(PageReadResult {
                        title: content.page.title,
                        content: text_content,
                        url: content.page.url,
                        from_api: true,
                    });
                }
                Err(NotionApiError::NoAuthorization(_)) => {
                    debug!("No API token, falling back to browser");
                }
                Err(NotionApiError::PermissionDenied(msg)) => {
                    warn!("API permission denied for page {}: {}", page_id, msg);
                    // Fall through to browser
                }
                Err(e) => {
                    warn!("API error reading page {}: {}, trying browser", page_id, e);
                }
            }
        }

        // Fall back to browser
        info!("Reading page {} via browser", page_id);
        let mut browser = self.browser.lock().await;
        let page_url = format!("https://www.notion.so/{}", page_id.replace('-', ""));

        browser
            .navigate(&page_url)
            .await
            .map_err(|e| NotionError::NavigationError(e.to_string()))?;

        let state = browser.get_state().await?;

        // Extract title and content from browser state
        let title = extract_title_from_state(&state.raw);
        let content = extract_content_from_state(&state.raw);

        Ok(PageReadResult {
            title,
            content,
            url: page_url,
            from_api: false,
        })
    }

    /// Get comments on a page - API only (browser doesn't support this well).
    pub fn get_comments(
        &self,
        workspace_id: &str,
        page_id: &str,
    ) -> Result<Vec<NotionComment>, NotionError> {
        if !self.api_client.has_access(workspace_id) {
            return Err(NotionError::ConfigError(
                "API access required for comments".to_string(),
            ));
        }

        self.api_client
            .get_comments(workspace_id, page_id)
            .map_err(|e| e.into())
    }

    /// Reply to a comment - API first, browser fallback.
    pub async fn reply_to_comment(
        &self,
        workspace_id: &str,
        discussion_id: &str,
        content: &str,
    ) -> Result<(), NotionError> {
        // Try API first
        if self.api_client.has_access(workspace_id) {
            match self.api_client.reply_to_comment(workspace_id, discussion_id, content) {
                Ok(_) => {
                    info!("Replied to discussion {} via API", discussion_id);
                    return Ok(());
                }
                Err(NotionApiError::NoAuthorization(_)) => {
                    debug!("No API token, falling back to browser");
                }
                Err(e) => {
                    warn!("API error replying to comment: {}, trying browser", e);
                }
            }
        }

        // Browser fallback for comment replies is not yet implemented.
        // This would require complex UI automation to:
        // 1. Navigate to the page containing the discussion
        // 2. Find and click the specific comment thread
        // 3. Type in the reply box
        // 4. Submit the comment
        //
        // For now, require API access for comment operations.
        warn!(
            "Browser comment reply not implemented, API access required for discussion {}",
            discussion_id
        );
        Err(NotionError::ConfigError(
            "Browser comment reply not yet implemented. Please set up Notion OAuth.".to_string(),
        ))
    }

    /// Create a new comment on a page - API only.
    pub fn create_comment(
        &self,
        workspace_id: &str,
        page_id: &str,
        content: &str,
    ) -> Result<NotionComment, NotionError> {
        if !self.api_client.has_access(workspace_id) {
            return Err(NotionError::ConfigError(
                "API access required to create comments".to_string(),
            ));
        }

        self.api_client
            .create_comment(workspace_id, page_id, content)
            .map_err(|e| e.into())
    }

    /// Search for pages - API only.
    pub fn search_pages(
        &self,
        workspace_id: &str,
        query: &str,
    ) -> Result<Vec<NotionPage>, NotionError> {
        if !self.api_client.has_access(workspace_id) {
            return Err(NotionError::ConfigError(
                "API access required for search".to_string(),
            ));
        }

        self.api_client
            .search_pages(workspace_id, query)
            .map_err(|e| e.into())
    }

    /// Check if we have API access to a workspace.
    pub fn has_api_access(&self, workspace_id: &str) -> bool {
        self.api_client.has_access(workspace_id)
    }
}

/// Convert page blocks to plain text.
fn blocks_to_text(content: &PageContent) -> String {
    content
        .blocks
        .iter()
        .filter_map(|block| block.text_content.as_ref())
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
}

/// Extract page title from browser state.
fn extract_title_from_state(raw_state: &str) -> String {
    // Look for heading elements or page title
    for line in raw_state.lines() {
        if line.contains("<h1") || line.contains("heading_1") {
            // Extract text after the element
            if let Some(idx) = line.find('>') {
                let text = &line[idx + 1..];
                let text = text.trim();
                if !text.is_empty() {
                    return text.to_string();
                }
            }
        }
    }

    // Fallback: look for any prominent text at the top
    for line in raw_state.lines().take(20) {
        let trimmed = line.trim();
        if !trimmed.is_empty()
            && !trimmed.starts_with('[')
            && !trimmed.starts_with("viewport")
            && !trimmed.starts_with("url")
        {
            return trimmed.to_string();
        }
    }

    "Untitled".to_string()
}

/// Extract page content from browser state.
fn extract_content_from_state(raw_state: &str) -> String {
    let mut content = Vec::new();

    for line in raw_state.lines() {
        let trimmed = line.trim();

        // Skip metadata lines
        if trimmed.starts_with("viewport")
            || trimmed.starts_with("url")
            || trimmed.is_empty()
        {
            continue;
        }

        // Extract text from element lines
        if trimmed.starts_with('[') {
            // Format: [123]<element>text or [123]<element />\n    text
            if let Some(close_bracket) = trimmed.find(']') {
                if let Some(close_tag) = trimmed.find('>') {
                    let text = &trimmed[close_tag + 1..];
                    let text = text.trim();
                    if !text.is_empty() && text != "/" {
                        content.push(text.to_string());
                    }
                }
            }
        } else if !trimmed.starts_with('<') {
            // Plain text on indented lines
            content.push(trimmed.to_string());
        }
    }

    content.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title_from_state() {
        let state = r#"
viewport: 1920x1080
url: https://www.notion.so/my-page
[1]<h1 />
    My Page Title
[2]<p />
    Some content here
"#;

        let title = extract_title_from_state(state);
        assert_eq!(title, "My Page Title");
    }

    #[test]
    fn test_extract_content_from_state() {
        let state = r#"
viewport: 1920x1080
url: https://www.notion.so/my-page
[1]<h1 />
    Title
[2]<p />
    First paragraph
[3]<p />
    Second paragraph
"#;

        let content = extract_content_from_state(state);
        assert!(content.contains("Title"));
        assert!(content.contains("First paragraph"));
        assert!(content.contains("Second paragraph"));
    }
}
