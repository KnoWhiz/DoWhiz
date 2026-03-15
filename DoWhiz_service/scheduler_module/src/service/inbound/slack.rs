use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::account_store::AccountStore;
use crate::adapters::slack::SlackEventWrapper;
use crate::channel::{Channel, InboundAdapter};
use crate::index_store::IndexStore;
use crate::slack_store::SlackStore;
use crate::user_store::UserStore;
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::scheduler::cancel_pending_thread_tasks;
use super::super::workspace::ensure_thread_workspace;
use super::super::BoxError;

pub(crate) fn process_slack_event(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    slack_store: &SlackStore,
    account_store: &AccountStore,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::adapters::slack::SlackInboundAdapter;

    info!("processing slack event");

    // Parse wrapper first to get team_id
    let wrapper: SlackEventWrapper = serde_json::from_slice(raw_payload)?;

    // Look up bot_user_id from SlackStore (with fallback to env var)
    let team_id = wrapper.team_id.as_deref().unwrap_or("");
    let mut bot_user_ids = HashSet::new();
    if let Ok(installation) = slack_store.get_installation_or_env(team_id) {
        if !installation.bot_user_id.is_empty() {
            bot_user_ids.insert(installation.bot_user_id);
        }
    } else if let Some(ref bot_id) = config.slack_bot_user_id {
        // Legacy fallback
        bot_user_ids.insert(bot_id.clone());
    }
    let adapter = SlackInboundAdapter::new(bot_user_ids);

    // Check if this is a bot message (should be ignored)
    if let Some(ref event) = wrapper.event {
        if adapter.is_bot_message(event) {
            info!("ignoring bot message from user {:?}", event.user);
            return Ok(());
        }
    }

    let message = adapter.parse(raw_payload)?;

    info!(
        "slack message from {} in channel {:?}: {:?}",
        message.sender, message.metadata.slack_channel_id, message.text_body
    );

    // Get channel ID (required for Slack)
    let channel_id = message
        .metadata
        .slack_channel_id
        .as_ref()
        .ok_or("missing slack_channel_id")?;

    let user = user_store.get_or_create_user("slack", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Thread key: channel_id + thread_id for grouping conversations
    let thread_key = format!("slack:{}:{}", channel_id, message.thread_id);

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

    // Save the incoming Slack message to workspace
    append_slack_message(
        &workspace,
        &message,
        raw_payload,
        thread_state.last_email_seq,
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
        // reply_to[0] = user_id (for account lookup), reply_to[1] = channel_id
        reply_to: vec![message.sender.clone(), channel_id.clone()],
        reply_from: None, // Slack uses bot token, not a "from" address
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Slack,
        slack_team_id: message.metadata.slack_team_id.clone(),
        employee_id: Some(config.employee_profile.id.clone()),
        requester_identifier_type: None,
        requester_identifier: None,
    };

    // Clone run_task before consuming it, in case we need to write to account-level storage
    let run_task_for_account = run_task.clone();

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

    // If the Slack user has linked their account, also write to account-level tasks.db
    // message.sender contains the Slack user ID
    if let Ok(Some(account)) = account_store.get_account_by_identifier("slack", &message.sender) {
        let account_tasks_dir = config.users_root.join(account.id.to_string()).join("state");
        if let Err(err) = std::fs::create_dir_all(&account_tasks_dir) {
            warn!(
                "failed to create account tasks dir for account {}: {}",
                account.id, err
            );
        } else {
            let account_tasks_db_path = account_tasks_dir.join("tasks.db");
            match Scheduler::load(&account_tasks_db_path, ModuleExecutor::default()) {
                Ok(mut account_scheduler) => {
                    // Use the same task_id so we can update status at completion
                    match account_scheduler.add_one_shot_in_with_id(
                        task_id,
                        Duration::from_secs(0),
                        TaskKind::RunTask(run_task_for_account),
                    ) {
                        Ok(()) => {
                            info!(
                                "also enqueued task to account-level storage account={} task_id={}",
                                account.id, task_id
                            );
                        }
                        Err(err) => {
                            warn!(
                                "failed to add task to account scheduler for account {}: {}",
                                account.id, err
                            );
                        }
                    }
                }
                Err(err) => {
                    warn!(
                        "failed to load account scheduler for account {}: {}",
                        account.id, err
                    );
                }
            }
        }
    }

    Ok(())
}

/// Save an incoming Slack message to the workspace.
pub(super) fn append_slack_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    // Save the raw JSON payload
    let raw_path = incoming_dir.join(format!("{:05}_slack_raw.json", seq));
    std::fs::write(&raw_path, raw_payload)?;

    // Save message text as a simple text file (similar to email body)
    let text_path = incoming_dir.join(format!("{:05}_slack_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    std::fs::write(&text_path, &text_content)?;

    // Create a metadata file with sender info
    let meta_path = incoming_dir.join(format!("{:05}_slack_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "slack",
        "sender": message.sender,
        "channel_id": message.metadata.slack_channel_id,
        "team_id": message.metadata.slack_team_id,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved Slack message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}
