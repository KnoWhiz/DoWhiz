use mockito::{Matcher, Server};
use scheduler_module::{
    channel::Channel, RunTaskTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use tempfile::TempDir;

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

fn success_body(to: &str) -> String {
    format!(
        r#"{{"To":"{to}","SubmittedAt":"2024-01-01T00:00:00Z","MessageID":"msg-123","ErrorCode":0,"Message":"OK"}}"#
    )
}

#[test]
fn run_task_failure_retries_and_notifies() -> Result<(), Box<dyn std::error::Error>> {
    let _lock = ENV_MUTEX.lock().unwrap();

    let mut server = Server::new();
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
    };

    let db_path = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&db_path, AlwaysFailExecutor)?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

    // First two failures should not disable or notify.
    for _ in 0..2 {
        let _ = scheduler.tick();
    }

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
            if name.contains(&needle) {
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
