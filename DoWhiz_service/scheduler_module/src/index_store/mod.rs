use chrono::{DateTime, Utc};
use mongodb::bson::{doc, DateTime as BsonDateTime, Document};
use mongodb::options::FindOptions;
use mongodb::options::IndexOptions;
use mongodb::options::UpdateOptions;
use mongodb::sync::Collection;
use mongodb::IndexModel;
use rusqlite::{params, Connection};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};
use crate::storage_backend::StorageBackend;
use crate::{Schedule, ScheduledTask};

#[derive(Debug)]
pub struct IndexStore {
    path: PathBuf,
    mongo: Option<MongoIndexStore>,
}

#[derive(Debug, Clone)]
struct MongoIndexStore {
    task_index: Collection<Document>,
}

#[derive(Debug, Clone)]
pub struct TaskRef {
    pub task_id: String,
    pub user_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum IndexStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("mongo config error: {0}")]
    MongoConfig(String),
}

impl IndexStore {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, IndexStoreError> {
        let backend = StorageBackend::from_env();
        let mongo = if backend.uses_mongo() {
            Some(MongoIndexStore::new()?)
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

    pub fn sync_user_tasks(
        &self,
        user_id: &str,
        tasks: &[ScheduledTask],
    ) -> Result<(), IndexStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.sync_user_tasks(user_id, tasks);
        }
        let mut conn = self.open()?;
        let tx = conn.transaction()?;
        tx.execute(
            "DELETE FROM task_index WHERE user_id = ?1",
            params![user_id],
        )?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO task_index (task_id, user_id, next_run, enabled)
                 VALUES (?1, ?2, ?3, ?4)",
            )?;
            for (task_id, next_run) in enabled_task_next_runs(tasks) {
                stmt.execute(params![task_id, user_id, format_datetime(&next_run), 1i64])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn due_user_ids(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<String>, IndexStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.due_user_ids(now, limit);
        }
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT DISTINCT user_id
             FROM task_index
             WHERE enabled = 1 AND next_run <= ?1
             ORDER BY next_run
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![format_datetime(&now), limit as i64], |row| {
            row.get::<_, String>(0)
        })?;
        let mut user_ids = Vec::new();
        for row in rows {
            user_ids.push(row?);
        }
        Ok(user_ids)
    }

    pub fn due_task_refs(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<TaskRef>, IndexStoreError> {
        if let Some(mongo) = &self.mongo {
            return mongo.due_task_refs(now, limit);
        }
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT task_id, user_id
             FROM task_index
             WHERE enabled = 1 AND next_run <= ?1
             ORDER BY next_run
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![format_datetime(&now), limit as i64], |row| {
            Ok(TaskRef {
                task_id: row.get::<_, String>(0)?,
                user_id: row.get::<_, String>(1)?,
            })
        })?;
        let mut task_refs = Vec::new();
        for row in rows {
            task_refs.push(row?);
        }
        Ok(task_refs)
    }

    fn open(&self) -> Result<Connection, IndexStoreError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(30))?;
        conn.execute_batch(INDEX_SCHEMA)?;
        Ok(conn)
    }
}

impl MongoIndexStore {
    fn new() -> Result<Self, IndexStoreError> {
        let client = create_client_from_env()
            .map_err(|err| IndexStoreError::MongoConfig(err.to_string()))?;
        let db = database_from_env(&client);
        let task_index = db.collection::<Document>("task_index");
        // user_id must come first for sharded collections (Cosmos DB shard key compatibility)
        ensure_index_compatible(
            &task_index,
            IndexModel::builder()
                .keys(doc! { "user_id": 1, "task_id": 1 })
                .options(IndexOptions::builder().unique(Some(true)).build())
                .build(),
        )?;
        ensure_index_compatible(
            &task_index,
            IndexModel::builder()
                .keys(doc! { "enabled": 1, "next_run": 1 })
                .build(),
        )?;
        ensure_index_compatible(
            &task_index,
            IndexModel::builder().keys(doc! { "user_id": 1 }).build(),
        )?;
        Ok(Self { task_index })
    }

    fn sync_user_tasks(
        &self,
        user_id: &str,
        tasks: &[ScheduledTask],
    ) -> Result<(), IndexStoreError> {
        let task_rows = enabled_task_next_runs(tasks);
        let task_ids: Vec<String> = task_rows
            .iter()
            .map(|(task_id, _)| task_id.clone())
            .collect();

        if task_ids.is_empty() {
            self.task_index
                .delete_many(doc! { "user_id": user_id }, None)?;
            return Ok(());
        }

        self.task_index.delete_many(
            doc! {
                "user_id": user_id,
                "task_id": { "$nin": task_ids.clone() },
            },
            None,
        )?;

        let options = UpdateOptions::builder().upsert(Some(true)).build();
        for (task_id, next_run) in task_rows {
            self.task_index.update_one(
                doc! { "task_id": &task_id, "user_id": user_id },
                doc! {
                    "$set": {
                        "next_run": BsonDateTime::from_chrono(next_run),
                        "enabled": true,
                    },
                    "$setOnInsert": {
                        "task_id": &task_id,
                        "user_id": user_id,
                    },
                },
                options.clone(),
            )?;
        }

        Ok(())
    }

    fn due_user_ids(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<String>, IndexStoreError> {
        let cursor = self.task_index.find(
            doc! {
                "enabled": true,
                "next_run": { "$lte": BsonDateTime::from_chrono(now) },
            },
            FindOptions::builder()
                .sort(doc! { "next_run": 1 })
                .limit(limit as i64)
                .build(),
        )?;
        let mut ids = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for row in cursor {
            let doc = row?;
            if let Ok(user_id) = doc.get_str("user_id") {
                if seen.insert(user_id.to_string()) {
                    ids.push(user_id.to_string());
                    if ids.len() >= limit {
                        break;
                    }
                }
            }
        }
        Ok(ids)
    }

    fn due_task_refs(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<TaskRef>, IndexStoreError> {
        let cursor = self.task_index.find(
            doc! {
                "enabled": true,
                "next_run": { "$lte": BsonDateTime::from_chrono(now) },
            },
            FindOptions::builder()
                .sort(doc! { "next_run": 1 })
                .limit(limit as i64)
                .build(),
        )?;
        let mut refs = Vec::new();
        for row in cursor {
            let doc = row?;
            let task_id = match doc.get_str("task_id") {
                Ok(value) => value.to_string(),
                Err(_) => continue,
            };
            let user_id = match doc.get_str("user_id") {
                Ok(value) => value.to_string(),
                Err(_) => continue,
            };
            refs.push(TaskRef { task_id, user_id });
        }
        Ok(refs)
    }
}

const INDEX_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS task_index (
    task_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    next_run TEXT NOT NULL,
    enabled INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_task_index_next_run ON task_index (next_run);
CREATE INDEX IF NOT EXISTS idx_task_index_user_id ON task_index (user_id);
"#;

fn format_datetime(value: &DateTime<Utc>) -> String {
    value.to_rfc3339()
}

fn enabled_task_next_runs(tasks: &[ScheduledTask]) -> Vec<(String, DateTime<Utc>)> {
    let mut deduped: BTreeMap<String, DateTime<Utc>> = BTreeMap::new();
    for task in tasks {
        if !task.enabled {
            continue;
        }
        let next_run = match &task.schedule {
            Schedule::Cron { next_run, .. } => *next_run,
            Schedule::OneShot { run_at } => *run_at,
        };
        deduped.insert(task.id.to_string(), next_run);
    }
    deduped.into_iter().collect()
}

#[cfg(test)]
mod tests;
