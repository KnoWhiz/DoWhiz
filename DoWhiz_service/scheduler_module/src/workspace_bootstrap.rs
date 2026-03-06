use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::{info, warn};
use uuid::Uuid;

use crate::blob_store::get_blob_store;

const STATE_DIR_NAME: &str = ".bootstrap";
const STATE_FILE_NAME: &str = "bootstrap_state.json";
const DEFAULT_COMMAND_TIMEOUT_SECS: u64 = 300;
const MAX_OUTPUT_PREVIEW_CHARS: usize = 2_000;

const DEFAULT_ALLOWED_COMMANDS: &[&str] = &[
    "npm", "pnpm", "yarn", "cargo", "rustup", "pip", "pip3", "uv", "python", "python3",
    "node", "npx", "go", "git", "bash", "sh",
];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceBootstrapProfile {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub commands: Vec<WorkspaceBootstrapCommand>,
    #[serde(default)]
    pub files: Vec<WorkspaceBootstrapFile>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceBootstrapCommand {
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default = "default_true")]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceBootstrapFile {
    pub file_id: String,
    pub path: String,
    #[serde(default = "default_true")]
    pub required: bool,
}

#[derive(Debug, Clone)]
pub enum WorkspaceBootstrapSource {
    Account { account_id: Uuid },
    Local { bootstrap_root: PathBuf },
}

#[derive(Debug, Clone)]
pub struct WorkspaceBootstrapApplyResult {
    pub applied: bool,
    pub skipped: bool,
    pub files_applied: usize,
    pub commands_applied: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceBootstrapState {
    pub applied_at: String,
    pub profile_version: String,
    pub profile_hash: String,
    pub status: String,
    pub source: String,
    pub files_applied: usize,
    pub commands_applied: usize,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default)]
    pub command_results: Vec<WorkspaceBootstrapCommandResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceBootstrapCommandResult {
    pub command: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: u128,
    pub stdout_preview: Option<String>,
    pub stderr_preview: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceBootstrapError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid bootstrap path: {0}")]
    InvalidPath(String),
    #[error("invalid bootstrap file id: {0}")]
    InvalidFileId(String),
    #[error("bootstrap command is not allowed: {0}")]
    CommandBlocked(String),
    #[error("bootstrap command failed: {0}")]
    CommandFailed(String),
    #[error("bootstrap command timed out: {0}")]
    CommandTimeout(String),
    #[error("bootstrap profile source unavailable")]
    ProfileSourceUnavailable,
    #[error("blob store error: {0}")]
    Blob(String),
    #[error("runtime error: {0}")]
    Runtime(String),
}

fn default_true() -> bool {
    true
}

impl WorkspaceBootstrapProfile {
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty() && self.files.is_empty() && self.version.trim().is_empty()
    }
}

pub fn load_workspace_bootstrap_profile(
    account_id: Option<Uuid>,
    user_memory_dir: Option<&Path>,
) -> Option<(WorkspaceBootstrapProfile, WorkspaceBootstrapSource)> {
    if let Some(account_id) = account_id {
        match load_account_profile(account_id) {
            Ok(Some(profile)) => {
                return Some((profile, WorkspaceBootstrapSource::Account { account_id }));
            }
            Ok(None) => {
                // No account-level profile. Fall through to local fallback.
            }
            Err(err) => {
                warn!(
                    "failed to load account workspace bootstrap profile account_id={}: {}",
                    account_id, err
                );
            }
        }
    }

    let bootstrap_root = match user_memory_dir
        .and_then(Path::parent)
        .map(|user_root| user_root.join("bootstrap"))
    {
        Some(path) => path,
        None => return None,
    };

    match load_local_profile(&bootstrap_root) {
        Ok(Some(profile)) => Some((profile, WorkspaceBootstrapSource::Local { bootstrap_root })),
        Ok(None) => None,
        Err(err) => {
            warn!(
                "failed to load local workspace bootstrap profile path={}: {}",
                bootstrap_root.display(),
                err
            );
            None
        }
    }
}

pub fn apply_workspace_bootstrap(
    workspace_dir: &Path,
    profile: &WorkspaceBootstrapProfile,
    source: &WorkspaceBootstrapSource,
) -> Result<WorkspaceBootstrapApplyResult, WorkspaceBootstrapError> {
    if profile.is_empty() {
        return Ok(WorkspaceBootstrapApplyResult {
            applied: false,
            skipped: true,
            files_applied: 0,
            commands_applied: 0,
        });
    }

    let state_path = workspace_dir.join(STATE_DIR_NAME).join(STATE_FILE_NAME);
    let profile_version = normalized_profile_version(profile);
    let profile_hash = hash_profile(profile)?;

    if let Some(existing) = load_state(&state_path) {
        if existing.status == "success"
            && existing.profile_hash == profile_hash
            && existing.profile_version == profile_version
        {
            info!(
                "workspace bootstrap skipped workspace={} version={} hash={}",
                workspace_dir.display(),
                profile_version,
                profile_hash
            );
            return Ok(WorkspaceBootstrapApplyResult {
                applied: false,
                skipped: true,
                files_applied: 0,
                commands_applied: 0,
            });
        }
    }

    let mut state = WorkspaceBootstrapState {
        applied_at: Utc::now().to_rfc3339(),
        profile_version,
        profile_hash,
        status: "running".to_string(),
        source: source_label(source).to_string(),
        ..WorkspaceBootstrapState::default()
    };

    fs::create_dir_all(workspace_dir.join(STATE_DIR_NAME))?;

    for file in &profile.files {
        if let Err(err) = validate_file_id(&file.file_id) {
            if file.required {
                state.errors.push(err.to_string());
                state.status = "failed".to_string();
                write_state(&state_path, &state)?;
                return Err(err);
            }
            state.warnings.push(err.to_string());
            continue;
        }

        let relative_path = match sanitize_relative_path(&file.path) {
            Ok(path) => path,
            Err(err) => {
                if file.required {
                    state.errors.push(err.to_string());
                    state.status = "failed".to_string();
                    write_state(&state_path, &state)?;
                    return Err(err);
                }
                state.warnings.push(err.to_string());
                continue;
            }
        };

        let file_bytes = match read_bootstrap_file_bytes(source, &file.file_id) {
            Ok(bytes) => bytes,
            Err(err) => {
                let message = format!(
                    "failed to load bootstrap file {} for {}: {}",
                    file.file_id, file.path, err
                );
                if file.required {
                    state.errors.push(message);
                    state.status = "failed".to_string();
                    write_state(&state_path, &state)?;
                    return Err(err);
                }
                state.warnings.push(message);
                continue;
            }
        };

        let target_path = workspace_dir.join(&relative_path);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target_path, &file_bytes)?;
        state.files_applied += 1;
    }

    for command in &profile.commands {
        let command_display = render_command(command);
        match run_bootstrap_command(workspace_dir, command) {
            Ok(result) => {
                state.commands_applied += 1;
                state.command_results.push(result);
            }
            Err(err) => {
                let message = format!("{} ({})", command_display, err);
                if command.required {
                    state.errors.push(message);
                    state.status = "failed".to_string();
                    write_state(&state_path, &state)?;
                    return Err(err);
                }
                state.warnings.push(message);
            }
        }
    }

    state.status = "success".to_string();
    write_state(&state_path, &state)?;

    info!(
        "workspace bootstrap applied workspace={} source={} files={} commands={} version={} hash={}",
        workspace_dir.display(),
        state.source,
        state.files_applied,
        state.commands_applied,
        state.profile_version,
        state.profile_hash
    );

    Ok(WorkspaceBootstrapApplyResult {
        applied: true,
        skipped: false,
        files_applied: state.files_applied,
        commands_applied: state.commands_applied,
    })
}

fn load_account_profile(
    account_id: Uuid,
) -> Result<Option<WorkspaceBootstrapProfile>, WorkspaceBootstrapError> {
    let blob_store = get_blob_store().ok_or(WorkspaceBootstrapError::ProfileSourceUnavailable)?;
    let runtime =
        tokio::runtime::Runtime::new().map_err(|e| WorkspaceBootstrapError::Runtime(e.to_string()))?;

    let profile = runtime
        .block_on(blob_store.read_workspace_bootstrap_profile(account_id))
        .map_err(|e| WorkspaceBootstrapError::Blob(e.to_string()))?;

    if profile.is_empty() {
        return Ok(None);
    }

    Ok(Some(profile))
}

fn load_local_profile(
    bootstrap_root: &Path,
) -> Result<Option<WorkspaceBootstrapProfile>, WorkspaceBootstrapError> {
    let profile_path = bootstrap_root.join("profile.json");
    if !profile_path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(&profile_path)?;
    let profile: WorkspaceBootstrapProfile = serde_json::from_str(&raw)?;
    if profile.is_empty() {
        return Ok(None);
    }
    Ok(Some(profile))
}

fn read_bootstrap_file_bytes(
    source: &WorkspaceBootstrapSource,
    file_id: &str,
) -> Result<Vec<u8>, WorkspaceBootstrapError> {
    match source {
        WorkspaceBootstrapSource::Local { bootstrap_root } => {
            let path = bootstrap_root.join("files").join(file_id);
            fs::read(path).map_err(WorkspaceBootstrapError::Io)
        }
        WorkspaceBootstrapSource::Account { account_id } => {
            let blob_store = get_blob_store().ok_or(WorkspaceBootstrapError::ProfileSourceUnavailable)?;
            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| WorkspaceBootstrapError::Runtime(e.to_string()))?;
            runtime
                .block_on(blob_store.read_workspace_bootstrap_file(*account_id, file_id))
                .map_err(|e| WorkspaceBootstrapError::Blob(e.to_string()))
        }
    }
}

fn run_bootstrap_command(
    workspace_dir: &Path,
    command: &WorkspaceBootstrapCommand,
) -> Result<WorkspaceBootstrapCommandResult, WorkspaceBootstrapError> {
    if command.cmd.trim().is_empty() {
        return Err(WorkspaceBootstrapError::CommandFailed(
            "empty command".to_string(),
        ));
    }

    let command_base = command_basename(&command.cmd);
    if !is_command_allowed(&command_base) {
        return Err(WorkspaceBootstrapError::CommandBlocked(command_base));
    }

    let cwd = match command.cwd.as_deref() {
        Some(raw) => workspace_dir.join(sanitize_relative_path(raw)?),
        None => workspace_dir.to_path_buf(),
    };

    let mut proc = Command::new(&command.cmd);
    proc.args(&command.args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    for (key, value) in &command.env {
        if valid_env_name(key) {
            proc.env(key, value);
        }
    }

    let timeout_secs = command
        .timeout_secs
        .unwrap_or(DEFAULT_COMMAND_TIMEOUT_SECS)
        .max(1);
    let timeout = Duration::from_secs(timeout_secs);

    let start = Instant::now();
    let mut child = proc.spawn().map_err(|e| {
        WorkspaceBootstrapError::CommandFailed(format!("failed to spawn {}: {}", command.cmd, e))
    })?;

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child.wait_with_output().map_err(|e| {
                    WorkspaceBootstrapError::CommandFailed(format!(
                        "failed to collect output for {}: {}",
                        command.cmd, e
                    ))
                })?;

                let stdout_preview = preview_output(&output.stdout);
                let stderr_preview = preview_output(&output.stderr);
                let result = WorkspaceBootstrapCommandResult {
                    command: render_command(command),
                    success: status.success(),
                    exit_code: status.code(),
                    duration_ms: start.elapsed().as_millis(),
                    stdout_preview,
                    stderr_preview: stderr_preview.clone(),
                };

                if status.success() {
                    return Ok(result);
                }

                let message = format!(
                    "{} exited with code {:?}: {}",
                    command.cmd,
                    status.code(),
                    stderr_preview.unwrap_or_else(|| "no stderr".to_string())
                );
                return Err(WorkspaceBootstrapError::CommandFailed(message));
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    let output = child.wait_with_output().ok();
                    let stderr_preview = output
                        .as_ref()
                        .and_then(|value| preview_output(&value.stderr))
                        .unwrap_or_else(|| "no stderr".to_string());
                    return Err(WorkspaceBootstrapError::CommandTimeout(format!(
                        "{} timed out after {}s: {}",
                        command.cmd, timeout_secs, stderr_preview
                    )));
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(WorkspaceBootstrapError::CommandFailed(format!(
                    "failed waiting for {}: {}",
                    command.cmd, e
                )));
            }
        }
    }
}

fn sanitize_relative_path(raw: &str) -> Result<PathBuf, WorkspaceBootstrapError> {
    let candidate = raw.trim();
    if candidate.is_empty() {
        return Err(WorkspaceBootstrapError::InvalidPath(
            "path cannot be empty".to_string(),
        ));
    }

    let path = Path::new(candidate);
    if path.is_absolute() {
        return Err(WorkspaceBootstrapError::InvalidPath(format!(
            "absolute paths are not allowed: {}",
            raw
        )));
    }

    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(value) => out.push(value),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(WorkspaceBootstrapError::InvalidPath(format!(
                    "path traversal is not allowed: {}",
                    raw
                )));
            }
        }
    }

    if out.as_os_str().is_empty() {
        return Err(WorkspaceBootstrapError::InvalidPath(format!(
            "invalid relative path: {}",
            raw
        )));
    }

    Ok(out)
}

fn validate_file_id(file_id: &str) -> Result<(), WorkspaceBootstrapError> {
    let trimmed = file_id.trim();
    if trimmed.is_empty() {
        return Err(WorkspaceBootstrapError::InvalidFileId(
            "file_id cannot be empty".to_string(),
        ));
    }

    if !trimmed
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'))
    {
        return Err(WorkspaceBootstrapError::InvalidFileId(format!(
            "file_id contains unsupported characters: {}",
            file_id
        )));
    }

    Ok(())
}

fn command_basename(command: &str) -> String {
    Path::new(command)
        .file_name()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| command.to_ascii_lowercase())
}

fn is_command_allowed(command: &str) -> bool {
    let allowed = allowed_commands();
    allowed.contains(&command.to_ascii_lowercase())
}

fn allowed_commands() -> HashSet<String> {
    if let Ok(raw) = std::env::var("WORKSPACE_BOOTSTRAP_ALLOWED_COMMANDS") {
        let parsed: HashSet<String> = raw
            .split(',')
            .map(|item| item.trim().to_ascii_lowercase())
            .filter(|item| !item.is_empty())
            .collect();
        if !parsed.is_empty() {
            return parsed;
        }
    }

    DEFAULT_ALLOWED_COMMANDS
        .iter()
        .map(|value| value.to_string())
        .collect()
}

fn valid_env_name(raw: &str) -> bool {
    !raw.trim().is_empty()
        && raw
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn preview_output(bytes: &[u8]) -> Option<String> {
    if bytes.is_empty() {
        return None;
    }

    let text = String::from_utf8_lossy(bytes).trim().to_string();
    if text.is_empty() {
        return None;
    }

    if text.chars().count() <= MAX_OUTPUT_PREVIEW_CHARS {
        return Some(text);
    }

    let preview: String = text.chars().take(MAX_OUTPUT_PREVIEW_CHARS).collect();
    Some(format!("{}...", preview))
}

fn load_state(state_path: &Path) -> Option<WorkspaceBootstrapState> {
    let raw = fs::read_to_string(state_path).ok()?;
    serde_json::from_str(&raw).ok()
}

fn write_state(state_path: &Path, state: &WorkspaceBootstrapState) -> Result<(), WorkspaceBootstrapError> {
    let encoded = serde_json::to_string_pretty(state)?;
    fs::write(state_path, encoded)?;
    Ok(())
}

fn normalized_profile_version(profile: &WorkspaceBootstrapProfile) -> String {
    let value = profile.version.trim();
    if value.is_empty() {
        "unversioned".to_string()
    } else {
        value.to_string()
    }
}

fn hash_profile(profile: &WorkspaceBootstrapProfile) -> Result<String, WorkspaceBootstrapError> {
    let payload = serde_json::to_vec(profile)?;
    let mut hasher = Sha256::new();
    hasher.update(payload);
    Ok(format!("{:x}", hasher.finalize()))
}

fn render_command(command: &WorkspaceBootstrapCommand) -> String {
    if command.args.is_empty() {
        return command.cmd.clone();
    }
    format!("{} {}", command.cmd, command.args.join(" "))
}

fn source_label(source: &WorkspaceBootstrapSource) -> &'static str {
    match source {
        WorkspaceBootstrapSource::Account { .. } => "account",
        WorkspaceBootstrapSource::Local { .. } => "local",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn sanitize_relative_path_rejects_parent_segments() {
        let err = sanitize_relative_path("../etc/passwd").expect_err("must reject");
        assert!(matches!(err, WorkspaceBootstrapError::InvalidPath(_)));
    }

    #[test]
    fn apply_workspace_bootstrap_writes_files_and_skips_when_unchanged() {
        let tmp = tempdir().expect("tempdir");
        let workspace = tmp.path().join("workspace");
        let bootstrap_root = tmp.path().join("bootstrap");
        fs::create_dir_all(bootstrap_root.join("files")).expect("bootstrap files");
        fs::create_dir_all(&workspace).expect("workspace");

        fs::write(bootstrap_root.join("files").join("script"), b"echo hello\n")
            .expect("bootstrap file");

        let profile = WorkspaceBootstrapProfile {
            version: "v1".to_string(),
            files: vec![WorkspaceBootstrapFile {
                file_id: "script".to_string(),
                path: "scripts/setup.sh".to_string(),
                required: true,
            }],
            ..WorkspaceBootstrapProfile::default()
        };

        let source = WorkspaceBootstrapSource::Local {
            bootstrap_root: bootstrap_root.clone(),
        };

        let first = apply_workspace_bootstrap(&workspace, &profile, &source).expect("first apply");
        assert!(first.applied);
        assert!(!first.skipped);
        assert_eq!(first.files_applied, 1);

        let contents = fs::read_to_string(workspace.join("scripts/setup.sh")).expect("file copied");
        assert_eq!(contents, "echo hello\n");

        let second = apply_workspace_bootstrap(&workspace, &profile, &source).expect("second apply");
        assert!(!second.applied);
        assert!(second.skipped);
    }

    #[test]
    fn apply_workspace_bootstrap_allows_non_required_command_failure() {
        let tmp = tempdir().expect("tempdir");
        let workspace = tmp.path().join("workspace");
        fs::create_dir_all(&workspace).expect("workspace");

        let profile = WorkspaceBootstrapProfile {
            version: "v2".to_string(),
            commands: vec![WorkspaceBootstrapCommand {
                cmd: "definitely_not_a_real_binary".to_string(),
                required: false,
                ..WorkspaceBootstrapCommand::default()
            }],
            ..WorkspaceBootstrapProfile::default()
        };

        let source = WorkspaceBootstrapSource::Local {
            bootstrap_root: tmp.path().join("bootstrap"),
        };

        let applied = apply_workspace_bootstrap(&workspace, &profile, &source).expect("apply");
        assert!(applied.applied);

        let state_raw = fs::read_to_string(workspace.join(STATE_DIR_NAME).join(STATE_FILE_NAME))
            .expect("state file");
        let state: WorkspaceBootstrapState = serde_json::from_str(&state_raw).expect("parse state");
        assert_eq!(state.status, "success");
        assert!(!state.warnings.is_empty());
    }
}
