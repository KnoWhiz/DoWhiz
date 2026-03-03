//! Slack installation store for multi-workspace support.
//!
//! Stores OAuth tokens and bot user IDs per workspace (team).

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::options::FindOptions;
use mongodb::options::IndexOptions;
use mongodb::sync::Collection;
use mongodb::IndexModel;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;
use std::time::Duration;

use crate::mongo_store::{create_client_from_env, database_from_env};
use crate::storage_backend::StorageBackend;

/// A Slack workspace installation record.
#[derive(Debug, Clone)]
pub struct SlackInstallation {
    pub team_id: String,
    pub team_name: Option<String>,
    pub bot_token: String,
    pub bot_user_id: String,
    pub installed_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum SlackStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("installation not found for team: {0}")]
    NotFound(String),
    #[error("datetime parse error: {0}")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error("mongo config error: {0}")]
    MongoConfig(String),
}

/// Store for Slack workspace installations.
#[derive(Debug, Clone)]
pub struct SlackStore {
    path: PathBuf,
    mongo: Option<MongoSlackStore>,
}

#[derive(Debug, Clone)]
struct MongoSlackStore {
    installations: Collection<Document>,
}

impl SlackStore {
    /// Create a new SlackStore, initializing the database if needed.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, SlackStoreError> {
        let backend = StorageBackend::from_env();
        let mongo = if backend.uses_mongo() {
            Some(MongoSlackStore::new()?)
        } else {
            None
        };
        let store = Self {
            path: path.into(),
            mongo,
        };
        if store.mongo.is_none() {
            let _ = store.open()?;
        }
        Ok(store)
    }

    /// Save or update an installation for a workspace.
    pub fn upsert_installation(
        &self,
        installation: &SlackInstallation,
    ) -> Result<(), SlackStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.upsert_installation(installation);
        }
        let conn = self.open()?;
        conn.execute(
            "INSERT INTO slack_installations (team_id, team_name, bot_token, bot_user_id, installed_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(team_id) DO UPDATE SET
                team_name = excluded.team_name,
                bot_token = excluded.bot_token,
                bot_user_id = excluded.bot_user_id,
                installed_at = excluded.installed_at",
            params![
                installation.team_id,
                installation.team_name,
                installation.bot_token,
                installation.bot_user_id,
                format_datetime(installation.installed_at),
            ],
        )?;
        Ok(())
    }

    /// Get installation by team_id.
    pub fn get_installation(&self, team_id: &str) -> Result<SlackInstallation, SlackStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.get_installation(team_id);
        }
        let conn = self.open()?;
        let row = conn
            .query_row(
                "SELECT team_id, team_name, bot_token, bot_user_id, installed_at
                 FROM slack_installations
                 WHERE team_id = ?1",
                params![team_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                    ))
                },
            )
            .optional()?;

        match row {
            Some((team_id, team_name, bot_token, bot_user_id, installed_at)) => {
                Ok(SlackInstallation {
                    team_id,
                    team_name,
                    bot_token,
                    bot_user_id,
                    installed_at: parse_datetime(&installed_at)?,
                })
            }
            None => Err(SlackStoreError::NotFound(team_id.to_string())),
        }
    }

    /// Get installation by team_id, with fallback to environment variables.
    /// This provides backward compatibility with single-workspace setup.
    pub fn get_installation_or_env(
        &self,
        team_id: &str,
    ) -> Result<SlackInstallation, SlackStoreError> {
        match self.get_installation(team_id) {
            Ok(installation) => Ok(installation),
            Err(SlackStoreError::NotFound(_)) => {
                // Fallback to environment variables for backward compatibility
                let bot_token = std::env::var("SLACK_BOT_TOKEN")
                    .map_err(|_| SlackStoreError::NotFound(team_id.to_string()))?;
                let bot_user_id = std::env::var("SLACK_BOT_USER_ID").unwrap_or_default();

                Ok(SlackInstallation {
                    team_id: team_id.to_string(),
                    team_name: None,
                    bot_token,
                    bot_user_id,
                    installed_at: Utc::now(),
                })
            }
            Err(e) => Err(e),
        }
    }

    /// Delete an installation (e.g., when app is uninstalled).
    pub fn delete_installation(&self, team_id: &str) -> Result<bool, SlackStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.delete_installation(team_id);
        }
        let conn = self.open()?;
        let rows_affected = conn.execute(
            "DELETE FROM slack_installations WHERE team_id = ?1",
            params![team_id],
        )?;
        Ok(rows_affected > 0)
    }

    /// List all installations.
    pub fn list_installations(&self) -> Result<Vec<SlackInstallation>, SlackStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.list_installations();
        }
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT team_id, team_name, bot_token, bot_user_id, installed_at
             FROM slack_installations
             ORDER BY installed_at DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let mut installations = Vec::new();
        for row in rows {
            let (team_id, team_name, bot_token, bot_user_id, installed_at) = row?;
            installations.push(SlackInstallation {
                team_id,
                team_name,
                bot_token,
                bot_user_id,
                installed_at: parse_datetime(&installed_at)?,
            });
        }
        Ok(installations)
    }

    fn open(&self) -> Result<Connection, SlackStoreError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS slack_installations (
                team_id TEXT PRIMARY KEY,
                team_name TEXT,
                bot_token TEXT NOT NULL,
                bot_user_id TEXT NOT NULL,
                installed_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(conn)
    }
}

impl MongoSlackStore {
    fn new() -> Result<Self, SlackStoreError> {
        let client = create_client_from_env()
            .map_err(|err| SlackStoreError::MongoConfig(err.to_string()))?;
        let db = database_from_env(&client);
        let installations = db.collection::<Document>("slack_installations");
        installations.create_index(
            IndexModel::builder()
                .keys(doc! { "team_id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
            None,
        )?;
        installations.create_index(
            IndexModel::builder()
                .keys(doc! { "installed_at": -1 })
                .build(),
            None,
        )?;
        Ok(Self { installations })
    }

    fn upsert_installation(&self, installation: &SlackInstallation) -> Result<(), SlackStoreError> {
        self.installations.update_one(
            doc! { "team_id": installation.team_id.as_str() },
            doc! {
                "$set": {
                    "team_id": installation.team_id.as_str(),
                    "team_name": installation.team_name.clone().map(Bson::from).unwrap_or(Bson::Null),
                    "bot_token": installation.bot_token.as_str(),
                    "bot_user_id": installation.bot_user_id.as_str(),
                    "installed_at": BsonDateTime::from_chrono(installation.installed_at),
                }
            },
            mongodb::options::UpdateOptions::builder()
                .upsert(true)
                .build(),
        )?;
        Ok(())
    }

    fn get_installation(&self, team_id: &str) -> Result<SlackInstallation, SlackStoreError> {
        let document = self
            .installations
            .find_one(doc! { "team_id": team_id }, None)?
            .ok_or_else(|| SlackStoreError::NotFound(team_id.to_string()))?;
        document_to_installation(document)
    }

    fn delete_installation(&self, team_id: &str) -> Result<bool, SlackStoreError> {
        let result = self
            .installations
            .delete_one(doc! { "team_id": team_id }, None)?;
        Ok(result.deleted_count > 0)
    }

    fn list_installations(&self) -> Result<Vec<SlackInstallation>, SlackStoreError> {
        let mut values = Vec::new();
        let cursor = self.installations.find(
            doc! {},
            FindOptions::builder()
                .sort(doc! { "installed_at": -1 })
                .build(),
        )?;
        for row in cursor {
            values.push(document_to_installation(row?)?);
        }
        Ok(values)
    }
}

fn document_to_installation(document: Document) -> Result<SlackInstallation, SlackStoreError> {
    let team_id = document
        .get_str("team_id")
        .map_err(|err| SlackStoreError::MongoConfig(format!("missing team_id: {err}")))?
        .to_string();
    let team_name = match document.get("team_name") {
        Some(Bson::String(value)) => Some(value.to_string()),
        _ => None,
    };
    let bot_token = document
        .get_str("bot_token")
        .map_err(|err| SlackStoreError::MongoConfig(format!("missing bot_token: {err}")))?
        .to_string();
    let bot_user_id = document
        .get_str("bot_user_id")
        .map_err(|err| SlackStoreError::MongoConfig(format!("missing bot_user_id: {err}")))?
        .to_string();
    let installed_at = match document.get("installed_at") {
        Some(Bson::DateTime(value)) => value.to_chrono(),
        Some(Bson::String(value)) => parse_datetime(value)?,
        _ => {
            return Err(SlackStoreError::MongoConfig(
                "missing installed_at".to_string(),
            ))
        }
    };
    Ok(SlackInstallation {
        team_id,
        team_name,
        bot_token,
        bot_user_id,
        installed_at,
    })
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

    fn test_store() -> (TempDir, SlackStore) {
        let temp = TempDir::new().expect("tempdir");
        let path = temp.path().join("slack.db");
        let store = SlackStore::new(&path).expect("store");
        (temp, store)
    }

    #[test]
    fn upsert_and_get_installation() {
        let (_temp, store) = test_store();

        let installation = SlackInstallation {
            team_id: "T12345".to_string(),
            team_name: Some("Test Workspace".to_string()),
            bot_token: "xoxb-test-token".to_string(),
            bot_user_id: "U12345".to_string(),
            installed_at: Utc::now(),
        };

        store.upsert_installation(&installation).expect("upsert");

        let retrieved = store.get_installation("T12345").expect("get");
        assert_eq!(retrieved.team_id, "T12345");
        assert_eq!(retrieved.team_name, Some("Test Workspace".to_string()));
        assert_eq!(retrieved.bot_token, "xoxb-test-token");
        assert_eq!(retrieved.bot_user_id, "U12345");
    }

    #[test]
    fn upsert_updates_existing() {
        let (_temp, store) = test_store();

        let installation1 = SlackInstallation {
            team_id: "T12345".to_string(),
            team_name: Some("Old Name".to_string()),
            bot_token: "xoxb-old-token".to_string(),
            bot_user_id: "U12345".to_string(),
            installed_at: Utc::now(),
        };
        store.upsert_installation(&installation1).expect("upsert1");

        let installation2 = SlackInstallation {
            team_id: "T12345".to_string(),
            team_name: Some("New Name".to_string()),
            bot_token: "xoxb-new-token".to_string(),
            bot_user_id: "U67890".to_string(),
            installed_at: Utc::now(),
        };
        store.upsert_installation(&installation2).expect("upsert2");

        let retrieved = store.get_installation("T12345").expect("get");
        assert_eq!(retrieved.team_name, Some("New Name".to_string()));
        assert_eq!(retrieved.bot_token, "xoxb-new-token");
        assert_eq!(retrieved.bot_user_id, "U67890");
    }

    #[test]
    fn get_not_found() {
        let (_temp, store) = test_store();

        let result = store.get_installation("TNOTEXIST");
        assert!(matches!(result, Err(SlackStoreError::NotFound(_))));
    }

    #[test]
    fn delete_installation() {
        let (_temp, store) = test_store();

        let installation = SlackInstallation {
            team_id: "T12345".to_string(),
            team_name: None,
            bot_token: "xoxb-test".to_string(),
            bot_user_id: "U12345".to_string(),
            installed_at: Utc::now(),
        };
        store.upsert_installation(&installation).expect("upsert");

        let deleted = store.delete_installation("T12345").expect("delete");
        assert!(deleted);

        let result = store.get_installation("T12345");
        assert!(matches!(result, Err(SlackStoreError::NotFound(_))));
    }

    #[test]
    fn list_installations() {
        let (_temp, store) = test_store();

        for i in 1..=3 {
            let installation = SlackInstallation {
                team_id: format!("T{}", i),
                team_name: Some(format!("Workspace {}", i)),
                bot_token: format!("xoxb-token-{}", i),
                bot_user_id: format!("U{}", i),
                installed_at: Utc::now(),
            };
            store.upsert_installation(&installation).expect("upsert");
        }

        let list = store.list_installations().expect("list");
        assert_eq!(list.len(), 3);
    }
}
