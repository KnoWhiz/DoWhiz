use crate::memory_store::ensure_default_user_memo;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserStore {
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct UserRecord {
    pub user_id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UserPaths {
    pub root: PathBuf,
    pub state_dir: PathBuf,
    pub tasks_db_path: PathBuf,
    pub memory_dir: PathBuf,
    pub mail_root: PathBuf,
    pub workspaces_root: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum UserStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid email: {0}")]
    InvalidEmail(String),
    #[error("datetime parse error: {0}")]
    DateTimeParse(#[from] chrono::ParseError),
}

impl UserStore {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, UserStoreError> {
        let store = Self { path: path.into() };
        let _ = store.open()?;
        Ok(store)
    }

    pub fn get_or_create_user(&self, email: &str) -> Result<UserRecord, UserStoreError> {
        let normalized = normalize_email(email)
            .ok_or_else(|| UserStoreError::InvalidEmail(email.to_string()))?;
        let conn = self.open()?;
        let now = Utc::now();
        let row = conn
            .query_row(
                "SELECT id, email, created_at, last_seen_at
                 FROM users
                 WHERE email = ?1",
                params![normalized.as_str()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                },
            )
            .optional()?;

        if let Some((id, email, created_at, _last_seen_at)) = row {
            let last_seen_at = now;
            conn.execute(
                "UPDATE users SET last_seen_at = ?1 WHERE id = ?2",
                params![format_datetime(last_seen_at), id],
            )?;
            return Ok(UserRecord {
                user_id: id,
                email,
                created_at: parse_datetime(&created_at)?,
                last_seen_at,
            });
        }

        let user_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO users (id, email, created_at, last_seen_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                user_id.as_str(),
                normalized.as_str(),
                format_datetime(now),
                format_datetime(now)
            ],
        )?;
        Ok(UserRecord {
            user_id,
            email: normalized,
            created_at: now,
            last_seen_at: now,
        })
    }

    pub fn list_user_ids(&self) -> Result<Vec<String>, UserStoreError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare("SELECT id FROM users ORDER BY created_at")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut user_ids = Vec::new();
        for row in rows {
            user_ids.push(row?);
        }
        Ok(user_ids)
    }

    pub fn user_paths(&self, users_root: &Path, user_id: &str) -> UserPaths {
        let root = users_root.join(user_id);
        let state_dir = root.join("state");
        let tasks_db_path = state_dir.join("tasks.db");
        let memory_dir = root.join("memory");
        let mail_root = root.join("mail");
        let workspaces_root = root.join("workspaces");
        UserPaths {
            root,
            state_dir,
            tasks_db_path,
            memory_dir,
            mail_root,
            workspaces_root,
        }
    }

    pub fn ensure_user_dirs(&self, paths: &UserPaths) -> Result<(), UserStoreError> {
        fs::create_dir_all(&paths.state_dir)?;
        fs::create_dir_all(&paths.memory_dir)?;
        fs::create_dir_all(&paths.mail_root)?;
        fs::create_dir_all(&paths.workspaces_root)?;
        ensure_default_user_memo(&paths.memory_dir)?;
        Ok(())
    }

    fn open(&self) -> Result<Connection, UserStoreError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.execute_batch(USERS_SCHEMA)?;
        Ok(conn)
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

const USERS_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    last_seen_at TEXT NOT NULL
);
"#;

fn format_datetime(value: DateTime<Utc>) -> String {
    value.to_rfc3339()
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

#[cfg(test)]
mod tests;
