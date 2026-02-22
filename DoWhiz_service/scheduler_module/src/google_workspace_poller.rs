//! Google Workspace (Sheets/Slides) comment polling service.
//!
//! This module provides a background service that polls for Google Sheets and Slides comments
//! mentioning the digital employee and creates tasks to handle them.

use chrono::Utc;
use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, warn};

use crate::adapters::google_common::{ActionableComment, DriveFile};
use crate::adapters::google_sheets::GoogleSheetsInboundAdapter;
use crate::adapters::google_slides::GoogleSlidesInboundAdapter;
use crate::channel::{Channel, InboundMessage};
use crate::google_auth::{GoogleAuth, GoogleAuthConfig};
use crate::SchedulerError;

/// Type of Google Workspace file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceFileType {
    Sheets,
    Slides,
}

impl WorkspaceFileType {
    pub fn channel(&self) -> Channel {
        match self {
            WorkspaceFileType::Sheets => Channel::GoogleSheets,
            WorkspaceFileType::Slides => Channel::GoogleSlides,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WorkspaceFileType::Sheets => "sheets",
            WorkspaceFileType::Slides => "slides",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            WorkspaceFileType::Sheets => "Google Sheets",
            WorkspaceFileType::Slides => "Google Slides",
        }
    }
}

/// Configuration for Google Workspace polling.
#[derive(Debug, Clone)]
pub struct GoogleWorkspacePollerConfig {
    /// Poll interval in seconds (default: 30)
    pub poll_interval_secs: u64,
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
            poll_interval_secs: 30,
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

        let sheets_enabled = std::env::var("GOOGLE_SHEETS_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let slides_enabled = std::env::var("GOOGLE_SLIDES_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let poll_interval_secs = std::env::var("GOOGLE_WORKSPACE_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

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

        Ok(Self {
            config,
            auth,
            store,
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

        let files = adapter
            .list_shared_spreadsheets()
            .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list spreadsheets: {}", e)))?;

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

        let files = adapter
            .list_shared_presentations()
            .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list presentations: {}", e)))?;

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

    /// Convert an actionable comment to an inbound message.
    pub fn actionable_to_inbound_message(
        &self,
        file: &DriveFile,
        actionable: &ActionableComment,
        file_type: WorkspaceFileType,
    ) -> InboundMessage {
        let file_name = file.name.as_deref().unwrap_or("Untitled");

        match file_type {
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

    /// Read file content for agent context.
    pub fn read_file_content(
        &self,
        file_id: &str,
        file_type: WorkspaceFileType,
    ) -> Result<String, SchedulerError> {
        match file_type {
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
}
