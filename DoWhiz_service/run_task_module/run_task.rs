use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::Deserialize;

const CODEX_CONFIG_MARKER: &str = "# IMPORTANT: Use your Azure *deployment name* here";
const CODEX_CONFIG_BLOCK_TEMPLATE: &str = r#"# IMPORTANT: Use your Azure *deployment name* here (e.g., "gpt-5.2-codex")
model = "{model_name}"
model_provider = "azure"
model_reasoning_effort = "xhigh"

[model_providers.azure]
name = "Azure OpenAI"
base_url = "{azure_endpoint}"
env_key = "AZURE_OPENAI_API_KEY_BACKUP"
wire_api = "responses"
"#;
const SCHEDULED_TASKS_BEGIN: &str = "SCHEDULED_TASKS_JSON_BEGIN";
const SCHEDULED_TASKS_END: &str = "SCHEDULED_TASKS_JSON_END";

#[derive(Debug, Clone)]
pub struct RunTaskParams {
    pub workspace_dir: PathBuf,
    pub input_email_dir: PathBuf,
    pub input_attachments_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub reference_dir: PathBuf,
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
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScheduledTaskRequest {
    SendEmail(ScheduledSendEmailTask),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduledSendEmailTask {
    pub subject: String,
    pub html_path: String,
    pub attachments_dir: Option<String>,
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

#[derive(Debug, Clone)]
pub struct RunTaskOutput {
    pub reply_html_path: PathBuf,
    pub reply_attachments_dir: PathBuf,
    pub codex_output: String,
    pub scheduled_tasks: Vec<ScheduledTaskRequest>,
    pub scheduled_tasks_error: Option<String>,
}

#[derive(Debug)]
pub enum RunTaskError {
    Io(io::Error),
    MissingEnv { key: &'static str },
    InvalidPath {
        label: &'static str,
        path: PathBuf,
        reason: &'static str,
    },
    CodexNotFound,
    CodexFailed { status: Option<i32>, output: String },
    OutputMissing { path: PathBuf },
}

impl fmt::Display for RunTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunTaskError::Io(err) => write!(f, "I/O error: {}", err),
            RunTaskError::MissingEnv { key } => write!(f, "Missing required environment variable: {}", key),
            RunTaskError::InvalidPath { label, path, reason } => {
                write!(f, "Invalid {} path ({}): {}", label, path.display(), reason)
            }
            RunTaskError::CodexNotFound => write!(f, "Codex CLI not found on PATH."),
            RunTaskError::CodexFailed { status, output } => write!(
                f,
                "Codex CLI failed (status: {:?}). Output tail:\n{}",
                status, output
            ),
            RunTaskError::OutputMissing { path } => {
                write!(f, "Expected output not found: {}", path.display())
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
    let request = RunTaskRequest {
        workspace_dir: &params.workspace_dir,
        input_email_dir: &params.input_email_dir,
        input_attachments_dir: &params.input_attachments_dir,
        memory_dir: &params.memory_dir,
        reference_dir: &params.reference_dir,
        model_name: params.model_name.as_str(),
    };

    let (reply_html_path, reply_attachments_dir) = prepare_workspace(&request)?;

    if params.codex_disabled {
        write_placeholder_reply(&reply_html_path)?;
        return Ok(RunTaskOutput {
            reply_html_path,
            reply_attachments_dir,
            codex_output: "codex disabled".to_string(),
            scheduled_tasks: Vec::new(),
            scheduled_tasks_error: None,
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

    let api_key = env::var("AZURE_OPENAI_API_KEY_BACKUP")
        .map_err(|_| RunTaskError::MissingEnv { key: "AZURE_OPENAI_API_KEY_BACKUP" })?;
    if api_key.trim().is_empty() {
        return Err(RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP",
        });
    }
    let azure_endpoint = env::var("AZURE_OPENAI_ENDPOINT_BACKUP")
        .map_err(|_| RunTaskError::MissingEnv { key: "AZURE_OPENAI_ENDPOINT_BACKUP" })?;
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

    ensure_codex_config(&model_name, &azure_endpoint)?;

    let prompt = build_prompt(
        request.input_email_dir,
        request.input_attachments_dir,
        request.memory_dir,
        request.reference_dir,
    );

    let mut cmd = Command::new("codex");
    cmd.arg("exec")
        .arg("--skip-git-repo-check")
        .arg("-m")
        .arg(&model_name)
        .arg("-c")
        .arg("web_search=\"disabled\"")
        .arg("-c")
        .arg("model_providers.azure.env_key=\"AZURE_OPENAI_API_KEY_BACKUP\"")
        .arg("--cd")
        .arg(request.workspace_dir)
        .arg("--dangerously-bypass-approvals-and-sandbox")
        .arg(prompt)
        .env("AZURE_OPENAI_API_KEY_BACKUP", api_key)
        .current_dir(request.workspace_dir);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Err(RunTaskError::CodexNotFound),
        Err(err) => return Err(RunTaskError::Io(err)),
    };

    let mut combined_output = String::new();
    combined_output.push_str(&String::from_utf8_lossy(&output.stdout));
    combined_output.push_str(&String::from_utf8_lossy(&output.stderr));
    let (scheduled_tasks, scheduled_tasks_error) = extract_scheduled_tasks(&combined_output);
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(RunTaskError::CodexFailed {
            status: output.status.code(),
            output: output_tail,
        });
    }

    if !reply_html_path.exists() {
        return Err(RunTaskError::OutputMissing {
            path: reply_html_path,
        });
    }

    Ok(RunTaskOutput {
        reply_html_path,
        reply_attachments_dir,
        codex_output: output_tail,
        scheduled_tasks,
        scheduled_tasks_error,
    })
}

fn prepare_workspace(request: &RunTaskRequest<'_>) -> Result<(PathBuf, PathBuf), RunTaskError> {
    ensure_workspace_dir(request.workspace_dir)?;

    let _input_email_dir =
        resolve_rel_dir(request.workspace_dir, request.input_email_dir, "input_email_dir")?;
    let _input_attachments_dir = resolve_rel_dir(
        request.workspace_dir,
        request.input_attachments_dir,
        "input_attachments_dir",
    )?;
    let _memory_dir = resolve_rel_dir(request.workspace_dir, request.memory_dir, "memory_dir")?;
    let _reference_dir =
        resolve_rel_dir(request.workspace_dir, request.reference_dir, "reference_dir")?;

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

fn ensure_codex_config(model_name: &str, azure_endpoint: &str) -> Result<(), RunTaskError> {
    let home = env::var("HOME")
        .map_err(|_| RunTaskError::MissingEnv { key: "HOME" })?;
    let config_path = PathBuf::from(home).join(".codex").join("config.toml");
    let config_dir = config_path
        .parent()
        .ok_or(RunTaskError::InvalidPath {
            label: "codex_config_dir",
            path: config_path.clone(),
            reason: "could not resolve config directory",
        })?;
    fs::create_dir_all(config_dir)?;

    let endpoint = normalize_azure_endpoint(azure_endpoint);
    let block = CODEX_CONFIG_BLOCK_TEMPLATE
        .replace("{model_name}", model_name)
        .replace("{azure_endpoint}", &endpoint);

    let existing = if config_path.exists() {
        fs::read_to_string(&config_path)?
    } else {
        String::new()
    };

    let updated = update_config_block(&existing, &block);
    fs::write(config_path, updated)?;
    Ok(())
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
        if let Some(block_end_index) =
            existing[marker_index..].find("wire_api = \"responses\"")
        {
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

fn extract_scheduled_tasks(
    output: &str,
) -> (Vec<ScheduledTaskRequest>, Option<String>) {
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

fn build_prompt(
    input_email_dir: &Path,
    input_attachments_dir: &Path,
    memory_dir: &Path,
    reference_dir: &Path,
) -> String {
    format!(
        "You are Oliver, a digital assistant at DoWhiz.\n\
You are running inside a workspace on disk. Use only files in this workspace.\n\
\n\
Inputs (relative to workspace root):\n\
- Incoming email dir: {input_email}\n\
- Incoming attachments dir: {input_attachments}\n\
- Memory dir: {memory}\n\
- Reference dir: {reference}\n\
\n\
Task:\n\
1) Read the incoming email files to understand what to reply to. The incoming email\n\
   dir may include multiple message entries under entries/; read all email.txt files,\n\
   plus the latest incoming_email/email.txt.\n\
   If there are attachments for specific messages, they may live under\n\
   incoming_attachments/entries/.\n\
2) Use memory and reference material for context when helpful.\n\
3) Write the reply as a full HTML email to reply_email_draft.html in the workspace root.\n\
   You may use drafts/ as history for reference, but only write the final reply to\n\
   reply_email_draft.html.\n\
4) Place any files to attach in reply_email_attachments/ (create it if missing).\n\
\n\
Optional scheduling:\n\
- If the user asks you to send a follow-up email later, create the follow-up draft HTML in the\n\
  workspace root and any attachment directory it needs.\n\
- Then output a schedule block to stdout at the end of your response, exactly in this format:\n\
  {schedule_begin}\n\
  [{{\"type\":\"send_email\",\"delay_minutes\":15,\"subject\":\"Quick reminder\",\"html_path\":\"reminder_email_draft.html\",\"attachments_dir\":\"reminder_email_attachments\",\"to\":[\"you@example.com\"],\"cc\":[],\"bcc\":[]}}]\n\
  {schedule_end}\n\
- Use delay_minutes for \"N minutes later\" requests. Use run_at (RFC3339 UTC) only for specific\n\
  times.\n\
- If no follow-up is needed, output an empty array in the schedule block.\n\
- Do not write scheduled_tasks.json or any other scheduling file.\n\
\n\
Rules:\n\
- Do not modify input directories.\n\
- If attachments include version suffixes like _v1, _v2, use the highest version.\n\
- Only write reply_email_draft.html and files under reply_email_attachments/.\n\
- If scheduling follow-ups, you may also write the follow-up draft HTML and any attachment\n\
  dirs referenced in the schedule block.\n\
- Keep the reply concise, friendly, and professional.\n",
        input_email = input_email_dir.display(),
        input_attachments = input_attachments_dir.display(),
        memory = memory_dir.display(),
        reference = reference_dir.display(),
        schedule_begin = SCHEDULED_TASKS_BEGIN,
        schedule_end = SCHEDULED_TASKS_END,
    )
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
