use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tempfile::TempDir;
use uuid::Uuid;

use crate::channel::Channel;

use super::{
    actions::{apply_scheduler_actions, schedule_send_email},
    snapshot::build_scheduler_snapshot,
    RunTaskTask, Schedule, ScheduledTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor,
    TaskKind,
};

#[derive(Default)]
struct NoopExecutor;

impl TaskExecutor for NoopExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Ok(TaskExecution::empty())
    }
}

struct FailingExecutor {
    message: String,
}

impl FailingExecutor {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl TaskExecutor for FailingExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Err(SchedulerError::TaskFailed(self.message.clone()))
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
        requester_identifier_type: None,
        requester_identifier: None,
    }
}

fn force_one_shot_due<E: TaskExecutor>(scheduler: &mut Scheduler<E>, task_id: Uuid) {
    let index = scheduler
        .tasks
        .iter()
        .position(|task| task.id == task_id)
        .expect("task exists");
    scheduler.tasks[index].enabled = true;
    scheduler.tasks[index].schedule = Schedule::OneShot {
        run_at: Utc::now() - chrono::Duration::seconds(1),
    };
    let updated = scheduler.tasks[index].clone();
    scheduler
        .store
        .update_task(&updated)
        .expect("persist forced one-shot schedule");
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

#[test]
fn schedule_send_email_supports_five_and_twenty_minute_reminders() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");

    let workspace = temp.path().join("workspaces").join("thread_1");
    let mail_root = temp.path().join("mail");
    fs::create_dir_all(workspace.join("reminder_email_attachments")).expect("attachments");
    fs::create_dir_all(&mail_root).expect("mail");
    fs::write(
        workspace.join("reminder_email_draft.html"),
        "<html><body>Reminder</body></html>",
    )
    .expect("html");

    let run_task = base_run_task(&workspace, &mail_root);

    let request_5 = run_task_module::ScheduledSendEmailTask {
        subject: "Reminder in 5 minutes".to_string(),
        html_path: "reminder_email_draft.html".to_string(),
        attachments_dir: Some("reminder_email_attachments".to_string()),
        from: None,
        to: vec!["user@example.com".to_string()],
        cc: Vec::new(),
        bcc: Vec::new(),
        delay_minutes: Some(5),
        delay_seconds: None,
        run_at: None,
    };
    let request_20 = run_task_module::ScheduledSendEmailTask {
        subject: "Reminder in 20 minutes".to_string(),
        html_path: "reminder_email_draft.html".to_string(),
        attachments_dir: Some("reminder_email_attachments".to_string()),
        from: None,
        to: vec!["user@example.com".to_string()],
        cc: Vec::new(),
        bcc: Vec::new(),
        delay_minutes: Some(20),
        delay_seconds: None,
        run_at: None,
    };

    let now_before_first = Utc::now();
    assert!(schedule_send_email(&mut scheduler, &run_task, &request_5).expect("schedule 5"));
    let now_before_second = Utc::now();
    assert!(schedule_send_email(&mut scheduler, &run_task, &request_20).expect("schedule 20"));
    let now_after_second = Utc::now();

    let mut five_min_run_at = None;
    let mut twenty_min_run_at = None;
    for task in scheduler.tasks() {
        if let TaskKind::SendReply(send_task) = &task.kind {
            if send_task.subject == "Reminder in 5 minutes" {
                if let Schedule::OneShot { run_at } = task.schedule.clone() {
                    five_min_run_at = Some(run_at);
                }
            }
            if send_task.subject == "Reminder in 20 minutes" {
                if let Schedule::OneShot { run_at } = task.schedule.clone() {
                    twenty_min_run_at = Some(run_at);
                }
            }
        }
    }

    let five_min_run_at = five_min_run_at.expect("5 minute task");
    let twenty_min_run_at = twenty_min_run_at.expect("20 minute task");

    let min_5 = now_before_first + chrono::Duration::minutes(5);
    let max_5 = now_before_second + chrono::Duration::minutes(5) + chrono::Duration::seconds(5);
    assert!(
        five_min_run_at >= min_5 && five_min_run_at <= max_5,
        "5-minute reminder run_at out of range: {} not in [{}, {}]",
        five_min_run_at,
        min_5,
        max_5
    );

    let min_20 = now_before_second + chrono::Duration::minutes(20);
    let max_20 = now_after_second + chrono::Duration::minutes(20) + chrono::Duration::seconds(5);
    assert!(
        twenty_min_run_at >= min_20 && twenty_min_run_at <= max_20,
        "20-minute reminder run_at out of range: {} not in [{}, {}]",
        twenty_min_run_at,
        min_20,
        max_20
    );

    let gap = twenty_min_run_at - five_min_run_at;
    assert!(
        gap >= chrono::Duration::minutes(14) && gap <= chrono::Duration::minutes(16),
        "expected ~15 minute gap between reminders, got {} seconds",
        gap.num_seconds()
    );
}

#[test]
fn add_one_shot_in_with_id_uses_specified_id() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");

    // Create a specific UUID
    let specific_id = Uuid::new_v4();

    // Add task with specific ID
    scheduler
        .add_one_shot_in_with_id(specific_id, Duration::from_secs(0), TaskKind::Noop)
        .expect("add task with id");

    // Verify the task has the specified ID
    assert_eq!(scheduler.tasks().len(), 1);
    assert_eq!(scheduler.tasks()[0].id, specific_id);
}

#[test]
fn same_task_id_can_be_used_in_different_schedulers() {
    let temp = TempDir::new().expect("tempdir");

    // Create two separate tasks.db files (simulating workspace and user storage)
    let workspace_db = temp.path().join("workspace_tasks.db");
    let user_db = temp.path().join("user_tasks.db");

    let mut workspace_scheduler =
        Scheduler::load(&workspace_db, NoopExecutor::default()).expect("load workspace");
    let mut user_scheduler = Scheduler::load(&user_db, NoopExecutor::default()).expect("load user");

    // Add task to workspace scheduler (generates new ID)
    let task_id = workspace_scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::Noop)
        .expect("add to workspace");

    // Add same task to user scheduler with the SAME ID
    user_scheduler
        .add_one_shot_in_with_id(task_id, Duration::from_secs(0), TaskKind::Noop)
        .expect("add to user with same id");

    // Verify both schedulers have a task with the same ID
    assert_eq!(workspace_scheduler.tasks().len(), 1);
    assert_eq!(user_scheduler.tasks().len(), 1);
    assert_eq!(workspace_scheduler.tasks()[0].id, task_id);
    assert_eq!(user_scheduler.tasks()[0].id, task_id);
}

#[test]
fn add_one_shot_in_with_id_persists_to_database() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let specific_id = Uuid::new_v4();

    // Add task with specific ID
    {
        let mut scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in_with_id(specific_id, Duration::from_secs(0), TaskKind::Noop)
            .expect("add task");
    }

    // Reload scheduler and verify task is still there with correct ID
    {
        let scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("reload");
        assert_eq!(scheduler.tasks().len(), 1);
        assert_eq!(scheduler.tasks()[0].id, specific_id);
    }
}

#[test]
fn execution_status_can_be_recorded_for_task() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let specific_id = Uuid::new_v4();

    // Add task and record execution
    {
        let mut scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in_with_id(specific_id, Duration::from_secs(0), TaskKind::Noop)
            .expect("add task");

        // Record execution start and finish (this is what sync_task_status_to_user_storage does)
        let now = Utc::now();
        let execution_id = scheduler
            .store
            .record_execution_start(specific_id, now)
            .expect("record start");
        scheduler
            .store
            .record_execution_finish(specific_id, execution_id, now, "success", None)
            .expect("record finish");
    }

    // Verify execution status is persisted by loading tasks with status
    {
        use super::store::SchedulerStore;
        let store = SchedulerStore::new(tasks_db).expect("open store");
        let tasks = store.list_tasks_with_status().expect("list tasks");

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, specific_id.to_string());
        assert_eq!(tasks[0].execution_status, Some("success".to_string()));
    }
}

#[test]
fn scheduler_load_ignores_zero_byte_placeholder_path() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    fs::write(&tasks_db, "").expect("create zero-byte db");

    let scheduler = Scheduler::load(&tasks_db, NoopExecutor::default()).expect("load");
    assert!(scheduler.tasks().is_empty());

    let size = fs::metadata(&tasks_db).expect("metadata").len();
    assert_eq!(
        size, 0,
        "mongo backend should not mutate placeholder state file"
    );

    let mut quarantined = false;
    for entry in fs::read_dir(temp.path()).expect("read dir") {
        let entry = entry.expect("entry");
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with("tasks.db.corrupt.") {
            quarantined = true;
            break;
        }
    }
    assert!(
        !quarantined,
        "mongo backend should not emit legacy corruption quarantine files"
    );
}

/// Helper to create a Discord-style RunTaskTask
fn discord_run_task(workspace: &Path) -> RunTaskTask {
    RunTaskTask {
        workspace_dir: workspace.to_path_buf(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-test".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        // Discord uses reply_to[0] = user_id, reply_to[1] = channel_id
        reply_to: vec!["discord_user_123".to_string(), "channel_456".to_string()],
        reply_from: None,
        archive_root: None,
        thread_id: Some("discord:guild123:channel456:thread789".to_string()),
        thread_epoch: Some(1),
        thread_state_path: Some(workspace.join("thread_state.json")),
        channel: Channel::Discord,
        slack_team_id: None,
        employee_id: Some("test_employee".to_string()),
        requester_identifier_type: None,
        requester_identifier: None,
    }
}

/// Helper to create a Slack-style RunTaskTask
fn slack_run_task(workspace: &Path) -> RunTaskTask {
    RunTaskTask {
        workspace_dir: workspace.to_path_buf(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-test".to_string(),
        runner: "codex".to_string(),
        codex_disabled: false,
        // Slack uses reply_to[0] = channel_id
        reply_to: vec!["C12345678".to_string()],
        reply_from: None,
        archive_root: None,
        thread_id: Some("slack:C12345678:1234567890.123456".to_string()),
        thread_epoch: Some(1),
        thread_state_path: Some(workspace.join("thread_state.json")),
        channel: Channel::Slack,
        slack_team_id: Some("T12345678".to_string()),
        employee_id: Some("test_employee".to_string()),
        requester_identifier_type: None,
        requester_identifier: None,
    }
}

#[test]
fn full_discord_flow_task_sync_and_status_update() {
    let temp = TempDir::new().expect("tempdir");

    // Simulate Discord workspace and user storage paths
    let workspace_dir = temp
        .path()
        .join("workspaces")
        .join("discord")
        .join("guild123");
    let workspace_db = workspace_dir.join("state").join("tasks.db");
    let user_db = temp
        .path()
        .join("users")
        .join("account_abc")
        .join("state")
        .join("tasks.db");

    fs::create_dir_all(workspace_db.parent().unwrap()).expect("create workspace dir");
    fs::create_dir_all(user_db.parent().unwrap()).expect("create user dir");
    fs::create_dir_all(&workspace_dir).expect("create workspace");

    let run_task = discord_run_task(&workspace_dir);

    // Step 1: Create task in workspace scheduler (simulates discord.rs)
    let task_id = {
        let mut workspace_scheduler =
            Scheduler::load(&workspace_db, NoopExecutor::default()).expect("load workspace");
        workspace_scheduler
            .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task.clone()))
            .expect("add to workspace")
    };

    // Step 2: Create same task in user scheduler with same ID (simulates discord.rs account sync)
    {
        let mut user_scheduler =
            Scheduler::load(&user_db, NoopExecutor::default()).expect("load user");
        user_scheduler
            .add_one_shot_in_with_id(
                task_id,
                Duration::from_secs(0),
                TaskKind::RunTask(run_task.clone()),
            )
            .expect("add to user");
    }

    // Verify both have the task with same ID
    {
        use super::store::SchedulerStore;
        let workspace_store = SchedulerStore::new(workspace_db.clone()).expect("open workspace");
        let user_store = SchedulerStore::new(user_db.clone()).expect("open user");

        let workspace_tasks = workspace_store
            .list_tasks_with_status()
            .expect("list workspace");
        let user_tasks = user_store.list_tasks_with_status().expect("list user");

        assert_eq!(workspace_tasks.len(), 1);
        assert_eq!(user_tasks.len(), 1);
        assert_eq!(workspace_tasks[0].id, task_id.to_string());
        assert_eq!(user_tasks[0].id, task_id.to_string());
        assert_eq!(workspace_tasks[0].channel, "discord");
        assert_eq!(user_tasks[0].channel, "discord");
        // Both should have no execution status yet
        assert!(workspace_tasks[0].execution_status.is_none());
        assert!(user_tasks[0].execution_status.is_none());
    }

    // Step 3: Simulate task execution in workspace (core.rs execute_task_at_index)
    let executed_at = Utc::now();
    {
        use super::store::SchedulerStore;
        let workspace_store = SchedulerStore::new(workspace_db.clone()).expect("open workspace");
        let execution_id = workspace_store
            .record_execution_start(task_id, executed_at)
            .expect("record start");
        workspace_store
            .record_execution_finish(task_id, execution_id, executed_at, "success", None)
            .expect("record finish");
    }

    // Step 4: Sync status to user storage (simulates sync_task_status_to_user_storage)
    {
        use super::store::SchedulerStore;
        let user_store = SchedulerStore::new(user_db.clone()).expect("open user");
        let execution_id = user_store
            .record_execution_start(task_id, executed_at)
            .expect("record start");
        user_store
            .record_execution_finish(task_id, execution_id, executed_at, "success", None)
            .expect("record finish");
    }

    // Verify both now have success status
    {
        use super::store::SchedulerStore;
        let workspace_store = SchedulerStore::new(workspace_db).expect("open workspace");
        let user_store = SchedulerStore::new(user_db).expect("open user");

        let workspace_tasks = workspace_store
            .list_tasks_with_status()
            .expect("list workspace");
        let user_tasks = user_store.list_tasks_with_status().expect("list user");

        assert_eq!(
            workspace_tasks[0].execution_status,
            Some("success".to_string())
        );
        assert_eq!(user_tasks[0].execution_status, Some("success".to_string()));
    }
}

#[test]
fn full_slack_flow_task_sync_and_status_update() {
    let temp = TempDir::new().expect("tempdir");

    // Simulate Slack workspace (uses user paths) and account storage
    let user_workspace_dir = temp
        .path()
        .join("users")
        .join("slack_user")
        .join("workspaces")
        .join("thread1");
    let workspace_db = temp
        .path()
        .join("users")
        .join("slack_user")
        .join("state")
        .join("tasks.db");
    let account_db = temp
        .path()
        .join("users")
        .join("account_xyz")
        .join("state")
        .join("tasks.db");

    fs::create_dir_all(workspace_db.parent().unwrap()).expect("create workspace dir");
    fs::create_dir_all(account_db.parent().unwrap()).expect("create account dir");
    fs::create_dir_all(&user_workspace_dir).expect("create workspace");

    let run_task = slack_run_task(&user_workspace_dir);

    // Step 1: Create task in user's scheduler (Slack uses user paths)
    let task_id = {
        let mut scheduler = Scheduler::load(&workspace_db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task.clone()))
            .expect("add task")
    };

    // Step 2: Create same task in account-level storage with same ID
    {
        let mut account_scheduler =
            Scheduler::load(&account_db, NoopExecutor::default()).expect("load account");
        account_scheduler
            .add_one_shot_in_with_id(
                task_id,
                Duration::from_secs(0),
                TaskKind::RunTask(run_task.clone()),
            )
            .expect("add to account");
    }

    // Verify both have Slack channel type
    {
        use super::store::SchedulerStore;
        let workspace_store = SchedulerStore::new(workspace_db.clone()).expect("open workspace");
        let account_store = SchedulerStore::new(account_db.clone()).expect("open account");

        let workspace_tasks = workspace_store
            .list_tasks_with_status()
            .expect("list workspace");
        let account_tasks = account_store
            .list_tasks_with_status()
            .expect("list account");

        assert_eq!(workspace_tasks[0].channel, "slack");
        assert_eq!(account_tasks[0].channel, "slack");
        assert_eq!(workspace_tasks[0].id, account_tasks[0].id);
    }

    // Step 3: Simulate failed execution
    let executed_at = Utc::now();
    let error_message = "Task failed: API timeout";
    {
        use super::store::SchedulerStore;
        let workspace_store = SchedulerStore::new(workspace_db.clone()).expect("open workspace");
        let execution_id = workspace_store
            .record_execution_start(task_id, executed_at)
            .expect("record start");
        workspace_store
            .record_execution_finish(task_id, execution_id, executed_at, "failed", Some(error_message))
            .expect("record finish");
    }

    // Step 4: Sync failure status to account storage
    {
        use super::store::SchedulerStore;
        let account_store = SchedulerStore::new(account_db.clone()).expect("open account");
        let execution_id = account_store
            .record_execution_start(task_id, executed_at)
            .expect("record start");
        account_store
            .record_execution_finish(task_id, execution_id, executed_at, "failed", Some(error_message))
            .expect("record finish");
    }

    // Verify both have failure status with error message
    {
        use super::store::SchedulerStore;
        let workspace_store = SchedulerStore::new(workspace_db).expect("open workspace");
        let account_store = SchedulerStore::new(account_db).expect("open account");

        let workspace_tasks = workspace_store
            .list_tasks_with_status()
            .expect("list workspace");
        let account_tasks = account_store
            .list_tasks_with_status()
            .expect("list account");

        assert_eq!(
            workspace_tasks[0].execution_status,
            Some("failed".to_string())
        );
        assert_eq!(
            account_tasks[0].execution_status,
            Some("failed".to_string())
        );
        assert_eq!(
            workspace_tasks[0].error_message,
            Some(error_message.to_string())
        );
        assert_eq!(
            account_tasks[0].error_message,
            Some(error_message.to_string())
        );
    }
}

#[test]
fn multiple_tasks_sync_independently() {
    let temp = TempDir::new().expect("tempdir");

    let workspace_db = temp.path().join("workspace_tasks.db");
    let user_db = temp.path().join("user_tasks.db");
    let workspace_dir = temp.path().join("workspace");
    fs::create_dir_all(&workspace_dir).expect("create workspace");

    // Create two Discord tasks
    let run_task_1 = discord_run_task(&workspace_dir);
    let mut run_task_2 = discord_run_task(&workspace_dir);
    run_task_2.thread_id = Some("discord:guild123:channel456:thread_different".to_string());

    // Add both tasks to workspace
    let task_id_1 = {
        let mut scheduler = Scheduler::load(&workspace_db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in(
                Duration::from_secs(0),
                TaskKind::RunTask(run_task_1.clone()),
            )
            .expect("add 1")
    };
    let task_id_2 = {
        let mut scheduler = Scheduler::load(&workspace_db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in(
                Duration::from_secs(0),
                TaskKind::RunTask(run_task_2.clone()),
            )
            .expect("add 2")
    };

    // Sync both to user storage
    {
        let mut user_scheduler =
            Scheduler::load(&user_db, NoopExecutor::default()).expect("load user");
        user_scheduler
            .add_one_shot_in_with_id(
                task_id_1,
                Duration::from_secs(0),
                TaskKind::RunTask(run_task_1),
            )
            .expect("sync 1");
        user_scheduler
            .add_one_shot_in_with_id(
                task_id_2,
                Duration::from_secs(0),
                TaskKind::RunTask(run_task_2),
            )
            .expect("sync 2");
    }

    // Mark task 1 as success, task 2 as failed
    let executed_at = Utc::now();
    {
        use super::store::SchedulerStore;
        let user_store = SchedulerStore::new(user_db.clone()).expect("open user");

        // Task 1: success
        let exec_id_1 = user_store
            .record_execution_start(task_id_1, executed_at)
            .expect("start 1");
        user_store
            .record_execution_finish(task_id_1, exec_id_1, executed_at, "success", None)
            .expect("finish 1");

        // Task 2: failed
        let exec_id_2 = user_store
            .record_execution_start(task_id_2, executed_at)
            .expect("start 2");
        user_store
            .record_execution_finish(task_id_2, exec_id_2, executed_at, "failed", Some("timeout"))
            .expect("finish 2");
    }

    // Verify each task has correct status
    {
        use super::store::SchedulerStore;
        let user_store = SchedulerStore::new(user_db).expect("open user");
        let tasks = user_store.list_tasks_with_status().expect("list");

        assert_eq!(tasks.len(), 2);

        let task_1 = tasks
            .iter()
            .find(|t| t.id == task_id_1.to_string())
            .expect("find task 1");
        let task_2 = tasks
            .iter()
            .find(|t| t.id == task_id_2.to_string())
            .expect("find task 2");

        assert_eq!(task_1.execution_status, Some("success".to_string()));
        assert_eq!(task_2.execution_status, Some("failed".to_string()));
        assert!(task_1.error_message.is_none());
        assert_eq!(task_2.error_message, Some("timeout".to_string()));
    }
}

#[test]
fn run_task_failures_persist_retry_count_and_disable_at_limit() {
    let temp = TempDir::new().expect("tempdir");
    let tasks_db = temp.path().join("tasks.db");
    let workspace = temp.path().join("workspace");
    let mail_root = temp.path().join("mail");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::create_dir_all(&mail_root).expect("mail");
    let run_task = base_run_task(&workspace, &mail_root);
    let quota_error = "ContainerGroupQuotaReached: container group quota reached";

    let mut scheduler =
        Scheduler::load(&tasks_db, FailingExecutor::new(quota_error)).expect("load scheduler");
    let task_id = scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
        .expect("add run_task");
    assert!(scheduler.execute_task_by_id(task_id).is_err());

    let mut scheduler =
        Scheduler::load(&tasks_db, FailingExecutor::new(quota_error)).expect("reload scheduler");
    let first_retry_task = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("task exists");
    let first_retry_at = match first_retry_task.schedule {
        Schedule::OneShot { run_at } => run_at,
        _ => panic!("expected one-shot schedule"),
    };
    assert!(first_retry_task.enabled);
    assert!(first_retry_at > Utc::now() + chrono::Duration::seconds(120));
    assert_eq!(
        scheduler
            .get_retry_count(&task_id.to_string())
            .expect("retry count"),
        1
    );

    force_one_shot_due(&mut scheduler, task_id);
    assert!(scheduler.execute_task_by_id(task_id).is_err());
    assert_eq!(
        scheduler
            .get_retry_count(&task_id.to_string())
            .expect("retry count"),
        2
    );

    force_one_shot_due(&mut scheduler, task_id);
    assert!(scheduler.execute_task_by_id(task_id).is_err());

    let scheduler =
        Scheduler::load(&tasks_db, FailingExecutor::new(quota_error)).expect("final reload");
    let final_task = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("final task exists");
    assert!(!final_task.enabled);
    assert_eq!(
        scheduler
            .get_retry_count(&task_id.to_string())
            .expect("retry count"),
        0
    );
}

#[test]
fn run_task_channel_is_preserved_in_sync() {
    let temp = TempDir::new().expect("tempdir");
    let workspace_dir = temp.path().join("workspace");
    fs::create_dir_all(&workspace_dir).expect("create workspace");

    // Test Discord
    {
        let db = temp.path().join("discord_tasks.db");
        let run_task = discord_run_task(&workspace_dir);
        let mut scheduler = Scheduler::load(&db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
            .expect("add");

        use super::store::SchedulerStore;
        let store = SchedulerStore::new(db).expect("open");
        let tasks = store.list_tasks_with_status().expect("list");
        assert_eq!(tasks[0].channel, "discord");
    }

    // Test Slack
    {
        let db = temp.path().join("slack_tasks.db");
        let run_task = slack_run_task(&workspace_dir);
        let mut scheduler = Scheduler::load(&db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
            .expect("add");

        use super::store::SchedulerStore;
        let store = SchedulerStore::new(db).expect("open");
        let tasks = store.list_tasks_with_status().expect("list");
        assert_eq!(tasks[0].channel, "slack");
    }

    // Test Email (default)
    {
        let db = temp.path().join("email_tasks.db");
        let mail_root = temp.path().join("mail");
        fs::create_dir_all(&mail_root).expect("mail");
        let run_task = base_run_task(&workspace_dir, &mail_root);
        let mut scheduler = Scheduler::load(&db, NoopExecutor::default()).expect("load");
        scheduler
            .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
            .expect("add");

        use super::store::SchedulerStore;
        let store = SchedulerStore::new(db).expect("open");
        let tasks = store.list_tasks_with_status().expect("list");
        assert_eq!(tasks[0].channel, "email");
    }
}
