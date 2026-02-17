use std::env;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crate::employee_config::{load_employee_directory, EmployeeDirectory, EmployeeProfile};

use super::BoxError;

pub const DEFAULT_INBOUND_BODY_MAX_BYTES: usize = 25 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub employee_id: String,
    pub employee_config_path: PathBuf,
    pub employee_profile: EmployeeProfile,
    pub employee_directory: EmployeeDirectory,
    pub workspace_root: PathBuf,
    pub scheduler_state_path: PathBuf,
    pub processed_ids_path: PathBuf,
    pub ingestion_db_path: PathBuf,
    pub ingestion_dedupe_path: PathBuf,
    pub ingestion_poll_interval: Duration,
    pub users_root: PathBuf,
    pub users_db_path: PathBuf,
    pub task_index_path: PathBuf,
    pub codex_model: String,
    pub codex_disabled: bool,
    pub scheduler_poll_interval: Duration,
    pub scheduler_max_concurrency: usize,
    pub scheduler_user_max_concurrency: usize,
    pub inbound_body_max_bytes: usize,
    pub skills_source_dir: Option<PathBuf>,
    /// Slack bot OAuth token for sending messages (legacy single-workspace)
    pub slack_bot_token: Option<String>,
    /// Slack bot user ID for filtering out bot's own messages (legacy single-workspace)
    pub slack_bot_user_id: Option<String>,
    /// Path to slack installations database
    pub slack_store_path: PathBuf,
    /// Slack OAuth client ID (for multi-workspace support)
    pub slack_client_id: Option<String>,
    /// Slack OAuth client secret (for multi-workspace support)
    pub slack_client_secret: Option<String>,
    /// Slack OAuth redirect URI
    pub slack_redirect_uri: Option<String>,
    /// Discord bot token
    pub discord_bot_token: Option<String>,
    /// Discord bot application ID (for filtering out bot's own messages)
    pub discord_bot_user_id: Option<u64>,
    /// Google Docs polling enabled
    pub google_docs_enabled: bool,
    /// BlueBubbles server URL (e.g., http://localhost:1234)
    pub bluebubbles_url: Option<String>,
    /// BlueBubbles server password
    pub bluebubbles_password: Option<String>,
    /// Telegram bot token
    pub telegram_bot_token: Option<String>,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, BoxError> {
        dotenvy::dotenv().ok();

        let host = env::var("RUST_SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("RUST_SERVICE_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(9001);

        let employee_config_path =
            resolve_path(env::var("EMPLOYEE_CONFIG_PATH").unwrap_or_else(|_| {
                default_employee_config_path()
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let employee_directory = load_employee_directory(&employee_config_path)?;
        let employee_id = env::var("EMPLOYEE_ID")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .or_else(|| employee_directory.default_employee_id.clone())
            .or_else(|| {
                employee_directory
                    .employees
                    .first()
                    .map(|emp| emp.id.clone())
            })
            .ok_or_else(|| "employee config has no employees".to_string())?;
        let employee_profile = employee_directory
            .employee(&employee_id)
            .ok_or_else(|| {
                format!(
                    "employee '{}' not found in {}",
                    employee_id,
                    employee_config_path.display()
                )
            })?
            .clone();

        let runtime_root = default_runtime_root()?;
        let employee_runtime_root = employee_profile
            .runtime_root
            .clone()
            .unwrap_or_else(|| runtime_root.join(&employee_id));
        let workspace_root = resolve_path(env::var("WORKSPACE_ROOT").unwrap_or_else(|_| {
            employee_runtime_root
                .join("workspaces")
                .to_string_lossy()
                .into_owned()
        }))?;
        let scheduler_state_path =
            resolve_path(env::var("SCHEDULER_STATE_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("tasks.db")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let processed_ids_path =
            resolve_path(env::var("PROCESSED_IDS_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("postmark_processed_ids.txt")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let ingestion_db_path =
            resolve_path(env::var("INGESTION_DB_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("ingestion.db")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let ingestion_dedupe_path =
            resolve_path(env::var("INGESTION_DEDUPE_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("ingestion_processed_ids.txt")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let users_root = resolve_path(env::var("USERS_ROOT").unwrap_or_else(|_| {
            employee_runtime_root
                .join("users")
                .to_string_lossy()
                .into_owned()
        }))?;
        let users_db_path = resolve_path(env::var("USERS_DB_PATH").unwrap_or_else(|_| {
            employee_runtime_root
                .join("state")
                .join("users.db")
                .to_string_lossy()
                .into_owned()
        }))?;
        let task_index_path = resolve_path(env::var("TASK_INDEX_PATH").unwrap_or_else(|_| {
            employee_runtime_root
                .join("state")
                .join("task_index.db")
                .to_string_lossy()
                .into_owned()
        }))?;
        let codex_model = env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.2-codex".to_string());
        let codex_disabled = env_flag("CODEX_DISABLED", false);
        let scheduler_poll_interval = env::var("SCHEDULER_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(1));
        let scheduler_max_concurrency = env::var("SCHEDULER_MAX_CONCURRENCY")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(10);
        let scheduler_user_max_concurrency = env::var("SCHEDULER_USER_MAX_CONCURRENCY")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(3);
        let ingestion_poll_interval = env::var("INGESTION_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(1));
        let inbound_body_max_bytes = env::var("POSTMARK_INBOUND_MAX_BYTES")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_INBOUND_BODY_MAX_BYTES);
        let skills_source_dir = Some(repo_skills_source_dir());

        // Slack configuration
        let slack_bot_token = env::var("SLACK_BOT_TOKEN").ok().filter(|s| !s.is_empty());
        let slack_bot_user_id = env::var("SLACK_BOT_USER_ID").ok().filter(|s| !s.is_empty());
        let slack_store_path = resolve_path(env::var("SLACK_STORE_PATH").unwrap_or_else(|_| {
            employee_runtime_root
                .join("state")
                .join("slack.db")
                .to_string_lossy()
                .into_owned()
        }))?;
        let slack_client_id = env::var("SLACK_CLIENT_ID").ok().filter(|s| !s.is_empty());
        let slack_client_secret = env::var("SLACK_CLIENT_SECRET")
            .ok()
            .filter(|s| !s.is_empty());
        let slack_redirect_uri = env::var("SLACK_REDIRECT_URI")
            .ok()
            .filter(|s| !s.is_empty());

        // Discord configuration
        let discord_bot_token = env::var("DISCORD_BOT_TOKEN").ok().filter(|s| !s.is_empty());
        let discord_bot_user_id = env::var("DISCORD_BOT_USER_ID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok());

        // Google Docs configuration
        let google_docs_enabled = env::var("GOOGLE_DOCS_ENABLED")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        // BlueBubbles configuration
        let bluebubbles_url = env::var("BLUEBUBBLES_URL").ok().filter(|s| !s.is_empty());
        let bluebubbles_password = env::var("BLUEBUBBLES_PASSWORD")
            .ok()
            .filter(|s| !s.is_empty());

        // Telegram configuration
        let telegram_bot_token = resolve_telegram_bot_token(&employee_profile);

        Ok(Self {
            host,
            port,
            employee_id,
            employee_config_path,
            employee_profile,
            employee_directory,
            workspace_root,
            scheduler_state_path,
            processed_ids_path,
            ingestion_db_path,
            ingestion_dedupe_path,
            ingestion_poll_interval,
            users_root,
            users_db_path,
            task_index_path,
            codex_model,
            codex_disabled,
            scheduler_poll_interval,
            scheduler_max_concurrency,
            scheduler_user_max_concurrency,
            inbound_body_max_bytes,
            skills_source_dir,
            slack_bot_token,
            slack_bot_user_id,
            slack_store_path,
            slack_client_id,
            slack_client_secret,
            slack_redirect_uri,
            discord_bot_token,
            discord_bot_user_id,
            google_docs_enabled,
            bluebubbles_url,
            bluebubbles_password,
            telegram_bot_token,
        })
    }
}

fn env_flag(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(
            value.trim().to_lowercase().as_str(),
            "1" | "true" | "yes" | "y"
        ),
        Err(_) => default,
    }
}

fn env_var_non_empty(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize_env_key_fragment(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut output = String::with_capacity(trimmed.len());
    let mut last_was_underscore = false;
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_uppercase());
            last_was_underscore = false;
        } else if !last_was_underscore {
            output.push('_');
            last_was_underscore = true;
        }
    }
    let normalized = output.trim_matches('_').to_string();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

pub(crate) fn resolve_telegram_bot_token(employee: &EmployeeProfile) -> Option<String> {
    let mut candidates: Vec<String> = Vec::new();

    let mut push_candidate = |value: &str| {
        if let Some(candidate) = normalize_env_key_fragment(value) {
            if !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    };

    if let Some(display_name) = employee.display_name.as_deref() {
        push_candidate(display_name);
    }

    for address in &employee.addresses {
        if let Some((local, _domain)) = address.split_once('@') {
            push_candidate(local);
        } else {
            push_candidate(address);
        }
    }

    push_candidate(&employee.id);

    for candidate in candidates {
        let env_key = format!("DO_WHIZ_{candidate}_BOT");
        if let Some(token) = env_var_non_empty(&env_key) {
            return Some(token);
        }
    }

    env_var_non_empty("TELEGRAM_BOT_TOKEN")
}

fn repo_skills_source_dir() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd
        .file_name()
        .map(|name| name == "DoWhiz_service")
        .unwrap_or(false)
    {
        cwd.join("skills")
    } else {
        cwd.join("DoWhiz_service").join("skills")
    }
}

pub(crate) fn default_employee_config_path() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd
        .file_name()
        .map(|name| name == "DoWhiz_service")
        .unwrap_or(false)
    {
        cwd.join("employee.toml")
    } else {
        cwd.join("DoWhiz_service").join("employee.toml")
    }
}

fn default_runtime_root() -> Result<PathBuf, io::Error> {
    let home =
        env::var("HOME").map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME not set"))?;
    Ok(PathBuf::from(home)
        .join(".dowhiz")
        .join("DoWhiz")
        .join("run_task"))
}

fn resolve_path(raw: String) -> Result<PathBuf, io::Error> {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Ok(path)
    } else {
        let cwd = env::current_dir()?;
        Ok(cwd.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::env;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    struct EnvGuard {
        key: String,
        previous: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &str, value: &str) -> Self {
            let previous = env::var(key).ok();
            env::set_var(key, value);
            Self {
                key: key.to_string(),
                previous,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(value) => env::set_var(&self.key, value),
                None => env::remove_var(&self.key),
            }
        }
    }

    fn test_employee_profile(id: &str, display_name: Option<&str>, addresses: Vec<&str>) -> EmployeeProfile {
        let addresses: Vec<String> = addresses.into_iter().map(|value| value.to_string()).collect();
        let address_set: HashSet<String> = addresses
            .iter()
            .map(|value| value.to_ascii_lowercase())
            .collect();
        EmployeeProfile {
            id: id.to_string(),
            display_name: display_name.map(|value| value.to_string()),
            runner: "codex".to_string(),
            model: None,
            addresses,
            address_set,
            runtime_root: None,
            agents_path: None,
            claude_path: None,
            soul_path: None,
            skills_dir: None,
            discord_enabled: false,
            slack_enabled: false,
            bluebubbles_enabled: false,
        }
    }

    #[test]
    fn resolve_telegram_bot_token_prefers_employee_specific_env() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard_employee = EnvGuard::set("DO_WHIZ_OLIVER_BOT", "employee-token");
        let _guard_fallback = EnvGuard::set("TELEGRAM_BOT_TOKEN", "fallback-token");

        let employee = test_employee_profile(
            "little_bear",
            Some("Oliver"),
            vec!["oliver@dowhiz.com", "little-bear@dowhiz.com"],
        );

        let token = resolve_telegram_bot_token(&employee);
        assert_eq!(token.as_deref(), Some("employee-token"));
    }

    #[test]
    fn resolve_telegram_bot_token_falls_back_to_address_then_global() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard_employee = EnvGuard::set("DO_WHIZ_DEVIN_BOT", "devin-token");
        let _guard_fallback = EnvGuard::set("TELEGRAM_BOT_TOKEN", "fallback-token");

        let employee = test_employee_profile(
            "sticky_octopus",
            Some("Sticky-Octopus"),
            vec!["devin@dowhiz.com", "sticky-octopus@dowhiz.com"],
        );

        let token = resolve_telegram_bot_token(&employee);
        assert_eq!(token.as_deref(), Some("devin-token"));
    }

    #[test]
    fn resolve_telegram_bot_token_uses_global_when_employee_missing() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let _guard_fallback = EnvGuard::set("TELEGRAM_BOT_TOKEN", "fallback-token");

        let employee = test_employee_profile("mini_mouse", Some("Maggie"), vec!["maggie@dowhiz.com"]);

        let token = resolve_telegram_bot_token(&employee);
        assert_eq!(token.as_deref(), Some("fallback-token"));
    }
}
