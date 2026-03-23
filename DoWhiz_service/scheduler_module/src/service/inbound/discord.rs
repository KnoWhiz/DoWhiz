use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::account_store::AccountStore;
use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::user_store::UserStore;
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::scheduler::cancel_pending_thread_tasks;
use super::super::workspace::ensure_thread_workspace;
use super::super::BoxError;
use super::discord_context::{
    build_discord_message_text_with_quote, hydrate_discord_context_files,
    hydrate_discord_context_files_from_snapshot, DiscordContextSnapshot,
};

#[derive(Debug, serde::Deserialize)]
struct DiscordAttachmentDownloadPayload {
    filename: String,
    url: String,
    #[serde(default)]
    proxy_url: String,
}

#[derive(Debug, serde::Deserialize)]
struct DiscordRawPayloadAttachments {
    #[serde(default)]
    attachments: Vec<DiscordAttachmentDownloadPayload>,
}

pub(crate) fn persist_discord_ingest_context(
    config: &ServiceConfig,
    user_store: &UserStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    snapshot: Option<&DiscordContextSnapshot>,
) -> Result<(), BoxError> {
    let channel_id = message
        .metadata
        .discord_channel_id
        .ok_or("missing discord_channel_id")?;
    let guild_id = message
        .metadata
        .discord_guild_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "dm".to_string());

    let user = user_store.get_or_create_user("discord", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!("discord:{}:{}:{}", guild_id, channel_id, message.thread_id);
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    append_discord_message_payload(
        &workspace,
        message,
        raw_payload,
        thread_state.last_email_seq,
    )?;
    if let Err(err) =
        hydrate_discord_attachments(config, &workspace, raw_payload, thread_state.last_email_seq)
    {
        warn!(
            "failed to hydrate discord attachments for {}: {}",
            workspace.display(),
            err
        );
    }

    if let Some(snapshot) = snapshot {
        hydrate_discord_context_files_from_snapshot(
            &workspace,
            thread_state.last_email_seq,
            snapshot,
        )?;
    } else {
        hydrate_discord_context_files(
            config,
            &workspace,
            message,
            raw_payload,
            thread_state.last_email_seq,
        )?;
    }

    Ok(())
}

pub(crate) fn process_discord_inbound_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    account_store: &AccountStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let channel_id = message
        .metadata
        .discord_channel_id
        .ok_or("missing discord_channel_id")?;
    let guild_id = message
        .metadata
        .discord_guild_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "dm".to_string());

    let user = user_store.get_or_create_user("discord", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!("discord:{}:{}:{}", guild_id, channel_id, message.thread_id);

    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    append_discord_message_payload(
        &workspace,
        message,
        raw_payload,
        thread_state.last_email_seq,
    )?;
    if let Err(err) =
        hydrate_discord_attachments(config, &workspace, raw_payload, thread_state.last_email_seq)
    {
        warn!(
            "failed to hydrate discord attachments for {}: {}",
            workspace.display(),
            err
        );
    }
    if let Err(err) = hydrate_discord_context_files(
        config,
        &workspace,
        message,
        raw_payload,
        thread_state.last_email_seq,
    ) {
        warn!(
            "failed to hydrate discord context files for {}: {}",
            workspace.display(),
            err
        );
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
        // reply_to[0] = user_id (for account lookup), reply_to[1] = channel_id
        reply_to: vec![message.sender.clone(), channel_id.to_string()],
        reply_from: None,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Discord,
        slack_team_id: None,
        employee_id: Some(config.employee_id.clone()),
        requester_identifier_type: None,
        requester_identifier: None,
        account_id: None,
    };

    // Clone run_task before consuming it, in case we need to write to account-level storage
    let run_task_for_account = run_task.clone();

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} guild={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        guild_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    // If the Discord user has linked their account, also write to user-level tasks.db
    // message.sender contains the Discord user ID (same as reply_to[0])
    if let Ok(Some(account)) = account_store.get_account_by_identifier("discord", &message.sender) {
        let user_tasks_dir = config.users_root.join(account.id.to_string()).join("state");
        if let Err(err) = std::fs::create_dir_all(&user_tasks_dir) {
            warn!(
                "failed to create user tasks dir for account {}: {}",
                account.id, err
            );
        } else {
            let user_tasks_db_path = user_tasks_dir.join("tasks.db");
            match Scheduler::load(&user_tasks_db_path, ModuleExecutor::default()) {
                Ok(mut user_scheduler) => {
                    // Use the same task_id so we can update status at completion
                    match user_scheduler.add_one_shot_in_with_id(
                        task_id,
                        Duration::from_secs(0),
                        TaskKind::RunTask(run_task_for_account),
                    ) {
                        Ok(()) => {
                            info!(
                                "also enqueued task to account-level storage account={} task_id={}",
                                account.id, task_id
                            );
                        }
                        Err(err) => {
                            warn!(
                                "failed to add task to user scheduler for account {}: {}",
                                account.id, err
                            );
                        }
                    }
                }
                Err(err) => {
                    warn!(
                        "failed to load user scheduler for account {}: {}",
                        account.id, err
                    );
                }
            }
        }
    }

    Ok(())
}

pub(super) fn append_discord_message_payload(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    let raw_path = incoming_dir.join(format!("{:05}_discord_raw.json", seq));
    std::fs::write(&raw_path, raw_payload)?;

    let text_path = incoming_dir.join(format!("{:05}_discord_message.txt", seq));
    let text_content = build_discord_message_text_with_quote(message, raw_payload);
    std::fs::write(&text_path, &text_content)?;

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
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved Discord message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

pub(crate) fn hydrate_discord_attachments(
    config: &ServiceConfig,
    workspace: &Path,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_attachments = workspace.join("incoming_attachments");
    let entries_dir = incoming_attachments.join("entries");
    std::fs::create_dir_all(&entries_dir)?;
    clear_dir_except(&incoming_attachments, &entries_dir)?;

    if raw_payload.is_empty() {
        return Ok(());
    }

    let payload: DiscordRawPayloadAttachments = match serde_json::from_slice(raw_payload) {
        Ok(value) => value,
        Err(err) => {
            warn!(
                "failed to parse discord raw payload for attachment hydration: {}",
                err
            );
            return Ok(());
        }
    };

    if payload.attachments.is_empty() {
        return Ok(());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    let bot_token = resolve_discord_bot_token(config);
    let entry_dir = entries_dir.join(format!("{:05}_discord_attachments", seq));
    std::fs::create_dir_all(&entry_dir)?;

    let mut saved = 0usize;
    let mut used_names = HashSet::new();
    for attachment in payload.attachments {
        let file_name = ascii_safe_attachment_name(&attachment.filename, &mut used_names);
        match download_discord_attachment(&client, &attachment, bot_token.as_deref()) {
            Ok(bytes) => {
                std::fs::write(incoming_attachments.join(&file_name), &bytes)?;
                std::fs::write(entry_dir.join(&file_name), &bytes)?;
                saved += 1;
            }
            Err(err) => {
                warn!(
                    "failed to download discord attachment '{}' for workspace {}: {}",
                    attachment.filename,
                    workspace.display(),
                    err
                );
            }
        }
    }

    if saved > 0 {
        info!(
            "saved {} Discord attachment(s) seq={} to {}",
            saved,
            seq,
            incoming_attachments.display()
        );
    }

    Ok(())
}

fn download_discord_attachment(
    client: &reqwest::blocking::Client,
    attachment: &DiscordAttachmentDownloadPayload,
    bot_token: Option<&str>,
) -> Result<Vec<u8>, BoxError> {
    let mut candidate_urls = vec![attachment.url.as_str()];
    if !attachment.proxy_url.trim().is_empty() && attachment.proxy_url != attachment.url {
        candidate_urls.push(attachment.proxy_url.as_str());
    }

    let mut last_error = None;
    for url in candidate_urls {
        match download_discord_attachment_from_url(client, url, None) {
            Ok(bytes) => return Ok(bytes),
            Err(err) => {
                last_error = Some(format!("{url}: {err}"));
            }
        }

        if let Some(token) = bot_token {
            match download_discord_attachment_from_url(client, url, Some(token)) {
                Ok(bytes) => return Ok(bytes),
                Err(err) => {
                    last_error = Some(format!("{url}: {err}"));
                }
            }
        }
    }

    Err(last_error
        .unwrap_or_else(|| {
            format!(
                "no downloadable URL found for discord attachment '{}'",
                attachment.filename
            )
        })
        .into())
}

fn download_discord_attachment_from_url(
    client: &reqwest::blocking::Client,
    url: &str,
    bot_token: Option<&str>,
) -> Result<Vec<u8>, BoxError> {
    let mut request = client.get(url);
    if let Some(token) = bot_token {
        request = request.header("Authorization", format!("Bot {}", token));
    }

    let response = request.send()?;
    if !response.status().is_success() {
        return Err(format!("discord attachment download returned {}", response.status()).into());
    }

    Ok(response.bytes()?.to_vec())
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

fn clear_dir_except(root: &Path, keep: &Path) -> Result<(), std::io::Error> {
    if !root.exists() {
        std::fs::create_dir_all(root)?;
        return Ok(());
    }
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path == keep {
            continue;
        }
        if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        } else {
            std::fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn ascii_safe_attachment_name(path: &str, used_names: &mut HashSet<String>) -> String {
    let trimmed = path.trim();
    let (stem, extension) = match trimmed.rsplit_once('.') {
        Some((stem, extension)) if !stem.is_empty() && !extension.is_empty() => {
            (stem, Some(extension))
        }
        _ => (trimmed, None),
    };
    let mut base = sanitize_ascii_attachment_stem(stem);
    if base.is_empty() {
        base = "attachment".to_string();
    }

    let extension = extension
        .map(sanitize_ascii_attachment_extension)
        .filter(|value| !value.is_empty());
    uniquify_attachment_name(base, extension.as_deref(), used_names)
}

fn sanitize_ascii_attachment_stem(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
        } else if matches!(ch, '.' | '_' | '-') {
            output.push(ch);
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }
    let trimmed = output.trim_matches(['.', '_', '-']);
    truncate_ascii(trimmed, 80)
}

fn sanitize_ascii_attachment_extension(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

fn uniquify_attachment_name(
    base: String,
    extension: Option<&str>,
    used_names: &mut HashSet<String>,
) -> String {
    let build_name = |suffix: Option<usize>| match (extension, suffix) {
        (Some(ext), Some(idx)) => format!("{base}_{idx}.{ext}"),
        (Some(ext), None) => format!("{base}.{ext}"),
        (None, Some(idx)) => format!("{base}_{idx}"),
        (None, None) => base.clone(),
    };

    let mut candidate = build_name(None);
    if used_names.insert(candidate.clone()) {
        return candidate;
    }

    for idx in 2..10_000 {
        candidate = build_name(Some(idx));
        if used_names.insert(candidate.clone()) {
            return candidate;
        }
    }

    build_name(Some(10_000))
}

fn truncate_ascii(value: &str, max_len: usize) -> String {
    if value.len() <= max_len {
        return value.to_string();
    }
    let mut out = value[..max_len].to_string();
    while out.ends_with(['.', '_', '-']) {
        out.pop();
    }
    if out.is_empty() {
        value.to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{ChannelMetadata, InboundMessage};
    use crate::employee_config::{EmployeeDirectory, EmployeeProfile};
    use mockito::Server;
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn build_test_config(root: &Path) -> ServiceConfig {
        let users_root = root.join("users");
        let state_root = root.join("state");

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

    #[test]
    fn append_discord_message_payload_writes_message_files() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let workspace = temp.path();

        let sender = "12345".to_string();
        let channel_id = 67890u64;
        let guild_id = 111u64;
        let raw_payload = br#"{"fake":"payload"}"#.to_vec();
        let message = InboundMessage {
            channel: Channel::Discord,
            sender: sender.clone(),
            sender_name: Some("test-user".to_string()),
            recipient: channel_id.to_string(),
            subject: None,
            text_body: Some("Hello".to_string()),
            html_body: None,
            thread_id: "thread-abc".to_string(),
            message_id: Some("msg-1".to_string()),
            attachments: Vec::new(),
            reply_to: vec![sender.clone(), channel_id.to_string()],
            raw_payload: raw_payload.clone(),
            metadata: ChannelMetadata {
                discord_guild_id: Some(guild_id),
                discord_channel_id: Some(channel_id),
                ..Default::default()
            },
        };

        let seq = 7;
        append_discord_message_payload(workspace, &message, &raw_payload, seq)?;

        let incoming_dir = workspace.join("incoming_email");
        assert!(incoming_dir
            .join(format!("{:05}_discord_raw.json", seq))
            .exists());
        assert!(incoming_dir
            .join(format!("{:05}_discord_message.txt", seq))
            .exists());
        assert!(incoming_dir
            .join(format!("{:05}_discord_meta.json", seq))
            .exists());
        Ok(())
    }

    #[test]
    fn hydrate_discord_attachments_downloads_attachments() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let root = temp.path();
        let workspace = root.join("workspace");
        fs::create_dir_all(&workspace)?;

        let config = build_test_config(root);

        let mut server = Server::new();
        let attachment_bytes = b"fake png bytes";
        let attachment_mock = server
            .mock("GET", "/attachments/test.png")
            .with_status(200)
            .with_body(attachment_bytes.as_slice())
            .create();

        let channel_id = 67890u64;
        let guild_id = 111u64;
        let raw_payload = serde_json::to_vec(&serde_json::json!({
            "id": 1001u64,
            "channel_id": channel_id,
            "guild_id": guild_id,
            "author_id": 12345u64,
            "author_name": "test-user",
            "content": "",
            "timestamp": "2026-03-22T05:00:00Z",
            "attachments": [{
                "id": 1u64,
                "filename": "test.png",
                "content_type": "image/png",
                "size": attachment_bytes.len(),
                "url": format!("{}/attachments/test.png", server.url()),
                "proxy_url": ""
            }]
        }))?;
        let seq = 7;
        hydrate_discord_attachments(&config, &workspace, &raw_payload, seq)?;

        attachment_mock.assert();

        let attachment_path = workspace.join("incoming_attachments").join("test.png");
        assert_eq!(fs::read(&attachment_path)?, attachment_bytes);

        let archived_attachment = workspace
            .join("incoming_attachments")
            .join("entries")
            .join(format!("{:05}_discord_attachments", seq))
            .join("test.png");
        assert_eq!(fs::read(archived_attachment)?, attachment_bytes);

        Ok(())
    }
}
