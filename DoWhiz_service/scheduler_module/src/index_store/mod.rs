use chrono::{DateTime, Utc};
use mongodb::bson::{doc, DateTime as BsonDateTime, Document};
use mongodb::error::ErrorKind;
use mongodb::options::FindOptions;
use mongodb::options::IndexOptions;
use mongodb::options::UpdateOptions;
use mongodb::sync::Collection;
use mongodb::IndexModel;
use std::collections::BTreeMap;
use std::path::PathBuf;
use tracing::warn;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};
use crate::{Schedule, ScheduledTask};

#[derive(Debug)]
pub struct IndexStore {
    mongo: MongoIndexStore,
}

#[derive(Debug, Clone)]
struct MongoIndexStore {
    task_index: Collection<Document>,
    scope: String,
}

#[derive(Debug, Clone)]
pub struct TaskRef {
    pub task_id: String,
    pub user_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum IndexStoreError {
    #[error("mongodb error: {0}")]
    Mongo(#[from] mongodb::error::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("mongo config error: {0}")]
    MongoConfig(String),
}

impl IndexStore {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, IndexStoreError> {
        let scope = scope_from_path(path.into());
        Ok(Self {
            mongo: MongoIndexStore::new(scope)?,
        })
    }

    pub fn sync_user_tasks(
        &self,
        user_id: &str,
        tasks: &[ScheduledTask],
    ) -> Result<(), IndexStoreError> {
        self.mongo.sync_user_tasks(user_id, tasks)
    }

    pub fn due_user_ids(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<String>, IndexStoreError> {
        self.mongo.due_user_ids(now, limit)
    }

    pub fn due_task_refs(
        &self,
        now: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<TaskRef>, IndexStoreError> {
        self.mongo.due_task_refs(now, limit)
    }
}

impl MongoIndexStore {
    fn new(scope: String) -> Result<Self, IndexStoreError> {
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
                .keys(doc! { "scope": 1, "enabled": 1, "next_run": 1 })
                .build(),
        )?;
        ensure_index_compatible(
            &task_index,
            IndexModel::builder()
                .keys(doc! { "scope": 1, "user_id": 1 })
                .build(),
        )?;
        Ok(Self { task_index, scope })
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
                .delete_many(doc! { "scope": &self.scope, "user_id": user_id }, None)?;
            return Ok(());
        }

        self.task_index.delete_many(
            doc! {
                "scope": &self.scope,
                "user_id": user_id,
                "task_id": { "$nin": task_ids.clone() },
            },
            None,
        )?;

        let options = UpdateOptions::builder().upsert(Some(true)).build();
        for (task_id, next_run) in task_rows {
            self.task_index.update_one(
                doc! { "scope": &self.scope, "task_id": &task_id, "user_id": user_id },
                doc! {
                    "$set": {
                        "next_run": BsonDateTime::from_chrono(next_run),
                        "enabled": true,
                    },
                    "$setOnInsert": {
                        "scope": &self.scope,
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
        let filter = doc! {
            "scope": &self.scope,
            "enabled": true,
            "next_run": { "$lte": BsonDateTime::from_chrono(now) },
        };
        let sorted_options = FindOptions::builder()
            .sort(doc! { "next_run": 1 })
            .limit(limit as i64)
            .build();
        let unsorted_limit = (limit as i64).saturating_mul(8).max(limit as i64);
        let unsorted_options = FindOptions::builder().limit(unsorted_limit).build();
        let cursor = match self.task_index.find(filter.clone(), sorted_options) {
            Ok(cursor) => cursor,
            Err(err) if is_order_by_index_excluded(&err) => {
                warn!(
                    "task_index next_run sort rejected by backend; falling back to unsorted due-user query"
                );
                self.task_index.find(filter, unsorted_options)?
            }
            Err(err) => return Err(err.into()),
        };
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
        let filter = doc! {
            "scope": &self.scope,
            "enabled": true,
            "next_run": { "$lte": BsonDateTime::from_chrono(now) },
        };
        let sorted_options = FindOptions::builder()
            .sort(doc! { "next_run": 1 })
            .limit(limit as i64)
            .build();
        let unsorted_options = FindOptions::builder().limit(limit as i64).build();
        let cursor = match self.task_index.find(filter.clone(), sorted_options) {
            Ok(cursor) => cursor,
            Err(err) if is_order_by_index_excluded(&err) => {
                warn!(
                    "task_index next_run sort rejected by backend; falling back to unsorted due-task query"
                );
                self.task_index.find(filter, unsorted_options)?
            }
            Err(err) => return Err(err.into()),
        };
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

fn scope_from_path(path: PathBuf) -> String {
    format!("{:x}", md5::compute(path.to_string_lossy().as_bytes()))
}

fn is_order_by_index_excluded(err: &mongodb::error::Error) -> bool {
    let ErrorKind::Command(command_error) = err.kind.as_ref() else {
        return false;
    };
    if command_error.code != 2 {
        return false;
    }
    command_error
        .message
        .to_ascii_lowercase()
        .contains("the index path corresponding to the specified order-by item is excluded")
}

#[cfg(test)]
mod tests;
