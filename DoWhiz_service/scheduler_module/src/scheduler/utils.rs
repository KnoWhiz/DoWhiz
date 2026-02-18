use chrono::{DateTime, Utc};
use rusqlite::params;
use std::path::{Path, PathBuf};

use crate::channel::Channel;

use super::types::{Schedule, SchedulerError, TaskKind};

pub(crate) fn task_kind_label(kind: &TaskKind) -> &'static str {
    match kind {
        TaskKind::SendReply(_) => "send_email",
        TaskKind::RunTask(_) => "run_task",
        TaskKind::Noop => "noop",
    }
}

pub(crate) fn task_kind_channel(kind: &TaskKind) -> Channel {
    match kind {
        TaskKind::SendReply(send) => send.channel.clone(),
        TaskKind::RunTask(run) => run.channel.clone(),
        TaskKind::Noop => Channel::default(),
    }
}

pub(crate) fn schedule_columns(
    schedule: &Schedule,
) -> (String, Option<String>, Option<String>, Option<String>) {
    match schedule {
        Schedule::Cron {
            expression,
            next_run,
        } => (
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

pub(crate) fn format_datetime(value: DateTime<Utc>) -> String {
    value.to_rfc3339()
}

pub(crate) fn parse_datetime(value: &str) -> Result<DateTime<Utc>, SchedulerError> {
    Ok(DateTime::parse_from_rfc3339(value)?.with_timezone(&Utc))
}

pub(crate) fn parse_optional_datetime(
    value: Option<&str>,
) -> Result<Option<DateTime<Utc>>, SchedulerError> {
    match value {
        Some(raw) => Ok(Some(parse_datetime(raw)?)),
        None => Ok(None),
    }
}

pub(crate) fn bool_to_int(value: bool) -> i64 {
    if value {
        1
    } else {
        0
    }
}

pub(crate) fn join_recipients(values: &[String]) -> String {
    values.join("\n")
}

pub(crate) fn split_recipients(raw: &str) -> Vec<String> {
    raw.lines()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .collect()
}

pub(crate) fn normalize_header_value(value: Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(|trimmed| trimmed.to_string())
}

pub(crate) fn normalize_optional_path(value: Option<String>) -> Option<PathBuf> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
        .map(PathBuf::from)
}

pub(crate) fn insert_recipients(
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

pub fn load_google_access_token_from_service_env() -> Option<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let service_root = manifest_dir.parent().unwrap_or(manifest_dir);
    let env_path = service_root.join(".env");
    let iter = dotenvy::from_path_iter(&env_path).ok()?;
    for item in iter {
        if let Ok((key, value)) = item {
            if key == "GOOGLE_ACCESS_TOKEN" {
                let value = value.trim();
                if value.is_empty() {
                    return None;
                }
                return Some(value.to_string());
            }
        }
    }
    None
}
