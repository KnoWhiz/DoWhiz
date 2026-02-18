use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RunTaskParams {
    pub workspace_dir: PathBuf,
    pub input_email_dir: PathBuf,
    pub input_attachments_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub reference_dir: PathBuf,
    pub reply_to: Vec<String>,
    pub model_name: String,
    pub runner: String,
    pub codex_disabled: bool,
    /// Channel for the reply: "email", "slack", "telegram", etc.
    pub channel: String,
    /// Pre-generated Google access token (for sandbox environments without network access)
    pub google_access_token: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct RunTaskRequest<'a> {
    pub(super) workspace_dir: &'a Path,
    pub(super) input_email_dir: &'a Path,
    pub(super) input_attachments_dir: &'a Path,
    pub(super) memory_dir: &'a Path,
    pub(super) reference_dir: &'a Path,
    pub(super) model_name: &'a str,
    pub(super) reply_to: &'a [String],
    pub(super) channel: &'a str,
    pub(super) google_access_token: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScheduledTaskRequest {
    SendEmail(ScheduledSendEmailTask),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum SchedulerActionRequest {
    Cancel {
        task_ids: Vec<String>,
    },
    Reschedule {
        task_id: String,
        schedule: ScheduleRequest,
    },
    CreateRunTask {
        schedule: ScheduleRequest,
        #[serde(default)]
        model_name: Option<String>,
        #[serde(default)]
        codex_disabled: Option<bool>,
        #[serde(default)]
        reply_to: Vec<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScheduleRequest {
    Cron { expression: String },
    OneShot { run_at: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledSendEmailTask {
    pub subject: String,
    pub html_path: String,
    pub attachments_dir: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub delay_minutes: Option<i64>,
    pub delay_seconds: Option<i64>,
    pub run_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RunTaskOutput {
    pub reply_html_path: PathBuf,
    pub reply_attachments_dir: PathBuf,
    pub codex_output: String,
    pub scheduled_tasks: Vec<ScheduledTaskRequest>,
    pub scheduled_tasks_error: Option<String>,
    pub scheduler_actions: Vec<SchedulerActionRequest>,
    pub scheduler_actions_error: Option<String>,
}
