//! Discord Gateway client for receiving messages via WebSocket.
//!
//! This module provides a serenity-based event handler that connects to Discord's
//! Gateway WebSocket and processes incoming messages, converting them to tasks.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serenity::all::{Context, EventHandler, GatewayIntents, Message, Ready};
use serenity::async_trait;
use serenity::Client;
use tracing::{error, info, warn};

use crate::adapters::discord::{DiscordInboundAdapter, DiscordOutboundAdapter};
use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::message_router::{MessageRouter, RouterDecision};
use crate::service::ServiceConfig;
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

/// Paths for Discord guild-based organization.
#[derive(Debug, Clone)]
pub struct DiscordGuildPaths {
    pub root: PathBuf,
    pub state_dir: PathBuf,
    pub tasks_db_path: PathBuf,
    pub workspaces_root: PathBuf,
}

impl DiscordGuildPaths {
    /// Create paths for a Discord guild under the workspace root.
    /// Structure: {workspace_root}/discord/{guild_id}/
    pub fn new(workspace_root: &Path, guild_id: &str) -> Self {
        let root = workspace_root.join("discord").join(guild_id);
        let state_dir = root.join("state");
        Self {
            tasks_db_path: state_dir.join("tasks.db"),
            state_dir,
            workspaces_root: root.join("workspaces"),
            root,
        }
    }

    /// Ensure all directories exist.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.state_dir)?;
        fs::create_dir_all(&self.workspaces_root)?;
        Ok(())
    }

    /// Get synthetic user_id for index_store integration.
    /// Format: discord:{guild_id}
    pub fn user_id(guild_id: &str) -> String {
        format!("discord:{}", guild_id)
    }
}

/// Shared state for the Discord event handler.
#[derive(Clone)]
pub struct DiscordHandlerState {
    pub config: Arc<ServiceConfig>,
    pub index_store: Arc<IndexStore>,
    /// Message router for handling simple queries locally
    pub message_router: Arc<MessageRouter>,
    /// Outbound adapter for sending quick responses
    pub outbound_adapter: DiscordOutboundAdapter,
}

/// Serenity event handler for Discord Gateway events.
pub struct DiscordEventHandler {
    state: DiscordHandlerState,
    adapter: DiscordInboundAdapter,
}

impl DiscordEventHandler {
    pub fn new(state: DiscordHandlerState, bot_user_ids: HashSet<u64>) -> Self {
        Self {
            state,
            adapter: DiscordInboundAdapter::new(bot_user_ids),
        }
    }
}

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Discord bot connected as {}", ready.user.name);
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        // Convert serenity Message to InboundMessage
        let inbound = match self.adapter.from_serenity_message(&msg) {
            Ok(m) => m,
            Err(e) => {
                // Most errors here are "ignoring bot message" which is expected
                if !e.to_string().contains("ignoring bot") {
                    warn!("failed to parse Discord message: {}", e);
                }
                return;
            }
        };

        // Only respond to:
        // 1. Messages that @ mention the bot
        // 2. Messages that are replies to the bot's messages
        let is_mention = msg
            .mentions
            .iter()
            .any(|u| self.adapter.bot_user_ids.contains(&u.id.get()));
        let is_reply_to_bot = msg
            .referenced_message
            .as_ref()
            .map(|ref_msg| self.adapter.bot_user_ids.contains(&ref_msg.author.id.get()))
            .unwrap_or(false);

        if !is_mention && !is_reply_to_bot {
            return;
        }

        let msg_len = inbound.text_body.as_ref().map(|t| t.len()).unwrap_or(0);
        info!(
            "Discord message from {} in channel {:?} (mention={}, reply_to_bot={}, len={}): {:?}",
            inbound.sender,
            inbound.metadata.discord_channel_id,
            is_mention,
            is_reply_to_bot,
            msg_len,
            inbound.text_body
        );

        // Try local router first for simple queries
        if let Some(text) = &inbound.text_body {
            match self.state.message_router.classify(text).await {
                RouterDecision::Simple(response) => {
                    info!("Router handled message locally, sending quick response");
                    if let Err(e) = send_quick_discord_response(
                        &self.state.outbound_adapter.bot_token,
                        &inbound,
                        &msg,
                        &response,
                    ).await {
                        error!("failed to send quick Discord response: {}", e);
                    }
                    return;
                }
                RouterDecision::Complex => {
                    info!("Router forwarding to full pipeline");
                }
                RouterDecision::Passthrough => {
                    info!("Router passthrough (disabled or error)");
                }
            }
        }

        // Process the message through full pipeline
        if let Err(e) = process_discord_message(&self.state, &inbound, &msg) {
            error!("failed to process Discord message: {}", e);
        }
    }
}

/// Process a Discord message and schedule a task.
fn process_discord_message(
    state: &DiscordHandlerState,
    message: &crate::channel::InboundMessage,
    raw_msg: &Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use crate::service::{bump_thread_state, default_thread_state_path};

    let config = &state.config;
    let index_store = &state.index_store;

    // Get IDs (required for Discord)
    let channel_id = message
        .metadata
        .discord_channel_id
        .ok_or("missing discord_channel_id")?;

    // Guild ID - use "dm" for direct messages
    let guild_id = message
        .metadata
        .discord_guild_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "dm".to_string());

    // Set up guild-based paths
    let guild_paths = DiscordGuildPaths::new(&config.workspace_root, &guild_id);
    guild_paths.ensure_dirs()?;

    // Thread key for conversation grouping
    let thread_key = format!("discord:{}:{}:{}", guild_id, channel_id, message.thread_id);

    // Create/get workspace for this channel conversation
    let workspace = ensure_discord_workspace(
        &guild_paths,
        channel_id,
        &message.thread_id,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    // Bump thread state
    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    // Save the incoming Discord message to workspace
    append_discord_message(&workspace, message, raw_msg, thread_state.last_email_seq)?;

    // Determine model and runner
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

    info!(
        "workspace ready at {} for guild {} thread={} epoch={}",
        workspace.display(),
        guild_id,
        thread_key,
        thread_state.epoch
    );

    // Create RunTask to process the message
    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![channel_id.to_string()],
        reply_from: None,
        archive_root: None, // Discord doesn't need mail archiving
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Discord,
        slack_team_id: None,
    };

    // Schedule the task using guild-based scheduler
    let mut scheduler = Scheduler::load(&guild_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) =
        crate::service::cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch)
    {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

    // Sync to index_store using synthetic user_id
    let synthetic_user_id = DiscordGuildPaths::user_id(&guild_id);
    index_store.sync_user_tasks(&synthetic_user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued guild={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        guild_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

/// Send a quick response via Discord for locally-handled queries (async version).
async fn send_quick_discord_response(
    bot_token: &str,
    inbound: &crate::channel::InboundMessage,
    raw_msg: &Message,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let channel_id = inbound
        .metadata
        .discord_channel_id
        .ok_or("missing discord_channel_id")?;

    let request = serde_json::json!({
        "content": response_text,
        "message_reference": {
            "message_id": raw_msg.id.get()
        }
    });

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://discord.com/api/v10/channels/{}/messages",
            channel_id
        ))
        .header("Authorization", format!("Bot {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if response.status().is_success() {
        let api_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        let message_id = api_response["id"].as_str().unwrap_or("unknown");
        info!(
            "Quick response sent to Discord channel {} message_id={}",
            channel_id, message_id
        );
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "unknown error".to_string());
        warn!("Quick response failed: {}", error_text);
    }

    Ok(())
}

/// Create or get a workspace for a Discord channel conversation.
fn ensure_discord_workspace(
    guild_paths: &DiscordGuildPaths,
    channel_id: u64,
    thread_id: &str,
    employee_profile: &crate::employee_config::EmployeeProfile,
    skills_source_dir: Option<&Path>,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    // Workspace path: {guild_root}/workspaces/{channel_id}/{thread_hash}/
    let thread_hash = &md5::compute(thread_id.as_bytes())
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()[..8];

    let workspace = guild_paths
        .workspaces_root
        .join(channel_id.to_string())
        .join(thread_hash);

    if !workspace.exists() {
        fs::create_dir_all(&workspace)?;

        // Create standard workspace directories
        fs::create_dir_all(workspace.join("incoming_email"))?;
        fs::create_dir_all(workspace.join("incoming_attachments"))?;
        fs::create_dir_all(workspace.join("memory"))?;
        fs::create_dir_all(workspace.join("references"))?;

        // Copy employee config files if available
        if let Some(agents_path) = &employee_profile.agents_path {
            if agents_path.exists() {
                fs::copy(agents_path, workspace.join("AGENTS.md"))?;
            }
        }
        if let Some(claude_path) = &employee_profile.claude_path {
            if claude_path.exists() {
                fs::copy(claude_path, workspace.join("CLAUDE.md"))?;
            }
        }
        if let Some(soul_path) = &employee_profile.soul_path {
            if soul_path.exists() {
                fs::copy(soul_path, workspace.join("SOUL.md"))?;
            }
        }

        // Copy skills if available
        if let Some(skills_dir) = skills_source_dir {
            let dest_skills = workspace.join("skills");
            if skills_dir.exists() && !dest_skills.exists() {
                crate::service::copy_dir_recursive(skills_dir, &dest_skills)?;
            }
        }

        info!("created Discord workspace at {}", workspace.display());
    }

    Ok(workspace)
}

/// Save an incoming Discord message to the workspace.
fn append_discord_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_msg: &Message,
    seq: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;

    // Save the raw message as JSON
    let raw_path = incoming_dir.join(format!("{:05}_discord_raw.json", seq));
    let raw_json = serde_json::to_string_pretty(&serde_json::json!({
        "id": raw_msg.id.get(),
        "channel_id": raw_msg.channel_id.get(),
        "guild_id": raw_msg.guild_id.map(|id| id.get()),
        "author": {
            "id": raw_msg.author.id.get(),
            "name": raw_msg.author.name,
            "bot": raw_msg.author.bot,
        },
        "content": raw_msg.content,
        "timestamp": raw_msg.timestamp.to_string(),
        "attachments": raw_msg.attachments.iter().map(|a| serde_json::json!({
            "id": a.id.get(),
            "filename": a.filename,
            "content_type": a.content_type,
            "size": a.size,
            "url": a.url,
        })).collect::<Vec<_>>(),
    }))?;
    fs::write(&raw_path, raw_json)?;

    // Save message text as a simple text file
    let text_path = incoming_dir.join(format!("{:05}_discord_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    fs::write(&text_path, &text_content)?;

    // Create a metadata file with sender info
    let meta_path = incoming_dir.join(format!("{:05}_discord_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "discord",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "guild_id": message.metadata.discord_guild_id,
        "channel_id": message.metadata.discord_channel_id,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved Discord message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

/// Create and start the Discord Gateway client.
///
/// This function creates a serenity Client with the appropriate intents and
/// event handler, then starts the Gateway connection. It should be spawned
/// as a background task in tokio.
pub async fn start_discord_client(
    token: String,
    state: DiscordHandlerState,
    bot_user_id: Option<u64>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set up intents - we need message content to read user messages
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut bot_user_ids = HashSet::new();
    if let Some(id) = bot_user_id {
        bot_user_ids.insert(id);
    }

    let handler = DiscordEventHandler::new(state, bot_user_ids);

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    info!("Starting Discord Gateway client...");
    client.start().await?;

    Ok(())
}
