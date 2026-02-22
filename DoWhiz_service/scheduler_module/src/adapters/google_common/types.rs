//! Type definitions for Google Workspace file types.

use serde::{Deserialize, Serialize};

/// MIME type for Google Docs
pub const GOOGLE_DOCS_MIME: &str = "application/vnd.google-apps.document";

/// MIME type for Google Sheets
pub const GOOGLE_SHEETS_MIME: &str = "application/vnd.google-apps.spreadsheet";

/// MIME type for Google Slides
pub const GOOGLE_SLIDES_MIME: &str = "application/vnd.google-apps.presentation";

/// Google Workspace file types that we support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoogleFileType {
    /// Google Docs document
    Docs,
    /// Google Sheets spreadsheet
    Sheets,
    /// Google Slides presentation
    Slides,
    /// Unknown or unsupported file type
    Unknown,
}

impl GoogleFileType {
    /// Create from MIME type string.
    pub fn from_mime_type(mime_type: &str) -> Self {
        match mime_type {
            GOOGLE_DOCS_MIME => Self::Docs,
            GOOGLE_SHEETS_MIME => Self::Sheets,
            GOOGLE_SLIDES_MIME => Self::Slides,
            _ => Self::Unknown,
        }
    }

    /// Get the MIME type for this file type.
    pub fn mime_type(&self) -> Option<&'static str> {
        match self {
            Self::Docs => Some(GOOGLE_DOCS_MIME),
            Self::Sheets => Some(GOOGLE_SHEETS_MIME),
            Self::Slides => Some(GOOGLE_SLIDES_MIME),
            Self::Unknown => None,
        }
    }

    /// Get the export MIME type for reading content as text.
    pub fn export_mime_type(&self) -> Option<&'static str> {
        match self {
            Self::Docs => Some("text/plain"),
            Self::Sheets => Some("text/csv"),
            Self::Slides => Some("text/plain"),
            Self::Unknown => None,
        }
    }

    /// Get a human-readable display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Docs => "Google Docs",
            Self::Sheets => "Google Sheets",
            Self::Slides => "Google Slides",
            Self::Unknown => "Unknown",
        }
    }

    /// Check if this file type supports comments via Drive API.
    pub fn supports_comments(&self) -> bool {
        matches!(self, Self::Docs | Self::Sheets | Self::Slides)
    }
}

impl std::fmt::Display for GoogleFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
