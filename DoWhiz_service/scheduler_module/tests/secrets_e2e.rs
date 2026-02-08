use scheduler_module::index_store::IndexStore;
use scheduler_module::service::{
    process_inbound_payload, PostmarkInbound, ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES,
};
use scheduler_module::user_store::UserStore;
use scheduler_module::{ModuleExecutor, RunTaskTask, Scheduler, TaskExecutor, TaskKind};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use tempfile::TempDir;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

struct EnvGuard {
    saved: Vec<(String, Option<String>)>,
}

impl EnvGuard {
    fn set(vars: &[(&str, &str)]) -> Self {
        let mut saved = Vec::with_capacity(vars.len());
        for (key, value) in vars {
            saved.push((key.to_string(), env::var(key).ok()));
            env::set_var(key, value);
        }
        Self { saved }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..) {
            match value {
                Some(value) => env::set_var(&key, value),
                None => env::remove_var(&key),
            }
        }
    }
}

struct EnvUnsetGuard {
    saved: Vec<(String, Option<String>)>,
}

impl EnvUnsetGuard {
    fn remove(keys: &[&str]) -> Self {
        let mut saved = Vec::with_capacity(keys.len());
        for key in keys {
            saved.push((key.to_string(), env::var(key).ok()));
            env::remove_var(key);
        }
        Self { saved }
    }
}

impl Drop for EnvUnsetGuard {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..) {
            match value {
                Some(value) => env::set_var(&key, value),
                None => env::remove_var(&key),
            }
        }
    }
}

#[cfg(unix)]
fn write_fake_codex(dir: &Path) -> std::io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;

    let script_path = dir.join("codex");
    let script = r#"#!/bin/sh
set -e
if [ "$USER_SECRET_TOKEN" != "origin" ]; then
  echo "missing user secret" >&2
  exit 3
fi
cat > .env <<EOF
USER_SECRET_TOKEN=updated
EOF
echo "<html><body>Test reply</body></html>" > reply_email_draft.html
mkdir -p reply_email_attachments
"#;

    fs::write(&script_path, script)?;
    let mut perms = fs::metadata(&script_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)?;
    Ok(script_path)
}

#[cfg(unix)]
fn write_instruction_codex(
    dir: &Path,
    instruction: &str,
    env_key: &str,
    env_value: &str,
) -> std::io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;

    let script_path = dir.join("codex");
    let script = format!(
        r#"#!/bin/sh
set -e
instruction="{instruction}"
key="{env_key}"
expected="{env_value}"
eval "value=\${{$key}}"
if [ "$value" != "$expected" ]; then
  if ! grep -Fq "$instruction" "incoming_email/email.html"; then
    echo "instruction not found" >&2
    exit 3
  fi
  cat > .env <<'EOF'
{env_key}={env_value}
EOF
fi
cat > reply_email_draft.html <<'EOF'
<html><body>Reply ready</body></html>
EOF
mkdir -p reply_email_attachments
"#,
        instruction = instruction,
        env_key = env_key,
        env_value = env_value,
    );

    fs::write(&script_path, script)?;
    let mut perms = fs::metadata(&script_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)?;
    Ok(script_path)
}

fn list_workspace_dirs(root: &Path) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(root)
        .expect("read workspaces dir")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    entries.sort();
    entries
}

#[test]
#[cfg(unix)]
fn secrets_sync_roundtrip_via_run_task() -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new()?;
    let home_dir = temp.path().join("home");
    let bin_dir = temp.path().join("bin");
    fs::create_dir_all(&home_dir)?;
    fs::create_dir_all(&bin_dir)?;
    write_fake_codex(&bin_dir)?;

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.com"),
        ("CODEX_MODEL", "test-model"),
    ]);

    let user_root = temp.path().join("users").join("user_1");
    let user_secrets = user_root.join("secrets");
    let user_mail = user_root.join("mail");
    let workspace = user_root.join("workspaces").join("thread_1");

    fs::create_dir_all(&user_secrets)?;
    fs::create_dir_all(&user_mail)?;
    fs::create_dir_all(workspace.join("incoming_email"))?;
    fs::create_dir_all(workspace.join("incoming_attachments"))?;
    fs::create_dir_all(workspace.join("memory"))?;
    fs::create_dir_all(workspace.join("references"))?;

    fs::write(user_secrets.join(".env"), "USER_SECRET_TOKEN=origin")?;
    fs::write(workspace.join(".env"), "USER_SECRET_TOKEN=stale")?;

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "test-model".to_string(),
        codex_disabled: false,
        reply_to: Vec::new(),
        archive_root: Some(user_mail),
        thread_id: None,
        thread_epoch: None,
        thread_state_path: None,
    };

    let executor = ModuleExecutor::default();
    executor.execute(&TaskKind::RunTask(run_task))?;

    let updated = fs::read_to_string(user_secrets.join(".env"))?;
    assert!(updated.contains("USER_SECRET_TOKEN=updated"));

    Ok(())
}

#[test]
#[cfg(unix)]
fn secrets_persist_across_workspaces_and_load(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new()?;
    let root = temp.path();
    let users_root = root.join("users");
    let state_root = root.join("state");
    let bin_root = root.join("bin");
    let home_root = root.join("home");
    fs::create_dir_all(&users_root)?;
    fs::create_dir_all(&state_root)?;
    fs::create_dir_all(&bin_root)?;
    fs::create_dir_all(&home_root)?;

    let instruction =
        "My api_key for weather is weather-123, could you help me store it?";
    let env_key = "DOWHIZ_TEST_WEATHER_API_KEY";
    let env_value = "weather-123";
    let expected_line = format!("{env_key}={env_value}");
    write_instruction_codex(&bin_root, instruction, env_key, env_value)?;

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_root.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_root.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.test"),
        ("GH_AUTH_DISABLED", "1"),
    ]);
    let _unset = EnvUnsetGuard::remove(&[env_key]);

    let config = ServiceConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        workspace_root: root.join("workspaces"),
        scheduler_state_path: state_root.join("tasks.db"),
        processed_ids_path: state_root.join("processed_ids.txt"),
        users_root: users_root.clone(),
        users_db_path: state_root.join("users.db"),
        task_index_path: state_root.join("task_index.db"),
        codex_model: "test-model".to_string(),
        codex_disabled: false,
        scheduler_poll_interval: Duration::from_millis(50),
        scheduler_max_concurrency: 1,
        scheduler_user_max_concurrency: 1,
        inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
        skills_source_dir: None,
    };

    let user_store = UserStore::new(&config.users_db_path)?;
    let index_store = IndexStore::new(&config.task_index_path)?;

    let inbound_raw = format!(
        r#"{{
  "From": "Alice <alice@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "Store API key",
  "TextBody": "{instruction}",
  "Headers": [{{"Name": "Message-ID", "Value": "<msg-1@example.com>"}}]
}}"#
    );
    let payload: PostmarkInbound = serde_json::from_str(&inbound_raw)?;
    process_inbound_payload(
        &config,
        &user_store,
        &index_store,
        &payload,
        inbound_raw.as_bytes(),
    )?;

    let user = user_store.get_or_create_user("alice@example.com")?;
    let user_paths = user_store.user_paths(&users_root, &user.user_id);
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    scheduler.tick()?;

    let workspaces = list_workspace_dirs(&user_paths.workspaces_root);
    assert_eq!(workspaces.len(), 1);
    let first_workspace = workspaces[0].clone();
    let email_html =
        fs::read_to_string(first_workspace.join("incoming_email").join("email.html"))?;
    assert!(email_html.contains("My api_key for weather"));

    let user_env_path = user_paths.secrets_dir.join(".env");
    let user_env = fs::read_to_string(&user_env_path)?;
    assert!(user_env.contains(&expected_line));

    let follow_up_raw = r#"{
  "From": "Alice <alice@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "Follow up",
  "TextBody": "Thanks again.",
  "Headers": [{"Name": "Message-ID", "Value": "<msg-2@example.com>"}]
}"#;
    let follow_up: PostmarkInbound = serde_json::from_str(follow_up_raw)?;
    process_inbound_payload(
        &config,
        &user_store,
        &index_store,
        &follow_up,
        follow_up_raw.as_bytes(),
    )?;

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    scheduler.tick()?;

    let workspaces = list_workspace_dirs(&user_paths.workspaces_root);
    assert_eq!(workspaces.len(), 2);
    let second_workspace = workspaces
        .into_iter()
        .find(|path| path != &first_workspace)
        .expect("new workspace");
    let workspace_env = fs::read_to_string(second_workspace.join(".env"))?;
    assert!(workspace_env.contains(&expected_line));
    let user_env_after = fs::read_to_string(&user_env_path)?;
    assert!(user_env_after.contains(&expected_line));

    Ok(())
}
