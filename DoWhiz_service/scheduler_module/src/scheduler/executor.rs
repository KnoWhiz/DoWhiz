use std::path::Path;
use tracing::{info, warn};

use crate::account_store::{
    get_global_account_store, lookup_account_by_channel, lookup_account_by_identifier,
};
use crate::blob_store::get_blob_store;
use crate::channel::Channel;
use crate::github_inbound::{
    extract_github_sender_login_from_postmark_payload, is_github_notifications_postmark_payload,
};
use crate::memory_diff::compute_memory_diff;
use crate::memory_queue::{global_memory_queue, MemoryWriteRequest};
use crate::memory_store::{
    read_memo_content, resolve_user_memory_dir, snapshot_memo_content,
    sync_user_memory_to_workspace,
};
use crate::secrets_store::{
    resolve_user_secrets_path, sync_user_secrets_to_workspace, sync_workspace_secrets_to_user,
};
use crate::thread_state::{current_thread_epoch, find_thread_state_path};
use uuid::Uuid;

/// Sync memo from Azure Blob to workspace directory.
/// Returns the memo content if successful, None otherwise.
fn sync_blob_memo_to_workspace(account_id: Uuid, workspace_memory_dir: &Path) -> Option<String> {
    let blob_store = get_blob_store()?;

    // Create a runtime for the async blob read
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            warn!("Failed to create tokio runtime for blob read: {}", e);
            return None;
        }
    };

    // Read memo from blob
    let memo_content = match rt.block_on(blob_store.read_memo(account_id)) {
        Ok(content) => content,
        Err(e) => {
            warn!(
                "Failed to read memo from blob for account {}: {}",
                account_id, e
            );
            return None;
        }
    };

    // Ensure workspace memory directory exists
    if let Err(e) = std::fs::create_dir_all(workspace_memory_dir) {
        warn!("Failed to create workspace memory dir: {}", e);
        return None;
    }

    // Write memo to workspace
    let memo_path = workspace_memory_dir.join("memo.md");
    if let Err(e) = std::fs::write(&memo_path, &memo_content) {
        warn!("Failed to write blob memo to workspace: {}", e);
        return None;
    }

    info!(
        "Synced memo from Azure Blob (account {}) to workspace ({} bytes)",
        account_id,
        memo_content.len()
    );

    Some(memo_content)
}

use super::outbound::{
    execute_bluebubbles_send, execute_discord_send, execute_email_send, execute_google_docs_send,
    execute_slack_send, execute_sms_send, execute_telegram_send, execute_whatsapp_send,
};
use super::types::{SchedulerError, TaskExecution, TaskKind};
use super::utils::load_google_access_token_from_service_env;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GitHubInboundContext {
    is_github_notification: bool,
    sender_login: Option<String>,
}

fn load_github_inbound_context(task: &super::types::RunTaskTask) -> GitHubInboundContext {
    if task.channel != Channel::Email {
        return GitHubInboundContext {
            is_github_notification: false,
            sender_login: None,
        };
    }
    let payload_path = task
        .workspace_dir
        .join(&task.input_email_dir)
        .join("postmark_payload.json");
    let payload = match std::fs::read(payload_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            return GitHubInboundContext {
                is_github_notification: false,
                sender_login: None,
            };
        }
    };
    GitHubInboundContext {
        is_github_notification: is_github_notifications_postmark_payload(&payload),
        sender_login: extract_github_sender_login_from_postmark_payload(&payload),
    }
}

fn resolve_account_for_run_task(
    task: &super::types::RunTaskTask,
    github_sender: Option<&str>,
) -> Option<Uuid> {
    if let Some(identifier) = task.reply_to.first() {
        if let Some(account_id) = lookup_account_by_channel(&task.channel, identifier) {
            return Some(account_id);
        }
    }

    if task.channel == Channel::Email {
        if let Some(github_sender) = github_sender {
            return lookup_account_by_identifier("github", github_sender);
        }
    }

    None
}

fn write_github_link_required_reply(
    task: &super::types::RunTaskTask,
    github_sender: &str,
) -> Result<(), SchedulerError> {
    let reply_path = task.workspace_dir.join("reply_email_draft.html");
    let attachments_dir = task.workspace_dir.join("reply_email_attachments");
    std::fs::create_dir_all(&attachments_dir)?;

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<body>
  <p>Hi there,</p>
  <p>I received a GitHub request from <strong>@{github_sender}</strong>.</p>
  <p>Before I can execute GitHub-driven tasks, please link this GitHub account to your DoWhiz account and make sure the account has available balance.</p>
  <p>You can link it from the DoWhiz auth page by adding identifier type <code>github</code> with identifier <code>{github_sender}</code>.</p>
  <p>After linking, send the request again and I will continue right away.</p>
  <p>Thanks!</p>
</body>
</html>
"#
    );
    std::fs::write(reply_path, html)?;
    Ok(())
}

fn write_github_sender_parse_failed_reply(
    task: &super::types::RunTaskTask,
) -> Result<(), SchedulerError> {
    let reply_path = task.workspace_dir.join("reply_email_draft.html");
    let attachments_dir = task.workspace_dir.join("reply_email_attachments");
    std::fs::create_dir_all(&attachments_dir)?;

    let html = r#"<!DOCTYPE html>
<html>
<body>
  <p>Hi there,</p>
  <p>I received a GitHub notification email, but I could not deterministically extract the requesting GitHub login from the message payload.</p>
  <p>For security, I did not execute the task. Please resend from the original GitHub notification format (the one that includes lines like <code>&lt;login&gt; left a comment</code> or <code>&lt;login&gt; created an issue</code>).</p>
  <p>After that, I can reliably identify the requester and continue.</p>
  <p>Thanks!</p>
</body>
</html>
"#;
    std::fs::write(reply_path, html)?;
    Ok(())
}

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
                    Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides => {
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
                let github_inbound = load_github_inbound_context(task);
                let account_id =
                    resolve_account_for_run_task(task, github_inbound.sender_login.as_deref());

                if task.channel == Channel::Email && github_inbound.is_github_notification {
                    match github_inbound.sender_login.as_deref() {
                        Some(github_sender) if account_id.is_none() => {
                            write_github_link_required_reply(task, github_sender)?;
                            info!(
                                "skipping run_task for github notification sender={} (no linked github account)",
                                github_sender
                            );
                            return Ok(TaskExecution::empty());
                        }
                        Some(_) => {}
                        None => {
                            write_github_sender_parse_failed_reply(task)?;
                            info!(
                                "skipping run_task for github notification: unable to extract sender login"
                            );
                            return Ok(TaskExecution::empty());
                        }
                    }
                }

                let workspace_memory_dir = task.workspace_dir.join(&task.memory_dir);
                let user_memory_dir = resolve_user_memory_dir(task);
                let user_secrets_path = resolve_user_secrets_path(task);

                // Sync memo to workspace: prefer Azure Blob if account exists, else local storage
                let original_memo_snapshot = if let Some(account_id) = account_id {
                    // User has a unified account - try to sync from Azure Blob
                    info!(
                        "Found unified account {} for channel {:?}, syncing from Azure Blob",
                        account_id, task.channel
                    );
                    match sync_blob_memo_to_workspace(account_id, &workspace_memory_dir) {
                        Some(content) => {
                            // Successfully synced from blob - use blob content as snapshot
                            Some(content)
                        }
                        None => {
                            // Blob sync failed - fall back to local storage
                            warn!(
                                "Blob sync failed for account {}, falling back to local storage",
                                account_id
                            );
                            if let Some(user_memory_dir) = user_memory_dir.as_ref() {
                                let snapshot = snapshot_memo_content(user_memory_dir);
                                if let Err(e) = sync_user_memory_to_workspace(
                                    user_memory_dir,
                                    &workspace_memory_dir,
                                ) {
                                    warn!("Local memory sync also failed: {}", e);
                                }
                                snapshot
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    // No unified account - use local storage (original behavior)
                    let snapshot = user_memory_dir
                        .as_ref()
                        .and_then(|dir| snapshot_memo_content(dir));

                    if let Some(user_memory_dir) = user_memory_dir.as_ref() {
                        sync_user_memory_to_workspace(user_memory_dir, &workspace_memory_dir)
                            .map_err(|err| {
                                SchedulerError::TaskFailed(format!("memory sync failed: {}", err))
                            })?;
                    } else {
                        warn!(
                            "unable to resolve user memory dir for workspace {}",
                            task.workspace_dir.display()
                        );
                    }
                    snapshot
                };
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
                    has_unified_account: account_id.is_some(),
                };
                let output = run_task_module::run_task(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;

                // Track token usage for accounts
                if let Some(account_id) = account_id {
                    if let Some(ref usage) = output.token_usage {
                        let total_tokens = (usage.input_tokens + usage.output_tokens) as i64;
                        if let Some(store) = get_global_account_store() {
                            if let Err(e) = store.add_tokens(account_id, total_tokens) {
                                warn!(
                                    "Failed to update token usage for account {}: {}",
                                    account_id, e
                                );
                            } else {
                                info!(
                                    "Recorded {} tokens for account {} (input: {}, output: {})",
                                    total_tokens,
                                    account_id,
                                    usage.input_tokens,
                                    usage.output_tokens
                                );
                            }
                        }
                    }
                }

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
                                    account_id,
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
                                    if let Err(e) =
                                        crate::memory_store::sync_workspace_memory_to_user(
                                            &workspace_memory_dir,
                                            user_memory_dir,
                                        )
                                    {
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

#[cfg(test)]
mod tests {
    use super::super::types::RunTaskTask;
    use super::*;
    use crate::channel::Channel;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn sample_email_task(workspace_dir: PathBuf) -> RunTaskTask {
        RunTaskTask {
            workspace_dir,
            input_email_dir: PathBuf::from("incoming_email"),
            input_attachments_dir: PathBuf::from("incoming_attachments"),
            memory_dir: PathBuf::from("memory"),
            reference_dir: PathBuf::from("references"),
            model_name: "gpt-5.3-codex".to_string(),
            runner: "codex".to_string(),
            codex_disabled: true,
            reply_to: vec!["reply@example.com".to_string()],
            reply_from: Some("service@example.com".to_string()),
            archive_root: None,
            thread_id: Some("thread-1".to_string()),
            thread_epoch: Some(1),
            thread_state_path: None,
            channel: Channel::Email,
            slack_team_id: None,
            employee_id: Some("little_bear".to_string()),
        }
    }

    #[test]
    fn load_github_inbound_context_reads_postmark_payload() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path().to_path_buf();
        let incoming_email = workspace.join("incoming_email");
        fs::create_dir_all(&incoming_email).expect("incoming_email");
        fs::write(
            incoming_email.join("postmark_payload.json"),
            r#"{
  "From": "notifications@github.com",
  "Headers": [{"Name": "X-GitHub-Sender", "Value": "bingran-you"}]
}"#,
        )
        .expect("postmark_payload.json");

        let task = sample_email_task(workspace);
        let context = load_github_inbound_context(&task);
        assert_eq!(
            context,
            GitHubInboundContext {
                is_github_notification: true,
                sender_login: Some("bingran-you".to_string()),
            }
        );
    }

    #[test]
    fn load_github_inbound_context_detects_unparseable_github_notification() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path().to_path_buf();
        let incoming_email = workspace.join("incoming_email");
        fs::create_dir_all(&incoming_email).expect("incoming_email");
        fs::write(
            incoming_email.join("postmark_payload.json"),
            r#"{
  "From": "notifications@github.com",
  "TextBody": "No activity line here"
}"#,
        )
        .expect("postmark_payload.json");

        let task = sample_email_task(workspace);
        let context = load_github_inbound_context(&task);
        assert_eq!(
            context,
            GitHubInboundContext {
                is_github_notification: true,
                sender_login: None,
            }
        );
    }

    #[test]
    fn write_github_link_required_reply_writes_template() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path().to_path_buf();
        let task = sample_email_task(workspace.clone());

        write_github_link_required_reply(&task, "bingran-you").expect("write template");

        let reply_path = workspace.join("reply_email_draft.html");
        let body = fs::read_to_string(reply_path).expect("reply body");
        assert!(body.contains("@bingran-you"));
        assert!(body.contains("identifier type <code>github</code>"));
        assert!(workspace.join("reply_email_attachments").is_dir());
    }

    #[test]
    fn write_github_sender_parse_failed_reply_writes_template() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path().to_path_buf();
        let task = sample_email_task(workspace.clone());

        write_github_sender_parse_failed_reply(&task).expect("write template");

        let reply_path = workspace.join("reply_email_draft.html");
        let body = fs::read_to_string(reply_path).expect("reply body");
        assert!(body.contains("could not deterministically extract"));
        assert!(body.contains("did not execute the task"));
        assert!(workspace.join("reply_email_attachments").is_dir());
    }
}
