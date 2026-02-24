use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::user_store::UserStore;
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::scheduler::cancel_pending_thread_tasks;
use super::super::workspace::ensure_thread_workspace;
use super::super::BoxError;

pub(crate) fn process_telegram_event(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    info!("processing Telegram event");

    info!(
        "Telegram message from {} in chat {:?}: {:?}",
        message.sender, message.metadata.telegram_chat_id, message.text_body
    );

    // Get chat ID (required for Telegram)
    let chat_id = message
        .metadata
        .telegram_chat_id
        .ok_or("missing telegram_chat_id")?;

    let user = user_store.get_or_create_user("telegram", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Thread key: chat_id for grouping conversations
    let thread_key = format!("telegram:{}", chat_id);

    // Create/get workspace for this thread
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    // Bump thread state
    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    // Save the incoming Telegram message to workspace
    append_telegram_message(
        &workspace,
        message,
        raw_payload,
        thread_state.last_email_seq.try_into().unwrap_or(u32::MAX),
    )?;

    // Determine model and runner
    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config
                .employee_profile
                .runner
                .eq_ignore_ascii_case("claude")
            {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };

    info!(
        "workspace ready at {} for user {} thread={} epoch={}",
        workspace.display(),
        user.user_id,
        thread_key,
        thread_state.epoch
    );

    // Create RunTask to process the message
    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: std::path::PathBuf::from("incoming_email"),
        input_attachments_dir: std::path::PathBuf::from("incoming_attachments"),
        memory_dir: std::path::PathBuf::from("memory"),
        reference_dir: std::path::PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![chat_id.to_string()],
        reply_from: None,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Telegram,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    // Schedule the task
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

/// Append a Telegram message to the workspace inbox.
pub(super) fn append_telegram_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u32,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    // Save raw JSON payload
    let json_filename = format!("{:04}_telegram.json", seq);
    std::fs::write(incoming_dir.join(&json_filename), raw_payload)?;

    // Save text content as .txt file (similar to other messaging platforms)
    if let Some(ref text) = message.text_body {
        let txt_filename = format!("{:04}_telegram.txt", seq);
        let sender_name = message.sender_name.as_deref().unwrap_or(&message.sender);
        let content = format!(
            "From: {} ({})\nDate: {}\n\n{}",
            sender_name,
            message.sender,
            chrono::Utc::now().to_rfc3339(),
            text
        );
        std::fs::write(incoming_dir.join(&txt_filename), content)?;
    }

    Ok(())
}
