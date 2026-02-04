use scheduler_module::service::{run_server, ServiceConfig};
use scheduler_module::user_store::normalize_email;
use scheduler_module::{Scheduler, ScheduledTask, SchedulerError, TaskExecution, TaskExecutor, TaskKind};
use lettre::Transport;
use rusqlite::OptionalExtension;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tempfile::TempDir;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

#[derive(Clone, Default)]
struct NoopExecutor;

impl TaskExecutor for NoopExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Ok(TaskExecution::default())
    }
}

fn load_env_from_repo() {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let candidate = dir.join(".env");
        if candidate.exists() {
            let _ = dotenvy::from_path(candidate);
            break;
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }
}

struct HookRestore {
    token: String,
    previous_hook: String,
}

impl Drop for HookRestore {
    fn drop(&mut self) {
        let _ = postmark_request(
            "PUT",
            "https://api.postmarkapp.com/server",
            &self.token,
            Some(json!({ "InboundHookUrl": self.previous_hook })),
        );
    }
}

fn env_enabled(key: &str) -> bool {
    matches!(env::var(key).as_deref(), Ok("1"))
}

fn timestamp_suffix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn postmark_request(
    method: &str,
    url: &str,
    token: &str,
    payload: Option<Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(30))
        .build()?;
    let request = client
        .request(method.parse()?, url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("X-Postmark-Server-Token", token);
    let request = if let Some(body) = payload {
        request.json(&body)
    } else {
        request
    };
    let response = request.send()?;
    let status = response.status();
    let body = response.text()?;
    if !status.is_success() {
        return Err(format!("postmark request failed: {} {}", status, body).into());
    }
    Ok(serde_json::from_str(&body)?)
}

fn poll_outbound(
    token: &str,
    recipient: &str,
    subject_hint: &str,
    timeout: Duration,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(20))
        .build()?;
    let start = SystemTime::now();

    loop {
        let url = format!(
            "https://api.postmarkapp.com/messages/outbound?recipient={}&count=50&offset=0",
            recipient
        );
        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Postmark-Server-Token", token)
            .send();

        let response = match response {
            Ok(response) => response,
            Err(err) => {
                if start.elapsed().unwrap_or_default() >= timeout {
                    return Err(format!("postmark search timed out: {}", err).into());
                }
                std::thread::sleep(Duration::from_secs(2));
                continue;
            }
        };

        let body = response.text()?;
        let payload: Value = serde_json::from_str(&body)?;
        if let Some(messages) = payload.get("Messages").and_then(|value| value.as_array()) {
            for message in messages {
                let subject = message.get("Subject").and_then(|value| value.as_str()).unwrap_or("");
                if subject.contains(subject_hint) {
                    return Ok(Some(message.clone()));
                }
            }
        }

        if start.elapsed().unwrap_or_default() >= timeout {
            return Ok(None);
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn check_public_health(base_url: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()?;
    let health_url = format!("{}/health", base_url.trim_end_matches('/'));
    let response = client.get(&health_url).send();
    match response {
        Ok(response) if response.status().is_success() => Ok(()),
        Ok(response) => Err(format!(
            "public health check failed: {} {} (ensure ngrok forwards to http://127.0.0.1:{})",
            response.status(),
            health_url,
            port
        )
        .into()),
        Err(err) => Err(format!(
            "public health check error: {} {} (ensure ngrok forwards to http://127.0.0.1:{})",
            err, health_url, port
        )
        .into()),
    }
}

fn send_smtp_inbound(
    from_addr: &str,
    to_addr: &str,
    subject: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = lettre::Message::builder()
        .from(from_addr.parse()?)
        .to(to_addr.parse()?)
        .subject(subject)
        .body("Rust service live email test.".to_string())?;

    let mailer = lettre::SmtpTransport::builder_dangerous("inbound.postmarkapp.com")
        .port(25)
        .build();
    mailer.send(&message)?;
    Ok(())
}

fn wait_for_workspace(root: &Path, timeout: Duration) -> Option<PathBuf> {
    let start = SystemTime::now();
    loop {
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() && path.join("reply_email_draft.html").exists() {
                    return Some(path);
                }
            }
        }
        if start.elapsed().unwrap_or_default() >= timeout {
            return None;
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn wait_for_tasks_complete(
    tasks_path: &Path,
    timeout: Duration,
) -> Result<Vec<ScheduledTask>, Box<dyn std::error::Error>> {
    let start = SystemTime::now();
    loop {
        if tasks_path.exists() {
            let scheduler = Scheduler::load(tasks_path, NoopExecutor)?;
            let tasks = scheduler.tasks().to_vec();
            if !tasks.is_empty()
                && tasks
                    .iter()
                    .all(|task| !task.enabled && task.last_run.is_some())
            {
                return Ok(tasks);
            }
        }
        if start.elapsed().unwrap_or_default() >= timeout {
            return Err("timed out waiting for tasks to complete".into());
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn wait_for_user_id(users_db_path: &Path, email: &str, timeout: Duration) -> Option<String> {
    let normalized = normalize_email(email)?;
    let start = SystemTime::now();
    loop {
        if users_db_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(users_db_path) {
                if let Ok(row) = conn
                    .query_row(
                        "SELECT id FROM users WHERE email = ?1",
                        rusqlite::params![normalized.as_str()],
                        |row| row.get::<_, String>(0),
                    )
                    .optional()
                {
                    if let Some(user_id) = row {
                        return Some(user_id);
                    }
                }
            }
        }
        if start.elapsed().unwrap_or_default() >= timeout {
            return None;
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

#[test]
fn rust_service_real_email_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt().with_target(false).try_init();
    load_env_from_repo();
    if !env_enabled("RUST_SERVICE_LIVE_TEST") {
        eprintln!("Skipping Rust service live email test. Set RUST_SERVICE_LIVE_TEST=1 to run.");
        return Ok(());
    }

    let token = env::var("POSTMARK_SERVER_TOKEN")
        .map_err(|_| "POSTMARK_SERVER_TOKEN must be set for live tests")?;
    let public_url = env::var("POSTMARK_INBOUND_HOOK_URL")
        .map_err(|_| "POSTMARK_INBOUND_HOOK_URL must be set (ngrok URL)")?;
    let from_addr = env::var("POSTMARK_TEST_FROM")
        .unwrap_or_else(|_| "oliver@dowhiz.com".to_string());

    let server_info = postmark_request("GET", "https://api.postmarkapp.com/server", &token, None)?;
    let inbound_address = server_info
        .get("InboundAddress")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_string();
    if inbound_address.is_empty() {
        return Err("Postmark server does not have an inbound address configured".into());
    }
    let previous_hook = server_info
        .get("InboundHookUrl")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_string();
    let _restore = HookRestore {
        token: token.clone(),
        previous_hook: previous_hook.clone(),
    };

    let temp = TempDir::new()?;
    let workspace_root = temp.path().join("workspaces");
    let state_dir = temp.path().join("state");
    let users_root = temp.path().join("users");

    let port = env::var("RUST_SERVICE_TEST_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(9010);

    let codex_disabled = !env_enabled("RUN_CODEX_E2E");
    let config = ServiceConfig {
        host: "127.0.0.1".to_string(),
        port,
        workspace_root: workspace_root.clone(),
        scheduler_state_path: state_dir.join("tasks.db"),
        processed_ids_path: state_dir.join("postmark_processed_ids.txt"),
        users_root: users_root.clone(),
        users_db_path: state_dir.join("users.db"),
        task_index_path: state_dir.join("task_index.db"),
        codex_model: env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.2-codex".to_string()),
        codex_disabled,
        scheduler_poll_interval: Duration::from_secs(1),
    };

    let rt = Runtime::new()?;
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let server_handle = rt.spawn(async move {
        run_server(config, async {
            let _ = shutdown_rx.await;
        })
        .await
    });

    rt.block_on(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    });

    let base_url = public_url.trim_end_matches('/');
    let base_url = base_url.strip_suffix("/postmark/inbound").unwrap_or(base_url);
    check_public_health(base_url, port)?;
    let hook_url = format!("{}/postmark/inbound", base_url);
    println!("Setting Postmark inbound hook to {}", hook_url);
    postmark_request(
        "PUT",
        "https://api.postmarkapp.com/server",
        &token,
        Some(json!({ "InboundHookUrl": hook_url })),
    )?;

    let subject = format!("Rust service live test {}", timestamp_suffix());
    println!("Sending inbound SMTP message with subject: {}", subject);
    send_smtp_inbound(&from_addr, &inbound_address, &subject)?;
    println!("Inbound message sent; waiting for workspace output...");

    let workspace_timeout = if env_enabled("RUN_CODEX_E2E") {
        Duration::from_secs(600)
    } else {
        Duration::from_secs(120)
    };

    let user_id = wait_for_user_id(&state_dir.join("users.db"), &from_addr, workspace_timeout)
        .ok_or("timed out waiting for user record")?;
    let workspace_root = users_root.join(&user_id).join("workspaces");
    let workspace = wait_for_workspace(&workspace_root, workspace_timeout)
        .ok_or("timed out waiting for workspace output")?;
    let reply_path = workspace.join("reply_email_draft.html");
    if !reply_path.exists() {
        return Err("reply_email_draft.html not written by run_task".into());
    }

    let reply_subject = format!("Re: {}", subject);
    let outbound_timeout = if env_enabled("RUN_CODEX_E2E") {
        Duration::from_secs(300)
    } else {
        Duration::from_secs(120)
    };
    let outbound = poll_outbound(&token, &from_addr, &reply_subject, outbound_timeout)?
        .ok_or("timed out waiting for outbound reply")?;
    let status = outbound
        .get("Status")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    if !matches!(status, "Delivered" | "Sent") {
        return Err(format!("unexpected outbound status: {}", status).into());
    }

    let tasks_path = users_root.join(&user_id).join("state").join("tasks.db");
    let tasks_timeout = if env_enabled("RUN_CODEX_E2E") {
        Duration::from_secs(480)
    } else {
        Duration::from_secs(120)
    };
    let tasks = wait_for_tasks_complete(&tasks_path, tasks_timeout)?;
    if tasks.len() < 2 {
        return Err("expected at least two scheduled tasks".into());
    }

    let _ = shutdown_tx.send(());
    let _ = rt.block_on(async { server_handle.await })?;

    Ok(())
}
