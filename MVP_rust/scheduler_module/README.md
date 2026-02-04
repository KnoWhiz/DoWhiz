# scheduler_module

Cron-based scheduler (6-field format with seconds) that persists tasks to disk and runs them in UTC. Supports two task types for now: send email and run task.

## Usage

- Cron format: `sec min hour day month weekday` (6 fields).
- All scheduling is evaluated in UTC.
- One-shot tasks convert local time to UTC before scheduling.

Example:
```rust
use scheduler_module::{ModuleExecutor, Scheduler, TaskKind, SendEmailTask};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

let storage_path = PathBuf::from("/tmp/dowhiz_tasks.db");
let executor = ModuleExecutor::default();
let mut scheduler = Scheduler::load(storage_path, executor)?;

let task = SendEmailTask {
    subject: "Hello".to_string(),
    html_path: PathBuf::from("/path/to/reply_email_draft.html"),
    attachments_dir: PathBuf::from("/path/to/reply_email_attachments"),
    to: vec!["mini-mouse@deep-tutor.com".to_string()],
    cc: Vec::new(),
    bcc: Vec::new(),
    in_reply_to: None,
    references: None,
};

scheduler.add_cron_task("0 */5 * * * *", TaskKind::SendEmail(task))?;

let stop_flag = AtomicBool::new(false);
scheduler.run_loop(Duration::from_secs(1), &stop_flag)?;
```

## Folder structure

- `MVP_rust/scheduler_module/src/lib.rs` : Scheduler core, task definitions, persistence.
- `MVP_rust/scheduler_module/tests/` : Cron validation, persistence, and tick tests.

## Notes

- Tasks are stored in a SQLite database at the provided storage path and reloaded on startup.
- Execution attempts are recorded in the `task_executions` table with status and error details.
- Use `add_one_shot_in` to schedule “N minutes from now” tasks based on local time converted to UTC.
