use std::path::Path;
use std::time::Duration;

use tracing::{info, warn};

use crate::channel::Channel;
use crate::discord_gateway::DiscordGuildPaths;
use crate::index_store::IndexStore;
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::super::bump_thread_state;
use super::super::config::ServiceConfig;
use super::super::default_thread_state_path;
use super::super::scheduler::cancel_pending_thread_tasks;
use super::super::workspace::{copy_skills_directory, ensure_workspace_employee_files};
use super::super::BoxError;

pub(crate) fn process_discord_inbound_message(
    config: &ServiceConfig,
    index_store: &IndexStore,
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

    let guild_paths = DiscordGuildPaths::new(&config.workspace_root, &guild_id);
    guild_paths.ensure_dirs()?;

    let thread_key = format!("discord:{}:{}:{}", guild_id, channel_id, message.thread_id);

    let workspace = ensure_discord_workspace(
        &guild_paths,
        channel_id,
        &message.thread_id,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    append_discord_message_payload(&workspace, message, raw_payload, thread_state.last_email_seq)?;

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
        reply_to: vec![channel_id.to_string()],
        reply_from: None,
        archive_root: None,
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Discord,
        slack_team_id: None,
        employee_id: Some(config.employee_id.clone()),
    };

    let mut scheduler = Scheduler::load(&guild_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

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

pub(super) fn ensure_discord_workspace(
    guild_paths: &DiscordGuildPaths,
    channel_id: u64,
    thread_id: &str,
    employee_profile: &crate::employee_config::EmployeeProfile,
    skills_source_dir: Option<&Path>,
) -> Result<std::path::PathBuf, BoxError> {
    let thread_hash = &md5::compute(thread_id.as_bytes())
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()[..8];

    let workspace = guild_paths
        .workspaces_root
        .join(channel_id.to_string())
        .join(thread_hash);

    if !workspace.exists() {
        std::fs::create_dir_all(&workspace)?;
        std::fs::create_dir_all(workspace.join("incoming_email"))?;
        std::fs::create_dir_all(workspace.join("incoming_attachments"))?;
        std::fs::create_dir_all(workspace.join("memory"))?;
        std::fs::create_dir_all(workspace.join("references"))?;

        ensure_workspace_employee_files(&workspace, employee_profile)?;

        let agents_skills_dir = workspace.join(".agents").join("skills");
        if let Some(skills_src) = skills_source_dir {
            if let Err(err) = copy_skills_directory(skills_src, &agents_skills_dir) {
                warn!("failed to copy base skills to workspace: {}", err);
            }
        }
        if let Some(employee_skills) = employee_profile.skills_dir.as_deref() {
            let should_copy = skills_source_dir
                .map(|base| base != employee_skills)
                .unwrap_or(true);
            if should_copy {
                if let Err(err) = copy_skills_directory(employee_skills, &agents_skills_dir) {
                    warn!("failed to copy employee skills to workspace: {}", err);
                }
            }
        }

        info!("created Discord workspace at {}", workspace.display());
    }

    Ok(workspace)
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
    let text_content = message.text_body.clone().unwrap_or_default();
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
