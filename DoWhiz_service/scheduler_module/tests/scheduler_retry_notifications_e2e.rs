mod test_support;

use chrono::Utc;
use mockito::Matcher;
use mongodb::bson::{doc, Bson, DateTime as BsonDateTime, Document};
use mongodb::sync::Client;
use scheduler_module::{
    channel::Channel, RunTaskTask, Schedule, ScheduledTask, Scheduler, SchedulerError,
    TaskExecution, TaskExecutor, TaskKind,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use tempfile::TempDir;
use uuid::Uuid;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

struct EnvGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let original = env::var(key).ok();
        env::set_var(key, value);
        Self { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct AlwaysFailExecutor;

impl TaskExecutor for AlwaysFailExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Err(SchedulerError::TaskFailed("boom".to_string()))
    }
}

#[derive(Debug, Default, Clone)]
struct TransientCodexFailExecutor;

impl TaskExecutor for TransientCodexFailExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Err(SchedulerError::TaskFailed(
            "stream disconnected before completion: response.failed event received".to_string(),
        ))
    }
}

fn success_body(to: &str) -> String {
    format!(
        r#"{{"To":"{to}","SubmittedAt":"2024-01-01T00:00:00Z","MessageID":"msg-123","ErrorCode":0,"Message":"OK"}}"#
    )
}

fn require_mongodb_uri(test_name: &str) -> Option<String> {
    match env::var("MONGODB_URI") {
        Ok(value) if !value.trim().is_empty() => Some(value),
        _ => {
            eprintln!("Skipping {test_name}; MONGODB_URI not set.");
            None
        }
    }
}

fn sanitize_fragment(value: &str) -> String {
    let mut output = String::new();
    let mut last_was_underscore = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            last_was_underscore = false;
        } else if !last_was_underscore {
            output.push('_');
            last_was_underscore = true;
        }
    }

    output.trim_matches('_').to_string()
}

fn mongo_database_name() -> String {
    if let Ok(value) = env::var("MONGODB_DATABASE") {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    let target = env::var("DEPLOY_TARGET")
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "production".to_string());
    let employee = env::var("EMPLOYEE_ID")
        .ok()
        .map(|value| sanitize_fragment(&value))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "default".to_string());
    format!("dowhiz_{}_{}", sanitize_fragment(&target), employee)
}

fn force_one_shot_due(
    mongo_uri: &str,
    tasks_db_path: &Path,
    task_id: Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_uri_str(mongo_uri)?;
    let db_name = mongo_database_name();
    let tasks = client.database(&db_name).collection::<Document>("tasks");
    let owner_id = format!(
        "{:x}",
        md5::compute(tasks_db_path.to_string_lossy().as_bytes())
    );
    let filter = doc! {
        "owner_scope.kind": "path_scope",
        "owner_scope.id": owner_id,
        "task_id": task_id.to_string(),
    };

    let task_doc = tasks
        .find_one(filter.clone(), None)?
        .ok_or("task document not found")?;
    let task_json = task_doc.get_str("task_json")?;
    let mut task: ScheduledTask = serde_json::from_str(task_json)?;
    let due_at = Utc::now() - chrono::Duration::seconds(1);
    task.enabled = true;
    task.schedule = Schedule::OneShot { run_at: due_at };
    let updated_task_json = serde_json::to_string(&task)?;

    tasks.update_one(
        filter,
        doc! {
            "$set": {
                "enabled": true,
                "schedule": {
                    "type": "one_shot",
                    "cron_expression": Bson::Null,
                    "next_run": Bson::Null,
                    "run_at": BsonDateTime::from_chrono(due_at),
                },
                "task_json": updated_task_json,
            }
        },
        None,
    )?;
    Ok(())
}

#[test]
fn run_task_failure_retries_and_notifies() -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let Some(mongo_uri) = require_mongodb_uri("run_task_failure_retries_and_notifies") else {
        return Ok(());
    };

    let Some(mut server) =
        test_support::start_mockito_server("run_task_failure_retries_and_notifies")
    else {
        return Ok(());
    };
    let admin_addr = "admin@example.com";
    let user_addr = "user@example.com";

    let user_mock = server
        .mock("POST", "/email")
        .match_header("x-postmark-server-token", "test-token")
        .match_body(Matcher::Regex(user_addr.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(success_body(user_addr))
        .expect(1)
        .create();

    let admin_mock = server
        .mock("POST", "/email")
        .match_header("x-postmark-server-token", "test-token")
        .match_body(Matcher::Regex(admin_addr.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(success_body(admin_addr))
        .expect(1)
        .create();

    let _guard_token = EnvGuard::set("POSTMARK_SERVER_TOKEN", "test-token");
    let _guard_api = EnvGuard::set("POSTMARK_API_BASE_URL", server.url());
    let _guard_admin = EnvGuard::set("ADMIN_EMAIL", admin_addr);

    let temp = TempDir::new()?;
    let workspace = temp.path().join("workspace");
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;
    fs::write(
        incoming_dir.join("postmark_payload.json"),
        r#"{"Subject":"Test subject","MessageID":"<msg-id>"}"#,
    )?;

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-test".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        reply_to: vec![user_addr.to_string()],
        reply_from: Some("service@example.com".to_string()),
        archive_root: None,
        thread_id: None,
        thread_epoch: None,
        thread_state_path: None,
        channel: Channel::Email,
        slack_team_id: None,
        employee_id: None,
        requester_identifier_type: None,
        requester_identifier: None,
        account_id: None,
    };

    let db_path = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&db_path, AlwaysFailExecutor)?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

    // First two failures should not disable or notify.
    let _ = scheduler.tick();
    force_one_shot_due(&mongo_uri, &db_path, task_id)?;
    scheduler = Scheduler::load(&db_path, AlwaysFailExecutor)?;
    let _ = scheduler.tick();

    let task = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("task exists");
    assert!(
        task.enabled,
        "task should remain enabled before third failure"
    );

    let failure_dir = workspace.join("failure_notifications");
    if failure_dir.exists() {
        let mut entries = fs::read_dir(&failure_dir)?;
        assert!(
            entries.next().is_none(),
            "no failure notice before third attempt"
        );
    }

    // Third failure should disable and notify.
    force_one_shot_due(&mongo_uri, &db_path, task_id)?;
    scheduler = Scheduler::load(&db_path, AlwaysFailExecutor)?;
    let _ = scheduler.tick();

    let reloaded = Scheduler::load(&db_path, AlwaysFailExecutor)?;
    let task = reloaded
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("task exists after reload");
    assert!(!task.enabled, "task should be disabled after third failure");

    let mut user_notice_files = Vec::new();
    if failure_dir.exists() {
        for entry in fs::read_dir(&failure_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("html") {
                user_notice_files.push(path);
            }
        }
    }
    assert_eq!(
        user_notice_files.len(),
        1,
        "expected one user failure notice"
    );
    let notice_html = fs::read_to_string(&user_notice_files[0])?;
    assert!(
        notice_html.contains("We could not complete your request"),
        "user failure notice should be English"
    );

    let report_dir = env::temp_dir().join("dowhiz_failure_reports");
    let mut report_files = Vec::new();
    if report_dir.exists() {
        let needle = format!("task_failure_{}", task_id);
        for entry in fs::read_dir(&report_dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("");
            if name.contains(&needle)
                && path.extension().and_then(|ext| ext.to_str()) == Some("html")
            {
                report_files.push(path);
            }
        }
    }
    assert_eq!(report_files.len(), 1, "expected one admin failure report");
    let report_html = fs::read_to_string(&report_files[0])?;
    assert!(
        report_html.contains(&format!("Task ID: {}", task_id)),
        "admin report should include task id"
    );

    user_mock.assert();
    admin_mock.assert();

    Ok(())
}

#[test]
fn transient_codex_failures_send_retry_alerts_and_terminal_notice(
) -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();
    let Some(mongo_uri) =
        require_mongodb_uri("transient_codex_failures_send_retry_alerts_and_terminal_notice")
    else {
        return Ok(());
    };

    let Some(mut server) =
        test_support::start_mockito_server("transient_codex_failures_send_retry_alerts")
    else {
        return Ok(());
    };
    let admin_addr = "admin@example.com";
    let user_addr = "user@example.com";

    let user_mock = server
        .mock("POST", "/email")
        .match_header("x-postmark-server-token", "test-token")
        .match_body(Matcher::Regex(user_addr.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(success_body(user_addr))
        .expect(1)
        .create();

    let admin_mock = server
        .mock("POST", "/email")
        .match_header("x-postmark-server-token", "test-token")
        .match_body(Matcher::Regex(admin_addr.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(success_body(admin_addr))
        .expect(3)
        .create();

    let _guard_token = EnvGuard::set("POSTMARK_SERVER_TOKEN", "test-token");
    let _guard_api = EnvGuard::set("POSTMARK_API_BASE_URL", server.url());
    let _guard_admin = EnvGuard::set("ADMIN_EMAIL", admin_addr);

    let temp = TempDir::new()?;
    let workspace = temp.path().join("workspace");
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;
    fs::write(
        incoming_dir.join("postmark_payload.json"),
        r#"{"Subject":"","TextBody":"Please resend the draft","MessageID":"<msg-id>"}"#,
    )?;

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-test".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        reply_to: vec![user_addr.to_string()],
        reply_from: Some("service@example.com".to_string()),
        archive_root: None,
        thread_id: None,
        thread_epoch: None,
        thread_state_path: None,
        channel: Channel::Email,
        slack_team_id: None,
        employee_id: None,
        requester_identifier_type: None,
        requester_identifier: None,
        account_id: None,
    };

    let db_path = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&db_path, TransientCodexFailExecutor)?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

    let _ = scheduler.tick();
    force_one_shot_due(&mongo_uri, &db_path, task_id)?;
    scheduler = Scheduler::load(&db_path, TransientCodexFailExecutor)?;
    let _ = scheduler.tick();
    force_one_shot_due(&mongo_uri, &db_path, task_id)?;
    scheduler = Scheduler::load(&db_path, TransientCodexFailExecutor)?;
    let _ = scheduler.tick();

    let reloaded = Scheduler::load(&db_path, TransientCodexFailExecutor)?;
    let task = reloaded
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("task exists after reload");
    assert!(!task.enabled, "task should be disabled after third failure");

    let failure_dir = workspace.join("failure_notifications");
    let notice_html =
        fs::read_to_string(failure_dir.join(format!("task_failure_{}.html", task_id)))?;
    assert!(
        notice_html.contains("temporary execution issue"),
        "transient terminal notice should explain the execution issue"
    );

    let report_dir = env::temp_dir().join("dowhiz_failure_reports");
    let retry_one = fs::read_to_string(
        report_dir.join(format!("task_retry_alert_{}_attempt_1.html", task_id)),
    )?;
    assert!(
        retry_one.contains("Failure class: codex_stream_disconnected"),
        "retry alert should classify the transient Codex failure"
    );
    assert!(
        retry_one.contains("Next attempt: 2/3 in 180 seconds"),
        "first retry alert should describe the 180 second backoff"
    );

    let retry_two = fs::read_to_string(
        report_dir.join(format!("task_retry_alert_{}_attempt_2.html", task_id)),
    )?;
    assert!(
        retry_two.contains("Next attempt: 3/3 in 360 seconds"),
        "second retry alert should describe the doubled backoff"
    );

    let final_report =
        fs::read_to_string(report_dir.join(format!("task_failure_{}.html", task_id)))?;
    assert!(
        final_report.contains("Failure class: codex_stream_disconnected"),
        "terminal report should preserve the failure classification"
    );
    assert!(
        final_report.contains("Retry status: retries exhausted"),
        "terminal report should say retries were exhausted"
    );

    user_mock.assert();
    admin_mock.assert();

    Ok(())
}
