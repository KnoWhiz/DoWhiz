use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::RunTaskTask;

const USER_SECRETS_DIR: &str = "secrets";
const USER_ENV_FILENAME: &str = ".env";

pub(crate) fn resolve_user_secrets_path(task: &RunTaskTask) -> Option<PathBuf> {
    if let Some(archive_root) = task.archive_root.as_ref() {
        if let Some(user_root) = archive_root.parent() {
            return Some(user_root.join(USER_SECRETS_DIR).join(USER_ENV_FILENAME));
        }
    }
    let user_root = task.workspace_dir.parent()?.parent()?;
    Some(user_root.join(USER_SECRETS_DIR).join(USER_ENV_FILENAME))
}

pub(crate) fn sync_user_secrets_to_workspace(
    user_env_path: &Path,
    workspace_dir: &Path,
) -> Result<(), io::Error> {
    let workspace_env = workspace_env_path(workspace_dir);
    if user_env_path.exists() {
        fs::copy(user_env_path, workspace_env)?;
        return Ok(());
    }
    if workspace_env.exists() {
        return Ok(());
    }
    if let Some(parent) = workspace_env.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(workspace_env, "")?;
    Ok(())
}

pub(crate) fn sync_workspace_secrets_to_user(
    workspace_dir: &Path,
    user_env_path: &Path,
) -> Result<(), io::Error> {
    let workspace_env = workspace_env_path(workspace_dir);
    if !workspace_env.exists() {
        return Ok(());
    }
    if let Some(parent) = user_env_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(workspace_env, user_env_path)?;
    Ok(())
}

fn workspace_env_path(workspace_dir: &Path) -> PathBuf {
    workspace_dir.join(USER_ENV_FILENAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn base_task(workspace_dir: PathBuf, archive_root: Option<PathBuf>) -> RunTaskTask {
        RunTaskTask {
            workspace_dir,
            input_email_dir: PathBuf::from("incoming_email"),
            input_attachments_dir: PathBuf::from("incoming_attachments"),
            memory_dir: PathBuf::from("memory"),
            reference_dir: PathBuf::from("references"),
            model_name: "test-model".to_string(),
            runner: "codex".to_string(),
            codex_disabled: true,
            reply_to: Vec::new(),
            reply_from: None,
            archive_root,
            thread_id: None,
            thread_epoch: None,
            thread_state_path: None,
            channel: crate::channel::Channel::default(),
        }
    }

    #[test]
    fn resolve_user_secrets_path_prefers_archive_root() {
        let temp = TempDir::new().expect("tempdir");
        let user_root = temp.path().join("users").join("user_1");
        let archive_root = user_root.join("mail");
        let workspace_dir = user_root.join("workspaces").join("thread_1");
        let task = base_task(workspace_dir, Some(archive_root));
        let expected = user_root.join("secrets").join(".env");
        let resolved = resolve_user_secrets_path(&task).expect("secrets path");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn sync_user_secrets_to_workspace_copies_env() {
        let temp = TempDir::new().expect("tempdir");
        let workspace_dir = temp.path().join("workspace");
        fs::create_dir_all(&workspace_dir).expect("workspace");
        let user_env = temp.path().join("secrets").join(".env");
        fs::create_dir_all(user_env.parent().unwrap()).expect("secrets dir");
        fs::write(&user_env, "TOKEN=origin").expect("user env");

        sync_user_secrets_to_workspace(&user_env, &workspace_dir).expect("sync");

        let workspace_env = workspace_dir.join(".env");
        let contents = fs::read_to_string(workspace_env).expect("workspace env");
        assert_eq!(contents, "TOKEN=origin");
    }

    #[test]
    fn sync_user_secrets_to_workspace_creates_empty_env_when_missing() {
        let temp = TempDir::new().expect("tempdir");
        let workspace_dir = temp.path().join("workspace");
        fs::create_dir_all(&workspace_dir).expect("workspace");
        let user_env = temp.path().join("secrets").join(".env");

        sync_user_secrets_to_workspace(&user_env, &workspace_dir).expect("sync");

        let workspace_env = workspace_dir.join(".env");
        let contents = fs::read_to_string(workspace_env).expect("workspace env");
        assert!(contents.is_empty());
    }
}
