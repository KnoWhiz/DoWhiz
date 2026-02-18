use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::channel::Channel;

pub(crate) const RUN_TASK_FAILURE_LIMIT: u32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaskKind {
    /// Send a reply message (email, Slack, etc.)
    /// Note: Serializes as "send_email" for backward compatibility
    #[serde(rename = "send_email")]
    SendReply(SendReplyTask),
    RunTask(RunTaskTask),
    Noop,
}

/// Task for sending an outbound reply message to any channel.
///
/// Supports email (Postmark), Slack, Telegram, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendReplyTask {
    /// The channel to send this message on (defaults to Email for backward compat)
    #[serde(default)]
    pub channel: Channel,
    pub subject: String,
    pub html_path: PathBuf,
    pub attachments_dir: PathBuf,
    #[serde(default)]
    pub from: Option<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    #[serde(default)]
    pub in_reply_to: Option<String>,
    #[serde(default)]
    pub references: Option<String>,
    #[serde(default)]
    pub archive_root: Option<PathBuf>,
    #[serde(default)]
    pub thread_epoch: Option<u64>,
    #[serde(default)]
    pub thread_state_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTaskTask {
    pub workspace_dir: PathBuf,
    #[serde(alias = "input_email_path")]
    pub input_email_dir: PathBuf,
    pub input_attachments_dir: PathBuf,
    pub memory_dir: PathBuf,
    #[serde(alias = "references_dir")]
    pub reference_dir: PathBuf,
    pub model_name: String,
    #[serde(default = "default_runner")]
    pub runner: String,
    pub codex_disabled: bool,
    #[serde(default)]
    pub reply_to: Vec<String>,
    #[serde(default)]
    pub reply_from: Option<String>,
    #[serde(default)]
    pub archive_root: Option<PathBuf>,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub thread_epoch: Option<u64>,
    #[serde(default)]
    pub thread_state_path: Option<PathBuf>,
    /// The channel to reply on (Email, Slack, etc.)
    #[serde(default)]
    pub channel: Channel,
    /// Slack-specific: Team ID for routing replies
    #[serde(default)]
    pub slack_team_id: Option<String>,
    /// Employee ID for per-employee credentials (optional)
    #[serde(default)]
    pub employee_id: Option<String>,
}

fn default_runner() -> String {
    "codex".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Schedule {
    Cron {
        expression: String,
        next_run: DateTime<Utc>,
    },
    OneShot {
        run_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub id: Uuid,
    pub kind: TaskKind,
    pub schedule: Schedule,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_run: Option<DateTime<Utc>>,
}

impl ScheduledTask {
    pub(crate) fn is_due(&self, now: DateTime<Utc>) -> bool {
        match &self.schedule {
            Schedule::Cron { next_run, .. } => *next_run <= now,
            Schedule::OneShot { run_at } => *run_at <= now,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("datetime parse error: {0}")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error("uuid parse error: {0}")]
    UuidParse(#[from] uuid::Error),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("cron parse error: {0}")]
    Cron(#[from] cron::error::Error),
    #[error("invalid cron expression (expected 6 fields, got {0})")]
    InvalidCron(usize),
    #[error("no next run available for cron expression")]
    NoNextRun,
    #[error("duration out of range")]
    DurationOutOfRange,
    #[error("task execution failed: {0}")]
    TaskFailed(String),
}

#[derive(Debug, Default)]
pub struct TaskExecution {
    pub follow_up_tasks: Vec<run_task_module::ScheduledTaskRequest>,
    pub follow_up_error: Option<String>,
    pub scheduler_actions: Vec<run_task_module::SchedulerActionRequest>,
    pub scheduler_actions_error: Option<String>,
}

impl TaskExecution {
    pub(crate) fn empty() -> Self {
        Self::default()
    }
}

pub(crate) const RUN_TASK_FAILURE_NOTICE: &str = "We could not complete your request";
pub(crate) const RUN_TASK_FAILURE_DIR: &str = "failure_notifications";
pub(crate) const RUN_TASK_FAILURE_REPORT_DIR: &str = "dowhiz_failure_reports";
