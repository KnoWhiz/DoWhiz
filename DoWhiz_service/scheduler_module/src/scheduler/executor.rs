use tracing::{info, warn};

use crate::channel::Channel;
use crate::memory_diff::compute_memory_diff;
use crate::memory_queue::{global_memory_queue, MemoryWriteRequest};
use crate::memory_store::{
    read_memo_content, resolve_user_memory_dir, snapshot_memo_content, sync_user_memory_to_workspace,
};
use crate::secrets_store::{
    resolve_user_secrets_path, sync_user_secrets_to_workspace, sync_workspace_secrets_to_user,
};
use crate::thread_state::{current_thread_epoch, find_thread_state_path};

use super::outbound::{
    execute_bluebubbles_send, execute_discord_send, execute_email_send, execute_google_docs_send,
    execute_slack_send, execute_sms_send, execute_telegram_send, execute_whatsapp_send,
};
use super::types::{SchedulerError, TaskExecution, TaskKind};
use super::utils::load_google_access_token_from_service_env;

pub trait TaskExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError>;
}

#[derive(Debug, Default, Clone)]
pub struct ModuleExecutor;

impl TaskExecutor for ModuleExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        match task {
            TaskKind::SendReply(task) => {
                if let Some(expected_epoch) = task.thread_epoch {
                    let state_path = task
                        .thread_state_path
                        .clone()
                        .or_else(|| task.html_path.parent().and_then(find_thread_state_path));
                    if let Some(state_path) = state_path {
                        if let Some(current_epoch) = current_thread_epoch(&state_path) {
                            if current_epoch != expected_epoch {
                                info!(
                                    "skip stale send_email (expected epoch {}, current {}) for {}",
                                    expected_epoch,
                                    current_epoch,
                                    task.html_path.display()
                                );
                                return Ok(TaskExecution::empty());
                            }
                        }
                    }
                }

                // Dispatch to the appropriate adapter based on channel
                match task.channel {
                    Channel::Slack => {
                        execute_slack_send(task)?;
                    }
                    Channel::Discord => {
                        execute_discord_send(task)?;
                    }
                    Channel::GoogleDocs => {
                        execute_google_docs_send(task)?;
                    }
                    Channel::Sms => {
                        execute_sms_send(task)?;
                    }
                    Channel::BlueBubbles => {
                        execute_bluebubbles_send(task)?;
                    }
                    Channel::Telegram => {
                        execute_telegram_send(task)?;
                    }
                    Channel::WhatsApp => {
                        execute_whatsapp_send(task)?;
                    }
                    Channel::Email => {
                        execute_email_send(task)?;
                    }
                }
                Ok(TaskExecution::empty())
            }
            TaskKind::RunTask(task) => {
                let workspace_memory_dir = task.workspace_dir.join(&task.memory_dir);
                let user_memory_dir = resolve_user_memory_dir(task);
                let user_secrets_path = resolve_user_secrets_path(task);

                // Snapshot original memo content before syncing to workspace
                let original_memo_snapshot = user_memory_dir
                    .as_ref()
                    .and_then(|dir| snapshot_memo_content(dir));

                if let Some(user_memory_dir) = user_memory_dir.as_ref() {
                    sync_user_memory_to_workspace(user_memory_dir, &workspace_memory_dir).map_err(
                        |err| SchedulerError::TaskFailed(format!("memory sync failed: {}", err)),
                    )?;
                } else {
                    warn!(
                        "unable to resolve user memory dir for workspace {}",
                        task.workspace_dir.display()
                    );
                }
                if let Some(user_secrets_path) = user_secrets_path.as_ref() {
                    sync_user_secrets_to_workspace(user_secrets_path, &task.workspace_dir)
                        .map_err(|err| {
                            SchedulerError::TaskFailed(format!("secrets sync failed: {}", err))
                        })?;
                } else {
                    warn!(
                        "unable to resolve user secrets for workspace {}",
                        task.workspace_dir.display()
                    );
                }
                let params = run_task_module::RunTaskParams {
                    workspace_dir: task.workspace_dir.clone(),
                    input_email_dir: task.input_email_dir.clone(),
                    input_attachments_dir: task.input_attachments_dir.clone(),
                    memory_dir: task.memory_dir.clone(),
                    reference_dir: task.reference_dir.clone(),
                    reply_to: task.reply_to.clone(),
                    model_name: task.model_name.clone(),
                    runner: task.runner.clone(),
                    codex_disabled: task.codex_disabled,
                    channel: task.channel.to_string(),
                    google_access_token: load_google_access_token_from_service_env(),
                };
                let output = run_task_module::run_task(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;

                // After task completes, compute diff and submit to queue instead of direct sync
                if let Some(user_memory_dir) = user_memory_dir.as_ref() {
                    if let Some(original_content) = original_memo_snapshot {
                        // Read modified workspace memo
                        if let Some(modified_content) = read_memo_content(&workspace_memory_dir) {
                            let diff = compute_memory_diff(&original_content, &modified_content);
                            if !diff.is_empty() {
                                // Extract user_id from path: users/{user_id}/memory
                                let user_id = user_memory_dir
                                    .parent()
                                    .and_then(|p| p.file_name())
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("unknown")
                                    .to_string();

                                let request = MemoryWriteRequest {
                                    user_id: user_id.clone(),
                                    user_memory_dir: user_memory_dir.clone(),
                                    diff,
                                };

                                // Submit to queue - blocks until worker applies the diff
                                if let Err(e) = global_memory_queue().submit(request) {
                                    warn!(
                                        "Failed to submit memory diff to queue for user {}: {}",
                                        user_id, e
                                    );
                                    // Fall back to direct sync on queue failure
                                    if let Err(e) = crate::memory_store::sync_workspace_memory_to_user(
                                        &workspace_memory_dir,
                                        user_memory_dir,
                                    ) {
                                        warn!("Fallback memory sync also failed: {}", e);
                                    }
                                }
                            }
                            // If diff is empty, no changes to sync
                        }
                    } else {
                        // No snapshot available, fall back to direct sync
                        warn!("No original memo snapshot, falling back to direct sync");
                        if let Err(e) = crate::memory_store::sync_workspace_memory_to_user(
                            &workspace_memory_dir,
                            user_memory_dir,
                        ) {
                            warn!("Memory sync failed: {}", e);
                        }
                    }
                }

                if let Some(user_secrets_path) = user_secrets_path.as_ref() {
                    sync_workspace_secrets_to_user(&task.workspace_dir, user_secrets_path)
                        .map_err(|err| {
                            SchedulerError::TaskFailed(format!("secrets sync failed: {}", err))
                        })?;
                }
                Ok(TaskExecution {
                    follow_up_tasks: output.scheduled_tasks,
                    follow_up_error: output.scheduled_tasks_error,
                    scheduler_actions: output.scheduler_actions,
                    scheduler_actions_error: output.scheduler_actions_error,
                })
            }
            TaskKind::Noop => Ok(TaskExecution::empty()),
        }
    }
}
