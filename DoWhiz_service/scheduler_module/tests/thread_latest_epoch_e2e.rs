use run_task_module::RunTaskParams;
use scheduler_module::index_store::IndexStore;
use scheduler_module::service::{process_inbound_payload, PostmarkInbound, ServiceConfig};
use scheduler_module::user_store::UserStore;
use scheduler_module::{Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::TempDir;

#[derive(Clone, Default)]
struct RecordingExecutor {
    sent_subjects: Arc<Mutex<Vec<String>>>,
}

struct EnvGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let original = env::var(key).ok();
        env::set_var(key, value);
        Self { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}

impl TaskExecutor for RecordingExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        match task {
            TaskKind::RunTask(run) => {
                let params = RunTaskParams {
                    workspace_dir: run.workspace_dir.clone(),
                    input_email_dir: run.input_email_dir.clone(),
                    input_attachments_dir: run.input_attachments_dir.clone(),
                    memory_dir: run.memory_dir.clone(),
                    reference_dir: run.reference_dir.clone(),
                    model_name: run.model_name.clone(),
                    codex_disabled: run.codex_disabled,
                };
                let output = run_task_module::run_task(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
                Ok(TaskExecution {
                    follow_up_tasks: output.scheduled_tasks,
                    follow_up_error: output.scheduled_tasks_error,
                    scheduler_actions: output.scheduler_actions,
                    scheduler_actions_error: output.scheduler_actions_error,
                })
            }
            TaskKind::SendEmail(send) => {
                self.sent_subjects
                    .lock()
                    .expect("sent_subjects lock poisoned")
                    .push(send.subject.clone());
                Ok(TaskExecution::default())
            }
            TaskKind::Noop => Ok(TaskExecution::default()),
        }
    }
}

fn write_fake_codex(bin_dir: &Path) -> io::Result<()> {
    let script = r#"#!/bin/sh
set -e
if [ -f "incoming_email/email.txt" ]; then
  subject=$(grep -m1 '^Subject:' incoming_email/email.txt | sed 's/^Subject: //')
else
  subject="(no subject)"
fi
cat > reply_email_draft.html <<EOF
<html><body>Reply to ${subject}</body></html>
EOF
exit 0
"#;
    let path = bin_dir.join("codex");
    fs::write(&path, script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }
    Ok(())
}

fn first_workspace_dir(root: &Path) -> PathBuf {
    let mut entries = fs::read_dir(root).expect("read workspaces dir");
    while let Some(entry) = entries.next() {
        let path = entry.expect("workspace entry").path();
        if path.is_dir() {
            return path;
        }
    }
    panic!("no workspace directory created");
}

#[test]
fn thread_latest_epoch_end_to_end() {
    let temp = TempDir::new().expect("tempdir");
    let root = temp.path();
    let users_root = root.join("users");
    let state_root = root.join("state");
    let bin_root = root.join("bin");
    let home_root = root.join("home");
    fs::create_dir_all(&users_root).expect("users root");
    fs::create_dir_all(&state_root).expect("state root");
    fs::create_dir_all(&bin_root).expect("bin root");
    fs::create_dir_all(&home_root).expect("home root");

    write_fake_codex(&bin_root).expect("write fake codex");
    let original_path = env::var("PATH").unwrap_or_default();
    let path_value = format!("{}:{}", bin_root.display(), original_path);
    let _path_guard = EnvGuard::set("PATH", path_value);
    let _api_guard = EnvGuard::set("AZURE_OPENAI_API_KEY_BACKUP", "test-key");
    let _endpoint_guard = EnvGuard::set("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.test");
    let _home_guard = EnvGuard::set("HOME", &home_root);

    let config = ServiceConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        workspace_root: root.join("workspaces"),
        scheduler_state_path: state_root.join("tasks.db"),
        processed_ids_path: state_root.join("processed_ids.txt"),
        users_root: users_root.clone(),
        users_db_path: state_root.join("users.db"),
        task_index_path: state_root.join("task_index.db"),
        codex_model: "gpt-5.2-codex".to_string(),
        codex_disabled: false,
        scheduler_poll_interval: Duration::from_millis(50),
        scheduler_max_concurrency: 2,
        scheduler_user_max_concurrency: 1,
        skills_source_dir: None,
    };

    let user_store = UserStore::new(&config.users_db_path).expect("user store");
    let index_store = IndexStore::new(&config.task_index_path).expect("index store");

    let inbound_raw_1 = r#"{
  "From": "Alice <alice@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "Hello 1",
  "TextBody": "First message",
  "Headers": [{"Name": "Message-ID", "Value": "<msg-1@example.com>"}]
}"#;
    let payload_1: PostmarkInbound = serde_json::from_str(inbound_raw_1).expect("parse inbound 1");
    process_inbound_payload(
        &config,
        &user_store,
        &index_store,
        &payload_1,
        inbound_raw_1.as_bytes(),
    )
    .expect("process inbound 1");

    let user = user_store
        .get_or_create_user("alice@example.com")
        .expect("user lookup");
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);

    let executor = RecordingExecutor::default();
    let mut scheduler =
        Scheduler::load(&user_paths.tasks_db_path, executor.clone()).expect("load scheduler");
    scheduler.tick().expect("tick run_task 1");

    let pending_send = scheduler
        .tasks()
        .iter()
        .filter(|task| matches!(task.kind, TaskKind::SendEmail(_)) && task.enabled)
        .count();
    assert_eq!(pending_send, 1, "pending send should exist");

    let inbound_raw_2 = r#"{
  "From": "Alice <alice@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "Hello 2",
  "TextBody": "Second message",
  "Headers": [
    {"Name": "Message-ID", "Value": "<msg-2@example.com>"},
    {"Name": "References", "Value": "<msg-1@example.com>"}
  ]
}"#;
    let payload_2: PostmarkInbound = serde_json::from_str(inbound_raw_2).expect("parse inbound 2");
    process_inbound_payload(
        &config,
        &user_store,
        &index_store,
        &payload_2,
        inbound_raw_2.as_bytes(),
    )
    .expect("process inbound 2");

    let mut scheduler =
        Scheduler::load(&user_paths.tasks_db_path, executor.clone()).expect("reload scheduler");
    let enabled_sends_after_cancel = scheduler
        .tasks()
        .iter()
        .filter(|task| matches!(task.kind, TaskKind::SendEmail(_)) && task.enabled)
        .count();
    assert_eq!(
        enabled_sends_after_cancel, 0,
        "stale send should be cancelled"
    );

    scheduler.tick().expect("tick run_task 2");
    scheduler.tick().expect("tick send 2");

    let sent = executor
        .sent_subjects
        .lock()
        .expect("sent_subjects lock poisoned");
    assert_eq!(sent.len(), 1, "only latest send should fire");

    let workspace = first_workspace_dir(&user_paths.workspaces_root);
    let reply_html =
        fs::read_to_string(workspace.join("reply_email_draft.html")).expect("reply draft");
    assert!(
        reply_html.contains("Hello 2"),
        "latest reply should use second email"
    );

    let drafts_dir = workspace.join("drafts");
    let drafts_count = fs::read_dir(drafts_dir).expect("drafts dir").count();
    assert!(drafts_count >= 2, "draft history should be preserved");
}
