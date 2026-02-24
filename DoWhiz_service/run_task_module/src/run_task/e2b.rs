use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use super::constants::GIT_ASKPASS_SCRIPT;
use super::env::{env_enabled_default, read_env_trimmed};
use super::errors::RunTaskError;
use super::utils::{run_command_with_input_and_timeout, run_command_with_timeout, run_task_timeout, tail_string};

#[derive(Debug)]
pub(super) struct E2bRunOutput {
    pub ok: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub error: Option<String>,
    pub sandbox_id: Option<String>,
}

#[derive(Debug)]
pub(super) struct E2bTaskConfig {
    pub workspace_dir: PathBuf,
    pub command: String,
    pub env: HashMap<String, String>,
    pub sandbox_env: HashMap<String, String>,
    pub bootstrap: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub timeout: Duration,
    pub bootstrap_user: Option<String>,
    pub command_user: Option<String>,
}

#[derive(Debug)]
pub(super) struct E2bRunnerFiles {
    pub prompt_path: PathBuf,
    pub runner_path: PathBuf,
    pub askpass_path: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct E2bRunnerConfig {
    template_id: String,
    api_key: String,
    timeout_ms: Option<u64>,
    sandbox_env: Option<HashMap<String, String>>,
    metadata: Option<HashMap<String, String>>,
    remote_workspace: Option<String>,
    remote_tar_path: Option<String>,
    remote_output_tar: Option<String>,
    user: Option<String>,
    bootstrap_user: Option<String>,
    command_user: Option<String>,
    workspace_tar: Option<String>,
    local_output_tar: Option<String>,
    bootstrap: Option<Vec<String>>,
    command: String,
    env: Option<HashMap<String, String>>,
    bootstrap_timeout_ms: Option<u64>,
    command_timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct E2bRunnerResult {
    ok: bool,
    exit_code: i32,
    stdout: String,
    stderr: String,
    error: Option<String>,
    sandbox_id: Option<String>,
}

pub(super) fn use_e2b() -> bool {
    env_enabled_default("RUN_TASK_USE_E2B", true)
}

fn e2b_template_id() -> Result<String, RunTaskError> {
    read_env_trimmed("E2B_TEMPLATE_ID").ok_or(RunTaskError::MissingEnv {
        key: "E2B_TEMPLATE_ID",
    })
}

fn e2b_api_key() -> Result<String, RunTaskError> {
    read_env_trimmed("E2B_API_KEY").ok_or(RunTaskError::MissingEnv {
        key: "E2B_API_KEY",
    })
}

pub(super) fn prepare_runner_files(
    workspace_dir: &Path,
    prompt: &str,
) -> Result<E2bRunnerFiles, RunTaskError> {
    let runner_dir = workspace_dir.join(".dowhiz");
    fs::create_dir_all(&runner_dir)?;

    let prompt_path = runner_dir.join("prompt.txt");
    fs::write(&prompt_path, prompt)?;

    let runner_path = runner_dir.join("e2b_runner.mjs");
    if !runner_path.exists() {
        fs::write(&runner_path, E2B_RUNNER_SCRIPT)?;
    }

    let askpass_path = runner_dir.join("git-askpass.sh");
    if !askpass_path.exists() {
        fs::write(&askpass_path, GIT_ASKPASS_SCRIPT)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&askpass_path)?.permissions();
            perms.set_mode(0o700);
            fs::set_permissions(&askpass_path, perms)?;
        }
    }

    Ok(E2bRunnerFiles {
        prompt_path,
        runner_path,
        askpass_path,
    })
}

pub(super) fn run_e2b_task(config: E2bTaskConfig) -> Result<E2bRunOutput, RunTaskError> {
    ensure_e2b_runner_ready()?;

    let (template_id, api_key) = (e2b_template_id()?, e2b_api_key()?);
    let runner_dir = e2b_runner_dir();
    let runner_script = runner_dir.join("e2b_runner.mjs");

    let workspace_tar = NamedTempFile::new().map_err(RunTaskError::Io)?;
    let output_tar = NamedTempFile::new().map_err(RunTaskError::Io)?;

    create_workspace_tar(&config.workspace_dir, workspace_tar.path())?;

    let max_session_ms = e2b_max_session_ms();
    let mut timeout_ms = config.timeout.as_millis() as u64;
    if timeout_ms > max_session_ms {
        eprintln!(
            "[run_task] E2B timeout capped from {}ms to {}ms",
            timeout_ms, max_session_ms
        );
        timeout_ms = max_session_ms;
    }
    let mut bootstrap_timeout_ms = std::cmp::max(timeout_ms, 15 * 60 * 1000);
    if bootstrap_timeout_ms > max_session_ms {
        bootstrap_timeout_ms = max_session_ms;
    }

    let runner_config = E2bRunnerConfig {
        template_id,
        api_key,
        timeout_ms: Some(timeout_ms),
        sandbox_env: Some(config.sandbox_env.clone()),
        metadata: Some(config.metadata.clone()),
        remote_workspace: Some("/workspace".to_string()),
        remote_tar_path: Some("/tmp/workspace.tar".to_string()),
        remote_output_tar: Some("/tmp/workspace_out.tar".to_string()),
        user: Some("root".to_string()),
        bootstrap_user: config.bootstrap_user.clone(),
        command_user: config.command_user.clone(),
        workspace_tar: Some(workspace_tar.path().to_string_lossy().into_owned()),
        local_output_tar: Some(output_tar.path().to_string_lossy().into_owned()),
        bootstrap: Some(config.bootstrap.clone()),
        command: config.command,
        env: Some(config.env.clone()),
        bootstrap_timeout_ms: Some(bootstrap_timeout_ms),
        command_timeout_ms: Some(timeout_ms),
    };

    let payload = serde_json::to_vec(&runner_config)
        .map_err(|err| RunTaskError::Io(std::io::Error::new(std::io::ErrorKind::Other, err)))?;

    let mut cmd = Command::new("node");
    cmd.arg(&runner_script).current_dir(&runner_dir);
    let output = run_command_with_input_and_timeout(cmd, &payload, config.timeout, "node e2b_runner")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let mut combined = String::new();
    combined.push_str(&stdout);
    combined.push_str(&stderr);

    let parsed: E2bRunnerResult = serde_json::from_str(&stdout).map_err(|err| {
        RunTaskError::E2bRunnerFailed {
            output: tail_string(
                &format!("Failed to parse E2B runner output: {err}\n{combined}"),
                2000,
            ),
        }
    })?;

    if let Err(err) = extract_workspace_tar(output_tar.path(), &config.workspace_dir) {
        return Err(err);
    }

    Ok(E2bRunOutput {
        ok: parsed.ok,
        exit_code: parsed.exit_code,
        stdout: parsed.stdout,
        stderr: parsed.stderr,
        error: parsed.error,
        sandbox_id: parsed.sandbox_id,
    })
}

fn ensure_e2b_runner_ready() -> Result<(), RunTaskError> {
    if matches!(read_env_trimmed("E2B_RUNNER_SKIP_INSTALL").as_deref(), Some("1")) {
        return Ok(());
    }

    let runner_dir = e2b_runner_dir();
    let node_modules = runner_dir.join("node_modules").join("e2b");
    if node_modules.exists() {
        return Ok(());
    }

    let timeout = run_task_timeout();

    let mut cmd = Command::new("npm");
    cmd.arg("install").current_dir(&runner_dir);
    let output = match run_command_with_timeout(cmd, timeout, "npm install e2b_runner") {
        Ok(output) => output,
        Err(RunTaskError::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(RunTaskError::E2bRunnerNotFound { command: "npm" });
        }
        Err(err) => return Err(err),
    };

    if !output.status.success() {
        let mut combined = String::new();
        combined.push_str(&String::from_utf8_lossy(&output.stdout));
        combined.push_str(&String::from_utf8_lossy(&output.stderr));
        return Err(RunTaskError::E2bRunnerInstallFailed {
            output: tail_string(&combined, 2000),
        });
    }

    Ok(())
}

fn e2b_runner_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("e2b_runner")
}

fn e2b_max_session_ms() -> u64 {
    read_env_trimmed("E2B_MAX_SESSION_SECS")
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(3600)
        .saturating_mul(1000)
}

fn create_workspace_tar(workspace_dir: &Path, tar_path: &Path) -> Result<(), RunTaskError> {
    let tar_file = fs::File::create(tar_path)?;
    let mut builder = tar::Builder::new(tar_file);
    builder.append_dir_all(".", workspace_dir)?;
    builder.finish()?;
    Ok(())
}

fn extract_workspace_tar(tar_path: &Path, workspace_dir: &Path) -> Result<(), RunTaskError> {
    let tar_file = fs::File::open(tar_path)?;
    let mut archive = tar::Archive::new(tar_file);
    archive.unpack(workspace_dir)?;
    Ok(())
}

pub(super) fn default_bootstrap_commands() -> Vec<String> {
    vec![
        "set -e".to_string(),
        "NODE_MAJOR=\"\"; \
        if command -v node >/dev/null 2>&1; then NODE_MAJOR=$(node -v | sed 's/^v//' | cut -d. -f1); fi; \
        if [ -z \"$NODE_MAJOR\" ] || [ \"$NODE_MAJOR\" -lt 18 ]; then \
          if command -v apt-get >/dev/null 2>&1; then \
            apt-get update && apt-get install -y curl ca-certificates; \
            curl -fsSL https://deb.nodesource.com/setup_20.x | bash -; \
            apt-get install -y nodejs; \
          elif command -v yum >/dev/null 2>&1; then \
            yum install -y curl ca-certificates; \
            curl -fsSL https://rpm.nodesource.com/setup_20.x | bash -; \
            yum install -y nodejs; \
          elif command -v apk >/dev/null 2>&1; then \
            apk add --no-cache nodejs npm; \
          else \
            echo \"node not found\" >&2; exit 1; \
          fi; \
        fi"
        .to_string(),
        "if ! command -v npm >/dev/null 2>&1; then \
          if command -v apt-get >/dev/null 2>&1; then apt-get update && apt-get install -y npm; \
          elif command -v yum >/dev/null 2>&1; then yum install -y npm; \
          elif command -v apk >/dev/null 2>&1; then apk add --no-cache npm; \
          else echo \"npm not found\" >&2; exit 1; fi; \
        fi"
        .to_string(),
        "if ! command -v git >/dev/null 2>&1; then \
          if command -v apt-get >/dev/null 2>&1; then apt-get update && apt-get install -y git; \
          elif command -v yum >/dev/null 2>&1; then yum install -y git; \
          elif command -v apk >/dev/null 2>&1; then apk add --no-cache git; \
          else echo \"git not installed\"; fi; \
        fi"
        .to_string(),
        "if ! command -v codex >/dev/null 2>&1; then npm i -g @openai/codex@latest; fi"
            .to_string(),
        "if ! command -v claude >/dev/null 2>&1; then npm i -g @anthropic-ai/claude-code@latest; fi"
            .to_string(),
        "if ! command -v gh >/dev/null 2>&1; then \
          if command -v apt-get >/dev/null 2>&1; then apt-get update && apt-get install -y gh; \
          elif command -v yum >/dev/null 2>&1; then yum install -y gh; \
          elif command -v apk >/dev/null 2>&1; then apk add --no-cache github-cli; \
          else echo \"gh not installed\"; fi; \
        fi"
        .to_string(),
        "if command -v gh >/dev/null 2>&1; then \
          if ! gh auth status --hostname github.com >/dev/null 2>&1; then \
            if [ -n \"$GH_TOKEN\" ]; then \
              echo \"$GH_TOKEN\" | gh auth login --with-token --hostname github.com --git-protocol https --insecure-storage || true; \
              gh auth setup-git --hostname github.com || true; \
            fi; \
          fi; \
        fi"
        .to_string(),
        "chmod -R a+rwx /workspace || true".to_string(),
    ]
}

const E2B_RUNNER_SCRIPT: &str = r#"import { spawnSync } from 'node:child_process';
import fs from 'node:fs';

const promptPath = process.env.DOWHIZ_PROMPT_PATH || '/workspace/.dowhiz/prompt.txt';
const command = process.env.DOWHIZ_RUN_CMD || 'codex';
const argsJson = process.env.DOWHIZ_RUN_ARGS || '[]';
const workdir = process.env.DOWHIZ_WORKDIR || '/workspace';

let args = [];
try {
  args = JSON.parse(argsJson);
} catch (err) {
  console.error('Failed to parse DOWHIZ_RUN_ARGS', err);
  process.exit(2);
}

let prompt = '';
try {
  prompt = fs.readFileSync(promptPath, 'utf8');
} catch (err) {
  console.error('Failed to read prompt file', err);
  process.exit(2);
}

const finalArgs = [...args, prompt];
const result = spawnSync(command, finalArgs, {
  cwd: workdir,
  env: process.env,
  stdio: 'inherit',
});

process.exit(result.status ?? 1);
"#;
