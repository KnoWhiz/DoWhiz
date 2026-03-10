//! Notion API client for page and comment operations.
//!
//! Provides methods to:
//! - Read page content and blocks
//! - Read comments on pages/blocks
//! - Reply to comment threads
//! - Search for pages
//!
//! Uses OAuth tokens stored in NotionOAuthStore.

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use super::oauth_store::NotionOAuthStore;
use super::NotionError;

const NOTION_API_BASE: &str = "https://api.notion.com/v1";
const NOTION_API_VERSION: &str = "2022-06-28";

/// Notion API client.
pub struct NotionApiClient {
    http_client: Client,
    oauth_store: NotionOAuthStore,
    employee_id: String,
}

/// Error types specific to the Notion API.
#[derive(Debug, thiserror::Error)]
pub enum NotionApiError {
    #[error("No authorization for workspace {0}")]
    NoAuthorization(String),

    #[error("API request failed: {0}")]
    RequestFailed(String),

    #[error("Rate limited, retry after {0} seconds")]
    RateLimited(u64),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl From<NotionApiError> for NotionError {
    fn from(e: NotionApiError) -> Self {
        NotionError::BrowserError(e.to_string())
    }
}

/// A page retrieved from the Notion API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPage {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub cover: Option<String>,
    pub created_time: String,
    pub last_edited_time: String,
}

/// A block from a Notion page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionBlock {
    pub id: String,
    pub block_type: String,
    pub has_children: bool,
    #[serde(default)]
    pub text_content: Option<String>,
}

/// A comment on a Notion page or block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionComment {
    pub id: String,
    pub discussion_id: String,
    pub parent_id: String,
    pub created_by: CommentUser,
    pub created_time: String,
    pub rich_text: Vec<RichTextItem>,
}

impl NotionComment {
    /// Get the plain text content of the comment.
    pub fn plain_text(&self) -> String {
        self.rich_text
            .iter()
            .map(|item| item.plain_text.as_str())
            .collect::<Vec<_>>()
            .join("")
    }
}

/// User who created a comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUser {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
}

/// Rich text item in a comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextItem {
    pub plain_text: String,
    #[serde(default)]
    pub href: Option<String>,
}

/// Page content including blocks.
#[derive(Debug, Clone)]
pub struct PageContent {
    pub page: NotionPage,
    pub blocks: Vec<NotionBlock>,
}

impl NotionApiClient {
    /// Create a new API client.
    pub fn new(oauth_store: NotionOAuthStore, employee_id: &str) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            oauth_store,
            employee_id: employee_id.to_string(),
        }
    }

    /// Create a client from environment configuration.
    pub fn from_env(employee_id: &str) -> Result<Self, NotionError> {
        let oauth_store = NotionOAuthStore::from_env(employee_id)?;
        Ok(Self::new(oauth_store, employee_id))
    }

    /// Build headers for API requests.
    fn build_headers(&self, access_token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "Notion-Version",
            HeaderValue::from_static(NOTION_API_VERSION),
        );
        headers
    }

    /// Get access token for a workspace.
    fn get_token(&self, workspace_id: &str) -> Result<String, NotionApiError> {
        self.oauth_store
            .get_token(workspace_id, &self.employee_id)
            .map_err(|e| NotionApiError::RequestFailed(e.to_string()))?
            .ok_or_else(|| NotionApiError::NoAuthorization(workspace_id.to_string()))
    }

    /// Make an API GET request.
    fn api_get(&self, workspace_id: &str, endpoint: &str) -> Result<Value, NotionApiError> {
        let token = self.get_token(workspace_id)?;
        let url = format!("{}{}", NOTION_API_BASE, endpoint);
        let headers = self.build_headers(&token);

        debug!("Notion API GET: {}", url);

        let response = self
            .http_client
            .get(&url)
            .headers(headers)
            .send()
            .map_err(|e| NotionApiError::RequestFailed(e.to_string()))?;

        self.handle_response(response)
    }

    /// Make an API POST request.
    fn api_post(&self, workspace_id: &str, endpoint: &str, body: &Value) -> Result<Value, NotionApiError> {
        let token = self.get_token(workspace_id)?;
        let url = format!("{}{}", NOTION_API_BASE, endpoint);
        let headers = self.build_headers(&token);

        debug!("Notion API POST: {}", url);

        let response = self
            .http_client
            .post(&url)
            .headers(headers)
            .json(body)
            .send()
            .map_err(|e| NotionApiError::RequestFailed(e.to_string()))?;

        self.handle_response(response)
    }

    /// Handle API response.
    fn handle_response(&self, response: reqwest::blocking::Response) -> Result<Value, NotionApiError> {
        let status = response.status();
        let body = response
            .text()
            .map_err(|e| NotionApiError::RequestFailed(e.to_string()))?;

        if status.is_success() {
            serde_json::from_str(&body)
                .map_err(|e| NotionApiError::InvalidResponse(format!("JSON parse error: {}", e)))
        } else if status.as_u16() == 429 {
            // Rate limited
            let retry_after = 60; // Default to 60 seconds
            warn!("Notion API rate limited, retry after {} seconds", retry_after);
            Err(NotionApiError::RateLimited(retry_after))
        } else if status.as_u16() == 404 {
            Err(NotionApiError::NotFound(body))
        } else if status.as_u16() == 403 {
            Err(NotionApiError::PermissionDenied(body))
        } else {
            error!("Notion API error {}: {}", status, body);
            Err(NotionApiError::RequestFailed(format!(
                "Status {}: {}",
                status, body
            )))
        }
    }

    /// Get page metadata.
    pub fn get_page(&self, workspace_id: &str, page_id: &str) -> Result<NotionPage, NotionApiError> {
        let data = self.api_get(workspace_id, &format!("/pages/{}", page_id))?;

        let title = extract_page_title(&data);
        let url = data["url"].as_str().unwrap_or("").to_string();

        Ok(NotionPage {
            id: page_id.to_string(),
            title,
            url,
            icon: data["icon"]["emoji"].as_str().map(|s| s.to_string()),
            cover: data["cover"]["external"]["url"]
                .as_str()
                .map(|s| s.to_string()),
            created_time: data["created_time"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            last_edited_time: data["last_edited_time"]
                .as_str()
                .unwrap_or("")
                .to_string(),
        })
    }

    /// Get page blocks (content).
    pub fn get_page_blocks(
        &self,
        workspace_id: &str,
        page_id: &str,
    ) -> Result<Vec<NotionBlock>, NotionApiError> {
        let data = self.api_get(workspace_id, &format!("/blocks/{}/children", page_id))?;

        let mut blocks = Vec::new();
        if let Some(results) = data["results"].as_array() {
            for block in results {
                let block_type = block["type"].as_str().unwrap_or("unknown").to_string();
                let text_content = extract_block_text(block, &block_type);

                blocks.push(NotionBlock {
                    id: block["id"].as_str().unwrap_or("").to_string(),
                    block_type,
                    has_children: block["has_children"].as_bool().unwrap_or(false),
                    text_content,
                });
            }
        }

        Ok(blocks)
    }

    /// Get full page content (metadata + blocks).
    pub fn get_page_content(
        &self,
        workspace_id: &str,
        page_id: &str,
    ) -> Result<PageContent, NotionApiError> {
        let page = self.get_page(workspace_id, page_id)?;
        let blocks = self.get_page_blocks(workspace_id, page_id)?;

        Ok(PageContent { page, blocks })
    }

    /// Get comments on a page or block.
    pub fn get_comments(
        &self,
        workspace_id: &str,
        block_id: &str,
    ) -> Result<Vec<NotionComment>, NotionApiError> {
        let data = self.api_get(
            workspace_id,
            &format!("/comments?block_id={}", block_id),
        )?;

        let mut comments = Vec::new();
        if let Some(results) = data["results"].as_array() {
            for comment in results {
                if let Some(parsed) = parse_comment(comment) {
                    comments.push(parsed);
                }
            }
        }

        Ok(comments)
    }

    /// Reply to a comment thread (discussion).
    pub fn reply_to_comment(
        &self,
        workspace_id: &str,
        discussion_id: &str,
        content: &str,
    ) -> Result<NotionComment, NotionApiError> {
        let body = serde_json::json!({
            "discussion_id": discussion_id,
            "rich_text": [{
                "type": "text",
                "text": {
                    "content": content
                }
            }]
        });

        let data = self.api_post(workspace_id, "/comments", &body)?;

        parse_comment(&data).ok_or_else(|| {
            NotionApiError::InvalidResponse("Failed to parse created comment".to_string())
        })
    }

    /// Create a new comment on a page.
    pub fn create_comment(
        &self,
        workspace_id: &str,
        page_id: &str,
        content: &str,
    ) -> Result<NotionComment, NotionApiError> {
        let body = serde_json::json!({
            "parent": {
                "page_id": page_id
            },
            "rich_text": [{
                "type": "text",
                "text": {
                    "content": content
                }
            }]
        });

        let data = self.api_post(workspace_id, "/comments", &body)?;

        parse_comment(&data).ok_or_else(|| {
            NotionApiError::InvalidResponse("Failed to parse created comment".to_string())
        })
    }

    /// Search for pages in a workspace.
    pub fn search_pages(
        &self,
        workspace_id: &str,
        query: &str,
    ) -> Result<Vec<NotionPage>, NotionApiError> {
        let body = serde_json::json!({
            "query": query,
            "filter": {
                "value": "page",
                "property": "object"
            }
        });

        let data = self.api_post(workspace_id, "/search", &body)?;

        let mut pages = Vec::new();
        if let Some(results) = data["results"].as_array() {
            for page_data in results {
                let title = extract_page_title(page_data);
                pages.push(NotionPage {
                    id: page_data["id"].as_str().unwrap_or("").to_string(),
                    title,
                    url: page_data["url"].as_str().unwrap_or("").to_string(),
                    icon: page_data["icon"]["emoji"].as_str().map(|s| s.to_string()),
                    cover: None,
                    created_time: page_data["created_time"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    last_edited_time: page_data["last_edited_time"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }

        Ok(pages)
    }

    /// Check if we have API access to a workspace.
    pub fn has_access(&self, workspace_id: &str) -> bool {
        self.oauth_store
            .has_token(workspace_id, &self.employee_id)
            .unwrap_or(false)
    }
}

/// Extract page title from Notion API response.
fn extract_page_title(page_data: &Value) -> String {
    // Try "title" property first (database pages)
    if let Some(properties) = page_data["properties"].as_object() {
        for (_, prop) in properties {
            if prop["type"].as_str() == Some("title") {
                if let Some(title_arr) = prop["title"].as_array() {
                    if let Some(first) = title_arr.first() {
                        if let Some(text) = first["plain_text"].as_str() {
                            return text.to_string();
                        }
                    }
                }
            }
        }
    }

    // Fallback to "Name" property
    if let Some(title_arr) = page_data["properties"]["Name"]["title"].as_array() {
        if let Some(first) = title_arr.first() {
            if let Some(text) = first["plain_text"].as_str() {
                return text.to_string();
            }
        }
    }

    "Untitled".to_string()
}

/// Extract text content from a block.
fn extract_block_text(block: &Value, block_type: &str) -> Option<String> {
    let rich_text_key = match block_type {
        "paragraph" | "heading_1" | "heading_2" | "heading_3" | "bulleted_list_item"
        | "numbered_list_item" | "quote" | "callout" | "toggle" => "rich_text",
        "code" => "rich_text",
        "to_do" => "rich_text",
        _ => return None,
    };

    let rich_text = block[block_type][rich_text_key].as_array()?;
    let text: String = rich_text
        .iter()
        .filter_map(|item| item["plain_text"].as_str())
        .collect();

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// Parse a comment from API response.
fn parse_comment(data: &Value) -> Option<NotionComment> {
    let rich_text = data["rich_text"]
        .as_array()?
        .iter()
        .filter_map(|item| {
            Some(RichTextItem {
                plain_text: item["plain_text"].as_str()?.to_string(),
                href: item["href"].as_str().map(|s| s.to_string()),
            })
        })
        .collect();

    Some(NotionComment {
        id: data["id"].as_str()?.to_string(),
        discussion_id: data["discussion_id"].as_str()?.to_string(),
        parent_id: data["parent"]["page_id"]
            .as_str()
            .or_else(|| data["parent"]["block_id"].as_str())?
            .to_string(),
        created_by: CommentUser {
            id: data["created_by"]["id"].as_str()?.to_string(),
            name: data["created_by"]["name"].as_str().map(|s| s.to_string()),
            avatar_url: data["created_by"]["avatar_url"]
                .as_str()
                .map(|s| s.to_string()),
        },
        created_time: data["created_time"].as_str()?.to_string(),
        rich_text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_page_title_from_title_property() {
        let data = serde_json::json!({
            "properties": {
                "Name": {
                    "type": "title",
                    "title": [{
                        "plain_text": "Test Page"
                    }]
                }
            }
        });

        assert_eq!(extract_page_title(&data), "Test Page");
    }

    #[test]
    fn test_extract_page_title_fallback() {
        let data = serde_json::json!({
            "properties": {}
        });

        assert_eq!(extract_page_title(&data), "Untitled");
    }

    #[test]
    fn test_parse_comment() {
        let data = serde_json::json!({
            "id": "comment-123",
            "discussion_id": "disc-456",
            "parent": {
                "page_id": "page-789"
            },
            "created_by": {
                "id": "user-111",
                "name": "Test User"
            },
            "created_time": "2024-01-15T10:00:00.000Z",
            "rich_text": [{
                "plain_text": "Hello, this is a comment"
            }]
        });

        let comment = parse_comment(&data).unwrap();
        assert_eq!(comment.id, "comment-123");
        assert_eq!(comment.discussion_id, "disc-456");
        assert_eq!(comment.plain_text(), "Hello, this is a comment");
    }
}
