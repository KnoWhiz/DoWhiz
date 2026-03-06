use run_task_module::RunTaskParams;
use scheduler_module::{
    RunTaskTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind,
};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::TempDir;

struct EnvGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = env::var(key).ok();
        env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}

struct EnvUnsetGuard {
    saved: Vec<(String, Option<OsString>)>,
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

#[derive(Clone, Default)]
struct RecordingExecutor {
    errors: Arc<Mutex<Vec<String>>>,
}

static TEST_MUTEX: Mutex<()> = Mutex::new(());

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
                    reply_to: run.reply_to.clone(),
                    model_name: run.model_name.clone(),
                    runner: run.runner.clone(),
                    codex_disabled: run.codex_disabled,
                    channel: run.channel.to_string(),
                    google_access_token:
                        scheduler_module::load_google_access_token_from_service_env(),
                    has_unified_account: false,
                    user_identities: Default::default(),
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
            TaskKind::SendReply(_) => Ok(TaskExecution::default()),
            TaskKind::Noop => Ok(TaskExecution::default()),
        }
    }
}

fn write_fake_codex_x402(dir: &Path) -> io::Result<PathBuf> {
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
check_exact_env() {
  key="$1"
  expected_key="EXPECTED_${key}"
  eval "expected=\${$expected_key}"
  if [ -n "$expected" ]; then
    eval "actual=\${$key}"
    if [ "$actual" != "$expected" ]; then
      echo "unexpected $key: expected '$expected' got '$actual'" >&2
      exit 3
    fi
  fi
}
check_env "GOATX402_API_URL"
check_env "GOATX402_MERCHANT_ID"
check_env "GOATX402_API_KEY"
check_env "GOATX402_API_SECRET"
check_exact_env "GOATX402_API_URL"
check_exact_env "GOATX402_MERCHANT_ID"
check_exact_env "GOATX402_API_KEY"
check_exact_env "GOATX402_API_SECRET"
cat > reply_email_draft.html <<EOF
<html><body>x402 route ready</body></html>
EOF
mkdir -p reply_email_attachments
echo "mock_tx_hash=0xabc123" > reply_email_attachments/x402_receipt.txt
"#;
    let path = dir.join("codex");
    fs::write(&path, script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }
    Ok(path)
}

fn setup_workspace(root: &Path) -> io::Result<PathBuf> {
    let workspace = root.join("workspace");
    fs::create_dir_all(workspace.join("memory"))?;
    fs::create_dir_all(workspace.join("references"))?;
    fs::create_dir_all(workspace.join("incoming_email"))?;
    fs::create_dir_all(workspace.join("incoming_attachments"))?;
    fs::write(
        workspace.join("incoming_email").join("email.html"),
        "<pre>Test x402 flow</pre>",
    )?;
    Ok(workspace)
}

fn run_scheduler_x402_env_test(
    dotenv_content: &str,
    employee_id: Option<&str>,
    expected_api_url: &str,
    expected_merchant_id: &str,
    expected_api_key: &str,
    expected_api_secret: &str,
) {
    let _test_lock = TEST_MUTEX.lock().expect("test lock");
    let temp = TempDir::new().expect("tempdir");
    let root = temp.path();
    let bin_root = root.join("bin");
    let home_root = root.join("home");
    fs::create_dir_all(&bin_root).expect("bin root");
    fs::create_dir_all(&home_root).expect("home root");
    let workspace = setup_workspace(root).expect("workspace setup");

    fs::write(root.join(".env"), dotenv_content).expect("write .env");
    write_fake_codex_x402(&bin_root).expect("write fake codex");

    let _unset_guard = EnvUnsetGuard::remove(&[
        "GOATX402_API_URL",
        "GOATX402_MERCHANT_ID",
        "GOATX402_API_KEY",
        "GOATX402_API_SECRET",
        "OLIVER_GOATX402_API_URL",
        "OLIVER_GOATX402_MERCHANT_ID",
        "OLIVER_GOATX402_API_KEY",
        "OLIVER_GOATX402_API_SECRET",
        "EMPLOYEE_PAYMENT_ENV_PREFIX",
        "PAYMENT_ENV_PREFIX",
        "EMPLOYEE_GITHUB_ENV_PREFIX",
        "GITHUB_ENV_PREFIX",
    ]);

    let original_path = env::var("PATH").unwrap_or_default();
    let path_value = format!("{}:{}", bin_root.display(), original_path);
    let _path_guard = EnvGuard::set("PATH", path_value);
    let _home_guard = EnvGuard::set("HOME", &home_root);
    let _docker_image_guard = EnvGuard::set("RUN_TASK_DOCKER_IMAGE", "");
    let _docker_mode_guard = EnvGuard::set("RUN_TASK_USE_DOCKER", "0");
    let _deploy_target_guard = EnvGuard::set("DEPLOY_TARGET", "local");
    let _execution_backend_guard = EnvGuard::set("RUN_TASK_EXECUTION_BACKEND", "local");
    let _gh_auth_guard = EnvGuard::set("GH_AUTH_DISABLED", "1");
    let _api_guard = EnvGuard::set("AZURE_OPENAI_API_KEY_BACKUP", "test-key");
    let _endpoint_guard = EnvGuard::set("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.test");
    let _expected_url_guard = EnvGuard::set("EXPECTED_GOATX402_API_URL", expected_api_url);
    let _expected_merchant_guard =
        EnvGuard::set("EXPECTED_GOATX402_MERCHANT_ID", expected_merchant_id);
    let _expected_key_guard = EnvGuard::set("EXPECTED_GOATX402_API_KEY", expected_api_key);
    let _expected_secret_guard = EnvGuard::set("EXPECTED_GOATX402_API_SECRET", expected_api_secret);
    let _employee_guard = employee_id.map(|id| EnvGuard::set("EMPLOYEE_ID", id));

    let executor = RecordingExecutor::default();
    let errors = executor.errors.clone();
    let mut scheduler = Scheduler::load(root.join("tasks.db"), executor).expect("load scheduler");

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-5.4".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        reply_to: vec!["user@example.com".to_string()],
        reply_from: None,
        archive_root: None,
        thread_id: Some("thread-x402".to_string()),
        thread_epoch: Some(1),
        thread_state_path: None,
        channel: scheduler_module::channel::Channel::default(),
        slack_team_id: None,
        employee_id: None,
    };

    scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
        .expect("add run_task");
    scheduler.tick().expect("tick run_task");

    assert!(
        workspace.join("reply_email_draft.html").exists(),
        "reply draft should be written"
    );
    assert!(
        workspace
            .join("reply_email_attachments")
            .join("x402_receipt.txt")
            .exists(),
        "x402 receipt should be written"
    );

    let errors = errors.lock().expect("errors lock poisoned").clone();
    assert!(errors.is_empty(), "expected no executor errors");
}

#[test]
fn scheduler_run_task_injects_x402_env_from_dotenv() {
    run_scheduler_x402_env_test(
        "GOATX402_API_URL=https://x402-api.example.test\nGOATX402_MERCHANT_ID=dowhiz_agent\nGOATX402_API_KEY=key_direct\nGOATX402_API_SECRET=secret_direct\n",
        None,
        "https://x402-api.example.test",
        "dowhiz_agent",
        "key_direct",
        "secret_direct",
    );
}

#[test]
fn scheduler_run_task_injects_employee_prefixed_x402_env_from_dotenv() {
    run_scheduler_x402_env_test(
        "OLIVER_GOATX402_API_URL=https://x402-api-prefixed.example.test\nOLIVER_GOATX402_MERCHANT_ID=dowhiz_agent_prefixed\nOLIVER_GOATX402_API_KEY=key_prefixed\nOLIVER_GOATX402_API_SECRET=secret_prefixed\n",
        Some("little_bear"),
        "https://x402-api-prefixed.example.test",
        "dowhiz_agent_prefixed",
        "key_prefixed",
        "secret_prefixed",
    );
}
