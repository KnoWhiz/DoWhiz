//! Notion API integration module.
//!
//! This module provides API-based integration with Notion, allowing the
//! digital employee to:
//! - Read page content and comments via Notion API
//! - Reply to comments and create new comments
//! - Store and manage OAuth tokens for multi-workspace access
//!
//! The API-based approach is faster and more reliable than browser automation.

pub mod api_client;
pub mod models;
pub mod oauth_store;
pub mod store;

pub use api_client::{NotionApiClient, NotionApiError, NotionBlock, NotionComment, NotionPage, PageContent};
pub use models::{NotionMention, NotionNotification, NotionPageContext};
pub use oauth_store::{NotionOAuthStore, NotionOAuthToken};
pub use store::MongoNotionProcessedStore;

/// Errors that can occur during Notion API operations.
#[derive(Debug, thiserror::Error)]
pub enum NotionError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
