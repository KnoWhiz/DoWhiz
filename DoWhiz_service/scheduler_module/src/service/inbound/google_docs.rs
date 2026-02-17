use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::adapters::google_docs::ActionableComment;
use crate::channel::Channel;
use crate::google_auth::GoogleAuth;
use crate::index_store::IndexStore;
use crate::user_store::{extract_emails, UserStore};
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::workspace::ensure_thread_workspace;
use super::super::BoxError;

pub(crate) fn process_google_docs_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::adapters::google_docs::GoogleDocsInboundAdapter;

    let actionable: ActionableComment = serde_json::from_slice(raw_payload)?;
    let document_id = message
        .metadata
        .google_docs_document_id
        .as_deref()
        .ok_or("missing google_docs_document_id")?;
    let user_email = extract_emails(&message.sender)
        .into_iter()
        .next()
        .unwrap_or_else(|| format!("gdocs_{}@local", message.sender.replace(' ', "_")));
    let user = user_store.get_or_create_user("google_docs", &user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!("gdocs:{}:{}", document_id, actionable.comment.id);

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

    append_google_docs_comment(&workspace, message, &actionable, thread_state.last_email_seq)?;

    if let Ok(auth) = GoogleAuth::from_env() {
        let adapter = GoogleDocsInboundAdapter::new(auth, HashSet::new());
        match adapter.read_document_content(document_id) {
            Ok(doc_content) => {
                let doc_content_path = workspace.join("incoming_email").join("document_content.txt");
                if let Err(e) = std::fs::write(&doc_content_path, &doc_content) {
                    warn!(
                        "Failed to save document content for {}: {}",
                        document_id, e
                    );
                }
            }
            Err(e) => {
                warn!("Failed to fetch document content for {}: {}", document_id, e);
            }
        }
    }

    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config.employee_profile.runner.eq_ignore_ascii_case("claude") {
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
        channel: Channel::GoogleDocs,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
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

/// Save an incoming Google Docs comment or reply to the workspace.
pub(super) fn append_google_docs_comment(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    actionable: &ActionableComment,
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    // Save the raw comment JSON (includes all replies)
    let raw_path = incoming_dir.join(format!("{:05}_gdocs_comment.json", seq));
    let raw_json = serde_json::to_string_pretty(&actionable.comment)?;
    std::fs::write(&raw_path, &raw_json)?;

    // Create HTML representation for the agent
    let doc_name = message.metadata.google_docs_document_name.as_deref().unwrap_or("Document");
    let doc_id = message.metadata.google_docs_document_id.as_deref().unwrap_or("");
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

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>Google Docs {}</title></head>
<body>
<h2>{} on: {}</h2>
<p><strong>Document ID:</strong> {}</p>
<p><strong>From:</strong> {} ({})</p>
<p><strong>Comment ID:</strong> {}</p>
<p><strong>Tracking ID:</strong> {}</p>
{}
{}
<hr>
<h3>Document Content:</h3>
<p>The full document content is available in: <code>incoming_email/document_content.txt</code></p>
<p>Read this file to understand the document context and make appropriate edits or suggestions.</p>
<hr>
<p><em>Respond by writing to reply_email_draft.html</em></p>
</body>
</html>"#,
        item_type,
        item_type,
        doc_name,
        doc_id,
        sender_name,
        message.sender,
        actionable.comment.id,
        actionable.tracking_id,
        if quoted_text.is_empty() {
            String::new()
        } else {
            format!("<h3>Quoted text:</h3><blockquote>{}</blockquote>", quoted_text)
        },
        thread_html
    );

    let html_path = incoming_dir.join(format!("{:05}_email.html", seq));
    std::fs::write(&html_path, &html_content)?;

    // Create metadata file
    let meta_path = incoming_dir.join(format!("{:05}_gdocs_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "google_docs",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "document_id": doc_id,
        "document_name": doc_name,
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
        "saved Google Docs {} seq={} tracking_id={} to {}",
        item_type_lower,
        seq,
        actionable.tracking_id,
        incoming_dir.display()
    );
    Ok(())
}
