use serde::Deserialize;
use std::env;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

const CODEX_CONFIG_MARKER: &str = "# IMPORTANT: Use your Azure *deployment name* here";
const CODEX_CONFIG_BLOCK_TEMPLATE: &str = r#"# IMPORTANT: Use your Azure *deployment name* here (e.g., "gpt-5.2-codex")
model = "{model_name}"
model_provider = "azure"
model_reasoning_effort = "xhigh"
web_search = "live"
ask_for_approval = "never"
sandbox = "{sandbox_mode}"

[model_providers.azure]
name = "Azure OpenAI"
base_url = "{azure_endpoint}"
env_key = "AZURE_OPENAI_API_KEY_BACKUP"
wire_api = "responses"
"#;
const DOCKER_WORKSPACE_DIR: &str = "/workspace";
const DOCKER_CODEX_HOME_DIR: &str = ".codex";
const SCHEDULED_TASKS_BEGIN: &str = "SCHEDULED_TASKS_JSON_BEGIN";
const SCHEDULED_TASKS_END: &str = "SCHEDULED_TASKS_JSON_END";
const SCHEDULER_ACTIONS_BEGIN: &str = "SCHEDULER_ACTIONS_JSON_BEGIN";
const SCHEDULER_ACTIONS_END: &str = "SCHEDULER_ACTIONS_JSON_END";
const GIT_ASKPASS_SCRIPT: &str = r#"#!/bin/sh
case "$1" in
  *Username*)
    if [ -n "$GITHUB_USERNAME" ]; then
      printf "%s" "$GITHUB_USERNAME"
    elif [ -n "$USER" ]; then
      printf "%s" "$USER"
    else
      printf "%s" "x-access-token"
    fi
    ;;
  *Password*)
    if [ -n "$GH_TOKEN" ]; then
      printf "%s" "$GH_TOKEN"
    elif [ -n "$GITHUB_TOKEN" ]; then
      printf "%s" "$GITHUB_TOKEN"
    elif [ -n "$GITHUB_PERSONAL_ACCESS_TOKEN" ]; then
      printf "%s" "$GITHUB_PERSONAL_ACCESS_TOKEN"
    fi
    ;;
  *)
    ;;
esac
exit 0
"#;
static GH_AUTH_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct RunTaskParams {
    pub workspace_dir: PathBuf,
    pub input_email_dir: PathBuf,
    pub input_attachments_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub reference_dir: PathBuf,
    pub reply_to: Vec<String>,
    pub model_name: String,
    pub codex_disabled: bool,
}

#[derive(Debug, Clone)]
struct RunTaskRequest<'a> {
    workspace_dir: &'a Path,
    input_email_dir: &'a Path,
    input_attachments_dir: &'a Path,
    memory_dir: &'a Path,
    reference_dir: &'a Path,
    model_name: &'a str,
    reply_to: &'a [String],
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScheduledTaskRequest {
    SendEmail(ScheduledSendEmailTask),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum SchedulerActionRequest {
    Cancel {
        task_ids: Vec<String>,
    },
    Reschedule {
        task_id: String,
        schedule: ScheduleRequest,
    },
    CreateRunTask {
        schedule: ScheduleRequest,
        #[serde(default)]
        model_name: Option<String>,
        #[serde(default)]
        codex_disabled: Option<bool>,
        #[serde(default)]
        reply_to: Vec<String>,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScheduleRequest {
    Cron { expression: String },
    OneShot { run_at: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledSendEmailTask {
    pub subject: String,
    pub html_path: String,
    pub attachments_dir: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub delay_minutes: Option<i64>,
    pub delay_seconds: Option<i64>,
    pub run_at: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ScheduledTasksBlock {
    List(Vec<ScheduledTaskRequest>),
    Wrapper { tasks: Vec<ScheduledTaskRequest> },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SchedulerActionsBlock {
    List(Vec<SchedulerActionRequest>),
    Wrapper {
        actions: Vec<SchedulerActionRequest>,
    },
}

#[derive(Debug, Clone)]
pub struct RunTaskOutput {
    pub reply_html_path: PathBuf,
    pub reply_attachments_dir: PathBuf,
    pub codex_output: String,
    pub scheduled_tasks: Vec<ScheduledTaskRequest>,
    pub scheduled_tasks_error: Option<String>,
    pub scheduler_actions: Vec<SchedulerActionRequest>,
    pub scheduler_actions_error: Option<String>,
}

#[derive(Debug)]
struct GitHubAuthConfig {
    env_overrides: Vec<(String, String)>,
    askpass_path: Option<PathBuf>,
    token: Option<String>,
    #[allow(dead_code)]
    username: Option<String>,
}

#[derive(Debug)]
pub enum RunTaskError {
    Io(io::Error),
    MissingEnv {
        key: &'static str,
    },
    InvalidPath {
        label: &'static str,
        path: PathBuf,
        reason: &'static str,
    },
    CodexNotFound,
    CodexFailed {
        status: Option<i32>,
        output: String,
    },
    DockerNotFound,
    DockerFailed {
        status: Option<i32>,
        output: String,
    },
    GitHubAuthCommandNotFound {
        command: &'static str,
    },
    GitHubAuthFailed {
        command: &'static str,
        status: Option<i32>,
        output: String,
    },
    OutputMissing {
        path: PathBuf,
        output: String,
    },
}

impl fmt::Display for RunTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunTaskError::Io(err) => write!(f, "I/O error: {}", err),
            RunTaskError::MissingEnv { key } => {
                write!(f, "Missing required environment variable: {}", key)
            }
            RunTaskError::InvalidPath {
                label,
                path,
                reason,
            } => {
                write!(f, "Invalid {} path ({}): {}", label, path.display(), reason)
            }
            RunTaskError::CodexNotFound => write!(f, "Codex CLI not found on PATH."),
            RunTaskError::CodexFailed { status, output } => write!(
                f,
                "Codex CLI failed (status: {:?}). Output tail:\n{}",
                status, output
            ),
            RunTaskError::DockerNotFound => write!(f, "Docker CLI not found on PATH."),
            RunTaskError::DockerFailed { status, output } => write!(
                f,
                "Docker run failed (status: {:?}). Output tail:\n{}",
                status, output
            ),
            RunTaskError::GitHubAuthCommandNotFound { command } => {
                write!(f, "GitHub auth command not found on PATH: {}", command)
            }
            RunTaskError::GitHubAuthFailed {
                command,
                status,
                output,
            } => write!(
                f,
                "GitHub auth command failed ({} status: {:?}). Output tail:\n{}",
                command, status, output
            ),
            RunTaskError::OutputMissing { path, output } => {
                write!(
                    f,
                    "Expected output not found: {}\nCodex output tail:\n{}",
                    path.display(),
                    output
                )
            }
        }
    }
}

impl std::error::Error for RunTaskError {}

impl From<io::Error> for RunTaskError {
    fn from(err: io::Error) -> Self {
        RunTaskError::Io(err)
    }
}

pub fn run_task(params: &RunTaskParams) -> Result<RunTaskOutput, RunTaskError> {
    let workspace_dir = remap_workspace_dir(&params.workspace_dir)?;
    let request = RunTaskRequest {
        workspace_dir: &workspace_dir,
        input_email_dir: &params.input_email_dir,
        input_attachments_dir: &params.input_attachments_dir,
        memory_dir: &params.memory_dir,
        reference_dir: &params.reference_dir,
        model_name: params.model_name.as_str(),
        reply_to: &params.reply_to,
    };

    let (reply_html_path, reply_attachments_dir) = prepare_workspace(&request)?;

    if params.codex_disabled {
        if !params.reply_to.is_empty() {
            write_placeholder_reply(&reply_html_path)?;
        }
        return Ok(RunTaskOutput {
            reply_html_path,
            reply_attachments_dir,
            codex_output: "codex disabled".to_string(),
            scheduled_tasks: Vec::new(),
            scheduled_tasks_error: None,
            scheduler_actions: Vec::new(),
            scheduler_actions_error: None,
        });
    }

    run_codex_task(request, reply_html_path, reply_attachments_dir)
}

fn run_codex_task(
    request: RunTaskRequest<'_>,
    reply_html_path: PathBuf,
    reply_attachments_dir: PathBuf,
) -> Result<RunTaskOutput, RunTaskError> {
    load_env_sources(request.workspace_dir)?;
    let docker_image = read_env_trimmed("RUN_TASK_DOCKER_IMAGE");
    let use_docker = docker_image.is_some() || env_enabled("RUN_TASK_USE_DOCKER");
    let docker_image = if use_docker {
        docker_image.ok_or(RunTaskError::MissingEnv {
            key: "RUN_TASK_DOCKER_IMAGE",
        })?
    } else {
        String::new()
    };
    let host_workspace_dir = if use_docker {
        Some(canonicalize_dir(request.workspace_dir)?)
    } else {
        None
    };
    let askpass_dir = if use_docker {
        host_workspace_dir
            .as_ref()
            .map(|dir| dir.join(DOCKER_CODEX_HOME_DIR))
    } else {
        None
    };
    let github_auth = resolve_github_auth(askpass_dir.as_deref())?;

    let api_key =
        env::var("AZURE_OPENAI_API_KEY_BACKUP").map_err(|_| RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        })?;
    if api_key.trim().is_empty() {
        return Err(RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        });
    }
    let azure_endpoint =
        env::var("AZURE_OPENAI_ENDPOINT_BACKUP").map_err(|_| RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_ENDPOINT_BACKUP",
        })?;
    if azure_endpoint.trim().is_empty() {
        return Err(RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_ENDPOINT_BACKUP",
        });
    }

    let model_name = if request.model_name.trim().is_empty() {
        env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.2-codex".to_string())
    } else {
        request.model_name.to_string()
    };

    let sandbox_mode = codex_sandbox_mode();
    let bypass_sandbox = codex_bypass_sandbox() || use_docker;
    if use_docker {
        let codex_home = host_workspace_dir
            .as_ref()
            .map(|dir| dir.join(DOCKER_CODEX_HOME_DIR))
            .unwrap_or_else(|| request.workspace_dir.join(DOCKER_CODEX_HOME_DIR));
        ensure_codex_config_at(
            &model_name,
            &azure_endpoint,
            &codex_home,
            Path::new(DOCKER_WORKSPACE_DIR),
            &sandbox_mode,
        )?;
    } else {
        ensure_codex_config(
            &model_name,
            &azure_endpoint,
            request.workspace_dir,
            &sandbox_mode,
        )?;
    }
    ensure_github_cli_auth(&github_auth)?;

    let memory_context = load_memory_context(request.workspace_dir, request.memory_dir)?;
    let prompt = build_prompt(
        request.input_email_dir,
        request.input_attachments_dir,
        request.memory_dir,
        request.reference_dir,
        &memory_context,
        !request.reply_to.is_empty(),
    );

    let output = if use_docker {
        ensure_docker_image_available(&docker_image)?;
        let host_workspace_dir = host_workspace_dir
            .as_ref()
            .ok_or(RunTaskError::MissingEnv {
                key: "RUN_TASK_DOCKER_IMAGE",
            })?;
        let askpass_container_path = github_auth
            .askpass_path
            .as_ref()
            .and_then(|path| workspace_path_in_container(path, host_workspace_dir));

        if github_auth.askpass_path.is_some() && askpass_container_path.is_none() {
            return Err(RunTaskError::InvalidPath {
                label: "git_askpass_path",
                path: github_auth
                    .askpass_path
                    .clone()
                    .unwrap_or_else(|| host_workspace_dir.join("missing")),
                reason: "askpass path is not within workspace_dir",
            });
        }

        let mut cmd = Command::new("docker");
        cmd.arg("run")
            .arg("--rm")
            .arg("--workdir")
            .arg(DOCKER_WORKSPACE_DIR)
            .arg("-v")
            .arg(format!(
                "{}:{}",
                host_workspace_dir.display(),
                DOCKER_WORKSPACE_DIR
            ))
            .arg("-e")
            .arg(format!("HOME={}", DOCKER_WORKSPACE_DIR))
            .arg("-e")
            .arg(format!(
                "CODEX_HOME={}/{}",
                DOCKER_WORKSPACE_DIR, DOCKER_CODEX_HOME_DIR
            ))
            .arg("-e")
            .arg(format!("AZURE_OPENAI_API_KEY_BACKUP={}", api_key))
            .arg("-e")
            .arg(format!("AZURE_OPENAI_ENDPOINT_BACKUP={}", azure_endpoint));
        for (key, value) in &github_auth.env_overrides {
            cmd.arg("-e").arg(format!("{}={}", key, value));
        }
        if let Some(container_path) = askpass_container_path {
            cmd.arg("-e")
                .arg(format!("GIT_ASKPASS={}", container_path.display()))
                .arg("-e")
                .arg("GIT_TERMINAL_PROMPT=0");
        }
        if let Some(network) = read_env_trimmed("RUN_TASK_DOCKER_NETWORK") {
            cmd.arg("--network").arg(network);
        }
        for dns in read_env_list("RUN_TASK_DOCKER_DNS") {
            cmd.arg("--dns").arg(dns);
        }
        for search_domain in read_env_list("RUN_TASK_DOCKER_DNS_SEARCH") {
            cmd.arg("--dns-search").arg(search_domain);
        }
        cmd.arg("--entrypoint")
            .arg("codex")
            .arg(&docker_image)
            .arg("exec");
        if bypass_sandbox {
            cmd.arg("--yolo");
        }
        cmd.arg("--skip-git-repo-check")
            .arg("-m")
            .arg(&model_name)
            .arg("-c")
            .arg("web_search=\"live\"")
            .arg("-c")
            .arg("ask_for_approval=\"never\"")
            .arg("-c")
            .arg(format!("sandbox=\"{}\"", sandbox_mode))
            .arg("-c")
            .arg("model_providers.azure.env_key=\"AZURE_OPENAI_API_KEY_BACKUP\"")
            .arg("--cd")
            .arg(DOCKER_WORKSPACE_DIR)
            .arg(prompt);

        match cmd.output() {
            Ok(output) => output,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Err(RunTaskError::DockerNotFound)
            }
            Err(err) => return Err(RunTaskError::Io(err)),
        }
    } else {
        let mut cmd = Command::new("codex");
        cmd.arg("exec");
        if bypass_sandbox {
            cmd.arg("--yolo");
        }
        cmd.arg("--skip-git-repo-check")
            .arg("-m")
            .arg(&model_name)
            .arg("-c")
            .arg("web_search=\"live\"")
            .arg("-c")
            .arg("ask_for_approval=\"never\"")
            .arg("-c")
            .arg(format!("sandbox=\"{}\"", sandbox_mode))
            .arg("-c")
            .arg("model_providers.azure.env_key=\"AZURE_OPENAI_API_KEY_BACKUP\"")
            .arg("--cd")
            .arg(request.workspace_dir)
            .arg(prompt)
            .env("AZURE_OPENAI_API_KEY_BACKUP", api_key)
            .current_dir(request.workspace_dir);
        for (key, value) in github_auth.env_overrides {
            cmd.env(key, value);
        }
        if let Some(askpass_path) = github_auth.askpass_path {
            cmd.env("GIT_ASKPASS", askpass_path);
            cmd.env("GIT_TERMINAL_PROMPT", "0");
        }

        match cmd.output() {
            Ok(output) => output,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return Err(RunTaskError::CodexNotFound)
            }
            Err(err) => return Err(RunTaskError::Io(err)),
        }
    };

    let mut combined_output = String::new();
    combined_output.push_str(&String::from_utf8_lossy(&output.stdout));
    combined_output.push_str(&String::from_utf8_lossy(&output.stderr));
    let (scheduled_tasks, scheduled_tasks_error) = extract_scheduled_tasks(&combined_output);
    let (scheduler_actions, scheduler_actions_error) = extract_scheduler_actions(&combined_output);
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(if use_docker {
            RunTaskError::DockerFailed {
                status: output.status.code(),
                output: output_tail,
            }
        } else {
            RunTaskError::CodexFailed {
                status: output.status.code(),
                output: output_tail,
            }
        });
    }

    if !reply_html_path.exists() {
        return Err(RunTaskError::OutputMissing {
            path: reply_html_path,
            output: output_tail,
        });
    }

    Ok(RunTaskOutput {
        reply_html_path,
        reply_attachments_dir,
        codex_output: output_tail,
        scheduled_tasks,
        scheduled_tasks_error,
        scheduler_actions,
        scheduler_actions_error,
    })
}

fn remap_workspace_dir(workspace_dir: &Path) -> Result<PathBuf, RunTaskError> {
    if !workspace_dir.is_absolute() {
        return Ok(workspace_dir.to_path_buf());
    }

    let home = env::var("HOME").map_err(|_| RunTaskError::MissingEnv { key: "HOME" })?;
    let new_users_root = PathBuf::from(&home)
        .join(".dowhiz")
        .join("DoWhiz")
        .join("run_task")
        .join("users");
    if workspace_dir.starts_with(&new_users_root) {
        return Ok(workspace_dir.to_path_buf());
    }

    let relative = match legacy_workspace_relative(workspace_dir, &home) {
        Some(relative) => relative,
        None => return Ok(workspace_dir.to_path_buf()),
    };
    let remapped = new_users_root.join(relative);

    if workspace_dir.exists() && !remapped.exists() {
        if let Some(parent) = remapped.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(workspace_dir, &remapped)?;
    }

    Ok(remapped)
}

fn legacy_workspace_relative(workspace_dir: &Path, home: &str) -> Option<PathBuf> {
    let legacy_roots = [
        PathBuf::from(home)
            .join("Documents")
            .join("GitHub_MacBook")
            .join("DoWhiz")
            .join("DoWhiz_service")
            .join(".workspace")
            .join("run_task")
            .join("users"),
        PathBuf::from(home)
            .join("Documents")
            .join("GitHub_MacBook")
            .join("DoWhiz")
            .join(".workspace")
            .join("run_task")
            .join("users"),
        PathBuf::from(home)
            .join(".dowhiz")
            .join("DoWhiz")
            .join("DoWhiz_service")
            .join(".workspace")
            .join("run_task")
            .join("users"),
    ];

    for root in legacy_roots {
        if workspace_dir.starts_with(&root) {
            return workspace_dir
                .strip_prefix(&root)
                .ok()
                .map(|path| path.to_path_buf());
        }
    }

    let path_str = workspace_dir.to_string_lossy();
    let marker = "/.workspace/run_task/users/";
    path_str
        .find(marker)
        .map(|idx| PathBuf::from(&path_str[idx + marker.len()..]))
}

fn prepare_workspace(request: &RunTaskRequest<'_>) -> Result<(PathBuf, PathBuf), RunTaskError> {
    ensure_workspace_dir(request.workspace_dir)?;

    let _input_email_dir = resolve_rel_dir(
        request.workspace_dir,
        request.input_email_dir,
        "input_email_dir",
    )?;
    let _input_attachments_dir = resolve_rel_dir(
        request.workspace_dir,
        request.input_attachments_dir,
        "input_attachments_dir",
    )?;
    let _memory_dir = resolve_rel_dir(request.workspace_dir, request.memory_dir, "memory_dir")?;
    let _reference_dir = resolve_rel_dir(
        request.workspace_dir,
        request.reference_dir,
        "reference_dir",
    )?;

    let reply_html_path = request.workspace_dir.join("reply_email_draft.html");
    let reply_attachments_dir = request.workspace_dir.join("reply_email_attachments");
    ensure_dir_exists(&reply_attachments_dir, "reply_attachments_dir")?;

    Ok((reply_html_path, reply_attachments_dir))
}

fn write_placeholder_reply(path: &Path) -> Result<(), RunTaskError> {
    let placeholder = "<html><body><p>Codex disabled. Received your email.</p></body></html>";
    fs::write(path, placeholder)?;
    Ok(())
}

fn ensure_workspace_dir(path: &Path) -> Result<(), RunTaskError> {
    if path.exists() && !path.is_dir() {
        return Err(RunTaskError::InvalidPath {
            label: "workspace_dir",
            path: path.to_path_buf(),
            reason: "path exists but is not a directory",
        });
    }
    fs::create_dir_all(path)?;
    Ok(())
}

fn ensure_dir_exists(path: &Path, label: &'static str) -> Result<(), RunTaskError> {
    if path.exists() && !path.is_dir() {
        return Err(RunTaskError::InvalidPath {
            label,
            path: path.to_path_buf(),
            reason: "path exists but is not a directory",
        });
    }
    fs::create_dir_all(path)?;
    Ok(())
}

fn canonicalize_dir(path: &Path) -> Result<PathBuf, RunTaskError> {
    fs::canonicalize(path).map_err(RunTaskError::Io)
}

fn workspace_path_in_container(path: &Path, host_workspace_dir: &Path) -> Option<PathBuf> {
    let relative = path.strip_prefix(host_workspace_dir).ok()?;
    Some(Path::new(DOCKER_WORKSPACE_DIR).join(relative))
}

fn ensure_docker_image_available(image: &str) -> Result<(), RunTaskError> {
    if docker_image_exists(image)? {
        return Ok(());
    }

    if !env_enabled_default("RUN_TASK_DOCKER_AUTO_BUILD", true) {
        return Err(RunTaskError::DockerFailed {
            status: None,
            output: format!(
                "docker image '{}' not found and auto-build disabled (set RUN_TASK_DOCKER_AUTO_BUILD=1)",
                image
            ),
        });
    }

    let (dockerfile, context) = resolve_docker_build_paths()?;
    let mut cmd = Command::new("docker");
    cmd.args([
        "build",
        "-t",
        image,
        "-f",
        dockerfile.to_string_lossy().as_ref(),
        context.to_string_lossy().as_ref(),
    ]);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::DockerNotFound)
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };
    let mut combined_output = String::new();
    combined_output.push_str(&String::from_utf8_lossy(&output.stdout));
    combined_output.push_str(&String::from_utf8_lossy(&output.stderr));
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(RunTaskError::DockerFailed {
            status: output.status.code(),
            output: output_tail,
        });
    }

    Ok(())
}

fn docker_image_exists(image: &str) -> Result<bool, RunTaskError> {
    let output = match Command::new("docker")
        .args(["image", "inspect", image])
        .output()
    {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::DockerNotFound)
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };

    Ok(output.status.success())
}

fn resolve_docker_build_paths() -> Result<(PathBuf, PathBuf), RunTaskError> {
    let cwd = env::current_dir().map_err(RunTaskError::Io)?;
    let dockerfile = if let Some(path) = resolve_env_path("RUN_TASK_DOCKERFILE", &cwd) {
        path
    } else {
        let candidate = cwd.join("Dockerfile");
        if candidate.exists() {
            candidate
        } else {
            let candidate = cwd.join("..").join("Dockerfile");
            if candidate.exists() {
                candidate
            } else {
                return Err(RunTaskError::InvalidPath {
                    label: "dockerfile",
                    path: cwd.join("Dockerfile"),
                    reason: "Dockerfile not found; set RUN_TASK_DOCKERFILE",
                });
            }
        }
    };

    let context = if let Some(path) = resolve_env_path("RUN_TASK_DOCKER_BUILD_CONTEXT", &cwd) {
        path
    } else {
        dockerfile
            .parent()
            .map(PathBuf::from)
            .ok_or(RunTaskError::InvalidPath {
                label: "docker_build_context",
                path: dockerfile.clone(),
                reason: "could not resolve Dockerfile directory",
            })?
    };

    Ok((dockerfile, context))
}

fn resolve_rel_dir(
    workspace_dir: &Path,
    rel_dir: &Path,
    label: &'static str,
) -> Result<PathBuf, RunTaskError> {
    if rel_dir.is_absolute() {
        return Err(RunTaskError::InvalidPath {
            label,
            path: rel_dir.to_path_buf(),
            reason: "path must be relative to workspace_dir",
        });
    }
    let resolved = workspace_dir.join(rel_dir);
    if !resolved.exists() {
        return Err(RunTaskError::InvalidPath {
            label,
            path: resolved,
            reason: "directory does not exist",
        });
    }
    if !resolved.is_dir() {
        return Err(RunTaskError::InvalidPath {
            label,
            path: resolved,
            reason: "path is not a directory",
        });
    }
    Ok(resolved)
}

fn load_env_sources(workspace_dir: &Path) -> Result<(), RunTaskError> {
    if let Some(env_path) = find_env_file(workspace_dir) {
        load_env_file(&env_path)?;
    }
    Ok(())
}

fn find_env_file(workspace_dir: &Path) -> Option<PathBuf> {
    for ancestor in workspace_dir.ancestors() {
        let candidate = ancestor.join(".env");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    if let Ok(cwd) = env::current_dir() {
        for ancestor in cwd.ancestors() {
            let candidate = ancestor.join(".env");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }
    None
}

fn load_env_file(path: &Path) -> Result<(), RunTaskError> {
    let content = fs::read_to_string(path)?;
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(2, '=');
        let key = match parts.next() {
            Some(key) => key.trim(),
            None => continue,
        };
        let value = match parts.next() {
            Some(value) => value.trim(),
            None => continue,
        };
        if key.is_empty() {
            continue;
        }
        if env::var_os(key).is_none() {
            let value = unquote_env_value(value);
            env::set_var(key, value);
        }
    }
    Ok(())
}

fn unquote_env_value(value: &str) -> &str {
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return &value[1..value.len() - 1];
        }
    }
    value
}

fn resolve_github_auth(askpass_dir: Option<&Path>) -> Result<GitHubAuthConfig, RunTaskError> {
    let gh_token = read_env_trimmed("GH_TOKEN");
    let github_token = read_env_trimmed("GITHUB_TOKEN");
    let pat_token = read_env_trimmed("GITHUB_PERSONAL_ACCESS_TOKEN");
    let token = gh_token.clone().or(github_token.clone()).or(pat_token);
    let github_username = read_env_trimmed("GITHUB_USERNAME")
        .or_else(|| read_env_trimmed("USER"))
        .or_else(|| read_env_trimmed("USERNAME"));

    let mut env_overrides = Vec::new();
    if env_missing_or_empty("GH_PROMPT_DISABLED") {
        env_overrides.push(("GH_PROMPT_DISABLED".to_string(), "1".to_string()));
    }
    if env_missing_or_empty("GH_NO_UPDATE_NOTIFIER") {
        env_overrides.push(("GH_NO_UPDATE_NOTIFIER".to_string(), "1".to_string()));
    }
    if env_missing_or_empty("GIT_EDITOR") {
        env_overrides.push(("GIT_EDITOR".to_string(), "true".to_string()));
    }
    if env_missing_or_empty("VISUAL") {
        env_overrides.push(("VISUAL".to_string(), "true".to_string()));
    }
    if env_missing_or_empty("EDITOR") {
        env_overrides.push(("EDITOR".to_string(), "true".to_string()));
    }
    if let Some(token) = token.clone() {
        if env_missing_or_empty("GH_TOKEN") {
            env_overrides.push(("GH_TOKEN".to_string(), token.clone()));
        }
        if env_missing_or_empty("GITHUB_TOKEN") {
            env_overrides.push(("GITHUB_TOKEN".to_string(), token.clone()));
        }
    }
    if let Some(username) = github_username.clone() {
        let email = format!("{}@users.noreply.github.com", username);
        if env_missing_or_empty("GIT_AUTHOR_NAME") {
            env_overrides.push(("GIT_AUTHOR_NAME".to_string(), username.clone()));
        }
        if env_missing_or_empty("GIT_COMMITTER_NAME") {
            env_overrides.push(("GIT_COMMITTER_NAME".to_string(), username.clone()));
        }
        if env_missing_or_empty("GIT_AUTHOR_EMAIL") {
            env_overrides.push(("GIT_AUTHOR_EMAIL".to_string(), email.clone()));
        }
        if env_missing_or_empty("GIT_COMMITTER_EMAIL") {
            env_overrides.push(("GIT_COMMITTER_EMAIL".to_string(), email));
        }
    }

    let askpass_path = if token.is_some() {
        let target_dir = askpass_dir
            .map(PathBuf::from)
            .unwrap_or_else(env::temp_dir);
        Some(write_git_askpass_script_in(&target_dir)?)
    } else {
        None
    };

    Ok(GitHubAuthConfig {
        env_overrides,
        askpass_path,
        token,
        username: github_username,
    })
}

fn is_keyring_error(output: &str) -> bool {
    let normalized = output.to_ascii_lowercase();
    normalized.contains("keyring")
        || normalized.contains("keychain")
        || normalized.contains("credential store")
        || normalized.contains("user interaction is not allowed")
}

fn gh_auth_status_ok(github_auth: &GitHubAuthConfig) -> Result<bool, RunTaskError> {
    let mut status_cmd = Command::new("gh");
    status_cmd.args(["auth", "status", "--hostname", "github.com"]);
    apply_env_overrides(&mut status_cmd, &github_auth.env_overrides, &[]);
    match run_auth_command(status_cmd, None, "gh auth status") {
        Ok(()) => Ok(true),
        Err(RunTaskError::GitHubAuthFailed { .. }) => Ok(false),
        Err(err) => Err(err),
    }
}

fn gh_auth_login(
    github_auth: &GitHubAuthConfig,
    token: &str,
    insecure_storage: bool,
) -> Result<(), RunTaskError> {
    let mut login_cmd = Command::new("gh");
    login_cmd.args([
        "auth",
        "login",
        "--with-token",
        "--hostname",
        "github.com",
        "--git-protocol",
        "https",
    ]);
    if insecure_storage {
        login_cmd.arg("--insecure-storage");
    }
    login_cmd.env_remove("GH_TOKEN").env_remove("GITHUB_TOKEN");
    apply_env_overrides(
        &mut login_cmd,
        &github_auth.env_overrides,
        &["GH_TOKEN", "GITHUB_TOKEN"],
    );
    run_auth_command(login_cmd, Some(token), "gh auth login")
}

fn ensure_github_cli_auth(github_auth: &GitHubAuthConfig) -> Result<(), RunTaskError> {
    if env_enabled("GH_AUTH_DISABLED") || env_enabled("GITHUB_AUTH_DISABLED") {
        return Ok(());
    }
    let Some(token) = github_auth.token.as_deref() else {
        return Ok(());
    };

    let auth_lock = GH_AUTH_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = auth_lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    if gh_auth_status_ok(github_auth)? {
        return Ok(());
    }

    match gh_auth_login(github_auth, token, false) {
        Ok(()) => {}
        Err(RunTaskError::GitHubAuthFailed { output, .. }) if is_keyring_error(&output) => {
            gh_auth_login(github_auth, token, true)?;
        }
        Err(err) => return Err(err),
    }

    let mut setup_cmd = Command::new("gh");
    setup_cmd.args(["auth", "setup-git", "--hostname", "github.com"]);
    apply_env_overrides(&mut setup_cmd, &github_auth.env_overrides, &[]);
    run_auth_command(setup_cmd, None, "gh auth setup-git")?;

    let mut status_cmd = Command::new("gh");
    status_cmd.args(["auth", "status", "--hostname", "github.com"]);
    apply_env_overrides(&mut status_cmd, &github_auth.env_overrides, &[]);
    run_auth_command(status_cmd, None, "gh auth status")?;

    Ok(())
}

fn apply_env_overrides(cmd: &mut Command, overrides: &[(String, String)], skip: &[&str]) {
    for (key, value) in overrides {
        if skip.iter().any(|blocked| *blocked == key.as_str()) {
            continue;
        }
        cmd.env(key, value);
    }
}

fn run_auth_command(
    mut cmd: Command,
    input: Option<&str>,
    label: &'static str,
) -> Result<(), RunTaskError> {
    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::GitHubAuthCommandNotFound { command: "gh" })
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };

    if let Some(payload) = input {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(payload.as_bytes())?;
            stdin.write_all(b"\n")?;
        }
    }

    let output = child.wait_with_output()?;
    let mut combined_output = String::new();
    combined_output.push_str(&String::from_utf8_lossy(&output.stdout));
    combined_output.push_str(&String::from_utf8_lossy(&output.stderr));
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(RunTaskError::GitHubAuthFailed {
            command: label,
            status: output.status.code(),
            output: output_tail,
        });
    }

    Ok(())
}

fn read_env_trimmed(key: &str) -> Option<String> {
    let value = env::var(key).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(unquote_env_value(trimmed).to_string())
    }
}

fn read_env_list(key: &str) -> Vec<String> {
    read_env_trimmed(key)
        .map(|value| {
            value
                .split(|ch: char| ch == ',' || ch.is_whitespace())
                .filter_map(|item| {
                    let trimmed = item.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed.to_string())
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn env_missing_or_empty(key: &str) -> bool {
    match env::var(key) {
        Ok(value) => value.trim().is_empty(),
        Err(_) => true,
    }
}

fn env_enabled(key: &str) -> bool {
    match env::var(key) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !(normalized.is_empty() || normalized == "0" || normalized == "false")
        }
        Err(_) => false,
    }
}

fn env_enabled_default(key: &str, default_value: bool) -> bool {
    match env::var(key) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !(normalized.is_empty() || normalized == "0" || normalized == "false")
        }
        Err(_) => default_value,
    }
}

fn resolve_env_path(key: &str, cwd: &Path) -> Option<PathBuf> {
    let value = env::var(key).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let path = PathBuf::from(trimmed);
    if path.is_absolute() {
        Some(path)
    } else {
        Some(cwd.join(path))
    }
}

fn write_git_askpass_script_in(dir: &Path) -> Result<PathBuf, RunTaskError> {
    fs::create_dir_all(dir)?;
    let mut path = dir.to_path_buf();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    path.push(format!(
        "dowhiz-git-askpass-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::write(&path, GIT_ASKPASS_SCRIPT)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o700);
        fs::set_permissions(&path, perms)?;
    }
    Ok(path)
}

fn ensure_codex_config(
    model_name: &str,
    azure_endpoint: &str,
    workspace_dir: &Path,
    sandbox_mode: &str,
) -> Result<(), RunTaskError> {
    let home = env::var("HOME").map_err(|_| RunTaskError::MissingEnv { key: "HOME" })?;
    let config_dir = PathBuf::from(home).join(".codex");
    ensure_codex_config_at(
        model_name,
        azure_endpoint,
        &config_dir,
        workspace_dir,
        sandbox_mode,
    )
}

fn ensure_codex_config_at(
    model_name: &str,
    azure_endpoint: &str,
    config_dir: &Path,
    trust_workspace_dir: &Path,
    sandbox_mode: &str,
) -> Result<(), RunTaskError> {
    let config_path = config_dir.join("config.toml");
    let config_dir = config_path.parent().ok_or(RunTaskError::InvalidPath {
        label: "codex_config_dir",
        path: config_path.clone(),
        reason: "could not resolve config directory",
    })?;
    fs::create_dir_all(config_dir)?;

    let endpoint = normalize_azure_endpoint(azure_endpoint);
    let block = CODEX_CONFIG_BLOCK_TEMPLATE
        .replace("{model_name}", model_name)
        .replace("{azure_endpoint}", &endpoint)
        .replace("{sandbox_mode}", sandbox_mode);

    let existing = if config_path.exists() {
        fs::read_to_string(&config_path)?
    } else {
        String::new()
    };

    let updated = update_config_block(&existing, &block);
    let updated = ensure_project_trust(&updated, trust_workspace_dir);
    fs::write(config_path, updated)?;
    Ok(())
}

fn ensure_project_trust(existing: &str, workspace_dir: &Path) -> String {
    let workspace_str = workspace_dir.to_string_lossy();
    let escaped = toml_escape(&workspace_str);
    let header = format!("[projects.\"{escaped}\"]");
    if existing.contains(&header) {
        return existing.to_string();
    }
    let mut updated = existing.trim_end().to_string();
    if !updated.is_empty() {
        updated.push_str("\n\n");
    }
    updated.push_str(&header);
    updated.push('\n');
    updated.push_str("trust_level = \"trusted\"\n");
    updated
}

fn toml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn codex_sandbox_mode() -> String {
    env::var("CODEX_SANDBOX").unwrap_or_else(|_| "workspace-write".to_string())
}

fn codex_bypass_sandbox() -> bool {
    env_enabled("CODEX_BYPASS_SANDBOX")
        || env_enabled("CODEX_DANGEROUSLY_BYPASS_SANDBOX")
}

fn normalize_azure_endpoint(endpoint: &str) -> String {
    let trimmed = endpoint.trim();
    if trimmed.ends_with("/openai/v1") {
        trimmed.to_string()
    } else {
        format!("{}/openai/v1", trimmed.trim_end_matches('/'))
    }
}

fn update_config_block(existing: &str, block: &str) -> String {
    if let Some(marker_index) = existing.find(CODEX_CONFIG_MARKER) {
        if let Some(block_end_index) = existing[marker_index..].find("wire_api = \"responses\"") {
            let end_index = marker_index + block_end_index + "wire_api = \"responses\"".len();
            let end_line_index = existing[end_index..]
                .find('\n')
                .map(|idx| end_index + idx + 1)
                .unwrap_or_else(|| existing.len());
            let mut updated = String::new();
            updated.push_str(existing[..marker_index].trim_end());
            if !updated.is_empty() {
                updated.push_str("\n\n");
            }
            updated.push_str(block.trim_end());
            updated.push('\n');
            updated.push_str(existing[end_line_index..].trim_start());
            return updated;
        }
    }

    let mut updated = existing.trim_end().to_string();
    if !updated.is_empty() {
        updated.push_str("\n\n");
    }
    updated.push_str(block.trim_end());
    updated.push('\n');
    updated
}

fn extract_scheduled_tasks(output: &str) -> (Vec<ScheduledTaskRequest>, Option<String>) {
    let end = match output.rfind(SCHEDULED_TASKS_END) {
        Some(end) => end,
        None => return (Vec::new(), None),
    };
    let start = match output[..end].rfind(SCHEDULED_TASKS_BEGIN) {
        Some(start) => start + SCHEDULED_TASKS_BEGIN.len(),
        None => {
            return (
                Vec::new(),
                Some("missing scheduled tasks start marker".to_string()),
            )
        }
    };

    let raw = output[start..end].trim();
    if raw.is_empty() {
        return (Vec::new(), None);
    }

    match serde_json::from_str::<ScheduledTasksBlock>(raw) {
        Ok(parsed) => {
            let tasks = match parsed {
                ScheduledTasksBlock::List(tasks) => tasks,
                ScheduledTasksBlock::Wrapper { tasks } => tasks,
            };
            (tasks, None)
        }
        Err(err) => (
            Vec::new(),
            Some(format!("failed to parse scheduled tasks JSON: {}", err)),
        ),
    }
}

fn extract_scheduler_actions(output: &str) -> (Vec<SchedulerActionRequest>, Option<String>) {
    let end = match output.rfind(SCHEDULER_ACTIONS_END) {
        Some(end) => end,
        None => return (Vec::new(), None),
    };
    let start = match output[..end].rfind(SCHEDULER_ACTIONS_BEGIN) {
        Some(start) => start + SCHEDULER_ACTIONS_BEGIN.len(),
        None => {
            return (
                Vec::new(),
                Some("missing scheduler actions start marker".to_string()),
            )
        }
    };

    let raw = output[start..end].trim();
    if raw.is_empty() {
        return (Vec::new(), None);
    }

    match serde_json::from_str::<SchedulerActionsBlock>(raw) {
        Ok(parsed) => {
            let actions = match parsed {
                SchedulerActionsBlock::List(actions) => actions,
                SchedulerActionsBlock::Wrapper { actions } => actions,
            };
            (actions, None)
        }
        Err(err) => (
            Vec::new(),
            Some(format!("failed to parse scheduler actions JSON: {}", err)),
        ),
    }
}

fn build_prompt(
    input_email_dir: &Path,
    input_attachments_dir: &Path,
    memory_dir: &Path,
    reference_dir: &Path,
    memory_context: &str,
    reply_required: bool,
) -> String {
    let memory_section = if memory_context.trim().is_empty() {
        "Memory context (from memory/*.md):\n- (no memory files found)\n\n".to_string()
    } else {
        format!(
            "Memory context (from memory/*.md):\n{memory_context}\n\n",
            memory_context = memory_context.trim_end()
        )
    };
    let reply_instruction = if reply_required {
        "2. After finishing the task (step one), make sure you write a proper HTML email draft in reply_email_draft.html in the workspace root. If there are files to attach, put them in reply_email_attachments/ and reference them in the email draft. Do not pretend the job has been done without actually doing it, and do not write the email draft until the task is done. If you are not sure about the task, send another email to ask for clarification (and if any, attach information about why did you fail to get the task done, what is the exact error you encountered)."
    } else {
        "2. After finishing the task (step one), do not write any email draft. This inbound message is from a non-replyable address, so skip creating reply_email_draft.html or reply_email_attachments/."
    };
    format!(
        r#"You are Oliver, a digital assistant at DoWhiz. You are powerful, resilient, and helpful. Your task is to read incoming emails, understand the user's intent, finish the task, and draft appropriate email replies. You can also use memory and reference materials for context (already saved under current workspace). Always be cute, patient, friendly and helpful in your replies.

You main goal is
1. Most importantly, understand the task described in the incoming email and get the task done.
{reply_instruction}

Inputs (relative to workspace root):
- Incoming email dir: {input_email} (email.html, postmark_payload.json, thread_history.md, entries/)
- For incoming email, all previous emails in current thread: /incoming_email/entries/
- Incoming attachments dir: {input_attachments}
- Memory dir (memory about the current user): {memory}
- Reference dir (contain all past emails with the current user): {reference}

Memory about the current user:
```{memory_section}```

Memory management and maintain policy:
- Read all Markdown files under memory/ before starting; they are long-term, per-user memory.
- Persist durable facts only (identity, preferences, recurring tasks, projects, contacts,
  decisions, and working processes). Do not store transient email-specific details.
- Default file is memory/memo.md (Markdown).
- If memo.md exceeds 500 lines, split by info type into multiple files (for example:
  memo_profile.md, memo_preferences.md, memo_projects.md, memo_contacts.md,
  memo_decisions.md, memo_processes.md). Keep every file <= 500 lines.
- When split, replace memo.md with a short index or highlights so it stays <= 500 lines.
- Update memory files at the end if new durable info is learned; otherwise leave unchanged.

Scheduling:
- For any scheduling (email or task), you MUST use the skill "scheduler_maintain".

Rules:
- Each workspace includes a `.env` file at the workspace root. You may edit it to manage per-user secrets; updates are synced back after the task completes.
- Do not modify input directories. Any file editing requests should be done on the copied version of attachments and save into reply_email_attachments/ to be sent back to the user. Mark version updates as "_v2", "_v3", etc. in the filename.
- You may create or modify other files and folders in the workspace as needed to complete the task.
  Prefer creating a work/ directory for clones, patches, and build artifacts.
- If attachments include version suffixes like _v1, _v2, the highest version should be the latest version.
- Avoid interactive commands; use non-interactive flags for git/gh (for example, `gh pr create --title ... --body ...`).
"#,
        input_email = input_email_dir.display(),
        input_attachments = input_attachments_dir.display(),
        memory = memory_dir.display(),
        reference = reference_dir.display(),
        memory_section = memory_section,
        reply_instruction = reply_instruction,
    )
}

fn load_memory_context(workspace_dir: &Path, memory_dir: &Path) -> Result<String, RunTaskError> {
    let resolved = resolve_rel_dir(workspace_dir, memory_dir, "memory_dir")?;
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&resolved)? {
        let entry = entry?;
        if entry.file_type()?.is_file() && is_markdown_file(&entry.path()) {
            files.push(entry.path());
        }
    }
    files.sort_by(|left, right| left.file_name().cmp(&right.file_name()));

    let mut sections = Vec::new();
    for path in files {
        let content = fs::read_to_string(&path)?;
        let rel_path = path.strip_prefix(workspace_dir).unwrap_or(&path);
        sections.push(format!(
            "--- {path} ---\n{content}",
            path = rel_path.display(),
            content = content.trim_end()
        ));
    }
    Ok(sections.join("\n\n"))
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown"))
        .unwrap_or(false)
}

fn tail_string(input: &str, max_len: usize) -> String {
    let trimmed = input.trim();
    if trimmed.len() <= max_len {
        return trimmed.to_string();
    }
    let mut start = trimmed.len().saturating_sub(max_len);
    while start < trimmed.len() && !trimmed.is_char_boundary(start) {
        start += 1;
    }
    trimmed[start..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_memory_context_sorts_and_includes_markdown() {
        let temp = TempDir::new().expect("tempdir");
        let workspace = temp.path().join("workspace");
        let memory_dir = workspace.join("memory");
        fs::create_dir_all(&memory_dir).expect("memory dir");
        fs::write(memory_dir.join("b.md"), "second").expect("b.md");
        fs::write(memory_dir.join("a.md"), "first").expect("a.md");
        fs::write(memory_dir.join("note.txt"), "ignore").expect("note.txt");

        let context = load_memory_context(&workspace, Path::new("memory")).expect("context");

        let first_idx = context.find("--- memory/a.md ---").expect("a.md marker");
        let second_idx = context.find("--- memory/b.md ---").expect("b.md marker");
        assert!(first_idx < second_idx, "expected a.md before b.md");
        assert!(context.contains("first"));
        assert!(context.contains("second"));
        assert!(!context.contains("note.txt"));
    }

    #[test]
    fn build_prompt_includes_memory_policy_and_section() {
        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            "--- memory/memo.md ---\nHello",
            true,
        );

        assert!(prompt.contains("Memory context"));
        assert!(prompt.contains("memory/memo.md"));
        assert!(prompt.contains("Memory management"));
        assert!(prompt.contains("memo.md"));
        assert!(prompt.contains("500 lines"));
    }

    #[test]
    fn build_prompt_skips_reply_instruction_for_non_replyable() {
        let prompt = build_prompt(
            Path::new("incoming_email"),
            Path::new("incoming_attachments"),
            Path::new("memory"),
            Path::new("references"),
            "",
            false,
        );

        assert!(prompt.contains("non-replyable address"));
        assert!(!prompt.contains("write a proper HTML email draft"));
    }

    #[test]
    fn extract_scheduler_actions_returns_empty_when_missing() {
        let output = "no scheduler actions here";
        let (actions, error) = extract_scheduler_actions(output);
        assert!(actions.is_empty());
        assert!(error.is_none());
    }

    #[test]
    fn extract_scheduler_actions_parses_list() {
        let output = format!(
            "before\n{}\n[{{\"action\":\"cancel\",\"task_ids\":[\"a\",\"b\"]}}]\n{}\nafter",
            SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END
        );
        let (actions, error) = extract_scheduler_actions(&output);
        assert!(error.is_none());
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            SchedulerActionRequest::Cancel { task_ids } => {
                assert_eq!(task_ids, &vec!["a".to_string(), "b".to_string()]);
            }
            other => panic!("unexpected action: {:?}", other),
        }
    }

    #[test]
    fn extract_scheduler_actions_reports_invalid_json() {
        let output = format!(
            "{}\n[{{\"action\":\"cancel\",\"task_ids\"::}}]\n{}",
            SCHEDULER_ACTIONS_BEGIN, SCHEDULER_ACTIONS_END
        );
        let (actions, error) = extract_scheduler_actions(&output);
        assert!(actions.is_empty());
        assert!(error.is_some());
    }
}
