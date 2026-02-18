use std::collections::HashSet;
use rusqlite::Connection;

use super::super::types::SchedulerError;

pub(super) fn ensure_send_email_task_columns(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_tasks_columns(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_send_slack_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_send_discord_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_send_sms_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_send_bluebubbles_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_send_telegram_tasks_table(conn: &Connection) -> Result<(), SchedulerError> {
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

pub(super) fn ensure_run_task_task_columns(conn: &Connection) -> Result<(), SchedulerError> {
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
