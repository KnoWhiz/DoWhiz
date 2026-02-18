use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use super::errors::RunTaskError;

pub(super) fn load_env_sources(workspace_dir: &Path) -> Result<(), RunTaskError> {
    if let Some(env_path) = find_env_file(workspace_dir) {
        load_env_file(&env_path)?;
    }
    Ok(())
}

pub(super) fn find_env_file(workspace_dir: &Path) -> Option<PathBuf> {
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
