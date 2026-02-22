//! Common functionality shared across Google Workspace adapters (Docs, Sheets, Slides).
//!
//! This module provides:
//! - Shared data models for comments and file metadata
//! - Common comments API operations (list, filter, reply)
//! - File type detection and routing

mod comments;
mod models;
mod types;

pub use comments::{filter_actionable_comments, GoogleCommentsClient};
pub use models::{
    ActionableComment, CommentAuthor, CommentReply, CommentsListResponse, DriveFile,
    DriveFileOwner, FilesListResponse, GoogleComment, QuotedFileContent,
};
pub use types::{GoogleFileType, GOOGLE_DOCS_MIME, GOOGLE_SHEETS_MIME, GOOGLE_SLIDES_MIME};
