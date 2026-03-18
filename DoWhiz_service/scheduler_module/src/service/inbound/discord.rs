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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_store::AccountStore;
    use crate::channel::{ChannelMetadata, InboundMessage};
    use crate::employee_config::{EmployeeDirectory, EmployeeProfile};
    use crate::index_store::IndexStore;
    use crate::user_store::UserStore;
    use crate::{ModuleExecutor, Scheduler, TaskKind};
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn process_discord_inbound_message_creates_run_task() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let root = temp.path();
        let users_root = root.join("users");
        let state_root = root.join("state");
        fs::create_dir_all(&users_root)?;
        fs::create_dir_all(&state_root)?;

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

        let config = ServiceConfig {
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
            users_root: users_root.clone(),
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
        };

        let user_store = UserStore::new(&config.users_db_path)?;
        let index_store = IndexStore::new(&config.task_index_path)?;
        let account_store = AccountStore::new(&config.ingestion_db_url)?;

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

        process_discord_inbound_message(
            &config,
            &user_store,
            &index_store,
            &account_store,
            &message,
            &raw_payload,
        )?;

        let user = user_store.get_or_create_user("discord", &sender)?;
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

        assert_eq!(run_task.channel, Channel::Discord);
        assert_eq!(
            run_task.reply_to,
            vec![sender.clone(), channel_id.to_string()]
        );
        assert_eq!(run_task.archive_root.as_ref(), Some(&user_paths.mail_root));
        assert_eq!(
            run_task.workspace_dir.parent(),
            Some(user_paths.workspaces_root.as_path())
        );

        let state_path = crate::thread_state::default_thread_state_path(&run_task.workspace_dir);
        let thread_state =
            crate::thread_state::load_thread_state(&state_path).expect("thread_state.json exists");
        let seq = thread_state.last_email_seq;
        let incoming_dir = run_task.workspace_dir.join("incoming_email");
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
}
