use std::fs;
use std::path::PathBuf;

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

/// Execute a SendReplyTask via Slack.
pub(crate) fn execute_slack_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::slack::SlackOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let bot_token = std::env::var("SLACK_BOT_TOKEN")
        .map_err(|_| SchedulerError::TaskFailed("SLACK_BOT_TOKEN not set".to_string()))?;

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

/// Execute a SendReplyTask via Discord.
pub(crate) fn execute_discord_send(task: &SendReplyTask) -> Result<(), SchedulerError> {
    use crate::adapters::discord::DiscordOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    dotenvy::dotenv().ok();
    let bot_token = std::env::var("DISCORD_BOT_TOKEN")
        .map_err(|_| SchedulerError::TaskFailed("DISCORD_BOT_TOKEN not set".to_string()))?;

    let adapter = DiscordOutboundAdapter::new(bot_token);

    let text_body = if task.html_path.exists() {
        fs::read_to_string(&task.html_path).unwrap_or_default()
    } else {
        String::new()
    };

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
