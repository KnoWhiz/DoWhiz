use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

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

pub(crate) fn process_sms_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let normalized_from = normalize_phone_number(&message.sender);
    let user = user_store.get_or_create_user("phone", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!(
        "sms:{}:{}",
        normalize_phone_number(&message.recipient),
        normalized_from
    );

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

    append_sms_message(&workspace, message, raw_payload, thread_state.last_email_seq)?;

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

    let reply_from = message
        .metadata
        .sms_to
        .clone()
        .or_else(|| Some(message.recipient.clone()));

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
        reply_from,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Sms,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

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
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

pub(super) fn append_sms_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    std::fs::create_dir_all(&incoming_dir)?;

    let raw_path = incoming_dir.join(format!("{:05}_sms_raw.txt", seq));
    std::fs::write(&raw_path, raw_payload)?;

    let text_path = incoming_dir.join(format!("{:05}_sms_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    std::fs::write(&text_path, &text_content)?;

    let meta_path = incoming_dir.join(format!("{:05}_sms_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "sms",
        "sender": message.sender,
        "recipient": message.recipient,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved SMS message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

pub(super) fn normalize_phone_number(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '+')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{ChannelMetadata, InboundMessage};
    use crate::employee_config::{EmployeeDirectory, EmployeeProfile};
    use crate::index_store::IndexStore;
    use crate::user_store::UserStore;
    use crate::{ModuleExecutor, Scheduler, TaskKind};
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn process_sms_message_creates_run_task() -> Result<(), BoxError> {
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
            ingestion_db_path: state_root.join("ingestion.db"),
            ingestion_dedupe_path: state_root.join("ingestion_processed_ids.txt"),
            ingestion_poll_interval: Duration::from_millis(50),
            users_root: users_root.clone(),
            users_db_path: state_root.join("users.db"),
            task_index_path: state_root.join("task_index.db"),
            codex_model: "gpt-5.2-codex".to_string(),
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

        let sender = "+1 (555) 123-4567".to_string();
        let recipient = "+1 555-999-0000".to_string();
        let raw_payload = b"From=%2B15551234567&To=%2B15559990000&Body=Hello".to_vec();
        let message = InboundMessage {
            channel: Channel::Sms,
            sender: sender.clone(),
            sender_name: None,
            recipient: recipient.clone(),
            subject: None,
            text_body: Some("Hello".to_string()),
            html_body: None,
            thread_id: "sms:test".to_string(),
            message_id: Some("SM123".to_string()),
            attachments: Vec::new(),
            reply_to: vec![sender.clone()],
            raw_payload: raw_payload.clone(),
            metadata: ChannelMetadata {
                sms_from: Some(sender.clone()),
                sms_to: Some(recipient.clone()),
                ..Default::default()
            },
        };

        process_sms_message(&config, &user_store, &index_store, &message, &raw_payload)?;

        let user = user_store.get_or_create_user("phone", &sender)?;
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

        assert_eq!(run_task.channel, Channel::Sms);
        assert_eq!(run_task.reply_to, vec![sender]);
        assert_eq!(run_task.reply_from.as_deref(), Some(recipient.as_str()));
        Ok(())
    }
}
