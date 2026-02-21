use chrono::{DateTime, Local, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tracing::warn;
use uuid::Uuid;

use crate::channel::Channel;

use super::actions::{apply_scheduler_actions, ingest_follow_up_tasks, schedule_auto_reply};
use super::executor::TaskExecutor;
use super::outbound::execute_slack_send;
use super::schedule::{next_run_after, validate_cron_expression};
use super::snapshot::{snapshot_reply_draft, write_scheduler_snapshot};
use super::store::SqliteSchedulerStore;
use super::types::{
    RunTaskTask, Schedule, ScheduledTask, SchedulerError, SendReplyTask, TaskKind,
    RUN_TASK_FAILURE_DIR, RUN_TASK_FAILURE_LIMIT, RUN_TASK_FAILURE_NOTICE,
    RUN_TASK_FAILURE_REPORT_DIR,
};

pub struct Scheduler<E: TaskExecutor> {
    pub(super) tasks: Vec<ScheduledTask>,
    executor: E,
    pub(super) store: SqliteSchedulerStore,
    failure_counts: HashMap<Uuid, u32>,
}

impl<E: TaskExecutor> Scheduler<E> {
    pub fn load(storage_path: impl Into<PathBuf>, executor: E) -> Result<Self, SchedulerError> {
        let storage_path = storage_path.into();
        let store = SqliteSchedulerStore::new(storage_path)?;
        let tasks = store.load_tasks()?;
        Ok(Self {
            tasks,
            executor,
            store,
            failure_counts: HashMap::new(),
        })
    }

    pub fn tasks(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    pub fn disable_tasks_by<F>(&mut self, mut predicate: F) -> Result<usize, SchedulerError>
    where
        F: FnMut(&ScheduledTask) -> bool,
    {
        let mut disabled = 0usize;
        for task in &mut self.tasks {
            if !task.enabled {
                continue;
            }
            if predicate(task) {
                task.enabled = false;
                self.store.update_task(task)?;
                disabled += 1;
            }
        }
        Ok(disabled)
    }

    pub fn add_cron_task(
        &mut self,
        expression: &str,
        kind: TaskKind,
    ) -> Result<Uuid, SchedulerError> {
        validate_cron_expression(expression)?;
        let now = Utc::now();
        let next_run = next_run_after(expression, now)?;

        let task = ScheduledTask {
            id: Uuid::new_v4(),
            kind,
            schedule: Schedule::Cron {
                expression: expression.to_string(),
                next_run,
            },
            enabled: true,
            created_at: now,
            last_run: None,
        };

        self.tasks.push(task);
        self.store.insert_task(self.tasks.last().unwrap())?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn add_one_shot_in(
        &mut self,
        delay: Duration,
        kind: TaskKind,
    ) -> Result<Uuid, SchedulerError> {
        let local_now = Local::now();
        let utc_now = local_now.with_timezone(&Utc);
        let chrono_delay =
            chrono::Duration::from_std(delay).map_err(|_| SchedulerError::DurationOutOfRange)?;
        let run_at = utc_now + chrono_delay;

        let task = ScheduledTask {
            id: Uuid::new_v4(),
            kind,
            schedule: Schedule::OneShot { run_at },
            enabled: true,
            created_at: utc_now,
            last_run: None,
        };

        self.tasks.push(task);
        self.store.insert_task(self.tasks.last().unwrap())?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn add_one_shot_at(
        &mut self,
        run_at: DateTime<Utc>,
        kind: TaskKind,
    ) -> Result<Uuid, SchedulerError> {
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            kind,
            schedule: Schedule::OneShot { run_at },
            enabled: true,
            created_at: Utc::now(),
            last_run: None,
        };

        self.tasks.push(task);
        self.store.insert_task(self.tasks.last().unwrap())?;
        Ok(self.tasks.last().unwrap().id)
    }

    pub fn execute_task_by_id(&mut self, task_id: Uuid) -> Result<bool, SchedulerError> {
        let now = Utc::now();
        let index = match self.tasks.iter().position(|task| task.id == task_id) {
            Some(index) => index,
            None => return Ok(false),
        };
        if !self.tasks[index].enabled || !self.tasks[index].is_due(now) {
            return Ok(false);
        }
        self.execute_task_at_index(index)?;
        Ok(true)
    }

    pub fn tick(&mut self) -> Result<(), SchedulerError> {
        let now = Utc::now();
        let task_count = self.tasks.len();
        for index in 0..task_count {
            if !self.tasks[index].enabled {
                continue;
            }
            if !self.tasks[index].is_due(now) {
                continue;
            }
            self.execute_task_at_index(index)?;
        }

        Ok(())
    }

    fn execute_task_at_index(&mut self, index: usize) -> Result<(), SchedulerError> {
        let task_id = self.tasks[index].id;
        let task_kind = self.tasks[index].kind.clone();
        if let TaskKind::RunTask(task) = &self.tasks[index].kind {
            if let Err(err) = write_scheduler_snapshot(&task.workspace_dir, &self.tasks, Utc::now())
            {
                warn!(
                    "failed to write scheduler snapshot for {}: {}",
                    task.workspace_dir.display(),
                    err
                );
            }
        }
        let started_at = Utc::now();
        let execution_id = self.store.record_execution_start(task_id, started_at)?;
        let result = self.executor.execute(&task_kind);
        let executed_at = Utc::now();

        match result {
            Ok(execution) => {
                self.failure_counts.remove(&task_id);
                self.store
                    .record_execution_finish(execution_id, executed_at, "success", None)?;
                self.tasks[index].last_run = Some(executed_at);
                match &mut self.tasks[index].schedule {
                    Schedule::Cron {
                        expression,
                        next_run,
                    } => {
                        *next_run = next_run_after(expression, executed_at)?;
                    }
                    Schedule::OneShot { .. } => {
                        self.tasks[index].enabled = false;
                    }
                }
                let updated_task = self.tasks[index].clone();
                self.store.update_task(&updated_task)?;
                if let TaskKind::RunTask(task) = &task_kind {
                    if let Some(err) = execution.follow_up_error.as_deref() {
                        warn!("scheduled tasks parse error: {}", err);
                    }
                    if let Err(err) = snapshot_reply_draft(task) {
                        warn!(
                            "failed to snapshot reply draft for {}: {}",
                            task.workspace_dir.display(),
                            err
                        );
                    }
                    ingest_follow_up_tasks(self, task, &execution.follow_up_tasks);
                    if let Err(err) = schedule_auto_reply(self, task) {
                        warn!(
                            "failed to schedule auto reply from {}: {}",
                            task.workspace_dir.display(),
                            err
                        );
                    }
                    if let Some(err) = execution.scheduler_actions_error.as_deref() {
                        warn!("scheduler actions parse error: {}", err);
                    }
                    if let Err(err) =
                        apply_scheduler_actions(self, task, &execution.scheduler_actions)
                    {
                        warn!(
                            "failed to apply scheduler actions from {}: {}",
                            task.workspace_dir.display(),
                            err
                        );
                    }
                }
            }
            Err(err) => {
                let message = err.to_string();
                self.store.record_execution_finish(
                    execution_id,
                    executed_at,
                    "failed",
                    Some(&message),
                )?;
                // Disable one-shot tasks on failure, but allow a few retries for RunTask.
                if matches!(self.tasks[index].schedule, Schedule::OneShot { .. }) {
                    let mut should_disable = true;
                    if let TaskKind::RunTask(task) = &self.tasks[index].kind {
                        let failures = self.failure_counts.entry(task_id).or_insert(0);
                        *failures += 1;
                        if *failures < RUN_TASK_FAILURE_LIMIT {
                            should_disable = false;
                        } else {
                            if let Err(err) = notify_run_task_failure(task_id, task, &message) {
                                warn!("failed to notify run_task failure: {}", err);
                            }
                            self.failure_counts.remove(&task_id);
                        }
                    }
                    if should_disable {
                        self.tasks[index].enabled = false;
                        let updated_task = self.tasks[index].clone();
                        self.store.update_task(&updated_task)?;
                        warn!(
                            "disabled one-shot task {} after failure: {}",
                            task_id, message
                        );
                    }
                }
                return Err(err);
            }
        }

        Ok(())
    }

    pub fn run_loop(
        &mut self,
        poll_interval: Duration,
        stop_flag: &AtomicBool,
    ) -> Result<(), SchedulerError> {
        while !stop_flag.load(Ordering::Relaxed) {
            self.tick()?;
            std::thread::sleep(poll_interval);
        }
        Ok(())
    }

    /// Get the current retry count for a task
    pub fn get_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        self.store.get_retry_count(task_id)
    }

    /// Increment the retry count for a task and return the new count
    pub fn increment_retry_count(&self, task_id: &str) -> Result<u32, SchedulerError> {
        self.store.increment_retry_count(task_id)
    }

    /// Reset the retry count for a task (after successful execution)
    pub fn reset_retry_count(&self, task_id: &str) -> Result<(), SchedulerError> {
        self.store.reset_retry_count(task_id)
    }

    /// Disable a task by its ID (used when max retries exceeded)
    pub fn disable_task_by_id(&mut self, task_id: &str) -> Result<(), SchedulerError> {
        // Update in-memory task list
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id.to_string() == task_id) {
            task.enabled = false;
        }
        // Update in database
        self.store.disable_task_by_id(task_id)
    }
}

fn notify_run_task_failure(
    task_id: Uuid,
    task: &RunTaskTask,
    error_message: &str,
) -> Result<(), SchedulerError> {
    let failure_dir = task.workspace_dir.join(RUN_TASK_FAILURE_DIR);
    std::fs::create_dir_all(&failure_dir)?;

    let is_slack = matches!(task.channel, Channel::Slack);
    let (notice_path, notice_body) = if is_slack {
        (
            failure_dir.join(format!("task_failure_{}.txt", task_id)),
            RUN_TASK_FAILURE_NOTICE.to_string(),
        )
    } else {
        (
            failure_dir.join(format!("task_failure_{}.html", task_id)),
            format!("<p>{}</p>", RUN_TASK_FAILURE_NOTICE),
        )
    };
    std::fs::write(&notice_path, notice_body)?;

    let notice_attachments = failure_dir.join(format!("task_failure_{}_attachments", task_id));
    std::fs::create_dir_all(&notice_attachments)?;

    if !task.reply_to.is_empty() {
        if is_slack {
            let send_task = SendReplyTask {
                channel: Channel::Slack,
                subject: RUN_TASK_FAILURE_NOTICE.to_string(),
                html_path: notice_path.clone(),
                attachments_dir: notice_attachments.clone(),
                from: None,
                to: task.reply_to.clone(),
                cc: vec![],
                bcc: vec![],
                in_reply_to: task.thread_id.clone(),
                references: None,
                archive_root: None,
                thread_epoch: None,
                thread_state_path: None,
            };
            execute_slack_send(&send_task)?;
        } else {
            let from = task
                .reply_from
                .clone()
                .or_else(|| std::env::var("ADMIN_EMAIL").ok())
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    SchedulerError::TaskFailed(
                        "from address missing for failure notice".to_string(),
                    )
                })?;
            let params = send_emails_module::SendEmailParams {
                subject: RUN_TASK_FAILURE_NOTICE.to_string(),
                html_path: notice_path.clone(),
                attachments_dir: notice_attachments.clone(),
                from: Some(from),
                to: task.reply_to.clone(),
                cc: vec![],
                bcc: vec![],
                in_reply_to: None,
                references: None,
                reply_to: None,
            };
            send_emails_module::send_email(&params)
                .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
        }
    } else {
        warn!("no reply_to recipients for task failure notice {}", task_id);
    }

    let admin_email = std::env::var("ADMIN_EMAIL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    if let Some(admin_email) = admin_email {
        let report_dir = std::env::temp_dir().join(RUN_TASK_FAILURE_REPORT_DIR);
        std::fs::create_dir_all(&report_dir)?;
        let report_path = report_dir.join(format!("task_failure_{}.html", task_id));
        let report_body = format!(
            "<p>{}</p><p>Task ID: {}</p><pre>{}</pre>",
            RUN_TASK_FAILURE_NOTICE, task_id, error_message
        );
        std::fs::write(&report_path, report_body)?;

        let report_attachments = report_dir.join(format!("attachments_{}", task_id));
        std::fs::create_dir_all(&report_attachments)?;
        let params = send_emails_module::SendEmailParams {
            subject: format!("Task failure: {}", task_id),
            html_path: report_path,
            attachments_dir: report_attachments,
            from: Some(admin_email.clone()),
            to: vec![admin_email],
            cc: vec![],
            bcc: vec![],
            in_reply_to: None,
            references: None,
            reply_to: None,
        };
        send_emails_module::send_email(&params)
            .map_err(|err| SchedulerError::TaskFailed(err.to_string()))?;
    } else {
        warn!("ADMIN_EMAIL not set; skipping failure report {}", task_id);
    }

    Ok(())
}
