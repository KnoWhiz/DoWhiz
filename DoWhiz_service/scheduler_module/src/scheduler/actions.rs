use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

use crate::account_store::{get_global_account_store, lookup_account_by_identifier};
use crate::channel::Channel;
use crate::employee_config;
use crate::service;
use crate::thread_state::{current_thread_epoch, default_thread_state_path};

use super::core::Scheduler;
use super::executor::TaskExecutor;
use super::reply::load_reply_context;
use super::schedule::{next_run_after, validate_cron_expression};
use super::types::{RunTaskTask, Schedule, SchedulerError, SendReplyTask, TaskKind};
use super::utils::parse_datetime;

const SECRET_SCAN_MAX_BYTES: u64 = 512 * 1024;
const SECRET_GUARD_MESSAGE: &str = "For security, I cannot send content that appears to contain credentials or secret tokens. Please resend the request without asking to expose secrets.";
const SECRET_ENV_KEY_MARKERS: &[&str] = &[
    "PASSWORD",
    "SECRET",
    "TOKEN",
    "API_KEY",
    "PRIVATE_KEY",
    "ACCESS_KEY",
    "REFRESH_TOKEN",
    "CLIENT_SECRET",
    "AUTH",
    "CREDENTIAL",
];
const CLOSURE_SIGNAL_MARKERS: &[&str] = &[
    "no further action",
    "nothing further",
    "nothing else needed",
    "no reply required",
    "no reply needed",
    "no reply should be sent",
    "no outbound reply",
    "no outgoing reply",
    "no outbound reply should be sent",
    "no outgoing reply should be sent",
    "do not send another acknowledgement",
    "avoid sending another acknowledgement",
    "wrapped up",
    "all set",
    "thread closed",
    "no action requested",
    "no further changes",
    "pass is complete",
    "done on my side",
    "complete on my side",
    "thread tucked away",
    "tucked away",
    "another generated reply in the same closed loop",
    "another generated reply",
    "same closed loop",
    "not a fresh task",
    "not a new task request",
    "not a new task",
    "已完成",
    "无需进一步",
    "没有进一步",
    "不需要再",
];
const REQUEST_SIGNAL_MARKERS: &[&str] = &[
    "can you",
    "could you",
    "would you",
    "please ",
    "please,",
    "need you to",
    "请你",
    "请帮",
    "麻烦",
];
const CLOSURE_NON_REQUEST_GUIDANCE_MARKERS: &[&str] = &[
    "please start a fresh thread",
    "please start a new thread",
    "please start a fresh email",
    "please start a new email",
    "please send a new task in a fresh thread",
];
const ACTION_SIGNAL_MARKERS: &[&str] = &[
    "action item",
    "next step",
    "todo",
    "to do",
    "follow up",
    "deadline",
    "due by",
];

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

/// Resolve the employee's primary email address from config by employee ID.
fn resolve_employee_primary_email(employee_id: &str) -> Option<String> {
    let config_path = std::env::var("EMPLOYEE_CONFIG_PATH")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(path)
            }
        })
        .unwrap_or_else(service::default_employee_config_path);
    let directory = employee_config::load_employee_directory(&config_path).ok()?;
    let profile = directory.employee(employee_id)?;
    profile.addresses.first().cloned()
}

/// Parse channel string to Channel enum.
fn parse_channel(channel_str: &str) -> Option<Channel> {
    match channel_str.to_lowercase().as_str() {
        "email" => Some(Channel::Email),
        "slack" => Some(Channel::Slack),
        "discord" => Some(Channel::Discord),
        "telegram" => Some(Channel::Telegram),
        "sms" => Some(Channel::Sms),
        "whatsapp" => Some(Channel::WhatsApp),
        "bluebubbles" => Some(Channel::BlueBubbles),
        "wechat" => Some(Channel::WeChat),
        _ => {
            warn!("Unknown channel in reply_routing.json: {}", channel_str);
            None
        }
    }
}

/// Check if a routing identifier is allowed for the given task's account.
/// Returns true if the identifier belongs to the user's linked accounts.
fn is_routing_identifier_allowed(task: &RunTaskTask, identifier: &str) -> bool {
    // Get account ID from requester info
    let account_id = match (
        task.requester_identifier_type.as_deref(),
        task.requester_identifier.as_deref(),
    ) {
        (Some(id_type), Some(id_value)) => lookup_account_by_identifier(id_type, id_value),
        _ => None,
    };

    let Some(account_id) = account_id else {
        // No account linked - allow only original reply_to (conservative default)
        return task.reply_to.contains(&identifier.to_string());
    };

    let Some(store) = get_global_account_store() else {
        warn!("Account store not available for routing validation");
        return task.reply_to.contains(&identifier.to_string());
    };

    let identifiers = match store.list_identifiers(account_id) {
        Ok(ids) => ids,
        Err(e) => {
            warn!(
                "Failed to fetch identifiers for account {} during routing validation: {}",
                account_id, e
            );
            return task.reply_to.contains(&identifier.to_string());
        }
    };

    // Check if the target identifier matches any verified identifier
    let identifier_lower = identifier.to_lowercase();
    for id in &identifiers {
        if !id.verified {
            continue;
        }
        if id.identifier.to_lowercase() == identifier_lower {
            return true;
        }
    }

    false
}

fn is_sensitive_env_key(key: &str) -> bool {
    let upper = key.trim().to_ascii_uppercase();
    SECRET_ENV_KEY_MARKERS
        .iter()
        .any(|marker| upper.contains(marker))
}

fn trim_env_value(raw: &str) -> &str {
    let trimmed = raw.trim();
    if trimmed.len() >= 2 {
        let bytes = trimmed.as_bytes();
        let first = bytes[0];
        let last = bytes[trimmed.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &trimmed[1..trimmed.len() - 1];
        }
    }
    trimmed
}

fn collect_sensitive_workspace_secret_values(workspace_dir: &Path) -> Vec<(String, String)> {
    let env_path = workspace_dir.join(".env");
    let content = match std::fs::read_to_string(&env_path) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };

    let mut seen_values = HashSet::new();
    let mut secrets = Vec::new();

    for line in content.lines() {
        let mut trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix("export ") {
            trimmed = stripped.trim();
        }
        let Some((raw_key, raw_value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = raw_key.trim();
        if key.is_empty() || !is_sensitive_env_key(key) {
            continue;
        }

        let value = trim_env_value(raw_value);
        if value.len() < 8 {
            continue;
        }

        if seen_values.insert(value.to_string()) {
            secrets.push((key.to_string(), value.to_string()));
        }
    }

    secrets
}

fn find_secret_in_file(path: &Path, secrets: &[(String, String)]) -> Option<String> {
    let meta = std::fs::metadata(path).ok()?;
    if !meta.is_file() || meta.len() > SECRET_SCAN_MAX_BYTES {
        return None;
    }

    let bytes = std::fs::read(path).ok()?;
    let content = String::from_utf8_lossy(&bytes);
    for (key, value) in secrets {
        if content.contains(value) {
            return Some(key.clone());
        }
    }
    None
}

fn find_secret_in_dir(dir: &Path, secrets: &[(String, String)]) -> Option<(PathBuf, String)> {
    if !dir.exists() {
        return None;
    }

    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let entries = match std::fs::read_dir(&current) {
            Ok(value) => value,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if let Some(key) = find_secret_in_file(&path, secrets) {
                return Some((path, key));
            }
        }
    }
    None
}

fn is_html_channel(channel: &Channel) -> bool {
    matches!(
        channel,
        Channel::Email | Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides
    )
}

fn write_secret_guard_reply(reply_path: &Path, channel: &Channel) {
    let content = if is_html_channel(channel) {
        format!(
            "<!DOCTYPE html><html><body><p>{}</p></body></html>",
            SECRET_GUARD_MESSAGE
        )
    } else {
        SECRET_GUARD_MESSAGE.to_string()
    };

    if let Err(err) = std::fs::write(reply_path, content) {
        warn!(
            "failed to write secret-guard message to {}: {}",
            reply_path.display(),
            err
        );
    }
}

fn clear_reply_attachments_dir(dir: &Path) {
    if dir.exists() {
        if let Err(err) = std::fs::remove_dir_all(dir) {
            warn!(
                "failed to clear attachments dir {} after secret leak detection: {}",
                dir.display(),
                err
            );
            return;
        }
    }
    if let Err(err) = std::fs::create_dir_all(dir) {
        warn!(
            "failed to recreate attachments dir {} after secret leak detection: {}",
            dir.display(),
            err
        );
    }
}

fn apply_workspace_secret_leak_guard(
    workspace_dir: &Path,
    reply_path: &Path,
    attachments_dir: &Path,
    channel: &Channel,
) -> bool {
    let secrets = collect_sensitive_workspace_secret_values(workspace_dir);
    if secrets.is_empty() {
        return false;
    }

    if let Some(key) = find_secret_in_file(reply_path, &secrets) {
        warn!(
            "blocked outbound reply in {}: reply file {} contains sensitive value from env key {}",
            workspace_dir.display(),
            reply_path.display(),
            key
        );
        write_secret_guard_reply(reply_path, channel);
        clear_reply_attachments_dir(attachments_dir);
        return true;
    }

    if let Some((path, key)) = find_secret_in_dir(attachments_dir, &secrets) {
        warn!(
            "blocked outbound reply in {}: attachment {} contains sensitive value from env key {}",
            workspace_dir.display(),
            path.display(),
            key
        );
        write_secret_guard_reply(reply_path, channel);
        clear_reply_attachments_dir(attachments_dir);
        return true;
    }

    false
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

fn normalize_signal_text(raw: &str) -> String {
    raw.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn has_any_signal(text: &str, markers: &[&str]) -> bool {
    markers.iter().any(|marker| text.contains(marker))
}

fn has_request_signal(text: &str) -> bool {
    let mut sanitized = text.to_string();
    for marker in CLOSURE_NON_REQUEST_GUIDANCE_MARKERS {
        sanitized = sanitized.replace(marker, " ");
    }
    sanitized.contains('?')
        || sanitized.contains('？')
        || has_any_signal(&sanitized, REQUEST_SIGNAL_MARKERS)
}

fn has_action_signal(text: &str) -> bool {
    has_any_signal(text, ACTION_SIGNAL_MARKERS)
}

fn is_closure_only_message(raw: &str) -> bool {
    let text = normalize_signal_text(raw);
    if text.len() < 8 {
        return false;
    }
    has_any_signal(&text, CLOSURE_SIGNAL_MARKERS)
        && !has_request_signal(&text)
        && !has_action_signal(&text)
}

fn strip_html_tags_lossy(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut in_tag = false;
    for ch in raw.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn parse_leading_seq(name: &str) -> u64 {
    let mut seq = String::new();
    for ch in name.chars() {
        if ch.is_ascii_digit() {
            seq.push(ch);
        } else {
            break;
        }
    }
    seq.parse::<u64>().unwrap_or(0)
}

fn normalize_identity_token(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}

fn parse_identity_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(normalize_identity_token)
        .filter(|token| !token.is_empty())
        .collect()
}

fn channel_internal_sender_env_key(channel: Channel) -> Option<&'static str> {
    match channel {
        Channel::Slack => Some("INTERNAL_SLACK_SENDER_IDS"),
        Channel::Discord => Some("INTERNAL_DISCORD_SENDER_IDS"),
        Channel::Telegram => Some("INTERNAL_TELEGRAM_SENDER_IDS"),
        Channel::Sms => Some("INTERNAL_SMS_SENDER_IDS"),
        Channel::WhatsApp => Some("INTERNAL_WHATSAPP_SENDER_IDS"),
        Channel::BlueBubbles => Some("INTERNAL_BLUEBUBBLES_SENDER_IDS"),
        _ => None,
    }
}

fn load_internal_sender_id_whitelist(channel: Channel) -> HashSet<String> {
    let mut allowlist = HashSet::new();

    if let Some(key) = channel_internal_sender_env_key(channel) {
        if let Ok(raw) = std::env::var(key) {
            for token in parse_identity_list(&raw) {
                allowlist.insert(token);
            }
        }
    }

    // Common convenience fallback for Slack bot user id.
    if channel == Channel::Slack {
        if let Ok(bot_user_id) = std::env::var("SLACK_BOT_USER_ID") {
            let token = normalize_identity_token(&bot_user_id);
            if !token.is_empty() {
                allowlist.insert(token);
            }
        }
    }

    allowlist
}

fn do_whiz_service_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd
        .file_name()
        .map(|name| name == "DoWhiz_service")
        .unwrap_or(false)
    {
        cwd
    } else {
        cwd.join("DoWhiz_service")
    }
}

fn collect_internal_email_whitelist_from_paths(config_paths: &[PathBuf]) -> HashSet<String> {
    let mut allowlist = HashSet::new();
    for path in config_paths {
        let directory = match employee_config::load_employee_directory(path) {
            Ok(value) => value,
            Err(_) => continue,
        };
        for employee in directory.employees {
            for address in employee.addresses {
                for token in crate::user_store::extract_emails(&address) {
                    let normalized = normalize_identity_token(&token);
                    if !normalized.is_empty() {
                        allowlist.insert(normalized);
                    }
                }
            }
        }
    }
    allowlist
}

fn load_internal_email_whitelist() -> HashSet<String> {
    let root = do_whiz_service_root();
    let config_paths = vec![
        root.join("employee.toml"),
        root.join("employee.staging.toml"),
    ];
    collect_internal_email_whitelist_from_paths(&config_paths)
}

fn load_latest_email_sender(workspace_dir: &Path) -> Option<String> {
    let payload_path = workspace_dir
        .join("incoming_email")
        .join("postmark_payload.json");
    let raw = std::fs::read_to_string(payload_path).ok()?;
    let payload: serde_json::Value = serde_json::from_str(&raw).ok()?;
    let from = payload
        .get("From")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim();
    if from.is_empty() {
        return None;
    }
    crate::user_store::extract_emails(from)
        .into_iter()
        .next()
        .map(|value| normalize_identity_token(&value))
        .filter(|value| !value.is_empty())
}

fn is_internal_sender(task: &RunTaskTask) -> bool {
    match task.channel {
        Channel::Email => {
            let sender = match load_latest_email_sender(&task.workspace_dir) {
                Some(value) => value,
                None => return false,
            };
            load_internal_email_whitelist().contains(&sender)
        }
        Channel::Slack
        | Channel::Discord
        | Channel::Telegram
        | Channel::Sms
        | Channel::WhatsApp
        | Channel::BlueBubbles
        | Channel::WeChat => {
            let sender = match task.reply_to.first() {
                Some(value) => normalize_identity_token(value),
                None => return false,
            };
            if sender.is_empty() {
                return false;
            }
            let allowlist = load_internal_sender_id_whitelist(task.channel);
            !allowlist.is_empty() && allowlist.contains(&sender)
        }
        Channel::GoogleDocs | Channel::GoogleSheets | Channel::GoogleSlides | Channel::Notion => {
            false
        }
    }
}

fn load_latest_email_text_from_postmark_payload(payload_path: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(payload_path).ok()?;
    let payload: serde_json::Value = serde_json::from_str(&raw).ok()?;

    let stripped = payload
        .get("StrippedTextReply")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if !stripped.is_empty() {
        return Some(stripped);
    }

    let text = payload
        .get("TextBody")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if !text.is_empty() {
        return Some(text);
    }

    let html = payload
        .get("HtmlBody")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if html.is_empty() {
        return None;
    }
    Some(strip_html_tags_lossy(&html))
}

fn is_inbound_body_candidate_file(name: &str) -> bool {
    if name == "email.html" || name == "email.txt" {
        return true;
    }
    name.ends_with("_message.txt")
        || name.ends_with("_telegram.txt")
        || name.ends_with("_whatsapp.txt")
        || name.ends_with("_email.html")
        || name.ends_with("_email.txt")
}

fn load_latest_inbound_body_text(workspace_dir: &Path) -> Option<String> {
    let incoming_dir = workspace_dir.join("incoming_email");
    if !incoming_dir.is_dir() {
        return None;
    }

    let payload_path = incoming_dir.join("postmark_payload.json");
    if let Some(text) = load_latest_email_text_from_postmark_payload(&payload_path) {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    let mut latest: Option<(u64, std::time::SystemTime, PathBuf)> = None;
    let entries = std::fs::read_dir(&incoming_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = match path.file_name().and_then(|value| value.to_str()) {
            Some(value) => value,
            None => continue,
        };
        if !is_inbound_body_candidate_file(name) {
            continue;
        }
        let seq = parse_leading_seq(name);
        let modified = entry
            .metadata()
            .ok()
            .and_then(|meta| meta.modified().ok())
            .unwrap_or(std::time::UNIX_EPOCH);
        match &latest {
            Some((best_seq, best_modified, _))
                if seq < *best_seq || (seq == *best_seq && modified <= *best_modified) => {}
            _ => latest = Some((seq, modified, path)),
        }
    }

    let (_, _, path) = latest?;
    let body = std::fs::read_to_string(&path).ok()?;
    let is_html = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("html") || ext.eq_ignore_ascii_case("htm"))
        .unwrap_or(false);
    let text = if is_html {
        strip_html_tags_lossy(&body)
    } else {
        body
    };
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn load_reply_text(reply_path: &Path) -> Option<String> {
    if !reply_path.exists() {
        return None;
    }
    let raw = std::fs::read_to_string(reply_path).ok()?;
    let is_html = reply_path
        .extension()
        .and_then(|value| value.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("html") || ext.eq_ignore_ascii_case("htm"))
        .unwrap_or(false);
    let text = if is_html {
        strip_html_tags_lossy(&raw)
    } else {
        raw
    };
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn should_skip_closure_loop_reply(
    task: &RunTaskTask,
    reply_path: &Path,
    outbound_channel: Channel,
) -> bool {
    if !is_internal_sender(task) {
        return false;
    }
    let inbound_is_closure_only = load_latest_inbound_body_text(&task.workspace_dir)
        .map(|value| is_closure_only_message(&value))
        .unwrap_or(false);
    let reply_is_closure_only = load_reply_text(reply_path)
        .map(|value| is_closure_only_message(&value))
        .unwrap_or(false);

    if inbound_is_closure_only && reply_is_closure_only {
        info!(
            "skip auto reply closure-loop guard in {} inbound={:?} outbound={:?}",
            task.workspace_dir.display(),
            task.channel,
            outbound_channel
        );
    } else {
        info!(
            "skip auto reply internal-sender guard in {} inbound={:?} outbound={:?}",
            task.workspace_dir.display(),
            task.channel,
            outbound_channel
        );
    }
    true
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
    let (target_channel, target_recipients, is_cross_channel) = if let Some(routing) =
        load_reply_routing(&task.workspace_dir)
    {
        if routing.identifier.trim().is_empty() {
            warn!("Empty identifier in reply_routing.json, falling back to inbound channel");
            (task.channel.clone(), task.reply_to.clone(), false)
        } else if !is_routing_identifier_allowed(task, &routing.identifier) {
            // Security: Block routing to unauthorized identifiers
            warn!(
                "Blocked unauthorized cross-channel routing to '{}' - identifier not in user's linked accounts",
                routing.identifier
            );
            // Write security message to reply file
            let security_message = "To maintain user isolation and privacy, I cannot send messages to recipients outside your linked accounts.";
            let reply_path = match task.channel {
                Channel::Email
                | Channel::GoogleDocs
                | Channel::GoogleSheets
                | Channel::GoogleSlides => {
                    let html = format!(
                        "<!DOCTYPE html><html><body><p>{}</p></body></html>",
                        security_message
                    );
                    let path = task.workspace_dir.join("reply_email_draft.html");
                    let _ = std::fs::write(&path, html);
                    path
                }
                _ => {
                    let path = task.workspace_dir.join("reply_message.txt");
                    let _ = std::fs::write(&path, security_message);
                    path
                }
            };
            info!(
                "Wrote security block message to {} for blocked routing attempt",
                reply_path.display()
            );
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
        | Channel::WhatsApp
        | Channel::Sms
| Channel::Notion
        | Channel::WeChat => ("reply_message.txt", "reply_attachments"),
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
    if should_skip_closure_loop_reply(task, &html_path, target_channel) {
        return Ok(false);
    }
    let attachments_dir = task.workspace_dir.join(attachments_dirname);
    apply_workspace_secret_leak_guard(
        &task.workspace_dir,
        &html_path,
        &attachments_dir,
        &target_channel,
    );

    let reply_context = load_reply_context(&task.workspace_dir);
    let reply_from = if is_cross_channel && matches!(target_channel, Channel::Email) {
        // For cross-channel routing to email, derive sender from employee config
        task.employee_id
            .as_ref()
            .and_then(|id| resolve_employee_primary_email(id))
            .or_else(|| task.reply_from.clone())
            .or(reply_context.from.clone())
    } else {
        task.reply_from.clone().or(reply_context.from.clone())
    };

    // For cross-channel routing, don't pass inbound thread context to outbound channel
    let (in_reply_to, references) = if is_cross_channel {
        (None, None)
    } else {
        (
            reply_context.in_reply_to.clone(),
            reply_context.references.clone(),
        )
    };

    // If cross-channel routing, first send an acknowledgement on the inbound channel
    if is_cross_channel {
        // Determine ack file format based on INBOUND channel
        let (ack_filename, ack_attachments_dirname) = match task.channel {
            Channel::Slack
            | Channel::Discord
            | Channel::BlueBubbles
            | Channel::Telegram
            | Channel::WhatsApp
            | Channel::Sms
| Channel::Notion
            | Channel::WeChat => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("cross_channel_ack.html", "reply_email_attachments"),
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
        Channel::Sms => "SMS",
        Channel::WhatsApp => "WhatsApp",
        Channel::BlueBubbles => "iMessage",
        Channel::WeChat => "WeChat",
        Channel::GoogleDocs => "Google Docs",
        Channel::GoogleSheets => "Google Sheets",
        Channel::GoogleSlides => "Google Slides",
        Channel::Notion => "Notion",
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

    apply_workspace_secret_leak_guard(
        &task.workspace_dir,
        &html_path,
        &attachments_dir,
        &task.channel,
    );

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
        assert_eq!(parse_channel("sms"), Some(Channel::Sms));
        assert_eq!(parse_channel("whatsapp"), Some(Channel::WhatsApp));
        assert_eq!(parse_channel("bluebubbles"), Some(Channel::BlueBubbles));
        assert_eq!(parse_channel("wechat"), Some(Channel::WeChat));
    }

    #[test]
    fn parse_channel_case_insensitive() {
        assert_eq!(parse_channel("EMAIL"), Some(Channel::Email));
        assert_eq!(parse_channel("Slack"), Some(Channel::Slack));
        assert_eq!(parse_channel("DISCORD"), Some(Channel::Discord));
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
        assert_eq!(format_channel_name(&Channel::Sms), "SMS");
        assert_eq!(format_channel_name(&Channel::WhatsApp), "WhatsApp");
        assert_eq!(format_channel_name(&Channel::BlueBubbles), "iMessage");
        assert_eq!(format_channel_name(&Channel::WeChat), "WeChat");
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
            | Channel::WhatsApp
            | Channel::Sms
| Channel::Notion
            | Channel::WeChat => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("cross_channel_ack.html", "reply_email_attachments"),
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
            | Channel::WhatsApp
            | Channel::Sms
| Channel::Notion
            | Channel::WeChat => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("cross_channel_ack.html", "reply_email_attachments"),
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

        assert_eq!(
            ack_content,
            "The request has been successfully completed! I've sent my response to you on Discord."
        );
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

    #[test]
    fn resolve_employee_primary_email_returns_first_address() {
        let temp = TempDir::new().expect("tempdir");
        let config_path = temp.path().join("employee.toml");

        let config_content = r#"
[[employees]]
id = "test_employee"
addresses = ["primary@example.com", "secondary@example.com"]
"#;
        fs::write(&config_path, config_content).expect("write config");

        // Set env var to point to our test config
        std::env::set_var("EMPLOYEE_CONFIG_PATH", config_path.to_str().unwrap());

        let email = resolve_employee_primary_email("test_employee");
        assert_eq!(email, Some("primary@example.com".to_string()));

        // Clean up env var
        std::env::remove_var("EMPLOYEE_CONFIG_PATH");
    }

    #[test]
    fn resolve_employee_primary_email_returns_none_for_unknown_employee() {
        let temp = TempDir::new().expect("tempdir");
        let config_path = temp.path().join("employee.toml");

        let config_content = r#"
[[employees]]
id = "known_employee"
addresses = ["known@example.com"]
"#;
        fs::write(&config_path, config_content).expect("write config");

        std::env::set_var("EMPLOYEE_CONFIG_PATH", config_path.to_str().unwrap());

        let email = resolve_employee_primary_email("unknown_employee");
        assert_eq!(email, None);

        std::env::remove_var("EMPLOYEE_CONFIG_PATH");
    }

    #[test]
    fn resolve_employee_primary_email_with_multiple_employees() {
        let temp = TempDir::new().expect("tempdir");
        let config_path = temp.path().join("employee.toml");

        let config_content = r#"
[[employees]]
id = "little_bear"
addresses = ["oliver@dowhiz.com", "little-bear@dowhiz.com"]

[[employees]]
id = "boiled_egg"
addresses = ["proto@dowhiz.com", "boiled-egg@dowhiz.com"]
"#;
        fs::write(&config_path, config_content).expect("write config");

        std::env::set_var("EMPLOYEE_CONFIG_PATH", config_path.to_str().unwrap());

        assert_eq!(
            resolve_employee_primary_email("little_bear"),
            Some("oliver@dowhiz.com".to_string())
        );
        assert_eq!(
            resolve_employee_primary_email("boiled_egg"),
            Some("proto@dowhiz.com".to_string())
        );

        std::env::remove_var("EMPLOYEE_CONFIG_PATH");
    }

    // Helper to create a minimal RunTaskTask for testing
    fn make_test_task(reply_to: Vec<String>) -> RunTaskTask {
        RunTaskTask {
            workspace_dir: PathBuf::from("."),
            input_email_dir: PathBuf::from("incoming_email"),
            input_attachments_dir: PathBuf::from("incoming_attachments"),
            memory_dir: PathBuf::from("memory"),
            reference_dir: PathBuf::from("references"),
            model_name: "test".to_string(),
            runner: "codex".to_string(),
            codex_disabled: false,
            reply_to,
            reply_from: None,
            archive_root: None,
            thread_id: None,
            thread_epoch: None,
            thread_state_path: None,
            channel: Channel::Email,
            slack_team_id: None,
            employee_id: None,
            requester_identifier_type: None,
            requester_identifier: None,
        }
    }

    #[test]
    fn is_routing_identifier_allowed_no_account_allows_reply_to() {
        // When no account is linked, only reply_to addresses are allowed
        let task = make_test_task(vec!["user@example.com".to_string()]);

        assert!(is_routing_identifier_allowed(&task, "user@example.com"));
        assert!(!is_routing_identifier_allowed(&task, "attacker@evil.com"));
    }

    #[test]
    fn is_routing_identifier_allowed_no_account_blocks_unknown() {
        let task = make_test_task(vec!["legit@example.com".to_string()]);

        // Should block any identifier not in reply_to
        assert!(!is_routing_identifier_allowed(
            &task,
            "not-in-reply-to@example.com"
        ));
        assert!(!is_routing_identifier_allowed(&task, "random@attacker.com"));
    }

    #[test]
    fn is_routing_identifier_allowed_empty_reply_to_blocks_all() {
        let task = make_test_task(vec![]);

        // With no reply_to and no account, everything should be blocked
        assert!(!is_routing_identifier_allowed(&task, "anyone@example.com"));
    }

    #[test]
    fn is_routing_identifier_allowed_case_insensitive_for_reply_to() {
        let task = make_test_task(vec!["User@Example.COM".to_string()]);

        // reply_to comparison should be exact (case-sensitive) since it's a contains check
        // This tests current behavior - reply_to uses exact string match
        assert!(is_routing_identifier_allowed(&task, "User@Example.COM"));
        assert!(!is_routing_identifier_allowed(&task, "user@example.com"));
    }

    #[test]
    fn blocked_routing_writes_security_message_email() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        // Write a routing file targeting an unauthorized address
        let routing_json = r#"{"channel": "email", "identifier": "attacker@evil.com"}"#;
        fs::write(workspace.join("reply_routing.json"), routing_json).expect("write");

        let mut task = make_test_task(vec!["legit@example.com".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Email;

        // Load routing and check it would be blocked
        let routing = load_reply_routing(workspace).expect("routing");
        assert!(!is_routing_identifier_allowed(&task, &routing.identifier));

        // Simulate what happens in schedule_reply_after_run_task when blocked
        let security_message = "To maintain user isolation and privacy, I cannot send messages to recipients outside your linked accounts.";
        let html = format!(
            "<!DOCTYPE html><html><body><p>{}</p></body></html>",
            security_message
        );
        fs::write(workspace.join("reply_email_draft.html"), &html).expect("write html");

        // Verify the security message was written
        let written = fs::read_to_string(workspace.join("reply_email_draft.html")).expect("read");
        assert!(written.contains("user isolation and privacy"));
    }

    #[test]
    fn blocked_routing_writes_security_message_chat() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        let mut task = make_test_task(vec!["U123456".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;

        // Simulate blocked routing for chat channel
        let security_message = "To maintain user isolation and privacy, I cannot send messages to recipients outside your linked accounts.";
        fs::write(workspace.join("reply_message.txt"), security_message).expect("write");

        let written = fs::read_to_string(workspace.join("reply_message.txt")).expect("read");
        assert!(written.contains("user isolation and privacy"));
    }

    #[test]
    fn workspace_secret_guard_blocks_html_reply_with_secret_value() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let reply_path = workspace.join("reply_email_draft.html");
        let attachments_dir = workspace.join("reply_email_attachments");

        fs::write(
            workspace.join(".env"),
            "NOTION_PASSWORD=super-secret-password-123\nGOOGLE_ACCOUNT_EMAIL=dowhiz@deep-tutor.com\n",
        )
        .expect("write env");
        fs::write(
            &reply_path,
            "<html><body>debug secret: super-secret-password-123</body></html>",
        )
        .expect("write reply");
        fs::create_dir_all(&attachments_dir).expect("create attachments");
        fs::write(
            attachments_dir.join("notes.txt"),
            "this attachment should be removed after secret detection",
        )
        .expect("write attachment");

        let blocked = apply_workspace_secret_leak_guard(
            workspace,
            &reply_path,
            &attachments_dir,
            &Channel::Email,
        );
        assert!(blocked);

        let reply = fs::read_to_string(&reply_path).expect("read sanitized reply");
        assert!(reply.contains(SECRET_GUARD_MESSAGE));
        assert!(!reply.contains("super-secret-password-123"));

        let attachment_files = fs::read_dir(&attachments_dir)
            .expect("attachments dir exists")
            .count();
        assert_eq!(attachment_files, 0);
    }

    #[test]
    fn workspace_secret_guard_blocks_chat_reply_with_secret_in_attachment() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let reply_path = workspace.join("reply_message.txt");
        let attachments_dir = workspace.join("reply_attachments");

        fs::write(workspace.join(".env"), "GOOGLE_PASSWORD=abc1234567890xyz\n").expect("write env");
        fs::write(&reply_path, "normal reply body").expect("write reply");
        fs::create_dir_all(&attachments_dir).expect("create attachments");
        fs::write(
            attachments_dir.join("dump.txt"),
            "temporary dump: abc1234567890xyz",
        )
        .expect("write attachment");

        let blocked = apply_workspace_secret_leak_guard(
            workspace,
            &reply_path,
            &attachments_dir,
            &Channel::Slack,
        );
        assert!(blocked);

        let reply = fs::read_to_string(&reply_path).expect("read sanitized reply");
        assert_eq!(reply, SECRET_GUARD_MESSAGE);
    }

    #[test]
    fn workspace_secret_guard_ignores_non_sensitive_env_keys() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let reply_path = workspace.join("reply_email_draft.html");
        let attachments_dir = workspace.join("reply_email_attachments");

        fs::write(
            workspace.join(".env"),
            "GOOGLE_ACCOUNT_EMAIL=dowhiz@deep-tutor.com\n",
        )
        .expect("write env");
        fs::write(
            &reply_path,
            "<html><body>contact: dowhiz@deep-tutor.com</body></html>",
        )
        .expect("write reply");

        let blocked = apply_workspace_secret_leak_guard(
            workspace,
            &reply_path,
            &attachments_dir,
            &Channel::Email,
        );
        assert!(!blocked);
    }

    // ==================== WeChat Cross-Channel Tests ====================

    #[test]
    fn cross_channel_ack_file_format_for_wechat_inbound() {
        // For WeChat inbound, ack should be plain text
        let channel = Channel::WeChat;
        let (ack_filename, attachments_dir) = match channel {
            Channel::Slack
            | Channel::Discord
            | Channel::BlueBubbles
            | Channel::Telegram
            | Channel::WhatsApp
            | Channel::Sms
            | Channel::WeChat => ("cross_channel_ack.txt", "reply_attachments"),
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("cross_channel_ack.html", "reply_email_attachments"),
        };
        assert_eq!(ack_filename, "cross_channel_ack.txt");
        assert_eq!(attachments_dir, "reply_attachments");
    }

    #[test]
    fn parse_channel_wechat_aliases() {
        // WeChat can be parsed with different aliases
        assert_eq!(parse_channel("wechat"), Some(Channel::WeChat));
        assert_eq!(parse_channel("WeChat"), Some(Channel::WeChat));
        assert_eq!(parse_channel("WECHAT"), Some(Channel::WeChat));
    }

    #[test]
    fn load_reply_routing_wechat_target() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        // Route to WeChat user
        let routing_json = r#"{"channel": "wechat", "identifier": "zhangsan"}"#;
        fs::write(workspace.join("reply_routing.json"), routing_json).expect("write");

        let routing = load_reply_routing(workspace).expect("should parse");
        assert_eq!(routing.channel, "wechat");
        assert_eq!(routing.identifier, "zhangsan");

        let parsed_channel = parse_channel(&routing.channel);
        assert_eq!(parsed_channel, Some(Channel::WeChat));
    }

    #[test]
    fn cross_channel_ack_content_for_wechat_target() {
        let target_channel = Channel::WeChat;
        let ack_message = format!(
            "The request has been successfully completed! I've sent my response to you on {}.",
            format_channel_name(&target_channel)
        );

        assert_eq!(
            ack_message,
            "The request has been successfully completed! I've sent my response to you on WeChat."
        );
    }

    #[test]
    fn reply_format_for_wechat_channel() {
        // WeChat uses plain text reply_message.txt like other chat channels
        let channel = Channel::WeChat;
        let (reply_filename, attachments_dirname) = match channel {
            Channel::Slack
            | Channel::Discord
            | Channel::BlueBubbles
            | Channel::Telegram
            | Channel::WhatsApp
            | Channel::Sms
            | Channel::WeChat => ("reply_message.txt", "reply_attachments"),
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("reply_email_draft.html", "reply_email_attachments"),
        };
        assert_eq!(reply_filename, "reply_message.txt");
        assert_eq!(attachments_dirname, "reply_attachments");
    }

    #[test]
    fn wechat_cross_channel_routing_to_email() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        // Simulate routing from WeChat to email
        let routing_json = r#"{"channel": "email", "identifier": "user@example.com"}"#;
        fs::write(workspace.join("reply_routing.json"), routing_json).expect("write");

        let routing = load_reply_routing(workspace).expect("should parse");
        let target_channel = parse_channel(&routing.channel);

        assert_eq!(target_channel, Some(Channel::Email));
        assert_eq!(routing.identifier, "user@example.com");

        // For email target, reply format should be HTML
        let (reply_filename, _) = match target_channel.unwrap() {
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("reply_email_draft.html", "reply_email_attachments"),
            _ => ("reply_message.txt", "reply_attachments"),
        };
        assert_eq!(reply_filename, "reply_email_draft.html");
    }

    #[test]
    fn email_cross_channel_routing_to_wechat() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();

        // Simulate routing from email to WeChat
        let routing_json = r#"{"channel": "wechat", "identifier": "lisi"}"#;
        fs::write(workspace.join("reply_routing.json"), routing_json).expect("write");

        let routing = load_reply_routing(workspace).expect("should parse");
        let target_channel = parse_channel(&routing.channel);

        assert_eq!(target_channel, Some(Channel::WeChat));
        assert_eq!(routing.identifier, "lisi");

        // For WeChat target, reply format should be plain text
        let (reply_filename, _) = match target_channel.unwrap() {
            Channel::Email
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides => ("reply_email_draft.html", "reply_email_attachments"),
            _ => ("reply_message.txt", "reply_attachments"),
        };
        assert_eq!(reply_filename, "reply_message.txt");
    }

    #[test]
    fn internal_email_whitelist_merges_employee_and_staging_configs() {
        let temp = TempDir::new().expect("tempdir");
        let employee = temp.path().join("employee.toml");
        let staging = temp.path().join("employee.staging.toml");
        fs::write(
            &employee,
            r#"
[[employees]]
id = "little_bear"
addresses = ["oliver@dowhiz.com"]
"#,
        )
        .expect("write employee.toml");
        fs::write(
            &staging,
            r#"
[[employees]]
id = "boiled_egg"
addresses = ["dowhiz@deep-tutor.com"]
"#,
        )
        .expect("write employee.staging.toml");

        let whitelist = collect_internal_email_whitelist_from_paths(&vec![employee, staging]);
        assert!(whitelist.contains("oliver@dowhiz.com"));
        assert!(whitelist.contains("dowhiz@deep-tutor.com"));
    }

    #[test]
    fn closure_loop_guard_skips_for_internal_slack_sender_only() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming = workspace.join("incoming_email");
        fs::create_dir_all(&incoming).expect("incoming dir");
        fs::write(
            incoming.join("00001_slack_message.txt"),
            "All set on my side too, no further action needed.",
        )
        .expect("write inbound");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(&reply_path, "Wrapped up on my side as well.").expect("write reply");

        std::env::set_var("INTERNAL_SLACK_SENDER_IDS", "u_internal");
        let mut task = make_test_task(vec!["U_INTERNAL".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;
        assert!(should_skip_closure_loop_reply(
            &task,
            &reply_path,
            Channel::Slack
        ));
        std::env::remove_var("INTERNAL_SLACK_SENDER_IDS");
    }

    #[test]
    fn closure_loop_guard_does_not_skip_for_external_slack_sender() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming = workspace.join("incoming_email");
        fs::create_dir_all(&incoming).expect("incoming dir");
        fs::write(
            incoming.join("00001_slack_message.txt"),
            "All set on my side too, no further action needed.",
        )
        .expect("write inbound");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(&reply_path, "Wrapped up on my side as well.").expect("write reply");

        std::env::set_var("INTERNAL_SLACK_SENDER_IDS", "u_internal");
        let mut task = make_test_task(vec!["U_EXTERNAL".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;
        assert!(!should_skip_closure_loop_reply(
            &task,
            &reply_path,
            Channel::Slack
        ));
        std::env::remove_var("INTERNAL_SLACK_SENDER_IDS");
    }

    #[test]
    fn closure_loop_guard_skips_for_no_reply_required_language() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming = workspace.join("incoming_email");
        fs::create_dir_all(&incoming).expect("incoming dir");
        fs::write(
            incoming.join("00001_slack_message.txt"),
            "No outgoing reply should be sent for this message. Status: thread closed, no action requested.",
        )
        .expect("write inbound");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(
            &reply_path,
            "No reply needed. Avoid sending another acknowledgement unless there is a new task.",
        )
        .expect("write reply");

        std::env::set_var("INTERNAL_SLACK_SENDER_IDS", "u_internal");
        let mut task = make_test_task(vec!["U_INTERNAL".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;
        assert!(should_skip_closure_loop_reply(
            &task,
            &reply_path,
            Channel::Slack
        ));
        std::env::remove_var("INTERNAL_SLACK_SENDER_IDS");
    }

    #[test]
    fn closure_loop_guard_skips_for_loop_diagnostic_language() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming = workspace.join("incoming_email");
        fs::create_dir_all(&incoming).expect("incoming dir");
        fs::write(
            incoming.join("00001_slack_message.txt"),
            "This thread is still another generated reply in the same closed loop, and I did not find a fresh task to carry out from this message.",
        )
        .expect("write inbound");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(
            &reply_path,
            "This is another generated reply in the same closed loop. I did not find a new task request.",
        )
        .expect("write reply");

        std::env::set_var("INTERNAL_SLACK_SENDER_IDS", "u_internal");
        let mut task = make_test_task(vec!["U_INTERNAL".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;
        assert!(should_skip_closure_loop_reply(
            &task,
            &reply_path,
            Channel::Slack
        ));
        std::env::remove_var("INTERNAL_SLACK_SENDER_IDS");
    }

    #[test]
    fn closure_loop_guard_skips_when_fresh_thread_guidance_is_present() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming = workspace.join("incoming_email");
        fs::create_dir_all(&incoming).expect("incoming dir");
        fs::write(
            incoming.join("00001_slack_message.txt"),
            "No reply needed. This thread is closed. If you mean to send a real task later, please start a fresh thread with only the new request.",
        )
        .expect("write inbound");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(
            &reply_path,
            "No outbound reply should be sent. This is an already-closed thread.",
        )
        .expect("write reply");

        std::env::set_var("INTERNAL_SLACK_SENDER_IDS", "u_internal");
        let mut task = make_test_task(vec!["U_INTERNAL".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;
        assert!(should_skip_closure_loop_reply(
            &task,
            &reply_path,
            Channel::Slack
        ));
        std::env::remove_var("INTERNAL_SLACK_SENDER_IDS");
    }

    #[test]
    fn internal_sender_always_skips_even_without_closure_text() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path();
        let incoming = workspace.join("incoming_email");
        fs::create_dir_all(&incoming).expect("incoming dir");
        fs::write(
            incoming.join("00001_slack_message.txt"),
            "Can you run a fresh check and send details back?",
        )
        .expect("write inbound");
        let reply_path = workspace.join("reply_message.txt");
        fs::write(&reply_path, "I will run the check and send details.").expect("write reply");

        std::env::set_var("INTERNAL_SLACK_SENDER_IDS", "u_internal");
        let mut task = make_test_task(vec!["U_INTERNAL".to_string()]);
        task.workspace_dir = workspace.to_path_buf();
        task.channel = Channel::Slack;
        assert!(should_skip_closure_loop_reply(
            &task,
            &reply_path,
            Channel::Slack
        ));
        std::env::remove_var("INTERNAL_SLACK_SENDER_IDS");
    }
}
