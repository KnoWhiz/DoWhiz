//! Unified handler for Google Workspace (Docs, Sheets, Slides) comments.

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::account_store::lookup_account_by_channel;
use crate::adapters::google_common::ActionableComment;
use crate::channel::Channel;
use crate::google_auth::{GoogleAuth, GoogleAuthConfig};
use crate::index_store::IndexStore;
use crate::user_store::{extract_emails, UserStore};
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::workspace::{ensure_thread_workspace, persist_inbound_payloads};
use super::super::BoxError;

/// Process an incoming Google Workspace comment (Docs, Sheets, or Slides).
pub(crate) fn process_google_workspace_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let actionable: ActionableComment = serde_json::from_slice(raw_payload)?;
    let channel = message.channel.clone();

    // Get file ID and name based on channel
    let (file_id, file_name, channel_prefix) = match channel {
        Channel::GoogleDocs => {
            let id = message
                .metadata
                .google_docs_document_id
                .as_deref()
                .ok_or("missing google_docs_document_id")?;
            let name = message
                .metadata
                .google_docs_document_name
                .as_deref()
                .unwrap_or("Document");
            (id, name, "gdocs")
        }
        Channel::GoogleSheets => {
            let id = message
                .metadata
                .google_sheets_spreadsheet_id
                .as_deref()
                .ok_or("missing google_sheets_spreadsheet_id")?;
            let name = message
                .metadata
                .google_sheets_spreadsheet_name
                .as_deref()
                .unwrap_or("Spreadsheet");
            (id, name, "gsheets")
        }
        Channel::GoogleSlides => {
            let id = message
                .metadata
                .google_slides_presentation_id
                .as_deref()
                .ok_or("missing google_slides_presentation_id")?;
            let name = message
                .metadata
                .google_slides_presentation_name
                .as_deref()
                .unwrap_or("Presentation");
            (id, name, "gslides")
        }
        _ => return Err("Invalid channel for Google Workspace handler".into()),
    };

    let user_email = extract_emails(&message.sender)
        .into_iter()
        .next()
        .unwrap_or_else(|| format!("{}_{}@local", channel_prefix, message.sender.replace(' ', "_")));
    let user = user_store.get_or_create_user(channel_prefix, &user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!("{}:{}:{}", channel_prefix, file_id, actionable.comment.id);

    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state = bump_thread_state(
        &thread_state_path,
        &thread_key,
        Some(actionable.tracking_id.clone()),
    )?;

    append_workspace_comment(
        &workspace,
        message,
        &actionable,
        thread_state.last_email_seq,
        &channel,
        file_id,
        file_name,
    )?;

    // Use employee-specific OAuth credentials
    let auth_config = GoogleAuthConfig::from_env_for_employee(Some(&config.employee_profile.id));
    if let Ok(auth) = GoogleAuth::new(auth_config) {
        // Write access token to workspace for agent to use
        match auth.get_access_token() {
            Ok(token) => {
                let token_path = workspace.join(".google_access_token");
                if let Err(e) = std::fs::write(&token_path, &token) {
                    warn!("failed to write .google_access_token: {}", e);
                } else {
                    info!(
                        "wrote .google_access_token to workspace for {} comment",
                        channel_display_name(&channel)
                    );
                }
            }
            Err(e) => {
                warn!("failed to get Google access token for workspace: {}", e);
            }
        }

        // Fetch file content based on channel
        let content_result = match channel {
            Channel::GoogleDocs => {
                use crate::adapters::google_docs::GoogleDocsInboundAdapter;
                let adapter = GoogleDocsInboundAdapter::new(auth, HashSet::new());
                adapter.read_document_content(file_id)
            }
            Channel::GoogleSheets => {
                use crate::adapters::google_sheets::GoogleSheetsInboundAdapter;
                let adapter = GoogleSheetsInboundAdapter::new(auth, HashSet::new());
                adapter.read_spreadsheet_content(file_id)
            }
            Channel::GoogleSlides => {
                use crate::adapters::google_slides::GoogleSlidesInboundAdapter;
                let adapter = GoogleSlidesInboundAdapter::new(auth, HashSet::new());
                adapter.read_presentation_content(file_id)
            }
            _ => Err(crate::channel::AdapterError::ConfigError(
                "Invalid channel".to_string(),
            )),
        };

        match content_result {
            Ok(content) => {
                let content_filename = match channel {
                    Channel::GoogleDocs => "document_content.txt",
                    Channel::GoogleSheets => "spreadsheet_content.csv",
                    Channel::GoogleSlides => "presentation_content.txt",
                    _ => "content.txt",
                };
                let content_path = workspace.join("incoming_email").join(content_filename);
                if let Err(e) = std::fs::write(&content_path, &content) {
                    warn!("Failed to save content for {}: {}", file_id, e);
                }
            }
            Err(e) => {
                warn!("Failed to fetch content for {}: {}", file_id, e);
            }
        }
    }

    let account_id = lookup_account_by_channel(&channel, &user_email);
    if let Err(err) = persist_inbound_payloads(
        &workspace,
        &channel,
        account_id,
        &user.user_id,
        Some(&thread_key),
    ) {
        warn!("failed to persist inbound payloads to blob: {}", err);
    }

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

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: std::path::PathBuf::from("incoming_email"),
        input_attachments_dir: std::path::PathBuf::from("incoming_attachments"),
        memory_dir: std::path::PathBuf::from("memory"),
        reference_dir: std::path::PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![message.sender.clone()],
        reply_from: config.employee_profile.addresses.first().cloned(),
        archive_root: None,
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: channel.clone(),
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={} channel={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch,
        channel_display_name(&channel)
    );

    Ok(())
}

fn channel_display_name(channel: &Channel) -> &'static str {
    match channel {
        Channel::GoogleDocs => "Google Docs",
        Channel::GoogleSheets => "Google Sheets",
        Channel::GoogleSlides => "Google Slides",
        _ => "Google Workspace",
    }
}

fn channel_file_type(channel: &Channel) -> &'static str {
    match channel {
        Channel::GoogleDocs => "Document",
        Channel::GoogleSheets => "Spreadsheet",
        Channel::GoogleSlides => "Presentation",
        _ => "File",
    }
}

fn channel_content_file(channel: &Channel) -> &'static str {
    match channel {
        Channel::GoogleDocs => "document_content.txt",
        Channel::GoogleSheets => "spreadsheet_content.csv",
        Channel::GoogleSlides => "presentation_content.txt",
        _ => "content.txt",
    }
}

/// Save an incoming Google Workspace comment or reply to the workspace.
fn append_workspace_comment(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    actionable: &ActionableComment,
    seq: u64,
    channel: &Channel,
    file_id: &str,
    file_name: &str,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    let display_name = channel_display_name(channel);
    let file_type = channel_file_type(channel);
    let content_file = channel_content_file(channel);

    // Save the raw comment JSON (includes all replies)
    let prefix = match channel {
        Channel::GoogleDocs => "gdocs",
        Channel::GoogleSheets => "gsheets",
        Channel::GoogleSlides => "gslides",
        _ => "gworkspace",
    };
    let raw_path = incoming_dir.join(format!("{:05}_{}_comment.json", seq, prefix));
    let raw_json = serde_json::to_string_pretty(&actionable.comment)?;
    std::fs::write(&raw_path, &raw_json)?;

    // Create HTML representation for the agent
    let sender_name = message.sender_name.as_deref().unwrap_or(&message.sender);
    let quoted_text = actionable
        .comment
        .quoted_file_content
        .as_ref()
        .and_then(|q| q.value.as_deref())
        .unwrap_or("");

    let item_type = if actionable.triggering_reply.is_some() {
        "Reply"
    } else {
        "Comment"
    };

    // Build conversation thread HTML if this is a reply
    let thread_html = if let Some(ref reply) = actionable.triggering_reply {
        let original_author = actionable
            .comment
            .author
            .as_ref()
            .and_then(|a| a.display_name.as_deref())
            .unwrap_or("Someone");

        format!(
            r#"<h3>Conversation Thread:</h3>
<div style="margin-bottom: 10px;">
    <p><strong>{} (original comment):</strong></p>
    <p>{}</p>
</div>
<div style="margin-left: 20px; border-left: 2px solid #ccc; padding-left: 10px;">
    <p><strong>{} (reply that mentions you):</strong></p>
    <p>{}</p>
</div>"#,
            original_author, actionable.comment.content, sender_name, reply.content
        )
    } else {
        format!(
            r#"<h3>Comment:</h3>
<p>{}</p>"#,
            actionable.comment.content
        )
    };

    // Generate channel-specific CLI hint
    let cli_hint = match channel {
        Channel::GoogleDocs => format!(
            r#"<h3>How to edit:</h3>
<p>Use the <code>google-docs</code> CLI to make edits and reply to this comment.</p>
<pre>
google-docs read-document {file_id}
google-docs reply-comment {file_id} {comment_id} "Your reply"
</pre>"#,
            file_id = file_id,
            comment_id = actionable.comment.id
        ),
        Channel::GoogleSheets => format!(
            r#"<h3>How to edit:</h3>
<p>Use the <code>google-sheets</code> CLI to make edits and reply to this comment.</p>
<pre>
google-sheets read-spreadsheet {file_id}
google-sheets update-values {file_id} "Sheet1!A1:B2" '[["value1","value2"]]'
google-sheets reply-comment {file_id} {comment_id} "Your reply"
</pre>"#,
            file_id = file_id,
            comment_id = actionable.comment.id
        ),
        Channel::GoogleSlides => format!(
            r#"<h3>How to edit:</h3>
<p>Use the <code>google-slides</code> CLI to make edits and reply to this comment.</p>
<pre>
google-slides read-presentation {file_id}
google-slides create-slide {file_id} --layout=TITLE_AND_BODY
google-slides reply-comment {file_id} {comment_id} "Your reply"
</pre>"#,
            file_id = file_id,
            comment_id = actionable.comment.id
        ),
        _ => String::new(),
    };

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>{display_name} {item_type}</title></head>
<body>
<h2>{item_type} on: {file_name}</h2>
<p><strong>{file_type} ID:</strong> {file_id}</p>
<p><strong>From:</strong> {sender_name} ({sender})</p>
<p><strong>Comment ID:</strong> {comment_id}</p>
<p><strong>Tracking ID:</strong> {tracking_id}</p>
{quoted_section}
{thread_html}
<hr>
<h3>{file_type} Content:</h3>
<p>The full {file_type_lower} content is available in: <code>incoming_email/{content_file}</code></p>
<p>Read this file to understand the context and make appropriate edits or suggestions.</p>
{cli_hint}
<hr>
<p><em>Respond by writing to reply_email_draft.html</em></p>
</body>
</html>"#,
        display_name = display_name,
        item_type = item_type,
        file_name = file_name,
        file_type = file_type,
        file_type_lower = file_type.to_lowercase(),
        file_id = file_id,
        sender_name = sender_name,
        sender = message.sender,
        comment_id = actionable.comment.id,
        tracking_id = actionable.tracking_id,
        quoted_section = if quoted_text.is_empty() {
            String::new()
        } else {
            format!(
                "<h3>Quoted text:</h3><blockquote>{}</blockquote>",
                quoted_text
            )
        },
        thread_html = thread_html,
        content_file = content_file,
        cli_hint = cli_hint
    );

    let html_path = incoming_dir.join(format!("{:05}_email.html", seq));
    std::fs::write(&html_path, &html_content)?;

    // Create metadata file
    let channel_name = match channel {
        Channel::GoogleDocs => "google_docs",
        Channel::GoogleSheets => "google_sheets",
        Channel::GoogleSlides => "google_slides",
        _ => "google_workspace",
    };
    let meta_path = incoming_dir.join(format!("{:05}_{}_meta.json", seq, prefix));
    let meta = serde_json::json!({
        "channel": channel_name,
        "sender": message.sender,
        "sender_name": message.sender_name,
        "file_id": file_id,
        "file_name": file_name,
        "comment_id": actionable.comment.id,
        "tracking_id": actionable.tracking_id,
        "is_reply": actionable.triggering_reply.is_some(),
        "thread_id": message.thread_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    let item_type_lower = if actionable.triggering_reply.is_some() {
        "reply"
    } else {
        "comment"
    };
    info!(
        "saved {} {} seq={} tracking_id={} to {}",
        display_name,
        item_type_lower,
        seq,
        actionable.tracking_id,
        incoming_dir.display()
    );
    Ok(())
}
