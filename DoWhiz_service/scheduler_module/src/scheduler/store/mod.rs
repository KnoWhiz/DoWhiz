use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

use crate::channel::Channel;

use super::types::{Schedule, ScheduledTask, SchedulerError, TaskKind};
use super::utils::{
    bool_to_int, format_datetime, parse_datetime, parse_optional_datetime, schedule_columns,
    task_kind_channel, task_kind_label,
};

mod migrations;
mod schema;
mod task_rows;

use migrations::{
    ensure_run_task_task_columns, ensure_send_bluebubbles_tasks_table,
    ensure_send_discord_tasks_table, ensure_send_email_task_columns, ensure_send_slack_tasks_table,
    ensure_send_sms_tasks_table, ensure_send_telegram_tasks_table, ensure_tasks_columns,
};
use schema::SCHEDULER_SCHEMA;

#[derive(Debug)]
pub(crate) struct SqliteSchedulerStore {
    path: PathBuf,
}

impl SqliteSchedulerStore {
    pub(crate) fn new(path: PathBuf) -> Result<Self, SchedulerError> {
        let store = Self { path };
        let _ = store.open()?;
        Ok(store)
    }

    pub(crate) fn load_tasks(&self) -> Result<Vec<ScheduledTask>, SchedulerError> {
        let conn = self.open()?;
        let mut stmt = conn.prepare(
            "SELECT id, kind, channel, enabled, created_at, last_run, schedule_type, cron_expression, next_run, run_at
             FROM tasks
             ORDER BY created_at",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
            ))
        })?;

        let mut tasks = Vec::new();
        for row in rows {
            let (
                id_raw,
                kind_raw,
                channel_raw,
                enabled_raw,
                created_at_raw,
                last_run_raw,
                schedule_type,
                cron_expression,
                next_run_raw,
                run_at_raw,
            ) = row?;
            let channel: Channel = channel_raw.parse().unwrap_or_default();
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
                    Schedule::Cron {
                        expression,
                        next_run,
                    }
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
                "send_email" => {
                    // Dispatch to appropriate loader based on channel
                    let send_task = match channel {
                        Channel::Slack => {
                            self.load_send_slack_task(&conn, &id_raw, channel.clone())?
                        }
                        Channel::Discord => self.load_send_discord_task(&conn, &id_raw)?,
                        Channel::GoogleDocs => {
                            // Google Docs uses a similar format to email for now
                            self.load_send_email_task(&conn, &id_raw, channel.clone())?
                        }
                        Channel::Sms => self.load_send_sms_task(&conn, &id_raw)?,
                        Channel::Email => {
                            self.load_send_email_task(&conn, &id_raw, channel.clone())?
                        }
                        Channel::Telegram => self.load_send_telegram_task(&conn, &id_raw)?,
                        Channel::BlueBubbles => self.load_send_bluebubbles_task(&conn, &id_raw)?,
                    };
                    TaskKind::SendReply(send_task)
                }
                "run_task" => TaskKind::RunTask(self.load_run_task_task(&conn, &id_raw, channel)?),
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

    pub(crate) fn insert_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
        let mut conn = self.open()?;
        let tx = conn.transaction()?;
        let (schedule_type, cron_expression, next_run, run_at) = schedule_columns(&task.schedule);
        let channel = task_kind_channel(&task.kind);
        tx.execute(
            "INSERT INTO tasks (id, kind, channel, enabled, created_at, last_run, schedule_type, cron_expression, next_run, run_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                task.id.to_string(),
                task_kind_label(&task.kind),
                channel.to_string(),
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
            TaskKind::SendReply(send) => {
                // Dispatch to appropriate child table based on channel
                match send.channel {
                    Channel::Slack => {
                        self.insert_send_slack_task(&tx, &task.id.to_string(), send)?;
                    }
                    Channel::Discord => {
                        self.insert_send_discord_task(&tx, &task.id.to_string(), send)?;
                    }
                    Channel::GoogleDocs => {
                        // Google Docs uses the email table format for now
                        self.insert_send_email_task(&tx, &task.id.to_string(), send)?;
                    }
                    Channel::Sms => {
                        self.insert_send_sms_task(&tx, &task.id.to_string(), send)?;
                    }
                    Channel::Email => {
                        self.insert_send_email_task(&tx, &task.id.to_string(), send)?;
                    }
                    Channel::Telegram => {
                        self.insert_send_telegram_task(&tx, &task.id.to_string(), send)?;
                    }
                    Channel::BlueBubbles => {
                        self.insert_send_bluebubbles_task(&tx, &task.id.to_string(), send)?;
                    }
                }
            }
            TaskKind::RunTask(run) => {
                tx.execute(
                    "INSERT INTO run_task_tasks (task_id, workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, runner, codex_disabled, reply_to, reply_from, archive_root, thread_id, thread_epoch, thread_state_path)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                    params![
                        task.id.to_string(),
                        run.workspace_dir.to_string_lossy().into_owned(),
                        run.input_email_dir.to_string_lossy().into_owned(),
                        run.input_attachments_dir.to_string_lossy().into_owned(),
                        run.memory_dir.to_string_lossy().into_owned(),
                        run.reference_dir.to_string_lossy().into_owned(),
                        run.model_name.as_str(),
                        run.runner.as_str(),
                        bool_to_int(run.codex_disabled),
                        super::utils::join_recipients(&run.reply_to),
                        run.reply_from.as_deref(),
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

    pub(crate) fn update_task(&self, task: &ScheduledTask) -> Result<(), SchedulerError> {
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
                task.last_run
                    .as_ref()
                    .map(|value| format_datetime(value.clone())),
                schedule_type,
                cron_expression,
                next_run,
                run_at,
                task.id.to_string()
            ],
        )?;
        Ok(())
    }

    pub(crate) fn record_execution_start(
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

    pub(crate) fn record_execution_finish(
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

    fn open(&self) -> Result<Connection, SchedulerError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        conn.busy_timeout(Duration::from_secs(5))?;
        conn.execute_batch(SCHEDULER_SCHEMA)?;
        ensure_tasks_columns(&conn)?;
        ensure_send_email_task_columns(&conn)?;
        ensure_send_slack_tasks_table(&conn)?;
        ensure_send_discord_tasks_table(&conn)?;
        ensure_send_sms_tasks_table(&conn)?;
        ensure_send_bluebubbles_tasks_table(&conn)?;
        ensure_send_telegram_tasks_table(&conn)?;
        ensure_run_task_task_columns(&conn)?;
        Ok(conn)
    }

    /// Get the current retry count for a task
    pub(crate) fn get_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        let conn = self.open()?;
        let count: i64 = conn
            .query_row(
                "SELECT retry_count FROM tasks WHERE id = ?1",
                params![task_id],
                |row| row.get(0),
            )
            .unwrap_or(0);
        Ok(count as u32)
    }

    /// Increment the retry count for a task and return the new count
    pub(crate) fn increment_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE tasks SET retry_count = retry_count + 1 WHERE id = ?1",
            params![task_id],
        )?;
        let count: i64 = conn.query_row(
            "SELECT retry_count FROM tasks WHERE id = ?1",
            params![task_id],
            |row| row.get(0),
        )?;
        Ok(count as u32)
    }

    /// Reset the retry count for a task (after successful execution)
    pub(crate) fn reset_retry_count(&self, task_id: &str) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE tasks SET retry_count = 0 WHERE id = ?1",
            params![task_id],
        )?;
        Ok(())
    }

    /// Disable a task by its ID (used when max retries exceeded)
    pub(crate) fn disable_task_by_id(&self, task_id: &str) -> Result<(), SchedulerError> {
        let conn = self.open()?;
        conn.execute(
            "UPDATE tasks SET enabled = 0 WHERE id = ?1",
            params![task_id],
        )?;
        Ok(())
    }
}
