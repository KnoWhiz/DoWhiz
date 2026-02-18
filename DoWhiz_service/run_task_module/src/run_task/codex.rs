use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::constants::{
    CODEX_CONFIG_BLOCK_TEMPLATE, CODEX_CONFIG_MARKER, DOCKER_CODEX_HOME_DIR, DOCKER_WORKSPACE_DIR,
};
use super::docker::{docker_cli_available, ensure_docker_image_available};
use super::env::{env_enabled, read_env_list, read_env_trimmed};
use super::errors::RunTaskError;
use super::github_auth::{ensure_github_cli_auth, resolve_github_auth};
use super::prompt::{build_prompt, load_memory_context};
use super::scheduled::{extract_scheduled_tasks, extract_scheduler_actions};
use super::types::{RunTaskOutput, RunTaskRequest};
use super::utils::tail_string;
use super::workspace::{canonicalize_dir, workspace_path_in_container};

pub(super) fn run_codex_task(
    request: RunTaskRequest<'_>,
    runner: &str,
    reply_html_path: PathBuf,
    reply_attachments_dir: PathBuf,
) -> Result<RunTaskOutput, RunTaskError> {
    super::env::load_env_sources(request.workspace_dir)?;
    let docker_image = read_env_trimmed("RUN_TASK_DOCKER_IMAGE");
    let docker_requested = docker_image.is_some() || env_enabled("RUN_TASK_USE_DOCKER");
    let docker_available = docker_requested && docker_cli_available();
    let docker_required = env_enabled("RUN_TASK_DOCKER_REQUIRED");
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
    // Bypass sandbox for GoogleDocs tasks to allow network access for Google APIs
    let channel_lower = request.channel.to_lowercase();
    let is_google_docs = channel_lower == "google_docs" || channel_lower == "googledocs";
    let bypass_sandbox = codex_bypass_sandbox() || use_docker || is_google_docs;
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
        request.workspace_dir,
        runner,
        &memory_context,
        !request.reply_to.is_empty(),
        request.channel,
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

    // Only check for reply file if a reply was expected
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
    })
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
    env_enabled("CODEX_BYPASS_SANDBOX") || env_enabled("CODEX_DANGEROUSLY_BYPASS_SANDBOX")
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
