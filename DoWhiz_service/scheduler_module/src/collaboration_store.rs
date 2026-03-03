//! Collaboration session store for multi-channel artifact collaboration.
//!
//! This module stores collaboration sessions in MongoDB:
//! - **Session**: A collaboration context linking a user, thread, and primary artifact
//! - **Message**: Individual messages from any channel within a session
//! - **Artifact**: External resources associated with a session

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::options::{IndexOptions, UpdateOptions};
use mongodb::sync::Collection;
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};

/// A collaboration session tracking multi-channel interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    /// Unique session identifier (UUID)
    pub id: String,
    /// Associated user ID
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

/// Session status.
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
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
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
    #[error("mongo config error: {0}")]
    MongoConfig(String),
}

/// Store for collaboration sessions, messages, and artifacts.
#[derive(Debug, Clone)]
pub struct CollaborationStore {
    sessions: Collection<Document>,
    messages: Collection<Document>,
    artifacts: Collection<Document>,
}

impl CollaborationStore {
    /// Create a new CollaborationStore.
    pub fn new(_path: impl Into<PathBuf>) -> Result<Self, CollaborationStoreError> {
        let client = create_client_from_env()
            .map_err(|err| CollaborationStoreError::MongoConfig(err.to_string()))?;
        let db = database_from_env(&client);
        let sessions = db.collection::<Document>("collaboration_sessions");
        let messages = db.collection::<Document>("collaboration_messages");
        let artifacts = db.collection::<Document>("collaboration_artifacts");

        ensure_index_compatible(
            &sessions,
            IndexModel::builder()
                .keys(doc! { "id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
        )?;
        ensure_index_compatible(
            &sessions,
            IndexModel::builder()
                .keys(doc! { "user_id": 1, "thread_id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
        )?;
        ensure_index_compatible(
            &sessions,
            IndexModel::builder()
                .keys(doc! { "status": 1, "last_activity_at": -1 })
                .build(),
        )?;
        ensure_index_compatible(
            &messages,
            IndexModel::builder()
                .keys(doc! { "session_id": 1, "timestamp": 1 })
                .build(),
        )?;
        ensure_index_compatible(
            &artifacts,
            IndexModel::builder()
                .keys(doc! { "artifact_type": 1, "artifact_id": 1 })
                .build(),
        )?;
        ensure_index_compatible(
            &artifacts,
            IndexModel::builder()
                .keys(doc! { "session_id": 1, "artifact_type": 1, "artifact_id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
        )?;

        Ok(Self {
            sessions,
            messages,
            artifacts,
        })
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
        if let Some(existing) = self.find_session_by_thread(user_id, thread_id)? {
            return Ok(existing);
        }

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

        self.sessions
            .insert_one(session_to_document(&session), None)?;

        if let (Some(art_type), Some(art_id)) = (artifact_type, artifact_id) {
            let _ = self.add_artifact_internal(
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
    pub fn get_session(
        &self,
        session_id: &str,
    ) -> Result<CollaborationSession, CollaborationStoreError> {
        let session = self
            .sessions
            .find_one(doc! { "id": session_id }, None)?
            .ok_or_else(|| CollaborationStoreError::SessionNotFound(session_id.to_string()))?;
        session_from_document(session)
    }

    /// Find an active session by artifact type and ID.
    /// Optionally filter by user_id (None = any user).
    pub fn find_session_by_artifact(
        &self,
        artifact_type: &str,
        artifact_id: &str,
        user_id: Option<&str>,
    ) -> Result<Option<CollaborationSession>, CollaborationStoreError> {
        let cursor = self.artifacts.find(
            doc! { "artifact_type": artifact_type, "artifact_id": artifact_id },
            None,
        )?;

        let mut best: Option<CollaborationSession> = None;
        for row in cursor {
            let artifact = row?;
            let session_id = match artifact.get_str("session_id") {
                Ok(value) => value,
                Err(_) => continue,
            };
            let session = match self.get_session(session_id) {
                Ok(value) => value,
                Err(CollaborationStoreError::SessionNotFound(_)) => continue,
                Err(err) => return Err(err),
            };
            if session.status != SessionStatus::Active {
                continue;
            }
            if let Some(uid) = user_id {
                if session.user_id != uid {
                    continue;
                }
            }
            match &best {
                Some(existing) if existing.last_activity_at >= session.last_activity_at => {}
                _ => best = Some(session),
            }
        }
        Ok(best)
    }

    /// Find a session by user_id and thread_id.
    pub fn find_session_by_thread(
        &self,
        user_id: &str,
        thread_id: &str,
    ) -> Result<Option<CollaborationSession>, CollaborationStoreError> {
        let doc = self
            .sessions
            .find_one(doc! { "user_id": user_id, "thread_id": thread_id }, None)?;
        doc.map(session_from_document).transpose()
    }

    /// Update the last activity timestamp of a session.
    pub fn touch_session(&self, session_id: &str) -> Result<(), CollaborationStoreError> {
        self.sessions.update_one(
            doc! { "id": session_id },
            doc! { "$set": { "last_activity_at": BsonDateTime::from_chrono(Utc::now()) } },
            None,
        )?;
        Ok(())
    }

    /// Update session status.
    pub fn update_session_status(
        &self,
        session_id: &str,
        status: SessionStatus,
    ) -> Result<(), CollaborationStoreError> {
        self.sessions.update_one(
            doc! { "id": session_id },
            doc! {
                "$set": {
                    "status": status.to_string(),
                    "last_activity_at": BsonDateTime::from_chrono(Utc::now()),
                }
            },
            None,
        )?;
        Ok(())
    }

    /// Update session workspace path.
    pub fn update_session_workspace(
        &self,
        session_id: &str,
        workspace_path: &str,
    ) -> Result<(), CollaborationStoreError> {
        self.sessions.update_one(
            doc! { "id": session_id },
            doc! { "$set": { "workspace_path": workspace_path } },
            None,
        )?;
        Ok(())
    }

    /// Mark sessions as stale if they haven't been active for the specified number of days.
    pub fn mark_stale_sessions(&self, stale_days: i64) -> Result<usize, CollaborationStoreError> {
        let cutoff = Utc::now() - chrono::Duration::days(stale_days);
        let result = self.sessions.update_many(
            doc! {
                "status": "active",
                "last_activity_at": { "$lt": BsonDateTime::from_chrono(cutoff) }
            },
            doc! { "$set": { "status": "stale" } },
            None,
        )?;
        Ok(result.modified_count as usize)
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

        self.messages
            .insert_one(message_to_document(&message), None)?;
        let touch = self.sessions.update_one(
            doc! { "id": session_id },
            doc! { "$set": { "last_activity_at": BsonDateTime::from_chrono(now) } },
            None,
        )?;
        if touch.matched_count == 0 {
            return Err(CollaborationStoreError::SessionNotFound(
                session_id.to_string(),
            ));
        }

        Ok(message)
    }

    /// Get all messages for a session, ordered by timestamp.
    pub fn get_messages(
        &self,
        session_id: &str,
    ) -> Result<Vec<CollaborationMessage>, CollaborationStoreError> {
        let cursor = self
            .messages
            .find(doc! { "session_id": session_id }, None)?;
        let mut messages = Vec::new();
        for row in cursor {
            messages.push(message_from_document(row?)?);
        }
        messages.sort_by_key(|message| message.timestamp);
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
        self.add_artifact_internal(
            session_id,
            artifact_type,
            artifact_id,
            artifact_url,
            artifact_title,
            role,
        )
    }

    fn add_artifact_internal(
        &self,
        session_id: &str,
        artifact_type: &str,
        artifact_id: &str,
        artifact_url: Option<&str>,
        artifact_title: Option<&str>,
        role: ArtifactRole,
    ) -> Result<CollaborationArtifact, CollaborationStoreError> {
        let now = Utc::now();
        let filter = doc! {
            "session_id": session_id,
            "artifact_type": artifact_type,
            "artifact_id": artifact_id,
        };
        let generated_id = Uuid::new_v4().to_string();
        self.artifacts.update_one(
            filter.clone(),
            doc! {
                "$set": {
                    "artifact_url": artifact_url.map(Bson::from).unwrap_or(Bson::Null),
                    "artifact_title": artifact_title.map(Bson::from).unwrap_or(Bson::Null),
                    "role": role.to_string(),
                },
                "$setOnInsert": {
                    "id": &generated_id,
                    "session_id": session_id,
                    "artifact_type": artifact_type,
                    "artifact_id": artifact_id,
                    "created_at": BsonDateTime::from_chrono(now),
                },
            },
            UpdateOptions::builder().upsert(Some(true)).build(),
        )?;
        let doc = self
            .artifacts
            .find_one(filter, None)?
            .ok_or_else(|| CollaborationStoreError::SessionNotFound(session_id.to_string()))?;
        artifact_from_document(doc)
    }

    /// Get all artifacts for a session.
    pub fn get_artifacts(
        &self,
        session_id: &str,
    ) -> Result<Vec<CollaborationArtifact>, CollaborationStoreError> {
        let cursor = self
            .artifacts
            .find(doc! { "session_id": session_id }, None)?;
        let mut artifacts = Vec::new();
        for row in cursor {
            artifacts.push(artifact_from_document(row?)?);
        }
        artifacts.sort_by_key(|artifact| artifact.created_at);
        Ok(artifacts)
    }
}

fn session_to_document(session: &CollaborationSession) -> Document {
    doc! {
        "id": &session.id,
        "user_id": &session.user_id,
        "thread_id": &session.thread_id,
        "primary_channel": &session.primary_channel,
        "artifact_type": session.artifact_type.clone().map(Bson::from).unwrap_or(Bson::Null),
        "artifact_id": session.artifact_id.clone().map(Bson::from).unwrap_or(Bson::Null),
        "artifact_title": session.artifact_title.clone().map(Bson::from).unwrap_or(Bson::Null),
        "original_request": session.original_request.clone().map(Bson::from).unwrap_or(Bson::Null),
        "status": session.status.to_string(),
        "created_at": BsonDateTime::from_chrono(session.created_at),
        "last_activity_at": BsonDateTime::from_chrono(session.last_activity_at),
        "workspace_path": session.workspace_path.clone().map(Bson::from).unwrap_or(Bson::Null),
    }
}

fn session_from_document(
    document: Document,
) -> Result<CollaborationSession, CollaborationStoreError> {
    let status = document
        .get_str("status")
        .unwrap_or("active")
        .parse()
        .map_err(CollaborationStoreError::StatusParse)?;
    Ok(CollaborationSession {
        id: required_string(&document, "id")?,
        user_id: required_string(&document, "user_id")?,
        thread_id: required_string(&document, "thread_id")?,
        primary_channel: required_string(&document, "primary_channel")?,
        artifact_type: optional_string(&document, "artifact_type"),
        artifact_id: optional_string(&document, "artifact_id"),
        artifact_title: optional_string(&document, "artifact_title"),
        original_request: optional_string(&document, "original_request"),
        status,
        created_at: read_datetime(&document, "created_at")?,
        last_activity_at: read_datetime(&document, "last_activity_at")?,
        workspace_path: optional_string(&document, "workspace_path"),
    })
}

fn message_to_document(message: &CollaborationMessage) -> Document {
    doc! {
        "id": &message.id,
        "session_id": &message.session_id,
        "source_channel": &message.source_channel,
        "external_message_id": message.external_message_id.clone().map(Bson::from).unwrap_or(Bson::Null),
        "sender_id": &message.sender_id,
        "content_preview": message.content_preview.clone().map(Bson::from).unwrap_or(Bson::Null),
        "has_attachments": message.has_attachments,
        "attachment_manifest": message.attachment_manifest.clone().map(Bson::from).unwrap_or(Bson::Null),
        "timestamp": BsonDateTime::from_chrono(message.timestamp),
    }
}

fn message_from_document(
    document: Document,
) -> Result<CollaborationMessage, CollaborationStoreError> {
    Ok(CollaborationMessage {
        id: required_string(&document, "id")?,
        session_id: required_string(&document, "session_id")?,
        source_channel: required_string(&document, "source_channel")?,
        external_message_id: optional_string(&document, "external_message_id"),
        sender_id: required_string(&document, "sender_id")?,
        content_preview: optional_string(&document, "content_preview"),
        has_attachments: document.get_bool("has_attachments").unwrap_or(false),
        attachment_manifest: optional_string(&document, "attachment_manifest"),
        timestamp: read_datetime(&document, "timestamp")?,
    })
}

fn artifact_from_document(
    document: Document,
) -> Result<CollaborationArtifact, CollaborationStoreError> {
    let role = document
        .get_str("role")
        .unwrap_or("target")
        .parse()
        .unwrap_or(ArtifactRole::Target);
    Ok(CollaborationArtifact {
        id: required_string(&document, "id")?,
        session_id: required_string(&document, "session_id")?,
        artifact_type: required_string(&document, "artifact_type")?,
        artifact_id: required_string(&document, "artifact_id")?,
        artifact_url: optional_string(&document, "artifact_url"),
        artifact_title: optional_string(&document, "artifact_title"),
        role,
        created_at: read_datetime(&document, "created_at")?,
    })
}

fn required_string(document: &Document, key: &str) -> Result<String, CollaborationStoreError> {
    document
        .get_str(key)
        .map(|value| value.to_string())
        .map_err(|err| CollaborationStoreError::MongoConfig(format!("missing {}: {}", key, err)))
}

fn optional_string(document: &Document, key: &str) -> Option<String> {
    match document.get(key) {
        Some(Bson::String(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn read_datetime(document: &Document, key: &str) -> Result<DateTime<Utc>, CollaborationStoreError> {
    match document.get(key) {
        Some(Bson::DateTime(value)) => Ok(value.to_chrono()),
        Some(Bson::String(value)) => Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc)),
        _ => Err(CollaborationStoreError::MongoConfig(format!(
            "missing datetime field {}",
            key
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn unique(label: &str) -> String {
        format!("{}-{}", label, Uuid::new_v4())
    }

    fn test_store() -> (TempDir, CollaborationStore) {
        let temp = TempDir::new().expect("tempdir");
        let store = CollaborationStore::new(temp.path().join("collaboration.db")).expect("store");
        (temp, store)
    }

    #[test]
    fn create_and_get_session() {
        let (_temp, store) = test_store();
        let user_id = unique("user");
        let thread_id = unique("thread");
        let doc_id = unique("doc");

        let session = store
            .create_session(
                &user_id,
                &thread_id,
                "email",
                Some("google_docs"),
                Some(&doc_id),
                Some("My Document"),
                Some("Please review this document"),
                Some("/path/to/workspace"),
            )
            .expect("create");

        let retrieved = store.get_session(&session.id).expect("get");
        assert_eq!(retrieved.id, session.id);
        assert_eq!(retrieved.user_id, user_id);
        assert_eq!(retrieved.thread_id, thread_id);
    }

    #[test]
    fn find_session_by_artifact_and_thread() {
        let (_temp, store) = test_store();
        let user_id = unique("user");
        let thread_id = unique("thread");
        let doc_id = unique("doc");

        let session = store
            .create_session(
                &user_id,
                &thread_id,
                "email",
                Some("google_docs"),
                Some(&doc_id),
                None,
                None,
                None,
            )
            .expect("create");

        let by_artifact = store
            .find_session_by_artifact("google_docs", &doc_id, Some(&user_id))
            .expect("find artifact")
            .expect("session");
        assert_eq!(by_artifact.id, session.id);

        let by_thread = store
            .find_session_by_thread(&user_id, &thread_id)
            .expect("find thread")
            .expect("session");
        assert_eq!(by_thread.id, session.id);
    }

    #[test]
    fn add_and_list_messages_and_artifacts() {
        let (_temp, store) = test_store();
        let user_id = unique("user");
        let thread_id = unique("thread");
        let doc_id = unique("doc");

        let session = store
            .create_session(
                &user_id,
                &thread_id,
                "email",
                Some("google_docs"),
                Some(&doc_id),
                None,
                None,
                None,
            )
            .expect("create");

        store
            .add_message(
                &session.id,
                "google_docs",
                Some("comment:1"),
                &user_id,
                Some("hello"),
                false,
                None,
            )
            .expect("add message");

        store
            .add_artifact(
                &session.id,
                "github_pr",
                &unique("pr"),
                Some("https://example.com/pr"),
                Some("PR"),
                ArtifactRole::Reference,
            )
            .expect("add artifact");

        let messages = store.get_messages(&session.id).expect("messages");
        let artifacts = store.get_artifacts(&session.id).expect("artifacts");
        assert_eq!(messages.len(), 1);
        assert!(artifacts.len() >= 2); // Includes primary artifact from create_session.
    }

    #[test]
    fn mark_stale_sessions_updates_status() {
        let (_temp, store) = test_store();
        let user_id = unique("user");
        let thread_id = unique("thread");
        let session = store
            .create_session(&user_id, &thread_id, "email", None, None, None, None, None)
            .expect("create");

        let stale_time = Utc::now() - chrono::Duration::days(10);
        store
            .sessions
            .update_one(
                doc! { "id": &session.id },
                doc! { "$set": { "last_activity_at": BsonDateTime::from_chrono(stale_time) } },
                None,
            )
            .expect("seed stale timestamp");

        let changed = store.mark_stale_sessions(7).expect("mark stale");
        assert!(changed >= 1);
        let refreshed = store.get_session(&session.id).expect("get");
        assert_eq!(refreshed.status, SessionStatus::Stale);
    }
}
