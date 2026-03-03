use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::constants::{
    CODEX_BASE_URL, CODEX_CONFIG_BLOCK_TEMPLATE, CODEX_CONFIG_MARKER, CODEX_MODEL_NAME,
    CODEX_SANDBOX_MODE, DOCKER_CODEX_HOME_DIR, DOCKER_WORKSPACE_DIR,
};
use super::docker::{docker_cli_available, ensure_docker_image_available};
use super::env::{env_enabled, normalize_env_prefix, read_env_list, read_env_trimmed};
use super::errors::RunTaskError;
use super::github_auth::{ensure_github_cli_auth, resolve_github_auth};
use super::prompt::{build_prompt, load_memory_context};
use super::scheduled::{extract_scheduled_tasks, extract_scheduler_actions};
use super::types::{RunTaskOutput, RunTaskRequest, TokenUsage};
use super::utils::{run_command_with_timeout, run_task_timeout, tail_string};
use super::workspace::{canonicalize_dir, workspace_path_in_container};

const PAYMENT_ENV_KEYS: &[&str] = &[
    "GOATX402_API_URL",
    "GOATX402_MERCHANT_ID",
    "GOATX402_API_KEY",
    "GOATX402_API_SECRET",
    "GOATX402_WALLET_ADDRESS",
    "GOATX402_AGENT_ID",
    "GOATX402_CHAIN_ID",
    "GOATX402_RPC_URL",
    "GOATX402_EXPLORER_URL",
    "GOATX402_USDC_ADDRESS",
    "GOATX402_USDT_ADDRESS",
    "GOAT_WALLET_ADDRESS",
    "GOAT_AGENT_ID",
    "GOAT_CHAIN_ID",
    "GOAT_RPC_URL",
    "GOAT_EXPLORER_URL",
    "GOAT_USDC_ADDRESS",
    "GOAT_USDT_ADDRESS",
    "X402_API_URL",
    "X402_MERCHANT_ID",
    "X402_API_KEY",
    "X402_API_SECRET",
];

const REMOTE_OUTPUT_FILENAME: &str = ".codex_remote_output.log";
const REMOTE_EXIT_CODE_FILENAME: &str = ".codex_remote_exit_code";
static ACI_CONTAINER_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionBackend {
    Local,
    AzureAci,
}

#[derive(Debug, Clone)]
struct AzureAciConfig {
    resource_group: String,
    image: String,
    location: Option<String>,
    registry_server: Option<String>,
    registry_username: Option<String>,
    registry_password: Option<String>,
    cpu: String,
    memory_gb: String,
    storage_account: String,
    storage_key: String,
    file_share: String,
    host_share_root: PathBuf,
    container_share_root: PathBuf,
}

pub(super) fn run_codex_task(
    request: RunTaskRequest<'_>,
    runner: &str,
    reply_html_path: PathBuf,
    reply_attachments_dir: PathBuf,
) -> Result<RunTaskOutput, RunTaskError> {
    super::env::load_env_sources(request.workspace_dir)?;
    let backend = resolve_execution_backend();
    match backend {
        ExecutionBackend::AzureAci => {
            eprintln!(
                "[run_task] execution_backend=azure_aci deploy_target={}",
                env::var("DEPLOY_TARGET").unwrap_or_else(|_| "unknown".to_string())
            );
        }
        ExecutionBackend::Local => {
            eprintln!("[run_task] execution_backend=local");
        }
    }
    if backend == ExecutionBackend::AzureAci {
        return run_codex_task_azure_aci(request, runner, reply_html_path, reply_attachments_dir);
    }
    ensure_local_execution_allowed()?;
    let docker_image = read_env_trimmed("RUN_TASK_DOCKER_IMAGE");
    let docker_requested =
        env_enabled_targeted("RUN_TASK_USE_DOCKER", "STAGING_RUN_TASK_USE_DOCKER");
    let docker_available = docker_requested && docker_cli_available();
    let docker_required = env_enabled_targeted(
        "RUN_TASK_DOCKER_REQUIRED",
        "STAGING_RUN_TASK_DOCKER_REQUIRED",
    );
    let use_docker = docker_requested && docker_available;
    if docker_requested && !docker_available {
        if docker_required {
            return Err(RunTaskError::DockerNotFound);
        }
        eprintln!(
            "[run_task] Docker CLI not found; falling back to host execution. Set RUN_TASK_DOCKER_REQUIRED=1 to fail."
        );
    }
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
    let azure_endpoint = normalize_azure_endpoint(CODEX_BASE_URL);
    // Use model from request/database, fallback to env var, then constant
    let model_name = if request.model_name.trim().is_empty() {
        env::var("CODEX_MODEL").unwrap_or_else(|_| CODEX_MODEL_NAME.to_string())
    } else {
        request.model_name.to_string()
    };
    let sandbox_mode = codex_sandbox_mode();
    // Bypass sandbox for GoogleDocs tasks to allow network access for Google APIs
    let channel_lower = request.channel.to_lowercase();
    let is_google_docs = channel_lower == "google_docs" || channel_lower == "googledocs";
    // Also bypass sandbox if workspace has .google_access_token (indicates Google Docs artifacts)
    let has_google_token = request.workspace_dir.join(".google_access_token").exists();
    let bypass_sandbox = codex_bypass_sandbox() || use_docker || is_google_docs || has_google_token;
    let sandbox_mode = effective_codex_sandbox_mode(&sandbox_mode, bypass_sandbox);
    let add_dirs = codex_add_dirs(request.workspace_dir, use_docker)?;
    if use_docker {
        let codex_home = host_workspace_dir
            .as_ref()
            .map(|dir| dir.join(DOCKER_CODEX_HOME_DIR))
            .unwrap_or_else(|| request.workspace_dir.join(DOCKER_CODEX_HOME_DIR));
        ensure_codex_config_at(&codex_home, Path::new(DOCKER_WORKSPACE_DIR))?;
    } else {
        ensure_codex_config(request.workspace_dir)?;
    }
    ensure_github_cli_auth(&github_auth)?;
    let payment_env_overrides = collect_payment_env_overrides();

    let memory_context = load_memory_context(request.workspace_dir, request.memory_dir)?;
    let prompt = build_prompt(
        request.input_email_dir,
        request.input_attachments_dir,
        request.memory_dir,
        request.reference_dir,
        request.workspace_dir,
        runner,
        &memory_context,
        !request.reply_to.is_empty(),
        request.channel,
        request.has_unified_account,
    );

    let timeout = run_task_timeout();
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
        // Write Google access token to file for sandbox environments without network access
        // (Codex sandbox may not pass environment variables to tools it spawns)
        if let Some(ref token) = request.google_access_token {
            cmd.arg("-e").arg(format!("GOOGLE_ACCESS_TOKEN={}", token));
            // Also write to file as backup since Codex sandbox may strip env vars
            let token_file = host_workspace_dir.join(".google_access_token");
            if let Err(e) = std::fs::write(&token_file, token) {
                eprintln!(
                    "[run_task] Warning: Failed to write Google access token file: {}",
                    e
                );
            }
        }
        for (key, value) in &payment_env_overrides {
            cmd.arg("-e").arg(format!("{}={}", key, value));
        }
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
            .arg("exec")
            .arg("--json");
        if bypass_sandbox {
            cmd.arg("--yolo");
        }
        for add_dir in &add_dirs {
            cmd.arg("--add-dir").arg(add_dir);
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

        match run_command_with_timeout(cmd, timeout, "docker run") {
            Ok(output) => output,
            Err(RunTaskError::Io(err)) if err.kind() == io::ErrorKind::NotFound => {
                return Err(RunTaskError::DockerNotFound)
            }
            Err(err) => return Err(err),
        }
    } else {
        let mut cmd = Command::new("codex");
        cmd.arg("exec").arg("--json");
        if bypass_sandbox {
            cmd.arg("--yolo");
        }
        for add_dir in &add_dirs {
            cmd.arg("--add-dir").arg(add_dir);
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
            .env("AZURE_OPENAI_ENDPOINT_BACKUP", &azure_endpoint)
            .current_dir(request.workspace_dir);
        // Extend PATH with DoWhiz bin directory for tools like google-docs
        let current_path = env::var("PATH").unwrap_or_default();
        let dowhiz_bin_dir = env::var("DOWHIZ_BIN_DIR")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| {
                let manifest_dir = env!("CARGO_MANIFEST_DIR");
                let parent = Path::new(manifest_dir).parent().unwrap_or(Path::new("."));
                parent.join("bin").to_string_lossy().into_owned()
            });
        let extended_path = format!("{}:{}", dowhiz_bin_dir, current_path);
        cmd.env("PATH", extended_path);
        // Write Google access token to file for sandbox environments without network access
        // (Codex sandbox may not pass environment variables to tools it spawns)
        if let Some(ref token) = request.google_access_token {
            cmd.env("GOOGLE_ACCESS_TOKEN", token);
            // Also write to file as backup since Codex sandbox may strip env vars
            let token_file = request.workspace_dir.join(".google_access_token");
            if let Err(e) = fs::write(&token_file, token) {
                eprintln!(
                    "[run_task] Warning: Failed to write Google access token file: {}",
                    e
                );
            }
        }
        for (key, value) in &payment_env_overrides {
            cmd.env(key, value);
        }
        for (key, value) in github_auth.env_overrides {
            cmd.env(key, value);
        }
        if let Some(askpass_path) = github_auth.askpass_path {
            cmd.env("GIT_ASKPASS", askpass_path);
            cmd.env("GIT_TERMINAL_PROMPT", "0");
        }

        match run_command_with_timeout(cmd, timeout, "codex") {
            Ok(output) => output,
            Err(RunTaskError::Io(err)) if err.kind() == io::ErrorKind::NotFound => {
                return Err(RunTaskError::CodexNotFound)
            }
            Err(err) => return Err(err),
        }
    };

    let stdout_output = String::from_utf8_lossy(&output.stdout);
    let stderr_output = String::from_utf8_lossy(&output.stderr);
    let mut combined_output = String::new();
    combined_output.push_str(&stdout_output);
    combined_output.push_str(&stderr_output);

    let (scheduled_tasks, scheduled_tasks_error, scheduler_actions, scheduler_actions_error) =
        parse_scheduling_from_outputs(
            &stdout_output,
            &stderr_output,
            &combined_output,
            request.workspace_dir,
        );
    let token_usage = extract_token_usage(&combined_output);
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(if use_docker {
            RunTaskError::DockerFailed {
                status: output.status.code(),
                output: output_tail.clone(),
            }
        } else {
            RunTaskError::CodexFailed {
                status: output.status.code(),
                output: output_tail.clone(),
            }
        });
    }

    // Codex can return process exit code 0 while reporting turn/task failure in JSON events.
    // Surface those runtime failures before checking for expected output files.
    if let Some(runtime_failure) = detect_codex_runtime_failure(&combined_output) {
        let status = runtime_failure
            .status_code
            .or_else(|| output.status.code().filter(|code| *code != 0));
        let mut failure_output = runtime_failure.message;
        if !output_tail.is_empty() {
            failure_output.push('\n');
            failure_output.push_str(&output_tail);
        }
        return Err(if use_docker {
            RunTaskError::DockerFailed {
                status,
                output: failure_output,
            }
        } else {
            RunTaskError::CodexFailed {
                status,
                output: failure_output,
            }
        });
    }

    // Only check for reply file if a reply was expected
    if !request.reply_to.is_empty() && !reply_html_path.exists() {
        return Err(RunTaskError::OutputMissing {
            path: reply_html_path,
            output: output_tail.clone(),
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
        token_usage,
    })
}

fn resolve_execution_backend() -> ExecutionBackend {
    match read_targeted_env(
        "RUN_TASK_EXECUTION_BACKEND",
        "STAGING_RUN_TASK_EXECUTION_BACKEND",
    )
    .unwrap_or_else(|| "auto".to_string())
    .to_ascii_lowercase()
    .as_str()
    {
        "azure_aci" => ExecutionBackend::AzureAci,
        "local" => ExecutionBackend::Local,
        _ => {
            let target = normalized_deploy_target();
            if target == "staging" || target == "production" {
                ExecutionBackend::AzureAci
            } else {
                ExecutionBackend::Local
            }
        }
    }
}

fn normalized_deploy_target() -> String {
    env::var("DEPLOY_TARGET")
        .unwrap_or_else(|_| "local".to_string())
        .trim()
        .to_ascii_lowercase()
}

fn is_staging_deploy_target() -> bool {
    normalized_deploy_target() == "staging"
}

fn read_targeted_env(base_key: &'static str, staging_key: &'static str) -> Option<String> {
    if is_staging_deploy_target() {
        read_env_trimmed(staging_key)
    } else {
        read_env_trimmed(base_key)
    }
}

fn required_targeted_env(
    base_key: &'static str,
    staging_key: &'static str,
) -> Result<String, RunTaskError> {
    let key = if is_staging_deploy_target() {
        staging_key
    } else {
        base_key
    };
    read_env_trimmed(key).ok_or(RunTaskError::MissingEnv { key })
}

fn targeted_key(base_key: &'static str, staging_key: &'static str) -> &'static str {
    if is_staging_deploy_target() {
        staging_key
    } else {
        base_key
    }
}

fn env_enabled_targeted(base_key: &'static str, staging_key: &'static str) -> bool {
    let key = if is_staging_deploy_target() {
        staging_key
    } else {
        base_key
    };
    env_enabled(key)
}

fn ensure_local_execution_allowed() -> Result<(), RunTaskError> {
    let target = normalized_deploy_target();
    if target == "staging" || target == "production" {
        return Err(RunTaskError::LocalExecutionForbidden {
            deploy_target: target,
        });
    }
    Ok(())
}

fn run_codex_task_azure_aci(
    request: RunTaskRequest<'_>,
    runner: &str,
    reply_html_path: PathBuf,
    reply_attachments_dir: PathBuf,
) -> Result<RunTaskOutput, RunTaskError> {
    let config = load_azure_aci_config()?;

    let host_workspace_dir = canonicalize_dir(request.workspace_dir)?;
    let host_share_root = canonicalize_dir(&config.host_share_root)?;
    let container_workspace_dir = map_workspace_to_container(
        &host_workspace_dir,
        &host_share_root,
        &config.container_share_root,
    )?;

    let askpass_dir = host_workspace_dir.join(DOCKER_CODEX_HOME_DIR);
    let github_auth = resolve_github_auth(Some(&askpass_dir))?;

    let api_key =
        env::var("AZURE_OPENAI_API_KEY_BACKUP").map_err(|_| RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        })?;
    if api_key.trim().is_empty() {
        return Err(RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        });
    }

    let azure_endpoint = normalize_azure_endpoint(CODEX_BASE_URL);
    let model_name = if request.model_name.trim().is_empty() {
        env::var("CODEX_MODEL").unwrap_or_else(|_| CODEX_MODEL_NAME.to_string())
    } else {
        request.model_name.to_string()
    };
    let sandbox_mode = codex_sandbox_mode();
    let channel_lower = request.channel.to_ascii_lowercase();
    let is_google_docs = channel_lower == "google_docs" || channel_lower == "googledocs";
    let has_google_token = request.workspace_dir.join(".google_access_token").exists();
    let bypass_sandbox = codex_bypass_sandbox() || is_google_docs || has_google_token;
    let sandbox_mode = effective_codex_sandbox_mode(&sandbox_mode, bypass_sandbox);

    let add_dirs = codex_add_dirs_remote(&host_workspace_dir, &container_workspace_dir)?;
    let codex_home = host_workspace_dir.join(DOCKER_CODEX_HOME_DIR);
    ensure_codex_config_at(&codex_home, &container_workspace_dir)?;
    let payment_env_overrides = collect_payment_env_overrides();

    let memory_context = load_memory_context(request.workspace_dir, request.memory_dir)?;
    let prompt = build_prompt(
        request.input_email_dir,
        request.input_attachments_dir,
        request.memory_dir,
        request.reference_dir,
        request.workspace_dir,
        runner,
        &memory_context,
        !request.reply_to.is_empty(),
        request.channel,
        request.has_unified_account,
    );

    // Remote executor reads prompt from workspace file to avoid oversized command lines.
    let prompt_path = host_workspace_dir.join(".codex_remote_prompt.txt");
    fs::write(&prompt_path, &prompt)?;

    let remote_output_path = host_workspace_dir.join(REMOTE_OUTPUT_FILENAME);
    let remote_exit_code_path = host_workspace_dir.join(REMOTE_EXIT_CODE_FILENAME);
    let _ = fs::remove_file(&remote_output_path);
    let _ = fs::remove_file(&remote_exit_code_path);

    if let Some(token) = request.google_access_token {
        // Keep token as workspace file so remote container tools can access it.
        fs::write(host_workspace_dir.join(".google_access_token"), token)?;
    }

    let askpass_container_path = github_auth.askpass_path.as_ref().and_then(|path| {
        map_path_to_container(path, &host_workspace_dir, &container_workspace_dir)
    });
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

    let mut env_overrides = vec![
        (
            "AZURE_OPENAI_API_KEY_BACKUP".to_string(),
            api_key.to_string(),
        ),
        (
            "AZURE_OPENAI_ENDPOINT_BACKUP".to_string(),
            azure_endpoint.to_string(),
        ),
        (
            "HOME".to_string(),
            container_workspace_dir.to_string_lossy().into_owned(),
        ),
        (
            "CODEX_HOME".to_string(),
            format!(
                "{}/{}",
                container_workspace_dir.to_string_lossy(),
                DOCKER_CODEX_HOME_DIR
            ),
        ),
        ("DEPLOY_TARGET".to_string(), "azure_aci_runner".to_string()),
    ];
    for (key, value) in payment_env_overrides {
        env_overrides.push((key, value));
    }
    for (key, value) in github_auth.env_overrides {
        env_overrides.push((key, value));
    }
    if let Some(token) = request.google_access_token {
        env_overrides.push(("GOOGLE_ACCESS_TOKEN".to_string(), token.to_string()));
    }
    if let Some(container_path) = askpass_container_path {
        env_overrides.push((
            "GIT_ASKPASS".to_string(),
            container_path.to_string_lossy().into_owned(),
        ));
        env_overrides.push(("GIT_TERMINAL_PROMPT".to_string(), "0".to_string()));
    }

    let container_name = build_aci_container_name();
    eprintln!(
        "[run_task] azure_aci create container={} resource_group={} image={}",
        container_name, config.resource_group, config.image
    );
    let timeout = run_task_timeout();
    let execution = run_azure_aci_execution(
        &config,
        &container_name,
        &container_workspace_dir,
        &add_dirs,
        &model_name,
        &sandbox_mode,
        bypass_sandbox,
        &env_overrides,
        timeout,
    );
    eprintln!(
        "[run_task] azure_aci delete container={} resource_group={}",
        container_name, config.resource_group
    );
    let _ = delete_aci_container(&config, &container_name);

    let (container_state, container_logs) = execution?;
    eprintln!(
        "[run_task] azure_aci finished container={} state={}",
        container_name, container_state
    );
    let output_content = fs::read_to_string(&remote_output_path).unwrap_or_default();
    let exit_status = read_remote_exit_code(&remote_exit_code_path);

    let mut combined_output = String::new();
    combined_output.push_str(&output_content);
    if !container_logs.trim().is_empty() {
        if !combined_output.trim().is_empty() {
            combined_output.push('\n');
        }
        combined_output.push_str(&container_logs);
    }

    let (scheduled_tasks, scheduled_tasks_error) = extract_scheduled_tasks(&combined_output);
    let (scheduler_actions, scheduler_actions_error) = extract_scheduler_actions(&combined_output);
    let token_usage = extract_token_usage(&combined_output);
    let output_tail = tail_string(&combined_output, 4000);

    if container_state != "Succeeded" || exit_status != Some(0) {
        return Err(RunTaskError::CodexFailed {
            status: exit_status,
            output: format!(
                "azure_aci_state={}{}\n{}",
                container_state,
                match exit_status {
                    Some(code) => format!(" exit_code={code}"),
                    None => String::new(),
                },
                output_tail
            ),
        });
    }

    if !request.reply_to.is_empty() && !reply_html_path.exists() {
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
        token_usage,
    })
}

fn load_azure_aci_config() -> Result<AzureAciConfig, RunTaskError> {
    let resource_group = required_targeted_env(
        "RUN_TASK_AZURE_ACI_RESOURCE_GROUP",
        "STAGING_RUN_TASK_AZURE_ACI_RESOURCE_GROUP",
    )?;
    let image = read_targeted_env(
        "RUN_TASK_AZURE_ACI_IMAGE",
        "STAGING_RUN_TASK_AZURE_ACI_IMAGE",
    )
    .or_else(|| read_targeted_env("RUN_TASK_DOCKER_IMAGE", "STAGING_RUN_TASK_DOCKER_IMAGE"))
    .ok_or(RunTaskError::MissingEnv {
        key: targeted_key(
            "RUN_TASK_AZURE_ACI_IMAGE",
            "STAGING_RUN_TASK_AZURE_ACI_IMAGE",
        ),
    })?;
    let location = read_targeted_env(
        "RUN_TASK_AZURE_ACI_LOCATION",
        "STAGING_RUN_TASK_AZURE_ACI_LOCATION",
    );
    let mut registry_server = read_targeted_env(
        "RUN_TASK_AZURE_ACI_REGISTRY_SERVER",
        "STAGING_RUN_TASK_AZURE_ACI_REGISTRY_SERVER",
    );
    if registry_server.is_none() {
        registry_server = image
            .split('/')
            .next()
            .filter(|candidate| candidate.contains('.'))
            .map(|value| value.to_string());
    }
    let registry_username = read_targeted_env(
        "RUN_TASK_AZURE_ACI_REGISTRY_USERNAME",
        "STAGING_RUN_TASK_AZURE_ACI_REGISTRY_USERNAME",
    );
    let registry_password = read_targeted_env(
        "RUN_TASK_AZURE_ACI_REGISTRY_PASSWORD",
        "STAGING_RUN_TASK_AZURE_ACI_REGISTRY_PASSWORD",
    );
    if registry_username.is_some() && registry_password.is_none() {
        return Err(RunTaskError::MissingEnv {
            key: targeted_key(
                "RUN_TASK_AZURE_ACI_REGISTRY_PASSWORD",
                "STAGING_RUN_TASK_AZURE_ACI_REGISTRY_PASSWORD",
            ),
        });
    }
    if registry_password.is_some() && registry_username.is_none() {
        return Err(RunTaskError::MissingEnv {
            key: targeted_key(
                "RUN_TASK_AZURE_ACI_REGISTRY_USERNAME",
                "STAGING_RUN_TASK_AZURE_ACI_REGISTRY_USERNAME",
            ),
        });
    }
    if registry_username.is_some() && registry_server.is_none() {
        return Err(RunTaskError::MissingEnv {
            key: targeted_key(
                "RUN_TASK_AZURE_ACI_REGISTRY_SERVER",
                "STAGING_RUN_TASK_AZURE_ACI_REGISTRY_SERVER",
            ),
        });
    }
    let cpu = read_targeted_env("RUN_TASK_AZURE_ACI_CPU", "STAGING_RUN_TASK_AZURE_ACI_CPU")
        .unwrap_or_else(|| "2.0".to_string());
    let memory_gb = read_targeted_env(
        "RUN_TASK_AZURE_ACI_MEMORY_GB",
        "STAGING_RUN_TASK_AZURE_ACI_MEMORY_GB",
    )
    .unwrap_or_else(|| "4.0".to_string());
    let file_share = read_targeted_env(
        "RUN_TASK_AZURE_ACI_FILE_SHARE",
        "STAGING_RUN_TASK_AZURE_ACI_FILE_SHARE",
    )
    .unwrap_or_else(|| "dowhiz-run-task".to_string());

    let host_share_root = PathBuf::from(required_targeted_env(
        "RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT",
        "STAGING_RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT",
    )?);
    let container_share_root = PathBuf::from(
        read_targeted_env(
            "RUN_TASK_AZURE_ACI_CONTAINER_SHARE_ROOT",
            "STAGING_RUN_TASK_AZURE_ACI_CONTAINER_SHARE_ROOT",
        )
        .unwrap_or_else(|| "/mnt/dowhiz-share".to_string()),
    );

    let storage_account = read_targeted_env(
        "RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT",
        "STAGING_RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT",
    )
    .or_else(|| read_env_trimmed("AZURE_STORAGE_ACCOUNT"))
    .or_else(|| {
        read_env_trimmed("AZURE_STORAGE_CONNECTION_STRING")
            .and_then(|cs| parse_connection_string_component(&cs, "AccountName"))
    })
    .ok_or(RunTaskError::MissingEnv {
        key: targeted_key(
            "RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT",
            "STAGING_RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT",
        ),
    })?;

    let storage_key = read_targeted_env(
        "RUN_TASK_AZURE_ACI_STORAGE_KEY",
        "STAGING_RUN_TASK_AZURE_ACI_STORAGE_KEY",
    )
    .or_else(|| {
        read_targeted_env(
            "RUN_TASK_AZURE_ACI_STORAGE_CONNECTION_STRING",
            "STAGING_RUN_TASK_AZURE_ACI_STORAGE_CONNECTION_STRING",
        )
        .and_then(|cs| parse_connection_string_component(&cs, "AccountKey"))
    })
    .or_else(|| {
        read_env_trimmed("AZURE_STORAGE_CONNECTION_STRING")
            .and_then(|cs| parse_connection_string_component(&cs, "AccountKey"))
    })
    .ok_or(RunTaskError::MissingEnv {
        key: targeted_key(
            "RUN_TASK_AZURE_ACI_STORAGE_KEY",
            "STAGING_RUN_TASK_AZURE_ACI_STORAGE_KEY",
        ),
    })?;

    Ok(AzureAciConfig {
        resource_group,
        image,
        location,
        registry_server,
        registry_username,
        registry_password,
        cpu,
        memory_gb,
        storage_account,
        storage_key,
        file_share,
        host_share_root,
        container_share_root,
    })
}

fn parse_connection_string_component(connection_string: &str, key: &str) -> Option<String> {
    for part in connection_string.split(';') {
        let mut iter = part.splitn(2, '=');
        let part_key = iter.next()?.trim();
        let value = iter.next()?.trim();
        if part_key.eq_ignore_ascii_case(key) && !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

fn map_workspace_to_container(
    workspace_dir: &Path,
    host_share_root: &Path,
    container_share_root: &Path,
) -> Result<PathBuf, RunTaskError> {
    let relative =
        workspace_dir
            .strip_prefix(host_share_root)
            .map_err(|_| RunTaskError::InvalidPath {
                label: "workspace_dir",
                path: workspace_dir.to_path_buf(),
                reason: "workspace is outside RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT",
            })?;
    Ok(container_share_root.join(relative))
}

fn map_path_to_container(
    host_path: &Path,
    host_workspace_dir: &Path,
    container_workspace_dir: &Path,
) -> Option<PathBuf> {
    let relative = host_path.strip_prefix(host_workspace_dir).ok()?;
    Some(container_workspace_dir.join(relative))
}

fn codex_add_dirs_remote(
    host_workspace_dir: &Path,
    container_workspace_dir: &Path,
) -> Result<Vec<String>, RunTaskError> {
    let host_gh_config_dir = host_workspace_dir.join(".config").join("gh");
    fs::create_dir_all(&host_gh_config_dir)?;
    let container_gh_config_dir = container_workspace_dir.join(".config").join("gh");
    Ok(vec![container_gh_config_dir.to_string_lossy().into_owned()])
}

fn build_aci_container_name() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let seq = ACI_CONTAINER_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("dwz-codex-{}-{}-{}", millis, std::process::id(), seq)
}

fn run_azure_aci_execution(
    config: &AzureAciConfig,
    container_name: &str,
    container_workspace_dir: &Path,
    add_dirs: &[String],
    model_name: &str,
    sandbox_mode: &str,
    bypass_sandbox: bool,
    env_overrides: &[(String, String)],
    timeout: Duration,
) -> Result<(String, String), RunTaskError> {
    let workspace_sh = shell_quote(&container_workspace_dir.to_string_lossy());
    let output_file = shell_quote(REMOTE_OUTPUT_FILENAME);
    let exit_file = shell_quote(REMOTE_EXIT_CODE_FILENAME);

    let mut codex_parts = vec![
        "codex".to_string(),
        "exec".to_string(),
        "--json".to_string(),
    ];
    if bypass_sandbox {
        codex_parts.push("--yolo".to_string());
    }
    for add_dir in add_dirs {
        codex_parts.push("--add-dir".to_string());
        codex_parts.push(add_dir.clone());
    }
    codex_parts.push("--skip-git-repo-check".to_string());
    codex_parts.push("-m".to_string());
    codex_parts.push(model_name.to_string());
    codex_parts.push("-c".to_string());
    codex_parts.push("web_search=\"live\"".to_string());
    codex_parts.push("-c".to_string());
    codex_parts.push("ask_for_approval=\"never\"".to_string());
    codex_parts.push("-c".to_string());
    codex_parts.push(format!("sandbox=\"{}\"", sandbox_mode));
    codex_parts.push("-c".to_string());
    codex_parts.push("model_providers.azure.env_key=\"AZURE_OPENAI_API_KEY_BACKUP\"".to_string());
    codex_parts.push("--cd".to_string());
    codex_parts.push(container_workspace_dir.to_string_lossy().into_owned());
    codex_parts.push("\"$(cat .codex_remote_prompt.txt)\"".to_string());
    let codex_command = codex_parts
        .iter()
        .map(|part| {
            if part.starts_with("\"$(cat ") {
                part.to_string()
            } else {
                shell_quote(part)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let script = format!(
        "set -euo pipefail\nexport PATH=/app/bin:$PATH\ncd {workspace}\nmkdir -p .config/gh .codex\nrm -f {output} {exit}\nset +e\n{codex} > {output} 2>&1\nstatus=$?\nprintf '%s' \"$status\" > {exit}\nexit \"$status\"\n",
        workspace = workspace_sh,
        codex = codex_command,
        output = output_file,
        exit = exit_file,
    );

    let mut create_cmd = Command::new("az");
    create_cmd
        .arg("container")
        .arg("create")
        .arg("--name")
        .arg(container_name)
        .arg("--resource-group")
        .arg(&config.resource_group)
        .arg("--image")
        .arg(&config.image)
        .arg("--os-type")
        .arg("Linux")
        .arg("--restart-policy")
        .arg("Never")
        .arg("--cpu")
        .arg(&config.cpu)
        .arg("--memory")
        .arg(&config.memory_gb)
        .arg("--azure-file-volume-account-name")
        .arg(&config.storage_account)
        .arg("--azure-file-volume-account-key")
        .arg(&config.storage_key)
        .arg("--azure-file-volume-share-name")
        .arg(&config.file_share)
        .arg("--azure-file-volume-mount-path")
        .arg(&config.container_share_root)
        .arg("--command-line")
        .arg(format!("/bin/bash -lc {}", shell_quote(&script)))
        .arg("--only-show-errors")
        .arg("--output")
        .arg("json");
    if let Some(location) = &config.location {
        create_cmd.arg("--location").arg(location);
    }
    if let (Some(server), Some(username), Some(password)) = (
        &config.registry_server,
        &config.registry_username,
        &config.registry_password,
    ) {
        create_cmd
            .arg("--registry-login-server")
            .arg(server)
            .arg("--registry-username")
            .arg(username)
            .arg("--registry-password")
            .arg(password);
    }

    if !env_overrides.is_empty() {
        create_cmd.arg("--environment-variables");
        for (key, value) in env_overrides {
            create_cmd.arg(format!("{key}={value}"));
        }
    }

    let create_output =
        match run_command_with_timeout(create_cmd, Duration::from_secs(300), "az container create")
        {
            Ok(output) => output,
            Err(RunTaskError::Io(err)) if err.kind() == io::ErrorKind::NotFound => {
                return Err(RunTaskError::AzureCliNotFound)
            }
            Err(err) => return Err(err),
        };
    if !create_output.status.success() {
        let mut combined = String::new();
        combined.push_str(&String::from_utf8_lossy(&create_output.stdout));
        combined.push_str(&String::from_utf8_lossy(&create_output.stderr));
        return Err(RunTaskError::CodexFailed {
            status: create_output.status.code(),
            output: tail_string(&combined, 4000),
        });
    }

    let container_state = poll_aci_state(config, container_name, timeout)?;
    let logs = fetch_aci_logs(config, container_name).unwrap_or_default();
    Ok((container_state, logs))
}

fn poll_aci_state(
    config: &AzureAciConfig,
    container_name: &str,
    timeout: Duration,
) -> Result<String, RunTaskError> {
    let start = Instant::now();
    loop {
        let mut show_cmd = Command::new("az");
        show_cmd
            .arg("container")
            .arg("show")
            .arg("--name")
            .arg(container_name)
            .arg("--resource-group")
            .arg(&config.resource_group)
            .arg("--query")
            .arg("instanceView.state")
            .arg("--output")
            .arg("tsv")
            .arg("--only-show-errors");
        let output =
            run_command_with_timeout(show_cmd, Duration::from_secs(60), "az container show")?;
        if !output.status.success() {
            let mut combined = String::new();
            combined.push_str(&String::from_utf8_lossy(&output.stdout));
            combined.push_str(&String::from_utf8_lossy(&output.stderr));
            return Err(RunTaskError::CodexFailed {
                status: output.status.code(),
                output: tail_string(&combined, 4000),
            });
        }
        let state = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if state.eq_ignore_ascii_case("Succeeded")
            || state.eq_ignore_ascii_case("Failed")
            || state.eq_ignore_ascii_case("Terminated")
            || state.eq_ignore_ascii_case("Stopped")
        {
            return Ok(state);
        }
        if start.elapsed() >= timeout {
            return Err(RunTaskError::CommandTimeout {
                command: "az container show",
                timeout_secs: timeout.as_secs(),
                output: format!("last_state={state}"),
            });
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn fetch_aci_logs(config: &AzureAciConfig, container_name: &str) -> Result<String, RunTaskError> {
    let mut logs_cmd = Command::new("az");
    logs_cmd
        .arg("container")
        .arg("logs")
        .arg("--name")
        .arg(container_name)
        .arg("--resource-group")
        .arg(&config.resource_group)
        .arg("--only-show-errors")
        .arg("--output")
        .arg("tsv");
    let output = run_command_with_timeout(logs_cmd, Duration::from_secs(120), "az container logs")?;
    if !output.status.success() {
        return Ok(String::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn delete_aci_container(config: &AzureAciConfig, container_name: &str) -> Result<(), RunTaskError> {
    let mut delete_cmd = Command::new("az");
    delete_cmd
        .arg("container")
        .arg("delete")
        .arg("--name")
        .arg(container_name)
        .arg("--resource-group")
        .arg(&config.resource_group)
        .arg("--yes")
        .arg("--only-show-errors");
    let output =
        run_command_with_timeout(delete_cmd, Duration::from_secs(120), "az container delete")?;
    if !output.status.success() {
        let mut combined = String::new();
        combined.push_str(&String::from_utf8_lossy(&output.stdout));
        combined.push_str(&String::from_utf8_lossy(&output.stderr));
        return Err(RunTaskError::CodexFailed {
            status: output.status.code(),
            output: tail_string(&combined, 4000),
        });
    }
    Ok(())
}

fn read_remote_exit_code(path: &Path) -> Option<i32> {
    fs::read_to_string(path).ok()?.trim().parse::<i32>().ok()
}

fn shell_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("'\"'\"'");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

fn ensure_codex_config(workspace_dir: &Path) -> Result<(), RunTaskError> {
    let home = env::var("HOME").map_err(|_| RunTaskError::MissingEnv { key: "HOME" })?;
    let config_dir = PathBuf::from(home).join(".codex");
    ensure_codex_config_at(&config_dir, workspace_dir)
}

fn ensure_codex_config_at(
    config_dir: &Path,
    trust_workspace_dir: &Path,
) -> Result<(), RunTaskError> {
    let config_path = config_dir.join("config.toml");
    let config_dir = config_path.parent().ok_or(RunTaskError::InvalidPath {
        label: "codex_config_dir",
        path: config_path.clone(),
        reason: "could not resolve config directory",
    })?;
    fs::create_dir_all(config_dir)?;

    let block = CODEX_CONFIG_BLOCK_TEMPLATE;

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
    read_targeted_env("CODEX_SANDBOX_MODE", "STAGING_CODEX_SANDBOX_MODE")
        .or_else(|| {
            read_targeted_env(
                "RUN_TASK_CODEX_SANDBOX_MODE",
                "STAGING_RUN_TASK_CODEX_SANDBOX_MODE",
            )
        })
        .unwrap_or_else(|| CODEX_SANDBOX_MODE.to_string())
}

fn effective_codex_sandbox_mode(sandbox_mode: &str, bypass_sandbox: bool) -> String {
    if bypass_sandbox {
        "danger-full-access".to_string()
    } else {
        sandbox_mode.to_string()
    }
}

fn codex_bypass_sandbox() -> bool {
    env_enabled_targeted("CODEX_BYPASS_SANDBOX", "STAGING_CODEX_BYPASS_SANDBOX")
}

fn employee_id_default_env_prefix(employee_id: &str) -> Option<&'static str> {
    let normalized = employee_id.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "little_bear" => Some("OLIVER"),
        "mini_mouse" => Some("MAGGIE"),
        "sticky_octopus" => Some("DEVIN"),
        "boiled_egg" => Some("PROTO"),
        _ => None,
    }
}

fn resolve_payment_env_prefix() -> Option<String> {
    read_env_trimmed("EMPLOYEE_PAYMENT_ENV_PREFIX")
        .or_else(|| read_env_trimmed("PAYMENT_ENV_PREFIX"))
        .or_else(|| read_env_trimmed("EMPLOYEE_GITHUB_ENV_PREFIX"))
        .or_else(|| read_env_trimmed("GITHUB_ENV_PREFIX"))
        .or_else(|| {
            read_env_trimmed("EMPLOYEE_ID").and_then(|id| {
                employee_id_default_env_prefix(&id)
                    .map(|value| value.to_string())
                    .or_else(|| Some(normalize_env_prefix(&id)))
            })
        })
}

fn collect_payment_env_overrides() -> Vec<(String, String)> {
    let prefix = resolve_payment_env_prefix();
    PAYMENT_ENV_KEYS
        .iter()
        .filter_map(|key| {
            read_env_trimmed(key)
                .or_else(|| {
                    prefix
                        .as_ref()
                        .and_then(|prefix| read_env_trimmed(&format!("{}_{}", prefix, key)))
                })
                .map(|value| ((*key).to_string(), value))
        })
        .collect()
}

fn codex_add_dirs(workspace_dir: &Path, use_docker: bool) -> Result<Vec<String>, RunTaskError> {
    let mut add_dirs = Vec::new();
    if use_docker {
        let gh_config_dir = workspace_dir.join(".config").join("gh");
        fs::create_dir_all(&gh_config_dir)?;
        add_dirs.push(format!("{}/.config/gh", DOCKER_WORKSPACE_DIR));
    } else {
        let home = env::var("HOME").map_err(|_| RunTaskError::MissingEnv { key: "HOME" })?;
        let gh_config_dir = PathBuf::from(home).join(".config").join("gh");
        fs::create_dir_all(&gh_config_dir)?;
        add_dirs.push(gh_config_dir.to_string_lossy().into_owned());
    }
    Ok(add_dirs)
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

/// Parse token usage from Codex JSON output (JSONL format)
/// Looks for: {"type":"turn.completed","usage":{"input_tokens":N,"output_tokens":M}}
fn extract_token_usage(output: &str) -> Option<TokenUsage> {
    #[derive(serde::Deserialize)]
    struct TurnCompleted {
        #[serde(rename = "type")]
        event_type: String,
        usage: Option<TokenUsage>,
    }

    for line in output.lines() {
        if line.contains("\"turn.completed\"") {
            if let Ok(event) = serde_json::from_str::<TurnCompleted>(line) {
                if event.event_type == "turn.completed" {
                    return event.usage;
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodexRuntimeFailure {
    status_code: Option<i32>,
    message: String,
}

fn detect_codex_runtime_failure(output: &str) -> Option<CodexRuntimeFailure> {
    enum TerminalState {
        Success,
        Failure(CodexRuntimeFailure),
    }

    let mut terminal_state: Option<TerminalState> = None;

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value.get("type").and_then(|v| v.as_str()) != Some("event_msg") {
            continue;
        }
        let Some(payload) = value.get("payload") else {
            continue;
        };

        match payload.get("type").and_then(|v| v.as_str()) {
            Some("task_complete") => {
                let status = payload.get("status").and_then(|v| v.as_str());
                let exit_code = payload
                    .get("exit_code")
                    .and_then(|v| v.as_i64())
                    .and_then(|v| i32::try_from(v).ok());
                let status_failed = matches!(status, Some("failed" | "error" | "aborted"));
                let exit_failed = matches!(exit_code, Some(code) if code != 0);

                if status_failed || exit_failed {
                    let status_text = status.unwrap_or("unknown");
                    let mut message = format!("Codex task_complete reported status={status_text}");
                    if let Some(code) = exit_code {
                        message.push_str(&format!(" exit_code={code}"));
                    }
                    if let Some(last_agent_message) =
                        payload.get("last_agent_message").and_then(|v| v.as_str())
                    {
                        let trimmed = last_agent_message.trim();
                        if !trimmed.is_empty() {
                            message.push_str(&format!(
                                ". last_agent_message: {}",
                                tail_string(trimmed, 400)
                            ));
                        }
                    }
                    terminal_state = Some(TerminalState::Failure(CodexRuntimeFailure {
                        status_code: exit_code,
                        message,
                    }));
                } else if matches!(status, Some("success")) || matches!(exit_code, Some(0)) {
                    terminal_state = Some(TerminalState::Success);
                }
            }
            Some("turn_aborted") => {
                let reason = payload
                    .get("reason")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                terminal_state = Some(TerminalState::Failure(CodexRuntimeFailure {
                    status_code: None,
                    message: format!("Codex turn aborted (reason: {reason})"),
                }));
            }
            _ => {}
        }
    }

    match terminal_state {
        Some(TerminalState::Failure(failure)) => Some(failure),
        _ => None,
    }
}

fn parse_scheduling_from_outputs(
    stdout_output: &str,
    stderr_output: &str,
    combined_output: &str,
    workspace_dir: &Path,
) -> (
    Vec<super::types::ScheduledTaskRequest>,
    Option<String>,
    Vec<super::types::SchedulerActionRequest>,
    Option<String>,
) {
    // In --json mode, assistant text lives inside JSON fields with escaping.
    // Decode assistant message payloads first, then parse scheduler blocks.
    // Codex may emit JSONL to stdout or stderr depending on runtime environment.
    let assistant_output = extract_assistant_text_from_jsonl(stdout_output)
        .or_else(|| extract_assistant_text_from_jsonl(stderr_output))
        .or_else(|| extract_assistant_text_from_jsonl(combined_output));
    let scheduling_output = assistant_output.as_deref().unwrap_or("");
    let (mut scheduled_tasks, mut scheduled_tasks_error) =
        extract_scheduled_tasks(scheduling_output);
    let (mut scheduler_actions, mut scheduler_actions_error) =
        extract_scheduler_actions(scheduling_output);

    if assistant_output.is_none() {
        // Avoid parsing prompt scaffolding as scheduler JSON when assistant extraction fails.
        // Fall back to raw output only if it yields concrete tasks/actions.
        let (fallback_tasks, fallback_tasks_error) = extract_scheduled_tasks(combined_output);
        let (fallback_actions, fallback_actions_error) = extract_scheduler_actions(combined_output);
        if !fallback_tasks.is_empty() || !fallback_actions.is_empty() {
            scheduled_tasks = fallback_tasks;
            scheduled_tasks_error = fallback_tasks_error;
            scheduler_actions = fallback_actions;
            scheduler_actions_error = fallback_actions_error;
        } else {
            scheduled_tasks_error = None;
            scheduler_actions_error = None;
        }
    }

    if scheduled_tasks.is_empty()
        && scheduler_actions.is_empty()
        && (scheduled_tasks_error.is_some() || scheduler_actions_error.is_some())
    {
        if let Some(session_output) = extract_assistant_text_from_recent_session(workspace_dir) {
            let (session_tasks, session_tasks_error) = extract_scheduled_tasks(&session_output);
            let (session_actions, session_actions_error) =
                extract_scheduler_actions(&session_output);
            if !session_tasks.is_empty() || !session_actions.is_empty() {
                scheduled_tasks = session_tasks;
                scheduled_tasks_error = session_tasks_error;
                scheduler_actions = session_actions;
                scheduler_actions_error = session_actions_error;
            }
        }
    }

    (
        scheduled_tasks,
        scheduled_tasks_error,
        scheduler_actions,
        scheduler_actions_error,
    )
}

fn extract_assistant_text_from_recent_session(workspace_dir: &Path) -> Option<String> {
    let home = env::var("HOME").ok()?;
    let sessions_root = PathBuf::from(home).join(".codex").join("sessions");
    if !sessions_root.exists() {
        return None;
    }

    let mut session_files = Vec::new();
    collect_session_jsonl_files(&sessions_root, &mut session_files).ok()?;
    session_files.sort_by(|a, b| {
        let a_time = a
            .metadata()
            .and_then(|meta| meta.modified())
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        let b_time = b
            .metadata()
            .and_then(|meta| meta.modified())
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        b_time.cmp(&a_time)
    });

    let workspace_marker = workspace_dir.to_string_lossy();
    for session_path in session_files.into_iter().take(40) {
        let Ok(contents) = fs::read_to_string(&session_path) else {
            continue;
        };
        if !contents.contains(workspace_marker.as_ref()) {
            continue;
        }
        let Some(assistant_output) = extract_assistant_text_from_jsonl(&contents) else {
            continue;
        };
        if assistant_output.contains("SCHEDULED_TASKS_JSON_BEGIN")
            || assistant_output.contains("SCHEDULER_ACTIONS_JSON_BEGIN")
        {
            return Some(assistant_output);
        }
    }
    None
}

fn collect_session_jsonl_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_session_jsonl_files(&path, files)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("jsonl") {
            files.push(path);
        }
    }
    Ok(())
}

fn extract_assistant_text_from_jsonl(output: &str) -> Option<String> {
    let mut collected = String::new();
    let mut found = false;

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };

        if collect_item_completed_agent_message(&value, &mut collected) {
            found = true;
        }
        if collect_event_msg_agent_message(&value, &mut collected) {
            found = true;
        }
        if collect_response_item_assistant_message(&value, &mut collected) {
            found = true;
        }
    }

    found.then_some(collected)
}

fn append_collected_text(target: &mut String, text: &str) {
    if text.trim().is_empty() {
        return;
    }
    if !target.is_empty() {
        target.push('\n');
    }
    target.push_str(text);
}

fn collect_item_completed_agent_message(value: &serde_json::Value, target: &mut String) -> bool {
    if value.get("type").and_then(|v| v.as_str()) != Some("item.completed") {
        return false;
    }
    let Some(item) = value.get("item") else {
        return false;
    };
    if item.get("type").and_then(|v| v.as_str()) != Some("agent_message") {
        return false;
    }
    let Some(text) = item.get("text").and_then(|v| v.as_str()) else {
        return false;
    };
    append_collected_text(target, text);
    true
}

fn collect_event_msg_agent_message(value: &serde_json::Value, target: &mut String) -> bool {
    if value.get("type").and_then(|v| v.as_str()) != Some("event_msg") {
        return false;
    }
    let Some(payload) = value.get("payload") else {
        return false;
    };
    match payload.get("type").and_then(|v| v.as_str()) {
        Some("agent_message") => {
            let Some(text) = payload.get("message").and_then(|v| v.as_str()) else {
                return false;
            };
            append_collected_text(target, text);
            true
        }
        Some("task_complete") => {
            let Some(text) = payload.get("last_agent_message").and_then(|v| v.as_str()) else {
                return false;
            };
            append_collected_text(target, text);
            true
        }
        _ => false,
    }
}

fn collect_response_item_assistant_message(value: &serde_json::Value, target: &mut String) -> bool {
    if value.get("type").and_then(|v| v.as_str()) != Some("response_item") {
        return false;
    }
    let Some(payload) = value.get("payload") else {
        return false;
    };
    if payload.get("type").and_then(|v| v.as_str()) != Some("message") {
        return false;
    }
    if payload.get("role").and_then(|v| v.as_str()) != Some("assistant") {
        return false;
    }

    let mut appended = false;
    if let Some(content) = payload.get("content").and_then(|v| v.as_array()) {
        for part in content {
            if part.get("type").and_then(|v| v.as_str()) == Some("output_text") {
                if let Some(text) = part.get("text").and_then(|v| v.as_str()) {
                    append_collected_text(target, text);
                    appended = true;
                }
            }
        }
    }
    appended
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }

    struct EnvVarGuard {
        key: String,
        previous: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &str, value: &str) -> Self {
            let previous = env::var(key).ok();
            env::set_var(key, value);
            Self {
                key: key.to_string(),
                previous,
            }
        }

        fn unset(key: &str) -> Self {
            let previous = env::var(key).ok();
            env::remove_var(key);
            Self {
                key: key.to_string(),
                previous,
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(previous) = self.previous.take() {
                env::set_var(&self.key, previous);
            } else {
                env::remove_var(&self.key);
            }
        }
    }

    #[test]
    fn test_extract_token_usage_success() {
        let output = r#"{"type":"thread.started","thread_id":"abc123"}
{"type":"turn.started"}
{"type":"item.completed","item":{"id":"item_0","type":"agent_message","text":"4"}}
{"type":"turn.completed","usage":{"input_tokens":8980,"cached_input_tokens":0,"output_tokens":90}}"#;

        let usage = extract_token_usage(output);
        assert!(usage.is_some());
        let usage = usage.unwrap();
        assert_eq!(usage.input_tokens, 8980);
        assert_eq!(usage.cached_input_tokens, 0);
        assert_eq!(usage.output_tokens, 90);
    }

    #[test]
    fn test_extract_token_usage_no_turn_completed() {
        let output = r#"{"type":"thread.started","thread_id":"abc123"}
{"type":"turn.started"}
{"type":"error","message":"Something went wrong"}"#;

        let usage = extract_token_usage(output);
        assert!(usage.is_none());
    }

    #[test]
    fn test_extract_token_usage_no_usage_field() {
        let output = r#"{"type":"turn.completed"}"#;

        let usage = extract_token_usage(output);
        assert!(usage.is_none());
    }

    #[test]
    fn test_extract_token_usage_empty_output() {
        let usage = extract_token_usage("");
        assert!(usage.is_none());
    }

    #[test]
    fn test_extract_token_usage_with_errors_in_output() {
        // Real-world output with errors before success
        let output = r#"{"type":"thread.started","thread_id":"019ca608-b971-71a3-abfd-4cf287a3acdf"}
{"type":"turn.started"}
{"type":"error","message":"Reconnecting... 1/5"}
{"type":"error","message":"Reconnecting... 2/5"}
{"type":"item.completed","item":{"id":"item_0","type":"agent_message","text":"Done"}}
{"type":"turn.completed","usage":{"input_tokens":1000,"output_tokens":50}}"#;

        let usage = extract_token_usage(output);
        assert!(usage.is_some());
        let usage = usage.unwrap();
        assert_eq!(usage.input_tokens, 1000);
        assert_eq!(usage.output_tokens, 50);
    }

    #[test]
    fn test_detect_codex_runtime_failure_from_task_complete_failed() {
        let output = r#"{"type":"event_msg","payload":{"type":"task_complete","status":"failed","exit_code":101,"last_agent_message":"compile failed"}} "#;
        let failure = detect_codex_runtime_failure(output).expect("expected failure");
        assert_eq!(failure.status_code, Some(101));
        assert!(failure.message.contains("status=failed"));
        assert!(failure.message.contains("exit_code=101"));
        assert!(failure.message.contains("compile failed"));
    }

    #[test]
    fn test_detect_codex_runtime_failure_from_turn_aborted() {
        let output =
            r#"{"type":"event_msg","payload":{"type":"turn_aborted","reason":"interrupted"}}"#;
        let failure = detect_codex_runtime_failure(output).expect("expected failure");
        assert_eq!(failure.status_code, None);
        assert!(failure.message.contains("turn aborted"));
        assert!(failure.message.contains("interrupted"));
    }

    #[test]
    fn test_detect_codex_runtime_failure_ignores_terminal_success() {
        let output = r#"{"type":"event_msg","payload":{"type":"task_complete","status":"failed","exit_code":101}}
{"type":"event_msg","payload":{"type":"task_complete","status":"success","exit_code":0}}"#;
        assert!(detect_codex_runtime_failure(output).is_none());
    }

    #[test]
    fn test_detect_codex_runtime_failure_none_when_success_only() {
        let output = r#"{"type":"event_msg","payload":{"type":"task_complete","status":"success","exit_code":0}}"#;
        assert!(detect_codex_runtime_failure(output).is_none());
    }

    #[test]
    fn test_extract_assistant_text_from_jsonl_item_completed() {
        let output = r#"{"type":"item.completed","item":{"id":"item_0","type":"agent_message","text":"hello\nSCHEDULED_TASKS_JSON_BEGIN\n[{\"type\":\"send_email\",\"delay_seconds\":60,\"subject\":\"x\",\"html_path\":\"x.html\"}]\nSCHEDULED_TASKS_JSON_END"}}"#;

        let parsed = extract_assistant_text_from_jsonl(output);
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert!(parsed.contains("SCHEDULED_TASKS_JSON_BEGIN"));
        assert!(parsed.contains("\"delay_seconds\":60"));
    }

    #[test]
    fn test_extract_assistant_text_from_jsonl_response_item_message() {
        let output = r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"SCHEDULER_ACTIONS_JSON_BEGIN\n[{\"action\":\"cancel\",\"task_ids\":[\"a\"]}]\nSCHEDULER_ACTIONS_JSON_END"}]}}"#;

        let parsed = extract_assistant_text_from_jsonl(output);
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert!(parsed.contains("SCHEDULER_ACTIONS_JSON_BEGIN"));
        assert!(parsed.contains("\"action\":\"cancel\""));
    }

    #[test]
    fn test_parse_scheduling_from_outputs_reads_stderr_jsonl() {
        let stderr = r#"{"type":"item.completed","item":{"id":"item_0","type":"agent_message","text":"SCHEDULED_TASKS_JSON_BEGIN\n[{\"type\":\"send_email\",\"delay_seconds\":60,\"subject\":\"x\",\"html_path\":\"x.html\"}]\nSCHEDULED_TASKS_JSON_END"}}"#;
        let combined = format!("{stderr}\n");
        let (tasks, task_error, actions, action_error) =
            parse_scheduling_from_outputs("", stderr, &combined, Path::new("/tmp/workspace"));
        assert_eq!(tasks.len(), 1);
        assert!(task_error.is_none());
        assert!(actions.is_empty());
        assert!(action_error.is_none());
    }

    #[test]
    fn test_parse_scheduling_from_outputs_ignores_prompt_markers_without_assistant() {
        let prompt_like = concat!(
            "SCHEDULED_TASKS_JSON_BEGIN\n",
            "<JSON array here>\n",
            "SCHEDULED_TASKS_JSON_END\n",
            "SCHEDULER_ACTIONS_JSON_BEGIN\n",
            "<JSON array here>\n",
            "SCHEDULER_ACTIONS_JSON_END\n"
        );
        let (tasks, task_error, actions, action_error) =
            parse_scheduling_from_outputs("", "", prompt_like, Path::new("/tmp/workspace"));
        assert!(tasks.is_empty());
        assert!(actions.is_empty());
        assert!(task_error.is_none());
        assert!(action_error.is_none());
    }

    #[test]
    fn test_parse_scheduling_from_outputs_falls_back_to_recent_session_file() {
        let _lock = env_lock();
        let temp_root =
            std::env::temp_dir().join(format!("codex-session-fallback-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_root);
        let session_dir = temp_root.join(".codex/sessions/2026/03/01");
        fs::create_dir_all(&session_dir).expect("create session dir");
        let workspace = Path::new("/tmp/fallback-workspace");

        let session_path = session_dir.join("rollout-test.jsonl");
        let session_jsonl = format!(
            "{{\"type\":\"response_item\",\"payload\":{{\"type\":\"message\",\"role\":\"user\",\"content\":[{{\"type\":\"input_text\",\"text\":\"workspace: {}\"}}]}}}}\n{{\"type\":\"item.completed\",\"item\":{{\"id\":\"item_0\",\"type\":\"agent_message\",\"text\":\"SCHEDULED_TASKS_JSON_BEGIN\\n[{{\\\"type\\\":\\\"send_email\\\",\\\"delay_seconds\\\":60,\\\"subject\\\":\\\"fallback\\\",\\\"html_path\\\":\\\"x.html\\\"}}]\\nSCHEDULED_TASKS_JSON_END\"}}}}\n",
            workspace.display()
        );
        fs::write(&session_path, session_jsonl).expect("write session");

        let _home_guard = EnvVarGuard::set("HOME", temp_root.to_string_lossy().as_ref());
        let invalid_stdout = r#"{"type":"item.completed","item":{"id":"item_0","type":"agent_message","text":"SCHEDULED_TASKS_JSON_BEGIN\n<JSON>\nSCHEDULED_TASKS_JSON_END"}}"#;
        let (tasks, task_error, actions, action_error) =
            parse_scheduling_from_outputs(invalid_stdout, "", invalid_stdout, workspace);

        assert_eq!(tasks.len(), 1);
        assert!(task_error.is_none());
        assert!(actions.is_empty());
        assert!(action_error.is_none());

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn test_collect_payment_env_overrides_uses_employee_prefix_fallback() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::unset("GOATX402_API_URL"),
            EnvVarGuard::unset("GOATX402_API_KEY"),
            EnvVarGuard::unset("EMPLOYEE_PAYMENT_ENV_PREFIX"),
            EnvVarGuard::unset("PAYMENT_ENV_PREFIX"),
            EnvVarGuard::unset("EMPLOYEE_GITHUB_ENV_PREFIX"),
            EnvVarGuard::unset("GITHUB_ENV_PREFIX"),
            EnvVarGuard::set("EMPLOYEE_ID", "little_bear"),
            EnvVarGuard::set("OLIVER_GOATX402_API_URL", "https://example.x402.test"),
            EnvVarGuard::set("OLIVER_GOATX402_API_KEY", "api-key-prefixed"),
        ];

        let overrides = collect_payment_env_overrides();
        assert!(overrides
            .iter()
            .any(|(k, v)| k == "GOATX402_API_URL" && v == "https://example.x402.test"));
        assert!(overrides
            .iter()
            .any(|(k, v)| k == "GOATX402_API_KEY" && v == "api-key-prefixed"));
    }

    #[test]
    fn test_collect_payment_env_overrides_prefers_unprefixed_values() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("EMPLOYEE_PAYMENT_ENV_PREFIX", "OLIVER"),
            EnvVarGuard::set("GOATX402_API_KEY", "api-key-global"),
            EnvVarGuard::set("OLIVER_GOATX402_API_KEY", "api-key-prefixed"),
        ];

        let overrides = collect_payment_env_overrides();
        assert!(overrides
            .iter()
            .any(|(k, v)| k == "GOATX402_API_KEY" && v == "api-key-global"));
    }

    #[test]
    fn test_codex_sandbox_mode_prefers_staging_targeted_keys() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
            EnvVarGuard::set("CODEX_SANDBOX_MODE", "workspace-write"),
            EnvVarGuard::set("RUN_TASK_CODEX_SANDBOX_MODE", "workspace-write"),
            EnvVarGuard::set("STAGING_CODEX_SANDBOX_MODE", "danger-full-access"),
            EnvVarGuard::set(
                "STAGING_RUN_TASK_CODEX_SANDBOX_MODE",
                "read-only",
            ),
        ];

        // STAGING_CODEX_SANDBOX_MODE has higher priority than STAGING_RUN_TASK_CODEX_SANDBOX_MODE.
        assert_eq!(codex_sandbox_mode(), "danger-full-access");
    }

    #[test]
    fn test_codex_bypass_sandbox_respects_staging_targeted_key() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
            EnvVarGuard::set("CODEX_BYPASS_SANDBOX", "0"),
            EnvVarGuard::set("STAGING_CODEX_BYPASS_SANDBOX", "1"),
        ];

        assert!(codex_bypass_sandbox());
    }

    #[test]
    fn test_resolve_execution_backend_defaults_to_local() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::unset("RUN_TASK_EXECUTION_BACKEND"),
            EnvVarGuard::unset("STAGING_RUN_TASK_EXECUTION_BACKEND"),
            EnvVarGuard::unset("DEPLOY_TARGET"),
        ];
        assert_eq!(resolve_execution_backend(), ExecutionBackend::Local);
    }

    #[test]
    fn test_resolve_execution_backend_auto_staging_uses_azure_aci() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::unset("RUN_TASK_EXECUTION_BACKEND"),
            EnvVarGuard::unset("STAGING_RUN_TASK_EXECUTION_BACKEND"),
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
        ];
        assert_eq!(resolve_execution_backend(), ExecutionBackend::AzureAci);
    }

    #[test]
    fn test_resolve_execution_backend_auto_production_uses_azure_aci() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::unset("RUN_TASK_EXECUTION_BACKEND"),
            EnvVarGuard::unset("STAGING_RUN_TASK_EXECUTION_BACKEND"),
            EnvVarGuard::set("DEPLOY_TARGET", "production"),
        ];
        assert_eq!(resolve_execution_backend(), ExecutionBackend::AzureAci);
    }

    #[test]
    fn test_resolve_execution_backend_staging_uses_staging_key() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
            EnvVarGuard::set("RUN_TASK_EXECUTION_BACKEND", "local"),
            EnvVarGuard::set("STAGING_RUN_TASK_EXECUTION_BACKEND", "azure_aci"),
        ];
        assert_eq!(resolve_execution_backend(), ExecutionBackend::AzureAci);
    }

    #[test]
    fn test_load_azure_aci_config_staging_uses_staging_keys() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
            EnvVarGuard::set("RUN_TASK_AZURE_ACI_RESOURCE_GROUP", "prod-rg"),
            EnvVarGuard::set("STAGING_RUN_TASK_AZURE_ACI_RESOURCE_GROUP", "stg-rg"),
            EnvVarGuard::set(
                "RUN_TASK_AZURE_ACI_IMAGE",
                "prod.azurecr.io/dowhiz-service:prod",
            ),
            EnvVarGuard::set(
                "STAGING_RUN_TASK_AZURE_ACI_IMAGE",
                "stg.azurecr.io/dowhiz-service:staging",
            ),
            EnvVarGuard::set("RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT", "/prod/run_task"),
            EnvVarGuard::set(
                "STAGING_RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT",
                "/stg/run_task",
            ),
            EnvVarGuard::set("RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT", "prodaccount"),
            EnvVarGuard::set("STAGING_RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT", "stgaccount"),
            EnvVarGuard::set("RUN_TASK_AZURE_ACI_STORAGE_KEY", "prod-key"),
            EnvVarGuard::set("STAGING_RUN_TASK_AZURE_ACI_STORAGE_KEY", "stg-key"),
            EnvVarGuard::set("RUN_TASK_AZURE_ACI_FILE_SHARE", "prod-share"),
            EnvVarGuard::set("STAGING_RUN_TASK_AZURE_ACI_FILE_SHARE", "stg-share"),
        ];

        let config = load_azure_aci_config().expect("load staging aci config");
        assert_eq!(config.resource_group, "stg-rg");
        assert_eq!(config.image, "stg.azurecr.io/dowhiz-service:staging");
        assert_eq!(config.host_share_root, PathBuf::from("/stg/run_task"));
        assert_eq!(config.storage_account, "stgaccount");
        assert_eq!(config.storage_key, "stg-key");
        assert_eq!(config.file_share, "stg-share");
    }

    #[test]
    fn test_load_azure_aci_config_staging_requires_staging_keys() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
            EnvVarGuard::set("RUN_TASK_AZURE_ACI_RESOURCE_GROUP", "prod-rg"),
            EnvVarGuard::unset("STAGING_RUN_TASK_AZURE_ACI_RESOURCE_GROUP"),
            EnvVarGuard::set(
                "STAGING_RUN_TASK_AZURE_ACI_IMAGE",
                "stg.azurecr.io/dowhiz-service:staging",
            ),
            EnvVarGuard::set(
                "STAGING_RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT",
                "/stg/run_task",
            ),
            EnvVarGuard::set("STAGING_RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT", "stgaccount"),
            EnvVarGuard::set("STAGING_RUN_TASK_AZURE_ACI_STORAGE_KEY", "stg-key"),
        ];

        let err = load_azure_aci_config().expect_err("staging should require STAGING_* key");
        match err {
            RunTaskError::MissingEnv { key } => {
                assert_eq!(key, "STAGING_RUN_TASK_AZURE_ACI_RESOURCE_GROUP")
            }
            other => panic!("unexpected error variant: {other}"),
        }
    }

    #[test]
    fn test_ensure_local_execution_allowed_rejects_staging_without_override() {
        let _lock = env_lock();
        let _guards = vec![
            EnvVarGuard::set("DEPLOY_TARGET", "staging"),
            EnvVarGuard::unset("RUN_TASK_ALLOW_LOCAL_EXECUTION"),
        ];
        let err = ensure_local_execution_allowed()
            .expect_err("staging should reject local execution by default");
        match err {
            RunTaskError::LocalExecutionForbidden { deploy_target } => {
                assert_eq!(deploy_target, "staging")
            }
            other => panic!("unexpected error variant: {other}"),
        }
    }

    #[test]
    fn test_build_aci_container_name_is_unique() {
        let first = build_aci_container_name();
        let second = build_aci_container_name();
        assert_ne!(first, second);
    }
}
