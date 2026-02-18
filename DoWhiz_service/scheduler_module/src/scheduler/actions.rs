use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

use crate::channel::Channel;
use crate::thread_state::{current_thread_epoch, default_thread_state_path};

use super::core::Scheduler;
use super::executor::TaskExecutor;
use super::reply::load_reply_context;
use super::schedule::{next_run_after, validate_cron_expression};
use super::types::{RunTaskTask, Schedule, SchedulerError, SendReplyTask, TaskKind};
use super::utils::parse_datetime;

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

pub(crate) fn ingest_follow_up_tasks<E: TaskExecutor>(
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

pub(crate) fn schedule_auto_reply<E: TaskExecutor>(
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

    // Non-email channels use plain text reply_message.txt
    // Email and GoogleDocs use HTML reply_email_draft.html
    let (reply_filename, attachments_dirname) = match task.channel {
        Channel::Slack
        | Channel::Discord
        | Channel::BlueBubbles
        | Channel::Telegram
        | Channel::Sms => ("reply_message.txt", "reply_attachments"),
        Channel::Email | Channel::GoogleDocs => {
            ("reply_email_draft.html", "reply_email_attachments")
        }
    };

    let html_path = task.workspace_dir.join(reply_filename);
    if !html_path.exists() {
        warn!(
            "auto reply missing {} in workspace {}",
            reply_filename,
            task.workspace_dir.display()
        );
        return Ok(false);
    }
    let attachments_dir = task.workspace_dir.join(attachments_dirname);
    let reply_context = load_reply_context(&task.workspace_dir);
    let reply_from = task.reply_from.clone().or(reply_context.from.clone());

    let send_task = SendReplyTask {
        channel: task.channel.clone(),
        subject: reply_context.subject,
        html_path,
        attachments_dir,
        from: reply_from,
        to: task.reply_to.clone(),
        cc: Vec::new(),
        bcc: Vec::new(),
        in_reply_to: reply_context.in_reply_to,
        references: reply_context.references,
        archive_root: task.archive_root.clone(),
        thread_epoch: task.thread_epoch,
        thread_state_path: task.thread_state_path.clone(),
    };

    let task_id =
        scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendReply(send_task))?;
    info!(
        "scheduled auto reply task {} from {} via {:?}",
        task_id,
        task.workspace_dir.display(),
        task.channel
    );
    Ok(true)
}

pub(crate) fn schedule_send_email<E: TaskExecutor>(
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

    let reply_context = load_reply_context(&task.workspace_dir);
    let from = request
        .from
        .as_deref()
        .map(str::trim)
        .filter(|value: &&str| !value.is_empty())
        .map(|value: &str| value.to_string())
        .or_else(|| task.reply_from.clone())
        .or_else(|| reply_context.from.clone());

    let send_task = SendReplyTask {
        channel: task.channel.clone(),
        subject: request.subject.clone(),
        html_path,
        attachments_dir,
        from,
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
                let task_id = scheduler.add_one_shot_at(run_at, TaskKind::SendReply(send_task))?;
                info!(
                    "scheduled follow-up send_email task {} from {} run_at={} via {:?}",
                    task_id,
                    task.workspace_dir.display(),
                    run_at.to_rfc3339(),
                    task.channel
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

    let delay_seconds = request.delay_seconds.or_else(|| {
        request
            .delay_minutes
            .map(|value: i64| value.saturating_mul(60))
    });
    let delay_seconds: u64 = match delay_seconds {
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
        TaskKind::SendReply(send_task),
    )?;
    info!(
        "scheduled follow-up send_email task {} from {} delay_seconds={}",
        task_id,
        task.workspace_dir.display(),
        delay_seconds
    );
    Ok(true)
}

pub(crate) fn apply_scheduler_actions<E: TaskExecutor>(
    scheduler: &mut Scheduler<E>,
    task: &RunTaskTask,
    actions: &[run_task_module::SchedulerActionRequest],
) -> Result<(), SchedulerError> {
    if actions.is_empty() {
        return Ok(());
    }
    let now = Utc::now();
    let mut canceled = 0usize;
    let mut rescheduled = 0usize;
    let mut created = 0usize;
    let mut skipped = 0usize;

    for action in actions {
        match action {
            run_task_module::SchedulerActionRequest::Cancel { task_ids } => {
                let (ids, invalid) = parse_action_task_ids(task_ids);
                if !invalid.is_empty() {
                    warn!("scheduler actions invalid task ids: {:?}", invalid);
                }
                if ids.is_empty() {
                    skipped += 1;
                    continue;
                }
                canceled += scheduler.disable_tasks_by(|task| ids.contains(&task.id))?;
            }
            run_task_module::SchedulerActionRequest::Reschedule { task_id, schedule } => {
                let task_id = match Uuid::parse_str(task_id) {
                    Ok(id) => id,
                    Err(_) => {
                        warn!("scheduler actions invalid task id: {}", task_id);
                        skipped += 1;
                        continue;
                    }
                };
                let target = scheduler.tasks.iter_mut().find(|task| task.id == task_id);
                let target = match target {
                    Some(target) => target,
                    None => {
                        warn!("scheduler actions task not found: {}", task_id);
                        skipped += 1;
                        continue;
                    }
                };
                match resolve_schedule_request(schedule, now) {
                    Ok(new_schedule) => {
                        target.schedule = new_schedule;
                        target.enabled = true;
                        scheduler.store.update_task(target)?;
                        rescheduled += 1;
                    }
                    Err(err) => {
                        warn!(
                            "scheduler actions invalid schedule for {}: {}",
                            task_id, err
                        );
                        skipped += 1;
                    }
                }
            }
            run_task_module::SchedulerActionRequest::CreateRunTask {
                schedule,
                model_name,
                codex_disabled,
                reply_to,
            } => {
                let schedule = match resolve_schedule_request(schedule, now) {
                    Ok(schedule) => schedule,
                    Err(err) => {
                        warn!(
                            "scheduler actions invalid create_run_task schedule: {}",
                            err
                        );
                        skipped += 1;
                        continue;
                    }
                };
                let mut new_task = task.clone();
                if let Some(model_name) =
                    model_name.as_ref().filter(|value| !value.trim().is_empty())
                {
                    new_task.model_name = model_name.to_string();
                }
                if let Some(codex_disabled) = codex_disabled {
                    new_task.codex_disabled = *codex_disabled;
                }
                if !reply_to.is_empty() {
                    new_task.reply_to = reply_to.clone();
                }
                match schedule {
                    Schedule::Cron { expression, .. } => {
                        scheduler.add_cron_task(&expression, TaskKind::RunTask(new_task))?;
                        created += 1;
                    }
                    Schedule::OneShot { run_at } => {
                        scheduler.add_one_shot_at(run_at, TaskKind::RunTask(new_task))?;
                        created += 1;
                    }
                }
            }
        }
    }

    info!(
        "scheduler actions applied workspace={} canceled={} rescheduled={} created={} skipped={}",
        task.workspace_dir.display(),
        canceled,
        rescheduled,
        created,
        skipped
    );
    Ok(())
}

fn parse_action_task_ids(task_ids: &[String]) -> (HashSet<Uuid>, Vec<String>) {
    let mut ids = HashSet::new();
    let mut invalid = Vec::new();
    for raw in task_ids {
        match Uuid::parse_str(raw) {
            Ok(id) => {
                ids.insert(id);
            }
            Err(_) => invalid.push(raw.clone()),
        }
    }
    (ids, invalid)
}

pub(crate) fn resolve_schedule_request(
    schedule: &run_task_module::ScheduleRequest,
    now: DateTime<Utc>,
) -> Result<Schedule, SchedulerError> {
    match schedule {
        run_task_module::ScheduleRequest::Cron { expression } => {
            validate_cron_expression(expression)?;
            let next_run = next_run_after(expression, now)?;
            Ok(Schedule::Cron {
                expression: expression.clone(),
                next_run,
            })
        }
        run_task_module::ScheduleRequest::OneShot { run_at } => {
            let run_at = parse_datetime(run_at)?;
            if run_at < now {
                return Err(SchedulerError::TaskFailed(
                    "one_shot run_at is in the past".to_string(),
                ));
            }
            Ok(Schedule::OneShot { run_at })
        }
    }
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
    if rel
        .components()
        .any(|comp| matches!(comp, Component::ParentDir))
    {
        return None;
    }
    Some(root.join(rel))
}
