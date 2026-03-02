use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use super::errors::RunTaskError;

pub(super) fn load_env_sources(workspace_dir: &Path) -> Result<(), RunTaskError> {
    for env_path in find_env_files(workspace_dir) {
        load_env_file(&env_path)?;
    }
    Ok(())
}

pub(super) fn find_env_files(workspace_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    collect_env_files(workspace_dir, &mut candidates);
    if let Ok(cwd) = env::current_dir() {
        collect_env_files(&cwd, &mut candidates);
    }
    dedupe_paths(candidates)
}

fn collect_env_files(start: &Path, out: &mut Vec<PathBuf>) {
    for ancestor in start.ancestors() {
        let candidate = ancestor.join(".env");
        if candidate.exists() {
            out.push(candidate);
        }
    }
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();
    for path in paths {
        if !deduped.iter().any(|existing: &PathBuf| existing == &path) {
            deduped.push(path);
        }
    }
    deduped
}

pub(super) fn load_env_file(path: &Path) -> Result<(), RunTaskError> {
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

pub(super) fn unquote_env_value(value: &str) -> &str {
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

pub(super) fn normalize_env_prefix(raw: &str) -> String {
    raw.trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

pub(super) fn read_env_trimmed(key: &str) -> Option<String> {
    let value = env::var(key).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(unquote_env_value(trimmed).to_string())
    }
}

pub(super) fn read_env_list(key: &str) -> Vec<String> {
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

pub(super) fn env_missing_or_empty(key: &str) -> bool {
    match env::var(key) {
        Ok(value) => value.trim().is_empty(),
        Err(_) => true,
    }
}

pub(super) fn env_enabled(key: &str) -> bool {
    match env::var(key) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !(normalized.is_empty() || normalized == "0" || normalized == "false")
        }
        Err(_) => false,
    }
}

pub(super) fn env_enabled_default(key: &str, default_value: bool) -> bool {
    match env::var(key) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !(normalized.is_empty() || normalized == "0" || normalized == "false")
        }
        Err(_) => default_value,
    }
}

pub(super) fn resolve_env_path(key: &str, cwd: &Path) -> Option<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvVarGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl EnvVarGuard {
        fn unset(key: &'static str) -> Self {
            let prev = env::var(key).ok();
            env::remove_var(key);
            Self { key, prev }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.prev {
                env::set_var(self.key, value);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    struct CwdGuard {
        prev: PathBuf,
    }

    impl CwdGuard {
        fn set(path: &Path) -> Self {
            let prev = env::current_dir().expect("current dir");
            env::set_current_dir(path).expect("set current dir");
            Self { prev }
        }
    }

    impl Drop for CwdGuard {
        fn drop(&mut self) {
            let _ = env::set_current_dir(&self.prev);
        }
    }

    #[test]
    fn load_env_sources_merges_workspace_and_cwd_env() {
        let _lock = ENV_LOCK.lock().expect("env lock");
        let _backend_guard = EnvVarGuard::unset("RUN_TASK_EXECUTION_BACKEND");
        let _group_guard = EnvVarGuard::unset("RUN_TASK_AZURE_ACI_RESOURCE_GROUP");

        let temp = TempDir::new().expect("tempdir");
        let service_root = temp.path().join("service");
        let workspace = temp.path().join("run_task/users/u1/workspaces/thread_1");
        fs::create_dir_all(&service_root).expect("service root");
        fs::create_dir_all(&workspace).expect("workspace");

        fs::write(
            service_root.join(".env"),
            "RUN_TASK_AZURE_ACI_RESOURCE_GROUP=rg-from-service\nRUN_TASK_EXECUTION_BACKEND=azure_aci\n",
        )
        .expect("write service env");
        fs::write(workspace.join(".env"), "RUN_TASK_EXECUTION_BACKEND=local\n")
            .expect("write workspace env");

        let _cwd_guard = CwdGuard::set(&service_root);
        load_env_sources(&workspace).expect("load env sources");

        assert_eq!(
            env::var("RUN_TASK_EXECUTION_BACKEND").ok().as_deref(),
            Some("local")
        );
        assert_eq!(
            env::var("RUN_TASK_AZURE_ACI_RESOURCE_GROUP")
                .ok()
                .as_deref(),
            Some("rg-from-service")
        );
    }
}
