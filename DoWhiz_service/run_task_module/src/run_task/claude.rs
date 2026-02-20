use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

use super::constants::{CLAUDE_FOUNDRY_RESOURCE_DEFAULT, DEFAULT_CLAUDE_MODEL};
use super::env::load_env_sources;
use super::errors::RunTaskError;
use super::github_auth::{ensure_github_cli_auth, resolve_github_auth};
use super::prompt::{build_prompt, load_memory_context};
use super::scheduled::{extract_scheduled_tasks, extract_scheduler_actions};
use super::types::{RunTaskOutput, RunTaskRequest};
use super::utils::{run_command_with_timeout, run_task_timeout, tail_string};

pub(super) fn run_claude_task(
    request: RunTaskRequest<'_>,
    runner: &str,
    reply_html_path: std::path::PathBuf,
    reply_attachments_dir: std::path::PathBuf,
) -> Result<RunTaskOutput, RunTaskError> {
    load_env_sources(request.workspace_dir)?;
    let github_auth = resolve_github_auth(None)?;

    let api_key =
        env::var("AZURE_OPENAI_API_KEY_BACKUP").map_err(|_| RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        })?;
    if api_key.trim().is_empty() {
        return Err(RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        });
    }

    let model_name = if request.model_name.trim().is_empty() {
        env::var("CLAUDE_MODEL").unwrap_or_else(|_| DEFAULT_CLAUDE_MODEL.to_string())
    } else {
        request.model_name.to_string()
    };

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

    ensure_github_cli_auth(&github_auth)?;
    let mut env_overrides = prepare_claude_env(&api_key, &model_name)?;
    env_overrides.extend(github_auth.env_overrides.clone());
    if let Some(askpass_path) = github_auth.askpass_path.as_ref() {
        env_overrides.push((
            "GIT_ASKPASS".to_string(),
            askpass_path.to_string_lossy().into_owned(),
        ));
        env_overrides.push(("GIT_TERMINAL_PROMPT".to_string(), "0".to_string()));
    }
    let output = run_claude_command(request.workspace_dir, &prompt, &model_name, &env_overrides)?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut combined_output = String::new();
    combined_output.push_str(&stdout);
    combined_output.push_str(&stderr);
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(RunTaskError::ClaudeFailed {
            status: output.status.code(),
            output: output_tail,
        });
    }

    let (assistant_text, _logs) = extract_claude_text(&stdout);
    if assistant_text.trim().is_empty() {
        return Err(RunTaskError::ClaudeFailed {
            status: output.status.code(),
            output: output_tail,
        });
    }
    let (scheduled_tasks, scheduled_tasks_error) = extract_scheduled_tasks(&assistant_text);
    let (scheduler_actions, scheduler_actions_error) = extract_scheduler_actions(&assistant_text);
    let assistant_tail = tail_string(&assistant_text, 2000);

    // Only check for reply file if a reply was expected
    if !request.reply_to.is_empty() && !reply_html_path.exists() {
        return Err(RunTaskError::OutputMissing {
            path: reply_html_path,
            output: assistant_tail,
        });
    }

    Ok(RunTaskOutput {
        reply_html_path,
        reply_attachments_dir,
        codex_output: assistant_tail,
        scheduled_tasks,
        scheduled_tasks_error,
        scheduler_actions,
        scheduler_actions_error,
    })
}

fn prepare_claude_env(
    api_key: &str,
    model_name: &str,
) -> Result<Vec<(String, String)>, RunTaskError> {
    let foundry_resource = env::var("ANTHROPIC_FOUNDRY_RESOURCE")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| CLAUDE_FOUNDRY_RESOURCE_DEFAULT.to_string());
    let default_opus = env::var("ANTHROPIC_DEFAULT_OPUS_MODEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_CLAUDE_MODEL.to_string());
    let default_sonnet = env::var("ANTHROPIC_DEFAULT_SONNET_MODEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "claude-sonnet-4-5".to_string());
    let default_haiku = env::var("ANTHROPIC_DEFAULT_HAIKU_MODEL")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "claude-haiku-4-5".to_string());

    ensure_claude_settings(
        model_name,
        api_key,
        &foundry_resource,
        &default_opus,
        &default_sonnet,
        &default_haiku,
    )?;

    // Get current PATH and prepend our custom bin directory for tools like google-docs
    let current_path = env::var("PATH").unwrap_or_default();
    // Look for DOWHIZ_BIN_DIR env var, or use default location relative to crate
    let dowhiz_bin_dir = env::var("DOWHIZ_BIN_DIR")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| {
            // Default: assume bin/ is sibling to scheduler_module
            let manifest_dir = env!("CARGO_MANIFEST_DIR");
            let parent = Path::new(manifest_dir).parent().unwrap_or(Path::new("."));
            parent.join("bin").to_string_lossy().into_owned()
        });
    let extended_path = format!("{}:{}", dowhiz_bin_dir, current_path);

    Ok(vec![
        (
            "AZURE_OPENAI_API_KEY_BACKUP".to_string(),
            api_key.to_string(),
        ),
        ("CLAUDE_CODE_USE_FOUNDRY".to_string(), "1".to_string()),
        ("ANTHROPIC_FOUNDRY_RESOURCE".to_string(), foundry_resource),
        ("ANTHROPIC_FOUNDRY_API_KEY".to_string(), api_key.to_string()),
        ("ANTHROPIC_DEFAULT_OPUS_MODEL".to_string(), default_opus),
        ("ANTHROPIC_DEFAULT_SONNET_MODEL".to_string(), default_sonnet),
        ("ANTHROPIC_DEFAULT_HAIKU_MODEL".to_string(), default_haiku),
        ("PATH".to_string(), extended_path),
    ])
}

fn ensure_claude_settings(
    model_name: &str,
    api_key: &str,
    foundry_resource: &str,
    default_opus: &str,
    default_sonnet: &str,
    default_haiku: &str,
) -> Result<(), RunTaskError> {
    let home = env::var("HOME").map_err(|_| RunTaskError::MissingEnv { key: "HOME" })?;
    let settings_dir = std::path::PathBuf::from(home).join(".claude");
    fs::create_dir_all(&settings_dir)?;
    let settings_path = settings_dir.join("settings.json");
    let payload = serde_json::json!({
        "env": {
            "CLAUDE_CODE_USE_FOUNDRY": "1",
            "ANTHROPIC_FOUNDRY_RESOURCE": foundry_resource,
            "ANTHROPIC_FOUNDRY_API_KEY": api_key,
            "ANTHROPIC_DEFAULT_OPUS_MODEL": default_opus,
            "ANTHROPIC_DEFAULT_SONNET_MODEL": default_sonnet,
            "ANTHROPIC_DEFAULT_HAIKU_MODEL": default_haiku,
        },
        "model": model_name,
    });
    let rendered = serde_json::to_string_pretty(&payload)
        .map_err(|err| RunTaskError::Io(io::Error::new(io::ErrorKind::Other, err.to_string())))?;
    fs::write(settings_path, format!("{}\n", rendered))?;
    Ok(())
}

fn run_claude_command(
    workspace_dir: &Path,
    prompt: &str,
    model_name: &str,
    env_overrides: &[(String, String)],
) -> Result<std::process::Output, RunTaskError> {
    let timeout = run_task_timeout();
    match run_command_with_timeout(
        build_claude_command(workspace_dir, prompt, model_name, env_overrides),
        timeout,
        "claude",
    ) {
        Ok(output) => return Ok(output),
        Err(RunTaskError::Io(err)) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => return Err(err),
    }

    ensure_claude_cli_installed(env_overrides)?;
    match run_command_with_timeout(
        build_claude_command(workspace_dir, prompt, model_name, env_overrides),
        timeout,
        "claude",
    ) {
        Ok(output) => Ok(output),
        Err(RunTaskError::Io(err)) if err.kind() == io::ErrorKind::NotFound => {
            Err(RunTaskError::ClaudeNotFound)
        }
        Err(err) => Err(err),
    }
}

fn build_claude_command(
    workspace_dir: &Path,
    prompt: &str,
    model_name: &str,
    env_overrides: &[(String, String)],
) -> Command {
    let max_turns = claude_max_turns();
    let mut cmd = Command::new("claude");
    cmd.arg("-p")
        .arg("--output-format")
        .arg("stream-json")
        .arg("--include-partial-messages")
        .arg("--verbose")
        .arg("--model")
        .arg(model_name)
        .arg("--allowedTools")
        .arg("Read,Glob,Grep,Bash")
        .arg("--max-turns")
        .arg(max_turns.to_string())
        .arg("--dangerously-skip-permissions")
        .arg(prompt)
        .current_dir(workspace_dir);
    apply_env_pairs(&mut cmd, env_overrides);
    cmd
}

fn claude_max_turns() -> u32 {
    env::var("CLAUDE_MAX_TURNS")
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(10)
}

fn ensure_claude_cli_installed(env_overrides: &[(String, String)]) -> Result<(), RunTaskError> {
    let mut cmd = Command::new("npm");
    cmd.args(["i", "-g", "@anthropic-ai/claude-code"]);
    apply_env_pairs(&mut cmd, env_overrides);
    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::ClaudeInstallFailed {
                output: "npm not found on PATH".to_string(),
            })
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        return Err(RunTaskError::ClaudeInstallFailed {
            output: tail_string(&combined, 2000),
        });
    }
    Ok(())
}

fn apply_env_pairs(cmd: &mut Command, overrides: &[(String, String)]) {
    for (key, value) in overrides {
        cmd.env(key, value);
    }
}

fn extract_claude_text(raw: &str) -> (String, Vec<String>) {
    let mut text = String::new();
    let mut logs = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let event: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(value) => value,
            Err(_) => {
                logs.push(trimmed.to_string());
                continue;
            }
        };
        let event_type = event
            .get("type")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        if matches!(
            event_type,
            "text_delta" | "message_delta" | "content_block_delta" | "message_stop" | "result"
        ) {
            if let Some(fragment) = extract_claude_fragment(&event) {
                text.push_str(&fragment);
            }
        }
    }
    (text, logs)
}

fn extract_claude_fragment(event: &serde_json::Value) -> Option<String> {
    if let Some(text) = event.get("text").and_then(|value| value.as_str()) {
        return Some(text.to_string());
    }
    if let Some(text) = event
        .get("delta")
        .and_then(|value| value.get("text"))
        .and_then(|value| value.as_str())
    {
        return Some(text.to_string());
    }
    if let Some(text) = event
        .get("message")
        .and_then(|value| value.get("text"))
        .and_then(|value| value.as_str())
    {
        return Some(text.to_string());
    }
    if let Some(text) = event.get("final_text").and_then(|value| value.as_str()) {
        return Some(text.to_string());
    }
    if let Some(text) = event.get("result").and_then(|value| value.as_str()) {
        return Some(text.to_string());
    }
    None
}
