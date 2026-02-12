use chrono::Utc;
use run_task_module::RunTaskParams;
use scheduler_module::{
    RunTaskTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind,
};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tempfile::TempDir;

struct EnvGuard {
    key: &'static str,
    previous: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let previous = env::var(key).ok();
        env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}

#[derive(Clone, Default)]
struct RecordingExecutor {
    sent_subjects: Arc<Mutex<Vec<String>>>,
}

impl TaskExecutor for RecordingExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        match task {
            TaskKind::RunTask(run) => {
                let params = RunTaskParams {
                    workspace_dir: run.workspace_dir.clone(),
                    input_email_dir: run.input_email_dir.clone(),
                    input_attachments_dir: run.input_attachments_dir.clone(),
                    memory_dir: run.memory_dir.clone(),
                    reference_dir: run.reference_dir.clone(),
                    reply_to: run.reply_to.clone(),
                    model_name: run.model_name.clone(),
                    runner: run.runner.clone(),
                    codex_disabled: run.codex_disabled,
                };
                let output = run_task_module::run_task(&params)
                    .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
                Ok(TaskExecution {
                    follow_up_tasks: output.scheduled_tasks,
                    follow_up_error: output.scheduled_tasks_error,
                    scheduler_actions: output.scheduler_actions,
                    scheduler_actions_error: output.scheduler_actions_error,
                })
            }
            TaskKind::SendReply(send) => {
                self.sent_subjects
                    .lock()
                    .expect("sent_subjects lock poisoned")
                    .push(send.subject.clone());
                Ok(TaskExecution::default())
            }
            TaskKind::Noop => Ok(TaskExecution::default()),
        }
    }
}

fn write_fake_codex(dir: &Path) -> io::Result<PathBuf> {
    let script = r#"#!/bin/sh
set -e
cat > reply_email_draft.html <<'HTML'
<html><body>Reply</body></html>
HTML
cat > reminder_email_draft.html <<'HTML'
<html><body>Reminder</body></html>
HTML
mkdir -p reminder_email_attachments
cat <<EOF
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_seconds":0,"subject":"Follow up","html_path":"reminder_email_draft.html","attachments_dir":"reminder_email_attachments","to":["you@example.com"],"cc":[],"bcc":[]}]
SCHEDULED_TASKS_JSON_END
SCHEDULER_ACTIONS_JSON_BEGIN
[
  {"action":"cancel","task_ids":["${CANCEL_TASK_ID}"]},
  {"action":"reschedule","task_id":"${RESCHEDULE_TASK_ID}","schedule":{"type":"one_shot","run_at":"${RESCHEDULE_RUN_AT}"}},
  {"action":"create_run_task","schedule":{"type":"one_shot","run_at":"${CREATE_RUN_AT}"}}
]
SCHEDULER_ACTIONS_JSON_END
EOF
exit 0
"#;
    let path = dir.join("codex");
    fs::write(&path, script)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }
    Ok(path)
}

#[test]
fn scheduler_actions_end_to_end() {
    let temp = TempDir::new().expect("tempdir");
    let root = temp.path();
    let bin_root = root.join("bin");
    let workspace = root.join("workspace");
    let memory = workspace.join("memory");
    let references = workspace.join("references");
    let input_email = workspace.join("incoming_email");
    let input_attachments = workspace.join("incoming_attachments");
    fs::create_dir_all(&bin_root).expect("bin root");
    fs::create_dir_all(&memory).expect("memory dir");
    fs::create_dir_all(&references).expect("references dir");
    fs::create_dir_all(&input_email).expect("input email dir");
    fs::create_dir_all(&input_attachments).expect("input attachments dir");

    write_fake_codex(&bin_root).expect("write fake codex");
    let original_path = env::var("PATH").unwrap_or_default();
    let path_value = format!("{}:{}", bin_root.display(), original_path);
    let _path_guard = EnvGuard::set("PATH", &path_value);
    let _docker_guard = EnvGuard::set("RUN_TASK_DOCKER_IMAGE", "");
    let _api_guard = EnvGuard::set("AZURE_OPENAI_API_KEY_BACKUP", "test-key");
    let _endpoint_guard = EnvGuard::set("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.test");

    let now = Utc::now();
    let reschedule_run_at = (now + chrono::Duration::seconds(30)).to_rfc3339();
    let create_run_at = (now + chrono::Duration::seconds(45)).to_rfc3339();

    let executor = RecordingExecutor::default();
    let sent_subjects = executor.sent_subjects.clone();
    let mut scheduler = Scheduler::load(root.join("tasks.db"), executor).expect("load scheduler");
    let cancel_id = scheduler
        .add_one_shot_at(now + chrono::Duration::seconds(60), TaskKind::Noop)
        .expect("cancel task");
    let reschedule_id = scheduler
        .add_one_shot_at(now + chrono::Duration::seconds(90), TaskKind::Noop)
        .expect("reschedule task");

    let _cancel_guard = EnvGuard::set("CANCEL_TASK_ID", &cancel_id.to_string());
    let _resched_guard = EnvGuard::set("RESCHEDULE_TASK_ID", &reschedule_id.to_string());
    let _resched_at_guard = EnvGuard::set("RESCHEDULE_RUN_AT", &reschedule_run_at);
    let _create_at_guard = EnvGuard::set("CREATE_RUN_AT", &create_run_at);

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-5.2-codex".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        reply_to: Vec::new(),
        reply_from: None,
        archive_root: None,
        thread_id: Some("thread-e2e".to_string()),
        thread_epoch: Some(1),
        thread_state_path: None,
        channel: scheduler_module::channel::Channel::default(),
    };

    scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
        .expect("add run_task");

    scheduler.tick().expect("tick run_task");

    let snapshot_path = workspace.join("scheduler_snapshot.json");
    assert!(snapshot_path.exists(), "scheduler snapshot missing");

    let canceled = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == cancel_id)
        .expect("cancel task present");
    assert!(!canceled.enabled, "cancel task should be disabled");

    let rescheduled = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == reschedule_id)
        .expect("reschedule task present");
    match &rescheduled.schedule {
        scheduler_module::Schedule::OneShot { run_at } => {
            let expected = chrono::DateTime::parse_from_rfc3339(&reschedule_run_at)
                .expect("parse reschedule")
                .with_timezone(&Utc);
            assert_eq!(*run_at, expected);
        }
        _ => panic!("expected one_shot schedule"),
    }
    assert!(rescheduled.enabled, "rescheduled task should be enabled");

    let created_run_task = scheduler
        .tasks()
        .iter()
        .find(|task| match &task.kind {
            TaskKind::RunTask(run) => {
                run.thread_id.as_deref() == Some("thread-e2e")
                    && matches!(task.schedule, scheduler_module::Schedule::OneShot { .. })
                    && task.enabled
            }
            _ => false,
        })
        .expect("created run_task missing");

    match &created_run_task.schedule {
        scheduler_module::Schedule::OneShot { run_at } => {
            let expected = chrono::DateTime::parse_from_rfc3339(&create_run_at)
                .expect("parse create")
                .with_timezone(&Utc);
            assert_eq!(*run_at, expected);
        }
        _ => panic!("expected one_shot schedule"),
    }

    let send_email_task = scheduler
        .tasks()
        .iter()
        .find(|task| matches!(task.kind, TaskKind::SendReply(_)))
        .expect("send_email task missing");
    if let TaskKind::SendReply(send) = &send_email_task.kind {
        assert_eq!(send.subject, "Follow up");
    }

    scheduler.tick().expect("tick send_email");
    let sent = sent_subjects.lock().expect("sent subjects lock");
    assert!(sent.iter().any(|subject| subject == "Follow up"));
}
