use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::account_store::lookup_account_by_channel;
use crate::adapters::bluebubbles::send_quick_bluebubbles_response;
use crate::adapters::telegram::send_quick_telegram_response;
use crate::adapters::whatsapp::send_quick_whatsapp_response;
use crate::blob_store::get_blob_store;
use crate::channel::Channel;
use crate::memory_diff::{MemoryDiff, SectionChange};
use crate::memory_queue::{global_memory_queue, MemoryWriteRequest};
use crate::message_router::{MessageRouter, RouterDecision};
use crate::slack_store::SlackStore;
use crate::user_store::UserStore;
use uuid::Uuid;

use super::super::config::ServiceConfig;
use super::super::BoxError;
use super::discord_context::build_discord_router_context;
use super::persist_discord_ingest_context;

const DISCORD_QUICK_RESPONSE_DEDUPE_FILE: &str = "discord_quick_response_dedupe.json";
const DISCORD_QUICK_RESPONSE_MAX_THREADS: usize = 512;
const DISCORD_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD: usize = 256;
const SLACK_QUICK_RESPONSE_DEDUPE_FILE: &str = "slack_quick_response_dedupe.json";
const SLACK_QUICK_RESPONSE_MAX_THREADS: usize = 512;
const SLACK_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD: usize = 256;

#[derive(Debug, Default, Serialize, Deserialize)]
struct SlackQuickResponseDedupeStore {
    #[serde(default)]
    threads: HashMap<String, SlackQuickResponseThreadStore>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SlackQuickResponseThreadStore {
    #[serde(default)]
    message_ids: Vec<String>,
    #[serde(default)]
    updated_at_unix_secs: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DiscordQuickResponseDedupeStore {
    #[serde(default)]
    threads: HashMap<String, DiscordQuickResponseThreadStore>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DiscordQuickResponseThreadStore {
    #[serde(default)]
    message_ids: Vec<String>,
    #[serde(default)]
    updated_at_unix_secs: i64,
}

/// Read memo.md from a user's memory directory (local file)
fn read_user_memo_local(memory_dir: &Path) -> Option<String> {
    let memo_path = memory_dir.join("memo.md");
    std::fs::read_to_string(&memo_path).ok()
}

/// Read memo from Azure Blob Storage (unified account)
fn read_user_memo_blob(runtime: &tokio::runtime::Handle, account_id: Uuid) -> Option<String> {
    let blob_store = get_blob_store()?;
    match runtime.block_on(blob_store.read_memo(account_id)) {
        Ok(content) => Some(content),
        Err(e) => {
            warn!(
                "Failed to read memo from blob for account {}: {}",
                account_id, e
            );
            None
        }
    }
}

/// Read memo - tries blob storage first if account exists, falls back to local
fn read_user_memo(
    runtime: &tokio::runtime::Handle,
    account_id: Option<Uuid>,
    memory_dir: &Path,
) -> Option<String> {
    if let Some(account_id) = account_id {
        if let Some(content) = read_user_memo_blob(runtime, account_id) {
            return Some(content);
        }
    }
    read_user_memo_local(memory_dir)
}

/// Write memory update via queue - uses blob storage if account_id is set
fn write_memory_update(
    account_id: Option<Uuid>,
    user_id: &str,
    memory_dir: &Path,
    update: &str,
) -> Result<(), String> {
    // Create a simple diff that appends to the "Notes" section
    let diff = MemoryDiff {
        changed_sections: std::collections::HashMap::from([(
            "Notes".to_string(),
            SectionChange::Added(vec![update.to_string()]),
        )]),
    };

    let request = MemoryWriteRequest {
        account_id,
        user_id: user_id.to_string(),
        user_memory_dir: memory_dir.to_path_buf(),
        diff,
    };

    global_memory_queue()
        .submit(request)
        .map_err(|e| e.to_string())
}

fn slack_quick_response_dedupe_path(state_dir: &Path) -> std::path::PathBuf {
    state_dir.join(SLACK_QUICK_RESPONSE_DEDUPE_FILE)
}

fn slack_quick_response_scope_key(message: &crate::channel::InboundMessage) -> Option<String> {
    let channel_id = message.metadata.slack_channel_id.as_deref()?;
    let team = message
        .metadata
        .slack_team_id
        .as_deref()
        .unwrap_or("unknown");
    Some(format!(
        "slack:{}:{}:{}",
        team, channel_id, message.thread_id
    ))
}

fn slack_inbound_message_id(message: &crate::channel::InboundMessage) -> Option<&str> {
    message.message_id.as_deref()
}

fn slack_quick_response_already_sent(state_dir: &Path, scope_key: &str, message_id: &str) -> bool {
    let path = slack_quick_response_dedupe_path(state_dir);
    let store = load_slack_quick_response_dedupe_store(&path);
    store
        .threads
        .get(scope_key)
        .map(|thread| thread.message_ids.iter().any(|entry| entry == message_id))
        .unwrap_or(false)
}

fn record_slack_quick_response_sent(
    state_dir: &Path,
    scope_key: &str,
    message_id: &str,
) -> Result<(), BoxError> {
    let path = slack_quick_response_dedupe_path(state_dir);
    let mut store = load_slack_quick_response_dedupe_store(&path);

    let thread = store.threads.entry(scope_key.to_string()).or_default();
    if !thread.message_ids.iter().any(|entry| entry == message_id) {
        thread.message_ids.push(message_id.to_string());
    }
    let overflow = thread
        .message_ids
        .len()
        .saturating_sub(SLACK_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD);
    if overflow > 0 {
        thread.message_ids.drain(0..overflow);
    }
    thread.updated_at_unix_secs = now_unix_secs();

    prune_slack_quick_response_store(&mut store);
    write_slack_quick_response_dedupe_store(&path, &store)?;
    Ok(())
}

fn load_slack_quick_response_dedupe_store(path: &Path) -> SlackQuickResponseDedupeStore {
    let Ok(raw) = std::fs::read(path) else {
        return SlackQuickResponseDedupeStore::default();
    };
    match serde_json::from_slice::<SlackQuickResponseDedupeStore>(&raw) {
        Ok(store) => store,
        Err(err) => {
            warn!(
                "failed to parse slack quick response dedupe store at {}: {}",
                path.display(),
                err
            );
            SlackQuickResponseDedupeStore::default()
        }
    }
}

fn write_slack_quick_response_dedupe_store(
    path: &Path,
    store: &SlackQuickResponseDedupeStore,
) -> Result<(), BoxError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!("tmp-{}", Uuid::new_v4()));
    let serialized = serde_json::to_vec_pretty(store)?;
    std::fs::write(&tmp, serialized)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn prune_slack_quick_response_store(store: &mut SlackQuickResponseDedupeStore) {
    if store.threads.len() <= SLACK_QUICK_RESPONSE_MAX_THREADS {
        return;
    }
    let mut thread_by_age = store
        .threads
        .iter()
        .map(|(key, value)| (key.clone(), value.updated_at_unix_secs))
        .collect::<Vec<_>>();
    thread_by_age.sort_by_key(|(_, updated_at)| *updated_at);
    let overflow = store
        .threads
        .len()
        .saturating_sub(SLACK_QUICK_RESPONSE_MAX_THREADS);
    for (key, _) in thread_by_age.into_iter().take(overflow) {
        store.threads.remove(&key);
    }
}

fn discord_quick_response_dedupe_path(state_dir: &Path) -> std::path::PathBuf {
    state_dir.join(DISCORD_QUICK_RESPONSE_DEDUPE_FILE)
}

fn discord_quick_response_scope_key(message: &crate::channel::InboundMessage) -> Option<String> {
    let channel_id = message.metadata.discord_channel_id?;
    let guild = message
        .metadata
        .discord_guild_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "dm".to_string());
    Some(format!(
        "discord:{}:{}:{}",
        guild, channel_id, message.thread_id
    ))
}

fn discord_inbound_message_id(message: &crate::channel::InboundMessage) -> Option<&str> {
    message
        .message_id
        .as_deref()
        .or(message.metadata.discord_message_id.as_deref())
}

fn discord_quick_response_already_sent(
    state_dir: &Path,
    scope_key: &str,
    message_id: &str,
) -> bool {
    let path = discord_quick_response_dedupe_path(state_dir);
    let store = load_discord_quick_response_dedupe_store(&path);
    store
        .threads
        .get(scope_key)
        .map(|thread| thread.message_ids.iter().any(|entry| entry == message_id))
        .unwrap_or(false)
}

fn record_discord_quick_response_sent(
    state_dir: &Path,
    scope_key: &str,
    message_id: &str,
) -> Result<(), BoxError> {
    let path = discord_quick_response_dedupe_path(state_dir);
    let mut store = load_discord_quick_response_dedupe_store(&path);

    let thread = store.threads.entry(scope_key.to_string()).or_default();
    if !thread.message_ids.iter().any(|entry| entry == message_id) {
        thread.message_ids.push(message_id.to_string());
    }
    let overflow = thread
        .message_ids
        .len()
        .saturating_sub(DISCORD_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD);
    if overflow > 0 {
        thread.message_ids.drain(0..overflow);
    }
    thread.updated_at_unix_secs = now_unix_secs();

    prune_discord_quick_response_store(&mut store);
    write_discord_quick_response_dedupe_store(&path, &store)?;
    Ok(())
}

fn load_discord_quick_response_dedupe_store(path: &Path) -> DiscordQuickResponseDedupeStore {
    let Ok(raw) = std::fs::read(path) else {
        return DiscordQuickResponseDedupeStore::default();
    };
    match serde_json::from_slice::<DiscordQuickResponseDedupeStore>(&raw) {
        Ok(store) => store,
        Err(err) => {
            warn!(
                "failed to parse discord quick response dedupe store at {}: {}",
                path.display(),
                err
            );
            DiscordQuickResponseDedupeStore::default()
        }
    }
}

fn write_discord_quick_response_dedupe_store(
    path: &Path,
    store: &DiscordQuickResponseDedupeStore,
) -> Result<(), BoxError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!("tmp-{}", Uuid::new_v4()));
    let serialized = serde_json::to_vec_pretty(store)?;
    std::fs::write(&tmp, serialized)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

fn prune_discord_quick_response_store(store: &mut DiscordQuickResponseDedupeStore) {
    if store.threads.len() <= DISCORD_QUICK_RESPONSE_MAX_THREADS {
        return;
    }
    let mut thread_by_age = store
        .threads
        .iter()
        .map(|(key, value)| (key.clone(), value.updated_at_unix_secs))
        .collect::<Vec<_>>();
    thread_by_age.sort_by_key(|(_, updated_at)| *updated_at);
    let overflow = store
        .threads
        .len()
        .saturating_sub(DISCORD_QUICK_RESPONSE_MAX_THREADS);
    for (key, _) in thread_by_age.into_iter().take(overflow) {
        store.threads.remove(&key);
    }
}

fn now_unix_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
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

    // Look up unified account first, fall back to legacy user_store
    let account_id = lookup_account_by_channel(&Channel::Slack, &message.sender);
    let user = user_store.get_or_create_user("slack", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(runtime, account_id, &user_paths.memory_dir);
    let dedupe_scope = slack_quick_response_scope_key(message);
    let inbound_message_id = slack_inbound_message_id(message);

    if let (Some(scope), Some(inbound_id)) = (dedupe_scope.as_deref(), inbound_message_id) {
        if slack_quick_response_already_sent(&user_paths.state_dir, scope, inbound_id) {
            info!(
                "slack quick response dedupe hit employee={} sender={} scope={} message_id={}",
                config.employee_profile.id, message.sender, scope, inbound_id
            );
            return Ok(true);
        }
    }

    let cleaned_text = text
        .split_whitespace()
        .filter(|word| !(word.starts_with("<@") && word.ends_with(">")))
        .collect::<Vec<_>>()
        .join(" ");

    let employee_name = config.employee_profile.display_name.as_deref();
    let decision = runtime.block_on(message_router.classify(
        &cleaned_text,
        memory.as_deref(),
        employee_name,
        None,
    ));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present (to blob storage if account linked)
            if let Some(update) = memory_update {
                if let Err(e) =
                    write_memory_update(account_id, &user.user_id, &user_paths.memory_dir, &update)
                {
                    warn!("Failed to write memory update: {}", e);
                } else if let Some(aid) = account_id {
                    info!("Updated memory for unified account {}", aid);
                } else {
                    info!("Updated memory for legacy user {}", user.user_id);
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
                        &token, channel_id, thread_ts, &response,
                    ))
                    .is_ok()
                {
                    if let (Some(scope), Some(inbound_id)) =
                        (dedupe_scope.as_deref(), inbound_message_id)
                    {
                        if let Err(err) = record_slack_quick_response_sent(
                            &user_paths.state_dir,
                            scope,
                            inbound_id,
                        ) {
                            warn!(
                                "failed to record slack quick response dedupe key scope={} message_id={}: {}",
                                scope, inbound_id, err
                            );
                        }
                    }
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
    // First, try per-employee token (e.g., LITTLE_BEAR_SLACK_BOT_TOKEN)
    let emp_upper = config.employee_profile.id.to_uppercase().replace('-', "_");
    let emp_token_key = format!("{}_SLACK_BOT_TOKEN", emp_upper);
    if let Ok(token) = std::env::var(&emp_token_key) {
        if !token.trim().is_empty() {
            info!(
                "quick response using {} for employee {}",
                emp_token_key, config.employee_profile.id
            );
            return Some(token);
        }
    }

    // Then try slack_store by team_id
    if let Some(team_id) = team_id {
        if let Ok(installation) = slack_store.get_installation_or_env(team_id) {
            if !installation.bot_token.trim().is_empty() {
                return Some(installation.bot_token);
            }
        }
    }

    // Fall back to global SLACK_BOT_TOKEN
    config.slack_bot_token.clone()
}

fn resolve_discord_bot_token(config: &ServiceConfig) -> Option<String> {
    // First, try per-employee token (e.g., LITTLE_BEAR_DISCORD_BOT_TOKEN)
    let emp_upper = config.employee_profile.id.to_uppercase().replace('-', "_");
    let emp_token_key = format!("{}_DISCORD_BOT_TOKEN", emp_upper);
    if let Ok(token) = std::env::var(&emp_token_key) {
        if !token.trim().is_empty() {
            info!(
                "quick response using {} for employee {}",
                emp_token_key, config.employee_profile.id
            );
            return Some(token);
        }
    }

    // Fall back to global DISCORD_BOT_TOKEN or config
    config.discord_bot_token.clone()
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

    // Normalize phone number (strip + prefix) for lookup
    let normalized_phone = message.sender.trim_start_matches('+');

    // Look up unified account first, fall back to legacy user_store
    let account_id = lookup_account_by_channel(&Channel::BlueBubbles, normalized_phone);
    let user = user_store.get_or_create_user("phone", normalized_phone)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(runtime, account_id, &user_paths.memory_dir);

    let employee_name = config.employee_profile.display_name.as_deref();
    let decision =
        runtime.block_on(message_router.classify(text, memory.as_deref(), employee_name, None));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present (to blob storage if account linked)
            if let Some(update) = memory_update {
                if let Err(e) =
                    write_memory_update(account_id, &user.user_id, &user_paths.memory_dir, &update)
                {
                    warn!("Failed to write memory update: {}", e);
                } else if let Some(aid) = account_id {
                    info!("Updated memory for unified account {}", aid);
                } else {
                    info!("Updated memory for legacy user {}", user.user_id);
                }
            }

            if runtime
                .block_on(send_quick_bluebubbles_response(
                    url, password, chat_guid, &response,
                ))
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
    raw_payload: &[u8],
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let channel_id = match message.metadata.discord_channel_id {
        Some(value) => value,
        None => return Ok(false),
    };
    let message_id = message.message_id.as_deref();
    let Some(token) = resolve_discord_bot_token(config) else {
        return Ok(false);
    };

    // Look up unified account first, fall back to legacy user_store
    let account_id = lookup_account_by_channel(&Channel::Discord, &message.sender);
    let user = user_store.get_or_create_user("discord", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(runtime, account_id, &user_paths.memory_dir);
    let dedupe_scope = discord_quick_response_scope_key(message);
    let inbound_message_id = discord_inbound_message_id(message);

    if let (Some(scope), Some(inbound_id)) = (dedupe_scope.as_deref(), inbound_message_id) {
        if discord_quick_response_already_sent(&user_paths.state_dir, scope, inbound_id) {
            info!(
                "discord quick response dedupe hit employee={} sender={} scope={} message_id={}",
                config.employee_profile.id, message.sender, scope, inbound_id
            );
            return Ok(true);
        }
    }

    let router_context = match build_discord_router_context(config, message, raw_payload) {
        Ok(context) => Some(context),
        Err(err) => {
            warn!("Failed to build Discord router context: {}", err);
            None
        }
    };
    let router_message = router_context
        .as_ref()
        .map(|context| context.message.as_str())
        .unwrap_or(text);
    let extra_context = router_context
        .as_ref()
        .map(|context| context.context.as_str());

    let employee_name = config.employee_profile.display_name.as_deref();
    let decision = runtime.block_on(message_router.classify(
        router_message,
        memory.as_deref(),
        employee_name,
        extra_context,
    ));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present (to blob storage if account linked)
            if let Some(update) = memory_update {
                if let Err(e) =
                    write_memory_update(account_id, &user.user_id, &user_paths.memory_dir, &update)
                {
                    warn!("Failed to write memory update: {}", e);
                } else if let Some(aid) = account_id {
                    info!("Updated memory for unified account {}", aid);
                } else {
                    info!("Updated memory for legacy user {}", user.user_id);
                }
            }

            let sent =
                send_quick_discord_response_simple(&token, channel_id, message_id, &response)
                    .is_ok();
            if sent {
                if let (Some(scope), Some(inbound_id)) =
                    (dedupe_scope.as_deref(), inbound_message_id)
                {
                    if let Err(err) =
                        record_discord_quick_response_sent(&user_paths.state_dir, scope, inbound_id)
                    {
                        warn!(
                            "failed to record discord quick response dedupe key scope={} message_id={}: {}",
                            scope, inbound_id, err
                        );
                    }
                }
                if let Err(err) = persist_discord_ingest_context(
                    config,
                    user_store,
                    message,
                    raw_payload,
                    router_context.as_ref().map(|context| &context.snapshot),
                ) {
                    warn!(
                        "Failed to persist Discord context after quick reply: {}",
                        err
                    );
                }
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

    // Look up unified account first, fall back to legacy user_store
    let account_id = lookup_account_by_channel(&Channel::Telegram, &message.sender);
    let user = user_store.get_or_create_user("telegram", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(runtime, account_id, &user_paths.memory_dir);

    let employee_name = config.employee_profile.display_name.as_deref();
    let decision =
        runtime.block_on(message_router.classify(text, memory.as_deref(), employee_name, None));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present (to blob storage if account linked)
            if let Some(update) = memory_update {
                if let Err(e) =
                    write_memory_update(account_id, &user.user_id, &user_paths.memory_dir, &update)
                {
                    warn!("Failed to write memory update: {}", e);
                } else if let Some(aid) = account_id {
                    info!("Updated memory for unified account {}", aid);
                } else {
                    info!("Updated memory for legacy user {}", user.user_id);
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
    let api_base =
        std::env::var("SLACK_API_BASE_URL").unwrap_or_else(|_| "https://slack.com/api".to_string());
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
        return Err(result
            .error
            .unwrap_or_else(|| "discord send failed".to_string())
            .into());
    }
    Ok(())
}

pub(crate) fn try_quick_response_whatsapp(
    config: &ServiceConfig,
    user_store: &UserStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let Some(phone_number) = message.metadata.whatsapp_phone_number.as_deref() else {
        return Ok(false);
    };
    let Some(access_token) = config.whatsapp_access_token.as_deref() else {
        return Ok(false);
    };
    let Some(phone_number_id) = config.whatsapp_phone_number_id.as_deref() else {
        return Ok(false);
    };

    // Normalize phone number (strip + prefix) for lookup
    let normalized_phone = message.sender.trim_start_matches('+');

    // Look up unified account first, fall back to legacy user_store
    let account_id = lookup_account_by_channel(&Channel::WhatsApp, normalized_phone);
    let user = user_store.get_or_create_user("whatsapp", normalized_phone)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(runtime, account_id, &user_paths.memory_dir);

    let employee_name = config.employee_profile.display_name.as_deref();
    let decision =
        runtime.block_on(message_router.classify(text, memory.as_deref(), employee_name, None));
    match decision {
        RouterDecision::Simple {
            response,
            memory_update,
        } => {
            // Write memory update if present (to blob storage if account linked)
            if let Some(update) = memory_update {
                if let Err(e) =
                    write_memory_update(account_id, &user.user_id, &user_paths.memory_dir, &update)
                {
                    warn!("Failed to write memory update: {}", e);
                } else if let Some(aid) = account_id {
                    info!("Updated memory for unified account {}", aid);
                } else {
                    info!("Updated memory for legacy user {}", user.user_id);
                }
            }

            if runtime
                .block_on(send_quick_whatsapp_response(
                    access_token,
                    phone_number_id,
                    phone_number,
                    &response,
                ))
                .is_ok()
            {
                return Ok(true);
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{Channel, ChannelMetadata, InboundMessage};
    use tempfile::TempDir;

    fn build_slack_message(
        thread_id: &str,
        message_id: Option<&str>,
        team_id: Option<&str>,
        channel_id: Option<&str>,
    ) -> InboundMessage {
        InboundMessage {
            channel: Channel::Slack,
            sender: "U1234".to_string(),
            sender_name: Some("Bingran".to_string()),
            recipient: "C1234".to_string(),
            subject: None,
            text_body: Some("Hi".to_string()),
            html_body: None,
            thread_id: thread_id.to_string(),
            message_id: message_id.map(str::to_string),
            attachments: Vec::new(),
            reply_to: Vec::new(),
            raw_payload: Vec::new(),
            metadata: ChannelMetadata {
                slack_team_id: team_id.map(str::to_string),
                slack_channel_id: channel_id.map(str::to_string),
                ..Default::default()
            },
        }
    }

    fn build_discord_message(
        thread_id: &str,
        message_id: Option<&str>,
        metadata_message_id: Option<&str>,
        guild_id: Option<u64>,
        channel_id: Option<u64>,
    ) -> InboundMessage {
        InboundMessage {
            channel: Channel::Discord,
            sender: "1234".to_string(),
            sender_name: Some("Bingran".to_string()),
            recipient: "oliver-bot".to_string(),
            subject: None,
            text_body: Some("Hi".to_string()),
            html_body: None,
            thread_id: thread_id.to_string(),
            message_id: message_id.map(str::to_string),
            attachments: Vec::new(),
            reply_to: Vec::new(),
            raw_payload: Vec::new(),
            metadata: ChannelMetadata {
                discord_guild_id: guild_id,
                discord_channel_id: channel_id,
                discord_message_id: metadata_message_id.map(str::to_string),
                ..Default::default()
            },
        }
    }

    #[test]
    fn discord_quick_response_dedupe_records_and_matches() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let state_dir = temp.path().join("state");
        let scope = "discord:guild-1:42:thread-1";
        let message_id = "msg-1";

        assert!(!discord_quick_response_already_sent(
            &state_dir, scope, message_id
        ));

        record_discord_quick_response_sent(&state_dir, scope, message_id)?;

        assert!(discord_quick_response_already_sent(
            &state_dir, scope, message_id
        ));

        let path = discord_quick_response_dedupe_path(&state_dir);
        let store = load_discord_quick_response_dedupe_store(&path);
        let entries = &store.threads.get(scope).expect("thread entry").message_ids;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], "msg-1");

        Ok(())
    }

    #[test]
    fn discord_quick_response_dedupe_prunes_old_message_ids() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let state_dir = temp.path().join("state");
        let scope = "discord:guild-1:42:thread-1";

        for index in 0..(DISCORD_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD + 5) {
            let message_id = format!("msg-{}", index);
            record_discord_quick_response_sent(&state_dir, scope, &message_id)?;
        }

        let path = discord_quick_response_dedupe_path(&state_dir);
        let store = load_discord_quick_response_dedupe_store(&path);
        let entries = &store.threads.get(scope).expect("thread entry").message_ids;

        assert_eq!(
            entries.len(),
            DISCORD_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD
        );
        assert!(!entries.iter().any(|entry| entry == "msg-0"));
        assert!(entries.iter().any(|entry| entry
            == &format!(
                "msg-{}",
                DISCORD_QUICK_RESPONSE_MAX_MESSAGE_IDS_PER_THREAD + 4
            )));

        Ok(())
    }

    #[test]
    fn discord_scope_key_and_message_id_fallback_work() {
        let message = build_discord_message("thread-1", None, Some("meta-msg-id"), None, Some(42));

        let scope = discord_quick_response_scope_key(&message).expect("scope key");
        assert_eq!(scope, "discord:dm:42:thread-1");

        let message_id = discord_inbound_message_id(&message).expect("message id");
        assert_eq!(message_id, "meta-msg-id");
    }

    #[test]
    fn slack_quick_response_dedupe_records_and_matches() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let state_dir = temp.path().join("state");
        let scope = "slack:T123:C123:thread-1";
        let message_id = "1712345678.000100";

        assert!(!slack_quick_response_already_sent(
            &state_dir, scope, message_id
        ));

        record_slack_quick_response_sent(&state_dir, scope, message_id)?;

        assert!(slack_quick_response_already_sent(
            &state_dir, scope, message_id
        ));

        Ok(())
    }

    #[test]
    fn slack_scope_key_uses_team_channel_thread() {
        let message = build_slack_message(
            "1712345678.000100",
            Some("1712345678.000100"),
            Some("T123"),
            Some("C123"),
        );

        let scope = slack_quick_response_scope_key(&message).expect("scope key");
        assert_eq!(scope, "slack:T123:C123:1712345678.000100");

        let message_id = slack_inbound_message_id(&message).expect("message id");
        assert_eq!(message_id, "1712345678.000100");
    }
}
