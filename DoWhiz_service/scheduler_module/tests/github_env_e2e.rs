use run_task_module::RunTaskParams;
use scheduler_module::index_store::IndexStore;
use scheduler_module::service::{
    process_inbound_payload, PostmarkInbound, ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES,
};
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
    errors: Arc<Mutex<Vec<String>>>,
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

struct EnvUnsetGuard {
    saved: Vec<(String, Option<std::ffi::OsString>)>,
}

impl EnvUnsetGuard {
    fn remove(keys: &[&str]) -> Self {
        let mut saved = Vec::with_capacity(keys.len());
        for key in keys {
            saved.push((key.to_string(), env::var_os(key)));
            env::remove_var(key);
        }
        Self { saved }
    }
}

impl Drop for EnvUnsetGuard {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..) {
            match value {
                Some(prev) => env::set_var(&key, prev),
                None => env::remove_var(&key),
            }
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
            TaskKind::SendEmail(_) => Ok(TaskExecution::default()),
            TaskKind::Noop => Ok(TaskExecution::default()),
        }
    }
}

fn write_fake_codex(bin_dir: &Path) -> io::Result<()> {
    let script = r#"#!/bin/sh
set -e
check_env() {
  key="$1"
  eval "value=\${$key}"
  if [ -z "$value" ]; then
    echo "missing $key" >&2
    exit 3
  fi
}
check_env "GH_TOKEN"
check_env "GITHUB_TOKEN"
check_env "GITHUB_USERNAME"
if [ -z "$GIT_ASKPASS" ] || [ ! -x "$GIT_ASKPASS" ]; then
  echo "missing GIT_ASKPASS" >&2
  exit 3
fi
cat > reply_email_draft.html <<EOF
<html><body>Reply ready</body></html>
EOF
mkdir -p reply_email_attachments
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

fn write_fake_gh(bin_dir: &Path) -> io::Result<()> {
    let script = r#"#!/bin/sh
set -e
if [ "$1" = "auth" ] && [ "$2" = "login" ]; then
  token="$(cat)"
  if [ -z "$token" ]; then
    echo "missing token" >&2
    exit 3
  fi
  exit 0
fi
if [ "$1" = "auth" ] && [ "$2" = "setup-git" ]; then
  exit 0
fi
if [ "$1" = "auth" ] && [ "$2" = "status" ]; then
  exit 0
fi
exit 0
"#;
    let path = bin_dir.join("gh");
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
fn email_flow_injects_github_env() {
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

    fs::write(
        root.join(".env"),
        "GITHUB_USERNAME=octo-user\nGITHUB_PERSONAL_ACCESS_TOKEN=pat-test-token\n",
    )
    .expect("write .env");

    write_fake_codex(&bin_root).expect("write fake codex");
    write_fake_gh(&bin_root).expect("write fake gh");

    let _unset_guard = EnvUnsetGuard::remove(&[
        "GH_TOKEN",
        "GITHUB_TOKEN",
        "GITHUB_PERSONAL_ACCESS_TOKEN",
        "GITHUB_USERNAME",
    ]);
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
        scheduler_max_concurrency: 1,
        scheduler_user_max_concurrency: 1,
        inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
        skills_source_dir: None,
    };

    let user_store = UserStore::new(&config.users_db_path).expect("user store");
    let index_store = IndexStore::new(&config.task_index_path).expect("index store");

    let inbound_raw = r#"{
  "From": "Alice <alice@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "Open a PR",
  "TextBody": "Please open a PR for issue 56.",
  "Headers": [{"Name": "Message-ID", "Value": "<msg-1@example.com>"}]
}"#;
    let payload: PostmarkInbound = serde_json::from_str(inbound_raw).expect("parse inbound");
    process_inbound_payload(
        &config,
        &user_store,
        &index_store,
        &payload,
        inbound_raw.as_bytes(),
    )
    .expect("process inbound");

    let user = user_store
        .get_or_create_user("alice@example.com")
        .expect("user lookup");
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);

    let executor = RecordingExecutor::default();
    let mut scheduler =
        Scheduler::load(&user_paths.tasks_db_path, executor.clone()).expect("load scheduler");
    scheduler.tick().expect("tick run_task");

    let workspace = first_workspace_dir(&user_paths.workspaces_root);
    assert!(
        workspace.join("reply_email_draft.html").exists(),
        "reply draft should be written"
    );

    let errors = executor
        .errors
        .lock()
        .expect("errors lock poisoned")
        .clone();
    assert!(errors.is_empty(), "expected no executor errors");
}
