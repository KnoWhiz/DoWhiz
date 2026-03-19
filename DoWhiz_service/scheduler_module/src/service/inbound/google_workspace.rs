//! Unified handler for Google Workspace (Docs, Sheets, Slides) comments.

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::account_store::AccountStore;
use crate::adapters::google_common::ActionableComment;
use crate::channel::Channel;
use crate::google_auth::{GoogleAuth, GoogleAuthConfig};
use crate::index_store::IndexStore;
use crate::user_store::{extract_emails, UserStore};
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::workspace::ensure_thread_workspace;
use crate::service::BoxError;

/// Process an incoming Google Workspace comment (Docs, Sheets, or Slides).
pub(crate) fn process_google_workspace_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    account_store: &AccountStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let actionable: ActionableComment = serde_json::from_slice(raw_payload)?;
    let channel = message.channel.clone();

    // Get file ID, name, and owner email based on channel
    let (file_id, file_name, channel_prefix, owner_email) = match channel {
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
            let owner = message.metadata.google_docs_owner_email.as_deref();
            (id, name, "gdocs", owner)
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
            let owner = message.metadata.google_sheets_owner_email.as_deref();
            (id, name, "gsheets", owner)
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
            let owner = message.metadata.google_slides_owner_email.as_deref();
            (id, name, "gslides", owner)
        }
        _ => return Err("Invalid channel for Google Workspace handler".into()),
    };

    // Extract commenter email, falling back to document owner email if unknown
    let extracted_email = extract_emails(&message.sender).into_iter().next();

    // Check if we got a real email or the "unknown@unknown.com" placeholder
    let user_email = match extracted_email {
        Some(email) if email != "unknown@unknown.com" => email,
        _ => {
            // Commenter email is not available or is the placeholder
            // Try to use owner email as fallback (works when commenter is also doc owner)
            if let Some(owner) = owner_email {
                info!(
                    "Commenter email unknown, using document owner email '{}' as fallback",
                    owner
                );
                owner.to_string()
            } else {
                format!(
                    "{}_{}@local",
                    channel_prefix,
                    message.sender.replace(' ', "_")
                )
            }
        }
    };
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
        reply_to: vec![user_email.clone()],
        reply_from: config.employee_profile.addresses.first().cloned(),
        archive_root: None,
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: channel.clone(),
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
        requester_identifier_type: None,
        requester_identifier: None,
        account_id: None,
    };

    // Clone run_task before consuming it, in case we need to write to account-level storage
    let run_task_for_account = run_task.clone();

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

    // If the Google Workspace user has linked their email, also write to account-level tasks.db
    // user_email is extracted from message.sender and maps to "email" identifier type
    info!(
        "Looking up account for email '{}' (sender was '{}')",
        user_email, message.sender
    );
    match account_store.get_account_by_identifier("email", &user_email) {
        Ok(Some(account)) => {
            info!("Found account {} for email {}", account.id, user_email);
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
                                "also enqueued task to account-level storage account={} task_id={} channel={}",
                                account.id, task_id, channel_display_name(&channel)
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
                "No account linked for email '{}', skipping account-level task",
                user_email
            );
        }
        Err(err) => {
            warn!(
                "Failed to look up account for email '{}': {}",
                user_email, err
            );
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_store::AccountStore;
    use crate::adapters::google_common::{ActionableComment, CommentAuthor, GoogleComment};
    use crate::channel::{ChannelMetadata, InboundMessage};
    use crate::employee_config::{EmployeeDirectory, EmployeeProfile};
    use crate::index_store::IndexStore;
    use crate::service::config::ServiceConfig;
    use crate::user_store::UserStore;
    use crate::{ModuleExecutor, Scheduler, TaskKind};
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::time::Duration;
    use tempfile::TempDir;

    fn create_test_config(root: &std::path::Path) -> ServiceConfig {
        let users_root = root.join("users");
        let state_root = root.join("state");
        fs::create_dir_all(&users_root).unwrap();
        fs::create_dir_all(&state_root).unwrap();

        let addresses = vec!["service@example.com".to_string()];
        let address_set: HashSet<String> = addresses
            .iter()
            .map(|value| value.to_ascii_lowercase())
            .collect();
        let employee = EmployeeProfile {
            id: "test-employee".to_string(),
            display_name: None,
            runner: "codex".to_string(),
            model: None,
            addresses,
            address_set: address_set.clone(),
            runtime_root: None,
            agents_path: None,
            claude_path: None,
            soul_path: None,
            skills_dir: None,
            discord_enabled: false,
            slack_enabled: false,
            bluebubbles_enabled: false,
        };
        let mut employee_by_id = HashMap::new();
        employee_by_id.insert(employee.id.clone(), employee.clone());
        let employee_directory = EmployeeDirectory {
            employees: vec![employee.clone()],
            employee_by_id,
            default_employee_id: Some(employee.id.clone()),
            service_addresses: address_set,
        };

        ServiceConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
            employee_id: employee.id.clone(),
            employee_config_path: root.join("employee.toml"),
            employee_profile: employee,
            employee_directory,
            workspace_root: root.join("workspaces"),
            scheduler_state_path: state_root.join("tasks.db"),
            processed_ids_path: state_root.join("processed_ids.txt"),
            ingestion_db_url: "postgres://localhost/test".to_string(),
            ingestion_poll_interval: Duration::from_millis(50),
            users_root,
            users_db_path: state_root.join("users.db"),
            task_index_path: state_root.join("task_index.db"),
            codex_model: "gpt-5.4".to_string(),
            codex_disabled: true,
            scheduler_poll_interval: Duration::from_millis(50),
            scheduler_max_concurrency: 1,
            scheduler_user_max_concurrency: 1,
            inbound_body_max_bytes: crate::service::DEFAULT_INBOUND_BODY_MAX_BYTES,
            skills_source_dir: None,
            slack_bot_token: None,
            slack_bot_user_id: None,
            slack_store_path: state_root.join("slack.db"),
            slack_client_id: None,
            slack_client_secret: None,
            slack_redirect_uri: None,
            discord_bot_token: None,
            discord_bot_user_id: None,
            google_docs_enabled: false,
            bluebubbles_url: None,
            bluebubbles_password: None,
            telegram_bot_token: None,
            whatsapp_access_token: None,
            whatsapp_phone_number_id: None,
            whatsapp_verify_token: None,
        }
    }

    fn create_actionable_comment(comment_id: &str, content: &str) -> ActionableComment {
        ActionableComment {
            comment: GoogleComment {
                id: comment_id.to_string(),
                content: content.to_string(),
                html_content: None,
                resolved: None,
                author: Some(CommentAuthor {
                    display_name: Some("Test User".to_string()),
                    email_address: Some("testuser@example.com".to_string()),
                    photo_link: None,
                    me: false,
                }),
                created_time: None,
                modified_time: None,
                replies: None,
                anchor: None,
                quoted_file_content: None,
            },
            triggering_reply: None,
            tracking_id: format!("tracking_{}", comment_id),
        }
    }

    #[test]
    fn process_google_docs_message_creates_run_task() -> Result<(), crate::service::BoxError> {
        let temp = TempDir::new()?;
        let config = create_test_config(temp.path());

        let user_store = UserStore::new(&config.users_db_path)?;
        let index_store = IndexStore::new(&config.task_index_path)?;
        let account_store = AccountStore::new(&config.ingestion_db_url)?;

        // Use angle bracket format so extract_emails() can parse it
        let sender_email = "testuser@example.com";
        let sender = format!("Test User <{}>", sender_email);
        let doc_id = "1abc123def456";
        let doc_name = "Test Document";
        let actionable = create_actionable_comment("comment-1", "Please review this section");
        let raw_payload = serde_json::to_vec(&actionable)?;

        let message = InboundMessage {
            channel: Channel::GoogleDocs,
            sender: sender.clone(),
            sender_name: Some("Test User".to_string()),
            recipient: doc_id.to_string(),
            subject: Some(format!("Comment on {}", doc_name)),
            text_body: Some(actionable.comment.content.clone()),
            html_body: None,
            thread_id: format!("gdocs:{}:{}", doc_id, actionable.comment.id),
            message_id: Some(actionable.tracking_id.clone()),
            attachments: Vec::new(),
            reply_to: vec![sender.clone()],
            raw_payload: raw_payload.clone(),
            metadata: ChannelMetadata {
                google_docs_document_id: Some(doc_id.to_string()),
                google_docs_document_name: Some(doc_name.to_string()),
                ..Default::default()
            },
        };

        process_google_workspace_message(
            &config,
            &user_store,
            &index_store,
            &account_store,
            &message,
            &raw_payload,
        )?;

        // Verify user was created with the extracted email
        let user = user_store.get_or_create_user("gdocs", sender_email)?;
        let user_paths = user_store.user_paths(&config.users_root, &user.user_id);

        // Verify task was created
        let scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
        let run_task = scheduler
            .tasks()
            .iter()
            .find_map(|task| match &task.kind {
                TaskKind::RunTask(run) => Some(run),
                _ => None,
            })
            .expect("run task created");

        assert_eq!(run_task.channel, Channel::GoogleDocs);
        assert!(run_task.workspace_dir.exists());

        // Verify files were written using thread_state seq
        let state_path = crate::thread_state::default_thread_state_path(&run_task.workspace_dir);
        let thread_state = crate::thread_state::load_thread_state(&state_path)
            .ok_or("thread_state.json not found")?;
        let seq = thread_state.last_email_seq;

        let incoming_dir = run_task.workspace_dir.join("incoming_email");
        assert!(incoming_dir
            .join(format!("{:05}_gdocs_comment.json", seq))
            .exists());
        assert!(incoming_dir.join(format!("{:05}_email.html", seq)).exists());
        assert!(incoming_dir
            .join(format!("{:05}_gdocs_meta.json", seq))
            .exists());

        Ok(())
    }

    #[test]
    fn process_google_sheets_message_creates_run_task() -> Result<(), crate::service::BoxError> {
        let temp = TempDir::new()?;
        let config = create_test_config(temp.path());

        let user_store = UserStore::new(&config.users_db_path)?;
        let index_store = IndexStore::new(&config.task_index_path)?;
        let account_store = AccountStore::new(&config.ingestion_db_url)?;

        let sender_email = "testuser@example.com";
        let sender = format!("Test User <{}>", sender_email);
        let spreadsheet_id = "spreadsheet-abc123";
        let actionable = create_actionable_comment("comment-2", "Check these numbers");
        let raw_payload = serde_json::to_vec(&actionable)?;

        let message = InboundMessage {
            channel: Channel::GoogleSheets,
            sender: sender.clone(),
            sender_name: Some("Test User".to_string()),
            recipient: spreadsheet_id.to_string(),
            subject: None,
            text_body: Some(actionable.comment.content.clone()),
            html_body: None,
            thread_id: format!("gsheets:{}:{}", spreadsheet_id, actionable.comment.id),
            message_id: Some(actionable.tracking_id.clone()),
            attachments: Vec::new(),
            reply_to: vec![sender.clone()],
            raw_payload: raw_payload.clone(),
            metadata: ChannelMetadata {
                google_sheets_spreadsheet_id: Some(spreadsheet_id.to_string()),
                google_sheets_spreadsheet_name: Some("Test Spreadsheet".to_string()),
                ..Default::default()
            },
        };

        process_google_workspace_message(
            &config,
            &user_store,
            &index_store,
            &account_store,
            &message,
            &raw_payload,
        )?;

        let user = user_store.get_or_create_user("gsheets", sender_email)?;
        let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
        let scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;

        let run_task = scheduler
            .tasks()
            .iter()
            .find_map(|task| match &task.kind {
                TaskKind::RunTask(run) => Some(run),
                _ => None,
            })
            .expect("run task created");

        assert_eq!(run_task.channel, Channel::GoogleSheets);

        Ok(())
    }

    #[test]
    fn process_google_slides_message_creates_run_task() -> Result<(), crate::service::BoxError> {
        let temp = TempDir::new()?;
        let config = create_test_config(temp.path());

        let user_store = UserStore::new(&config.users_db_path)?;
        let index_store = IndexStore::new(&config.task_index_path)?;
        let account_store = AccountStore::new(&config.ingestion_db_url)?;

        let sender_email = "testuser@example.com";
        let sender = format!("Test User <{}>", sender_email);
        let presentation_id = "presentation-xyz789";
        let actionable = create_actionable_comment("comment-3", "Update this slide");
        let raw_payload = serde_json::to_vec(&actionable)?;

        let message = InboundMessage {
            channel: Channel::GoogleSlides,
            sender: sender.clone(),
            sender_name: Some("Test User".to_string()),
            recipient: presentation_id.to_string(),
            subject: None,
            text_body: Some(actionable.comment.content.clone()),
            html_body: None,
            thread_id: format!("gslides:{}:{}", presentation_id, actionable.comment.id),
            message_id: Some(actionable.tracking_id.clone()),
            attachments: Vec::new(),
            reply_to: vec![sender.clone()],
            raw_payload: raw_payload.clone(),
            metadata: ChannelMetadata {
                google_slides_presentation_id: Some(presentation_id.to_string()),
                google_slides_presentation_name: Some("Test Presentation".to_string()),
                ..Default::default()
            },
        };

        process_google_workspace_message(
            &config,
            &user_store,
            &index_store,
            &account_store,
            &message,
            &raw_payload,
        )?;

        let user = user_store.get_or_create_user("gslides", sender_email)?;
        let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
        let scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;

        let run_task = scheduler
            .tasks()
            .iter()
            .find_map(|task| match &task.kind {
                TaskKind::RunTask(run) => Some(run),
                _ => None,
            })
            .expect("run task created");

        assert_eq!(run_task.channel, Channel::GoogleSlides);

        Ok(())
    }
}
