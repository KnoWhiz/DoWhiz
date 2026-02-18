use serde::{Deserialize, Serialize};

/// Google Docs comment from the Drive API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleDocsComment {
    /// The comment ID
    pub id: String,
    /// Plain text content of the comment
    pub content: String,
    /// HTML content of the comment (if available)
    #[serde(rename = "htmlContent")]
    pub html_content: Option<String>,
    /// Whether the comment is resolved
    pub resolved: Option<bool>,
    /// Author information
    pub author: Option<CommentAuthor>,
    /// Creation time (RFC3339)
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    /// Modification time (RFC3339)
    #[serde(rename = "modifiedTime")]
    pub modified_time: Option<String>,
    /// Replies to this comment
    pub replies: Option<Vec<CommentReply>>,
    /// Anchor information (position in document)
    pub anchor: Option<String>,
    /// Quoted text from the document
    #[serde(rename = "quotedFileContent")]
    pub quoted_file_content: Option<QuotedFileContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentAuthor {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "photoLink")]
    pub photo_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    pub id: String,
    pub content: String,
    #[serde(rename = "htmlContent")]
    pub html_content: Option<String>,
    pub author: Option<CommentAuthor>,
    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotedFileContent {
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub value: Option<String>,
}

/// Represents an actionable item from Google Docs (either a comment or a reply).
/// This structure helps track whether a parent comment or a specific reply triggered the action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionableComment {
    /// The parent comment
    pub comment: GoogleDocsComment,
    /// If this is a reply, the specific reply that triggered the action
    pub triggering_reply: Option<CommentReply>,
    /// The unique ID for tracking (either "comment:{id}" or "comment:{id}:reply:{reply_id}")
    pub tracking_id: String,
}

impl ActionableComment {
    /// Create an actionable item for a parent comment.
    pub fn from_comment(comment: GoogleDocsComment) -> Self {
        let tracking_id = format!("comment:{}", comment.id);
        Self {
            comment,
            triggering_reply: None,
            tracking_id,
        }
    }

    /// Create an actionable item for a reply.
    pub fn from_reply(comment: GoogleDocsComment, reply: CommentReply) -> Self {
        let tracking_id = format!("comment:{}:reply:{}", comment.id, reply.id);
        Self {
            triggering_reply: Some(reply),
            comment,
            tracking_id,
        }
    }

    /// Get the content that triggered this action (either comment content or reply content).
    pub fn triggering_content(&self) -> &str {
        self.triggering_reply
            .as_ref()
            .map(|r| r.content.as_str())
            .unwrap_or(&self.comment.content)
    }

    /// Get the author of the triggering content.
    pub fn triggering_author(&self) -> Option<&CommentAuthor> {
        self.triggering_reply
            .as_ref()
            .and_then(|r| r.author.as_ref())
            .or(self.comment.author.as_ref())
    }
}

/// Response from Google Drive API comments.list
#[derive(Debug, Clone, Deserialize)]
pub struct CommentsListResponse {
    pub comments: Option<Vec<GoogleDocsComment>>,
    #[allow(dead_code)]
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

/// Google Drive file metadata
#[derive(Debug, Clone, Deserialize)]
pub struct DriveFile {
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub owners: Option<Vec<DriveFileOwner>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DriveFileOwner {
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

/// Response from Google Drive API files.list
#[derive(Debug, Clone, Deserialize)]
pub struct FilesListResponse {
    pub files: Option<Vec<DriveFile>>,
    #[allow(dead_code)]
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}
