use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;

use super::types::{ScheduledTask, SchedulerError};

mod mongo;

use mongo::MongoSchedulerStore;

#[derive(Debug)]
pub(crate) struct SchedulerStore {
    mongo: MongoSchedulerStore,
}

impl SchedulerStore {
    pub(crate) fn new(path: PathBuf) -> Result<Self, SchedulerError> {
        Ok(Self {
            mongo: MongoSchedulerStore::new(&path)?,
        })
    }

    pub(crate) fn load_tasks(&self) -> Result<Vec<ScheduledTask>, SchedulerError> {
        self.mongo.load_tasks()
    }

    pub(crate) fn insert_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        self.mongo.insert_task(task)
    }

    pub(crate) fn update_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        self.mongo.update_task(task)
    }

    pub(crate) fn record_execution_start(
        &self,
        task_id: Uuid,
        started_at: DateTime<Utc>,
    ) -> Result<i64, SchedulerError> {
        self.mongo.record_execution_start(task_id, started_at)
    }

    pub(crate) fn record_execution_finish(
        &self,
        execution_id: i64,
        finished_at: DateTime<Utc>,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), SchedulerError> {
        self.mongo
            .record_execution_finish(execution_id, finished_at, status, error_message)
    }

    pub(crate) fn get_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        self.mongo.get_retry_count(task_id)
    }

    pub(crate) fn increment_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        self.mongo.increment_retry_count(task_id)
    }

    pub(crate) fn reset_retry_count(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.mongo.reset_retry_count(task_id)
    }

    pub(crate) fn disable_task_by_id(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.mongo.disable_task_by_id(task_id)
    }

    pub fn list_tasks_with_status(&self) -> Result<Vec<TaskStatusSummary>, SchedulerError> {
        self.mongo.list_tasks_with_status()
    }
}

/// Summary of a task with its latest execution status.
/// Used for API responses.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskStatusSummary {
    pub id: String,
    pub kind: String,
    pub channel: String,
    pub enabled: bool,
    pub created_at: String,
    pub last_run: Option<String>,
    pub schedule_type: String,
    pub next_run: Option<String>,
    pub run_at: Option<String>,
    /// Status from the latest execution: "running", "success", "failed", or None if never executed
    pub execution_status: Option<String>,
    pub error_message: Option<String>,
    pub execution_started_at: Option<String>,
}
