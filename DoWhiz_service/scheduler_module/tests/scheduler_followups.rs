use run_task_module::{ScheduledSendEmailTask, ScheduledTaskRequest};
use scheduler_module::{
    RunTaskTask, Scheduler, SchedulerError, TaskExecution, TaskExecutor, TaskKind,
};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Default, Clone)]
struct FollowUpExecutor;

impl TaskExecutor for FollowUpExecutor {
    fn execute(&self, task: &TaskKind) -> Result<TaskExecution, SchedulerError> {
        match task {
            TaskKind::RunTask(_) => {
                let follow_up = ScheduledTaskRequest::SendEmail(ScheduledSendEmailTask {
                    subject: "Follow up".to_string(),
                    html_path: "followup.html".to_string(),
                    attachments_dir: Some("followup_attachments".to_string()),
                    from: None,
                    to: vec!["you@example.com".to_string()],
                    cc: Vec::new(),
                    bcc: Vec::new(),
                    delay_minutes: Some(0),
                    delay_seconds: None,
                    run_at: None,
                });
                Ok(TaskExecution {
                    follow_up_tasks: vec![follow_up],
                    follow_up_error: None,
                    scheduler_actions: Vec::new(),
                    scheduler_actions_error: None,
                })
            }
            _ => Ok(TaskExecution::default()),
        }
    }
}

#[test]
fn run_task_followups_persist_to_sqlite() {
    let temp = tempfile::tempdir().expect("tempdir failed");
    let storage = temp.path().join("tasks.db");
    let workspace = temp.path().join("workspace");
    fs::create_dir_all(&workspace).expect("create workspace");
    fs::write(workspace.join("followup.html"), "<html>Hi</html>").expect("write followup html");
    fs::create_dir_all(workspace.join("followup_attachments")).expect("attachments dir");

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: "gpt-5.2-codex".to_string(),
        runner: "codex".to_string(),
        codex_disabled: true,
        reply_to: Vec::new(),
        reply_from: None,
        archive_root: None,
        thread_id: None,
        thread_epoch: None,
        thread_state_path: None,
    };

    let mut scheduler =
        Scheduler::load(&storage, FollowUpExecutor::default()).expect("load failed");
    scheduler
        .add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))
        .expect("add run_task failed");

    scheduler.tick().expect("tick failed");
    assert_eq!(scheduler.tasks().len(), 2);

    let reloaded = Scheduler::load(&storage, FollowUpExecutor::default()).expect("reload failed");
    assert_eq!(reloaded.tasks().len(), 2);

    let send_task = reloaded
        .tasks()
        .iter()
        .find_map(|task| {
            if let TaskKind::SendEmail(send) = &task.kind {
                Some(send)
            } else {
                None
            }
        })
        .expect("send_email task missing");
    assert_eq!(send_task.subject, "Follow up");
    assert_eq!(send_task.html_path, workspace.join("followup.html"));
}
