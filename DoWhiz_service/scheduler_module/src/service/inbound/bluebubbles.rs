use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::channel::{Channel, InboundAdapter};
use crate::index_store::IndexStore;
use crate::user_store::UserStore;
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::scheduler::cancel_pending_thread_tasks;
use super::super::workspace::ensure_thread_workspace;
use super::super::BoxError;

pub(crate) fn process_bluebubbles_event(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::adapters::bluebubbles::BlueBubblesInboundAdapter;

    info!("processing BlueBubbles event");

    let adapter = BlueBubblesInboundAdapter::new();
    let message = adapter.parse(raw_payload)?;

    info!(
        "iMessage from {} in chat {:?}: {:?}",
        message.sender, message.metadata.bluebubbles_chat_guid, message.text_body
    );

    // Get chat GUID (required for BlueBubbles)
    let chat_guid = message
        .metadata
        .bluebubbles_chat_guid
        .as_ref()
        .ok_or("missing bluebubbles_chat_guid")?;

    let user = user_store.get_or_create_user("phone", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Thread key: chat_guid for grouping conversations
    let thread_key = format!("imessage:{}", chat_guid);

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

    // Save the incoming BlueBubbles message to workspace
    append_bluebubbles_message(
        &workspace,
        &message,
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
        reply_to: vec![chat_guid.clone()],
        reply_from: None,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::BlueBubbles,
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

/// Append a BlueBubbles message to the workspace inbox.
pub(super) fn append_bluebubbles_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u32,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    // Save raw payload for debugging/archival
    let raw_path = incoming_dir.join(format!("{:05}_bluebubbles_raw.json", seq));
    std::fs::write(&raw_path, raw_payload)?;

    // Save message text as a simple text file
    let text_path = incoming_dir.join(format!("{:05}_bluebubbles_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    std::fs::write(&text_path, &text_content)?;

    // Create a metadata file with sender info
    let meta_path = incoming_dir.join(format!("{:05}_bluebubbles_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "bluebubbles",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "chat_guid": message.metadata.bluebubbles_chat_guid,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved BlueBubbles message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}
