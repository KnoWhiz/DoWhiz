use std::io;
use std::path::Path;

use tracing::{info, warn};

use crate::adapters::bluebubbles::send_quick_bluebubbles_response;
use crate::adapters::telegram::send_quick_telegram_response;
use crate::message_router::{MessageRouter, RouterDecision};
use crate::slack_store::SlackStore;
use crate::user_store::UserStore;

use super::super::config::ServiceConfig;
use super::super::BoxError;

/// Read memo.md from a user's memory directory
fn read_user_memo(memory_dir: &Path) -> Option<String> {
    let memo_path = memory_dir.join("memo.md");
    std::fs::read_to_string(&memo_path).ok()
}

/// Append memory update to a user's memo.md
fn append_user_memo(memory_dir: &Path, update: &str) -> io::Result<()> {
    std::fs::create_dir_all(memory_dir)?;
    let memo_path = memory_dir.join("memo.md");
    let existing = std::fs::read_to_string(&memo_path).unwrap_or_default();
    let new_content = if existing.trim().is_empty() {
        format!("# Memo\n\n{}\n", update.trim())
    } else {
        format!("{}\n\n{}\n", existing.trim_end(), update.trim())
    };
    std::fs::write(&memo_path, new_content)
}

pub(crate) fn try_quick_response_slack(
    config: &ServiceConfig,
    user_store: &UserStore,
    slack_store: &SlackStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let channel_id = match message.metadata.slack_channel_id.as_deref() {
        Some(value) => value,
        None => return Ok(false),
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("slack", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let cleaned_text = text
        .split_whitespace()
        .filter(|word| !(word.starts_with("<@") && word.ends_with(">")))
        .collect::<Vec<_>>()
        .join(" ");

    let decision = runtime.block_on(message_router.classify(&cleaned_text, memory.as_deref()));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            let token = resolve_slack_bot_token(
                config,
                slack_store,
                message.metadata.slack_team_id.as_deref(),
            );
            if let Some(token) = token {
                let thread_ts = Some(message.thread_id.as_str());
                if runtime
                    .block_on(send_quick_slack_response(
                        &token,
                        channel_id,
                        thread_ts,
                        &response,
                    ))
                    .is_ok()
                {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

fn resolve_slack_bot_token(
    config: &ServiceConfig,
    slack_store: &SlackStore,
    team_id: Option<&str>,
) -> Option<String> {
    if let Some(team_id) = team_id {
        if let Ok(installation) = slack_store.get_installation_or_env(team_id) {
            if !installation.bot_token.trim().is_empty() {
                return Some(installation.bot_token);
            }
        }
    }
    config.slack_bot_token.clone()
}

pub(crate) fn try_quick_response_bluebubbles(
    config: &ServiceConfig,
    user_store: &UserStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let Some(chat_guid) = message.metadata.bluebubbles_chat_guid.as_deref() else {
        return Ok(false);
    };
    let Some(url) = config.bluebubbles_url.as_deref() else {
        return Ok(false);
    };
    let Some(password) = config.bluebubbles_password.as_deref() else {
        return Ok(false);
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("phone", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let decision = runtime.block_on(message_router.classify(text, memory.as_deref()));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            if runtime
                .block_on(send_quick_bluebubbles_response(url, password, chat_guid, &response))
                .is_ok()
            {
                return Ok(true);
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

pub(crate) fn try_quick_response_discord(
    config: &ServiceConfig,
    user_store: &UserStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let channel_id = match message.metadata.discord_channel_id {
        Some(value) => value,
        None => return Ok(false),
    };
    let message_id = message.message_id.as_deref();
    let token = match config.discord_bot_token.as_deref() {
        Some(token) => token,
        None => return Ok(false),
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("discord", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let decision = runtime.block_on(message_router.classify(text, memory.as_deref()));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            if send_quick_discord_response_simple(token, channel_id, message_id, &response).is_ok()
            {
                return Ok(true);
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

pub(crate) fn try_quick_response_telegram(
    config: &ServiceConfig,
    user_store: &UserStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let Some(chat_id) = message.metadata.telegram_chat_id else {
        return Ok(false);
    };
    let Some(token) = config.telegram_bot_token.as_deref() else {
        return Ok(false);
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("telegram", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let decision = runtime.block_on(message_router.classify(text, memory.as_deref()));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            if runtime
                .block_on(send_quick_telegram_response(token, chat_id, &response))
                .is_ok()
            {
                return Ok(true);
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

/// Send a quick response via Slack for locally-handled queries (async version).
async fn send_quick_slack_response(
    bot_token: &str,
    channel: &str,
    thread_ts: Option<&str>,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_base = std::env::var("SLACK_API_BASE_URL")
        .unwrap_or_else(|_| "https://slack.com/api".to_string());
    let url = format!("{}/chat.postMessage", api_base.trim_end_matches('/'));

    let mut request = serde_json::json!({
        "channel": channel,
        "text": response_text,
        "mrkdwn": true
    });

    if let Some(ts) = thread_ts {
        request["thread_ts"] = serde_json::Value::String(ts.to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Slack API returned {}: {}", status, body).into());
    }

    let api_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if api_response.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        let error = api_response
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or("unknown error");
        return Err(format!("Slack API error: {}", error).into());
    }

    Ok(())
}

fn send_quick_discord_response_simple(
    bot_token: &str,
    channel_id: u64,
    message_id: Option<&str>,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use crate::adapters::discord::DiscordOutboundAdapter;
    use crate::channel::{Channel, ChannelMetadata, OutboundAdapter, OutboundMessage};

    let adapter = DiscordOutboundAdapter::new(bot_token.to_string());

    let message = OutboundMessage {
        channel: Channel::Discord,
        from: None,
        to: vec![channel_id.to_string()],
        cc: vec![],
        bcc: vec![],
        subject: String::new(),
        text_body: response_text.to_string(),
        html_body: String::new(),
        html_path: None,
        attachments_dir: None,
        thread_id: message_id.map(|value| value.to_string()),
        metadata: ChannelMetadata {
            discord_channel_id: Some(channel_id),
            ..Default::default()
        },
    };

    let result = adapter.send(&message)?;
    if !result.success {
        return Err(result.error.unwrap_or_else(|| "discord send failed".to_string()).into());
    }
    Ok(())
}
