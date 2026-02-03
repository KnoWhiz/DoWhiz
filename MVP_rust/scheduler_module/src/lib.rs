use chrono::{DateTime, Local, Utc};
use cron::Schedule as CronSchedule;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TaskKind {
    SendEmail(SendEmailTask),
    RunTask(RunTaskTask),
    Noop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailTask {
    pub subject: String,
    pub html_path: PathBuf,
    pub attachments_dir: PathBuf,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
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
    pub codex_disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Schedule {
    Cron { expression: String, next_run: DateTime<Utc> },
    OneShot { run_at: DateTime<Utc> },
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

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
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

pub trait TaskExecutor {
    fn execute(&self, task: &TaskKind) -> Result<(), SchedulerError>;
}

#[derive(Debug, Default, Clone)]
pub struct ModuleExecutor;

impl TaskExecutor for ModuleExecutor {
    fn execute(&self, task: &TaskKind) -> Result<(), SchedulerError> {
        match task {
            TaskKind::SendEmail(task) => {
                let params = send_emails_module::SendEmailParams {
                    subject: task.subject.clone(),
                    html_path: task.html_path.clone(),
                    attachments_dir: task.attachments_dir.clone(),
                    to: task.to.clone(),
                    cc: task.cc.clone(),
                    bcc: task.bcc.clone(),
                };
                send_emails_module::send_email(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
                Ok(())
            }
            TaskKind::RunTask(task) => {
                let params = run_task_module::RunTaskParams {
                    workspace_dir: task.workspace_dir.clone(),
                    input_email_dir: task.input_email_dir.clone(),
                    input_attachments_dir: task.input_attachments_dir.clone(),
                    memory_dir: task.memory_dir.clone(),
                    reference_dir: task.reference_dir.clone(),
                    model_name: task.model_name.clone(),
                    codex_disabled: task.codex_disabled,
                };
                run_task_module::run_task(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
                Ok(())
            }
            TaskKind::Noop => Ok(()),
        }
    }
}

pub struct Scheduler<E: TaskExecutor> {
    storage_path: PathBuf,
    tasks: Vec<ScheduledTask>,
    executor: E,
}

impl<E: TaskExecutor> Scheduler<E> {
    pub fn load(storage_path: impl Into<PathBuf>, executor: E) -> Result<Self, SchedulerError> {
        let storage_path = storage_path.into();
        let tasks = load_tasks(&storage_path)?;
        Ok(Self {
            storage_path,
            tasks,
            executor,
        })
    }

    pub fn tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    pub fn add_cron_task(
        &mut self,
        expression: &str,
        kind: TaskKind,
    ) -> Result<Uuid, SchedulerError> {
        validate_cron_expression(expression)?;
        let now = Utc::now();
        let next_run = next_run_after(expression, now)?;

        let task = ScheduledTask {
            id: Uuid::new_v4(),
            kind,
            schedule: Schedule::Cron {
                expression: expression.to_string(),
                next_run,
            },
            enabled: true,
            created_at: now,
            last_run: None,
        };

        self.tasks.push(task);
        self.save()?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn add_one_shot_in(
        &mut self,
        delay: Duration,
        kind: TaskKind,
    ) -> Result<Uuid, SchedulerError> {
        let local_now = Local::now();
        let utc_now = local_now.with_timezone(&Utc);
        let chrono_delay = chrono::Duration::from_std(delay).map_err(|_| SchedulerError::DurationOutOfRange)?;
        let run_at = utc_now + chrono_delay;

        let task = ScheduledTask {
            id: Uuid::new_v4(),
            kind,
            schedule: Schedule::OneShot { run_at },
            enabled: true,
            created_at: utc_now,
            last_run: None,
        };

        self.tasks.push(task);
        self.save()?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn tick(&mut self) -> Result<(), SchedulerError> {
        let now = Utc::now();
        for index in 0..self.tasks.len() {
            if !self.tasks[index].enabled {
                continue;
            }
            if !self.tasks[index].is_due(now) {
                continue;
            }

            let task_kind = self.tasks[index].kind.clone();
            self.executor.execute(&task_kind)?;

            let executed_at = Utc::now();
            self.tasks[index].last_run = Some(executed_at);
            match &mut self.tasks[index].schedule {
                Schedule::Cron { expression, next_run } => {
                    *next_run = next_run_after(expression, executed_at)?;
                }
                Schedule::OneShot { .. } => {
                    self.tasks[index].enabled = false;
                }
            }
        }

        self.save()?;
        Ok(())
    }

    pub fn run_loop(
        &mut self,
        poll_interval: Duration,
        stop_flag: &AtomicBool,
    ) -> Result<(), SchedulerError> {
        while !stop_flag.load(Ordering::Relaxed) {
            self.tick()?;
            std::thread::sleep(poll_interval);
        }
        Ok(())
    }

    fn save(&self) -> Result<(), SchedulerError> {
        save_tasks(&self.storage_path, &self.tasks)
    }
}

impl ScheduledTask {
    fn is_due(&self, now: DateTime<Utc>) -> bool {
        match &self.schedule {
            Schedule::Cron { next_run, .. } => *next_run <= now,
            Schedule::OneShot { run_at } => *run_at <= now,
        }
    }
}

fn validate_cron_expression(expression: &str) -> Result<(), SchedulerError> {
    let fields = expression.split_whitespace().count();
    if fields != 6 {
        return Err(SchedulerError::InvalidCron(fields));
    }
    Ok(())
}

fn next_run_after(expression: &str, after: DateTime<Utc>) -> Result<DateTime<Utc>, SchedulerError> {
    validate_cron_expression(expression)?;
    let schedule = CronSchedule::from_str(expression)?;
    for datetime in schedule.upcoming(Utc) {
        if datetime > after {
            return Ok(datetime);
        }
    }
    Err(SchedulerError::NoNextRun)
}

fn load_tasks(path: &Path) -> Result<Vec<ScheduledTask>, SchedulerError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    if data.trim().is_empty() {
        return Ok(Vec::new());
    }
    Ok(serde_json::from_str(&data)?)
}

fn save_tasks(path: &Path, tasks: &[ScheduledTask]) -> Result<(), SchedulerError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let payload = serde_json::to_string_pretty(tasks)?;
    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, payload)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}
