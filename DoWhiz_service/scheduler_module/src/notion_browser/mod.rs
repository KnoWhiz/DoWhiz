//! Notion browser automation module for monitoring @mentions and comments.
//!
//! This module provides browser-based automation for Notion integration,
//! allowing the digital employee to:
//! - Monitor notifications for @mentions
//! - Read page content and comment threads
//! - Reply to comments as a real user
//!
//! Uses browser-use CLI for browser automation with anti-detection measures.

pub mod browser;
pub mod models;
pub mod parser;
pub mod poller;
pub mod store;

pub use browser::{BrowserState, NotionBrowser, NotionBrowserConfig};
pub use models::{NotionMention, NotionNotification, NotionPageContext};
pub use parser::{parse_notifications, parse_page_content};
pub use poller::{spawn_notion_browser_poller, NotionBrowserPoller, NotionPollerConfig};
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

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
