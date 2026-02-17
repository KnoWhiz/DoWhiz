use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs;
use std::path::Path;

use super::types::{RunTaskTask, Schedule, ScheduledTask, SchedulerError, TaskKind};
use super::utils::task_kind_label;

const SCHEDULER_SNAPSHOT_FILENAME: &str = "scheduler_snapshot.json";
const SCHEDULER_SNAPSHOT_WINDOW_DAYS: i64 = 7;

pub(crate) fn snapshot_reply_draft(task: &RunTaskTask) -> Result<(), SchedulerError> {
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

#[derive(Debug, Serialize)]
pub(crate) struct SchedulerSnapshot {
    pub(crate) generated_at: DateTime<Utc>,
    pub(crate) window_start: DateTime<Utc>,
    pub(crate) window_end: DateTime<Utc>,
    pub(crate) total_enabled: usize,
    pub(crate) upcoming: Vec<SchedulerSnapshotTask>,
    pub(crate) omitted_past_due: usize,
    pub(crate) omitted_after_window: usize,
}

#[derive(Debug, Serialize)]
pub(crate) struct SchedulerSnapshotTask {
    pub(crate) id: String,
    pub(crate) kind: String,
    pub(crate) schedule: SchedulerSnapshotSchedule,
    pub(crate) next_run: DateTime<Utc>,
    pub(crate) last_run: Option<DateTime<Utc>>,
    pub(crate) status: String,
    pub(crate) label: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum SchedulerSnapshotSchedule {
    Cron {
        expression: String,
        next_run: DateTime<Utc>,
    },
    OneShot {
        run_at: DateTime<Utc>,
    },
}

pub(crate) fn write_scheduler_snapshot(
    workspace_dir: &Path,
    tasks: &[ScheduledTask],
    now: DateTime<Utc>,
) -> Result<(), SchedulerError> {
    let snapshot = build_scheduler_snapshot(tasks, now);
    let payload = serde_json::to_string_pretty(&snapshot)
        .map_err(|err| SchedulerError::Storage(format!("snapshot json error: {}", err)))?;
    let path = workspace_dir.join(SCHEDULER_SNAPSHOT_FILENAME);
    fs::write(path, payload)?;
    Ok(())
}

pub(crate) fn build_scheduler_snapshot(
    tasks: &[ScheduledTask],
    now: DateTime<Utc>,
) -> SchedulerSnapshot {
    let window_end = now + chrono::Duration::days(SCHEDULER_SNAPSHOT_WINDOW_DAYS);
    let mut upcoming = Vec::new();
    let mut omitted_past_due = 0usize;
    let mut omitted_after_window = 0usize;
    let mut total_enabled = 0usize;

    for task in tasks {
        if !task.enabled {
            continue;
        }
        total_enabled += 1;
        let next_run = schedule_next_run_at(&task.schedule);
        if next_run < now {
            omitted_past_due += 1;
            continue;
        }
        if next_run > window_end {
            omitted_after_window += 1;
            continue;
        }
        upcoming.push(SchedulerSnapshotTask {
            id: task.id.to_string(),
            kind: task_kind_label(&task.kind).to_string(),
            schedule: snapshot_schedule(&task.schedule),
            next_run,
            last_run: task.last_run,
            status: task_status_label(task, now),
            label: task_label(&task.kind),
        });
    }

    upcoming.sort_by_key(|task| task.next_run);

    SchedulerSnapshot {
        generated_at: now,
        window_start: now,
        window_end,
        total_enabled,
        upcoming,
        omitted_past_due,
        omitted_after_window,
    }
}

fn snapshot_schedule(schedule: &Schedule) -> SchedulerSnapshotSchedule {
    match schedule {
        Schedule::Cron {
            expression,
            next_run,
        } => SchedulerSnapshotSchedule::Cron {
            expression: expression.clone(),
            next_run: next_run.clone(),
        },
        Schedule::OneShot { run_at } => SchedulerSnapshotSchedule::OneShot {
            run_at: run_at.clone(),
        },
    }
}

fn schedule_next_run_at(schedule: &Schedule) -> DateTime<Utc> {
    match schedule {
        Schedule::Cron { next_run, .. } => next_run.clone(),
        Schedule::OneShot { run_at } => run_at.clone(),
    }
}

fn task_status_label(task: &ScheduledTask, now: DateTime<Utc>) -> String {
    if !task.enabled {
        if task.last_run.is_some() {
            return "completed".to_string();
        }
        return "disabled".to_string();
    }
    if task.is_due(now) {
        "due".to_string()
    } else {
        "scheduled".to_string()
    }
}

fn task_label(kind: &TaskKind) -> Option<String> {
    match kind {
        TaskKind::SendReply(task) => {
            if task.subject.trim().is_empty() {
                None
            } else {
                Some(truncate_label(task.subject.trim(), 120))
            }
        }
        TaskKind::RunTask(task) => {
            if let Some(thread_id) = task
                .thread_id
                .as_deref()
                .filter(|value| !value.trim().is_empty())
            {
                Some(truncate_label(thread_id, 120))
            } else {
                task.workspace_dir
                    .file_name()
                    .map(|value| truncate_label(&value.to_string_lossy(), 120))
            }
        }
        TaskKind::Noop => None,
    }
}

fn truncate_label(value: &str, max_len: usize) -> String {
    if value.len() <= max_len {
        return value.to_string();
    }
    let mut end = max_len.saturating_sub(1);
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &value[..end])
}
