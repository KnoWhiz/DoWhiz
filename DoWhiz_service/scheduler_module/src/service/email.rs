use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::Utc;
use tracing::{error, info, warn};

use crate::channel::Channel;
use crate::index_store::IndexStore;
use crate::mailbox;
use crate::user_store::{extract_emails, UserStore};
use crate::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};

use super::bump_thread_state;
use super::config::ServiceConfig;
use super::default_thread_state_path;
use super::html::render_email_html;
use super::postmark::{collect_service_address_candidates, normalize_message_id};
use super::recipients::replyable_recipients;
use super::scheduler::cancel_pending_thread_tasks;
use super::workspace::{create_unique_dir, ensure_thread_workspace, write_thread_history};
use super::BoxError;

pub use super::postmark::PostmarkInbound;

pub fn process_inbound_payload(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    info!("processing inbound payload into workspace");

    let sender = payload.from.as_deref().unwrap_or("").trim();
    if is_blacklisted_sender(sender, &config.employee_directory.service_addresses) {
        info!("skipping blacklisted sender: {}", sender);
        return Ok(());
    }
    let user_email = payload.from.as_deref().unwrap_or("").trim();
    let user_email = extract_emails(user_email)
        .into_iter()
        .next()
        .ok_or_else(|| "missing sender email".to_string())?;
    let user = user_store.get_or_create_user("email", &user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let reply_to_raw = payload.reply_to.as_deref().unwrap_or("");
    let from_raw = payload.from.as_deref().unwrap_or("");
    let mut to_list = replyable_recipients(reply_to_raw);
    if to_list.is_empty() {
        to_list = replyable_recipients(from_raw);
    }
    if to_list.is_empty() {
        info!(
            "no replyable recipients found (reply_to='{}', from='{}')",
            reply_to_raw, from_raw
        );
    }

    let inbound_candidates = collect_service_address_candidates(payload);
    let inbound_service_mailbox = mailbox::select_inbound_service_mailbox(
        &inbound_candidates,
        &config.employee_profile.address_set,
    );
    let inbound_service_mailbox = match inbound_service_mailbox {
        Some(mailbox) => mailbox,
        None => {
            info!("no service address found in inbound payload; skipping");
            return Ok(());
        }
    };

    let thread_key = thread_key(payload, raw_payload);
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;
    let reply_from = Some(inbound_service_mailbox.formatted());
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
    let thread_state_path = default_thread_state_path(&workspace);
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .map(|value| value.trim().to_string());
    let thread_state = bump_thread_state(&thread_state_path, &thread_key, message_id.clone())?;
    append_inbound_payload(
        &workspace,
        payload,
        raw_payload,
        thread_state.last_email_seq,
    )?;
    if let Err(err) = archive_inbound(&user_paths, payload, raw_payload) {
        error!("failed to archive inbound email: {}", err);
    }
    info!(
        "workspace ready at {} for user {} thread={} epoch={}",
        workspace.display(),
        user.user_id,
        thread_key,
        thread_state.epoch
    );

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: to_list.clone(),
        reply_from,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Email,
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
        "scheduler tasks enqueued user_id={} task_id={} message_id={} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message_id.unwrap_or_else(|| "-".to_string()),
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

fn is_blacklisted_sender(sender: &str, service_addresses: &HashSet<String>) -> bool {
    if sender.is_empty() {
        return false;
    }
    let mut matched = false;
    let addresses = extract_emails(sender);
    for address in addresses {
        if is_blacklisted_address(&address, service_addresses) {
            matched = true;
            break;
        }
    }
    matched
}

fn is_blacklisted_address(address: &str, service_addresses: &HashSet<String>) -> bool {
    mailbox::is_service_address(address, service_addresses)
}

fn thread_key(payload: &PostmarkInbound, raw_payload: &[u8]) -> String {
    if let Some(value) = payload.header_value("References") {
        if let Some(id) = extract_first_message_id(value) {
            return id;
        }
    }
    if let Some(value) = payload.header_value("In-Reply-To") {
        if let Some(id) = extract_first_message_id(value) {
            return id;
        }
    }
    if let Some(id) = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .and_then(normalize_message_id)
    {
        return id;
    }
    format!("{:x}", md5::compute(raw_payload))
}

fn extract_first_message_id(value: &str) -> Option<String> {
    for token in value.split(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | ',' | ';')) {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(id) = normalize_message_id(trimmed) {
            return Some(id);
        }
    }
    None
}

fn append_inbound_payload(
    workspace: &Path,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_email = workspace.join("incoming_email");
    let incoming_attachments = workspace.join("incoming_attachments");
    let entries_email = incoming_email.join("entries");
    let entries_attachments = incoming_attachments.join("entries");
    std::fs::create_dir_all(&entries_email)?;
    std::fs::create_dir_all(&entries_attachments)?;

    let entry_name = build_inbound_entry_name(payload, seq);
    let entry_email_dir = entries_email.join(&entry_name);
    let entry_attachments_dir = entries_attachments.join(&entry_name);
    std::fs::create_dir_all(&entry_email_dir)?;
    std::fs::create_dir_all(&entry_attachments_dir)?;
    write_inbound_payload(
        payload,
        raw_payload,
        &entry_email_dir,
        &entry_attachments_dir,
    )?;

    clear_dir_except(&incoming_attachments, &entries_attachments)?;
    write_inbound_payload(payload, raw_payload, &incoming_email, &incoming_attachments)?;
    if let Err(err) = write_thread_history(&incoming_email, &incoming_attachments) {
        warn!("failed to write thread history: {}", err);
    }
    Ok(())
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

fn archive_inbound(
    user_paths: &crate::user_store::UserPaths,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let fallback = format!("email_{}", Utc::now().timestamp());
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .unwrap_or("");
    let base = sanitize_token(message_id, &fallback);
    let year = Utc::now().format("%Y").to_string();
    let month = Utc::now().format("%m").to_string();
    let mail_root = user_paths.mail_root.join(year).join(month);
    std::fs::create_dir_all(&mail_root)?;
    let mail_dir = create_unique_dir(&mail_root, &base)?;
    let incoming_email = mail_dir.join("incoming_email");
    let incoming_attachments = mail_dir.join("incoming_attachments");
    std::fs::create_dir_all(&incoming_email)?;
    std::fs::create_dir_all(&incoming_attachments)?;
    write_inbound_payload(payload, raw_payload, &incoming_email, &incoming_attachments)?;
    Ok(())
}

fn write_inbound_payload(
    payload: &PostmarkInbound,
    raw_payload: &[u8],
    incoming_email: &Path,
    incoming_attachments: &Path,
) -> Result<(), BoxError> {
    std::fs::write(incoming_email.join("postmark_payload.json"), raw_payload)?;
    let email_html = render_email_html(payload);
    std::fs::write(incoming_email.join("email.html"), email_html)?;

    if let Some(attachments) = payload.attachments.as_ref() {
        for attachment in attachments {
            let name = sanitize_token(&attachment.name, "attachment");
            let target = incoming_attachments.join(name);
            let data = BASE64_STANDARD
                .decode(attachment.content.as_bytes())
                .unwrap_or_default();
            std::fs::write(target, data)?;
        }
    }
    Ok(())
}

fn build_inbound_entry_name(payload: &PostmarkInbound, seq: u64) -> String {
    let subject = payload.subject.as_deref().unwrap_or("");
    let subject_token = sanitize_token(subject, "no_subject");
    let subject_token = truncate_ascii(&subject_token, 48);
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let base = format!("{}_{}", timestamp, subject_token);
    format!("{:04}_{}", seq, base)
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

fn sanitize_token(value: &str, fallback: &str) -> String {
    let trimmed = value.trim().trim_start_matches('<').trim_end_matches('>');
    let mut out = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    let cleaned = out.trim_matches(&['.', '_', '-'][..]);
    if cleaned.is_empty() {
        fallback.to_string()
    } else {
        cleaned.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::employee_config::EmployeeProfile;
    use crate::user_store::UserPaths;
    use std::collections::HashSet;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn create_workspace_hydrates_past_emails() {
        let temp = TempDir::new().expect("tempdir");
        let user_root = temp.path().join("user");
        let user_paths = UserPaths {
            root: user_root.clone(),
            state_dir: user_root.join("state"),
            tasks_db_path: user_root.join("state/tasks.db"),
            memory_dir: user_root.join("memory"),
            secrets_dir: user_root.join("secrets"),
            mail_root: user_root.join("mail"),
            workspaces_root: user_root.join("workspaces"),
        };
        fs::create_dir_all(&user_paths.mail_root).expect("mail root");
        fs::create_dir_all(&user_paths.workspaces_root).expect("workspaces root");

        let archive_dir = user_paths.mail_root.join("2026").join("02").join("msg_1");
        let incoming_email = archive_dir.join("incoming_email");
        let incoming_attachments = archive_dir.join("incoming_attachments");
        fs::create_dir_all(&incoming_email).expect("incoming_email");
        fs::create_dir_all(&incoming_attachments).expect("incoming_attachments");
        fs::write(incoming_email.join("email.html"), "<pre>Hello</pre>").expect("email.html");
        let archived_payload = r#"{
  "From": "Alice <alice@example.com>",
  "To": "Bob <bob@example.com>",
  "Subject": "Archive hello",
  "Date": "Tue, 03 Feb 2026 20:10:44 -0800",
  "MessageID": "<msg-1@example.com>",
  "Attachments": [
    {"Name": "report.pdf", "ContentType": "application/pdf"}
  ]
}"#;
        fs::write(
            incoming_email.join("postmark_payload.json"),
            archived_payload,
        )
        .expect("postmark payload");
        fs::write(incoming_attachments.join("report.pdf"), "data").expect("attachment");

        let inbound_raw = r#"{
  "From": "New <new@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "New request",
  "TextBody": "Hi"
}"#;
        let inbound_payload: PostmarkInbound =
            serde_json::from_str(inbound_raw).expect("parse inbound");
        let thread = thread_key(&inbound_payload, inbound_raw.as_bytes());
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
            address_set,
            runtime_root: None,
            agents_path: None,
            claude_path: None,
            soul_path: None,
            skills_dir: None,
            discord_enabled: false,
            slack_enabled: false,
            bluebubbles_enabled: false,
        };
        let workspace = ensure_thread_workspace(&user_paths, "user123", &thread, &employee, None)
            .expect("create workspace");

        let past_root = workspace.join("references").join("past_emails");
        let index_path = past_root.join("index.json");
        assert!(index_path.exists(), "index.json created");

        let index_data = fs::read_to_string(index_path).expect("read index");
        let index_json: serde_json::Value = serde_json::from_str(&index_data).expect("parse index");
        let entries = index_json["entries"].as_array().expect("entries array");
        assert_eq!(entries.len(), 1, "one archived entry");
        let entry_path = entries[0]["path"].as_str().expect("entry path");
        assert!(past_root.join(entry_path).join("incoming_email").exists());
        assert!(past_root
            .join(entry_path)
            .join("attachments_manifest.json")
            .exists());
    }

}
