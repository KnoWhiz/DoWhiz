use crate::memory_store::ensure_default_user_memo;
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::options::FindOptions;
use mongodb::options::IndexOptions;
use mongodb::sync::Collection;
use mongodb::IndexModel;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};

#[derive(Debug)]
pub struct UserStore {
    mongo: MongoUserStore,
}

#[derive(Debug, Clone)]
struct MongoUserStore {
    users: Collection<Document>,
}

#[derive(Debug, Clone)]
pub struct UserRecord {
    pub user_id: String,
    pub identifier_type: String,
    pub identifier: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UserPaths {
    pub root: PathBuf,
    pub state_dir: PathBuf,
    pub tasks_db_path: PathBuf,
    pub memory_dir: PathBuf,
    pub secrets_dir: PathBuf,
    pub mail_root: PathBuf,
    pub workspaces_root: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum UserStoreError {
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
    #[error("datetime parse error: {0}")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error("mongo config error: {0}")]
    MongoConfig(String),
}

impl UserStore {
    pub fn new(_path: impl Into<PathBuf>) -> Result<Self, UserStoreError> {
        Ok(Self {
            mongo: MongoUserStore::new()?,
        })
    }

    /// Get an existing user by identifier without creating one.
    /// Returns None if the user doesn't exist.
    pub fn get_user_by_identifier(
        &self,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<Option<UserRecord>, UserStoreError> {
        self.mongo
            .get_user_by_identifier(identifier_type, identifier)
    }

    pub fn get_or_create_user(
        &self,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<UserRecord, UserStoreError> {
        self.mongo.get_or_create_user(identifier_type, identifier)
    }

    pub fn list_user_ids(&self) -> Result<Vec<String>, UserStoreError> {
        self.mongo.list_user_ids()
    }

    pub fn user_paths(&self, users_root: &Path, user_id: &str) -> UserPaths {
        let root = users_root.join(user_id);
        let state_dir = root.join("state");
        let tasks_db_path = state_dir.join("tasks.db");
        let memory_dir = root.join("memory");
        let secrets_dir = root.join("secrets");
        let mail_root = root.join("mail");
        let workspaces_root = root.join("workspaces");
        UserPaths {
            root,
            state_dir,
            tasks_db_path,
            memory_dir,
            secrets_dir,
            mail_root,
            workspaces_root,
        }
    }

    pub fn ensure_user_dirs(&self, paths: &UserPaths) -> Result<(), UserStoreError> {
        fs::create_dir_all(&paths.state_dir)?;
        fs::create_dir_all(&paths.memory_dir)?;
        fs::create_dir_all(&paths.secrets_dir)?;
        fs::create_dir_all(&paths.mail_root)?;
        fs::create_dir_all(&paths.workspaces_root)?;
        ensure_default_user_memo(&paths.memory_dir)?;
        Ok(())
    }
}

/// Normalize an identifier based on its type.
pub fn normalize_identifier(identifier_type: &str, raw: &str) -> Option<String> {
    match identifier_type {
        "email" => normalize_email(raw),
        "phone" => normalize_phone(raw),
        "slack" => normalize_slack_id(raw),
        // For unknown types, just trim and lowercase
        _ => {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_lowercase())
            }
        }
    }
}

/// Normalize a phone number (strip non-digits, keep leading +).
pub fn normalize_phone(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let has_plus = trimmed.starts_with('+');
    let digits: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    if has_plus {
        Some(format!("+{}", digits))
    } else {
        Some(digits)
    }
}

/// Normalize a Slack user ID (just trim and uppercase).
pub fn normalize_slack_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_uppercase())
    }
}

pub fn normalize_email(raw: &str) -> Option<String> {
    let mut value = raw.trim();
    if value.is_empty() {
        return None;
    }
    if let Some(stripped) = value.strip_prefix("mailto:") {
        value = stripped.trim();
    }
    value = value.trim_matches(|ch: char| matches!(ch, '<' | '>' | '"' | '\'' | ',' | ';'));
    if !value.contains('@') {
        return None;
    }

    let mut parts = value.splitn(2, '@');
    let local = parts.next().unwrap_or("").trim();
    let domain = parts.next().unwrap_or("").trim();
    if local.is_empty() || domain.is_empty() {
        return None;
    }
    let local = local.split('+').next().unwrap_or(local);

    Some(format!(
        "{}@{}",
        local.to_ascii_lowercase(),
        domain.to_ascii_lowercase()
    ))
}

pub fn extract_emails(raw: &str) -> Vec<String> {
    let mut emails = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let mut remainder = raw;
    while let Some(start) = remainder.find('<') {
        let after_start = &remainder[start + 1..];
        if let Some(end) = after_start.find('>') {
            let inside = &after_start[..end];
            if let Some(email) = normalize_email(inside) {
                if seen.insert(email.clone()) {
                    emails.push(email);
                }
            }
            remainder = &after_start[end + 1..];
        } else {
            break;
        }
    }

    for token in raw.split(|ch| matches!(ch, ',' | ';' | ' ' | '\t' | '\n' | '\r')) {
        if let Some(email) = normalize_email(token) {
            if seen.insert(email.clone()) {
                emails.push(email);
            }
        }
    }

    emails
}

impl MongoUserStore {
    fn new() -> Result<Self, UserStoreError> {
        let client =
            create_client_from_env().map_err(|err| UserStoreError::MongoConfig(err.to_string()))?;
        let db = database_from_env(&client);
        let users = db.collection::<Document>("users");
        ensure_index_compatible(
            &users,
            IndexModel::builder()
                .keys(doc! { "user_id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
        )?;
        ensure_index_compatible(
            &users,
            IndexModel::builder()
                .keys(doc! { "identifier_type": 1, "identifier": 1 })
                .build(),
        )?;
        ensure_index_compatible(
            &users,
            IndexModel::builder().keys(doc! { "created_at": 1 }).build(),
        )?;
        Ok(Self { users })
    }

    fn get_user_by_identifier(
        &self,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<Option<UserRecord>, UserStoreError> {
        let normalized = normalize_identifier(identifier_type, identifier)
            .ok_or_else(|| UserStoreError::InvalidIdentifier(identifier.to_string()))?;
        let doc = self
            .users
            .find_one(
                doc! {
                    "identifier_type": identifier_type,
                    "identifier": normalized.as_str(),
                },
                None,
            )?
            .map(document_to_user_record)
            .transpose()?;
        Ok(doc)
    }

    fn get_or_create_user(
        &self,
        identifier_type: &str,
        identifier: &str,
    ) -> Result<UserRecord, UserStoreError> {
        let normalized = normalize_identifier(identifier_type, identifier)
            .ok_or_else(|| UserStoreError::InvalidIdentifier(identifier.to_string()))?;
        let now = Utc::now();
        let filter = doc! {
            "identifier_type": identifier_type,
            "identifier": normalized.as_str(),
        };

        if let Some(existing) = self.users.find_one(filter.clone(), None)? {
            let mut record = document_to_user_record(existing)?;
            if should_refresh_last_seen(record.last_seen_at, now) {
                self.users.update_one(
                    doc! {
                        "user_id": record.user_id.as_str(),
                    },
                    doc! { "$set": { "last_seen_at": BsonDateTime::from_chrono(now) } },
                    None,
                )?;
                record.last_seen_at = now;
            }
            return Ok(record);
        }

        let new_user_id = Uuid::new_v4().to_string();
        let insert_result = self.users.insert_one(
            doc! {
                "user_id": new_user_id.as_str(),
                "identifier_type": identifier_type,
                "identifier": normalized.as_str(),
                "created_at": BsonDateTime::from_chrono(now),
                "last_seen_at": BsonDateTime::from_chrono(now),
            },
            None,
        );

        match insert_result {
            Ok(_) => Ok(UserRecord {
                user_id: new_user_id,
                identifier_type: identifier_type.to_string(),
                identifier: normalized,
                created_at: now,
                last_seen_at: now,
            }),
            Err(err) => {
                if let Some(existing) = self.users.find_one(filter, None)? {
                    return document_to_user_record(existing);
                }
                Err(err.into())
            }
        }
    }

    fn list_user_ids(&self) -> Result<Vec<String>, UserStoreError> {
        let mut ids = Vec::new();
        let cursor = self.users.find(
            doc! {},
            FindOptions::builder()
                .sort(doc! { "created_at": 1 })
                .projection(doc! { "user_id": 1 })
                .build(),
        )?;
        for row in cursor {
            let document = row?;
            if let Ok(value) = document.get_str("user_id") {
                ids.push(value.to_string());
            }
        }
        Ok(ids)
    }
}

fn document_to_user_record(document: Document) -> Result<UserRecord, UserStoreError> {
    let user_id = document
        .get_str("user_id")
        .map_err(|err| UserStoreError::MongoConfig(format!("missing user_id: {err}")))?
        .to_string();
    let identifier_type = document
        .get_str("identifier_type")
        .map_err(|err| UserStoreError::MongoConfig(format!("missing identifier_type: {err}")))?
        .to_string();
    let identifier = document
        .get_str("identifier")
        .map_err(|err| UserStoreError::MongoConfig(format!("missing identifier: {err}")))?
        .to_string();
    let created_at = bson_datetime_to_utc(&document, "created_at")?;
    let last_seen_at = bson_datetime_to_utc(&document, "last_seen_at")?;
    Ok(UserRecord {
        user_id,
        identifier_type,
        identifier,
        created_at,
        last_seen_at,
    })
}

fn bson_datetime_to_utc(document: &Document, key: &str) -> Result<DateTime<Utc>, UserStoreError> {
    match document.get(key) {
        Some(Bson::DateTime(value)) => Ok(value.to_chrono()),
        Some(Bson::String(value)) => parse_datetime(value).map_err(UserStoreError::from),
        _ => Err(UserStoreError::MongoConfig(format!(
            "missing datetime field: {}",
            key
        ))),
    }
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

fn should_refresh_last_seen(last_seen_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
    (now - last_seen_at).num_seconds() >= LAST_SEEN_UPDATE_INTERVAL_SECS
}

const LAST_SEEN_UPDATE_INTERVAL_SECS: i64 = 5 * 60;

use std::sync::Arc;

/// Lazy-initialized global UserStore
static USER_STORE: std::sync::OnceLock<Option<Arc<UserStore>>> = std::sync::OnceLock::new();

/// Get or initialize the global UserStore (returns None if not configured)
pub fn get_global_user_store() -> Option<Arc<UserStore>> {
    USER_STORE
        .get_or_init(|| {
            match UserStore::new("") {
                Ok(store) => {
                    tracing::info!("UserStore initialized for user lookups");
                    Some(Arc::new(store))
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize UserStore: {}", e);
                    None
                }
            }
        })
        .clone()
}

/// Look up filesystem user_id by identifier type and identifier.
/// Returns the user_id (UUID string) if found, None otherwise.
pub fn lookup_user_id_by_identifier(identifier_type: &str, identifier: &str) -> Option<String> {
    let store = get_global_user_store()?;
    match store.get_user_by_identifier(identifier_type, identifier) {
        Ok(Some(record)) => Some(record.user_id),
        Ok(None) => None,
        Err(e) => {
            tracing::warn!(
                "Failed to lookup user by identifier {}:{}: {}",
                identifier_type,
                identifier,
                e
            );
            None
        }
    }
}

#[cfg(test)]
mod tests;
