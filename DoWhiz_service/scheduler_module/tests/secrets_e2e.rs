use scheduler_module::{ModuleExecutor, RunTaskTask, TaskExecutor, TaskKind};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

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

#[test]
#[cfg(unix)]
fn secrets_sync_roundtrip_via_run_task() -> Result<(), Box<dyn std::error::Error>> {
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
