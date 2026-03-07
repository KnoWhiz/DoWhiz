use chrono::{DateTime, Utc};
use serde::Deserialize;
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

/// Cross-channel routing configuration written by codex.
#[derive(Debug, Clone, Deserialize)]
struct ReplyRouting {
    channel: String,
    identifier: String,
}

/// Load cross-channel routing from reply_routing.json if it exists.
fn load_reply_routing(workspace_dir: &Path) -> Option<ReplyRouting> {
    let path = workspace_dir.join("reply_routing.json");
    let content = std::fs::read_to_string(&path).ok()?;
    match serde_json::from_str(&content) {
        Ok(routing) => {
            info!("Loaded cross-channel routing from {}", path.display());
            Some(routing)
        }
        Err(e) => {
            warn!("Failed to parse reply_routing.json: {}", e);
            None
        }
    }
}

/// Parse channel string to Channel enum.
fn parse_channel(channel_str: &str) -> Option<Channel> {
    match channel_str.to_lowercase().as_str() {
        "email" => Some(Channel::Email),
        "slack" => Some(Channel::Slack),
        "discord" => Some(Channel::Discord),
        "telegram" => Some(Channel::Telegram),
        "wechat" | "weixin" => Some(Channel::WeChat),
        "sms" => Some(Channel::Sms),
        "whatsapp" => Some(Channel::WhatsApp),
        "bluebubbles" => Some(Channel::BlueBubbles),
        _ => {
            warn!("Unknown channel in reply_routing.json: {}", channel_str);
            None
        }
    }
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

    // For Google Workspace channels (Docs, Sheets, Slides), the Claude agent
    // already replies to comments via CLI during task execution, so we skip
    // the auto_reply to avoid duplicate replies.
    if matches!(
        task.channel,
        Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides
    ) {
        info!(
            "skip auto reply for Google Workspace channel {:?} in {} (Claude replies via CLI)",
            task.channel,
            task.workspace_dir.display()
        );
        return Ok(false);
    }

    // Check for cross-channel routing override
    let (target_channel, target_recipients, is_cross_channel) = if let Some(routing) = load_reply_routing(&task.workspace_dir) {
        if routing.identifier.trim().is_empty() {
            warn!("Empty identifier in reply_routing.json, falling back to inbound channel");
            (task.channel.clone(), task.reply_to.clone(), false)
        } else if let Some(channel) = parse_channel(&routing.channel) {
            info!(
                "Cross-channel routing: {} -> {:?} (identifier: {})",
                task.channel, channel, routing.identifier
            );
            (channel, vec![routing.identifier], true)
        } else {
            // Invalid channel in routing file, fall back to inbound
            (task.channel.clone(), task.reply_to.clone(), false)
        }
    } else {
        // No routing file, use inbound channel
        (task.channel.clone(), task.reply_to.clone(), false)
    };

    // Non-email channels use plain text reply_message.txt
    // Email and Google Workspace use HTML reply_email_draft.html
    // Use TARGET channel to determine reply format (for cross-channel routing)
    let (reply_filename, attachments_dirname) = match target_channel {
        Channel::Slack
        | Channel::Discord
        | Channel::BlueBubbles
        | Channel::Telegram
        | Channel::WeChat
        | Channel::WhatsApp
        | Channel::Sms => ("reply_message.txt", "reply_attachments"),
        Channel::Email | Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => {
            ("reply_email_draft.html", "reply_email_attachments")
        }
    };

    let html_path = task.workspace_dir.join(reply_filename);
    if !html_path.exists() {
        if task.channel != target_channel {
            // Cross-channel routing was requested but codex likely wrote the wrong file format
            warn!(
                "Cross-channel routing to {:?} requested but {} not found in {} (codex may have written wrong format)",
                target_channel,
                reply_filename,
                task.workspace_dir.display()
            );
        } else {
            warn!(
                "auto reply missing {} in workspace {}",
                reply_filename,
                task.workspace_dir.display()
            );
        }
        return Ok(false);
    }
    let attachments_dir = task.workspace_dir.join(attachments_dirname);
    let reply_context = load_reply_context(&task.workspace_dir);
    let reply_from = task.reply_from.clone().or(reply_context.from.clone());

    // For cross-channel routing, don't pass inbound thread context to outbound channel
    let (in_reply_to, references) = if is_cross_channel {
        (None, None)
    } else {
        (reply_context.in_reply_to.clone(), reply_context.references.clone())
    };

    // If cross-channel routing, first send an acknowledgement on the inbound channel
    if is_cross_channel {
        // Determine ack file format based on INBOUND channel
        let (ack_filename, ack_attachments_dirname) = match task.channel {
            Channel::Slack
            | Channel::Discord
            | Channel::BlueBubbles
            | Channel::Telegram
            | Channel::WeChat
            | Channel::WhatsApp
            | Channel::Sms => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email | Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => {
                ("cross_channel_ack.html", "reply_email_attachments")
            }
        };

        let ack_path = task.workspace_dir.join(ack_filename);
        let ack_message = format!(
            "The request was successfully completed! I've sent my response to you on {}.",
            format_channel_name(&target_channel)
        );

        // For email, wrap in basic HTML
        let ack_content = if ack_filename.ends_with(".html") {
            format!("<html><body><p>{}</p></body></html>", ack_message)
        } else {
            ack_message
        };

        if let Err(e) = std::fs::write(&ack_path, &ack_content) {
            warn!("Failed to write cross-channel acknowledgement file: {}", e);
        } else {
            // Schedule acknowledgement on inbound channel
            let ack_task = SendReplyTask {
                channel: task.channel.clone(),
                subject: reply_context.subject.clone(),
                html_path: ack_path,
                attachments_dir: task.workspace_dir.join(ack_attachments_dirname),
                from: reply_from.clone(),
                to: task.reply_to.clone(),
                cc: Vec::new(),
                bcc: Vec::new(),
                in_reply_to: reply_context.in_reply_to.clone(),
                references: reply_context.references.clone(),
                archive_root: task.archive_root.clone(),
                thread_epoch: task.thread_epoch,
                thread_state_path: task.thread_state_path.clone(),
                employee_id: task.employee_id.clone(),
            };

            let ack_task_id =
                scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendReply(ack_task))?;
            info!(
                "scheduled cross-channel acknowledgement task {} on {:?}",
                ack_task_id, task.channel
            );
        }
    }

    let send_task = SendReplyTask {
        channel: target_channel.clone(),
        subject: reply_context.subject,
        html_path,
        attachments_dir,
        from: reply_from,
        to: target_recipients,
        cc: Vec::new(),
        bcc: Vec::new(),
        in_reply_to,
        references,
        archive_root: task.archive_root.clone(),
        thread_epoch: task.thread_epoch,
        thread_state_path: task.thread_state_path.clone(),
        employee_id: task.employee_id.clone(),
    };

    let task_id =
        scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::SendReply(send_task))?;
    info!(
        "scheduled auto reply task {} from {} via {:?}",
        task_id,
        task.workspace_dir.display(),
        target_channel
    );
    Ok(true)
}

/// Format channel name for user-friendly display in acknowledgement messages.
fn format_channel_name(channel: &Channel) -> &'static str {
    match channel {
        Channel::Email => "Email",
        Channel::Slack => "Slack",
        Channel::Discord => "Discord",
        Channel::Telegram => "Telegram",
        Channel::WeChat => "WeChat",
        Channel::Sms => "SMS",
        Channel::WhatsApp => "WhatsApp",
        Channel::BlueBubbles => "iMessage",
        Channel::GoogleDocs => "Google Docs",
        Channel::GoogleSheets => "Google Sheets",
        Channel::GoogleSlides => "Google Slides",
    }
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
        employee_id: task.employee_id.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn parse_channel_valid_channels() {
        assert_eq!(parse_channel("email"), Some(Channel::Email));
        assert_eq!(parse_channel("slack"), Some(Channel::Slack));
        assert_eq!(parse_channel("discord"), Some(Channel::Discord));
        assert_eq!(parse_channel("telegram"), Some(Channel::Telegram));
        assert_eq!(parse_channel("wechat"), Some(Channel::WeChat));
        assert_eq!(parse_channel("weixin"), Some(Channel::WeChat));
        assert_eq!(parse_channel("sms"), Some(Channel::Sms));
        assert_eq!(parse_channel("whatsapp"), Some(Channel::WhatsApp));
        assert_eq!(parse_channel("bluebubbles"), Some(Channel::BlueBubbles));
    }

    #[test]
    fn parse_channel_case_insensitive() {
        assert_eq!(parse_channel("EMAIL"), Some(Channel::Email));
        assert_eq!(parse_channel("Slack"), Some(Channel::Slack));
        assert_eq!(parse_channel("DISCORD"), Some(Channel::Discord));
        assert_eq!(parse_channel("WeChat"), Some(Channel::WeChat));
    }

    #[test]
    fn parse_channel_unknown_returns_none() {
        assert_eq!(parse_channel("unknown"), None);
        assert_eq!(parse_channel("fax"), None);
        assert_eq!(parse_channel(""), None);
    }

    #[test]
    fn load_reply_routing_parses_valid_json() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        let routing_json = r#"{"channel": "discord", "identifier": "123456789"}"#;
        fs::write(workspace.join("reply_routing.json"), routing_json).expect("write");

        let routing = load_reply_routing(workspace).expect("should parse");
        assert_eq!(routing.channel, "discord");
        assert_eq!(routing.identifier, "123456789");
    }

    #[test]
    fn load_reply_routing_returns_none_when_missing() {
        let temp = TempDir::new().expect("tempdir");
        let routing = load_reply_routing(temp.path());
        assert!(routing.is_none());
    }

    #[test]
    fn load_reply_routing_returns_none_for_invalid_json() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        fs::write(workspace.join("reply_routing.json"), "not valid json").expect("write");

        let routing = load_reply_routing(workspace);
        assert!(routing.is_none());
    }

    #[test]
    fn load_reply_routing_returns_none_for_missing_fields() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        // Missing identifier field
        let routing_json = r#"{"channel": "discord"}"#;
        fs::write(workspace.join("reply_routing.json"), routing_json).expect("write");

        let routing = load_reply_routing(workspace);
        assert!(routing.is_none());
    }

    #[test]
    fn format_channel_name_returns_friendly_names() {
        assert_eq!(format_channel_name(&Channel::Email), "Email");
        assert_eq!(format_channel_name(&Channel::Slack), "Slack");
        assert_eq!(format_channel_name(&Channel::Discord), "Discord");
        assert_eq!(format_channel_name(&Channel::Telegram), "Telegram");
        assert_eq!(format_channel_name(&Channel::WeChat), "WeChat");
        assert_eq!(format_channel_name(&Channel::Sms), "SMS");
        assert_eq!(format_channel_name(&Channel::WhatsApp), "WhatsApp");
        assert_eq!(format_channel_name(&Channel::BlueBubbles), "iMessage");
        assert_eq!(format_channel_name(&Channel::GoogleDocs), "Google Docs");
        assert_eq!(format_channel_name(&Channel::GoogleSheets), "Google Sheets");
        assert_eq!(format_channel_name(&Channel::GoogleSlides), "Google Slides");
    }

    #[test]
    fn cross_channel_ack_file_format_for_slack_inbound() {
        // For Slack/Discord inbound, ack should be plain text
        let channel = Channel::Slack;
        let (ack_filename, _) = match channel {
            Channel::Slack
            | Channel::Discord
            | Channel::BlueBubbles
            | Channel::Telegram
            | Channel::WeChat
            | Channel::WhatsApp
            | Channel::Sms => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email | Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => {
                ("cross_channel_ack.html", "reply_email_attachments")
            }
        };
        assert_eq!(ack_filename, "cross_channel_ack.txt");
    }

    #[test]
    fn cross_channel_ack_file_format_for_email_inbound() {
        // For Email inbound, ack should be HTML
        let channel = Channel::Email;
        let (ack_filename, _) = match channel {
            Channel::Slack
            | Channel::Discord
            | Channel::BlueBubbles
            | Channel::Telegram
            | Channel::WeChat
            | Channel::WhatsApp
            | Channel::Sms => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email | Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => {
                ("cross_channel_ack.html", "reply_email_attachments")
            }
        };
        assert_eq!(ack_filename, "cross_channel_ack.html");
    }

    #[test]
    fn cross_channel_ack_content_plain_text() {
        let target_channel = Channel::Discord;
        let ack_filename = "cross_channel_ack.txt";
        let ack_message = format!(
            "The request has been successfully completed! I've sent my response to you on {}.",
            format_channel_name(&target_channel)
        );

        let ack_content = if ack_filename.ends_with(".html") {
            format!("<html><body><p>{}</p></body></html>", ack_message)
        } else {
            ack_message.clone()
        };

        assert_eq!(ack_content, "The request has been successfully completed! I've sent my response to you on Discord.");
        assert!(!ack_content.contains("<html>"));
    }

    #[test]
    fn cross_channel_ack_content_html() {
        let target_channel = Channel::Slack;
        let ack_filename = "cross_channel_ack.html";
        let ack_message = format!(
            "The request has been successfully completed! I've sent my response to you on {}.",
            format_channel_name(&target_channel)
        );

        let ack_content = if ack_filename.ends_with(".html") {
            format!("<html><body><p>{}</p></body></html>", ack_message)
        } else {
            ack_message
        };

        assert!(ack_content.contains("<html>"));
        assert!(ack_content.contains("Slack"));
    }

    #[test]
    fn cross_channel_ack_file_written_correctly() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        let target_channel = Channel::Discord;
        let ack_filename = "cross_channel_ack.txt";
        let ack_path = workspace.join(ack_filename);
        let ack_message = format!(
            "The request has been successfully completed! I've sent my response to you on {}.",
            format_channel_name(&target_channel)
        );

        fs::write(&ack_path, &ack_message).expect("write ack file");

        assert!(ack_path.exists());
        let content = fs::read_to_string(&ack_path).expect("read ack file");
        assert!(content.contains("Discord"));
        assert!(content.contains("I've sent my response"));
    }
}
