//! Data models for Notion browser integration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A notification from Notion (parsed from the notifications page).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionNotification {
    /// Unique notification ID
    pub id: String,
    /// Type of notification (e.g., "mention", "comment", "invite")
    pub notification_type: String,
    /// Workspace ID where the notification originated
    pub workspace_id: Option<String>,
    /// Workspace name
    pub workspace_name: Option<String>,
    /// Page ID where the notification originated
    pub page_id: String,
    /// Block ID (if applicable)
    pub block_id: Option<String>,
    /// Actor who triggered the notification (user ID)
    pub actor_id: Option<String>,
    /// Actor's display name
    pub actor_name: Option<String>,
    /// Preview text of the notification
    pub preview_text: Option<String>,
    /// URL to the notification target
    pub url: String,
    /// When the notification was created
    pub created_at: Option<DateTime<Utc>>,
    /// Whether the notification has been read
    pub is_read: bool,
}

/// A mention detected in Notion (processed notification ready for handling).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionMention {
    /// Unique ID for this mention (derived from notification ID)
    pub id: String,
    /// Workspace ID
    pub workspace_id: String,
    /// Workspace name
    pub workspace_name: String,
    /// Page ID where the mention occurred
    pub page_id: String,
    /// Page title
    pub page_title: String,
    /// Block ID where the mention is located
    pub block_id: Option<String>,
    /// Comment ID (if in a comment thread)
    pub comment_id: Option<String>,
    /// Name of the user who mentioned us
    pub sender_name: String,
    /// User ID of the sender
    pub sender_id: Option<String>,
    /// The comment/text containing the mention
    pub comment_text: String,
    /// Previous comments in the thread (for context)
    pub thread_context: Vec<CommentInThread>,
    /// Direct URL to the comment/mention
    pub url: String,
    /// When we detected this mention
    pub detected_at: DateTime<Utc>,
}

/// A comment within a thread (for context).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentInThread {
    /// Comment author name
    pub author_name: String,
    /// Comment author ID
    pub author_id: Option<String>,
    /// Comment text content
    pub text: String,
    /// When the comment was created
    pub created_at: Option<DateTime<Utc>>,
}

/// Context extracted from a Notion page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPageContext {
    /// Page title
    pub title: String,
    /// Page ID
    pub page_id: String,
    /// Page URL
    pub url: String,
    /// Text content of the page (for agent context)
    pub content_text: String,
    /// Parent page ID (if nested)
    pub parent_page_id: Option<String>,
    /// Database ID (if page is in a database)
    pub database_id: Option<String>,
    /// Comments on the page/block
    pub comment_thread: Vec<CommentInThread>,
}

/// Configuration for Notion session/credentials.
#[derive(Debug, Clone)]
pub struct NotionSessionConfig {
    /// Email for Notion login
    pub email: String,
    /// Password for Notion login
    pub password: String,
    /// Profile directory for session persistence
    pub profile_dir: std::path::PathBuf,
}

/// Result of posting a reply via browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionReplyResult {
    /// Whether the reply was successfully posted
    pub success: bool,
    /// The comment ID of the posted reply (if available)
    pub comment_id: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}
