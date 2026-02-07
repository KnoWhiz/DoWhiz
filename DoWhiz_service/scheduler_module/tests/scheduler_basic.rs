use scheduler_module::{Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

#[derive(Default, Clone)]
struct NoopExecutor;

impl TaskExecutor for NoopExecutor {
    fn execute(&self, _task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        Ok(TaskExecution::default())
    }
}

#[test]
fn cron_requires_six_fields() {
    let temp = tempfile::tempdir().expect("tempdir failed");
    let storage = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(storage, NoopExecutor).expect("load failed");

    let bad = scheduler.add_cron_task("0 0 * * *", TaskKind::Noop);
    assert!(bad.is_err(), "expected 5-field cron to fail");

    let good = scheduler.add_cron_task("0 0 0 * * *", TaskKind::Noop);
    assert!(good.is_ok(), "expected 6-field cron to succeed");
}

#[test]
fn one_shot_persists_across_restarts() {
    let temp = tempfile::tempdir().expect("tempdir failed");
    let storage = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(&storage, NoopExecutor).expect("load failed");

    scheduler
        .add_one_shot_in(Duration::from_secs(60), TaskKind::Noop)
        .expect("add one-shot failed");

    let loaded = Scheduler::load(&storage, NoopExecutor).expect("reload failed");
    assert_eq!(loaded.tasks().len(), 1);
}

#[test]
fn tick_disables_one_shot_tasks() {
    let temp = tempfile::tempdir().expect("tempdir failed");
    let storage = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(storage, NoopExecutor).expect("load failed");
    let task_id = scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::Noop)
        .expect("add one-shot failed");

    scheduler.tick().expect("tick failed");

    let task = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("task not found");
    assert!(!task.enabled, "one-shot task should be disabled after run");
}

#[test]
fn tick_sets_last_run_for_one_shot() {
    let temp = tempfile::tempdir().expect("tempdir failed");
    let storage = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(storage, NoopExecutor).expect("load failed");
    let task_id = scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::Noop)
        .expect("add one-shot failed");

    scheduler.tick().expect("tick failed");

    let task = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .expect("task not found");
    assert!(
        task.last_run.is_some(),
        "last_run should be set after execution"
    );
}

#[test]
fn run_loop_stops_when_flag_set() {
    let temp = tempfile::tempdir().expect("tempdir failed");
    let storage = temp.path().join("tasks.db");
    let mut scheduler = Scheduler::load(storage, NoopExecutor).expect("load failed");
    let stop_flag = AtomicBool::new(true);

    scheduler
        .run_loop(Duration::from_millis(10), &stop_flag)
        .expect("run_loop failed");
}
