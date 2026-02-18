use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

use crate::channel::Channel;

use super::types::{RunTaskTask, Schedule, ScheduledTask, SchedulerError, SendReplyTask, TaskKind};
use super::utils::{
    bool_to_int, format_datetime, insert_recipients, join_recipients, normalize_header_value,
    normalize_optional_path, parse_datetime, parse_optional_datetime, schedule_columns,
    split_recipients, task_kind_channel, task_kind_label,
};

const SCHEDULER_SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    channel TEXT NOT NULL DEFAULT 'email',
    enabled INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_run TEXT,
    schedule_type TEXT NOT NULL,
    cron_expression TEXT,
    next_run TEXT,
    run_at TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS send_email_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    subject TEXT NOT NULL,
    html_path TEXT NOT NULL,
    attachments_dir TEXT NOT NULL,
    from_address TEXT,
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

CREATE TABLE IF NOT EXISTS send_slack_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    slack_channel_id TEXT NOT NULL,
    thread_ts TEXT,
    text_path TEXT NOT NULL,
    workspace_dir TEXT
);

CREATE TABLE IF NOT EXISTS send_sms_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    from_number TEXT,
    to_number TEXT NOT NULL,
    text_path TEXT NOT NULL,
    thread_id TEXT,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS send_bluebubbles_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    chat_guid TEXT NOT NULL,
    text_path TEXT NOT NULL,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS send_telegram_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    chat_id TEXT NOT NULL,
    text_path TEXT NOT NULL,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS run_task_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    workspace_dir TEXT NOT NULL,
    input_email_dir TEXT NOT NULL,
    input_attachments_dir TEXT NOT NULL,
    memory_dir TEXT NOT NULL,
    reference_dir TEXT NOT NULL,
    model_name TEXT NOT NULL,
    runner TEXT NOT NULL,
    codex_disabled INTEGER NOT NULL,
    reply_to TEXT NOT NULL,
    reply_from TEXT,
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
    if !columns.contains("from_address") {
        conn.execute(
            "ALTER TABLE send_email_tasks ADD COLUMN from_address TEXT",
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

fn ensure_tasks_columns(conn: &Connection) -> Result<(), SchedulerError> {
    let mut stmt = conn.prepare("PRAGMA table_info(tasks)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;
    let mut columns = HashSet::new();
    for row in rows {
        columns.insert(row?);
    }

    if !columns.contains("channel") {
        conn.execute(
            "ALTER TABLE tasks ADD COLUMN channel TEXT NOT NULL DEFAULT 'email'",
            [],
        )?;
    }

    if !columns.contains("retry_count") {
        conn.execute(
            "ALTER TABLE tasks ADD COLUMN retry_count INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    Ok(())
}

fn ensure_send_slack_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS send_slack_tasks (
            task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
            slack_channel_id TEXT NOT NULL,
            thread_ts TEXT,
            text_path TEXT NOT NULL,
            workspace_dir TEXT
        )",
        [],
    )?;
    Ok(())
}

fn ensure_send_discord_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS send_discord_tasks (
            task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
            discord_channel_id TEXT NOT NULL,
            thread_id TEXT,
            text_path TEXT NOT NULL,
            workspace_dir TEXT
        )",
        [],
    )?;
    Ok(())
}

fn ensure_send_sms_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS send_sms_tasks (
            task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
            from_number TEXT,
            to_number TEXT NOT NULL,
            text_path TEXT NOT NULL,
            thread_id TEXT,
            thread_epoch INTEGER,
            thread_state_path TEXT
        )",
        [],
    )?;
    Ok(())
}

fn ensure_send_bluebubbles_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS send_bluebubbles_tasks (
            task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
            chat_guid TEXT NOT NULL,
            text_path TEXT NOT NULL,
            thread_epoch INTEGER,
            thread_state_path TEXT
        )",
        [],
    )?;
    Ok(())
}

fn ensure_send_telegram_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS send_telegram_tasks (
            task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
            chat_id TEXT NOT NULL,
            text_path TEXT NOT NULL,
            thread_epoch INTEGER,
            thread_state_path TEXT
        )",
        [],
    )?;
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
    if !columns.contains("runner") {
        conn.execute("ALTER TABLE run_task_tasks ADD COLUMN runner TEXT", [])?;
    }
    if !columns.contains("reply_from") {
        conn.execute("ALTER TABLE run_task_tasks ADD COLUMN reply_from TEXT", [])?;
    }
    if !columns.contains("thread_id") {
        conn.execute("ALTER TABLE run_task_tasks ADD COLUMN thread_id TEXT", [])?;
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
                        join_recipients(&run.reply_to),
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

    fn insert_send_email_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        tx.execute(
            "INSERT INTO send_email_tasks (task_id, subject, html_path, attachments_dir, from_address, in_reply_to, references_header, archive_root, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                task_id,
                send.subject.as_str(),
                send.html_path.to_string_lossy().into_owned(),
                send.attachments_dir.to_string_lossy().into_owned(),
                send.from.as_deref(),
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
        insert_recipients(tx, task_id, "to", &send.to)?;
        insert_recipients(tx, task_id, "cc", &send.cc)?;
        insert_recipients(tx, task_id, "bcc", &send.bcc)?;
        Ok(())
    }

    fn insert_send_slack_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For Slack, we use to[0] as channel_id and html_path as text_path
        let slack_channel_id = send.to.first().cloned().unwrap_or_default();
        let thread_ts = send.in_reply_to.clone();
        let workspace_dir = send
            .archive_root
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned());
        tx.execute(
            "INSERT INTO send_slack_tasks (task_id, slack_channel_id, thread_ts, text_path, workspace_dir)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                slack_channel_id,
                thread_ts,
                send.html_path.to_string_lossy().into_owned(),
                workspace_dir,
            ],
        )?;
        Ok(())
    }

    fn insert_send_discord_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For Discord, we use to[0] as channel_id and html_path as text_path
        let discord_channel_id = send.to.first().cloned().unwrap_or_default();
        let thread_id = send.in_reply_to.clone();
        let workspace_dir = send
            .archive_root
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned());
        tx.execute(
            "INSERT INTO send_discord_tasks (task_id, discord_channel_id, thread_id, text_path, workspace_dir)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                discord_channel_id,
                thread_id,
                send.html_path.to_string_lossy().into_owned(),
                workspace_dir,
            ],
        )?;
        Ok(())
    }

    fn insert_send_sms_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        let to_number = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_sms_tasks (task_id, from_number, to_number, text_path, thread_id, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                task_id,
                send.from.as_deref(),
                to_number,
                send.html_path.to_string_lossy().into_owned(),
                send.in_reply_to.as_deref(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    fn insert_send_bluebubbles_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For BlueBubbles, we use to[0] as chat_guid and html_path as text_path
        let chat_guid = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_bluebubbles_tasks (task_id, chat_guid, text_path, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                chat_guid,
                send.html_path.to_string_lossy().into_owned(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    fn insert_send_telegram_task(
        &self,
        tx: &Transaction,
        task_id: &str,
        send: &SendReplyTask,
    ) -> Result<(), SchedulerError> {
        // For Telegram, we use to[0] as chat_id and html_path as text_path
        let chat_id = send.to.first().cloned().unwrap_or_default();
        tx.execute(
            "INSERT INTO send_telegram_tasks (task_id, chat_id, text_path, thread_epoch, thread_state_path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                task_id,
                chat_id,
                send.html_path.to_string_lossy().into_owned(),
                send.thread_epoch.map(|value| value as i64),
                send.thread_state_path
                    .as_ref()
                    .map(|value| value.to_string_lossy().into_owned()),
            ],
        )?;
        Ok(())
    }

    fn load_send_email_task(
        &self,
        conn: &Connection,
        task_id: &str,
        channel: Channel,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT subject, html_path, attachments_dir, from_address, in_reply_to, references_header, archive_root, thread_epoch, thread_state_path
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
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, Option<i64>>(7)?,
                        row.get::<_, Option<String>>(8)?,
                    ))
                },
            )
            .optional()?;
        let (
            subject,
            html_path,
            attachments_dir,
            from_raw,
            in_reply_to_raw,
            references_raw,
            archive_root,
            thread_epoch_raw,
            thread_state_path,
        ) = row.ok_or_else(|| {
            SchedulerError::Storage(format!("missing send_email_tasks row for task {}", task_id))
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

        Ok(SendReplyTask {
            channel,
            subject,
            html_path: PathBuf::from(html_path),
            attachments_dir: PathBuf::from(attachments_dir),
            from: normalize_header_value(from_raw),
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

    fn load_send_slack_task(
        &self,
        conn: &Connection,
        task_id: &str,
        channel: Channel,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT slack_channel_id, thread_ts, text_path, workspace_dir
                 FROM send_slack_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (slack_channel_id, thread_ts, text_path, workspace_dir) = row.ok_or_else(|| {
            SchedulerError::Storage(format!("missing send_slack_tasks row for task {}", task_id))
        })?;

        Ok(SendReplyTask {
            channel,
            subject: String::new(), // Slack doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // Slack attachments handled differently
            from: None,
            to: vec![slack_channel_id], // channel_id stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: thread_ts, // thread_ts stored in in_reply_to
            references: None,
            archive_root: workspace_dir.map(PathBuf::from),
            thread_epoch: None,
            thread_state_path: None,
        })
    }

    fn load_send_discord_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT discord_channel_id, thread_id, text_path, workspace_dir
                 FROM send_discord_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (discord_channel_id, thread_id, text_path, workspace_dir) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_discord_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(SendReplyTask {
            channel: Channel::Discord,
            subject: String::new(), // Discord doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // Discord attachments handled differently
            from: None,
            to: vec![discord_channel_id], // channel_id stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: thread_id, // thread_id stored in in_reply_to
            references: None,
            archive_root: workspace_dir.map(PathBuf::from),
            thread_epoch: None,
            thread_state_path: None,
        })
    }

    fn load_send_sms_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT from_number, to_number, text_path, thread_id, thread_epoch, thread_state_path
                 FROM send_sms_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, Option<String>>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<i64>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                    ))
                },
            )
            .optional()?;
        let (from_number, to_number, text_path, thread_id, thread_epoch_raw, thread_state_path) =
            row.ok_or_else(|| {
                SchedulerError::Storage(format!("missing send_sms_tasks row for task {}", task_id))
            })?;

        Ok(SendReplyTask {
            channel: Channel::Sms,
            subject: String::new(),
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(),
            from: normalize_header_value(from_number),
            to: vec![to_number],
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: normalize_header_value(thread_id),
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    fn load_send_bluebubbles_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT chat_guid, text_path, thread_epoch, thread_state_path
                 FROM send_bluebubbles_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (chat_guid, text_path, thread_epoch_raw, thread_state_path) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_bluebubbles_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(SendReplyTask {
            channel: Channel::BlueBubbles,
            subject: String::new(), // BlueBubbles doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // BlueBubbles attachments handled differently
            from: None,
            to: vec![chat_guid], // chat_guid stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: None,
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    fn load_send_telegram_task(
        &self,
        conn: &Connection,
        task_id: &str,
    ) -> Result<SendReplyTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT chat_id, text_path, thread_epoch, thread_state_path
                 FROM send_telegram_tasks
                 WHERE task_id = ?1",
                params![task_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                    ))
                },
            )
            .optional()?;
        let (chat_id, text_path, thread_epoch_raw, thread_state_path) = row.ok_or_else(|| {
            SchedulerError::Storage(format!(
                "missing send_telegram_tasks row for task {}",
                task_id
            ))
        })?;

        Ok(SendReplyTask {
            channel: Channel::Telegram,
            subject: String::new(), // Telegram doesn't use subject
            html_path: PathBuf::from(text_path),
            attachments_dir: PathBuf::new(), // Telegram attachments handled differently
            from: None,
            to: vec![chat_id], // chat_id stored in to[0]
            cc: Vec::new(),
            bcc: Vec::new(),
            in_reply_to: None,
            references: None,
            archive_root: None,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
        })
    }

    fn load_run_task_task(
        &self,
        conn: &Connection,
        task_id: &str,
        channel: Channel,
    ) -> Result<RunTaskTask, SchedulerError> {
        let row = conn
            .query_row(
                "SELECT workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, runner, codex_disabled, reply_to, reply_from, archive_root, thread_id, thread_epoch, thread_state_path
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
                        row.get::<_, Option<String>>(6)?,
                        row.get::<_, i64>(7)?,
                        row.get::<_, String>(8)?,
                        row.get::<_, Option<String>>(9)?,
                        row.get::<_, Option<String>>(10)?,
                        row.get::<_, Option<String>>(11)?,
                        row.get::<_, Option<i64>>(12)?,
                        row.get::<_, Option<String>>(13)?,
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
            runner,
            codex_disabled,
            reply_to_raw,
            reply_from,
            archive_root,
            thread_id,
            thread_epoch_raw,
            thread_state_path,
        ) = row.ok_or_else(|| {
            SchedulerError::Storage(format!("missing run_task_tasks row for task {}", task_id))
        })?;

        Ok(RunTaskTask {
            workspace_dir: PathBuf::from(workspace_dir),
            input_email_dir: PathBuf::from(input_email_dir),
            input_attachments_dir: PathBuf::from(input_attachments_dir),
            memory_dir: PathBuf::from(memory_dir),
            reference_dir: PathBuf::from(reference_dir),
            model_name,
            runner: runner
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| value.to_string())
                .unwrap_or_else(|| "codex".to_string()),
            codex_disabled: codex_disabled != 0,
            reply_to: split_recipients(&reply_to_raw),
            reply_from: normalize_header_value(reply_from),
            archive_root: normalize_optional_path(archive_root),
            thread_id,
            thread_epoch: thread_epoch_raw.map(|value| value as u64),
            thread_state_path: normalize_optional_path(thread_state_path),
            channel,
            slack_team_id: None,
            employee_id: None,
        })
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
