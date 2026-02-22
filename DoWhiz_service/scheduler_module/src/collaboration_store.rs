//! Collaboration session store for multi-channel artifact collaboration.
//!
//! This module provides storage for collaboration sessions that track
//! multi-channel interactions around shared artifacts (Google Docs, GitHub PRs, etc.).
//!
//! Key concepts:
//! - **Session**: A collaboration context linking a user, thread, and primary artifact
//! - **Message**: Individual messages from any channel within a session
//! - **Artifact**: External resources (documents, PRs, etc.) associated with a session

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

/// A collaboration session tracking multi-channel interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    /// Unique session identifier (UUID)
    pub id: String,
    /// Associated user ID (from users table)
    pub user_id: String,
    /// Thread ID for grouping related messages
    pub thread_id: String,
    /// The channel where collaboration started
    pub primary_channel: String,
    /// Primary artifact type (e.g., "google_docs", "github_pr")
    pub artifact_type: Option<String>,
    /// Primary artifact external ID
    pub artifact_id: Option<String>,
    /// Primary artifact title/name
    pub artifact_title: Option<String>,
    /// Original request/instruction from user
    pub original_request: Option<String>,
    /// Session status: "active", "completed", "stale"
    pub status: SessionStatus,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_activity_at: DateTime<Utc>,
    /// Path to the session's workspace directory
    pub workspace_path: Option<String>,
}

/// Session status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Completed,
    Stale,
}

impl Default for SessionStatus {
    fn default() -> Self {
        SessionStatus::Active
    }
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::Active => write!(f, "active"),
            SessionStatus::Completed => write!(f, "completed"),
            SessionStatus::Stale => write!(f, "stale"),
        }
    }
}

impl std::str::FromStr for SessionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(SessionStatus::Active),
            "completed" => Ok(SessionStatus::Completed),
            "stale" => Ok(SessionStatus::Stale),
            _ => Err(format!("unknown session status: {}", s)),
        }
    }
}

/// A message within a collaboration session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationMessage {
    /// Unique message identifier
    pub id: String,
    /// Parent session ID
    pub session_id: String,
    /// Source channel (e.g., "email", "google_docs")
    pub source_channel: String,
    /// External message ID from the source platform
    pub external_message_id: Option<String>,
    /// Sender's user ID
    pub sender_id: String,
    /// Content preview (first 500 chars)
    pub content_preview: Option<String>,
    /// Whether this message has attachments
    pub has_attachments: bool,
    /// JSON manifest of attachments: [{name, type, path}]
    pub attachment_manifest: Option<String>,
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
}

/// An artifact linked to a collaboration session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationArtifact {
    /// Unique artifact link identifier
    pub id: String,
    /// Parent session ID
    pub session_id: String,
    /// Artifact type (e.g., "google_docs", "github_pr")
    pub artifact_type: String,
    /// External artifact ID
    pub artifact_id: String,
    /// Full URL to the artifact
    pub artifact_url: Option<String>,
    /// Artifact title/name
    pub artifact_title: Option<String>,
    /// Role: "target" (to modify) or "reference" (for context)
    pub role: ArtifactRole,
    /// When this artifact was linked
    pub created_at: DateTime<Utc>,
}

/// Role of an artifact in a collaboration session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactRole {
    /// The artifact to be modified
    Target,
    /// Reference material
    Reference,
}

impl Default for ArtifactRole {
    fn default() -> Self {
        ArtifactRole::Target
    }
}

impl std::fmt::Display for ArtifactRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactRole::Target => write!(f, "target"),
            ArtifactRole::Reference => write!(f, "reference"),
        }
    }
}

impl std::str::FromStr for ArtifactRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "target" => Ok(ArtifactRole::Target),
            "reference" => Ok(ArtifactRole::Reference),
            _ => Err(format!("unknown artifact role: {}", s)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CollaborationStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("session not found: {0}")]
    SessionNotFound(String),
    #[error("datetime parse error: {0}")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error("status parse error: {0}")]
    StatusParse(String),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Store for collaboration sessions, messages, and artifacts.
#[derive(Debug, Clone)]
pub struct CollaborationStore {
    path: PathBuf,
}

impl CollaborationStore {
    /// Create a new CollaborationStore, initializing the database if needed.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, CollaborationStoreError> {
        let store = Self { path: path.into() };
        let _ = store.open()?;
        Ok(store)
    }

    // =========================================================================
    // Session operations
    // =========================================================================

    /// Create a new collaboration session.
    pub fn create_session(
        &self,
        user_id: &str,
        thread_id: &str,
        primary_channel: &str,
        artifact_type: Option<&str>,
        artifact_id: Option<&str>,
        artifact_title: Option<&str>,
        original_request: Option<&str>,
        workspace_path: Option<&str>,
    ) -> Result<CollaborationSession, CollaborationStoreError> {
        let conn = self.open()?;
        let now = Utc::now();
        let session = CollaborationSession {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            thread_id: thread_id.to_string(),
            primary_channel: primary_channel.to_string(),
            artifact_type: artifact_type.map(String::from),
            artifact_id: artifact_id.map(String::from),
            artifact_title: artifact_title.map(String::from),
            original_request: original_request.map(String::from),
            status: SessionStatus::Active,
            created_at: now,
            last_activity_at: now,
            workspace_path: workspace_path.map(String::from),
        };

        conn.execute(
            "INSERT INTO collaboration_sessions
             (id, user_id, thread_id, primary_channel, artifact_type, artifact_id,
              artifact_title, original_request, status, created_at, last_activity_at, workspace_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                session.id,
                session.user_id,
                session.thread_id,
                session.primary_channel,
                session.artifact_type,
                session.artifact_id,
                session.artifact_title,
                session.original_request,
                session.status.to_string(),
                format_datetime(session.created_at),
                format_datetime(session.last_activity_at),
                session.workspace_path,
            ],
        )?;

        // If there's a primary artifact, also add it to the artifacts table
        if let (Some(art_type), Some(art_id)) = (artifact_type, artifact_id) {
            self.add_artifact_internal(
                &conn,
                &session.id,
                art_type,
                art_id,
                None,
                artifact_title,
                ArtifactRole::Target,
            )?;
        }

        Ok(session)
    }

    /// Get a session by ID.
    pub fn get_session(&self, session_id: &str) -> Result<CollaborationSession, CollaborationStoreError> {
        let conn = self.open()?;
        self.get_session_internal(&conn, session_id)
    }

    fn get_session_internal(
        &self,
        conn: &Connection,
        session_id: &str,
    ) -> Result<CollaborationSession, CollaborationStoreError> {
        let row = conn
            .query_row(
                "SELECT id, user_id, thread_id, primary_channel, artifact_type, artifact_id,
                        artifact_title, original_request, status, created_at, last_activity_at, workspace_path
                 FROM collaboration_sessions
                 WHERE id = ?1",
                params![session_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, Option<String>>(7)?,
                        row.get::<_, String>(8)?,
                        row.get::<_, String>(9)?,
                        row.get::<_, String>(10)?,
                        row.get::<_, Option<String>>(11)?,
                    ))
                },
            )
            .optional()?;

        match row {
            Some((id, user_id, thread_id, primary_channel, artifact_type, artifact_id,
                  artifact_title, original_request, status, created_at, last_activity_at, workspace_path)) => {
                Ok(CollaborationSession {
                    id,
                    user_id,
                    thread_id,
                    primary_channel,
                    artifact_type,
                    artifact_id,
                    artifact_title,
                    original_request,
                    status: status.parse().map_err(CollaborationStoreError::StatusParse)?,
                    created_at: parse_datetime(&created_at)?,
                    last_activity_at: parse_datetime(&last_activity_at)?,
                    workspace_path,
                })
            }
            None => Err(CollaborationStoreError::SessionNotFound(session_id.to_string())),
        }
    }

    /// Find an active session by artifact type and ID.
    /// Optionally filter by user_id (None = any user).
    pub fn find_session_by_artifact(
        &self,
        artifact_type: &str,
        artifact_id: &str,
        user_id: Option<&str>,
    ) -> Result<Option<CollaborationSession>, CollaborationStoreError> {
        let conn = self.open()?;

        let query = if user_id.is_some() {
            "SELECT s.id FROM collaboration_sessions s
             JOIN collaboration_artifacts a ON s.id = a.session_id
             WHERE a.artifact_type = ?1 AND a.artifact_id = ?2 AND s.status = 'active' AND s.user_id = ?3
             ORDER BY s.last_activity_at DESC
             LIMIT 1"
        } else {
            "SELECT s.id FROM collaboration_sessions s
             JOIN collaboration_artifacts a ON s.id = a.session_id
             WHERE a.artifact_type = ?1 AND a.artifact_id = ?2 AND s.status = 'active'
             ORDER BY s.last_activity_at DESC
             LIMIT 1"
        };

        let session_id: Option<String> = if let Some(uid) = user_id {
            conn.query_row(query, params![artifact_type, artifact_id, uid], |row| row.get(0))
                .optional()?
        } else {
            conn.query_row(query, params![artifact_type, artifact_id], |row| row.get(0))
                .optional()?
        };

        match session_id {
            Some(id) => Ok(Some(self.get_session_internal(&conn, &id)?)),
            None => Ok(None),
        }
    }

    /// Find a session by user_id and thread_id.
    pub fn find_session_by_thread(
        &self,
        user_id: &str,
        thread_id: &str,
    ) -> Result<Option<CollaborationSession>, CollaborationStoreError> {
        let conn = self.open()?;

        let session_id: Option<String> = conn
            .query_row(
                "SELECT id FROM collaboration_sessions
                 WHERE user_id = ?1 AND thread_id = ?2
                 LIMIT 1",
                params![user_id, thread_id],
                |row| row.get(0),
            )
            .optional()?;

        match session_id {
            Some(id) => Ok(Some(self.get_session_internal(&conn, &id)?)),
            None => Ok(None),
        }
    }

    /// Update the last activity timestamp of a session.
    pub fn touch_session(&self, session_id: &str) -> Result<(), CollaborationStoreError> {
        let conn = self.open()?;
        let now = format_datetime(Utc::now());
        conn.execute(
            "UPDATE collaboration_sessions SET last_activity_at = ?1 WHERE id = ?2",
            params![now, session_id],
        )?;
        Ok(())
    }

    /// Update session status.
    pub fn update_session_status(
        &self,
        session_id: &str,
        status: SessionStatus,
    ) -> Result<(), CollaborationStoreError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE collaboration_sessions SET status = ?1, last_activity_at = ?2 WHERE id = ?3",
            params![status.to_string(), format_datetime(Utc::now()), session_id],
        )?;
        Ok(())
    }

    /// Update session workspace path.
    pub fn update_session_workspace(
        &self,
        session_id: &str,
        workspace_path: &str,
    ) -> Result<(), CollaborationStoreError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE collaboration_sessions SET workspace_path = ?1 WHERE id = ?2",
            params![workspace_path, session_id],
        )?;
        Ok(())
    }

    /// Mark sessions as stale if they haven't been active for the specified number of days.
    pub fn mark_stale_sessions(&self, stale_days: i64) -> Result<usize, CollaborationStoreError> {
        let conn = self.open()?;
        let cutoff = Utc::now() - chrono::Duration::days(stale_days);
        let rows = conn.execute(
            "UPDATE collaboration_sessions
             SET status = 'stale'
             WHERE status = 'active' AND last_activity_at < ?1",
            params![format_datetime(cutoff)],
        )?;
        Ok(rows)
    }

    // =========================================================================
    // Message operations
    // =========================================================================

    /// Add a message to a session.
    pub fn add_message(
        &self,
        session_id: &str,
        source_channel: &str,
        external_message_id: Option<&str>,
        sender_id: &str,
        content_preview: Option<&str>,
        has_attachments: bool,
        attachment_manifest: Option<&str>,
    ) -> Result<CollaborationMessage, CollaborationStoreError> {
        let conn = self.open()?;
        let now = Utc::now();
        let message = CollaborationMessage {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            source_channel: source_channel.to_string(),
            external_message_id: external_message_id.map(String::from),
            sender_id: sender_id.to_string(),
            content_preview: content_preview.map(String::from),
            has_attachments,
            attachment_manifest: attachment_manifest.map(String::from),
            timestamp: now,
        };

        conn.execute(
            "INSERT INTO collaboration_messages
             (id, session_id, source_channel, external_message_id, sender_id,
              content_preview, has_attachments, attachment_manifest, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                message.id,
                message.session_id,
                message.source_channel,
                message.external_message_id,
                message.sender_id,
                message.content_preview,
                message.has_attachments as i32,
                message.attachment_manifest,
                format_datetime(message.timestamp),
            ],
        )?;

        // Update session's last activity
        conn.execute(
            "UPDATE collaboration_sessions SET last_activity_at = ?1 WHERE id = ?2",
            params![format_datetime(now), session_id],
        )?;

        Ok(message)
    }

    /// Get all messages for a session, ordered by timestamp.
    pub fn get_messages(&self, session_id: &str) -> Result<Vec<CollaborationMessage>, CollaborationStoreError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT id, session_id, source_channel, external_message_id, sender_id,
                    content_preview, has_attachments, attachment_manifest, timestamp
             FROM collaboration_messages
             WHERE session_id = ?1
             ORDER BY timestamp ASC",
        )?;

        let rows = stmt.query_map(params![session_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, i32>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, String>(8)?,
            ))
        })?;

        let mut messages = Vec::new();
        for row in rows {
            let (id, session_id, source_channel, external_message_id, sender_id,
                 content_preview, has_attachments, attachment_manifest, timestamp) = row?;
            messages.push(CollaborationMessage {
                id,
                session_id,
                source_channel,
                external_message_id,
                sender_id,
                content_preview,
                has_attachments: has_attachments != 0,
                attachment_manifest,
                timestamp: parse_datetime(&timestamp)?,
            });
        }
        Ok(messages)
    }

    // =========================================================================
    // Artifact operations
    // =========================================================================

    /// Add an artifact to a session.
    pub fn add_artifact(
        &self,
        session_id: &str,
        artifact_type: &str,
        artifact_id: &str,
        artifact_url: Option<&str>,
        artifact_title: Option<&str>,
        role: ArtifactRole,
    ) -> Result<CollaborationArtifact, CollaborationStoreError> {
        let conn = self.open()?;
        self.add_artifact_internal(&conn, session_id, artifact_type, artifact_id, artifact_url, artifact_title, role)
    }

    fn add_artifact_internal(
        &self,
        conn: &Connection,
        session_id: &str,
        artifact_type: &str,
        artifact_id: &str,
        artifact_url: Option<&str>,
        artifact_title: Option<&str>,
        role: ArtifactRole,
    ) -> Result<CollaborationArtifact, CollaborationStoreError> {
        let now = Utc::now();
        let artifact = CollaborationArtifact {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            artifact_type: artifact_type.to_string(),
            artifact_id: artifact_id.to_string(),
            artifact_url: artifact_url.map(String::from),
            artifact_title: artifact_title.map(String::from),
            role,
            created_at: now,
        };

        conn.execute(
            "INSERT INTO collaboration_artifacts
             (id, session_id, artifact_type, artifact_id, artifact_url, artifact_title, role, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(session_id, artifact_type, artifact_id) DO UPDATE SET
                artifact_url = excluded.artifact_url,
                artifact_title = excluded.artifact_title",
            params![
                artifact.id,
                artifact.session_id,
                artifact.artifact_type,
                artifact.artifact_id,
                artifact.artifact_url,
                artifact.artifact_title,
                artifact.role.to_string(),
                format_datetime(artifact.created_at),
            ],
        )?;

        Ok(artifact)
    }

    /// Get all artifacts for a session.
    pub fn get_artifacts(&self, session_id: &str) -> Result<Vec<CollaborationArtifact>, CollaborationStoreError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT id, session_id, artifact_type, artifact_id, artifact_url, artifact_title, role, created_at
             FROM collaboration_artifacts
             WHERE session_id = ?1
             ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map(params![session_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;

        let mut artifacts = Vec::new();
        for row in rows {
            let (id, session_id, artifact_type, artifact_id, artifact_url, artifact_title, role, created_at) = row?;
            artifacts.push(CollaborationArtifact {
                id,
                session_id,
                artifact_type,
                artifact_id,
                artifact_url,
                artifact_title,
                role: role.parse().unwrap_or(ArtifactRole::Target),
                created_at: parse_datetime(&created_at)?,
            });
        }
        Ok(artifacts)
    }

    // =========================================================================
    // Database initialization
    // =========================================================================

    fn open(&self) -> Result<Connection, CollaborationStoreError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(5))?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS collaboration_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                thread_id TEXT NOT NULL,
                primary_channel TEXT NOT NULL,
                artifact_type TEXT,
                artifact_id TEXT,
                artifact_title TEXT,
                original_request TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                created_at TEXT NOT NULL,
                last_activity_at TEXT NOT NULL,
                workspace_path TEXT,
                UNIQUE(user_id, thread_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_artifact
             ON collaboration_sessions(artifact_type, artifact_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_sessions_status
             ON collaboration_sessions(status)",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS collaboration_messages (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                source_channel TEXT NOT NULL,
                external_message_id TEXT,
                sender_id TEXT NOT NULL,
                content_preview TEXT,
                has_attachments INTEGER DEFAULT 0,
                attachment_manifest TEXT,
                timestamp TEXT NOT NULL,
                FOREIGN KEY(session_id) REFERENCES collaboration_sessions(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_session
             ON collaboration_messages(session_id)",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS collaboration_artifacts (
                id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                artifact_type TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                artifact_url TEXT,
                artifact_title TEXT,
                role TEXT DEFAULT 'target',
                created_at TEXT NOT NULL,
                FOREIGN KEY(session_id) REFERENCES collaboration_sessions(id),
                UNIQUE(session_id, artifact_type, artifact_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_artifacts_lookup
             ON collaboration_artifacts(artifact_type, artifact_id)",
            [],
        )?;

        Ok(conn)
    }
}

fn format_datetime(value: DateTime<Utc>) -> String {
    value.to_rfc3339()
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_store() -> (TempDir, CollaborationStore) {
        let temp = TempDir::new().expect("tempdir");
        let path = temp.path().join("collaboration.db");
        let store = CollaborationStore::new(&path).expect("store");
        (temp, store)
    }

    #[test]
    fn create_and_get_session() {
        let (_temp, store) = test_store();

        let session = store
            .create_session(
                "user-123",
                "thread-456",
                "email",
                Some("google_docs"),
                Some("doc-789"),
                Some("My Document"),
                Some("Please review this document"),
                Some("/path/to/workspace"),
            )
            .expect("create");

        assert_eq!(session.user_id, "user-123");
        assert_eq!(session.thread_id, "thread-456");
        assert_eq!(session.primary_channel, "email");
        assert_eq!(session.artifact_type, Some("google_docs".to_string()));
        assert_eq!(session.artifact_id, Some("doc-789".to_string()));
        assert_eq!(session.status, SessionStatus::Active);

        let retrieved = store.get_session(&session.id).expect("get");
        assert_eq!(retrieved.id, session.id);
        assert_eq!(retrieved.user_id, "user-123");
    }

    #[test]
    fn find_session_by_artifact() {
        let (_temp, store) = test_store();

        let session = store
            .create_session(
                "user-123",
                "thread-456",
                "email",
                Some("google_docs"),
                Some("doc-789"),
                None,
                None,
                None,
            )
            .expect("create");

        let found = store
            .find_session_by_artifact("google_docs", "doc-789", Some("user-123"))
            .expect("find");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, session.id);

        let not_found = store
            .find_session_by_artifact("google_docs", "nonexistent", None)
            .expect("find");
        assert!(not_found.is_none());
    }

    #[test]
    fn find_session_by_thread() {
        let (_temp, store) = test_store();

        let session = store
            .create_session("user-123", "thread-456", "slack", None, None, None, None, None)
            .expect("create");

        let found = store
            .find_session_by_thread("user-123", "thread-456")
            .expect("find");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, session.id);

        let not_found = store
            .find_session_by_thread("user-123", "other-thread")
            .expect("find");
        assert!(not_found.is_none());
    }

    #[test]
    fn add_and_get_messages() {
        let (_temp, store) = test_store();

        let session = store
            .create_session("user-123", "thread-456", "email", None, None, None, None, None)
            .expect("create");

        // Add email message
        store
            .add_message(
                &session.id,
                "email",
                Some("msg-1"),
                "user-123",
                Some("Please help me with..."),
                true,
                Some(r#"[{"name":"doc.pdf","type":"application/pdf"}]"#),
            )
            .expect("add message 1");

        // Add comment message
        store
            .add_message(
                &session.id,
                "google_docs",
                Some("comment-1"),
                "user-123",
                Some("Also fix the intro"),
                false,
                None,
            )
            .expect("add message 2");

        let messages = store.get_messages(&session.id).expect("get messages");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].source_channel, "email");
        assert_eq!(messages[1].source_channel, "google_docs");
    }

    #[test]
    fn add_and_get_artifacts() {
        let (_temp, store) = test_store();

        let session = store
            .create_session("user-123", "thread-456", "email", None, None, None, None, None)
            .expect("create");

        store
            .add_artifact(
                &session.id,
                "google_docs",
                "doc-123",
                Some("https://docs.google.com/document/d/doc-123"),
                Some("Main Document"),
                ArtifactRole::Target,
            )
            .expect("add artifact 1");

        store
            .add_artifact(
                &session.id,
                "github_pr",
                "pr-456",
                Some("https://github.com/owner/repo/pull/456"),
                Some("Related PR"),
                ArtifactRole::Reference,
            )
            .expect("add artifact 2");

        let artifacts = store.get_artifacts(&session.id).expect("get artifacts");
        assert_eq!(artifacts.len(), 2);
        assert_eq!(artifacts[0].artifact_type, "google_docs");
        assert_eq!(artifacts[0].role, ArtifactRole::Target);
        assert_eq!(artifacts[1].artifact_type, "github_pr");
        assert_eq!(artifacts[1].role, ArtifactRole::Reference);
    }

    #[test]
    fn update_session_status() {
        let (_temp, store) = test_store();

        let session = store
            .create_session("user-123", "thread-456", "email", None, None, None, None, None)
            .expect("create");

        assert_eq!(session.status, SessionStatus::Active);

        store
            .update_session_status(&session.id, SessionStatus::Completed)
            .expect("update status");

        let updated = store.get_session(&session.id).expect("get");
        assert_eq!(updated.status, SessionStatus::Completed);
    }

    #[test]
    fn mark_stale_sessions() {
        let (_temp, store) = test_store();

        // This test just verifies the function doesn't error
        // In a real scenario, we'd need to manipulate timestamps
        let count = store.mark_stale_sessions(7).expect("mark stale");
        assert_eq!(count, 0); // No sessions are stale yet
    }
}
