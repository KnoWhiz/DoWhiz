use lettre::message::header::{ContentType, HeaderName, HeaderValue};
use lettre::message::{Attachment as LettreAttachment, MultiPart, SinglePart};
use lettre::Transport;
use scheduler_module::employee_config::{
    load_employee_directory, EmployeeDirectory, EmployeeProfile,
};
use scheduler_module::service::{run_server, ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES};
use scheduler_module::user_store::normalize_email;
use scheduler_module::{
    ScheduledTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind,
};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tempfile::TempDir;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

type BoxError = Box<dyn std::error::Error + Send + Sync>;
const DEFAULT_NGROK_HOOK_URL: &str =
    "https://shayne-laminar-lillian.ngrok-free.dev/postmark/inbound";
const E2E_ATTACHMENT_NAME: &str = "e2e_note.txt";
const E2E_ATTACHMENT_CONTENT: &[u8] = b"dowhiz-e2e-attachment";

#[derive(Clone, Default)]
struct NoopExecutor;

impl TaskExecutor for NoopExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Ok(TaskExecution::default())
    }
}

fn resolve_employee_config_path() -> PathBuf {
    if let Ok(raw_path) = env::var("EMPLOYEE_CONFIG_PATH") {
        let path = PathBuf::from(raw_path);
        if path.is_absolute() {
            return path;
        }
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let local = manifest_dir.join(&path);
        if local.exists() {
            return local;
        }
        if let Some(parent) = manifest_dir.parent() {
            let parent_path = parent.join(&path);
            if parent_path.exists() {
                return parent_path;
            }
        }
        return path;
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let local = manifest_dir.join("employee.toml");
    if local.exists() {
        return local;
    }
    manifest_dir
        .parent()
        .unwrap_or(&manifest_dir)
        .join("employee.toml")
}

fn load_employee_for_address(
    service_address: &str,
) -> Result<(EmployeeProfile, EmployeeDirectory, PathBuf), BoxError> {
    let config_path = resolve_employee_config_path();
    let directory = load_employee_directory(&config_path)?;
    let normalized = service_address.trim();

    let employee = directory
        .employees
        .iter()
        .find(|emp| emp.matches_address(normalized))
        .cloned()
        .or_else(|| {
            directory
                .default_employee_id
                .as_ref()
                .and_then(|id| directory.employee(id))
                .cloned()
        })
        .ok_or("no employee matches service address and no default employee")?;

    Ok((employee, directory, config_path))
}

fn attach_inbound_alias(
    employee_profile: &mut EmployeeProfile,
    employee_directory: &mut EmployeeDirectory,
    inbound_address: &str,
) {
    let Some(alias) = normalize_email(inbound_address) else {
        return;
    };
    if alias.is_empty() {
        return;
    }

    if employee_profile.address_set.insert(alias.clone()) {
        employee_profile.addresses.push(alias.clone());
    }

    if let Some(profile) = employee_directory
        .employee_by_id
        .get_mut(&employee_profile.id)
    {
        if profile.address_set.insert(alias.clone()) {
            profile.addresses.push(alias.clone());
        }
    }

    if let Some(profile) = employee_directory
        .employees
        .iter_mut()
        .find(|profile| profile.id == employee_profile.id)
    {
        if profile.address_set.insert(alias.clone()) {
            profile.addresses.push(alias.clone());
        }
    }

    employee_directory.service_addresses.insert(alias);
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

fn resolve_postmark_hook_url() -> String {
    if let Ok(value) = env::var("POSTMARK_TEST_HOOK_URL") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if let Ok(value) = env::var("POSTMARK_INBOUND_HOOK_URL") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    DEFAULT_NGROK_HOOK_URL.to_string()
}

fn env_with_scale_oliver(key: &str) -> Option<String> {
    let prefixed = format!("SCALE_OLIVER_{key}");
    env::var(prefixed)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            env::var(key)
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
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

struct ChildGuard {
    child: Child,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[derive(Clone)]
struct LogCapture {
    buffer: Arc<Mutex<Vec<u8>>>,
}

struct LogWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for LogCapture {
    type Writer = LogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        LogWriter {
            buffer: Arc::clone(&self.buffer),
        }
    }
}

impl Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut guard = self
            .buffer
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        guard.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn write_gateway_config(
    path: &Path,
    host: &str,
    port: u16,
    service_address: &str,
    employee_id: &str,
) -> Result<(), BoxError> {
    let contents = format!(
        r#"[server]
host = "{host}"
port = {port}

[defaults]
tenant_id = "default"
employee_id = "{employee_id}"

[[routes]]
channel = "email"
key = "{service_address}"
employee_id = "{employee_id}"
tenant_id = "default"
"#,
        host = host,
        port = port,
        service_address = service_address,
        employee_id = employee_id,
    );
    fs::write(path, contents)?;
    Ok(())
}

fn write_employee_config_with_inbound_alias(
    source_path: &Path,
    target_path: &Path,
    employee_id: &str,
    inbound_address: &str,
) -> Result<(), BoxError> {
    let Some(alias) = normalize_email(inbound_address) else {
        return Err(format!("invalid inbound alias address: {}", inbound_address).into());
    };

    let content = fs::read_to_string(source_path)?;
    let mut parsed: toml::Value = toml::from_str(&content)?;
    let employees = parsed
        .get_mut("employees")
        .and_then(|value| value.as_array_mut())
        .ok_or("employee config missing [employees] array")?;

    let mut updated = false;
    for employee in employees {
        let id = employee
            .get("id")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        if id != employee_id {
            continue;
        }
        let Some(addresses) = employee
            .get_mut("addresses")
            .and_then(|value| value.as_array_mut())
        else {
            return Err(format!("employee '{}' missing addresses", employee_id).into());
        };
        let exists = addresses.iter().any(|value| {
            value
                .as_str()
                .map(|existing| existing.eq_ignore_ascii_case(&alias))
                .unwrap_or(false)
        });
        if !exists {
            addresses.push(toml::Value::String(alias.clone()));
        }
        updated = true;
        break;
    }

    if !updated {
        return Err(format!(
            "employee '{}' not found in {}",
            employee_id,
            source_path.display()
        )
        .into());
    }

    let rendered = toml::to_string_pretty(&parsed)?;
    fs::write(target_path, rendered)?;
    Ok(())
}

fn spawn_gateway(
    gateway_config_path: &Path,
    employee_config_path: &Path,
    host: &str,
    port: u16,
) -> Result<ChildGuard, BoxError> {
    let gateway_bin = env::var("CARGO_BIN_EXE_inbound_gateway")
        .ok()
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .unwrap_or_else(|| {
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            manifest_dir
                .parent()
                .unwrap_or(&manifest_dir)
                .join("target")
                .join("debug")
                .join("inbound_gateway")
        });
    if !gateway_bin.exists() {
        return Err(format!(
            "inbound_gateway binary not found at {}",
            gateway_bin.display()
        )
        .into());
    }

    let child = Command::new(gateway_bin)
        .env("GATEWAY_CONFIG_PATH", gateway_config_path)
        .env("SCALE_OLIVER_INGESTION_QUEUE_BACKEND", "servicebus")
        .env("SCALE_OLIVER_RAW_PAYLOAD_STORAGE_BACKEND", "azure")
        .env("EMPLOYEE_CONFIG_PATH", employee_config_path)
        .env("GATEWAY_HOST", host)
        .env("GATEWAY_PORT", port.to_string())
        .env("GOOGLE_DOCS_ENABLED", "false")
        .env("DISCORD_BOT_TOKEN", "")
        .env("DISCORD_BOT_USER_ID", "")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    Ok(ChildGuard { child })
}

fn wait_for_local_health(host: &str, port: u16, timeout: Duration) -> Result<(), BoxError> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(5))
        .build()?;
    let start = SystemTime::now();
    let url = format!("http://{}:{}/health", host, port);
    loop {
        match client.get(&url).send() {
            Ok(response) if response.status().is_success() => return Ok(()),
            Ok(_) | Err(_) => {
                if start.elapsed().unwrap_or_default() >= timeout {
                    return Err(format!("gateway health check timed out: {}", url).into());
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        }
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
) -> Result<Value, BoxError> {
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
) -> Result<Option<Value>, BoxError> {
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
                let subject = message
                    .get("Subject")
                    .and_then(|value| value.as_str())
                    .unwrap_or("");
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

fn check_public_health(base_url: &str, local_host: &str, port: u16) -> Result<(), BoxError> {
    let client = reqwest::blocking::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(10))
        .build()?;
    let health_url = format!("{}/health", base_url.trim_end_matches('/'));
    let response = client.get(&health_url).send();
    match response {
        Ok(response) if response.status().is_success() => Ok(()),
        Ok(response) => Err(format!(
            "public health check failed: {} {} (ensure ngrok forwards to http://{}:{})",
            response.status(),
            health_url,
            local_host,
            port
        )
        .into()),
        Err(err) => Err(format!(
            "public health check error: {} {} (ensure ngrok forwards to http://{}:{})",
            err, health_url, local_host, port
        )
        .into()),
    }
}

fn send_smtp_inbound(
    from_addr: &str,
    to_addr: &str,
    subject: &str,
    original_to: Option<&str>,
) -> Result<(), BoxError> {
    let mut builder = lettre::Message::builder()
        .from(from_addr.parse()?)
        .to(to_addr.parse()?)
        .subject(subject);
    if let Some(original_to) = original_to {
        builder = builder.raw_header(HeaderValue::new(
            HeaderName::new_from_ascii_str("X-Original-To"),
            original_to.to_string(),
        ));
    }
    let message = builder.multipart(
        MultiPart::mixed()
            .singlepart(SinglePart::plain(
                "Rust service live email test.".to_string(),
            ))
            .singlepart(LettreAttachment::new(E2E_ATTACHMENT_NAME.to_string()).body(
                E2E_ATTACHMENT_CONTENT.to_vec(),
                ContentType::parse("text/plain; charset=utf-8")?,
            )),
    )?;

    let smtp_host =
        env::var("POSTMARK_SMTP_HOST").unwrap_or_else(|_| "inbound.postmarkapp.com".to_string());
    let smtp_port = env::var("POSTMARK_SMTP_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(25);

    let mailer = lettre::SmtpTransport::builder_dangerous(&smtp_host)
        .port(smtp_port)
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
) -> Result<Vec<ScheduledTask>, BoxError> {
    let start = SystemTime::now();
    loop {
        let scheduler = Scheduler::load(tasks_path, NoopExecutor)?;
        let tasks = scheduler.tasks().to_vec();
        if !tasks.is_empty()
            && tasks
                .iter()
                .all(|task| !task.enabled && task.last_run.is_some())
        {
            return Ok(tasks);
        }
        if start.elapsed().unwrap_or_default() >= timeout {
            return Err("timed out waiting for tasks to complete".into());
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn wait_for_user_id(
    _users_db_path: &Path,
    users_root: &Path,
    _email: &str,
    timeout: Duration,
) -> Option<String> {
    let start = SystemTime::now();
    loop {
        if let Ok(entries) = fs::read_dir(users_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(name) = path.file_name().and_then(|value| value.to_str()) {
                        return Some(name.to_string());
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
fn rust_service_real_email_end_to_end() -> Result<(), BoxError> {
    load_env_from_repo();
    if !env_enabled("RUST_SERVICE_LIVE_TEST") {
        eprintln!("Skipping Rust service live email test. Set RUST_SERVICE_LIVE_TEST=1 to run.");
        return Ok(());
    }
    let log_buffer = Arc::new(Mutex::new(Vec::new()));
    let log_capture = LogCapture {
        buffer: Arc::clone(&log_buffer),
    };
    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_writer(log_capture)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("set tracing subscriber");

    let token = env::var("POSTMARK_SERVER_TOKEN")
        .map_err(|_| "POSTMARK_SERVER_TOKEN must be set for live tests")?;
    let public_url = resolve_postmark_hook_url();
    let from_addr =
        env::var("POSTMARK_TEST_FROM").unwrap_or_else(|_| "oliver@dowhiz.com".to_string());
    let service_address = env::var("POSTMARK_TEST_SERVICE_ADDRESS")
        .unwrap_or_else(|_| "oliver@dowhiz.com".to_string());
    let gateway_bind_host = env::var("GATEWAY_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let gateway_health_host = if gateway_bind_host == "0.0.0.0" {
        "127.0.0.1".to_string()
    } else {
        gateway_bind_host.clone()
    };
    let gateway_port = env::var("GATEWAY_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(9100);

    let server_info = postmark_request("GET", "https://api.postmarkapp.com/server", &token, None)?;
    let inbound_address = server_info
        .get("InboundAddress")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_string();
    if inbound_address.is_empty() {
        return Err("Postmark server does not have an inbound address configured".into());
    }
    let (mut employee_profile, mut employee_directory, employee_config_path) =
        load_employee_for_address(&service_address)?;
    attach_inbound_alias(
        &mut employee_profile,
        &mut employee_directory,
        &inbound_address,
    );
    let previous_hook = server_info
        .get("InboundHookUrl")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_string();
    let _restore = HookRestore {
        token: token.clone(),
        previous_hook: previous_hook.clone(),
    };

    dotenvy::dotenv().ok();
    let _service_bus_connection = match env_with_scale_oliver("SERVICE_BUS_CONNECTION_STRING") {
        Some(value) => value,
        None => {
            eprintln!(
                "Skipping live test: SCALE_OLIVER_SERVICE_BUS_CONNECTION_STRING or SERVICE_BUS_CONNECTION_STRING required for Service Bus ingestion."
            );
            return Ok(());
        }
    };
    let azure_container = env_with_scale_oliver("AZURE_STORAGE_CONTAINER_INGEST");
    let azure_sas_url = env_with_scale_oliver("AZURE_STORAGE_CONTAINER_SAS_URL");
    let azure_sas_token = env_with_scale_oliver("AZURE_STORAGE_SAS_TOKEN");
    let azure_account = env_with_scale_oliver("AZURE_STORAGE_ACCOUNT");
    let azure_conn_str = env_with_scale_oliver("AZURE_STORAGE_CONNECTION_STRING_INGEST");
    let has_azure_blob = azure_container.is_some()
        && (azure_sas_url.is_some()
            || (azure_sas_token.is_some()
                && (azure_account.is_some() || azure_conn_str.is_some())));
    if !has_azure_blob {
        eprintln!(
            "Skipping live test: Azure Blob SAS configuration is required (SCALE_OLIVER_AZURE_STORAGE_CONTAINER_SAS_URL or SCALE_OLIVER_AZURE_STORAGE_CONTAINER_INGEST + SCALE_OLIVER_AZURE_STORAGE_SAS_TOKEN + SCALE_OLIVER_AZURE_STORAGE_ACCOUNT/SCALE_OLIVER_AZURE_STORAGE_CONNECTION_STRING_INGEST)."
        );
        return Ok(());
    }
    if let Some(test_queue_name) = env_with_scale_oliver("SERVICE_BUS_TEST_QUEUE_NAME") {
        env::set_var("SERVICE_BUS_QUEUE_NAME", &test_queue_name);
        env::set_var("SCALE_OLIVER_SERVICE_BUS_QUEUE_NAME", &test_queue_name);
    }
    env::set_var("SCALE_OLIVER_INGESTION_QUEUE_BACKEND", "servicebus");
    env::set_var("SCALE_OLIVER_RAW_PAYLOAD_STORAGE_BACKEND", "azure");
    let temp = TempDir::new()?;
    let workspace_root = temp.path().join("workspaces");
    let state_dir = temp.path().join("state");
    let users_root = temp.path().join("users");
    fs::create_dir_all(&workspace_root)?;
    fs::create_dir_all(&state_dir)?;
    fs::create_dir_all(&users_root)?;

    let test_host = env::var("RUST_SERVICE_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("RUST_SERVICE_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(9001);

    let codex_disabled = !env_enabled("RUN_CODEX_E2E");
    let employee_id = employee_profile.id.clone();
    let effective_employee_config_path = state_dir.join("employee.live.toml");
    write_employee_config_with_inbound_alias(
        &employee_config_path,
        &effective_employee_config_path,
        &employee_id,
        &inbound_address,
    )?;
    let config = ServiceConfig {
        host: test_host.clone(),
        port,
        employee_id: employee_id.clone(),
        employee_config_path: effective_employee_config_path.clone(),
        employee_profile,
        employee_directory,
        workspace_root: workspace_root.clone(),
        scheduler_state_path: state_dir.join("tasks.db"),
        processed_ids_path: state_dir.join("postmark_processed_ids.txt"),
        ingestion_db_url: String::new(),
        ingestion_poll_interval: Duration::from_millis(50),
        users_root: users_root.clone(),
        users_db_path: state_dir.join("users.db"),
        task_index_path: state_dir.join("task_index.db"),
        codex_model: env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.3-codex".to_string()),
        codex_disabled,
        scheduler_poll_interval: Duration::from_secs(1),
        scheduler_max_concurrency: 10,
        scheduler_user_max_concurrency: 3,
        inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
        skills_source_dir: None,
        slack_bot_token: None,
        slack_bot_user_id: None,
        slack_store_path: state_dir.join("slack.db"),
        slack_client_id: None,
        slack_client_secret: None,
        slack_redirect_uri: None,
        discord_bot_token: None,
        discord_bot_user_id: None,
        google_docs_enabled: false,
        bluebubbles_url: None,
        bluebubbles_password: None,
        telegram_bot_token: None,
        whatsapp_access_token: None,
        whatsapp_phone_number_id: None,
        whatsapp_verify_token: None,
    };

    let gateway_config_path = state_dir.join("gateway.toml");
    write_gateway_config(
        &gateway_config_path,
        &gateway_bind_host,
        gateway_port,
        &inbound_address,
        &employee_id,
    )?;
    let _gateway = spawn_gateway(
        &gateway_config_path,
        &effective_employee_config_path,
        &gateway_bind_host,
        gateway_port,
    )?;
    wait_for_local_health(&gateway_health_host, gateway_port, Duration::from_secs(15))?;

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
    let base_url = base_url
        .strip_suffix("/postmark/inbound")
        .unwrap_or(base_url);
    check_public_health(base_url, &gateway_health_host, gateway_port)?;
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
    send_smtp_inbound(
        &from_addr,
        &inbound_address,
        &subject,
        Some(&service_address),
    )?;
    println!("Inbound message sent; waiting for workspace output...");

    let workspace_timeout = if env_enabled("RUN_CODEX_E2E") {
        Duration::from_secs(600)
    } else {
        Duration::from_secs(120)
    };

    println!("Waiting for user record...");
    let user_id = wait_for_user_id(
        &state_dir.join("users.db"),
        &users_root,
        &from_addr,
        workspace_timeout,
    )
    .ok_or("timed out waiting for user record")?;
    println!("User id resolved: {}", user_id);
    let workspace_root = users_root.join(&user_id).join("workspaces");
    println!("Waiting for workspace output...");
    let workspace = wait_for_workspace(&workspace_root, workspace_timeout)
        .ok_or("timed out waiting for workspace output")?;
    let reply_path = workspace.join("reply_email_draft.html");
    if !reply_path.exists() {
        return Err("reply_email_draft.html not written by run_task".into());
    }
    let inbound_attachment_path = workspace
        .join("incoming_attachments")
        .join(E2E_ATTACHMENT_NAME);
    if !inbound_attachment_path.exists() {
        return Err(format!(
            "expected inbound attachment at {}",
            inbound_attachment_path.display()
        )
        .into());
    }
    let inbound_attachment_bytes = fs::read(&inbound_attachment_path)?;
    if inbound_attachment_bytes != E2E_ATTACHMENT_CONTENT {
        return Err("workspace attachment bytes mismatch".into());
    }

    let payload_path = workspace
        .join("incoming_email")
        .join("postmark_payload.json");
    let payload_bytes = fs::read(&payload_path)?;
    let payload_json: Value = serde_json::from_slice(&payload_bytes)?;
    let attachments = payload_json
        .get("Attachments")
        .and_then(Value::as_array)
        .ok_or("postmark payload missing Attachments array")?;
    let attachment = attachments
        .iter()
        .find(|value| {
            value
                .get("Name")
                .and_then(Value::as_str)
                .map(|name| name == E2E_ATTACHMENT_NAME)
                .unwrap_or(false)
        })
        .ok_or("postmark payload missing expected attachment entry")?;
    let storage_ref = attachment
        .get("StorageRef")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or("attachment missing StorageRef")?;
    if !storage_ref.starts_with("azure://") {
        return Err(format!("unexpected StorageRef format: {}", storage_ref).into());
    }
    let content = attachment
        .get("Content")
        .and_then(Value::as_str)
        .unwrap_or("");
    if !content.is_empty() {
        return Err("attachment Content should be empty after blob offload".into());
    }
    let blob_bytes = scheduler_module::raw_payload_store::download_raw_payload(storage_ref)?;
    if blob_bytes != E2E_ATTACHMENT_CONTENT {
        return Err("blob attachment bytes mismatch".into());
    }

    let reply_subject = format!("Re: {}", subject);
    let outbound_timeout = if env_enabled("RUN_CODEX_E2E") {
        Duration::from_secs(300)
    } else {
        Duration::from_secs(120)
    };
    println!("Polling outbound for subject hint: {}", reply_subject);
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
    println!("Waiting for tasks to complete...");
    let tasks = wait_for_tasks_complete(&tasks_path, tasks_timeout)?;
    if tasks.len() < 2 {
        return Err("expected at least two scheduled tasks".into());
    }

    let _ = shutdown_tx.send(());
    let _ = rt.block_on(async { server_handle.await })?;
    drop(_gateway);
    temp.close()?;
    std::thread::sleep(Duration::from_millis(200));
    let log_guard = log_buffer
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    let logs = String::from_utf8_lossy(&log_guard);
    if logs.contains("unable to open database file") {
        return Err("database warning detected after cleanup".into());
    }

    Ok(())
}
