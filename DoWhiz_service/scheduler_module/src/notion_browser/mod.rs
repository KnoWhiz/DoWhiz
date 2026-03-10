//! Notion browser automation module for monitoring @mentions and comments.
//!
//! This module provides browser-based automation for Notion integration,
//! allowing the digital employee to:
//! - Monitor notifications for @mentions
//! - Read page content and comment threads
//! - Reply to comments as a real user
//!
//! Uses browser-use CLI for browser automation with anti-detection measures.
//!
//! ## Detection Modes
//!
//! The poller supports two detection modes (set via `NOTION_DETECTION_MODE` env var):
//! - `agent_driven` (default): Uses LLM to analyze inbox screenshots for robustness
//! - `hardcoded`: Uses regex patterns to parse browser state (legacy fallback)

pub mod agent_detector;
pub mod api_client;
pub mod browser;
pub mod models;
pub mod oauth_store;
pub mod operations;
pub mod parser;
pub mod poller;
pub mod store;

pub use agent_detector::{AgentDetector, AgentDetectorConfig, DetectedMention};
pub use api_client::{NotionApiClient, NotionApiError, NotionBlock, NotionComment, NotionPage, PageContent};
pub use browser::{BrowserState, NotionBrowser, NotionBrowserConfig};
pub use models::{NotionMention, NotionNotification, NotionPageContext};
pub use oauth_store::{NotionOAuthStore, NotionOAuthToken};
pub use operations::{NotionOperations, PageReadResult};
pub use parser::{parse_notifications, parse_page_content};
pub use poller::{spawn_notion_browser_poller, DetectionMode, NotionBrowserPoller, NotionPollerConfig};
pub use store::MongoNotionProcessedStore;

/// Errors that can occur during Notion browser operations.
#[derive(Debug, thiserror::Error)]
pub enum NotionError {
    #[error("Browser error: {0}")]
    BrowserError(String),

    #[error("Navigation error: {0}")]
    NavigationError(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Login failed: {0}")]
    LoginFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Queue error: {0}")]
    QueueError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
