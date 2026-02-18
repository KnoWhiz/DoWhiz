use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use super::constants::GIT_ASKPASS_SCRIPT;
use super::env::{env_enabled, env_missing_or_empty, normalize_env_prefix, read_env_trimmed};
use super::errors::RunTaskError;
use super::utils::tail_string;

static GH_AUTH_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug)]
pub(super) struct GitHubAuthConfig {
    pub(super) env_overrides: Vec<(String, String)>,
    pub(super) askpass_path: Option<PathBuf>,
    pub(super) token: Option<String>,
    #[allow(dead_code)]
    pub(super) username: Option<String>,
}

#[derive(Debug, Clone)]
struct EmployeeGithubEnv {
    username: Option<String>,
    token: Option<String>,
}

fn employee_id_default_github_prefix(employee_id: &str) -> Option<&'static str> {
    let normalized = employee_id.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "little_bear" => Some("OLIVER"),
        "mini_mouse" => Some("MAGGIE"),
        "sticky_octopus" => Some("DEVIN"),
        "boiled_egg" => Some("PROTO"),
        _ => None,
    }
}

fn resolve_employee_github_env() -> Option<EmployeeGithubEnv> {
    let explicit_prefix = read_env_trimmed("EMPLOYEE_GITHUB_ENV_PREFIX")
        .or_else(|| read_env_trimmed("GITHUB_ENV_PREFIX"));
    let prefix = explicit_prefix.or_else(|| {
        read_env_trimmed("EMPLOYEE_ID").and_then(|id| {
            employee_id_default_github_prefix(&id)
                .map(|value| value.to_string())
                .or_else(|| Some(normalize_env_prefix(&id)))
        })
    })?;

    let username = read_env_trimmed(&format!("{}_GITHUB_USERNAME", prefix));
    let token = read_env_trimmed(&format!("{}_GH_TOKEN", prefix))
        .or_else(|| read_env_trimmed(&format!("{}_GITHUB_TOKEN", prefix)))
        .or_else(|| read_env_trimmed(&format!("{}_GITHUB_PERSONAL_ACCESS_TOKEN", prefix)));

    if username.is_none() && token.is_none() {
        return None;
    }

    Some(EmployeeGithubEnv { username, token })
}

pub(super) fn resolve_github_auth(askpass_dir: Option<&Path>) -> Result<GitHubAuthConfig, RunTaskError> {
    let gh_token = read_env_trimmed("GH_TOKEN");
    let github_token = read_env_trimmed("GITHUB_TOKEN");
    let pat_token = read_env_trimmed("GITHUB_PERSONAL_ACCESS_TOKEN");
    let employee_env = resolve_employee_github_env();
    let token = gh_token
        .clone()
        .or(github_token.clone())
        .or(pat_token)
        .or_else(|| employee_env.as_ref().and_then(|env| env.token.clone()));
    let github_username = read_env_trimmed("GITHUB_USERNAME")
        .or_else(|| employee_env.as_ref().and_then(|env| env.username.clone()))
        .or_else(|| read_env_trimmed("USER"))
        .or_else(|| read_env_trimmed("USERNAME"));

    let mut env_overrides = Vec::new();
    if env_missing_or_empty("GH_PROMPT_DISABLED") {
        env_overrides.push(("GH_PROMPT_DISABLED".to_string(), "1".to_string()));
    }
    if env_missing_or_empty("GH_NO_UPDATE_NOTIFIER") {
        env_overrides.push(("GH_NO_UPDATE_NOTIFIER".to_string(), "1".to_string()));
    }
    if env_missing_or_empty("GIT_EDITOR") {
        env_overrides.push(("GIT_EDITOR".to_string(), "true".to_string()));
    }
    if env_missing_or_empty("VISUAL") {
        env_overrides.push(("VISUAL".to_string(), "true".to_string()));
    }
    if env_missing_or_empty("EDITOR") {
        env_overrides.push(("EDITOR".to_string(), "true".to_string()));
    }
    if let Some(token) = token.clone() {
        if env_missing_or_empty("GH_TOKEN") {
            env_overrides.push(("GH_TOKEN".to_string(), token.clone()));
        }
        if env_missing_or_empty("GITHUB_TOKEN") {
            env_overrides.push(("GITHUB_TOKEN".to_string(), token.clone()));
        }
    }
    if let Some(username) = github_username.clone() {
        let email = format!("{}@users.noreply.github.com", username);
        if env_missing_or_empty("GITHUB_USERNAME") {
            env_overrides.push(("GITHUB_USERNAME".to_string(), username.clone()));
        }
        if env_missing_or_empty("GIT_AUTHOR_NAME") {
            env_overrides.push(("GIT_AUTHOR_NAME".to_string(), username.clone()));
        }
        if env_missing_or_empty("GIT_COMMITTER_NAME") {
            env_overrides.push(("GIT_COMMITTER_NAME".to_string(), username.clone()));
        }
        if env_missing_or_empty("GIT_AUTHOR_EMAIL") {
            env_overrides.push(("GIT_AUTHOR_EMAIL".to_string(), email.clone()));
        }
        if env_missing_or_empty("GIT_COMMITTER_EMAIL") {
            env_overrides.push(("GIT_COMMITTER_EMAIL".to_string(), email));
        }
    }

    let askpass_path = if token.is_some() {
        let target_dir = askpass_dir.map(PathBuf::from).unwrap_or_else(env::temp_dir);
        Some(write_git_askpass_script_in(&target_dir)?)
    } else {
        None
    };

    Ok(GitHubAuthConfig {
        env_overrides,
        askpass_path,
        token,
        username: github_username,
    })
}

fn is_keyring_error(output: &str) -> bool {
    let normalized = output.to_ascii_lowercase();
    normalized.contains("keyring")
        || normalized.contains("keychain")
        || normalized.contains("credential store")
        || normalized.contains("user interaction is not allowed")
}

fn gh_auth_status_ok(github_auth: &GitHubAuthConfig) -> Result<bool, RunTaskError> {
    let mut status_cmd = Command::new("gh");
    status_cmd.args(["auth", "status", "--hostname", "github.com"]);
    apply_env_overrides(&mut status_cmd, &github_auth.env_overrides, &[]);
    match run_auth_command(status_cmd, None, "gh auth status") {
        Ok(()) => Ok(true),
        Err(RunTaskError::GitHubAuthFailed { .. }) => Ok(false),
        Err(err) => Err(err),
    }
}

fn gh_auth_login(
    github_auth: &GitHubAuthConfig,
    token: &str,
    insecure_storage: bool,
) -> Result<(), RunTaskError> {
    let mut login_cmd = Command::new("gh");
    login_cmd.args([
        "auth",
        "login",
        "--with-token",
        "--hostname",
        "github.com",
        "--git-protocol",
        "https",
    ]);
    if insecure_storage {
        login_cmd.arg("--insecure-storage");
    }
    login_cmd.env_remove("GH_TOKEN").env_remove("GITHUB_TOKEN");
    apply_env_overrides(
        &mut login_cmd,
        &github_auth.env_overrides,
        &["GH_TOKEN", "GITHUB_TOKEN"],
    );
    run_auth_command(login_cmd, Some(token), "gh auth login")
}

pub(super) fn ensure_github_cli_auth(github_auth: &GitHubAuthConfig) -> Result<(), RunTaskError> {
    if env_enabled("GH_AUTH_DISABLED") || env_enabled("GITHUB_AUTH_DISABLED") {
        return Ok(());
    }
    let Some(token) = github_auth.token.as_deref() else {
        return Ok(());
    };

    let auth_lock = GH_AUTH_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = auth_lock
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    if gh_auth_status_ok(github_auth)? {
        return Ok(());
    }

    match gh_auth_login(github_auth, token, false) {
        Ok(()) => {}
        Err(RunTaskError::GitHubAuthFailed { output, .. }) if is_keyring_error(&output) => {
            gh_auth_login(github_auth, token, true)?;
        }
        Err(err) => return Err(err),
    }

    let mut setup_cmd = Command::new("gh");
    setup_cmd.args(["auth", "setup-git", "--hostname", "github.com"]);
    apply_env_overrides(&mut setup_cmd, &github_auth.env_overrides, &[]);
    run_auth_command(setup_cmd, None, "gh auth setup-git")?;

    let mut status_cmd = Command::new("gh");
    status_cmd.args(["auth", "status", "--hostname", "github.com"]);
    apply_env_overrides(&mut status_cmd, &github_auth.env_overrides, &[]);
    run_auth_command(status_cmd, None, "gh auth status")?;

    Ok(())
}

fn apply_env_overrides(cmd: &mut Command, overrides: &[(String, String)], skip: &[&str]) {
    for (key, value) in overrides {
        if skip.iter().any(|blocked| *blocked == key.as_str()) {
            continue;
        }
        cmd.env(key, value);
    }
}

fn run_auth_command(
    mut cmd: Command,
    input: Option<&str>,
    label: &'static str,
) -> Result<(), RunTaskError> {
    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(RunTaskError::GitHubAuthCommandNotFound { command: "gh" })
        }
        Err(err) => return Err(RunTaskError::Io(err)),
    };

    if let Some(payload) = input {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(payload.as_bytes())?;
            stdin.write_all(b"\n")?;
        }
    }

    let output = child.wait_with_output()?;
    let mut combined_output = String::new();
    combined_output.push_str(&String::from_utf8_lossy(&output.stdout));
    combined_output.push_str(&String::from_utf8_lossy(&output.stderr));
    let output_tail = tail_string(&combined_output, 2000);

    if !output.status.success() {
        return Err(RunTaskError::GitHubAuthFailed {
            command: label,
            status: output.status.code(),
            output: output_tail,
        });
    }

    Ok(())
}

fn write_git_askpass_script_in(dir: &Path) -> Result<PathBuf, RunTaskError> {
    fs::create_dir_all(dir)?;
    let mut path = dir.to_path_buf();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    path.push(format!(
        "dowhiz-git-askpass-{}-{}",
        std::process::id(),
        nanos
    ));
    fs::write(&path, GIT_ASKPASS_SCRIPT)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o700);
        fs::set_permissions(&path, perms)?;
    }
    Ok(path)
}
