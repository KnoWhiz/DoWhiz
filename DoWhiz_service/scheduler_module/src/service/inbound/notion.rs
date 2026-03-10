//! Inbound handler for Notion comments/mentions.
//!
//! Processes @mentions from Notion browser automation and creates tasks.

use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::account_store::AccountStore;
use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::notion_browser::models::NotionMention;
use crate::user_store::{extract_emails, UserStore};
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::workspace::ensure_thread_workspace;
use crate::service::BoxError;

/// Process an incoming Notion mention/comment.
pub(crate) fn process_notion_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    account_store: &AccountStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    // Parse the Notion mention from raw payload
    let mention: NotionMention = serde_json::from_slice(raw_payload)
        .map_err(|e| format!("Failed to parse NotionMention: {}", e))?;

    // Extract page/workspace info from metadata
    let workspace_id = message
        .metadata
        .notion_workspace_id
        .as_deref()
        .unwrap_or("unknown");
    let page_id = message
        .metadata
        .notion_page_id
        .as_deref()
        .ok_or("missing notion_page_id")?;
    let page_title = message
        .metadata
        .notion_page_title
        .as_deref()
        .unwrap_or("Untitled");

    // Extract sender email, with fallbacks
    let extracted_email = extract_emails(&message.sender).into_iter().next();
    let user_email = match extracted_email {
        Some(email) if email != "unknown@unknown.com" => email,
        _ => {
            // Use sender name or ID as fallback
            format!("notion_{}@local", message.sender.replace(' ', "_"))
        }
    };

    // Create or get user
    let user = user_store.get_or_create_user("notion", &user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Create thread key from workspace:page:notification
    let thread_key = format!(
        "notion:{}:{}:{}",
        workspace_id, page_id, mention.id
    );

    // Ensure workspace directory exists
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    // Bump thread state for sequencing
    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state = bump_thread_state(
        &thread_state_path,
        &thread_key,
        message.message_id.clone(),
    )?;

    // Save incoming comment to workspace
    append_workspace_notion_comment(
        &workspace,
        message,
        &mention,
        thread_state.last_email_seq,
        page_id,
        page_title,
    )?;

    // Write Notion context to workspace for agent
    write_notion_context_to_workspace(&workspace, &mention, page_id, page_title)?;

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

    // Create RunTask
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
        requester_identifier_type: Some("notion_user".to_string()),
        requester_identifier: Some(user_email.clone()),
    };

    let run_task_for_account = run_task.clone();

    // Schedule the task
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={} channel=Notion",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    // Check for linked account
    match account_store.get_account_by_identifier("email", &user_email) {
        Ok(Some(account)) => {
            info!("Found account {} for Notion user {}", account.id, user_email);
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
                        match account_scheduler.add_one_shot_in_with_id(
                            task_id,
                            Duration::from_secs(0),
                            TaskKind::RunTask(run_task_for_account),
                        ) {
                            Ok(()) => {
                                info!(
                                    "also enqueued task to account-level storage account={} task_id={} channel=Notion",
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
        Ok(None) => {
            info!(
                "No account linked for Notion user '{}', skipping account-level task",
                user_email
            );
        }
        Err(err) => {
            warn!(
                "Failed to look up account for Notion user '{}': {}",
                user_email, err
            );
        }
    }

    Ok(())
}

/// Write Notion context file for agent to understand how to reply.
fn write_notion_context_to_workspace(
    workspace: &Path,
    mention: &NotionMention,
    page_id: &str,
    page_title: &str,
) -> Result<(), BoxError> {
    let context = serde_json::json!({
        "channel": "notion",
        "workspace_id": mention.workspace_id,
        "workspace_name": mention.workspace_name,
        "page_id": page_id,
        "page_title": page_title,
        "comment_id": mention.comment_id,
        "block_id": mention.block_id,
        "notification_id": mention.id,
        "url": mention.url,
        "reply_instructions": "Write your reply to reply_message.txt. The system will post it as a comment reply via browser automation."
    });

    let context_path = workspace.join(".notion_context.json");
    std::fs::write(&context_path, serde_json::to_string_pretty(&context)?)?;

    info!("wrote .notion_context.json to workspace");
    Ok(())
}

/// Save an incoming Notion comment to the workspace.
fn append_workspace_notion_comment(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    mention: &NotionMention,
    seq: u64,
    page_id: &str,
    page_title: &str,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    // Save the raw mention JSON
    let raw_path = incoming_dir.join(format!("{:05}_notion_mention.json", seq));
    let raw_json = serde_json::to_string_pretty(&mention)?;
    std::fs::write(&raw_path, &raw_json)?;

    // Create HTML representation for the agent
    let sender_name = message.sender_name.as_deref().unwrap_or(&message.sender);

    // Build conversation thread HTML if available
    let thread_html = if !mention.thread_context.is_empty() {
        let mut html = String::from("<h3>Previous conversation:</h3>\n");
        for comment in &mention.thread_context {
            html.push_str(&format!(
                "<div style=\"margin-bottom: 10px;\">\n<p><strong>{}:</strong></p>\n<p>{}</p>\n</div>\n",
                comment.author_name, comment.text
            ));
        }
        html
    } else {
        String::new()
    };

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>Notion Comment</title></head>
<body>
<h2>@mention on: {page_title}</h2>
<p><strong>Workspace:</strong> {workspace_name}</p>
<p><strong>Page ID:</strong> {page_id}</p>
<p><strong>From:</strong> {sender_name} ({sender})</p>
<p><strong>Notification ID:</strong> {notification_id}</p>
<p><strong>URL:</strong> <a href="{url}">{url}</a></p>

<h3>Message:</h3>
<p>{comment_text}</p>

{thread_html}

<hr>
<h3>How to reply:</h3>
<p>Write your reply to <code>reply_message.txt</code>. The system will post it as a comment via browser automation.</p>
<p>You can reference the page content from the context in this message.</p>
<hr>
<p><em>Respond by writing to reply_message.txt</em></p>
</body>
</html>"#,
        page_title = page_title,
        workspace_name = mention.workspace_name,
        page_id = page_id,
        sender_name = sender_name,
        sender = message.sender,
        notification_id = mention.id,
        url = mention.url,
        comment_text = mention.comment_text,
        thread_html = thread_html
    );

    let html_path = incoming_dir.join(format!("{:05}_email.html", seq));
    std::fs::write(&html_path, &html_content)?;

    // Create metadata file
    let meta_path = incoming_dir.join(format!("{:05}_notion_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "notion",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "workspace_id": mention.workspace_id,
        "workspace_name": mention.workspace_name,
        "page_id": page_id,
        "page_title": page_title,
        "notification_id": mention.id,
        "comment_id": mention.comment_id,
        "block_id": mention.block_id,
        "url": mention.url,
        "thread_id": message.thread_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved Notion mention seq={} notification_id={} to {}",
        seq,
        mention.id,
        incoming_dir.display()
    );

    Ok(())
}
