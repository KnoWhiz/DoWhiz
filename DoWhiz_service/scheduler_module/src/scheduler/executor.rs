use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;
use tracing::{info, warn};

use crate::account_store::{
    get_global_account_store, lookup_account_by_channel, lookup_account_by_identifier,
    AccountIdentifier,
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
use run_task_module::UserIdentities;
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
    execute_slack_send, execute_sms_send, execute_telegram_send, execute_wechat_send,
    execute_whatsapp_send,
};
use super::types::{SchedulerError, SendReplyTask, TaskExecution, TaskKind};
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

/// Fetch user identities from the account store for cross-channel routing.
fn fetch_user_identities(account_id: Option<Uuid>) -> UserIdentities {
    let Some(account_id) = account_id else {
        return UserIdentities::default();
    };

    let Some(store) = get_global_account_store() else {
        warn!("Account store not available for fetching user identities");
        return UserIdentities::default();
    };

    let identifiers = match store.list_identifiers(account_id) {
        Ok(ids) => ids,
        Err(e) => {
            warn!(
                "Failed to fetch identifiers for account {}: {}",
                account_id, e
            );
            return UserIdentities::default();
        }
    };

    identifiers_to_user_identities(account_id, &identifiers)
}

/// Convert account identifiers to UserIdentities struct for cross-channel routing.
fn identifiers_to_user_identities(
    account_id: Uuid,
    identifiers: &[AccountIdentifier],
) -> UserIdentities {
    let mut result = UserIdentities {
        account_id: Some(account_id.to_string()),
        ..Default::default()
    };

    for identifier in identifiers.iter().filter(|id| id.verified) {
        match identifier.identifier_type.as_str() {
            "email" => result.emails.push(identifier.identifier.clone()),
            "slack" | "slack_user_id" => result.slack_user_ids.push(identifier.identifier.clone()),
            "discord" | "discord_user_id" => {
                result.discord_user_ids.push(identifier.identifier.clone())
            }
            "phone" | "sms" | "whatsapp" | "bluebubbles" => {
                result.phone_numbers.push(identifier.identifier.clone())
            }
            "telegram" | "telegram_user_id" => {
                result.telegram_user_ids.push(identifier.identifier.clone())
            }
            "wechat" | "wechat_user_id" => {
                result.wechat_user_ids.push(identifier.identifier.clone())
            }
            _ => {
                // Unknown identifier type, skip
            }
        }
    }

    result
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

const DISCORD_TYPING_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(8);
const SLACK_WORKING_PLACEHOLDER_FILE: &str = ".slack_working_placeholder.json";
const SLACK_WORKING_PLACEHOLDER_TEXT: &str = "⏳ Working on it...";

fn resolve_discord_bot_token_for_employee(employee_id: Option<&str>) -> Option<String> {
    if let Some(emp_id) = employee_id {
        let emp_upper = emp_id.to_uppercase().replace('-', "_");
        let emp_token_key = format!("{}_DISCORD_BOT_TOKEN", emp_upper);
        if let Ok(token) = std::env::var(&emp_token_key) {
            if !token.trim().is_empty() {
                return Some(token);
            }
        }
    }

    std::env::var("DISCORD_BOT_TOKEN")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn resolve_slack_bot_token_for_employee(employee_id: Option<&str>) -> Option<String> {
    if let Some(emp_id) = employee_id {
        let emp_upper = emp_id.to_uppercase().replace('-', "_");
        let emp_token_key = format!("{}_SLACK_BOT_TOKEN", emp_upper);
        if let Ok(token) = std::env::var(&emp_token_key) {
            if !token.trim().is_empty() {
                return Some(token);
            }
        }
    }

    std::env::var("SLACK_BOT_TOKEN")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn slack_channel_and_thread_from_thread_key(thread_key: Option<&str>) -> Option<(String, String)> {
    let raw = thread_key
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    let mut parts = raw.splitn(3, ':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("slack"), Some(channel_id), Some(thread_ts))
            if !channel_id.trim().is_empty() && !thread_ts.trim().is_empty() =>
        {
            Some((channel_id.trim().to_string(), thread_ts.trim().to_string()))
        }
        _ => None,
    }
}

fn slack_channel_and_thread(task: &super::types::RunTaskTask) -> Option<(String, String)> {
    if task.channel != Channel::Slack {
        return None;
    }
    if let Some(pair) = slack_channel_and_thread_from_thread_key(task.thread_id.as_deref()) {
        return Some(pair);
    }

    let channel_id = task.reply_to.get(1).map(|value| value.trim()).unwrap_or("");
    let thread_ts = task
        .thread_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .or_else(|| {
            super::reply::load_reply_context(&task.workspace_dir)
                .in_reply_to
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })?;

    if channel_id.is_empty() {
        return None;
    }

    Some((channel_id.to_string(), thread_ts))
}

fn slack_placeholder_path(workspace_dir: &Path) -> std::path::PathBuf {
    workspace_dir.join(SLACK_WORKING_PLACEHOLDER_FILE)
}

fn write_slack_placeholder(
    workspace_dir: &Path,
    channel_id: &str,
    thread_ts: &str,
    message_ts: &str,
) -> Result<(), SchedulerError> {
    let path = slack_placeholder_path(workspace_dir);
    let payload = serde_json::json!({
        "channel_id": channel_id,
        "thread_ts": thread_ts,
        "message_ts": message_ts,
    });
    let serialized = serde_json::to_vec_pretty(&payload)
        .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
    std::fs::write(path, serialized)?;
    Ok(())
}

fn post_slack_working_placeholder(task: &super::types::RunTaskTask) {
    let Some((channel_id, thread_ts)) = slack_channel_and_thread(task) else {
        return;
    };
    let Some(bot_token) = resolve_slack_bot_token_for_employee(task.employee_id.as_deref()) else {
        return;
    };

    let marker_path = slack_placeholder_path(&task.workspace_dir);
    if marker_path.is_file() {
        return;
    }

    let api_base =
        std::env::var("SLACK_API_BASE_URL").unwrap_or_else(|_| "https://slack.com/api".to_string());
    let url = format!("{}/chat.postMessage", api_base.trim_end_matches('/'));
    let request = serde_json::json!({
        "channel": channel_id,
        "thread_ts": thread_ts,
        "text": SLACK_WORKING_PLACEHOLDER_TEXT,
        "mrkdwn": true
    });

    let client = reqwest::blocking::Client::new();
    let response = match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
    {
        Ok(response) => response,
        Err(err) => {
            warn!(
                "failed to send slack working placeholder for employee={} channel={}: {}",
                task.employee_id.as_deref().unwrap_or_default(),
                channel_id,
                err
            );
            return;
        }
    };

    let status = response.status();
    let body = match response.text() {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "failed reading slack working placeholder response for employee={} channel={}: {}",
                task.employee_id.as_deref().unwrap_or_default(),
                channel_id,
                err
            );
            return;
        }
    };
    if !status.is_success() {
        warn!(
            "slack working placeholder failed for employee={} channel={} status={} body={}",
            task.employee_id.as_deref().unwrap_or_default(),
            channel_id,
            status,
            body
        );
        return;
    }

    let payload: serde_json::Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "failed parsing slack working placeholder response for employee={} channel={}: {}",
                task.employee_id.as_deref().unwrap_or_default(),
                channel_id,
                err
            );
            return;
        }
    };

    let ok = payload.get("ok").and_then(|value| value.as_bool()) == Some(true);
    let message_ts = payload
        .get("ts")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());
    if !ok || message_ts.is_none() {
        warn!(
            "slack working placeholder API error for employee={} channel={} error={}",
            task.employee_id.as_deref().unwrap_or_default(),
            channel_id,
            payload
                .get("error")
                .and_then(|value| value.as_str())
                .unwrap_or("unknown")
        );
        return;
    }

    if let Some(message_ts) = message_ts {
        if let Err(err) =
            write_slack_placeholder(&task.workspace_dir, &channel_id, &thread_ts, &message_ts)
        {
            warn!(
                "failed writing slack placeholder marker for workspace {}: {}",
                task.workspace_dir.display(),
                err
            );
        }
    }
}

fn find_slack_placeholder_marker(task: &SendReplyTask) -> Option<std::path::PathBuf> {
    if let Some(state_path) = task.thread_state_path.as_ref() {
        if let Some(workspace_dir) = state_path.parent() {
            let marker = slack_placeholder_path(workspace_dir);
            if marker.is_file() {
                return Some(marker);
            }
        }
    }

    let workspace_dir = task.html_path.parent()?;
    let marker = slack_placeholder_path(workspace_dir);
    if marker.is_file() {
        return Some(marker);
    }
    None
}

fn load_slack_placeholder_marker(marker_path: &Path) -> Option<(String, String)> {
    let content = std::fs::read_to_string(marker_path).ok()?;
    let payload: serde_json::Value = serde_json::from_str(&content).ok()?;
    let channel_id = payload
        .get("channel_id")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())?
        .to_string();
    let message_ts = payload
        .get("message_ts")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())?
        .to_string();
    Some((channel_id, message_ts))
}

fn clear_slack_placeholder_marker(path: &Path) {
    if let Err(err) = std::fs::remove_file(path) {
        warn!(
            "failed to remove slack placeholder marker {}: {}",
            path.display(),
            err
        );
    }
}

fn delete_slack_working_placeholder_before_send(task: &SendReplyTask) {
    if task.channel != Channel::Slack {
        return;
    }
    let Some(marker_path) = find_slack_placeholder_marker(task) else {
        return;
    };
    let Some((channel_id, message_ts)) = load_slack_placeholder_marker(&marker_path) else {
        clear_slack_placeholder_marker(&marker_path);
        return;
    };
    let Some(bot_token) = resolve_slack_bot_token_for_employee(task.employee_id.as_deref()) else {
        return;
    };

    let api_base =
        std::env::var("SLACK_API_BASE_URL").unwrap_or_else(|_| "https://slack.com/api".to_string());
    let url = format!("{}/chat.delete", api_base.trim_end_matches('/'));
    let request = serde_json::json!({
        "channel": channel_id,
        "ts": message_ts
    });

    let client = reqwest::blocking::Client::new();
    let response = match client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
    {
        Ok(response) => response,
        Err(err) => {
            warn!(
                "failed to delete slack working placeholder for {}: {}",
                task.html_path.display(),
                err
            );
            return;
        }
    };

    let status = response.status();
    let body = match response.text() {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "failed reading slack delete placeholder response for {}: {}",
                task.html_path.display(),
                err
            );
            return;
        }
    };
    if !status.is_success() {
        warn!(
            "slack delete placeholder failed for {} status={} body={}",
            task.html_path.display(),
            status,
            body
        );
        return;
    }

    let payload: serde_json::Value = match serde_json::from_str(&body) {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "failed parsing slack delete placeholder response for {}: {}",
                task.html_path.display(),
                err
            );
            return;
        }
    };

    let ok = payload.get("ok").and_then(|value| value.as_bool()) == Some(true);
    let error = payload.get("error").and_then(|value| value.as_str());
    if ok || matches!(error, Some("message_not_found")) {
        clear_slack_placeholder_marker(&marker_path);
    } else {
        warn!(
            "slack delete placeholder API error for {}: {}",
            task.html_path.display(),
            error.unwrap_or("unknown")
        );
    }
}

fn discord_typing_channel_id(task: &super::types::RunTaskTask) -> Option<u64> {
    if task.channel != Channel::Discord {
        return None;
    }
    // Prefer the legacy slot (index 1) if present, otherwise use index 0.
    if let Some(channel_id) = task
        .reply_to
        .get(1)
        .and_then(|value| value.parse::<u64>().ok())
    {
        return Some(channel_id);
    }
    task.reply_to
        .first()
        .and_then(|value| value.parse::<u64>().ok())
}

struct DiscordTypingHeartbeat {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl DiscordTypingHeartbeat {
    fn start(task: &super::types::RunTaskTask) -> Option<Self> {
        let channel_id = discord_typing_channel_id(task)?;
        let bot_token = resolve_discord_bot_token_for_employee(task.employee_id.as_deref())?;
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);
        let worker_employee_id = task.employee_id.clone().unwrap_or_default();

        let handle = std::thread::spawn(move || {
            let api_base = std::env::var("DISCORD_API_BASE_URL")
                .unwrap_or_else(|_| "https://discord.com/api/v10".to_string());
            let url = format!(
                "{}/channels/{}/typing",
                api_base.trim_end_matches('/'),
                channel_id
            );
            let client = reqwest::blocking::Client::new();
            let mut logged_failure = false;

            while !stop_clone.load(Ordering::Relaxed) {
                match client
                    .post(&url)
                    .header("Authorization", format!("Bot {}", bot_token))
                    .header("Content-Type", "application/json")
                    .send()
                {
                    Ok(response) if response.status().is_success() => {
                        if logged_failure {
                            info!(
                                "discord typing heartbeat recovered for employee={} channel={}",
                                worker_employee_id, channel_id
                            );
                            logged_failure = false;
                        }
                    }
                    Ok(response) => {
                        if !logged_failure {
                            warn!(
                                "discord typing heartbeat failed for employee={} channel={} status={}",
                                worker_employee_id,
                                channel_id,
                                response.status()
                            );
                            logged_failure = true;
                        }
                    }
                    Err(err) => {
                        if !logged_failure {
                            warn!(
                                "discord typing heartbeat request error for employee={} channel={}: {}",
                                worker_employee_id, channel_id, err
                            );
                            logged_failure = true;
                        }
                    }
                }

                let mut slept = Duration::from_millis(0);
                while slept < DISCORD_TYPING_HEARTBEAT_INTERVAL {
                    if stop_clone.load(Ordering::Relaxed) {
                        return;
                    }
                    let remaining = DISCORD_TYPING_HEARTBEAT_INTERVAL.saturating_sub(slept);
                    let step = remaining.min(Duration::from_millis(250));
                    std::thread::sleep(step);
                    slept += step;
                }
            }
        });

        Some(Self {
            stop,
            handle: Some(handle),
        })
    }
}

impl Drop for DiscordTypingHeartbeat {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
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
                        delete_slack_working_placeholder_before_send(task);
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
                    Channel::WeChat => {
                        execute_wechat_send(task)?;
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
                let _typing_heartbeat = DiscordTypingHeartbeat::start(task);
                post_slack_working_placeholder(task);

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
                // Check balance before running task (only for unified accounts)
                if let Some(account_id) = account_id {
                    if let Some(store) = get_global_account_store() {
                        match store.has_sufficient_balance(account_id) {
                            Ok(false) => {
                                // Insufficient balance - write error reply and skip task
                                warn!(
                                    "Account {} has insufficient balance, skipping task execution",
                                    account_id
                                );
                                let reply_message = "Insufficient balance. Please increase your balance for more employee hours.";

                                // Write to appropriate reply file based on channel
                                let reply_path = match task.channel {
                                    Channel::Email
                                    | Channel::GoogleDocs
                                    | Channel::GoogleSheets
                                    | Channel::GoogleSlides => {
                                        task.workspace_dir.join("reply_email_draft.html")
                                    }
                                    _ => task.workspace_dir.join("reply_message.txt"),
                                };

                                if let Err(e) = std::fs::write(&reply_path, reply_message) {
                                    warn!("Failed to write balance error reply: {}", e);
                                }

                                // Return empty execution (no token usage, task considered complete)
                                return Ok(TaskExecution::empty());
                            }
                            Ok(true) => {
                                // Sufficient balance, continue with task
                            }
                            Err(e) => {
                                // Balance check failed - log but continue (fail open)
                                warn!(
                                    "Failed to check balance for account {}: {}, continuing anyway",
                                    account_id, e
                                );
                            }
                        }
                    }
                }

                let user_identities = fetch_user_identities(account_id);
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
                    user_identities,
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
            model_name: "gpt-5.4".to_string(),
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
            requester_identifier_type: None,
            requester_identifier: None,
        }
    }

    fn run_task_with_reply_to(channel: Channel, reply_to: Vec<&str>) -> RunTaskTask {
        RunTaskTask {
            workspace_dir: PathBuf::from("."),
            input_email_dir: PathBuf::from("incoming_email"),
            input_attachments_dir: PathBuf::from("incoming_attachments"),
            memory_dir: PathBuf::from("memory"),
            reference_dir: PathBuf::from("references"),
            model_name: "gpt-5".to_string(),
            runner: "codex".to_string(),
            codex_disabled: false,
            reply_to: reply_to.into_iter().map(str::to_string).collect(),
            reply_from: None,
            archive_root: None,
            thread_id: None,
            thread_epoch: None,
            thread_state_path: None,
            channel,
            slack_team_id: None,
            employee_id: None,
            requester_identifier_type: None,
            requester_identifier: None,
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

    #[test]
    fn typing_channel_id_prefers_second_slot_when_present() {
        let task = run_task_with_reply_to(Channel::Discord, vec!["123", "456"]);
        assert_eq!(discord_typing_channel_id(&task), Some(456));
    }

    #[test]
    fn typing_channel_id_falls_back_to_first_slot() {
        let task = run_task_with_reply_to(Channel::Discord, vec!["456"]);
        assert_eq!(discord_typing_channel_id(&task), Some(456));
    }

    #[test]
    fn typing_channel_id_returns_none_for_non_discord() {
        let task = run_task_with_reply_to(Channel::Slack, vec!["456"]);
        assert_eq!(discord_typing_channel_id(&task), None);
    }

    #[test]
    fn typing_channel_id_returns_none_when_reply_to_is_not_numeric() {
        let task = run_task_with_reply_to(Channel::Discord, vec!["abc", "def"]);
        assert_eq!(discord_typing_channel_id(&task), None);
    }

    #[test]
    fn slack_channel_and_thread_from_thread_key_parses_compound_key() {
        assert_eq!(
            slack_channel_and_thread_from_thread_key(Some("slack:C12345:1700000000.123456")),
            Some(("C12345".to_string(), "1700000000.123456".to_string()))
        );
    }

    #[test]
    fn slack_channel_and_thread_from_thread_key_rejects_non_slack_key() {
        assert_eq!(
            slack_channel_and_thread_from_thread_key(Some("discord:123:456")),
            None
        );
        assert_eq!(
            slack_channel_and_thread_from_thread_key(Some("1700000000.123456")),
            None
        );
    }

    #[test]
    fn find_slack_placeholder_marker_finds_workspace_marker() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let marker = workspace.join(SLACK_WORKING_PLACEHOLDER_FILE);
        fs::write(
            &marker,
            r#"{"channel_id":"C123","thread_ts":"1700.1","message_ts":"1700.2"}"#,
        )
        .expect("marker");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(&reply_path, "hello").expect("reply");

        let send_task = SendReplyTask {
            channel: Channel::Slack,
            subject: "Slack reply".to_string(),
            html_path: reply_path,
            attachments_dir: workspace.join("reply_attachments"),
            from: None,
            to: vec!["U123".to_string(), "C123".to_string()],
            cc: vec![],
            bcc: vec![],
            in_reply_to: Some("1700.1".to_string()),
            references: None,
            archive_root: None,
            thread_epoch: None,
            thread_state_path: Some(workspace.join("thread_state.json")),
            employee_id: Some("little_bear".to_string()),
        };

        let found = find_slack_placeholder_marker(&send_task).expect("marker found");
        assert_eq!(found, marker);
    }

    #[test]
    fn load_slack_placeholder_marker_reads_channel_and_message_ts() {
        let temp = TempDir::new().expect("tempdir");
        let marker = temp.path().join(SLACK_WORKING_PLACEHOLDER_FILE);
        fs::write(
            &marker,
            r#"{"channel_id":"C123","thread_ts":"1700.1","message_ts":"1700.2"}"#,
        )
        .expect("marker");

        let marker_data = load_slack_placeholder_marker(&marker).expect("marker data");
        assert_eq!(marker_data.0, "C123");
        assert_eq!(marker_data.1, "1700.2");
    }

    fn make_identifier(
        account_id: Uuid,
        identifier_type: &str,
        identifier: &str,
        verified: bool,
    ) -> AccountIdentifier {
        AccountIdentifier {
            id: Uuid::new_v4(),
            account_id,
            identifier_type: identifier_type.to_string(),
            identifier: identifier.to_string(),
            verified,
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn identifiers_to_user_identities_maps_email() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![make_identifier(
            account_id,
            "email",
            "test@example.com",
            true,
        )];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.account_id, Some(account_id.to_string()));
        assert_eq!(result.emails, vec!["test@example.com"]);
    }

    #[test]
    fn identifiers_to_user_identities_maps_slack() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![
            make_identifier(account_id, "slack", "U123456", true),
            make_identifier(account_id, "slack_user_id", "U789012", true),
        ];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.slack_user_ids, vec!["U123456", "U789012"]);
    }

    #[test]
    fn identifiers_to_user_identities_maps_discord() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![make_identifier(
            account_id,
            "discord_user_id",
            "123456789012345678",
            true,
        )];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.discord_user_ids, vec!["123456789012345678"]);
    }

    #[test]
    fn identifiers_to_user_identities_maps_phone_channels() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![
            make_identifier(account_id, "phone", "+15551234567", true),
            make_identifier(account_id, "sms", "+15559876543", true),
        ];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.phone_numbers, vec!["+15551234567", "+15559876543"]);
    }

    #[test]
    fn identifiers_to_user_identities_maps_telegram() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![make_identifier(account_id, "telegram", "12345678", true)];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.telegram_user_ids, vec!["12345678"]);
    }

    #[test]
    fn identifiers_to_user_identities_maps_wechat() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![
            make_identifier(account_id, "wechat", "wxid_abc123", true),
            make_identifier(account_id, "wechat_user_id", "wxid_xyz789", true),
        ];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.wechat_user_ids, vec!["wxid_abc123", "wxid_xyz789"]);
    }

    #[test]
    fn identifiers_to_user_identities_skips_unverified() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![
            make_identifier(account_id, "email", "verified@example.com", true),
            make_identifier(account_id, "email", "unverified@example.com", false),
        ];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.emails, vec!["verified@example.com"]);
    }

    #[test]
    fn identifiers_to_user_identities_skips_unknown_types() {
        let account_id = Uuid::new_v4();
        let identifiers = vec![
            make_identifier(account_id, "email", "test@example.com", true),
            make_identifier(account_id, "fax", "555-1234", true),
            make_identifier(account_id, "carrier_pigeon", "coo-coo", true),
        ];

        let result = identifiers_to_user_identities(account_id, &identifiers);

        assert_eq!(result.emails, vec!["test@example.com"]);
        assert!(result.slack_user_ids.is_empty());
        assert!(result.discord_user_ids.is_empty());
    }
}
