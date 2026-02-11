use run_task_module::RunTaskParams;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub static ENV_MUTEX: Mutex<()> = Mutex::new(());

pub struct TempDir {
    pub path: PathBuf,
}

impl TempDir {
    pub fn new(label: &str) -> io::Result<Self> {
        let mut path = env::temp_dir();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        path.push(format!("{}_{}_{}", label, process::id(), now));
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub struct EnvGuard {
    saved: Vec<(String, Option<OsString>)>,
}

impl EnvGuard {
    pub fn set(vars: &[(&str, &str)]) -> Self {
        let mut saved = Vec::with_capacity(vars.len() + 1);
        let mut has_docker_override = false;
        let mut has_docker_use_override = false;
        let mut has_codex_e2e_override = false;
        for (key, value) in vars {
            saved.push((key.to_string(), env::var_os(key)));
            env::set_var(key, value);
            if *key == "RUN_TASK_DOCKER_IMAGE" {
                has_docker_override = true;
            }
            if *key == "RUN_TASK_USE_DOCKER" {
                has_docker_use_override = true;
            }
            if *key == "RUN_CODEX_E2E" {
                has_codex_e2e_override = true;
            }
        }
        if !has_docker_override {
            saved.push((
                "RUN_TASK_DOCKER_IMAGE".to_string(),
                env::var_os("RUN_TASK_DOCKER_IMAGE"),
            ));
            env::set_var("RUN_TASK_DOCKER_IMAGE", "");
        }
        if !has_docker_use_override {
            saved.push((
                "RUN_TASK_USE_DOCKER".to_string(),
                env::var_os("RUN_TASK_USE_DOCKER"),
            ));
            env::set_var("RUN_TASK_USE_DOCKER", "0");
        }
        if !has_codex_e2e_override {
            saved.push(("RUN_CODEX_E2E".to_string(), env::var_os("RUN_CODEX_E2E")));
            env::set_var("RUN_CODEX_E2E", "0");
        }
        Self { saved }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in self.saved.drain(..) {
            match value {
                Some(prev) => env::set_var(&key, prev),
                None => env::remove_var(&key),
            }
        }
    }
}

#[allow(dead_code)]
pub struct EnvUnsetGuard {
    saved: Vec<(String, Option<OsString>)>,
}

#[allow(dead_code)]
impl EnvUnsetGuard {
    pub fn remove(keys: &[&str]) -> Self {
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

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FakeCodexMode {
    Success,
    NoOutput,
    Fail,
    GithubEnvCheck,
}

#[cfg(unix)]
pub fn write_fake_codex(dir: &Path, mode: FakeCodexMode) -> io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;

    let script_path = dir.join("codex");
    let script = match mode {
        FakeCodexMode::Success => {
            r#"#!/bin/sh
set -e
echo '{"type":"item.delta","item":{"type":"agent_message"},"delta":{"text":"ok"}}'
echo "<html><body>Test reply</body></html>" > reply_email_draft.html
mkdir -p reply_email_attachments
echo "attachment" > reply_email_attachments/attachment.txt
"#
        }
        FakeCodexMode::NoOutput => {
            r#"#!/bin/sh
set -e
echo '{"type":"item.delta","item":{"type":"agent_message"},"delta":{"text":"ok"}}'
"#
        }
        FakeCodexMode::Fail => {
            r#"#!/bin/sh
echo "simulated failure" >&2
exit 2
"#
        }
        FakeCodexMode::GithubEnvCheck => {
            r#"#!/bin/sh
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
echo "<html><body>Test reply</body></html>" > reply_email_draft.html
mkdir -p reply_email_attachments
echo "attachment" > reply_email_attachments/attachment.txt
"#
        }
    };

    fs::write(&script_path, script)?;
    let mut perms = fs::metadata(&script_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)?;
    Ok(script_path)
}

#[cfg(unix)]
#[allow(dead_code)]
pub fn write_fake_gh(dir: &Path) -> io::Result<PathBuf> {
    use std::os::unix::fs::PermissionsExt;

    let script_path = dir.join("gh");
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
    fs::write(&script_path, script)?;
    let mut perms = fs::metadata(&script_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)?;
    Ok(script_path)
}

pub fn create_workspace(root: &Path) -> io::Result<PathBuf> {
    let workspace = root.join("workspace");
    fs::create_dir_all(&workspace)?;
    fs::create_dir_all(workspace.join("incoming_email"))?;
    fs::create_dir_all(workspace.join("incoming_attachments"))?;
    fs::create_dir_all(workspace.join("memory"))?;
    fs::create_dir_all(workspace.join("references"))?;

    fs::write(
        workspace.join("incoming_email").join("email.html"),
        "<pre>Hello</pre>",
    )?;
    fs::write(
        workspace.join("incoming_attachments").join("doc_v1.txt"),
        "v1",
    )?;
    fs::write(
        workspace.join("incoming_attachments").join("doc_v2.txt"),
        "v2",
    )?;
    Ok(workspace)
}

pub fn build_params(workspace: &Path) -> RunTaskParams {
    RunTaskParams {
        workspace_dir: workspace.to_path_buf(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        reply_to: vec!["user@example.com".to_string()],
        model_name: "test-model".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
    }
}
