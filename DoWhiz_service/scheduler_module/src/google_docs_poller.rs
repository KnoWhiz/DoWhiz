//! Google Docs comment polling service.
//!
//! This module provides a background service that polls for Google Docs comments
//! mentioning the digital employee and creates tasks to handle them.

use chrono::Utc;
use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use crate::adapters::google_docs::{ActionableComment, GoogleDocsInboundAdapter};
use crate::channel::Channel;
use crate::google_auth::{GoogleAuth, GoogleAuthConfig};
use crate::{RunTaskTask, Scheduler, SchedulerError, TaskExecutor, TaskKind};

/// Configuration for Google Docs polling.
#[derive(Debug, Clone)]
pub struct GoogleDocsPollerConfig {
    /// Poll interval in seconds (default: 30)
    pub poll_interval_secs: u64,
    /// Whether Google Docs integration is enabled
    pub enabled: bool,
    /// Employee email addresses (to identify our own replies)
    pub employee_emails: HashSet<String>,
    /// Root directory for workspaces
    pub workspace_root: PathBuf,
    /// Path to the processed comments database
    pub processed_db_path: PathBuf,
    /// Employee ID (e.g., "little_bear")
    pub employee_id: String,
    /// Model name for task execution
    pub model_name: String,
    /// Runner type (e.g., "codex" or "claude")
    pub runner: String,
}

impl Default for GoogleDocsPollerConfig {
    fn default() -> Self {
        Self {
            poll_interval_secs: 30,
            enabled: false,
            employee_emails: HashSet::new(),
            workspace_root: PathBuf::from("workspaces"),
            processed_db_path: PathBuf::from("google_docs_processed.db"),
            employee_id: "little_bear".to_string(),
            model_name: "gpt-5.2-codex".to_string(),
            runner: "codex".to_string(),
        }
    }
}

impl GoogleDocsPollerConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let enabled = std::env::var("GOOGLE_DOCS_ENABLED")
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(false);

        let poll_interval_secs = std::env::var("GOOGLE_DOCS_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        let mut employee_emails = HashSet::new();
        // Add default employee emails
        employee_emails.insert("oliver@dowhiz.com".to_string());
        employee_emails.insert("maggie@dowhiz.com".to_string());
        employee_emails.insert("little-bear@dowhiz.com".to_string());
        employee_emails.insert("mini-mouse@dowhiz.com".to_string());

        let workspace_root = std::env::var("WORKSPACE_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join(".dowhiz")
                    .join("DoWhiz")
                    .join("run_task")
            });

        let processed_db_path = workspace_root.join("google_docs_processed.db");

        let employee_id =
            std::env::var("EMPLOYEE_ID").unwrap_or_else(|_| "little_bear".to_string());

        let model_name =
            std::env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.2-codex".to_string());

        let runner = if std::env::var("CLAUDE_MODEL").is_ok() {
            "claude".to_string()
        } else {
            "codex".to_string()
        };

        Self {
            poll_interval_secs,
            enabled,
            employee_emails,
            workspace_root,
            processed_db_path,
            employee_id,
            model_name,
            runner,
        }
    }
}

/// Database schema for tracking processed comments.
const GOOGLE_DOCS_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS google_docs_documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT UNIQUE NOT NULL,
    document_name TEXT,
    owner_email TEXT,
    last_checked_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS google_docs_processed_comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    comment_id TEXT NOT NULL,
    processed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(document_id, comment_id)
);

CREATE INDEX IF NOT EXISTS idx_processed_comments_document
ON google_docs_processed_comments(document_id);
"#;

/// Store for tracking processed Google Docs comments.
#[derive(Debug)]
pub struct GoogleDocsProcessedStore {
    path: PathBuf,
}

impl GoogleDocsProcessedStore {
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
        conn.execute_batch(GOOGLE_DOCS_SCHEMA)?;
        Ok(())
    }

    fn open(&self) -> Result<Connection, SchedulerError> {
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        Ok(conn)
    }

    /// Check if a comment has been processed.
    pub fn is_processed(
        &self,
        document_id: &str,
        comment_id: &str,
    ) -> Result<bool, SchedulerError> {
        let conn = self.open()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM google_docs_processed_comments WHERE document_id = ?1 AND comment_id = ?2",
            params![document_id, comment_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Mark a comment as processed.
    pub fn mark_processed(
        &self,
        document_id: &str,
        comment_id: &str,
    ) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "INSERT OR IGNORE INTO google_docs_processed_comments (document_id, comment_id, processed_at) VALUES (?1, ?2, ?3)",
            params![document_id, comment_id, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Get all processed comment IDs for a document.
    /// @deprecated Use get_processed_ids instead for new tracking_id format.
    pub fn get_processed_comments(
        &self,
        document_id: &str,
    ) -> Result<HashSet<String>, SchedulerError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT comment_id FROM google_docs_processed_comments WHERE document_id = ?1",
        )?;
        let rows = stmt.query_map(params![document_id], |row| row.get::<_, String>(0))?;
        let mut result = HashSet::new();
        for row in rows {
            result.insert(row?);
        }
        Ok(result)
    }

    /// Get all processed tracking IDs for a document.
    /// Tracking IDs can be "comment:{id}" or "comment:{id}:reply:{reply_id}".
    pub fn get_processed_ids(&self, document_id: &str) -> Result<HashSet<String>, SchedulerError> {
        // Same implementation, but named for clarity
        self.get_processed_comments(document_id)
    }

    /// Mark a tracking ID as processed.
    /// Tracking IDs can be "comment:{id}" or "comment:{id}:reply:{reply_id}".
    pub fn mark_processed_id(
        &self,
        document_id: &str,
        tracking_id: &str,
    ) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "INSERT OR IGNORE INTO google_docs_processed_comments (document_id, comment_id, processed_at) VALUES (?1, ?2, ?3)",
            params![document_id, tracking_id, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Register a document for tracking.
    pub fn register_document(
        &self,
        document_id: &str,
        document_name: Option<&str>,
        owner_email: Option<&str>,
    ) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "INSERT OR REPLACE INTO google_docs_documents (document_id, document_name, owner_email, last_checked_at, created_at)
             VALUES (?1, ?2, ?3, ?4, COALESCE((SELECT created_at FROM google_docs_documents WHERE document_id = ?1), ?4))",
            params![document_id, document_name, owner_email, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Update last checked time for a document.
    pub fn update_last_checked(&self, document_id: &str) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE google_docs_documents SET last_checked_at = ?1 WHERE document_id = ?2",
            params![Utc::now().to_rfc3339(), document_id],
        )?;
        Ok(())
    }
}

/// Google Docs polling service.
pub struct GoogleDocsPoller {
    config: GoogleDocsPollerConfig,
    auth: GoogleAuth,
    store: GoogleDocsProcessedStore,
}

impl GoogleDocsPoller {
    /// Create a new poller from configuration.
    pub fn new(config: GoogleDocsPollerConfig) -> Result<Self, SchedulerError> {
        // Use employee-specific OAuth credentials (e.g., GOOGLE_REFRESH_TOKEN_BOILED_EGG for employee "boiled_egg")
        let auth_config = GoogleAuthConfig::from_env_for_employee(Some(&config.employee_id));
        if !auth_config.is_valid() {
            return Err(SchedulerError::TaskFailed(
                "Google OAuth credentials not configured".to_string(),
            ));
        }

        let auth = GoogleAuth::new(auth_config)
            .map_err(|e| SchedulerError::TaskFailed(format!("Google auth failed: {}", e)))?;

        let store = GoogleDocsProcessedStore::new(config.processed_db_path.clone())?;

        Ok(Self {
            config,
            auth,
            store,
        })
    }

    /// Get reference to the auth manager.
    pub fn auth(&self) -> &GoogleAuth {
        &self.auth
    }

    /// Get reference to the config.
    pub fn config(&self) -> &GoogleDocsPollerConfig {
        &self.config
    }

    /// Get reference to the store.
    pub fn store(&self) -> &GoogleDocsProcessedStore {
        &self.store
    }

    /// Run one polling cycle.
    pub fn poll_once<E: TaskExecutor>(
        &self,
        scheduler: &mut Scheduler<E>,
    ) -> Result<usize, SchedulerError> {
        let adapter =
            GoogleDocsInboundAdapter::new(self.auth.clone(), self.config.employee_emails.clone());

        // List all shared documents
        let documents = adapter
            .list_shared_documents()
            .map_err(|e| SchedulerError::TaskFailed(format!("Failed to list documents: {}", e)))?;

        debug!("Found {} shared documents", documents.len());

        let mut tasks_created = 0;

        for doc in documents {
            // Register document for tracking
            let owner_email = doc
                .owners
                .as_ref()
                .and_then(|owners| owners.first())
                .and_then(|o| o.email_address.as_deref());

            self.store
                .register_document(&doc.id, doc.name.as_deref(), owner_email)?;

            // Get comments for this document
            let comments = match adapter.list_comments(&doc.id) {
                Ok(c) => c,
                Err(e) => {
                    warn!("Failed to list comments for {}: {}", doc.id, e);
                    continue;
                }
            };

            // Get already processed comments/replies (using tracking IDs)
            let processed = self.store.get_processed_ids(&doc.id)?;

            // Filter for actionable comments (returns ActionableComment items)
            let actionable_items = adapter.filter_actionable_comments(&comments, &processed);

            for actionable in actionable_items {
                // Convert to inbound message using the new method
                let doc_name = doc.name.as_deref().unwrap_or("Untitled");
                let message = adapter.actionable_to_inbound_message(&doc.id, doc_name, &actionable);

                // Create workspace for this task (use tracking_id for unique workspace)
                let workspace_dir = self.create_workspace(&doc.id, &actionable.tracking_id)?;

                // Write incoming comment to workspace
                self.write_incoming_actionable(&workspace_dir, &message, &actionable)?;

                // Fetch and save document content for agent context
                match adapter.read_document_content(&doc.id) {
                    Ok(doc_content) => {
                        let doc_content_path = workspace_dir
                            .join("incoming_email")
                            .join("document_content.txt");
                        if let Err(e) = std::fs::write(&doc_content_path, &doc_content) {
                            warn!("Failed to save document content for {}: {}", doc.id, e);
                        } else {
                            info!(
                                "Saved document content ({} chars) to {}",
                                doc_content.len(),
                                doc_content_path.display()
                            );
                        }
                    }
                    Err(e) => {
                        warn!("Failed to fetch document content for {}: {}", doc.id, e);
                    }
                }

                // Create RunTask
                let run_task = RunTaskTask {
                    workspace_dir: workspace_dir.clone(),
                    input_email_dir: PathBuf::from("incoming_email"),
                    input_attachments_dir: PathBuf::from("incoming_attachments"),
                    memory_dir: PathBuf::from("memory"),
                    reference_dir: PathBuf::from("references"),
                    model_name: self.config.model_name.clone(),
                    runner: self.config.runner.clone(),
                    codex_disabled: false,
                    reply_to: vec![message.sender.clone()],
                    reply_from: Some("oliver@dowhiz.com".to_string()),
                    archive_root: None,
                    thread_id: Some(message.thread_id.clone()),
                    thread_epoch: None,
                    thread_state_path: None,
                    channel: Channel::GoogleDocs,
                    slack_team_id: None,
                    employee_id: Some(self.config.employee_id.clone()),
                };

                // Schedule the task
                scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

                // Mark as processed using the tracking_id
                self.store
                    .mark_processed_id(&doc.id, &actionable.tracking_id)?;

                tasks_created += 1;
                let item_type = if actionable.triggering_reply.is_some() {
                    "reply"
                } else {
                    "comment"
                };
                info!(
                    "Created task for Google Docs {} {} on {} ({})",
                    item_type, actionable.tracking_id, doc_name, doc.id
                );
            }

            // Update last checked time
            self.store.update_last_checked(&doc.id)?;
        }

        Ok(tasks_created)
    }

    fn create_workspace(
        &self,
        document_id: &str,
        tracking_id: &str,
    ) -> Result<PathBuf, SchedulerError> {
        // Sanitize tracking_id for use in filesystem path (replace : with _)
        let sanitized_id = tracking_id.replace(':', "_");
        let workspace_id = format!("gdocs_{}_{}", document_id, sanitized_id);
        let workspace_dir = self
            .config
            .workspace_root
            .join(&self.config.employee_id)
            .join("workspaces")
            .join(&workspace_id);

        std::fs::create_dir_all(&workspace_dir)?;
        std::fs::create_dir_all(workspace_dir.join("incoming_email"))?;
        std::fs::create_dir_all(workspace_dir.join("incoming_attachments"))?;
        std::fs::create_dir_all(workspace_dir.join("memory"))?;
        std::fs::create_dir_all(workspace_dir.join("references"))?;

        Ok(workspace_dir)
    }

    fn write_incoming_actionable(
        &self,
        workspace_dir: &Path,
        message: &crate::channel::InboundMessage,
        actionable: &ActionableComment,
    ) -> Result<(), SchedulerError> {
        let incoming_dir = workspace_dir.join("incoming_email");

        // Write raw comment JSON (include full comment with replies)
        let raw_path = incoming_dir.join("google_docs_comment.json");
        let raw_json = serde_json::to_string_pretty(&actionable.comment)
            .map_err(|e| SchedulerError::TaskFailed(format!("JSON error: {}", e)))?;
        std::fs::write(&raw_path, raw_json)?;

        // Write HTML representation for the agent
        let html_content = self.format_actionable_as_html(message, actionable);
        let html_path = incoming_dir.join("email.html");
        std::fs::write(&html_path, html_content)?;

        Ok(())
    }

    fn format_actionable_as_html(
        &self,
        message: &crate::channel::InboundMessage,
        actionable: &ActionableComment,
    ) -> String {
        let doc_name = message
            .metadata
            .google_docs_document_name
            .as_deref()
            .unwrap_or("Document");

        let doc_id = message
            .metadata
            .google_docs_document_id
            .as_deref()
            .unwrap_or("");

        let sender_name = message.sender_name.as_deref().unwrap_or(&message.sender);

        let quoted_text = actionable
            .comment
            .quoted_file_content
            .as_ref()
            .and_then(|q| q.value.as_deref())
            .unwrap_or("");

        // Build conversation thread HTML if this is a reply
        let thread_html = if let Some(ref reply) = actionable.triggering_reply {
            let original_author = actionable
                .comment
                .author
                .as_ref()
                .and_then(|a| a.display_name.as_deref())
                .unwrap_or("Someone");

            format!(
                r#"<div class="conversation-thread">
            <h3>Conversation Thread:</h3>
            <div class="original-comment">
                <p><strong>{} (original comment):</strong></p>
                <p>{}</p>
            </div>
            <div class="reply" style="margin-left: 20px; border-left: 2px solid #ccc; padding-left: 10px;">
                <p><strong>{} (reply that mentions you):</strong></p>
                <p>{}</p>
            </div>
        </div>"#,
                original_author, actionable.comment.content, sender_name, reply.content
            )
        } else {
            format!(
                r#"<div class="comment-content">
            <h3>Comment:</h3>
            <p>{}</p>
        </div>"#,
                actionable.comment.content
            )
        };

        let item_type = if actionable.triggering_reply.is_some() {
            "Reply"
        } else {
            "Comment"
        };

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Google Docs {}</title>
</head>
<body>
    <div class="google-docs-comment">
        <h2>{} on: {}</h2>
        <p><strong>Document ID:</strong> {}</p>
        <p><strong>From:</strong> {} ({})</p>
        <p><strong>Comment ID:</strong> {}</p>
        <p><strong>Tracking ID:</strong> {}</p>

        {}

        {}

        <div class="instructions">
            <h3>Instructions:</h3>
            <p>This is a {} from a Google Docs document. The user is requesting your help.</p>
            <p>To respond:</p>
            <ol>
                <li>Read the {} and quoted text (if any)</li>
                <li>Read the document content from <code>incoming_email/document_content.txt</code></li>
                <li>Process the user's request</li>
                <li>Write your response to <code>reply_email_draft.html</code></li>
                <li>If you need to propose document edits, ask the user whether they prefer "direct editing" or "suggesting mode"</li>
            </ol>
        </div>
    </div>
</body>
</html>"#,
            item_type,
            item_type,
            doc_name,
            doc_id,
            sender_name,
            message.sender,
            actionable.comment.id,
            actionable.tracking_id,
            if quoted_text.is_empty() {
                String::new()
            } else {
                format!(
                    r#"<div class="quoted-text">
            <h3>Quoted text from document:</h3>
            <blockquote>{}</blockquote>
        </div>"#,
                    quoted_text
                )
            },
            thread_html,
            item_type.to_lowercase(),
            item_type.to_lowercase()
        )
    }

    /// Run the polling loop.
    pub fn run_loop<E: TaskExecutor>(
        &self,
        scheduler: &mut Scheduler<E>,
        stop_flag: &AtomicBool,
    ) -> Result<(), SchedulerError> {
        info!(
            "Starting Google Docs poller with {}s interval",
            self.config.poll_interval_secs
        );

        while !stop_flag.load(Ordering::Relaxed) {
            match self.poll_once(scheduler) {
                Ok(count) => {
                    if count > 0 {
                        info!("Google Docs poll created {} tasks", count);
                    }
                }
                Err(e) => {
                    error!("Google Docs poll error: {}", e);
                }
            }

            std::thread::sleep(Duration::from_secs(self.config.poll_interval_secs));
        }

        info!("Google Docs poller stopped");
        Ok(())
    }
}

/// Start the Google Docs polling thread if enabled.
pub fn start_google_docs_poller_thread<E: TaskExecutor + Send + 'static>(
    _scheduler: Arc<std::sync::Mutex<Scheduler<E>>>,
    stop_flag: Arc<AtomicBool>,
) -> Option<std::thread::JoinHandle<()>> {
    let config = GoogleDocsPollerConfig::from_env();

    if !config.enabled {
        info!("Google Docs integration is disabled");
        return None;
    }

    let handle = std::thread::spawn(move || {
        match GoogleDocsPoller::new(config) {
            Ok(_poller) => {
                // Note: This simplified version doesn't actually share the scheduler
                // In a real implementation, you'd need proper synchronization
                warn!("Google Docs poller started (scheduler sharing not implemented yet)");

                while !stop_flag.load(Ordering::Relaxed) {
                    std::thread::sleep(Duration::from_secs(30));
                }
            }
            Err(e) => {
                error!("Failed to start Google Docs poller: {}", e);
            }
        }
    });

    Some(handle)
}
