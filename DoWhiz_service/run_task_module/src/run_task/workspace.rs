use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use super::constants::DOCKER_WORKSPACE_DIR;
use super::env::env_enabled;
use super::errors::RunTaskError;
use super::types::RunTaskRequest;

pub(super) fn remap_workspace_dir(workspace_dir: &Path) -> Result<PathBuf, RunTaskError> {
    if env_enabled("RUN_TASK_SKIP_WORKSPACE_REMAP") {
        return Ok(workspace_dir.to_path_buf());
    }
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
        if let Err(err) = fs::rename(workspace_dir, &remapped) {
            if err.raw_os_error() == Some(18) {
                return Ok(workspace_dir.to_path_buf());
            }
            return Err(RunTaskError::Io(err));
        }
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

pub(super) fn prepare_workspace(
    request: &RunTaskRequest<'_>,
) -> Result<(PathBuf, PathBuf), RunTaskError> {
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

    // Use channel-specific reply file and attachments directory
    // Non-email channels use plain text reply_message.txt
    // Email and GoogleDocs use HTML reply_email_draft.html
    let (reply_path, reply_attachments_dir) = match request.channel.to_lowercase().as_str() {
        "slack" | "discord" | "telegram" | "sms" | "bluebubbles" => (
            request.workspace_dir.join("reply_message.txt"),
            request.workspace_dir.join("reply_attachments"),
        ),
        _ => (
            request.workspace_dir.join("reply_email_draft.html"),
            request.workspace_dir.join("reply_email_attachments"),
        ),
    };
    ensure_dir_exists(&reply_attachments_dir, "reply_attachments_dir")?;

    Ok((reply_path, reply_attachments_dir))
}

pub(super) fn write_placeholder_reply(path: &Path) -> Result<(), RunTaskError> {
    let placeholder = "<html><body><p>Codex disabled. Received your email.</p></body></html>";
    fs::write(path, placeholder)?;
    Ok(())
}

pub(super) fn ensure_workspace_dir(path: &Path) -> Result<(), RunTaskError> {
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

pub(super) fn ensure_dir_exists(path: &Path, label: &'static str) -> Result<(), RunTaskError> {
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

pub(super) fn canonicalize_dir(path: &Path) -> Result<PathBuf, RunTaskError> {
    fs::canonicalize(path).map_err(RunTaskError::Io)
}

pub(super) fn workspace_path_in_container(
    path: &Path,
    host_workspace_dir: &Path,
) -> Option<PathBuf> {
    let relative = path.strip_prefix(host_workspace_dir).ok()?;
    Some(Path::new(DOCKER_WORKSPACE_DIR).join(relative))
}

pub(super) fn resolve_rel_dir(
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
