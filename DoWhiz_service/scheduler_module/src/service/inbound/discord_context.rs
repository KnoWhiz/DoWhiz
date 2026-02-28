use std::collections::{HashMap, HashSet};
use std::path::Path;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::super::config::ServiceConfig;
use super::super::BoxError;

const HISTORY_WINDOW_HOURS: i64 = 24;
const MAX_HISTORY_PAGES: usize = 20;
const INLINE_THREAD_CHAR_LIMIT: usize = 6000;
const INLINE_THREAD_RECENT_COUNT: usize = 8;
const INLINE_CHANNEL_CHAR_LIMIT: usize = 4000;
const INLINE_CHANNEL_RECENT_COUNT: usize = 12;
const ROUTER_CONTEXT_CHAR_LIMIT: usize = 4000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DiscordMessageEntry {
    id: String,
    timestamp: String,
    author_id: String,
    author_name: String,
    content: String,
    #[serde(default)]
    reference_message_id: Option<String>,
    #[serde(default)]
    attachments: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct DiscordContextSnapshot {
    generated_at: DateTime<Utc>,
    channel_id: u64,
    guild_id: Option<u64>,
    current_message_id: String,
    channel_messages: Vec<DiscordMessageEntry>,
    thread_messages: Vec<DiscordMessageEntry>,
    inline_thread_messages: Vec<DiscordMessageEntry>,
    inline_channel_messages: Vec<DiscordMessageEntry>,
    quoted_message: Option<DiscordMessageEntry>,
    thread_truncated: bool,
    channel_truncated: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct DiscordRouterContext {
    pub message: String,
    pub context: String,
    pub snapshot: DiscordContextSnapshot,
}

#[derive(Debug, Deserialize)]
struct DiscordRawPayloadLite {
    id: u64,
    author_id: u64,
    author_name: String,
    content: String,
    timestamp: String,
    #[serde(default)]
    referenced_message_id: Option<u64>,
    #[serde(default)]
    referenced_message_author_id: Option<u64>,
    #[serde(default)]
    referenced_message_author_name: Option<String>,
    #[serde(default)]
    referenced_message_content: Option<String>,
}

pub(crate) fn hydrate_discord_context_files(
    config: &ServiceConfig,
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    let snapshot = collect_discord_context_snapshot(config, message, raw_payload)?;
    write_discord_context_files(&incoming_dir, seq, &snapshot)?;

    info!(
        "discord context files updated for channel {} at {}",
        snapshot.channel_id,
        incoming_dir.display()
    );

    Ok(())
}

pub(crate) fn hydrate_discord_context_files_from_snapshot(
    workspace: &Path,
    seq: u64,
    snapshot: &DiscordContextSnapshot,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;
    write_discord_context_files(&incoming_dir, seq, snapshot)?;
    info!(
        "discord context files updated for channel {} at {}",
        snapshot.channel_id,
        incoming_dir.display()
    );
    Ok(())
}

pub(crate) fn build_discord_router_context(
    config: &ServiceConfig,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<DiscordRouterContext, BoxError> {
    let snapshot = collect_discord_context_snapshot(config, message, raw_payload)?;
    let user_text = message.text_body.clone().unwrap_or_default();
    let message_text = format_message_with_quote(&user_text, snapshot.quoted_message.as_ref());

    let inline_thread = render_entries(&snapshot.inline_thread_messages);
    let inline_channel = render_entries(&snapshot.inline_channel_messages);
    let thread_block = if inline_thread.trim().is_empty() {
        "- (none)".to_string()
    } else {
        inline_thread
    };
    let channel_block = if inline_channel.trim().is_empty() {
        "- (none)".to_string()
    } else {
        inline_channel
    };

    let mut context = format!(
        "Thread window (recent replies in this thread):\n{thread}\n\nChannel window (recent in last {hours}h):\n{channel}",
        thread = thread_block,
        hours = HISTORY_WINDOW_HOURS,
        channel = channel_block
    );
    if context.chars().count() > ROUTER_CONTEXT_CHAR_LIMIT {
        context = context
            .chars()
            .take(ROUTER_CONTEXT_CHAR_LIMIT)
            .collect::<String>();
        context.push_str("\n\n(Truncated.)");
    }

    Ok(DiscordRouterContext {
        message: message_text,
        context,
        snapshot,
    })
}

pub(crate) fn build_discord_message_text_with_quote(
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> String {
    let user_text = message.text_body.clone().unwrap_or_default();
    let quoted = build_quoted_message_from_raw(raw_payload);
    format_message_with_quote(&user_text, quoted.as_ref())
}

fn collect_discord_context_snapshot(
    config: &ServiceConfig,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<DiscordContextSnapshot, BoxError> {
    let channel_id = message
        .metadata
        .discord_channel_id
        .ok_or("missing discord_channel_id")?;
    let guild_id = message.metadata.discord_guild_id;

    let now = Utc::now();
    let cutoff = now - Duration::hours(HISTORY_WINDOW_HOURS);

    let mut channel_messages = Vec::new();
    if let Some(bot_token) = resolve_discord_bot_token(config) {
        match fetch_channel_messages_last_day(&bot_token, channel_id, cutoff) {
            Ok(messages) => {
                channel_messages = messages;
            }
            Err(err) => {
                warn!(
                    "failed to fetch discord channel history for channel {}: {}",
                    channel_id, err
                );
            }
        }
    }

    let current_message = build_current_message_entry(message, raw_payload, now);
    upsert_by_id(&mut channel_messages, current_message.clone());

    let mut quoted_message = build_quoted_message_from_raw(raw_payload);
    let referenced_id = message
        .metadata
        .discord_referenced_message_id
        .clone()
        .or_else(|| current_message.reference_message_id.clone());

    if quoted_message.is_none() {
        if let Some(ref_id) = referenced_id.as_deref() {
            quoted_message = channel_messages.iter().find(|m| m.id == ref_id).cloned();
        }
    }

    if quoted_message.is_none() {
        if let (Some(bot_token), Some(ref_id)) =
            (resolve_discord_bot_token(config), referenced_id.as_deref())
        {
            match fetch_single_message(&bot_token, channel_id, ref_id) {
                Ok(found) => {
                    upsert_by_id(&mut channel_messages, found.clone());
                    quoted_message = Some(found);
                }
                Err(err) => {
                    warn!(
                        "failed to fetch referenced discord message {} in channel {}: {}",
                        ref_id, channel_id, err
                    );
                }
            }
        }
    }

    channel_messages.sort_by_key(|entry| parse_timestamp(&entry.timestamp));
    channel_messages.dedup_by(|left, right| left.id == right.id);

    let current_message_id = message
        .message_id
        .clone()
        .or_else(|| message.metadata.discord_message_id.clone())
        .unwrap_or_else(|| current_message.id.clone());
    let thread_messages = collect_thread_messages(&channel_messages, &current_message_id);

    let inline_thread_messages =
        if render_entries(&thread_messages).len() <= INLINE_THREAD_CHAR_LIMIT {
            thread_messages.clone()
        } else {
            let keep = INLINE_THREAD_RECENT_COUNT.min(thread_messages.len());
            thread_messages[thread_messages.len().saturating_sub(keep)..].to_vec()
        };
    let thread_truncated = inline_thread_messages.len() < thread_messages.len();

    let mut inline_channel_messages = if channel_messages.len() <= INLINE_CHANNEL_RECENT_COUNT {
        channel_messages.clone()
    } else {
        channel_messages[channel_messages
            .len()
            .saturating_sub(INLINE_CHANNEL_RECENT_COUNT)..]
            .to_vec()
    };
    inline_channel_messages.sort_by_key(|entry| parse_timestamp(&entry.timestamp));
    let mut inline_channel_text = render_entries(&inline_channel_messages);
    while inline_channel_text.chars().count() > INLINE_CHANNEL_CHAR_LIMIT
        && inline_channel_messages.len() > 1
    {
        inline_channel_messages.remove(0);
        inline_channel_text = render_entries(&inline_channel_messages);
    }
    let channel_truncated = inline_channel_messages.len() < channel_messages.len();

    Ok(DiscordContextSnapshot {
        generated_at: now,
        channel_id,
        guild_id,
        current_message_id,
        channel_messages,
        thread_messages,
        inline_thread_messages,
        inline_channel_messages,
        quoted_message,
        thread_truncated,
        channel_truncated,
    })
}

fn write_discord_context_files(
    incoming_dir: &Path,
    seq: u64,
    snapshot: &DiscordContextSnapshot,
) -> Result<(), BoxError> {
    let channel_history_path = incoming_dir.join("discord_channel_last_24h.json");
    let thread_full_path = incoming_dir.join("discord_thread_context_full.json");
    let thread_recent_path = incoming_dir.join("discord_thread_context_recent.txt");
    let channel_recent_path = incoming_dir.join("discord_channel_context_recent.txt");
    let context_md_path = incoming_dir.join("discord_context_for_agent.md");
    let seq_context_md_path = incoming_dir.join(format!("{:05}_discord_context_for_agent.md", seq));
    let seq_recent_path =
        incoming_dir.join(format!("{:05}_discord_thread_context_recent.txt", seq));
    let seq_channel_recent_path =
        incoming_dir.join(format!("{:05}_discord_channel_context_recent.txt", seq));

    let channel_payload = serde_json::json!({
        "generated_at": snapshot.generated_at.to_rfc3339(),
        "window_hours": HISTORY_WINDOW_HOURS,
        "guild_id": snapshot.guild_id,
        "channel_id": snapshot.channel_id,
        "message_count": snapshot.channel_messages.len(),
        "messages": &snapshot.channel_messages,
    });
    std::fs::write(
        &channel_history_path,
        serde_json::to_string_pretty(&channel_payload)?,
    )?;

    let thread_payload = serde_json::json!({
        "generated_at": snapshot.generated_at.to_rfc3339(),
        "current_message_id": &snapshot.current_message_id,
        "message_count": snapshot.thread_messages.len(),
        "messages": &snapshot.thread_messages,
    });
    std::fs::write(
        &thread_full_path,
        serde_json::to_string_pretty(&thread_payload)?,
    )?;

    if let Some(quoted) = &snapshot.quoted_message {
        let quoted_path = incoming_dir.join("discord_quoted_message.json");
        let quoted_payload = serde_json::json!({
            "generated_at": snapshot.generated_at.to_rfc3339(),
            "message": quoted,
        });
        std::fs::write(quoted_path, serde_json::to_string_pretty(&quoted_payload)?)?;
    }

    let thread_recent_text = render_entries(&snapshot.inline_thread_messages);
    let channel_recent_text = render_entries(&snapshot.inline_channel_messages);
    std::fs::write(&thread_recent_path, &thread_recent_text)?;
    std::fs::write(&seq_recent_path, &thread_recent_text)?;
    std::fs::write(&channel_recent_path, &channel_recent_text)?;
    std::fs::write(&seq_channel_recent_path, &channel_recent_text)?;

    let quoted_block = if let Some(quoted) = snapshot.quoted_message.as_ref() {
        format!(
            "## Quoted Message (Must Include)\n{}\n\n",
            render_single_entry(quoted)
        )
    } else {
        "## Quoted Message (Must Include)\n- (none)\n\n".to_string()
    };

    let thread_note = if snapshot.thread_truncated {
        format!(
            "Thread context is large; inline section keeps the most recent {} messages. Older thread messages are stored in `incoming_email/discord_thread_context_full.json`.\n\n",
            INLINE_THREAD_RECENT_COUNT
        )
    } else {
        String::new()
    };

    let channel_note = if snapshot.channel_truncated {
        format!(
            "Channel context is large; inline section keeps the most recent {} messages. Older channel messages are stored in `incoming_email/discord_channel_last_24h.json`.\n\n",
            INLINE_CHANNEL_RECENT_COUNT
        )
    } else {
        String::new()
    };

    let context_markdown = format!(
        "# Discord Context Snapshot\n\
Generated at: {generated_at}\n\n\
Current message ID: `{current_id}`\n\
Channel ID: `{channel_id}`\n\
Guild ID: `{guild_id}`\n\n\
{quoted_block}\
## Thread Context (Inline)\n\
{inline_thread}\n\n\
{thread_note}\
## Channel Context (Inline, last 24h window)\n\
{inline_channel}\n\n\
{channel_note}\
## Local Context Files\n\
- `incoming_email/discord_channel_last_24h.json`: full channel history from the last 24 hours.\n\
- `incoming_email/discord_channel_context_recent.txt`: recent channel context used inline.\n\
- `incoming_email/discord_thread_context_full.json`: full thread context.\n\
- `incoming_email/discord_thread_context_recent.txt`: recent thread context used inline.\n\
- `incoming_email/discord_quoted_message.json`: quoted message payload when available.\n",
        generated_at = snapshot.generated_at.to_rfc3339(),
        current_id = snapshot.current_message_id,
        channel_id = snapshot.channel_id,
        guild_id = snapshot
            .guild_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "dm".to_string()),
        quoted_block = quoted_block,
        inline_thread = if thread_recent_text.trim().is_empty() {
            "- (none)".to_string()
        } else {
            thread_recent_text
        },
        thread_note = thread_note,
        inline_channel = if channel_recent_text.trim().is_empty() {
            "- (none)".to_string()
        } else {
            channel_recent_text
        },
        channel_note = channel_note,
    );
    std::fs::write(&context_md_path, &context_markdown)?;
    std::fs::write(&seq_context_md_path, &context_markdown)?;

    Ok(())
}

fn format_message_with_quote(user_text: &str, quoted: Option<&DiscordMessageEntry>) -> String {
    match quoted {
        Some(entry) => {
            if user_text.trim().is_empty() {
                format!(
                    "Quoted message:\n{}\n\nUser message:\n- (empty)",
                    render_single_entry(entry)
                )
            } else {
                format!(
                    "Quoted message:\n{}\n\nUser message:\n{}",
                    render_single_entry(entry),
                    user_text.trim_end()
                )
            }
        }
        None => user_text.to_string(),
    }
}

fn resolve_discord_bot_token(config: &ServiceConfig) -> Option<String> {
    let emp_upper = config.employee_profile.id.to_uppercase().replace('-', "_");
    let emp_token_key = format!("{}_DISCORD_BOT_TOKEN", emp_upper);
    if let Ok(token) = std::env::var(&emp_token_key) {
        if !token.trim().is_empty() {
            return Some(token);
        }
    }
    config.discord_bot_token.clone()
}

fn fetch_channel_messages_last_day(
    bot_token: &str,
    channel_id: u64,
    cutoff: DateTime<Utc>,
) -> Result<Vec<DiscordMessageEntry>, BoxError> {
    let mut all_messages = Vec::new();
    let mut before: Option<String> = None;
    let api_base = std::env::var("DISCORD_API_BASE_URL")
        .unwrap_or_else(|_| "https://discord.com/api/v10".to_string());
    let client = reqwest::blocking::Client::new();

    for _ in 0..MAX_HISTORY_PAGES {
        let mut url = format!(
            "{}/channels/{}/messages?limit=100",
            api_base.trim_end_matches('/'),
            channel_id
        );
        if let Some(cursor) = before.as_deref() {
            url.push_str("&before=");
            url.push_str(cursor);
        }

        let response = client
            .get(&url)
            .header("Authorization", format!("Bot {}", bot_token))
            .send()?;
        if !response.status().is_success() {
            return Err(format!("discord history api returned {}", response.status()).into());
        }

        let payload: serde_json::Value = response.json()?;
        let Some(list) = payload.as_array() else {
            return Err("discord history api returned invalid payload".into());
        };
        if list.is_empty() {
            break;
        }

        for item in list {
            if let Some(entry) = parse_api_message(item) {
                if parse_timestamp(&entry.timestamp)
                    .map(|ts| ts >= cutoff)
                    .unwrap_or(false)
                {
                    all_messages.push(entry);
                }
            }
        }

        before = list
            .last()
            .and_then(|value| value.get("id"))
            .and_then(|value| value.as_str())
            .map(|value| value.to_string());

        let oldest_ts = list
            .last()
            .and_then(|value| value.get("timestamp"))
            .and_then(|value| value.as_str())
            .and_then(parse_timestamp);
        if oldest_ts.map(|ts| ts < cutoff).unwrap_or(false) {
            break;
        }
    }

    Ok(all_messages)
}

fn fetch_single_message(
    bot_token: &str,
    channel_id: u64,
    message_id: &str,
) -> Result<DiscordMessageEntry, BoxError> {
    let api_base = std::env::var("DISCORD_API_BASE_URL")
        .unwrap_or_else(|_| "https://discord.com/api/v10".to_string());
    let url = format!(
        "{}/channels/{}/messages/{}",
        api_base.trim_end_matches('/'),
        channel_id,
        message_id
    );
    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("Authorization", format!("Bot {}", bot_token))
        .send()?;
    if !response.status().is_success() {
        return Err(format!("discord message api returned {}", response.status()).into());
    }
    let payload: serde_json::Value = response.json()?;
    parse_api_message(&payload).ok_or_else(|| "invalid discord message payload".into())
}

fn parse_api_message(value: &serde_json::Value) -> Option<DiscordMessageEntry> {
    let id = value.get("id")?.as_str()?.to_string();
    let timestamp = value.get("timestamp")?.as_str()?.to_string();
    let author = value.get("author")?;
    let author_id = author
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let author_name = author
        .get("global_name")
        .or_else(|| author.get("username"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let content = value
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let reference_message_id = value
        .get("message_reference")
        .and_then(|mr| mr.get("message_id"))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
        .or_else(|| {
            value
                .get("referenced_message")
                .and_then(|msg| msg.get("id"))
                .and_then(|v| v.as_str())
                .map(|v| v.to_string())
        });
    let attachments = value
        .get("attachments")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.get("filename").and_then(|v| v.as_str()))
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Some(DiscordMessageEntry {
        id,
        timestamp,
        author_id,
        author_name,
        content,
        reference_message_id,
        attachments,
    })
}

fn build_current_message_entry(
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    now: DateTime<Utc>,
) -> DiscordMessageEntry {
    if let Ok(payload) = serde_json::from_slice::<DiscordRawPayloadLite>(raw_payload) {
        return DiscordMessageEntry {
            id: payload.id.to_string(),
            timestamp: payload.timestamp,
            author_id: payload.author_id.to_string(),
            author_name: payload.author_name,
            content: payload.content,
            reference_message_id: payload.referenced_message_id.map(|id| id.to_string()),
            attachments: Vec::new(),
        };
    }

    DiscordMessageEntry {
        id: message
            .message_id
            .clone()
            .or_else(|| message.metadata.discord_message_id.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        timestamp: now.to_rfc3339(),
        author_id: message.sender.clone(),
        author_name: message
            .sender_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string()),
        content: message.text_body.clone().unwrap_or_default(),
        reference_message_id: message.metadata.discord_referenced_message_id.clone(),
        attachments: Vec::new(),
    }
}

fn build_quoted_message_from_raw(raw_payload: &[u8]) -> Option<DiscordMessageEntry> {
    let payload = serde_json::from_slice::<DiscordRawPayloadLite>(raw_payload).ok()?;
    let ref_id = payload.referenced_message_id?;
    Some(DiscordMessageEntry {
        id: ref_id.to_string(),
        timestamp: payload.timestamp,
        author_id: payload
            .referenced_message_author_id
            .map(|id| id.to_string())
            .unwrap_or_default(),
        author_name: payload
            .referenced_message_author_name
            .unwrap_or_else(|| "unknown".to_string()),
        content: payload.referenced_message_content.unwrap_or_default(),
        reference_message_id: None,
        attachments: Vec::new(),
    })
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|ts| ts.with_timezone(&Utc))
}

fn upsert_by_id(messages: &mut Vec<DiscordMessageEntry>, candidate: DiscordMessageEntry) {
    if let Some(existing) = messages.iter_mut().find(|entry| entry.id == candidate.id) {
        if existing.content.trim().is_empty() && !candidate.content.trim().is_empty() {
            *existing = candidate;
        }
        return;
    }
    messages.push(candidate);
}

fn collect_thread_messages(
    messages: &[DiscordMessageEntry],
    current_message_id: &str,
) -> Vec<DiscordMessageEntry> {
    let mut map = HashMap::new();
    for message in messages {
        map.insert(message.id.clone(), message.clone());
    }
    if !map.contains_key(current_message_id) {
        return Vec::new();
    }

    let mut root_cache = HashMap::new();
    let thread_root = resolve_root_id(current_message_id, &map, &mut root_cache);
    let mut thread_messages = Vec::new();
    for message in messages {
        let root = resolve_root_id(&message.id, &map, &mut root_cache);
        if root == thread_root {
            thread_messages.push(message.clone());
        }
    }

    thread_messages.sort_by_key(|entry| parse_timestamp(&entry.timestamp));
    thread_messages
}

fn resolve_root_id(
    message_id: &str,
    map: &HashMap<String, DiscordMessageEntry>,
    cache: &mut HashMap<String, String>,
) -> String {
    if let Some(cached) = cache.get(message_id) {
        return cached.clone();
    }

    let mut seen = HashSet::new();
    let mut path = Vec::new();
    let mut current = message_id.to_string();
    loop {
        if !seen.insert(current.clone()) {
            break;
        }
        path.push(current.clone());
        let parent = map
            .get(&current)
            .and_then(|message| message.reference_message_id.clone());
        match parent {
            Some(parent_id) if map.contains_key(&parent_id) => {
                current = parent_id;
            }
            Some(parent_id) => {
                current = parent_id;
                break;
            }
            None => break,
        }
    }

    for id in path {
        cache.insert(id, current.clone());
    }
    current
}

fn render_entries(messages: &[DiscordMessageEntry]) -> String {
    messages
        .iter()
        .map(render_single_entry)
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_single_entry(message: &DiscordMessageEntry) -> String {
    let timestamp = parse_timestamp(&message.timestamp)
        .map(|ts| ts.to_rfc3339())
        .unwrap_or_else(|| message.timestamp.clone());
    let mut content = message.content.replace('\n', " ");
    if content.trim().is_empty() && !message.attachments.is_empty() {
        content = format!("[attachments: {}]", message.attachments.join(", "));
    }
    if content.chars().count() > 240 {
        content = content.chars().take(240).collect::<String>() + "...";
    }
    if let Some(parent) = message.reference_message_id.as_deref() {
        format!(
            "- [{}] {} ({}), reply_to={}: {}",
            timestamp, message.author_name, message.author_id, parent, content
        )
    } else {
        format!(
            "- [{}] {} ({}): {}",
            timestamp, message.author_name, message.author_id, content
        )
    }
}
