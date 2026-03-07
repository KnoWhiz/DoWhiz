use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use tracing::{info, warn};

const DEFAULT_TIMEOUT_SECS: u64 = 90;
const DEFAULT_SCRIPT_REL_PATH: &str = "DoWhiz_service/scripts/bootstrap_web_auth.py";

pub(crate) fn bootstrap_workspace_web_auth(workspace_dir: &Path) {
    if !env_flag_enabled("WEB_AUTH_BOOTSTRAP_ENABLED", true) {
        return;
    }

    let Some(script_path) = resolve_bootstrap_script_path() else {
        warn!(
            "web auth bootstrap script not found (checked WEB_AUTH_BOOTSTRAP_SCRIPT and default paths)"
        );
        return;
    };

    let timeout_secs = env::var("WEB_AUTH_BOOTSTRAP_TIMEOUT_SECS")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_TIMEOUT_SECS);

    let output = Command::new("python3")
        .arg(&script_path)
        .arg("--workspace")
        .arg(workspace_dir)
        .arg("--timeout-secs")
        .arg(timeout_secs.to_string())
        .current_dir(workspace_dir)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout_tail = tail_utf8(&out.stdout, 400);
            if !stdout_tail.trim().is_empty() {
                info!("web auth bootstrap completed: {}", stdout_tail.trim());
            }
        }
        Ok(out) => {
            let stderr_tail = tail_utf8(&out.stderr, 1000);
            let stdout_tail = tail_utf8(&out.stdout, 1000);
            warn!(
                "web auth bootstrap failed status={:?} stdout_tail={} stderr_tail={}",
                out.status.code(),
                stdout_tail.trim(),
                stderr_tail.trim()
            );
        }
        Err(err) => {
            warn!("web auth bootstrap command failed to start: {}", err);
        }
    }
}

fn env_flag_enabled(key: &str, default_value: bool) -> bool {
    match env::var(key) {
        Ok(value) => parse_env_bool(&value).unwrap_or(default_value),
        Err(_) => default_value,
    }
}

fn parse_env_bool(raw: &str) -> Option<bool> {
    let normalized = raw.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Some(false);
    }
    match normalized.as_str() {
        "0" | "false" | "no" | "off" => Some(false),
        "1" | "true" | "yes" | "on" => Some(true),
        _ => None,
    }
}

fn resolve_bootstrap_script_path() -> Option<PathBuf> {
    if let Ok(raw) = env::var("WEB_AUTH_BOOTSTRAP_SCRIPT") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            let configured = PathBuf::from(trimmed);
            let path = if configured.is_absolute() {
                configured
            } else if let Ok(cwd) = env::current_dir() {
                cwd.join(configured)
            } else {
                configured
            };
            if path.exists() {
                return Some(path);
            }
        }
    }

    if let Ok(cwd) = env::current_dir() {
        let candidate = cwd.join(DEFAULT_SCRIPT_REL_PATH);
        if candidate.exists() {
            return Some(candidate);
        }
    }

    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let candidate = exe_dir.join(DEFAULT_SCRIPT_REL_PATH);
            if candidate.exists() {
                return Some(candidate);
            }
            if let Some(parent) = exe_dir.parent() {
                let candidate = parent.join(DEFAULT_SCRIPT_REL_PATH);
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }
    }

    let fallback = Path::new("/app").join(DEFAULT_SCRIPT_REL_PATH);
    if fallback.exists() {
        return Some(fallback);
    }
    None
}

fn tail_utf8(bytes: &[u8], max_chars: usize) -> String {
    let text = String::from_utf8_lossy(bytes);
    let count = text.chars().count();
    if count <= max_chars {
        return text.into_owned();
    }
    text.chars()
        .skip(count.saturating_sub(max_chars))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn parse_env_bool_understands_common_values() {
        assert_eq!(parse_env_bool("1"), Some(true));
        assert_eq!(parse_env_bool("true"), Some(true));
        assert_eq!(parse_env_bool("YES"), Some(true));
        assert_eq!(parse_env_bool("on"), Some(true));
        assert_eq!(parse_env_bool("0"), Some(false));
        assert_eq!(parse_env_bool("false"), Some(false));
        assert_eq!(parse_env_bool("NO"), Some(false));
        assert_eq!(parse_env_bool("off"), Some(false));
        assert_eq!(parse_env_bool(""), Some(false));
        assert_eq!(parse_env_bool("maybe"), None);
    }

    #[test]
    fn tail_utf8_returns_suffix() {
        let out = tail_utf8("0123456789".as_bytes(), 4);
        assert_eq!(out, "6789");
    }

    #[test]
    fn resolve_bootstrap_script_path_prefers_explicit_env() {
        let temp = TempDir::new().expect("tempdir");
        let script = temp.path().join("bootstrap.py");
        fs::write(&script, "print('ok')").expect("write script");

        let prev = env::var("WEB_AUTH_BOOTSTRAP_SCRIPT").ok();
        env::set_var("WEB_AUTH_BOOTSTRAP_SCRIPT", &script);
        let resolved = resolve_bootstrap_script_path();
        match prev {
            Some(value) => env::set_var("WEB_AUTH_BOOTSTRAP_SCRIPT", value),
            None => env::remove_var("WEB_AUTH_BOOTSTRAP_SCRIPT"),
        }

        assert_eq!(resolved.as_deref(), Some(script.as_path()));
    }
}
