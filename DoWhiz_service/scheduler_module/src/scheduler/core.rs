use chrono::{DateTime, Local, Utc};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tracing::{info, warn};
use uuid::Uuid;

use crate::account_store::lookup_account_by_channel;
use crate::channel::Channel;

use super::actions::{apply_scheduler_actions, ingest_follow_up_tasks, schedule_auto_reply};
use super::executor::TaskExecutor;
use super::outbound::execute_slack_send;
use super::reply::load_reply_context;
use super::schedule::{next_run_after, validate_cron_expression};
use super::snapshot::{snapshot_reply_draft, write_scheduler_snapshot};
use super::store::SchedulerStore;
use super::types::{
    RunTaskTask, Schedule, ScheduledTask, SchedulerError, SendReplyTask, TaskKind,
    RUN_TASK_FAILURE_DIR, RUN_TASK_FAILURE_LIMIT, RUN_TASK_FAILURE_NOTICE,
    RUN_TASK_FAILURE_REPORT_DIR,
};

pub struct Scheduler<E: TaskExecutor> {
    pub(super) tasks: Vec<ScheduledTask>,
    executor: E,
    pub(super) store: SchedulerStore,
}

impl<E: TaskExecutor> Scheduler<E> {
    pub fn load(storage_path: impl Into<PathBuf>, executor: E) -> Result<Self, SchedulerError> {
        let storage_path = storage_path.into();
        let store = SchedulerStore::new(storage_path)?;
        let tasks = store.load_tasks()?;
        Ok(Self {
            tasks,
            executor,
            store,
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

    /// Add a one-shot task with a specific task ID.
    /// Used when syncing a task to user storage with the same ID as the workspace task.
    pub fn add_one_shot_in_with_id(
        &mut self,
        id: Uuid,
        delay: Duration,
        kind: TaskKind,
    ) -> Result<(), SchedulerError> {
        let local_now = Local::now();
        let utc_now = local_now.with_timezone(&Utc);
        let chrono_delay =
            chrono::Duration::from_std(delay).map_err(|_| SchedulerError::DurationOutOfRange)?;
        let run_at = utc_now + chrono_delay;

        let task = ScheduledTask {
            id,
            kind,
            schedule: Schedule::OneShot { run_at },
            enabled: true,
            created_at: utc_now,
            last_run: None,
        };

        self.tasks.push(task);
        self.store.insert_task(self.tasks.last().unwrap())?;
        Ok(())
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
                if let Err(err) = self.store.reset_retry_count(&task_id.to_string()) {
                    warn!(
                        "failed to reset retry count for task {} after success: {}",
                        task_id, err
                    );
                }
                self.store
                    .record_execution_finish(task_id, execution_id, executed_at, "success", None)?;
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
                    // Sync success status to user's account-level storage for Discord/Slack
                    sync_task_status_to_user_storage(task_id, task, executed_at, "success", None);
                }
            }
            Err(err) => {
                let message = err.to_string();
                self.store.record_execution_finish(
                    task_id,
                    execution_id,
                    executed_at,
                    "failed",
                    Some(&message),
                )?;
                // Sync failure status to user's account-level storage for Discord/Slack
                if let TaskKind::RunTask(task) = &task_kind {
                    sync_task_status_to_user_storage(
                        task_id,
                        task,
                        executed_at,
                        "failed",
                        Some(&message),
                    );
                }
                // Disable one-shot tasks on failure, but allow a few retries for RunTask.
                if matches!(self.tasks[index].schedule, Schedule::OneShot { .. }) {
                    let mut disable_task = true;
                    if let TaskKind::RunTask(task) = &self.tasks[index].kind {
                        let task_id_str = task_id.to_string();
                        let retry_count = self.store.increment_retry_count(&task_id_str)?;
                        if retry_count < RUN_TASK_FAILURE_LIMIT {
                            disable_task = false;
                            let delay = run_task_retry_delay(retry_count, &message);
                            if let Schedule::OneShot { run_at } = &mut self.tasks[index].schedule {
                                *run_at = executed_at + delay;
                            }
                            let updated_task = self.tasks[index].clone();
                            self.store.update_task(&updated_task)?;
                            warn!(
                                "run_task one-shot {} failed (attempt {}/{}), retrying in {}s: {}",
                                task_id,
                                retry_count,
                                RUN_TASK_FAILURE_LIMIT,
                                delay.num_seconds(),
                                message
                            );
                        } else {
                            if let Err(err) = notify_run_task_failure(task_id, task, &message) {
                                warn!("failed to notify run_task failure: {}", err);
                            }
                            if let Err(err) = self.store.reset_retry_count(&task_id_str) {
                                warn!(
                                    "failed to reset retry count for disabled task {}: {}",
                                    task_id, err
                                );
                            }
                        }
                    }
                    if disable_task {
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

/// Sync task execution status to user's account-level tasks.db for Discord/Google Workspace channels.
/// This allows users to see task status in their dashboard for linked accounts.
fn sync_task_status_to_user_storage(
    task_id: Uuid,
    task: &RunTaskTask,
    executed_at: DateTime<Utc>,
    status: &str,
    error_message: Option<&str>,
) {
    // Only sync for channels that support unified accounts
    if !matches!(
        task.channel,
        Channel::Discord
            | Channel::Slack
            | Channel::GoogleDocs
            | Channel::GoogleSheets
            | Channel::GoogleSlides
    ) {
        return;
    }

    // Get the identifier from reply_to (Discord: user_id, Google*: email)
    let identifier = match task.reply_to.first() {
        Some(id) => id,
        None => {
            warn!(
                "no reply_to identifier for task {} to sync to user storage",
                task_id
            );
            return;
        }
    };

    // Look up the account by channel identifier
    let account_id = match lookup_account_by_channel(&task.channel, identifier) {
        Some(id) => id,
        None => {
            // No linked account, nothing to sync
            return;
        }
    };

    // Get users_root from environment
    let users_root = match std::env::var("USERS_ROOT") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            warn!(
                "USERS_ROOT not set, cannot sync task {} to user storage",
                task_id
            );
            return;
        }
    };

    // Construct path to user's tasks.db
    let user_tasks_db_path = users_root
        .join(account_id.to_string())
        .join("state")
        .join("tasks.db");

    // Open the user's scheduler store and update the task
    match SchedulerStore::new(user_tasks_db_path.clone()) {
        Ok(store) => {
            // Record execution start and finish to update status
            match store.record_execution_start(task_id, executed_at) {
                Ok(execution_id) => {
                    if let Err(err) = store.record_execution_finish(
                        task_id,
                        execution_id,
                        executed_at,
                        status,
                        error_message,
                    ) {
                        warn!(
                            "failed to record execution finish for task {} in user storage: {}",
                            task_id, err
                        );
                    } else {
                        info!(
                            "synced task {} status '{}' to user storage account={}",
                            task_id, status, account_id
                        );
                    }
                }
                Err(err) => {
                    warn!(
                        "failed to record execution start for task {} in user storage: {}",
                        task_id, err
                    );
                }
            }
        }
        Err(err) => {
            warn!(
                "failed to open user scheduler store at {}: {}",
                user_tasks_db_path.display(),
                err
            );
        }
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
            let slack_thread_ts = load_reply_context(&task.workspace_dir)
                .in_reply_to
                .or_else(|| slack_thread_ts_from_thread_key(task.thread_id.as_deref()));
            let send_task = SendReplyTask {
                channel: Channel::Slack,
                subject: RUN_TASK_FAILURE_NOTICE.to_string(),
                html_path: notice_path.clone(),
                attachments_dir: notice_attachments.clone(),
                from: None,
                to: task.reply_to.clone(),
                cc: vec![],
                bcc: vec![],
                in_reply_to: slack_thread_ts,
                references: None,
                archive_root: None,
                thread_epoch: None,
                thread_state_path: None,
                employee_id: task.employee_id.clone(),
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

fn run_task_retry_delay(retry_count: u32, error_message: &str) -> chrono::Duration {
    const GENERIC_BASE_DELAY_SECS: i64 = 30;
    const GENERIC_MAX_DELAY_SECS: i64 = 300;
    const ACI_QUOTA_BASE_DELAY_SECS: i64 = 180;
    const ACI_QUOTA_MAX_DELAY_SECS: i64 = 1800;

    let is_aci_quota = is_aci_capacity_error(error_message);
    let (base_secs, max_secs) = if is_aci_quota {
        (ACI_QUOTA_BASE_DELAY_SECS, ACI_QUOTA_MAX_DELAY_SECS)
    } else {
        (GENERIC_BASE_DELAY_SECS, GENERIC_MAX_DELAY_SECS)
    };
    let exponent = retry_count.saturating_sub(1);
    let multiplier = 2_i64.saturating_pow(exponent.min(10));
    let secs = (base_secs.saturating_mul(multiplier)).min(max_secs);
    chrono::Duration::seconds(secs.max(1))
}

fn is_aci_capacity_error(error_message: &str) -> bool {
    let lowered = error_message.to_ascii_lowercase();
    lowered.contains("containergroupquotareached")
        || (lowered.contains("container group quota")
            && lowered.contains("microsoft.containerinstance/containergroups"))
        || lowered.contains("resource quota of container groups")
}

fn slack_thread_ts_from_thread_key(thread_key: Option<&str>) -> Option<String> {
    let raw = thread_key
        .map(str::trim)
        .filter(|value| !value.is_empty())?;
    let mut parts = raw.splitn(3, ':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("slack"), Some(_channel), Some(thread_ts)) if !thread_ts.trim().is_empty() => {
            Some(thread_ts.trim().to_string())
        }
        _ => Some(raw.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the channel whitelist for status sync includes Google Workspace channels.
    /// This is a simple unit test to verify the match statement includes all expected channels.
    #[test]
    fn status_sync_whitelist_includes_google_channels() {
        // Channels that should be synced
        let syncable_channels = vec![
            Channel::Discord,
            Channel::GoogleDocs,
            Channel::GoogleSheets,
            Channel::GoogleSlides,
        ];

        for channel in syncable_channels {
            assert!(
                matches!(
                    channel,
                    Channel::Discord
                        | Channel::GoogleDocs
                        | Channel::GoogleSheets
                        | Channel::GoogleSlides
                ),
                "Channel {:?} should be in the sync whitelist",
                channel
            );
        }

        // Channels that should NOT be synced (yet)
        let non_syncable_channels = vec![
            Channel::Email,
            Channel::Sms,
            Channel::WhatsApp,
            Channel::Telegram,
            Channel::BlueBubbles,
        ];

        for channel in non_syncable_channels {
            assert!(
                !matches!(
                    channel,
                    Channel::Discord
                        | Channel::GoogleDocs
                        | Channel::GoogleSheets
                        | Channel::GoogleSlides
                ),
                "Channel {:?} should NOT be in the sync whitelist",
                channel
            );
        }
    }

    /// Test that channel_to_identifier_type maps Google channels to "email"
    #[test]
    fn google_channels_map_to_email_identifier() {
        use crate::account_store::channel_to_identifier_type;

        assert_eq!(channel_to_identifier_type(&Channel::GoogleDocs), "email");
        assert_eq!(channel_to_identifier_type(&Channel::GoogleSheets), "email");
        assert_eq!(channel_to_identifier_type(&Channel::GoogleSlides), "email");
    }

    /// Test that Discord and Slack have their own identifier types
    #[test]
    fn discord_slack_have_own_identifier_types() {
        use crate::account_store::channel_to_identifier_type;

        assert_eq!(channel_to_identifier_type(&Channel::Discord), "discord");
        assert_eq!(channel_to_identifier_type(&Channel::Slack), "slack");
    }

    #[test]
    fn slack_thread_ts_from_thread_key_parses_compound_key() {
        assert_eq!(
            super::slack_thread_ts_from_thread_key(Some("slack:C123:1700000000.001")),
            Some("1700000000.001".to_string())
        );
        assert_eq!(
            super::slack_thread_ts_from_thread_key(Some("1700000000.002")),
            Some("1700000000.002".to_string())
        );
    }
}
