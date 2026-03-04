use chrono::{Duration as ChronoDuration, Utc};
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::options::{FindOneOptions, FindOptions, UpdateOptions};
use mongodb::sync::Collection;
use mongodb::IndexModel;
use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};
use uuid::Uuid;

use crate::mongo_store::{create_client_from_env, database_from_env, ensure_index_compatible};

use super::super::types::{Schedule, ScheduledTask, SchedulerError};
use super::super::utils::{task_kind_channel, task_kind_label};
use super::TaskStatusSummary;

static EXECUTION_SEQ: AtomicI64 = AtomicI64::new(1);

#[derive(Debug)]
pub(crate) struct MongoSchedulerStore {
    tasks: Collection<Document>,
    executions: Collection<Document>,
    owner_kind: String,
    owner_id: String,
}

impl MongoSchedulerStore {
    pub(crate) fn new(tasks_db_path: &Path) -> Result<Self, SchedulerError> {
        let client = create_client_from_env().map_err(mongo_config_err)?;
        let db = database_from_env(&client);
        let (owner_kind, owner_id) = resolve_owner_scope(tasks_db_path);
        let tasks = db.collection::<Document>("tasks");
        ensure_index_compatible(
            &tasks,
            IndexModel::builder()
                .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "task_id": 1 })
                .build(),
        )
        .map_err(mongo_err)?;
        ensure_index_compatible(
            &tasks,
            IndexModel::builder()
                .keys(doc! { "owner_scope.kind": 1, "owner_scope.id": 1, "created_at": 1 })
                .build(),
        )
        .map_err(mongo_err)?;
        let executions = db.collection::<Document>("task_executions");
        ensure_index_compatible(
            &executions,
            IndexModel::builder()
                .keys(doc! {
                    "owner_scope.kind": 1,
                    "owner_scope.id": 1,
                    "task_id": 1,
                    "started_at": -1
                })
                .build(),
        )
        .map_err(mongo_err)?;
        Ok(Self {
            tasks,
            executions,
            owner_kind,
            owner_id,
        })
    }

    pub(crate) fn load_tasks(&self) -> Result<Vec<ScheduledTask>, SchedulerError> {
        let cursor = self
            .tasks
            .find(
                self.owner_filter(),
                FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .build(),
            )
            .map_err(mongo_err)?;
        let mut seen_task_ids = HashSet::new();
        let mut tasks = Vec::new();
        for row in cursor {
            let document = row.map_err(mongo_err)?;
            if let Ok(task_id) = document.get_str("task_id") {
                if !seen_task_ids.insert(task_id.to_string()) {
                    continue;
                }
            }
            let task_json = document.get_str("task_json").map_err(|err| {
                SchedulerError::Storage(format!("missing task_json for task document: {err}"))
            })?;
            let task: ScheduledTask = serde_json::from_str(task_json)
                .map_err(|err| SchedulerError::Storage(format!("invalid task_json: {err}")))?;
            tasks.push(task);
        }
        tasks.sort_by_key(|task| task.created_at);
        Ok(tasks)
    }

    pub(crate) fn insert_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let task_json = serde_json::to_string(task)
            .map_err(|err| SchedulerError::Storage(format!("serialize task failed: {err}")))?;
        self.tasks
            .update_one(
                self.task_filter(&task.id.to_string()),
                doc! {
                    "$set": {
                        "owner_scope": self.owner_scope_doc(),
                        "task_id": task.id.to_string(),
                        "kind": task_kind_label(&task.kind),
                        "channel": task_kind_channel(&task.kind).to_string(),
                        "enabled": task.enabled,
                        "created_at": BsonDateTime::from_chrono(task.created_at),
                        "last_run": task.last_run.map(BsonDateTime::from_chrono).map(Bson::DateTime).unwrap_or(Bson::Null),
                        "schedule": schedule_doc(&task.schedule),
                        "task_json": task_json,
                    },
                    "$setOnInsert": {
                        "retry_count": 0i32,
                    },
                },
                UpdateOptions::builder().upsert(Some(true)).build(),
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn update_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let task_json = serde_json::to_string(task)
            .map_err(|err| SchedulerError::Storage(format!("serialize task failed: {err}")))?;
        self.tasks
            .update_one(
                self.task_filter(&task.id.to_string()),
                doc! {
                    "$set": {
                        "enabled": task.enabled,
                        "last_run": task.last_run.map(BsonDateTime::from_chrono).map(Bson::DateTime).unwrap_or(Bson::Null),
                        "schedule": schedule_doc(&task.schedule),
                        "task_json": task_json,
                    }
                },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn record_execution_start(
        &self,
        task_id: Uuid,
        started_at: chrono::DateTime<Utc>,
    ) -> Result<i64, SchedulerError> {
        let execution_id = EXECUTION_SEQ.fetch_add(1, Ordering::Relaxed);
        self.executions
            .insert_one(
                doc! {
                    "owner_scope": self.owner_scope_doc(),
                    "execution_id": execution_id,
                    "task_id": task_id.to_string(),
                    "started_at": BsonDateTime::from_chrono(started_at),
                    "finished_at": Bson::Null,
                    "status": "running",
                    "error_message": Bson::Null,
                },
                None,
            )
            .map_err(mongo_err)?;
        Ok(execution_id)
    }

    pub(crate) fn record_execution_finish(
        &self,
        execution_id: i64,
        finished_at: chrono::DateTime<Utc>,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), SchedulerError> {
        self.executions
            .update_one(
                doc! {
                    "owner_scope.kind": &self.owner_kind,
                    "owner_scope.id": &self.owner_id,
                    "execution_id": execution_id,
                },
                doc! {
                    "$set": {
                        "finished_at": BsonDateTime::from_chrono(finished_at),
                        "status": status,
                        "error_message": error_message.map(Bson::from).unwrap_or(Bson::Null),
                    }
                },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn get_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        let document = self
            .tasks
            .find_one(self.task_filter(task_id), None)
            .map_err(mongo_err)?;
        Ok(document
            .as_ref()
            .and_then(|doc| numeric_field_to_u32(doc, "retry_count"))
            .unwrap_or(0))
    }

    pub(crate) fn increment_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        self.tasks
            .update_one(
                self.task_filter(task_id),
                doc! { "$inc": { "retry_count": 1i32 } },
                None,
            )
            .map_err(mongo_err)?;
        self.get_retry_count(task_id)
    }

    pub(crate) fn reset_retry_count(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.tasks
            .update_one(
                self.task_filter(task_id),
                doc! { "$set": { "retry_count": 0i32 } },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn disable_task_by_id(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.tasks
            .update_one(
                self.task_filter(task_id),
                doc! { "$set": { "enabled": false } },
                None,
            )
            .map_err(mongo_err)?;
        Ok(())
    }

    pub(crate) fn list_tasks_with_status(&self) -> Result<Vec<TaskStatusSummary>, SchedulerError> {
        let created_after = BsonDateTime::from_chrono(Utc::now() - ChronoDuration::hours(24));
        let cursor = self
            .tasks
            .find(
                doc! {
                    "owner_scope.kind": &self.owner_kind,
                    "owner_scope.id": &self.owner_id,
                    "created_at": { "$gte": created_after },
                },
                FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .build(),
            )
            .map_err(mongo_err)?;
        let mut summaries = Vec::new();
        let mut seen_task_ids = HashSet::new();
        for row in cursor {
            let task_doc = row.map_err(mongo_err)?;
            let task_id = task_doc
                .get_str("task_id")
                .map_err(|err| SchedulerError::Storage(format!("missing task_id: {err}")))?;
            if !seen_task_ids.insert(task_id.to_string()) {
                continue;
            }
            let schedule = task_doc.get_document("schedule").ok();
            let execution = self
                .executions
                .find_one(
                    doc! {
                        "owner_scope.kind": &self.owner_kind,
                        "owner_scope.id": &self.owner_id,
                        "task_id": task_id,
                    },
                    FindOneOptions::builder()
                        .sort(doc! { "started_at": -1 })
                        .build(),
                )
                .map_err(mongo_err)?;
            summaries.push(TaskStatusSummary {
                id: task_id.to_string(),
                kind: task_doc.get_str("kind").unwrap_or("unknown").to_string(),
                channel: task_doc.get_str("channel").unwrap_or("email").to_string(),
                enabled: task_doc.get_bool("enabled").unwrap_or(false),
                created_at: datetime_field_to_rfc3339(&task_doc, "created_at").unwrap_or_default(),
                last_run: datetime_field_to_rfc3339(&task_doc, "last_run"),
                schedule_type: schedule
                    .and_then(|doc| doc.get_str("type").ok())
                    .unwrap_or("one_shot")
                    .to_string(),
                next_run: schedule.and_then(|doc| datetime_field_to_rfc3339(doc, "next_run")),
                run_at: schedule.and_then(|doc| datetime_field_to_rfc3339(doc, "run_at")),
                execution_status: execution
                    .as_ref()
                    .and_then(|doc| doc.get_str("status").ok())
                    .map(|value| value.to_string()),
                error_message: execution.as_ref().and_then(|doc| {
                    doc.get_str("error_message")
                        .ok()
                        .map(|value| value.to_string())
                }),
                execution_started_at: execution
                    .as_ref()
                    .and_then(|doc| datetime_field_to_rfc3339(doc, "started_at")),
            });
        }
        Ok(summaries)
    }

    fn owner_filter(&self) -> Document {
        doc! {
            "owner_scope.kind": &self.owner_kind,
            "owner_scope.id": &self.owner_id,
        }
    }

    fn task_filter(&self, task_id: &str) -> Document {
        doc! {
            "owner_scope.kind": &self.owner_kind,
            "owner_scope.id": &self.owner_id,
            "task_id": task_id,
        }
    }

    fn owner_scope_doc(&self) -> Document {
        doc! {
            "kind": &self.owner_kind,
            "id": &self.owner_id,
        }
    }
}

fn schedule_doc(schedule: &Schedule) -> Document {
    match schedule {
        Schedule::Cron {
            expression,
            next_run,
        } => doc! {
            "type": "cron",
            "cron_expression": expression,
            "next_run": BsonDateTime::from_chrono(*next_run),
            "run_at": Bson::Null,
        },
        Schedule::OneShot { run_at } => doc! {
            "type": "one_shot",
            "cron_expression": Bson::Null,
            "next_run": Bson::Null,
            "run_at": BsonDateTime::from_chrono(*run_at),
        },
    }
}

fn resolve_owner_scope(path: &Path) -> (String, String) {
    let mut components: Vec<String> = Vec::new();
    for component in path.components() {
        if let Some(value) = component.as_os_str().to_str() {
            components.push(value.to_string());
        }
    }

    for (idx, value) in components.iter().enumerate() {
        if value == "users" {
            if let Some(owner_id) = components.get(idx + 1) {
                return ("user".to_string(), owner_id.to_string());
            }
        }
    }

    if path.file_name().and_then(|v| v.to_str()) == Some("tasks.db") {
        if let Some(state_dir) = path.parent() {
            if state_dir.file_name().and_then(|v| v.to_str()) == Some("state") {
                if let Some(owner_dir) = state_dir.parent() {
                    if let Some(owner_id) = owner_dir.file_name().and_then(|v| v.to_str()) {
                        return ("user".to_string(), owner_id.to_string());
                    }
                }
            }
        }
    }

    let hashed = format!("{:x}", md5::compute(path.to_string_lossy().as_bytes()));
    ("path_scope".to_string(), hashed)
}

fn datetime_field_to_rfc3339(document: &Document, key: &str) -> Option<String> {
    match document.get(key) {
        Some(Bson::DateTime(value)) => Some(value.to_chrono().to_rfc3339()),
        Some(Bson::String(value)) => Some(value.to_string()),
        _ => None,
    }
}

fn numeric_field_to_u32(document: &Document, key: &str) -> Option<u32> {
    match document.get(key) {
        Some(Bson::Int32(value)) if *value >= 0 => Some(*value as u32),
        Some(Bson::Int64(value)) if *value >= 0 => Some(*value as u32),
        _ => None,
    }
}

fn mongo_err(err: mongodb::error::Error) -> SchedulerError {
    SchedulerError::Storage(format!("mongodb error: {err}"))
}

fn mongo_config_err(err: crate::mongo_store::MongoStoreError) -> SchedulerError {
    SchedulerError::Storage(err.to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::resolve_owner_scope;

    #[test]
    fn resolve_owner_scope_extracts_user_id() {
        let path = PathBuf::from("/tmp/runtime/users/user-123/state/tasks.db");
        let scope = resolve_owner_scope(&path);
        assert_eq!(scope.0, "user");
        assert_eq!(scope.1, "user-123");
    }
}
