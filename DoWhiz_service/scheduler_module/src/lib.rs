use chrono::{DateTime, Local, Utc};
use cron::Schedule as CronSchedule;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

pub(crate) mod thread_state;
use crate::thread_state::{current_thread_epoch, default_thread_state_path, find_thread_state_path};
use crate::memory_store::{resolve_user_memory_dir, sync_user_memory_to_workspace, sync_workspace_memory_to_user};

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
    pub codex_disabled: bool,
    #[serde(default)]
    pub reply_to: Vec<String>,
    #[serde(default)]
    pub archive_root: Option<PathBuf>,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub thread_epoch: Option<u64>,
    #[serde(default)]
    pub thread_state_path: Option<PathBuf>,
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
}

impl TaskExecution {
    fn empty() -> Self {
        Self::default()
    }
}

pub trait TaskExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError>;
}

#[derive(Debug, Default, Clone)]
pub struct ModuleExecutor;

impl TaskExecutor for ModuleExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        match task {
            TaskKind::SendEmail(task) => {
                if let Some(expected_epoch) = task.thread_epoch {
                    let state_path = task
                        .thread_state_path
                        .clone()
                        .or_else(|| task.html_path.parent().and_then(find_thread_state_path));
                    if let Some(state_path) = state_path {
                        if let Some(current_epoch) = current_thread_epoch(&state_path) {
                            if current_epoch != expected_epoch {
                                info!(
                                    "skip stale send_email (expected epoch {}, current {}) for {}",
                                    expected_epoch,
                                    current_epoch,
                                    task.html_path.display()
                                );
                                return Ok(TaskExecution::empty());
                            }
                        }
                    }
                }
                let params = send_emails_module::SendEmailParams {
                    subject: task.subject.clone(),
                    html_path: task.html_path.clone(),
                    attachments_dir: task.attachments_dir.clone(),
                    to: task.to.clone(),
                    cc: task.cc.clone(),
                    bcc: task.bcc.clone(),
                    in_reply_to: task.in_reply_to.clone(),
                    references: task.references.clone(),
                };
                let response = send_emails_module::send_email(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
                if let Some(archive_root) = task.archive_root.as_ref() {
                    dotenvy::dotenv().ok();
                    let mut from =
                        env::var("OUTBOUND_FROM").unwrap_or_else(|_| "oliver@dowhiz.com".to_string());
                    if from.trim().is_empty() {
                        from = "oliver@dowhiz.com".to_string();
                    }
                    if let Err(err) = crate::past_emails::archive_outbound(
                        archive_root,
                        &task.subject,
                        &task.html_path,
                        &task.attachments_dir,
                        &task.to,
                        &task.cc,
                        &task.bcc,
                        task.in_reply_to.as_deref(),
                        task.references.as_deref(),
                        &response.message_id,
                        &response.submitted_at,
                        &from,
                    ) {
                        warn!("failed to archive outbound email: {}", err);
                    }
                }
                Ok(TaskExecution::empty())
            }
            TaskKind::RunTask(task) => {
                let workspace_memory_dir = task.workspace_dir.join(&task.memory_dir);
                let user_memory_dir = resolve_user_memory_dir(task);
                if let Some(user_memory_dir) = user_memory_dir.as_ref() {
                    sync_user_memory_to_workspace(user_memory_dir, &workspace_memory_dir)
                        .map_err(|err| SchedulerError::TaskFailed(format!("memory sync failed: {}", err)))?;
                } else {
                    warn!(
                        "unable to resolve user memory dir for workspace {}",
                        task.workspace_dir.display()
                    );
                }
                let params = run_task_module::RunTaskParams {
                    workspace_dir: task.workspace_dir.clone(),
                    input_email_dir: task.input_email_dir.clone(),
                    input_attachments_dir: task.input_attachments_dir.clone(),
                    memory_dir: task.memory_dir.clone(),
                    reference_dir: task.reference_dir.clone(),
                    model_name: task.model_name.clone(),
                    codex_disabled: task.codex_disabled,
                };
                let output = run_task_module::run_task(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
                if let Some(user_memory_dir) = user_memory_dir.as_ref() {
                    sync_workspace_memory_to_user(&workspace_memory_dir, user_memory_dir)
                        .map_err(|err| SchedulerError::TaskFailed(format!("memory sync failed: {}", err)))?;
                }
                Ok(TaskExecution {
                    follow_up_tasks: output.scheduled_tasks,
                    follow_up_error: output.scheduled_tasks_error,
                })
            }
            TaskKind::Noop => Ok(TaskExecution::empty()),
        }
    }
}

pub struct Scheduler<E: TaskExecutor> {
    tasks: Vec<ScheduledTask>,
    executor: E,
    store: SqliteSchedulerStore,
}

impl<E: TaskExecutor> Scheduler<E> {
    pub fn load(storage_path: impl Into<PathBuf>, executor: E) -> Result<Self, SchedulerError> {
        let storage_path = storage_path.into();
        let store = SqliteSchedulerStore::new(storage_path)?;
        let tasks = store.load_tasks()?;
        Ok(Self {
            tasks,
            executor,
            store,
        })
    }

    pub fn tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    pub fn disable_tasks_by<F>(&mut self, mut predicate: F) -> Result<usize, SchedulerError>
    where
        F: FnMut(&ScheduledTask) -> bool,
    {
        let mut disabled = 0usize;
        for task in &mut self.tasks {
            if !task.enabled {
                continue;
            }
            if predicate(task) {
                task.enabled = false;
                self.store.update_task(task)?;
                disabled += 1;
            }
        }
        Ok(disabled)
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
        self.store.insert_task(self.tasks.last().unwrap())?;
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
        self.store.insert_task(self.tasks.last().unwrap())?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn add_one_shot_at(
        &mut self,
        run_at: DateTime<Utc>,
        kind: TaskKind,
    ) -> Result<Uuid, SchedulerError> {
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            kind,
            schedule: Schedule::OneShot { run_at },
            enabled: true,
            created_at: Utc::now(),
            last_run: None,
        };

        self.tasks.push(task);
        self.store.insert_task(self.tasks.last().unwrap())?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn execute_task_by_id(&mut self, task_id: Uuid) -> Result<bool, SchedulerError> {
        let now = Utc::now();
        let index = match self.tasks.iter().position(|task| task.id == task_id) {
            Some(index) => index,
            None => return Ok(false),
        };
        if !self.tasks[index].enabled || !self.tasks[index].is_due(now) {
            return Ok(false);
        }
        self.execute_task_at_index(index)?;
        Ok(true)
    }

    pub fn tick(&mut self) -> Result<(), SchedulerError> {
        let now = Utc::now();
        let task_count = self.tasks.len();
        for index in 0..task_count {
            if !self.tasks[index].enabled {
                continue;
            }
            if !self.tasks[index].is_due(now) {
                continue;
            }
            self.execute_task_at_index(index)?;
        }

        Ok(())
    }

    fn execute_task_at_index(&mut self, index: usize) -> Result<(), SchedulerError> {
        let task_id = self.tasks[index].id;
        let task_kind = self.tasks[index].kind.clone();
        let started_at = Utc::now();
        let execution_id = self.store.record_execution_start(task_id, started_at)?;
        let result = self.executor.execute(&task_kind);
        let executed_at = Utc::now();

        match result {
            Ok(execution) => {
                self.store.record_execution_finish(execution_id, executed_at, "success", None)?;
                self.tasks[index].last_run = Some(executed_at);
                match &mut self.tasks[index].schedule {
                    Schedule::Cron { expression, next_run } => {
                        *next_run = next_run_after(expression, executed_at)?;
                    }
                    Schedule::OneShot { .. } => {
                        self.tasks[index].enabled = false;
                    }
                }
                let updated_task = self.tasks[index].clone();
                self.store.update_task(&updated_task)?;
                if let TaskKind::RunTask(task) = &task_kind {
                    if let Some(err) = execution.follow_up_error.as_deref() {
                        warn!("scheduled tasks parse error: {}", err);
                    }
                    if let Err(err) = snapshot_reply_draft(task) {
                        warn!(
                            "failed to snapshot reply draft for {}: {}",
                            task.workspace_dir.display(),
                            err
                        );
                    }
                    ingest_follow_up_tasks(self, task, &execution.follow_up_tasks);
                    if let Err(err) = schedule_auto_reply(self, task) {
                        warn!(
                            "failed to schedule auto reply from {}: {}",
                            task.workspace_dir.display(),
                            err
                        );
                    }
                }
            }
            Err(err) => {
                let message = err.to_string();
                self.store.record_execution_finish(
                    execution_id,
                    executed_at,
                    "failed",
                    Some(&message),
                )?;
                return Err(err);
            }
        }

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

const SCHEDULER_SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_run TEXT,
    schedule_type TEXT NOT NULL,
    cron_expression TEXT,
    next_run TEXT,
    run_at TEXT
);

CREATE TABLE IF NOT EXISTS send_email_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    subject TEXT NOT NULL,
    html_path TEXT NOT NULL,
    attachments_dir TEXT NOT NULL,
    in_reply_to TEXT,
    references_header TEXT,
    archive_root TEXT,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS send_email_recipients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    recipient_type TEXT NOT NULL,
    address TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS run_task_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    workspace_dir TEXT NOT NULL,
    input_email_dir TEXT NOT NULL,
    input_attachments_dir TEXT NOT NULL,
    memory_dir TEXT NOT NULL,
    reference_dir TEXT NOT NULL,
    model_name TEXT NOT NULL,
    codex_disabled INTEGER NOT NULL,
    reply_to TEXT NOT NULL,
    archive_root TEXT,
    thread_id TEXT,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS task_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    status TEXT NOT NULL,
    error_message TEXT
);
"#;

fn ensure_send_email_task_columns(conn: &Connection) -> Result<(), SchedulerError> {
    let mut stmt = conn.prepare("PRAGMA table_info(send_email_tasks)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let mut columns = HashSet::new();
    for row in rows {
        columns.insert(row?);
    }

    if !columns.contains("in_reply_to") {
        conn.execute(
            "ALTER TABLE send_email_tasks ADD COLUMN in_reply_to TEXT",
            [],
        )?;
    }
    if !columns.contains("references_header") {
        conn.execute(
            "ALTER TABLE send_email_tasks ADD COLUMN references_header TEXT",
            [],
        )?;
    }
    if !columns.contains("archive_root") {
        conn.execute(
            "ALTER TABLE send_email_tasks ADD COLUMN archive_root TEXT",
            [],
        )?;
    }
    if !columns.contains("thread_epoch") {
        conn.execute(
            "ALTER TABLE send_email_tasks ADD COLUMN thread_epoch INTEGER",
            [],
        )?;
    }
    if !columns.contains("thread_state_path") {
        conn.execute(
            "ALTER TABLE send_email_tasks ADD COLUMN thread_state_path TEXT",
            [],
        )?;
    }
    Ok(())
}

fn ensure_run_task_task_columns(conn: &Connection) -> Result<(), SchedulerError> {
    let mut stmt = conn.prepare("PRAGMA table_info(run_task_tasks)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let mut columns = HashSet::new();
    for row in rows {
        columns.insert(row?);
    }

    if !columns.contains("archive_root") {
        conn.execute(
            "ALTER TABLE run_task_tasks ADD COLUMN archive_root TEXT",
            [],
        )?;
    }
    if !columns.contains("thread_id") {
        conn.execute(
            "ALTER TABLE run_task_tasks ADD COLUMN thread_id TEXT",
            [],
        )?;
    }
    if !columns.contains("thread_epoch") {
        conn.execute(
            "ALTER TABLE run_task_tasks ADD COLUMN thread_epoch INTEGER",
            [],
        )?;
    }
    if !columns.contains("thread_state_path") {
        conn.execute(
            "ALTER TABLE run_task_tasks ADD COLUMN thread_state_path TEXT",
            [],
        )?;
    }
    Ok(())
}

#[derive(Debug)]
struct SqliteSchedulerStore {
    path: PathBuf,
}

impl SqliteSchedulerStore {
    fn new(path: PathBuf) -> Result<Self, SchedulerError> {
        let store = Self { path };
        let _ = store.open()?;
        Ok(store)
    }

    fn load_tasks(&self) -> Result<Vec<ScheduledTask>, SchedulerError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT id, kind, enabled, created_at, last_run, schedule_type, cron_expression, next_run, run_at
             FROM tasks
             ORDER BY created_at",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
            ))
        })?;

        let mut tasks = Vec::new();
        for row in rows {
            let (
                id_raw,
                kind_raw,
                enabled_raw,
                created_at_raw,
                last_run_raw,
                schedule_type,
                cron_expression,
                next_run_raw,
                run_at_raw,
            ) = row?;
            let id = Uuid::parse_str(&id_raw)?;
            let created_at = parse_datetime(&created_at_raw)?;
            let last_run = parse_optional_datetime(last_run_raw.as_deref())?;
            let schedule = match schedule_type.as_str() {
                "cron" => {
                    let expression = cron_expression.ok_or_else(|| {
                        SchedulerError::Storage(format!(
                            "missing cron expression for task {}",
                            id_raw
                        ))
                    })?;
                    let next_run_raw = next_run_raw.ok_or_else(|| {
                        SchedulerError::Storage(format!(
                            "missing cron next_run for task {}",
                            id_raw
                        ))
                    })?;
                    let next_run = parse_datetime(&next_run_raw)?;
                    Schedule::Cron { expression, next_run }
                }
                "one_shot" => {
                    let run_at_raw = run_at_raw.ok_or_else(|| {
                        SchedulerError::Storage(format!(
                            "missing one_shot run_at for task {}",
                            id_raw
                        ))
                    })?;
                    let run_at = parse_datetime(&run_at_raw)?;
                    Schedule::OneShot { run_at }
                }
                other => {
                    return Err(SchedulerError::Storage(format!(
                        "unknown schedule type {} for task {}",
                        other, id_raw
                    )))
                }
            };
            let kind = match kind_raw.as_str() {
                "send_email" => TaskKind::SendEmail(self.load_send_email_task(&conn, &id_raw)?),
                "run_task" => TaskKind::RunTask(self.load_run_task_task(&conn, &id_raw)?),
                "noop" => TaskKind::Noop,
                other => {
                    return Err(SchedulerError::Storage(format!(
                        "unknown task kind {} for task {}",
                        other, id_raw
                    )))
                }
            };
            tasks.push(ScheduledTask {
                id,
                kind,
                schedule,
                enabled: enabled_raw != 0,
                created_at,
                last_run,
            });
        }
        Ok(tasks)
    }

    fn insert_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let mut conn = self.open()?;
        let tx = conn.transaction()?;
        let (schedule_type, cron_expression, next_run, run_at) = schedule_columns(&task.schedule);
        tx.execute(
            "INSERT INTO tasks (id, kind, enabled, created_at, last_run, schedule_type, cron_expression, next_run, run_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                task.id.to_string(),
                task_kind_label(&task.kind),
                bool_to_int(task.enabled),
                format_datetime(task.created_at.clone()),
                task.last_run.as_ref().map(|value| format_datetime(value.clone())),
                schedule_type,
                cron_expression,
                next_run,
                run_at
            ],
        )?;

        match &task.kind {
            TaskKind::SendEmail(send) => {
                tx.execute(
                    "INSERT INTO send_email_tasks (task_id, subject, html_path, attachments_dir, in_reply_to, references_header, archive_root, thread_epoch, thread_state_path)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    params![
                        task.id.to_string(),
                        send.subject.as_str(),
                        send.html_path.to_string_lossy().into_owned(),
                        send.attachments_dir.to_string_lossy().into_owned(),
                        send.in_reply_to.as_deref(),
                        send.references.as_deref(),
                        send.archive_root
                            .as_ref()
                            .map(|value| value.to_string_lossy().into_owned()),
                        send.thread_epoch.map(|value| value as i64),
                        send.thread_state_path
                            .as_ref()
                            .map(|value| value.to_string_lossy().into_owned()),
                    ],
                )?;
                insert_recipients(
                    &tx,
                    &task.id.to_string(),
                    "to",
                    &send.to,
                )?;
                insert_recipients(
                    &tx,
                    &task.id.to_string(),
                    "cc",
                    &send.cc,
                )?;
                insert_recipients(
                    &tx,
                    &task.id.to_string(),
                    "bcc",
                    &send.bcc,
                )?;
            }
            TaskKind::RunTask(run) => {
                tx.execute(
                    "INSERT INTO run_task_tasks (task_id, workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, codex_disabled, reply_to, archive_root, thread_id, thread_epoch, thread_state_path)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                    params![
                        task.id.to_string(),
                        run.workspace_dir.to_string_lossy().into_owned(),
                        run.input_email_dir.to_string_lossy().into_owned(),
                        run.input_attachments_dir.to_string_lossy().into_owned(),
                        run.memory_dir.to_string_lossy().into_owned(),
                        run.reference_dir.to_string_lossy().into_owned(),
                        run.model_name.as_str(),
                        bool_to_int(run.codex_disabled),
                        join_recipients(&run.reply_to),
                        run.archive_root
                            .as_ref()
                            .map(|value| value.to_string_lossy().into_owned()),
                        run.thread_id.as_deref(),
                        run.thread_epoch.map(|value| value as i64),
                        run.thread_state_path
                            .as_ref()
                            .map(|value| value.to_string_lossy().into_owned()),
                    ],
                )?;
            }
            TaskKind::Noop => {}
        }

        tx.commit()?;
        Ok(())
    }

    fn update_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        let (schedule_type, cron_expression, next_run, run_at) = schedule_columns(&task.schedule);
        conn.execute(
            "UPDATE tasks
             SET enabled = ?1,
                 last_run = ?2,
                 schedule_type = ?3,
                 cron_expression = ?4,
                 next_run = ?5,
                 run_at = ?6
             WHERE id = ?7",
            params![
                bool_to_int(task.enabled),
                task.last_run.as_ref().map(|value| format_datetime(value.clone())),
                schedule_type,
                cron_expression,
                next_run,
                run_at,
                task.id.to_string()
            ],
        )?;
        Ok(())
    }

    fn record_execution_start(
        &self,
        task_id: Uuid,
        started_at: DateTime<Utc>,
    ) -> Result<i64, SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "INSERT INTO task_executions (task_id, started_at, status)
             VALUES (?1, ?2, 'running')",
            params![task_id.to_string(), format_datetime(started_at)],
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn record_execution_finish(
        &self,
        execution_id: i64,
        finished_at: DateTime<Utc>,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE task_executions
             SET finished_at = ?1,
                 status = ?2,
                 error_message = ?3
             WHERE id = ?4",
            params![
                format_datetime(finished_at),
                status,
                error_message,
                execution_id
            ],
        )?;
        Ok(())
    }

    fn load_send_email_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendEmailTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT subject, html_path, attachments_dir, in_reply_to, references_header, archive_root, thread_epoch, thread_state_path
                 FROM send_email_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<i64>>(6)?,
                        row.get::<_, Option<String>>(7)?,
                    ))
                },
            )
            .optional()?;
        let (
            subject,
            html_path,
            attachments_dir,
            in_reply_to_raw,
            references_raw,
            archive_root,
            thread_epoch_raw,
            thread_state_path,
        ) =
            row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_email_tasks row for task {}",
                task_id
            ))
        })?;

        let mut to = Vec::new();
        let mut cc = Vec::new();
        let mut bcc = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT recipient_type, address
             FROM send_email_recipients
             WHERE task_id = ?1
             ORDER BY id",
        )?;
        let rows = stmt.query_map(params![task_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (recipient_type, address) = row?;
            match recipient_type.as_str() {
                "to" => to.push(address),
                "cc" => cc.push(address),
                "bcc" => bcc.push(address),
                _ => {}
            }
        }

        Ok(SendEmailTask {
            subject,
            html_path: PathBuf::from(html_path),
            attachments_dir: PathBuf::from(attachments_dir),
            to,
            cc,
            bcc,
            in_reply_to: normalize_header_value(in_reply_to_raw),
            references: normalize_header_value(references_raw),
            archive_root: normalize_optional_path(archive_root),
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    fn load_run_task_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<RunTaskTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, codex_disabled, reply_to, archive_root, thread_id, thread_epoch, thread_state_path
                 FROM run_task_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, i64>(6)?,
                        row.get::<_, String>(7)?,
                        row.get::<_, Option<String>>(8)?,
                        row.get::<_, Option<String>>(9)?,
                        row.get::<_, Option<i64>>(10)?,
                        row.get::<_, Option<String>>(11)?,
                    ))
                },
            )
            .optional()?;
        let (
            workspace_dir,
            input_email_dir,
            input_attachments_dir,
            memory_dir,
            reference_dir,
            model_name,
            codex_disabled,
            reply_to_raw,
            archive_root,
            thread_id,
            thread_epoch_raw,
            thread_state_path,
        ) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing run_task_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(RunTaskTask {
            workspace_dir: PathBuf::from(workspace_dir),
            input_email_dir: PathBuf::from(input_email_dir),
            input_attachments_dir: PathBuf::from(input_attachments_dir),
            memory_dir: PathBuf::from(memory_dir),
            reference_dir: PathBuf::from(reference_dir),
            model_name,
            codex_disabled: codex_disabled != 0,
            reply_to: split_recipients(&reply_to_raw),
            archive_root: normalize_optional_path(archive_root),
            thread_id,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    fn open(&self) -> Result<Connection, SchedulerError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.execute_batch(SCHEDULER_SCHEMA)?;
        ensure_send_email_task_columns(&conn)?;
        ensure_run_task_task_columns(&conn)?;
        Ok(conn)
    }
}

fn task_kind_label(kind: &TaskKind) -> &'static str {
    match kind {
        TaskKind::SendEmail(_) => "send_email",
        TaskKind::RunTask(_) => "run_task",
        TaskKind::Noop => "noop",
    }
}

fn schedule_columns(
    schedule: &Schedule,
) -> (String, Option<String>, Option<String>, Option<String>) {
    match schedule {
        Schedule::Cron { expression, next_run } => (
            "cron".to_string(),
            Some(expression.clone()),
            Some(format_datetime(next_run.clone())),
            None,
        ),
        Schedule::OneShot { run_at } => (
            "one_shot".to_string(),
            None,
            None,
            Some(format_datetime(run_at.clone())),
        ),
    }
}

fn format_datetime(value: DateTime<Utc>) -> String {
    value.to_rfc3339()
}

fn parse_datetime(value: &str) -> Result<DateTime<Utc>, SchedulerError> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

fn parse_optional_datetime(value: Option<&str>) -> Result<Option<DateTime<Utc>>, SchedulerError> {
    match value {
        Some(raw) => Ok(Some(parse_datetime(raw)?)),
        None => Ok(None),
    }
}

fn bool_to_int(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

fn join_recipients(values: &[String]) -> String {
    values.join("\n")
}

fn split_recipients(raw: &str) -> Vec<String> {
    raw.lines()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect()
}

fn normalize_header_value(value: Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(|trimmed| trimmed.to_string())
}

fn normalize_optional_path(value: Option<String>) -> Option<PathBuf> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(PathBuf::from)
}

fn insert_recipients(
    tx: &rusqlite::Transaction<'_>,
    task_id: &str,
    recipient_type: &str,
    recipients: &[String],
) -> Result<(), SchedulerError> {
    let mut stmt = tx.prepare(
        "INSERT INTO send_email_recipients (task_id, recipient_type, address)
         VALUES (?1, ?2, ?3)",
    )?;
    for address in recipients {
        stmt.execute(params![task_id, recipient_type, address])?;
    }
    Ok(())
}

fn snapshot_reply_draft(task: &RunTaskTask) -> Result<(), SchedulerError> {
    let draft_path = task.workspace_dir.join("reply_email_draft.html");
    if !draft_path.exists() {
        return Ok(());
    }
    let drafts_dir = task.workspace_dir.join("drafts");
    fs::create_dir_all(&drafts_dir)?;
    let timestamp = Utc::now().format("%Y%m%dT%H%M%S");
    let filename = match task.thread_epoch {
        Some(epoch) => format!("reply_email_draft_epoch_{epoch}_{timestamp}.html"),
        None => format!("reply_email_draft_{timestamp}.html"),
    };
    let dest = drafts_dir.join(filename);
    fs::copy(&draft_path, dest)?;
    Ok(())
}

fn thread_epoch_matches(task: &RunTaskTask) -> bool {
    let expected = match task.thread_epoch {
        Some(value) => value,
        None => return true,
    };
    let state_path = task
        .thread_state_path
        .clone()
        .unwrap_or_else(|| default_thread_state_path(&task.workspace_dir));
    match current_thread_epoch(&state_path) {
        Some(current) => current == expected,
        None => true,
    }
}

fn ingest_follow_up_tasks<E: TaskExecutor>(
    scheduler: &mut Scheduler<E>,
    task: &RunTaskTask,
    requests: &[run_task_module::ScheduledTaskRequest],
) {
    if !thread_epoch_matches(task) {
        info!(
            "skip follow-up scheduling for stale thread epoch in {}",
            task.workspace_dir.display()
        );
        return;
    }
    if requests.is_empty() {
        return;
    }
    let mut scheduled = 0usize;
    for request in requests {
        match request {
            run_task_module::ScheduledTaskRequest::SendEmail(request) => {
                match schedule_send_email(scheduler, task, request) {
                    Ok(true) => scheduled += 1,
                    Ok(false) => {}
                    Err(err) => warn!(
                        "failed to schedule follow-up email from {}: {}",
                        task.workspace_dir.display(),
                        err
                    ),
                }
            }
        }
    }

    info!(
        "scheduled {} follow-up task(s) from {}",
        scheduled,
        task.workspace_dir.display()
    );
}

fn schedule_auto_reply<E: TaskExecutor>(
    scheduler: &mut Scheduler<E>,
    task: &RunTaskTask,
) -> Result<bool, SchedulerError> {
    if !thread_epoch_matches(task) {
        info!(
            "skip auto reply for stale thread epoch in {}",
            task.workspace_dir.display()
        );
        return Ok(false);
    }
    if task.reply_to.is_empty() {
        return Ok(false);
    }

    let html_path = task.workspace_dir.join("reply_email_draft.html");
    if !html_path.exists() {
        warn!(
            "auto reply missing reply_email_draft.html in workspace {}",
            task.workspace_dir.display()
        );
        return Ok(false);
    }
    let attachments_dir = task.workspace_dir.join("reply_email_attachments");
    let reply_context = load_reply_context(&task.workspace_dir);

    let send_task = SendEmailTask {
        subject: reply_context.subject,
        html_path,
        attachments_dir,
        to: task.reply_to.clone(),
        cc: Vec::new(),
        bcc: Vec::new(),
        in_reply_to: reply_context.in_reply_to,
        references: reply_context.references,
        archive_root: task.archive_root.clone(),
        thread_epoch: task.thread_epoch,
        thread_state_path: task.thread_state_path.clone(),
    };

    let task_id = scheduler.add_one_shot_in(
        Duration::from_secs(0),
        TaskKind::SendEmail(send_task),
    )?;
    info!(
        "scheduled auto reply task {} from {}",
        task_id,
        task.workspace_dir.display()
    );
    Ok(true)
}

fn schedule_send_email<E: TaskExecutor>(
    scheduler: &mut Scheduler<E>,
    task: &RunTaskTask,
    request: &run_task_module::ScheduledSendEmailTask,
) -> Result<bool, SchedulerError> {
    if request.html_path.trim().is_empty() {
        warn!(
            "scheduled send_email missing html_path in workspace {}",
            task.workspace_dir.display()
        );
        return Ok(false);
    }

    let html_path = match resolve_rel_path(&task.workspace_dir, &request.html_path) {
        Some(path) => path,
        None => {
            warn!(
                "scheduled send_email has invalid html_path '{}' in workspace {}",
                request.html_path,
                task.workspace_dir.display()
            );
            return Ok(false);
        }
    };

    if !html_path.exists() {
        warn!(
            "scheduled send_email html_path does not exist: {}",
            html_path.display()
        );
        return Ok(false);
    }

    let attachments_raw = request
        .attachments_dir
        .as_deref()
        .unwrap_or("scheduled_email_attachments");
    let attachments_dir = match resolve_rel_path(&task.workspace_dir, attachments_raw) {
        Some(path) => path,
        None => {
            warn!(
                "scheduled send_email has invalid attachments_dir '{}' in workspace {}",
                attachments_raw,
                task.workspace_dir.display()
            );
            return Ok(false);
        }
    };

    let mut to = request.to.clone();
    if to.is_empty() {
        to = task.reply_to.clone();
    }
    if to.is_empty() {
        warn!(
            "scheduled send_email missing recipients in workspace {}",
            task.workspace_dir.display()
        );
        return Ok(false);
    }

    let send_task = SendEmailTask {
        subject: request.subject.clone(),
        html_path,
        attachments_dir,
        to,
        cc: request.cc.clone(),
        bcc: request.bcc.clone(),
        in_reply_to: None,
        references: None,
        archive_root: task.archive_root.clone(),
        thread_epoch: task.thread_epoch,
        thread_state_path: task.thread_state_path.clone(),
    };

    if let Some(run_at_raw) = request.run_at.as_deref() {
        match parse_datetime(run_at_raw) {
            Ok(run_at) => {
                let task_id =
                    scheduler.add_one_shot_at(run_at, TaskKind::SendEmail(send_task))?;
                info!(
                    "scheduled follow-up send_email task {} from {} run_at={}",
                    task_id,
                    task.workspace_dir.display(),
                    run_at.to_rfc3339()
                );
                return Ok(true);
            }
            Err(err) => {
                warn!(
                    "scheduled send_email has invalid run_at '{}' in workspace {}: {}",
                    run_at_raw,
                    task.workspace_dir.display(),
                    err
                );
                return Ok(false);
            }
        }
    }

    let delay_seconds = request
        .delay_seconds
        .or_else(|| request.delay_minutes.map(|value| value.saturating_mul(60)));
    let delay_seconds = match delay_seconds {
        Some(value) => value.max(0) as u64,
        None => {
            warn!(
                "scheduled send_email missing delay for workspace {}",
                task.workspace_dir.display()
            );
            return Ok(false);
        }
    };

    let task_id = scheduler.add_one_shot_in(
        Duration::from_secs(delay_seconds),
        TaskKind::SendEmail(send_task),
    )?;
    info!(
        "scheduled follow-up send_email task {} from {} delay_seconds={}",
        task_id,
        task.workspace_dir.display(),
        delay_seconds
    );
    Ok(true)
}

fn resolve_rel_path(root: &Path, raw: &str) -> Option<PathBuf> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let rel = PathBuf::from(trimmed);
    if rel.is_absolute() {
        return None;
    }
    if rel.components().any(|comp| matches!(comp, Component::ParentDir)) {
        return None;
    }
    Some(root.join(rel))
}

#[derive(Debug)]
struct ReplyContext {
    subject: String,
    in_reply_to: Option<String>,
    references: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PostmarkInboundLite {
    #[serde(rename = "Subject")]
    subject: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    message_id: Option<String>,
    #[serde(rename = "Headers")]
    headers: Option<Vec<PostmarkHeaderLite>>,
}

#[derive(Debug, Deserialize)]
struct PostmarkHeaderLite {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

impl PostmarkInboundLite {
    fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.as_ref().and_then(|headers| {
            headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case(name))
                .map(|header| header.value.as_str())
        })
    }

    fn header_message_id(&self) -> Option<&str> {
        self.header_value("message-id")
    }
}

fn load_reply_context(workspace_dir: &Path) -> ReplyContext {
    let payload_path = workspace_dir
        .join("incoming_email")
        .join("postmark_payload.json");
    let payload = fs::read_to_string(&payload_path)
        .ok()
        .and_then(|content| serde_json::from_str::<PostmarkInboundLite>(&content).ok());

    if let Some(payload) = payload {
        let subject_raw = payload.subject.as_deref().unwrap_or("");
        let subject = reply_subject(subject_raw);
        let (in_reply_to, references) = reply_headers(&payload);
        ReplyContext {
            subject,
            in_reply_to,
            references,
        }
    } else {
        ReplyContext {
            subject: reply_subject(""),
            in_reply_to: None,
            references: None,
        }
    }
}

fn reply_subject(original: &str) -> String {
    let trimmed = original.trim();
    if trimmed.is_empty() {
        "Re: (no subject)".to_string()
    } else if trimmed.to_lowercase().starts_with("re:") {
        trimmed.to_string()
    } else {
        format!("Re: {}", trimmed)
    }
}

fn reply_headers(payload: &PostmarkInboundLite) -> (Option<String>, Option<String>) {
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());

    let mut references = payload
        .header_value("References")
        .or_else(|| payload.header_value("In-Reply-To"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    if let Some(ref msg_id) = message_id {
        references = match references {
            Some(existing) => {
                if references_contains(&existing, msg_id) {
                    Some(existing)
                } else {
                    Some(format!("{existing} {msg_id}"))
                }
            }
            None => Some(msg_id.clone()),
        };
    }

    (message_id, references)
}

fn references_contains(references: &str, message_id: &str) -> bool {
    references.split_whitespace().any(|entry| entry == message_id)
}

pub mod service;
pub mod index_store;
pub mod user_store;
pub mod past_emails;
pub mod memory_store;
