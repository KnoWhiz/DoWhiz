use std::fs;
use std::path::{Path, PathBuf};

use tracing::{info, warn};

use crate::channel::Channel;
use crate::employee_config;
use crate::service;

use super::types::{SchedulerError, SendReplyTask};

/// Execute a SendReplyTask via email (Postmark).
pub(crate) fn execute_email_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    let params = send_emails_module::SendEmailParams {
        subject: task.subject.clone(),
        html_path: task.html_path.clone(),
        attachments_dir: task.attachments_dir.clone(),
        from: task.from.clone(),
        to: task.to.clone(),
        cc: task.cc.clone(),
        bcc: task.bcc.clone(),
        in_reply_to: task.in_reply_to.clone(),
        references: task.references.clone(),
        reply_to: None,
    };
    let response = send_emails_module::send_email(&params)
        .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;

    if let Some(archive_root) = task.archive_root.as_ref() {
        dotenvy::dotenv().ok();
        let from = task
            .from
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("");
        if let Err(err) = crate::past_emails::archive_outbound(
            archive_root,
            &task.subject,
            &task.html_path,
            &task.attachments_dir,
            &task.to,
            &task.cc,
            &task.bcc,
            task.in_reply_to.as_deref(),
            task.references.as_deref(),
            &response.message_id,
            &response.submitted_at,
            from,
        ) {
            warn!("failed to archive outbound email: {}", err);
        }
    }
    Ok(())
}

/// Resolve the Slack bot token for a specific employee.
///
/// Looks for `{EMPLOYEE}_SLACK_BOT_TOKEN` env var first (e.g., `OLIVER_SLACK_BOT_TOKEN`),
/// then falls back to the global `SLACK_BOT_TOKEN`.
fn resolve_slack_bot_token_for_employee(
    employee_id: Option<&str>,
) -> Result<String, SchedulerError> {
    if let Some(emp_id) = employee_id {
        let emp_upper = emp_id.to_uppercase().replace('-', "_");
        let emp_token_key = format!("{}_SLACK_BOT_TOKEN", emp_upper);
        if let Ok(token) = std::env::var(&emp_token_key) {
            if !token.trim().is_empty() {
                info!("using {} for employee {}", emp_token_key, emp_id);
                return Ok(token);
            }
        }
    }
    std::env::var("SLACK_BOT_TOKEN")
        .map_err(|_| SchedulerError::TaskFailed("SLACK_BOT_TOKEN not set".to_string()))
}

/// Execute a SendReplyTask via Slack.
pub(crate) fn execute_slack_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::slack::SlackOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let bot_token = resolve_slack_bot_token_for_employee(task.employee_id.as_deref())?;

    let adapter = SlackOutboundAdapter::new(bot_token);

    // Read text content from html_path if it exists
    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

    let message = OutboundMessage {
        channel: Channel::Slack,
        from: task.from.clone(),
        to: task.to.clone(),
        cc: vec![],
        bcc: vec![],
        subject: task.subject.clone(),
        text_body,
        html_body: String::new(),
        html_path: Some(task.html_path.clone()),
        attachments_dir: Some(task.attachments_dir.clone()),
        thread_id: task.in_reply_to.clone(), // Use in_reply_to as thread_ts for Slack
        metadata: ChannelMetadata {
            slack_channel_id: task.to.first().cloned(),
            ..Default::default()
        },
    };

    let result = adapter
        .send(&message)
        .map_err(|err| SchedulerError::TaskFailed(format!("Slack send failed: {}", err)))?;

    if !result.success {
        return Err(SchedulerError::TaskFailed(format!(
            "Slack API error: {}",
            result.error.unwrap_or_default()
        )));
    }

    info!(
        "sent Slack message to {:?}, message_id={}",
        task.to, result.message_id
    );
    Ok(())
}

/// Resolve the Discord bot token for a specific employee.
///
/// Looks for `{EMPLOYEE}_DISCORD_BOT_TOKEN` env var first (e.g., `LITTLE_BEAR_DISCORD_BOT_TOKEN`),
/// then falls back to the global `DISCORD_BOT_TOKEN`.
fn resolve_discord_bot_token_for_employee(
    employee_id: Option<&str>,
) -> Result<String, SchedulerError> {
    if let Some(emp_id) = employee_id {
        let emp_upper = emp_id.to_uppercase().replace('-', "_");
        let emp_token_key = format!("{}_DISCORD_BOT_TOKEN", emp_upper);
        if let Ok(token) = std::env::var(&emp_token_key) {
            if !token.trim().is_empty() {
                info!("using {} for employee {}", emp_token_key, emp_id);
                return Ok(token);
            }
        }
    }
    std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| SchedulerError::TaskFailed("DISCORD_BOT_TOKEN not set".to_string()))
}

fn read_cached_azure_url(sidecar_path: &Path) -> Option<String> {
    fs::read_to_string(sidecar_path)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| value.starts_with("http://") || value.starts_with("https://"))
}

fn collect_discord_attachment_links(attachments_dir: &Path) -> Vec<(String, String)> {
    if !attachments_dir.is_dir() {
        return Vec::new();
    }

    let mut attachment_paths = match fs::read_dir(attachments_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok().map(|value| value.path()))
            .filter(|path| path.is_file())
            .collect::<Vec<_>>(),
        Err(err) => {
            warn!(
                "failed to read Discord attachments dir {}: {}",
                attachments_dir.display(),
                err
            );
            return Vec::new();
        }
    };
    attachment_paths.sort();

    let envelope_id = uuid::Uuid::new_v4();
    let received_at = chrono::Utc::now();
    let mut links = Vec::new();

    for (index, path) in attachment_paths.into_iter().enumerate() {
        let file_name = path
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_default();
        if file_name.is_empty() || file_name.ends_with(".azure_url") {
            continue;
        }

        let sidecar_path = path.with_file_name(format!("{}.azure_url", file_name));
        if let Some(url) = read_cached_azure_url(&sidecar_path) {
            links.push((file_name, url));
            continue;
        }

        let bytes = match fs::read(&path) {
            Ok(value) if !value.is_empty() => value,
            Ok(_) => {
                warn!(
                    "skipping empty Discord attachment for blob upload: {}",
                    path.display()
                );
                continue;
            }
            Err(err) => {
                warn!(
                    "failed to read Discord attachment {}: {}",
                    path.display(),
                    err
                );
                continue;
            }
        };

        let storage_ref = match crate::raw_payload_store::upload_attachment_azure_blocking(
            envelope_id,
            received_at,
            index,
            &file_name,
            &bytes,
        ) {
            Ok(value) => value,
            Err(err) => {
                warn!(
                    "failed to upload Discord attachment {} to Azure: {}",
                    path.display(),
                    err
                );
                continue;
            }
        };

        let blob_url = match crate::raw_payload_store::resolve_azure_blob_url(&storage_ref) {
            Ok(value) => value,
            Err(err) => {
                warn!(
                    "failed to resolve Azure URL for Discord attachment {}: {}",
                    path.display(),
                    err
                );
                continue;
            }
        };

        if let Err(err) = fs::write(&sidecar_path, &blob_url) {
            warn!(
                "failed to write Discord attachment URL sidecar {}: {}",
                sidecar_path.display(),
                err
            );
        }

        links.push((file_name, blob_url));
    }

    links
}

fn append_discord_attachment_links(base_text: &str, attachments_dir: &Path) -> String {
    let links = collect_discord_attachment_links(attachments_dir);
    if links.is_empty() {
        return base_text.to_string();
    }

    let mut suffix = String::from("Attachments:\n");
    for (name, url) in links {
        suffix.push_str(&format!("- {}: {}\n", name, url));
    }

    if base_text.trim().is_empty() {
        suffix.trim_end().to_string()
    } else {
        format!("{}\n\n{}", base_text.trim_end(), suffix.trim_end())
    }
}

/// Execute a SendReplyTask via Discord.
pub(crate) fn execute_discord_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::discord::DiscordOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let bot_token = resolve_discord_bot_token_for_employee(task.employee_id.as_deref())?;

    let adapter = DiscordOutboundAdapter::new(bot_token);

    let base_text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };
    let text_body = append_discord_attachment_links(&base_text_body, &task.attachments_dir);

    let channel_id = task.to.first().and_then(|value| value.parse::<u64>().ok());

    let message = OutboundMessage {
        channel: Channel::Discord,
        from: task.from.clone(),
        to: task.to.clone(),
        cc: vec![],
        bcc: vec![],
        subject: task.subject.clone(),
        text_body,
        html_body: String::new(),
        html_path: Some(task.html_path.clone()),
        attachments_dir: Some(task.attachments_dir.clone()),
        thread_id: task.in_reply_to.clone(),
        metadata: ChannelMetadata {
            discord_channel_id: channel_id,
            ..Default::default()
        },
    };

    let result = adapter
        .send(&message)
        .map_err(|err| SchedulerError::TaskFailed(format!("Discord send failed: {}", err)))?;

    if !result.success {
        return Err(SchedulerError::TaskFailed(format!(
            "Discord API error: {}",
            result.error.unwrap_or_default()
        )));
    }

    info!(
        "sent Discord message to {:?}, message_id={}",
        task.to, result.message_id
    );
    Ok(())
}

/// Execute a SendReplyTask via BlueBubbles (iMessage).
pub(crate) fn execute_bluebubbles_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::bluebubbles::BlueBubblesOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let server_url = std::env::var("BLUEBUBBLES_URL")
        .or_else(|_| std::env::var("BLUEBUBBLES_SERVER_URL"))
        .map_err(|_| SchedulerError::TaskFailed("BLUEBUBBLES_URL not set".to_string()))?;
    let password = std::env::var("BLUEBUBBLES_PASSWORD")
        .map_err(|_| SchedulerError::TaskFailed("BLUEBUBBLES_PASSWORD not set".to_string()))?;

    let adapter = BlueBubblesOutboundAdapter::new(server_url, password);

    // Read plain text content from reply_message.txt (html_path field reused for simplicity)
    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

    // For BlueBubbles, to[0] contains the chat_guid
    let chat_guid = task.to.first().cloned();

    let message = OutboundMessage {
        channel: Channel::BlueBubbles,
        from: task.from.clone(),
        to: task.to.clone(),
        cc: vec![],
        bcc: vec![],
        subject: task.subject.clone(),
        text_body,
        html_body: String::new(),
        html_path: Some(task.html_path.clone()),
        attachments_dir: Some(task.attachments_dir.clone()),
        thread_id: task.in_reply_to.clone(),
        metadata: ChannelMetadata {
            bluebubbles_chat_guid: chat_guid,
            ..Default::default()
        },
    };

    let result = adapter
        .send(&message)
        .map_err(|err| SchedulerError::TaskFailed(format!("BlueBubbles send failed: {}", err)))?;

    if !result.success {
        return Err(SchedulerError::TaskFailed(format!(
            "BlueBubbles API error: {}",
            result.error.unwrap_or_default()
        )));
    }

    info!(
        "sent BlueBubbles message to {:?}, message_id={}",
        task.to, result.message_id
    );
    Ok(())
}

fn env_var_non_empty(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn resolve_employee_profile_from_env() -> Option<employee_config::EmployeeProfile> {
    let config_path = env_var_non_empty("EMPLOYEE_CONFIG_PATH")
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
    let employee_directory = employee_config::load_employee_directory(&config_path).ok()?;
    let employee_id = env_var_non_empty("EMPLOYEE_ID")
        .or_else(|| employee_directory.default_employee_id.clone())?;
    employee_directory.employee(&employee_id).cloned()
}

fn resolve_telegram_bot_token_from_env() -> Option<String> {
    resolve_employee_profile_from_env()
        .and_then(|profile| service::resolve_telegram_bot_token(&profile))
        .or_else(|| env_var_non_empty("TELEGRAM_BOT_TOKEN"))
}

/// Execute a SendReplyTask via Telegram Bot API.
pub(crate) fn execute_telegram_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::telegram::TelegramOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let bot_token = resolve_telegram_bot_token_from_env().ok_or_else(|| {
        SchedulerError::TaskFailed("telegram bot token not configured".to_string())
    })?;

    let adapter = TelegramOutboundAdapter::new(bot_token);

    // Read plain text content from reply_message.txt (html_path field reused for simplicity)
    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

    // For Telegram, to[0] contains the chat_id as a string
    let chat_id = task.to.first().and_then(|s| s.parse::<i64>().ok());

    let message = OutboundMessage {
        channel: Channel::Telegram,
        from: task.from.clone(),
        to: task.to.clone(),
        cc: vec![],
        bcc: vec![],
        subject: task.subject.clone(),
        text_body,
        html_body: String::new(),
        html_path: Some(task.html_path.clone()),
        attachments_dir: Some(task.attachments_dir.clone()),
        thread_id: task.in_reply_to.clone(),
        metadata: ChannelMetadata {
            telegram_chat_id: chat_id,
            ..Default::default()
        },
    };

    let result = adapter
        .send(&message)
        .map_err(|err| SchedulerError::TaskFailed(format!("Telegram send failed: {}", err)))?;

    if !result.success {
        return Err(SchedulerError::TaskFailed(format!(
            "Telegram API error: {}",
            result.error.unwrap_or_default()
        )));
    }

    info!(
        "sent Telegram message to {:?}, message_id={}",
        task.to, result.message_id
    );
    Ok(())
}

/// Execute a SendReplyTask via WhatsApp Cloud API.
pub(crate) fn execute_whatsapp_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::whatsapp::WhatsAppOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let access_token = env_var_non_empty("WHATSAPP_ACCESS_TOKEN")
        .ok_or_else(|| SchedulerError::TaskFailed("WHATSAPP_ACCESS_TOKEN not set".to_string()))?;
    let phone_number_id = env_var_non_empty("WHATSAPP_PHONE_NUMBER_ID").ok_or_else(|| {
        SchedulerError::TaskFailed("WHATSAPP_PHONE_NUMBER_ID not set".to_string())
    })?;

    let adapter = WhatsAppOutboundAdapter::new(access_token, phone_number_id);

    // Read plain text content from reply_message.txt (html_path field reused for simplicity)
    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

    // For WhatsApp, to[0] contains the phone number
    let phone_number = task.to.first().cloned();

    let message = OutboundMessage {
        channel: Channel::WhatsApp,
        from: task.from.clone(),
        to: task.to.clone(),
        cc: vec![],
        bcc: vec![],
        subject: task.subject.clone(),
        text_body,
        html_body: String::new(),
        html_path: Some(task.html_path.clone()),
        attachments_dir: Some(task.attachments_dir.clone()),
        thread_id: task.in_reply_to.clone(),
        metadata: ChannelMetadata {
            whatsapp_phone_number: phone_number,
            ..Default::default()
        },
    };

    let result = adapter
        .send(&message)
        .map_err(|err| SchedulerError::TaskFailed(format!("WhatsApp send failed: {}", err)))?;

    if !result.success {
        return Err(SchedulerError::TaskFailed(format!(
            "WhatsApp API error: {}",
            result.error.unwrap_or_default()
        )));
    }

    info!(
        "sent WhatsApp message to {:?}, message_id={}",
        task.to, result.message_id
    );
    Ok(())
}

/// Execute a SendReplyTask via SMS (Twilio).
pub(crate) fn execute_sms_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    dotenvy::dotenv().ok();

    let account_sid = std::env::var("TWILIO_ACCOUNT_SID")
        .map_err(|_| SchedulerError::TaskFailed("TWILIO_ACCOUNT_SID not set".to_string()))?;
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN")
        .map_err(|_| SchedulerError::TaskFailed("TWILIO_AUTH_TOKEN not set".to_string()))?;

    let from = task
        .from
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| SchedulerError::TaskFailed("SMS from number missing".to_string()))?;

    let to = task
        .to
        .first()
        .map(|value| value.as_str())
        .ok_or_else(|| SchedulerError::TaskFailed("SMS to number missing".to_string()))?;

    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

    let api_base = std::env::var("TWILIO_API_BASE_URL")
        .unwrap_or_else(|_| "https://api.twilio.com".to_string());
    let url = format!(
        "{}/2010-04-01/Accounts/{}/Messages.json",
        api_base.trim_end_matches('/'),
        account_sid
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .basic_auth(&account_sid, Some(&auth_token))
        .form(&[("To", to), ("From", from), ("Body", text_body.trim())])
        .send()
        .map_err(|err| SchedulerError::TaskFailed(format!("Twilio send failed: {}", err)))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(SchedulerError::TaskFailed(format!(
            "Twilio API error {}: {}",
            status, body
        )));
    }

    info!("sent SMS message to {}", to);
    Ok(())
}

/// Execute a SendReplyTask via Google Docs (reply to comment).
pub(crate) fn execute_google_docs_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::google_docs::GoogleDocsOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};
    use crate::google_auth::{GoogleAuth, GoogleAuthConfig};

    dotenvy::dotenv().ok();
    let config = GoogleAuthConfig::from_env();
    if !config.is_valid() {
        return Err(SchedulerError::TaskFailed(
            "Google OAuth credentials not configured".to_string(),
        ));
    }

    let auth = GoogleAuth::new(config)
        .map_err(|e| SchedulerError::TaskFailed(format!("Google auth failed: {}", e)))?;

    let adapter = GoogleDocsOutboundAdapter::new(auth);

    // Read text content from html_path if it exists
    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

    // Extract document_id and comment_id from task metadata
    // For Google Docs, we use in_reply_to format: "document_id:comment_id"
    let (document_id, comment_id) = task
        .in_reply_to
        .as_ref()
        .and_then(|reply_to| {
            let parts: Vec<&str> = reply_to.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .ok_or_else(|| {
            SchedulerError::TaskFailed(
                "Missing document_id:comment_id in in_reply_to for Google Docs".to_string(),
            )
        })?;

    let message = OutboundMessage {
        channel: Channel::GoogleDocs,
        from: task.from.clone(),
        to: task.to.clone(),
        cc: vec![],
        bcc: vec![],
        subject: task.subject.clone(),
        text_body,
        html_body: String::new(),
        html_path: Some(task.html_path.clone()),
        attachments_dir: Some(task.attachments_dir.clone()),
        thread_id: task.in_reply_to.clone(),
        metadata: ChannelMetadata {
            google_docs_document_id: Some(document_id),
            google_docs_comment_id: Some(comment_id),
            ..Default::default()
        },
    };

    let result = adapter
        .send(&message)
        .map_err(|err| SchedulerError::TaskFailed(format!("Google Docs send failed: {}", err)))?;

    if !result.success {
        return Err(SchedulerError::TaskFailed(format!(
            "Google Docs API error: {}",
            result.error.unwrap_or_default()
        )));
    }

    info!(
        "posted Google Docs reply to {:?}, reply_id={}",
        task.to, result.message_id
    );
    Ok(())
}

/// Get the central Notion reply queue directory for an employee.
pub fn notion_reply_queue_dir(employee_id: &str) -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".dowhiz")
        .join("notion_reply_queue")
        .join(employee_id)
}

/// Execute a SendReplyTask via Notion browser automation.
///
/// This function queues a reply request for the Notion browser poller to process.
/// The reply is written to a central queue directory that the poller monitors.
pub(crate) fn execute_notion_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    dotenvy::dotenv().ok();

    // Read text content from reply_message.txt (html_path field reused)
    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        return Err(SchedulerError::TaskFailed(
            "Notion reply: reply_message.txt not found".to_string(),
        ));
    };

    if text_body.trim().is_empty() {
        return Err(SchedulerError::TaskFailed(
            "Notion reply: reply_message.txt is empty".to_string(),
        ));
    }

    // Read Notion context from workspace
    let workspace_dir = task.html_path.parent().unwrap_or(Path::new("."));
    let context_path = workspace_dir.join(".notion_context.json");

    let context: serde_json::Value = if context_path.exists() {
        let content = fs::read_to_string(&context_path)
            .map_err(|e| SchedulerError::TaskFailed(format!("Failed to read context: {}", e)))?;
        serde_json::from_str(&content)
            .map_err(|e| SchedulerError::TaskFailed(format!("Failed to parse context: {}", e)))?
    } else {
        // Try to extract from in_reply_to which has format "notion:workspace:page:notification"
        let parts: Vec<&str> = task
            .in_reply_to
            .as_deref()
            .unwrap_or("")
            .split(':')
            .collect();
        if parts.len() >= 4 && parts[0] == "notion" {
            serde_json::json!({
                "workspace_id": parts[1],
                "page_id": parts[2],
                "notification_id": parts[3]
            })
        } else {
            return Err(SchedulerError::TaskFailed(
                "Notion reply: no context available (missing .notion_context.json)".to_string(),
            ));
        }
    };

    // Get employee ID from task or environment
    let employee_id = task
        .employee_id
        .clone()
        .or_else(|| std::env::var("EMPLOYEE_ID").ok())
        .unwrap_or_else(|| "default".to_string());

    // Create central queue directory
    let queue_dir = notion_reply_queue_dir(&employee_id);
    fs::create_dir_all(&queue_dir)
        .map_err(|e| SchedulerError::TaskFailed(format!("Failed to create queue dir: {}", e)))?;

    // Create a reply request for the poller to process
    let request_id = uuid::Uuid::new_v4().to_string();
    let reply_request = serde_json::json!({
        "id": request_id,
        "reply_text": text_body.trim(),
        "workspace_id": context.get("workspace_id"),
        "page_id": context.get("page_id"),
        "comment_id": context.get("comment_id"),
        "block_id": context.get("block_id"),
        "notification_id": context.get("notification_id"),
        "url": context.get("url"),
        "workspace_dir": workspace_dir.to_string_lossy(),
        "requested_at": chrono::Utc::now().to_rfc3339(),
        "status": "pending"
    });

    // Write to central queue with unique filename
    let request_filename = format!("{}.json", request_id);
    let request_path = queue_dir.join(&request_filename);
    fs::write(
        &request_path,
        serde_json::to_string_pretty(&reply_request)
            .map_err(|e| SchedulerError::TaskFailed(format!("Failed to serialize reply: {}", e)))?,
    )
    .map_err(|e| SchedulerError::TaskFailed(format!("Failed to write reply request: {}", e)))?;

    // Also write to workspace for reference
    let workspace_request_path = workspace_dir.join(".notion_reply_request.json");
    let _ = fs::write(&workspace_request_path, serde_json::to_string_pretty(&reply_request).unwrap_or_default());

    info!(
        "queued Notion reply request id={} to {:?}, page_id={:?}, queue={}",
        request_id,
        task.to,
        context.get("page_id"),
        queue_dir.display()
    );

    Ok(())
}
