use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use uuid::Uuid;

use crate::channel::Channel;

use super::{
    actions::apply_scheduler_actions, snapshot::build_scheduler_snapshot, RunTaskTask, Schedule,
    ScheduledTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind,
};

#[derive(Default)]
struct NoopExecutor;

impl TaskExecutor for NoopExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Ok(TaskExecution::empty())
    }
}

fn base_run_task(workspace: &Path, mail_root: &Path) -> RunTaskTask {
    RunTaskTask {
        workspace_dir: workspace.to_path_buf(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-test".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        reply_to: vec!["user@example.com".to_string()],
        reply_from: None,
        archive_root: Some(mail_root.to_path_buf()),
        thread_id: Some("thread-test".to_string()),
        thread_epoch: Some(1),
        thread_state_path: Some(workspace.join("thread_state.json")),
        channel: Channel::default(),
        slack_team_id: None,
        employee_id: None,
    }
}

#[test]
fn build_scheduler_snapshot_limits_to_window() {
    let now = Utc::now();
    let in_window = ScheduledTask {
        id: Uuid::new_v4(),
        kind: TaskKind::Noop,
        schedule: Schedule::OneShot {
            run_at: now + chrono::Duration::days(1),
        },
        enabled: true,
        created_at: now,
        last_run: None,
    };
    let out_window = ScheduledTask {
        id: Uuid::new_v4(),
        kind: TaskKind::Noop,
        schedule: Schedule::OneShot {
            run_at: now + chrono::Duration::days(10),
        },
        enabled: true,
        created_at: now,
        last_run: None,
    };

    let snapshot = build_scheduler_snapshot(&[in_window, out_window], now);
    assert_eq!(snapshot.upcoming.len(), 1);
    assert_eq!(snapshot.omitted_after_window, 1);
    assert_eq!(snapshot.total_enabled, 2);
}

#[test]
fn apply_scheduler_actions_cancels_and_reschedules() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");
    let now = Utc::now();

    let cancel_id = scheduler
        .add_one_shot_at(now + chrono::Duration::days(1), TaskKind::Noop)
        .expect("cancel task");
    let resched_id = scheduler
        .add_one_shot_at(now + chrono::Duration::days(2), TaskKind::Noop)
        .expect("resched task");

    let workspace = temp.path().join("workspaces").join("thread_1");
    let mail_root = temp.path().join("mail");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::create_dir_all(&mail_root).expect("mail");
    let run_task = base_run_task(&workspace, &mail_root);

    let new_run_at = (now + chrono::Duration::days(3)).to_rfc3339();
    let actions = vec![
        run_task_module::SchedulerActionRequest::Cancel {
            task_ids: vec![cancel_id.to_string()],
        },
        run_task_module::SchedulerActionRequest::Reschedule {
            task_id: resched_id.to_string(),
            schedule: run_task_module::ScheduleRequest::OneShot { run_at: new_run_at },
        },
    ];

    apply_scheduler_actions(&mut scheduler, &run_task, &actions).expect("apply actions");

    let canceled = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == cancel_id)
        .expect("cancel task found");
    assert!(!canceled.enabled);

    let rescheduled = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == resched_id)
        .expect("resched task found");
    match &rescheduled.schedule {
        Schedule::OneShot { run_at } => {
            assert!(*run_at >= now + chrono::Duration::days(3));
        }
        _ => panic!("expected one_shot schedule"),
    }
    assert!(rescheduled.enabled);
}

#[test]
fn apply_scheduler_actions_creates_run_task() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");
    let now = Utc::now();

    let workspace = temp.path().join("workspaces").join("thread_1");
    let mail_root = temp.path().join("mail");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::create_dir_all(&mail_root).expect("mail");
    let run_task = base_run_task(&workspace, &mail_root);

    let run_at = (now + chrono::Duration::hours(2)).to_rfc3339();
    let actions = vec![run_task_module::SchedulerActionRequest::CreateRunTask {
        schedule: run_task_module::ScheduleRequest::OneShot { run_at },
        model_name: None,
        codex_disabled: None,
        reply_to: Vec::new(),
    }];

    apply_scheduler_actions(&mut scheduler, &run_task, &actions).expect("apply actions");

    assert_eq!(scheduler.tasks().len(), 1);
    match &scheduler.tasks()[0].kind {
        TaskKind::RunTask(task) => {
            assert_eq!(task.workspace_dir, workspace);
            assert_eq!(task.model_name, "gpt-test");
        }
        _ => panic!("expected run_task kind"),
    }
}
