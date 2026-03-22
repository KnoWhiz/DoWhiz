//! Inbound handler for Notion email notifications.
//!
//! This module processes email notifications from Notion and creates tasks
//! that use the Notion API (via OAuth tokens) to interact with pages.
//!
//! ## Flow
//!
//! 1. Email from `notify@mail.notion.so` is detected
//! 2. NotionEmailNotification is parsed from email content
//! 3. OAuth token is fetched from NotionStore (by workspace_id)
//! 4. A workspace is created with Notion context and API token
//! 5. A RunTask is scheduled (agent uses notion_api_cli to access Notion)
//! 6. Agent reads the page, processes the request, and replies via API

use std::path::Path;
use std::time::Duration;

use chrono::Utc;
use tracing::{info, warn};

use crate::account_store::AccountStore;
use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::notion_email_detector::NotionEmailNotification;
use crate::notion_store::NotionStore;
use crate::user_store::{extract_emails, UserStore};
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::postmark::PostmarkInbound;
use super::super::workspace::ensure_thread_workspace;
use crate::service::BoxError;

/// Process an inbound email that has been identified as a Notion notification.
///
/// This creates a task workspace with Notion context and schedules a RunTask
/// for the agent to handle the notification.
pub(crate) fn process_notion_email(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    _account_store: &AccountStore,
    email_payload: &PostmarkInbound,
    _raw_payload: &[u8],
    notification: &NotionEmailNotification,
) -> Result<(), BoxError> {
    info!(
        "processing Notion email notification type={:?} actor={:?} page={:?}",
        notification.notification_type,
        notification.actor_name,
        notification.page_title,
    );

    // Skip self-notifications to prevent feedback loops
    // Check if the actor_name matches the employee's display name or any known alias
    if let Some(actor) = &notification.actor_name {
        let actor_lower = actor.to_lowercase();
        let employee_id = &config.employee_profile.id;
        let display_name = config.employee_profile.display_name.as_deref();

        // Check against display_name (e.g., "Oliver")
        if let Some(name) = display_name {
            if actor_lower == name.to_lowercase() {
                info!(
                    "skipping self-notification from employee '{}' (display_name match)",
                    actor
                );
                return Ok(());
            }
        }

        // Check against common patterns: "DoWhiz at <name>", "Oliver the little bear", etc.
        let employee_id_lower = employee_id.to_lowercase().replace('_', " ");
        if actor_lower.contains(&employee_id_lower) {
            info!(
                "skipping self-notification from employee '{}' (employee_id match: {})",
                actor, employee_id
            );
            return Ok(());
        }

        // Check against "DoWhiz at <display_name>" pattern
        if let Some(name) = display_name {
            if actor_lower.contains(&format!("dowhiz at {}", name.to_lowercase()))
                || actor_lower.contains(&format!("dowhiz@{}", name.to_lowercase()))
            {
                info!(
                    "skipping self-notification from employee '{}' (DoWhiz at pattern)",
                    actor
                );
                return Ok(());
            }
        }

        // Check against Notion integration names (e.g., "dowhiz_staging", "dowhiz_production")
        // These are the display names shown in Notion when the integration posts a comment
        if actor_lower.starts_with("dowhiz_") || actor_lower == "dowhiz" {
            info!(
                "skipping self-notification from Notion integration '{}' (integration name match)",
                actor
            );
            return Ok(());
        }
    }

    // Determine the requester identity
    // For Notion emails, we use the actor who triggered the notification
    // or fall back to the original email sender
    let requester_name = notification
        .actor_name
        .as_deref()
        .unwrap_or("Unknown Notion User");

    // Try to extract an email from the original sender (for account linking)
    let sender = email_payload.from.as_deref().unwrap_or("").trim();
    let user_email = extract_emails(sender)
        .into_iter()
        .next()
        .unwrap_or_else(|| format!("notion_{}@local", sanitize_identifier(requester_name)));

    // Create or get user based on requester name (Notion-specific identifier)
    let notion_identifier = format!(
        "notion:{}",
        notification
            .actor_name
            .as_deref()
            .map(sanitize_identifier)
            .unwrap_or_else(|| "unknown".to_string())
    );

    let user = user_store.get_or_create_user("notion_actor", &notion_identifier)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Create thread key based on page ID or notification content
    let thread_key = create_notion_thread_key(notification, email_payload);

    // Ensure workspace exists
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    // Bump thread state
    let thread_state_path = default_thread_state_path(&workspace);
    let message_id = email_payload
        .header_message_id()
        .or(email_payload.message_id.as_deref())
        .map(|v| v.trim().to_string());
    let thread_state = bump_thread_state(&thread_state_path, &thread_key, message_id.clone())?;

    // Try to get Notion OAuth token and account_id
    // Priority: 1) env var, 2) workspace_id (from tracking URL), 3) workspace_name fuzzy, 4) any credential
    let (access_token, credential_account_id): (Option<String>, Option<uuid::Uuid>) =
        if let Ok(token) = std::env::var("NOTION_API_TOKEN") {
            (Some(token), None)
        } else {
            match NotionStore::new() {
                Ok(store) => {
                    // Try #1: workspace_id from decoded tracking URL (most reliable)
                    if let Some(ref ws_id) = notification.workspace_id {
                        info!("Looking for Notion token by workspace_id: {}", ws_id);
                        match store.get_credential_by_workspace(ws_id) {
                            Ok(credential) => {
                                info!(
                                    "Found Notion token by workspace_id '{}' (workspace_name: {:?}, account_id: {})",
                                    ws_id,
                                    credential.workspace_name,
                                    credential.account_id
                                );
                                (Some(credential.access_token), Some(credential.account_id))
                            }
                            Err(e) => {
                                warn!("No Notion token found for workspace_id '{}': {}", ws_id, e);
                                // Try next method
                                if let Some(ref ws_name) = notification.workspace_name {
                                    info!("Looking for Notion token by workspace_name: {}", ws_name);
                                    match store.get_credential_by_workspace_name_fuzzy(ws_name) {
                                        Ok(credential) => {
                                            info!(
                                                "Found Notion token for workspace_name '{}' (matched: {:?}, account_id: {})",
                                                ws_name,
                                                credential.workspace_name,
                                                credential.account_id
                                            );
                                            (Some(credential.access_token), Some(credential.account_id))
                                        }
                                        Err(e2) => {
                                            warn!("No Notion token found for workspace_name '{}': {}", ws_name, e2);
                                            // Try fallback
                                            match store.get_any_credential() {
                                                Ok(credential) => {
                                                    info!(
                                                        "Found fallback Notion token (workspace_id: {}, workspace_name: {:?}, account_id: {})",
                                                        credential.workspace_id,
                                                        credential.workspace_name,
                                                        credential.account_id
                                                    );
                                                    (Some(credential.access_token), Some(credential.account_id))
                                                }
                                                Err(e3) => {
                                                    warn!("No fallback Notion credential available: {}", e3);
                                                    (None, None)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    // No workspace_name, try fallback directly
                                    match store.get_any_credential() {
                                        Ok(credential) => {
                                            info!(
                                                "Found fallback Notion token (workspace_id: {}, workspace_name: {:?}, account_id: {})",
                                                credential.workspace_id,
                                                credential.workspace_name,
                                                credential.account_id
                                            );
                                            (Some(credential.access_token), Some(credential.account_id))
                                        }
                                        Err(e2) => {
                                            warn!("No fallback Notion credential available: {}", e2);
                                            (None, None)
                                        }
                                    }
                                }
                            }
                        }
                    } else if let Some(ref ws_name) = notification.workspace_name {
                        // Try #2: workspace_name fuzzy match (from direct URLs)
                        info!("Looking for Notion token by workspace_name: {}", ws_name);
                        match store.get_credential_by_workspace_name_fuzzy(ws_name) {
                            Ok(credential) => {
                                info!(
                                    "Found Notion token for workspace_name '{}' (matched: {:?}, account_id: {})",
                                    ws_name,
                                    credential.workspace_name,
                                    credential.account_id
                                );
                                (Some(credential.access_token), Some(credential.account_id))
                            }
                            Err(e) => {
                                warn!("No Notion token found for workspace_name '{}': {}", ws_name, e);
                                // Try fallback
                                match store.get_any_credential() {
                                    Ok(credential) => {
                                        info!(
                                            "Found fallback Notion token (workspace_id: {}, workspace_name: {:?}, account_id: {})",
                                            credential.workspace_id,
                                            credential.workspace_name,
                                            credential.account_id
                                        );
                                        (Some(credential.access_token), Some(credential.account_id))
                                    }
                                    Err(e2) => {
                                        warn!("No fallback Notion credential available: {}", e2);
                                        (None, None)
                                    }
                                }
                            }
                        }
                    } else {
                        // Fallback #3: try to get any available credential
                        info!("Trying fallback: looking for any available Notion credential");
                        match store.get_any_credential() {
                            Ok(credential) => {
                                info!(
                                    "Found fallback Notion token (workspace_id: {}, workspace_name: {:?}, account_id: {})",
                                    credential.workspace_id,
                                    credential.workspace_name,
                                    credential.account_id
                                );
                                (Some(credential.access_token), Some(credential.account_id))
                            }
                            Err(e) => {
                                warn!("No fallback Notion credential available: {}", e);
                                (None, None)
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to connect to NotionStore: {}", e);
                    (None, None)
                }
            }
        };

    // Write Notion context to workspace
    write_notion_email_context(
        &workspace,
        notification,
        email_payload,
        thread_state.last_email_seq,
        access_token.as_deref(),
    )?;

    // Determine model
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

    // Create RunTask with Notion channel
    // Use account_id from NotionCredential if available
    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: std::path::PathBuf::from("incoming_email"),
        input_attachments_dir: std::path::PathBuf::from("incoming_attachments"),
        memory_dir: std::path::PathBuf::from("memory"),
        reference_dir: std::path::PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![user_email.clone()],
        reply_from: config.employee_profile.addresses.first().cloned(),
        archive_root: None,
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Notion,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
        requester_identifier_type: Some("notion_actor".to_string()),
        requester_identifier: Some(notion_identifier.clone()),
        account_id: credential_account_id,
    };

    let run_task_for_account = run_task.clone();

    // Schedule the task
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduled Notion task user_id={} task_id={} workspace={} thread_epoch={} channel=Notion account_id={:?}",
        user.user_id,
        task_id,
        workspace.display(),
        thread_state.epoch,
        credential_account_id
    );

    // Enqueue to account-level storage if we have an account_id from the Notion credential
    if let Some(account_id) = credential_account_id {
        info!(
            "found linked account {} from Notion credential",
            account_id
        );
        let account_tasks_dir = config.users_root.join(account_id.to_string()).join("state");
        if let Err(err) = std::fs::create_dir_all(&account_tasks_dir) {
            warn!(
                "failed to create account tasks dir for account {}: {}",
                account_id, err
            );
        } else {
            let account_tasks_db_path = account_tasks_dir.join("tasks.db");
            match Scheduler::load(&account_tasks_db_path, ModuleExecutor::default()) {
                Ok(mut account_scheduler) => {
                    match account_scheduler.add_one_shot_in_with_id(
                        task_id,
                        Duration::from_secs(0),
                        TaskKind::RunTask(run_task_for_account),
                    ) {
                        Ok(()) => {
                            info!(
                                "also enqueued task to account-level storage account={} task_id={} channel=Notion",
                                account_id, task_id
                            );
                        }
                        Err(err) => {
                            warn!(
                                "failed to add task to account scheduler for account {}: {}",
                                account_id, err
                            );
                        }
                    }
                }
                Err(err) => {
                    warn!(
                        "failed to load account scheduler for account {}: {}",
                        account_id, err
                    );
                }
            }
        }
    } else {
        info!(
            "no account_id from Notion credential, skipping account-level task"
        );
    }

    Ok(())
}

/// Create a unique thread key for a Notion notification.
fn create_notion_thread_key(
    notification: &NotionEmailNotification,
    email_payload: &PostmarkInbound,
) -> String {
    // Prefer page_id if available for thread grouping
    if let Some(page_id) = &notification.page_id {
        return format!("notion:page:{}", page_id);
    }

    // Fall back to email message ID
    if let Some(msg_id) = email_payload
        .header_message_id()
        .or(email_payload.message_id.as_deref())
    {
        return format!("notion:email:{}", sanitize_identifier(msg_id));
    }

    // Last resort: hash of subject and timestamp
    let hash_input = format!("{}:{}", notification.subject, Utc::now().timestamp());
    format!("notion:hash:{:x}", md5::compute(hash_input.as_bytes()))
}

/// Write Notion context files to the workspace for the agent.
fn write_notion_email_context(
    workspace: &Path,
    notification: &NotionEmailNotification,
    email_payload: &PostmarkInbound,
    seq: u64,
    access_token: Option<&str>,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    // If we have an OAuth token, write it to .notion_env for the agent
    let has_api_access = if let Some(token) = access_token {
        let env_path = workspace.join(".notion_env");
        std::fs::write(&env_path, format!("NOTION_API_TOKEN={}\n", token))?;
        info!("Wrote Notion API token to workspace");
        true
    } else {
        warn!("No Notion OAuth token available for this workspace");
        false
    };

    // Write the notification context as JSON
    let context_path = workspace.join(".notion_email_context.json");
    let instructions = if has_api_access {
        "This task was triggered by a Notion @mention. Use notion_api_cli to read the page and post replies. The NOTION_API_TOKEN is available in .notion_env - source it before using the CLI. IMPORTANT: After posting via notion_api_cli, run 'touch .notion_api_replied' to prevent duplicate sends."
    } else {
        "This task was triggered by a Notion @mention. No API token is available - the workspace owner needs to authorize the bot at dowhiz.com/settings. For now, inform the user that you cannot access the page."
    };
    let context = serde_json::json!({
        "channel": "notion",
        "trigger": "email_notification",
        "notification_type": notification.notification_type,
        "actor_name": notification.actor_name,
        "page_url": notification.page_url,
        "page_id": notification.page_id,
        "page_title": notification.page_title,
        "workspace_name": notification.workspace_name,
        "comment_preview": notification.comment_preview,
        "comment_url": notification.comment_url,
        "email_subject": notification.subject,
        "has_api_access": has_api_access,
        "instructions": instructions
    });
    std::fs::write(&context_path, serde_json::to_string_pretty(&context)?)?;

    // Create HTML representation for the agent (same format as other channels)
    let actor_name = notification.actor_name.as_deref().unwrap_or("Someone");
    let page_title = notification
        .page_title
        .as_deref()
        .unwrap_or("a Notion page");
    let page_url = notification
        .page_url
        .as_deref()
        .unwrap_or("https://notion.so");
    let comment_preview = notification
        .comment_preview
        .as_deref()
        .unwrap_or("[No preview available - please open the page to read the full comment]");
    let page_id = notification
        .page_id
        .as_deref()
        .unwrap_or("[Not available - extract from URL]");

    let api_section = if has_api_access {
        r#"<h3>How to respond (using Notion API):</h3>
<ol>
<li>Source the token: <code>source .notion_env</code></li>
<li>Read the page: <code>notion_api_cli read-blocks PAGE_ID</code></li>
<li>Get comments: <code>notion_api_cli get-comments PAGE_ID</code></li>
<li>Complete the requested task</li>
<li>Post reply: <code>notion_api_cli create-comment PAGE_ID "Your reply"</code></li>
</ol>"#
    } else {
        r#"<h3>No API Access</h3>
<p>The workspace owner needs to authorize the bot at dowhiz.com/settings.</p>
<p>Please inform the user that you cannot access this page until authorization is complete.</p>"#
    };

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>Notion Notification</title></head>
<body>
<h2>Notion: {notification_type} from {actor_name}</h2>
<p><strong>Page:</strong> {page_title}</p>
<p><strong>URL:</strong> <a href="{page_url}">{page_url}</a></p>
{workspace_section}

<h3>Notification:</h3>
<p>{comment_preview}</p>

<hr>
{api_section}

<p><strong>Page ID:</strong> <code>{page_id}</code></p>
</body>
</html>"#,
        notification_type = format!("{:?}", notification.notification_type),
        actor_name = actor_name,
        page_title = page_title,
        page_url = page_url,
        page_id = page_id,
        comment_preview = comment_preview,
        workspace_section = notification
            .workspace_name
            .as_ref()
            .map(|ws| format!("<p><strong>Workspace:</strong> {}</p>", ws))
            .unwrap_or_default(),
    );

    let html_path = incoming_dir.join(format!("{:05}_email.html", seq));
    std::fs::write(&html_path, &html_content)?;

    // Also save the raw email payload for reference
    let raw_path = incoming_dir.join(format!("{:05}_notion_email_raw.json", seq));
    std::fs::write(
        &raw_path,
        serde_json::to_string_pretty(&serde_json::json!({
            "from": email_payload.from,
            "to": email_payload.to,
            "subject": email_payload.subject,
            "text_body": email_payload.text_body,
            "html_body": email_payload.html_body,
        }))?,
    )?;

    // Write metadata
    let meta_path = incoming_dir.join(format!("{:05}_notion_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "notion",
        "trigger_type": "email_notification",
        "actor_name": notification.actor_name,
        "page_id": notification.page_id,
        "page_title": notification.page_title,
        "page_url": notification.page_url,
        "comment_url": notification.comment_url,
        "notification_type": notification.notification_type,
        "timestamp": Utc::now().to_rfc3339(),
    });
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "wrote Notion email context to workspace: seq={} page_id={:?}",
        seq, notification.page_id
    );

    Ok(())
}

/// Sanitize a string to be used as an identifier.
fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() {
                Some('_')
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("Alice Smith"), "alice_smith");
        assert_eq!(sanitize_identifier("user@example.com"), "userexamplecom");
        assert_eq!(sanitize_identifier("Test-User_123"), "test-user_123");
    }

    #[test]
    fn test_create_thread_key_with_page_id() {
        let notification = NotionEmailNotification {
            notification_type: crate::notion_email_detector::NotionNotificationType::CommentMention,
            actor_name: Some("Alice".to_string()),
            page_url: Some("https://notion.so/workspace/Page-abc123".to_string()),
            page_id: Some("abc123".to_string()),
            workspace_id: None,
            workspace_name: None,
            page_title: Some("Test Page".to_string()),
            comment_preview: None,
            comment_url: None,
            subject: "Alice mentioned you".to_string(),
        };

        let payload = PostmarkInbound {
            from: Some("notify@mail.notion.so".to_string()),
            to: None,
            cc: None,
            bcc: None,
            to_full: None,
            cc_full: None,
            bcc_full: None,
            reply_to: None,
            subject: None,
            text_body: None,
            stripped_text_reply: None,
            html_body: None,
            message_id: None,
            attachments: None,
            headers: None,
        };

        let thread_key = create_notion_thread_key(&notification, &payload);
        assert_eq!(thread_key, "notion:page:abc123");
    }
}
