//! Google Workspace (Docs/Sheets/Slides) comment polling service.
//!
//! This module provides a background service that polls for Google Docs, Sheets and Slides
//! comments mentioning the digital employee and creates tasks to handle them.

use chrono::Utc;
use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

use crate::adapters::google_common::{
    ActionableComment, DriveFile, GoogleCommentsClient, GoogleFileType,
};
use crate::adapters::google_docs::contains_employee_mention;
use crate::adapters::google_sheets::GoogleSheetsInboundAdapter;
use crate::adapters::google_slides::GoogleSlidesInboundAdapter;
use crate::channel::{Channel, InboundMessage};
use crate::google_auth::{GoogleAuth, GoogleAuthConfig};
use crate::SchedulerError;

/// Default TTL for file list cache (5 minutes).
const FILE_LIST_CACHE_TTL_SECS: u64 = 300;

/// Cache entry for file lists.
struct FileListCacheEntry {
    files: Vec<DriveFile>,
    fetched_at: Instant,
}

/// Thread-safe cache for file lists.
/// Reduces redundant API calls when file list hasn't changed.
pub struct FileListCache {
    docs: Mutex<Option<FileListCacheEntry>>,
    sheets: Mutex<Option<FileListCacheEntry>>,
    slides: Mutex<Option<FileListCacheEntry>>,
    ttl: Duration,
}

impl FileListCache {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            docs: Mutex::new(None),
            sheets: Mutex::new(None),
            slides: Mutex::new(None),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    /// Get cached docs file list if not expired.
    pub fn get_docs(&self) -> Option<Vec<DriveFile>> {
        let cache = self.docs.lock().ok()?;
        if let Some(ref entry) = *cache {
            if entry.fetched_at.elapsed() < self.ttl {
                return Some(entry.files.clone());
            }
        }
        None
    }

    /// Get cached sheets file list if not expired.
    pub fn get_sheets(&self) -> Option<Vec<DriveFile>> {
        let cache = self.sheets.lock().ok()?;
        if let Some(ref entry) = *cache {
            if entry.fetched_at.elapsed() < self.ttl {
                return Some(entry.files.clone());
            }
        }
        None
    }

    /// Get cached slides file list if not expired.
    pub fn get_slides(&self) -> Option<Vec<DriveFile>> {
        let cache = self.slides.lock().ok()?;
        if let Some(ref entry) = *cache {
            if entry.fetched_at.elapsed() < self.ttl {
                return Some(entry.files.clone());
            }
        }
        None
    }

    /// Update docs file list cache.
    pub fn set_docs(&self, files: Vec<DriveFile>) {
        if let Ok(mut cache) = self.docs.lock() {
            *cache = Some(FileListCacheEntry {
                files,
                fetched_at: Instant::now(),
            });
        }
    }

    /// Update sheets file list cache.
    pub fn set_sheets(&self, files: Vec<DriveFile>) {
        if let Ok(mut cache) = self.sheets.lock() {
            *cache = Some(FileListCacheEntry {
                files,
                fetched_at: Instant::now(),
            });
        }
    }

    /// Update slides file list cache.
    pub fn set_slides(&self, files: Vec<DriveFile>) {
        if let Ok(mut cache) = self.slides.lock() {
            *cache = Some(FileListCacheEntry {
                files,
                fetched_at: Instant::now(),
            });
        }
    }

    /// Invalidate all caches (force refresh on next poll).
    #[allow(dead_code)]
    pub fn invalidate_all(&self) {
        if let Ok(mut cache) = self.docs.lock() {
            *cache = None;
        }
        if let Ok(mut cache) = self.sheets.lock() {
            *cache = None;
        }
        if let Ok(mut cache) = self.slides.lock() {
            *cache = None;
        }
    }
}

/// Type of Google Workspace file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceFileType {
    Docs,
    Sheets,
    Slides,
}

impl WorkspaceFileType {
    pub fn channel(&self) -> Channel {
        match self {
            WorkspaceFileType::Docs => Channel::GoogleDocs,
            WorkspaceFileType::Sheets => Channel::GoogleSheets,
            WorkspaceFileType::Slides => Channel::GoogleSlides,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WorkspaceFileType::Docs => "docs",
            WorkspaceFileType::Sheets => "sheets",
            WorkspaceFileType::Slides => "slides",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            WorkspaceFileType::Docs => "Google Docs",
            WorkspaceFileType::Sheets => "Google Sheets",
            WorkspaceFileType::Slides => "Google Slides",
        }
    }

    /// Returns true if this file type supports Google Drive push notifications (files.watch).
    /// Note: Google Slides does NOT support files.watch (returns 403 Forbidden).
    pub fn supports_push_notifications(&self) -> bool {
        match self {
            WorkspaceFileType::Docs => true,
            WorkspaceFileType::Sheets => true,
            WorkspaceFileType::Slides => false, // Google API limitation
        }
    }
}

/// Configuration for Google Workspace polling.
#[derive(Debug, Clone)]
pub struct GoogleWorkspacePollerConfig {
    /// Poll interval in seconds (default: 30)
    pub poll_interval_secs: u64,
    /// Whether Google Docs integration is enabled
    pub docs_enabled: bool,
    /// Whether Google Sheets integration is enabled
    pub sheets_enabled: bool,
    /// Whether Google Slides integration is enabled
    pub slides_enabled: bool,
    /// Employee email addresses (to identify our own replies)
    pub employee_emails: HashSet<String>,
    /// Root directory for workspaces
    pub workspace_root: PathBuf,
    /// Path to the processed comments database
    pub processed_db_path: PathBuf,
    /// Employee ID (e.g., "proto")
    pub employee_id: String,
}

impl Default for GoogleWorkspacePollerConfig {
    fn default() -> Self {
        Self {
            // Reduced from 30s to 15s for faster comment detection
            poll_interval_secs: 15,
            docs_enabled: false,
            sheets_enabled: false,
            slides_enabled: false,
            employee_emails: HashSet::new(),
            workspace_root: PathBuf::from("workspaces"),
            processed_db_path: PathBuf::from("google_workspace_processed.db"),
            employee_id: "proto".to_string(),
        }
    }
}

impl GoogleWorkspacePollerConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let docs_enabled = std::env::var("GOOGLE_DOCS_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let sheets_enabled = std::env::var("GOOGLE_SHEETS_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let slides_enabled = std::env::var("GOOGLE_SLIDES_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        // Default reduced from 30s to 15s for faster comment detection
        let poll_interval_secs = std::env::var("GOOGLE_WORKSPACE_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(15);

        let mut employee_emails = HashSet::new();
        employee_emails.insert("oliver@dowhiz.com".to_string());
        employee_emails.insert("proto@dowhiz.com".to_string());
        employee_emails.insert("maggie@dowhiz.com".to_string());
        employee_emails.insert("little-bear@dowhiz.com".to_string());

        // Add additional employee emails from environment variable (comma-separated).
        // This allows adding the actual Google account email used for OAuth to prevent
        // the polling from treating our own replies as actionable messages.
        if let Ok(extra_emails) = std::env::var("GOOGLE_EMPLOYEE_EMAILS") {
            for email in extra_emails.split(',') {
                let trimmed = email.trim().to_lowercase();
                if !trimmed.is_empty() && trimmed.contains('@') {
                    employee_emails.insert(trimmed);
                }
            }
        }

        let workspace_root = std::env::var("WORKSPACE_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".dowhiz")
                    .join("DoWhiz")
                    .join("run_task")
            });

        let processed_db_path = workspace_root.join("google_workspace_processed.db");

        let employee_id = std::env::var("EMPLOYEE_ID").unwrap_or_else(|_| "proto".to_string());

        Self {
            poll_interval_secs,
            docs_enabled,
            sheets_enabled,
            slides_enabled,
            employee_emails,
            workspace_root,
            processed_db_path,
            employee_id,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.sheets_enabled || self.slides_enabled
    }
}

/// Database schema for tracking processed comments.
const WORKSPACE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS google_workspace_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id TEXT UNIQUE NOT NULL,
    file_name TEXT,
    file_type TEXT NOT NULL,
    owner_email TEXT,
    last_checked_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS google_workspace_processed_comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id TEXT NOT NULL,
    comment_id TEXT NOT NULL,
    file_type TEXT NOT NULL,
    processed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(file_id, comment_id)
);

CREATE INDEX IF NOT EXISTS idx_workspace_processed_file
ON google_workspace_processed_comments(file_id);
"#;

/// Store for tracking processed Google Workspace comments.
#[derive(Debug)]
pub struct GoogleWorkspaceProcessedStore {
    path: PathBuf,
}

impl GoogleWorkspaceProcessedStore {
    pub fn new(path: PathBuf) -> Result<Self, SchedulerError> {
        let store = Self { path };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> Result<(), SchedulerError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.execute_batch(WORKSPACE_SCHEMA)?;
        Ok(())
    }

    fn open(&self) -> Result<Connection, SchedulerError> {
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        Ok(conn)
    }

    /// Get all processed tracking IDs for a file.
    pub fn get_processed_ids(&self, file_id: &str) -> Result<HashSet<String>, SchedulerError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT comment_id FROM google_workspace_processed_comments WHERE file_id = ?1",
        )?;
        let rows = stmt.query_map(params![file_id], |row| row.get::<_, String>(0))?;
        let mut result = HashSet::new();
        for row in rows {
            result.insert(row?);
        }
        Ok(result)
    }

    /// Mark a tracking ID as processed.
    pub fn mark_processed_id(
        &self,
        file_id: &str,
        tracking_id: &str,
        file_type: WorkspaceFileType,
    ) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "INSERT OR IGNORE INTO google_workspace_processed_comments (file_id, comment_id, file_type, processed_at) VALUES (?1, ?2, ?3, ?4)",
            params![file_id, tracking_id, file_type.name(), Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Register a file for tracking.
    pub fn register_file(
        &self,
        file_id: &str,
        file_name: Option<&str>,
        file_type: WorkspaceFileType,
        owner_email: Option<&str>,
    ) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "INSERT OR REPLACE INTO google_workspace_files (file_id, file_name, file_type, owner_email, last_checked_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, COALESCE((SELECT created_at FROM google_workspace_files WHERE file_id = ?1), ?5))",
            params![file_id, file_name, file_type.name(), owner_email, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Update last checked time for a file.
    pub fn update_last_checked(&self, file_id: &str) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE google_workspace_files SET last_checked_at = ?1 WHERE file_id = ?2",
            params![Utc::now().to_rfc3339(), file_id],
        )?;
        Ok(())
    }
}

/// Google Workspace polling service.
pub struct GoogleWorkspacePoller {
    config: GoogleWorkspacePollerConfig,
    auth: GoogleAuth,
    store: GoogleWorkspaceProcessedStore,
    file_cache: FileListCache,
}

impl GoogleWorkspacePoller {
    /// Create a new poller from configuration.
    pub fn new(config: GoogleWorkspacePollerConfig) -> Result<Self, SchedulerError> {
        let auth_config = GoogleAuthConfig::from_env_for_employee(Some(&config.employee_id));
        if !auth_config.is_valid() {
            return Err(SchedulerError::TaskFailed(
                "Google OAuth credentials not configured".to_string(),
            ));
        }

        let auth = GoogleAuth::new(auth_config)
            .map_err(|e| SchedulerError::TaskFailed(format!("Google auth failed: {}", e)))?;

        let store = GoogleWorkspaceProcessedStore::new(config.processed_db_path.clone())?;

        // Create file list cache with 5 minute TTL
        let file_cache = FileListCache::new(FILE_LIST_CACHE_TTL_SECS);

        Ok(Self {
            config,
            auth,
            store,
            file_cache,
        })
    }

    pub fn auth(&self) -> &GoogleAuth {
        &self.auth
    }

    pub fn config(&self) -> &GoogleWorkspacePollerConfig {
        &self.config
    }

    pub fn store(&self) -> &GoogleWorkspaceProcessedStore {
        &self.store
    }

    /// Poll Google Sheets for comments.
    pub fn poll_sheets(&self) -> Result<Vec<(DriveFile, Vec<ActionableComment>)>, SchedulerError> {
        if !self.config.sheets_enabled {
            return Ok(vec![]);
        }

        let adapter = GoogleSheetsInboundAdapter::new(
            self.auth.clone(),
            self.config.employee_emails.clone(),
        );

        // Try to use cached file list first
        let files = if let Some(cached_files) = self.file_cache.get_sheets() {
            debug!("Using cached sheets file list ({} files)", cached_files.len());
            cached_files
        } else {
            let fetched_files = adapter
                .list_shared_spreadsheets()
                .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list spreadsheets: {}", e)))?;
            info!("Fetched {} shared spreadsheets (cache miss)", fetched_files.len());
            self.file_cache.set_sheets(fetched_files.clone());
            fetched_files
        };

        debug!("Found {} shared spreadsheets", files.len());

        let mut results = vec![];

        for file in files {
            let owner_email = file
                .owners
                .as_ref()
                .and_then(|owners| owners.first())
                .and_then(|o| o.email_address.as_deref());

            self.store.register_file(
                &file.id,
                file.name.as_deref(),
                WorkspaceFileType::Sheets,
                owner_email,
            )?;

            let comments = match adapter.list_comments(&file.id) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to list comments for spreadsheet {}: {}", file.id, e);
                    continue;
                }
            };

            let processed = self.store.get_processed_ids(&file.id)?;
            let actionable = adapter.filter_actionable_comments(&comments, &processed);

            if !actionable.is_empty() {
                results.push((file, actionable));
            }
        }

        Ok(results)
    }

    /// Poll Google Slides for comments.
    pub fn poll_slides(&self) -> Result<Vec<(DriveFile, Vec<ActionableComment>)>, SchedulerError> {
        if !self.config.slides_enabled {
            return Ok(vec![]);
        }

        let adapter = GoogleSlidesInboundAdapter::new(
            self.auth.clone(),
            self.config.employee_emails.clone(),
        );

        // Try to use cached file list first
        let files = if let Some(cached_files) = self.file_cache.get_slides() {
            debug!("Using cached slides file list ({} files)", cached_files.len());
            cached_files
        } else {
            let fetched_files = adapter
                .list_shared_presentations()
                .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list presentations: {}", e)))?;
            info!("Fetched {} shared presentations (cache miss)", fetched_files.len());
            self.file_cache.set_slides(fetched_files.clone());
            fetched_files
        };

        debug!("Found {} shared presentations", files.len());

        let mut results = vec![];

        for file in files {
            let owner_email = file
                .owners
                .as_ref()
                .and_then(|owners| owners.first())
                .and_then(|o| o.email_address.as_deref());

            self.store.register_file(
                &file.id,
                file.name.as_deref(),
                WorkspaceFileType::Slides,
                owner_email,
            )?;

            let comments = match adapter.list_comments(&file.id) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to list comments for presentation {}: {}", file.id, e);
                    continue;
                }
            };

            let processed = self.store.get_processed_ids(&file.id)?;
            let actionable = adapter.filter_actionable_comments(&comments, &processed);

            if !actionable.is_empty() {
                results.push((file, actionable));
            }
        }

        Ok(results)
    }

    /// Poll Google Docs for comments.
    pub fn poll_docs(&self) -> Result<Vec<(DriveFile, Vec<ActionableComment>)>, SchedulerError> {
        if !self.config.docs_enabled {
            return Ok(vec![]);
        }

        // Use GoogleCommentsClient directly (same as Sheets/Slides adapters do internally)
        let client = GoogleCommentsClient::new(
            self.auth.clone(),
            self.config.employee_emails.clone(),
            contains_employee_mention,
        );

        // Try to use cached file list first
        let files = if let Some(cached_files) = self.file_cache.get_docs() {
            debug!("Using cached docs file list ({} files)", cached_files.len());
            cached_files
        } else {
            let fetched_files = client
                .list_shared_files()
                .map_err(|e| {
                    SchedulerError::TaskFailed(format!("Failed to list documents: {}", e))
                })?
                .into_iter()
                .filter(|f| f.file_type() == GoogleFileType::Docs)
                .collect::<Vec<_>>();
            info!("Fetched {} shared documents (cache miss)", fetched_files.len());
            self.file_cache.set_docs(fetched_files.clone());
            fetched_files
        };

        debug!("Found {} shared documents", files.len());

        let mut results = vec![];

        for file in files {
            let owner_email = file
                .owners
                .as_ref()
                .and_then(|owners| owners.first())
                .and_then(|o| o.email_address.as_deref());

            self.store.register_file(
                &file.id,
                file.name.as_deref(),
                WorkspaceFileType::Docs,
                owner_email,
            )?;

            let comments = match client.list_comments(&file.id) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to list comments for document {}: {}", file.id, e);
                    continue;
                }
            };

            let processed = self.store.get_processed_ids(&file.id)?;
            let actionable = client.filter_actionable_comments(&comments, &processed);

            if !actionable.is_empty() {
                results.push((file, actionable));
            }
        }

        Ok(results)
    }

    /// Convert an actionable comment to an inbound message.
    pub fn actionable_to_inbound_message(
        &self,
        file: &DriveFile,
        actionable: &ActionableComment,
        file_type: WorkspaceFileType,
    ) -> InboundMessage {
        let file_name = file.name.as_deref().unwrap_or("Untitled");
        let owner_email = file
            .owners
            .as_ref()
            .and_then(|owners| owners.first())
            .and_then(|o| o.email_address.clone());

        match file_type {
            WorkspaceFileType::Docs => {
                // Build InboundMessage for Docs using the common ActionableComment type
                self.build_docs_inbound_message(&file.id, file_name, actionable, owner_email.as_deref())
            }
            WorkspaceFileType::Sheets => {
                let adapter = GoogleSheetsInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                adapter.actionable_to_inbound_message(&file.id, file_name, actionable)
            }
            WorkspaceFileType::Slides => {
                let adapter = GoogleSlidesInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                adapter.actionable_to_inbound_message(&file.id, file_name, actionable)
            }
        }
    }

    /// Build InboundMessage for Google Docs from a common ActionableComment.
    fn build_docs_inbound_message(
        &self,
        document_id: &str,
        document_name: &str,
        actionable: &ActionableComment,
        owner_email: Option<&str>,
    ) -> InboundMessage {
        use crate::channel::ChannelMetadata;

        let sender = actionable
            .triggering_author()
            .and_then(|a| a.email_address.clone())
            .unwrap_or_else(|| "unknown@unknown.com".to_string());

        let sender_name = actionable
            .triggering_author()
            .and_then(|a| a.display_name.clone());

        let mut text_body = actionable.triggering_content().to_string();

        if let Some(ref reply) = actionable.triggering_reply {
            let original_content = &actionable.comment.content;
            let original_author = actionable
                .comment
                .author
                .as_ref()
                .and_then(|a| a.display_name.as_deref())
                .unwrap_or("Someone");

            text_body = format!(
                "Original comment by {}: \"{}\"\n\nReply: {}",
                original_author, original_content, reply.content
            );
        }

        if let Some(ref quoted) = actionable.comment.quoted_file_content {
            if let Some(ref value) = quoted.value {
                text_body = format!("Quoted content: \"{}\"\n\n{}", value, text_body);
            }
        }

        let thread_id = format!("{}:{}", document_id, actionable.comment.id);
        let reply_to = vec![sender.clone()];

        let message_id = if let Some(ref reply) = actionable.triggering_reply {
            format!("{}:{}", actionable.comment.id, reply.id)
        } else {
            actionable.comment.id.clone()
        };

        InboundMessage {
            channel: Channel::GoogleDocs,
            sender,
            sender_name,
            recipient: "oliver@dowhiz.com".to_string(),
            subject: Some(format!("Comment on: {}", document_name)),
            text_body: Some(text_body),
            html_body: actionable
                .triggering_reply
                .as_ref()
                .and_then(|r| r.html_content.clone())
                .or_else(|| actionable.comment.html_content.clone()),
            thread_id,
            message_id: Some(message_id),
            attachments: vec![],
            reply_to,
            raw_payload: serde_json::to_vec(&actionable.comment).unwrap_or_default(),
            metadata: ChannelMetadata {
                google_docs_document_id: Some(document_id.to_string()),
                google_docs_comment_id: Some(actionable.comment.id.clone()),
                google_docs_document_name: Some(document_name.to_string()),
                google_docs_owner_email: owner_email.map(|s| s.to_string()),
                ..Default::default()
            },
        }
    }

    /// Read file content for agent context.
    pub fn read_file_content(
        &self,
        file_id: &str,
        file_type: WorkspaceFileType,
    ) -> Result<String, SchedulerError> {
        match file_type {
            WorkspaceFileType::Docs => {
                // Use GoogleCommentsClient to export document as plain text
                let client = GoogleCommentsClient::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                    contains_employee_mention,
                );
                client
                    .export_file_content(file_id, "text/plain")
                    .map_err(|e| SchedulerError::TaskFailed(e.to_string()))
            }
            WorkspaceFileType::Sheets => {
                let adapter = GoogleSheetsInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                adapter
                    .read_spreadsheet_content(file_id)
                    .map_err(|e| SchedulerError::TaskFailed(e.to_string()))
            }
            WorkspaceFileType::Slides => {
                let adapter = GoogleSlidesInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                adapter
                    .read_presentation_content(file_id)
                    .map_err(|e| SchedulerError::TaskFailed(e.to_string()))
            }
        }
    }

    /// List all monitored files of a given type.
    /// Used by push notification system to register watch channels.
    pub fn list_files(&self, file_type: WorkspaceFileType) -> Result<Vec<DriveFile>, SchedulerError> {
        match file_type {
            WorkspaceFileType::Docs => {
                if let Some(cached) = self.file_cache.get_docs() {
                    return Ok(cached);
                }
                let client = GoogleCommentsClient::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                    contains_employee_mention,
                );
                let files = client
                    .list_shared_files()
                    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list documents: {}", e)))?
                    .into_iter()
                    .filter(|f| f.file_type() == GoogleFileType::Docs)
                    .collect::<Vec<_>>();
                self.file_cache.set_docs(files.clone());
                Ok(files)
            }
            WorkspaceFileType::Sheets => {
                if let Some(cached) = self.file_cache.get_sheets() {
                    return Ok(cached);
                }
                let adapter = GoogleSheetsInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                let files = adapter
                    .list_shared_spreadsheets()
                    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list spreadsheets: {}", e)))?;
                self.file_cache.set_sheets(files.clone());
                Ok(files)
            }
            WorkspaceFileType::Slides => {
                if let Some(cached) = self.file_cache.get_slides() {
                    return Ok(cached);
                }
                let adapter = GoogleSlidesInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                let files = adapter
                    .list_shared_presentations()
                    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list presentations: {}", e)))?;
                self.file_cache.set_slides(files.clone());
                Ok(files)
            }
        }
    }

    /// Poll a single file for comments (used by push notifications for immediate response).
    pub fn poll_single_file(
        &self,
        file_id: &str,
        file_type: WorkspaceFileType,
    ) -> Result<Vec<(DriveFile, Vec<ActionableComment>)>, SchedulerError> {
        // Get file metadata from cache or fetch
        let files = self.list_files(file_type)?;
        let file = files.into_iter().find(|f| f.id == file_id);

        let Some(file) = file else {
            debug!("File {} not found in {} file list", file_id, file_type.display_name());
            return Ok(vec![]);
        };

        // Get comments for this specific file
        let comments = match file_type {
            WorkspaceFileType::Docs => {
                let client = GoogleCommentsClient::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                    contains_employee_mention,
                );
                let comments = client
                    .list_comments(file_id)
                    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list comments: {}", e)))?;
                let processed = self.store.get_processed_ids(file_id)?;
                client.filter_actionable_comments(&comments, &processed)
            }
            WorkspaceFileType::Sheets => {
                let adapter = GoogleSheetsInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                let comments = adapter
                    .list_comments(file_id)
                    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list comments: {}", e)))?;
                let processed = self.store.get_processed_ids(file_id)?;
                adapter.filter_actionable_comments(&comments, &processed)
            }
            WorkspaceFileType::Slides => {
                let adapter = GoogleSlidesInboundAdapter::new(
                    self.auth.clone(),
                    self.config.employee_emails.clone(),
                );
                let comments = adapter
                    .list_comments(file_id)
                    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list comments: {}", e)))?;
                let processed = self.store.get_processed_ids(file_id)?;
                adapter.filter_actionable_comments(&comments, &processed)
            }
        };

        if comments.is_empty() {
            Ok(vec![])
        } else {
            Ok(vec![(file, comments)])
        }
    }
}
