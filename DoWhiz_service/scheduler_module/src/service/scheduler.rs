use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::index_store::{IndexStore, TaskRef};
use crate::thread_state::default_thread_state_path;
use crate::user_store::UserStore;
use crate::{ModuleExecutor, Schedule, ScheduledTask, Scheduler, SchedulerError, TaskKind};

use super::config::ServiceConfig;
use super::state::{ClaimResult, ConcurrencyLimiter, SchedulerClaims, TaskClaim};
use super::BoxError;

/// Default task timeout in seconds (10 minutes)
const DEFAULT_TASK_TIMEOUT_SECS: u64 = 600;
/// Maximum number of retries before giving up
const MAX_TASK_RETRIES: u32 = 3;
/// Watchdog check interval in seconds
const WATCHDOG_INTERVAL_SECS: u64 = 30;

pub(super) struct SchedulerControl {
    stop: Arc<AtomicBool>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl SchedulerControl {
    pub(super) fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub(super) fn stop_and_join(&mut self) {
        self.stop();
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

pub(super) fn start_scheduler_threads(
    config: Arc<ServiceConfig>,
    user_store: Arc<UserStore>,
    index_store: Arc<IndexStore>,
) -> SchedulerControl {
    let scheduler_stop = Arc::new(AtomicBool::new(false));
    let scheduler_poll_interval = config.scheduler_poll_interval;
    let scheduler_max_concurrency = config.scheduler_max_concurrency;
    let scheduler_user_max_concurrency = config.scheduler_user_max_concurrency;
    let claims = Arc::new(Mutex::new(SchedulerClaims::default()));
    let running_threads = Arc::new(Mutex::new(HashSet::new()));
    let limiter = Arc::new(ConcurrencyLimiter::new(scheduler_max_concurrency));

    let mut handles = Vec::with_capacity(2);

    {
        let config = config.clone();
        let user_store = user_store.clone();
        let index_store = index_store.clone();
        let scheduler_stop = scheduler_stop.clone();
        let claims = claims.clone();
        let running_threads = running_threads.clone();
        let limiter = limiter.clone();
        let query_limit = scheduler_max_concurrency.saturating_mul(4).max(1);
        let handle = thread::spawn(move || {
            let mut last_due_tasks: HashSet<String> = HashSet::new();
            let mut logged_user_busy: HashSet<String> = HashSet::new();
            let mut logged_task_busy: HashSet<String> = HashSet::new();
            let mut last_capacity_deferral: Option<usize> = None;
            while !scheduler_stop.load(Ordering::Relaxed) {
                let now = Utc::now();
                match index_store.due_task_refs(now, query_limit) {
                    Ok(task_refs) => {
                        let mut current_due_tasks = HashSet::with_capacity(task_refs.len());
                        for task_ref in &task_refs {
                            current_due_tasks
                                .insert(format!("{}@{}", task_ref.task_id, task_ref.user_id));
                        }
                        if current_due_tasks != last_due_tasks {
                            if !current_due_tasks.is_empty() {
                                let refs = task_refs
                                    .iter()
                                    .map(|task_ref| {
                                        format!("{}@{}", task_ref.task_id, task_ref.user_id)
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                info!("scheduler found {} due task(s): {}", task_refs.len(), refs);
                            }
                            last_due_tasks = current_due_tasks.clone();
                        }
                        logged_user_busy.retain(|key| current_due_tasks.contains(key));
                        logged_task_busy.retain(|key| current_due_tasks.contains(key));
                        if current_due_tasks.is_empty() {
                            last_capacity_deferral = None;
                        }
                        let total_refs = task_refs.len();
                        for (idx, task_ref) in task_refs.into_iter().enumerate() {
                            if !limiter.try_acquire() {
                                let remaining = total_refs.saturating_sub(idx);
                                if last_capacity_deferral != Some(remaining) {
                                    info!(
                                        "scheduler at capacity; deferring {} due task(s)",
                                        remaining
                                    );
                                    last_capacity_deferral = Some(remaining);
                                }
                                break;
                            }
                            last_capacity_deferral = None;
                            let task_key = format!("{}@{}", task_ref.task_id, task_ref.user_id);
                            let claim_result = {
                                let mut claims =
                                    claims.lock().unwrap_or_else(|poison| poison.into_inner());
                                // TODO: Get retry_count from task metadata in the future
                                claims.try_claim(&task_ref, scheduler_user_max_concurrency, 0)
                            };
                            match claim_result {
                                ClaimResult::Claimed => {
                                    logged_user_busy.remove(&task_key);
                                    logged_task_busy.remove(&task_key);
                                    info!(
                                        "scheduler claimed task {} for user {}",
                                        task_ref.task_id, task_ref.user_id
                                    );
                                }
                                ClaimResult::UserBusy => {
                                    if logged_user_busy.insert(task_key) {
                                        info!(
                                            "scheduler deferred task {} for user {} (user already running)",
                                            task_ref.task_id, task_ref.user_id
                                        );
                                    }
                                    limiter.release();
                                    continue;
                                }
                                ClaimResult::TaskBusy => {
                                    if logged_task_busy.insert(task_key) {
                                        info!(
                                            "scheduler deferred task {} for user {} (task already running)",
                                            task_ref.task_id, task_ref.user_id
                                        );
                                    }
                                    limiter.release();
                                    continue;
                                }
                            }

                            let config = config.clone();
                            let user_store = user_store.clone();
                            let index_store = index_store.clone();
                            let claims = claims.clone();
                            let limiter = limiter.clone();
                            let running_threads = running_threads.clone();
                            thread::spawn(move || {
                                if let Err(err) = execute_due_task(
                                    &config,
                                    &user_store,
                                    &index_store,
                                    &task_ref,
                                    &running_threads,
                                ) {
                                    error!(
                                        "scheduler task {} for user {} failed: {}",
                                        task_ref.task_id, task_ref.user_id, err
                                    );
                                }
                                let mut claims =
                                    claims.lock().unwrap_or_else(|poison| poison.into_inner());
                                claims.release(&task_ref);
                                limiter.release();
                            });
                        }
                    }
                    Err(err) => {
                        error!("index store query failed: {}", err);
                    }
                }
                thread::sleep(scheduler_poll_interval);
            }
        });
        handles.push(handle);
    }

    // Start task watchdog thread to detect and recover from stuck/crashed tasks
    {
        let claims = claims.clone();
        let scheduler_stop = scheduler_stop.clone();
        let user_store = user_store.clone();
        let users_root = config.users_root.clone();
        let task_timeout_secs = std::env::var("TASK_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_TASK_TIMEOUT_SECS);
        let watchdog_interval_ms = std::env::var("WATCHDOG_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(WATCHDOG_INTERVAL_SECS * 1000);
        let watchdog_interval = Duration::from_millis(watchdog_interval_ms);

        let handle = thread::spawn(move || {
            info!(
                "Task watchdog started (timeout={}s, check_interval={}ms)",
                task_timeout_secs, watchdog_interval_ms
            );

            while !scheduler_stop.load(Ordering::Relaxed) {
                thread::sleep(watchdog_interval);

                let stale_tasks = {
                    let claims = claims.lock().unwrap_or_else(|poison| poison.into_inner());
                    claims.find_stale_tasks(task_timeout_secs)
                };

                for stale_claim in stale_tasks {
                    warn!(
                        "Watchdog detected stale task: task_id={} user_id={} thread_id={:?} started_at={} retry_count={}",
                        stale_claim.task_id,
                        stale_claim.user_id,
                        stale_claim.thread_id,
                        stale_claim.started_at,
                        stale_claim.retry_count
                    );

                    // Force release the stale task from claims
                    let released = {
                        let mut claims = claims.lock().unwrap_or_else(|poison| poison.into_inner());
                        claims.force_release(&stale_claim.task_id)
                    };

                    if released.is_some() {
                        // Load scheduler to manage retry count
                        let user_paths = user_store.user_paths(&users_root, &stale_claim.user_id);
                        let scheduler_result =
                            Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default());

                        match scheduler_result {
                            Ok(mut scheduler) => {
                                // Increment retry count in database
                                match scheduler.increment_retry_count(&stale_claim.task_id) {
                                    Ok(new_count) => {
                                        if new_count < MAX_TASK_RETRIES {
                                            warn!(
                                                "Watchdog released stale task {} (will be retried, attempt {}/{})",
                                                stale_claim.task_id,
                                                new_count,
                                                MAX_TASK_RETRIES
                                            );
                                            // Task will be re-picked up by scheduler on next tick
                                        } else {
                                            error!(
                                                "Watchdog: Task {} exceeded max retries ({}), disabling task",
                                                stale_claim.task_id, MAX_TASK_RETRIES
                                            );

                                            // Disable the task in database
                                            if let Err(err) =
                                                scheduler.disable_task_by_id(&stale_claim.task_id)
                                            {
                                                error!(
                                                    "Failed to disable task {}: {}",
                                                    stale_claim.task_id, err
                                                );
                                            }

                                            // Notify user about the failure
                                            if let Err(err) = notify_task_failure(
                                                &user_store,
                                                &users_root,
                                                &stale_claim,
                                            ) {
                                                error!(
                                                    "Failed to notify user about task failure {}: {}",
                                                    stale_claim.task_id, err
                                                );
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        error!(
                                            "Failed to increment retry count for task {}: {}",
                                            stale_claim.task_id, err
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                error!(
                                    "Watchdog failed to load scheduler for user {}: {}",
                                    stale_claim.user_id, err
                                );
                            }
                        }
                    }
                }
            }
            info!("Task watchdog stopped");
        });
        handles.push(handle);
    }

    SchedulerControl {
        stop: scheduler_stop,
        handles,
    }
}

/// Notify user that a task has failed after max retries
fn notify_task_failure(
    user_store: &UserStore,
    users_root: &Path,
    stale_claim: &TaskClaim,
) -> Result<(), BoxError> {
    let user_paths = user_store.user_paths(users_root, &stale_claim.user_id);

    // Create a failure notification file in the user's workspace root
    let notification_dir = user_paths.workspaces_root.join("_notifications");
    std::fs::create_dir_all(&notification_dir)?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let notification_file = notification_dir.join(format!("task_failure_{}.txt", timestamp));

    let notification_content = format!(
        "Task Failure Notification\n\
        ==========================\n\
        \n\
        Task ID: {}\n\
        User ID: {}\n\
        Thread ID: {:?}\n\
        Started at: {}\n\
        Failed at: {}\n\
        Retry count: {} (max: {})\n\
        \n\
        The task has been automatically disabled after exceeding the maximum retry attempts.\n\
        \n\
        Possible causes:\n\
        - The task timed out (took longer than {} seconds)\n\
        - The processing service crashed or became unresponsive\n\
        - Network or external service issues\n\
        \n\
        Recommended actions:\n\
        - Check the service logs for more details\n\
        - Try the operation again by creating a new request\n\
        - Contact support if the issue persists\n",
        stale_claim.task_id,
        stale_claim.user_id,
        stale_claim.thread_id,
        stale_claim.started_at,
        Utc::now(),
        stale_claim.retry_count,
        MAX_TASK_RETRIES,
        DEFAULT_TASK_TIMEOUT_SECS,
    );

    std::fs::write(&notification_file, &notification_content)?;
    info!(
        "Task failure notification written to: {}",
        notification_file.display()
    );

    // Log the failure for monitoring
    error!(
        "TASK_FAILURE_ALERT: task_id={} user_id={} thread_id={:?} retries={}",
        stale_claim.task_id, stale_claim.user_id, stale_claim.thread_id, stale_claim.retry_count
    );

    Ok(())
}

fn execute_due_task(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    task_ref: &TaskRef,
    running_threads: &Arc<Mutex<HashSet<String>>>,
) -> Result<(), BoxError> {
    let task_id = Uuid::parse_str(&task_ref.task_id)?;

    // Handle Discord guild-based paths differently from regular user paths
    let tasks_db_path = if task_ref.user_id.starts_with("discord:") {
        let guild_id = task_ref
            .user_id
            .strip_prefix("discord:")
            .unwrap_or(&task_ref.user_id);
        let guild_paths =
            crate::discord_gateway::DiscordGuildPaths::new(&config.workspace_root, guild_id);
        guild_paths.tasks_db_path
    } else {
        let user_paths = user_store.user_paths(&config.users_root, &task_ref.user_id);
        user_paths.tasks_db_path
    };

    let mut scheduler = Scheduler::load(&tasks_db_path, ModuleExecutor::default())?;
    let now = Utc::now();
    let summary = summarize_tasks(scheduler.tasks(), now);
    log_task_snapshot(&task_ref.user_id, "before_execute", &summary);

    let (kind_label, status_label) = scheduler
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .map(|task| (task_kind_label(&task.kind), task_status(task, now)))
        .unwrap_or(("unknown", "missing"));
    info!(
        "scheduler executing task_id={} user_id={} kind={} status={}",
        task_ref.task_id, task_ref.user_id, kind_label, status_label
    );
    let mut thread_key: Option<String> = None;
    if let Some(task) = scheduler.tasks().iter().find(|task| task.id == task_id) {
        if let TaskKind::RunTask(run) = &task.kind {
            let key = run.workspace_dir.to_string_lossy().into_owned();
            let mut running = running_threads
                .lock()
                .expect("running thread lock poisoned");
            if running.contains(&key) {
                info!(
                    "scheduler deferred run_task task_id={} user_id={} (thread busy)",
                    task_ref.task_id, task_ref.user_id
                );
                return Ok(());
            }
            running.insert(key.clone());
            thread_key = Some(key);
        }
    }

    let executed = scheduler.execute_task_by_id(task_id);

    if let Some(key) = thread_key {
        let mut running = running_threads
            .lock()
            .expect("running thread lock poisoned");
        running.remove(&key);
    }
    let executed = executed?;
    if executed {
        info!(
            "scheduler task completed task_id={} user_id={} status=success",
            task_ref.task_id, task_ref.user_id
        );

        // Reset retry count on successful execution
        if let Err(err) = scheduler.reset_retry_count(&task_ref.task_id) {
            warn!(
                "Failed to reset retry count for task {}: {}",
                task_ref.task_id, err
            );
        }

        let refreshed_scheduler = Scheduler::load(&tasks_db_path, ModuleExecutor::default());
        match refreshed_scheduler {
            Ok(refreshed_scheduler) => {
                index_store.sync_user_tasks(&task_ref.user_id, refreshed_scheduler.tasks())?;
                let summary = summarize_tasks(refreshed_scheduler.tasks(), Utc::now());
                log_task_snapshot(&task_ref.user_id, "after_execute", &summary);
                Ok(())
            }
            Err(err) => {
                if let Err(sync_err) =
                    index_store.sync_user_tasks(&task_ref.user_id, scheduler.tasks())
                {
                    warn!(
                        "scheduler sync failed after error task_id={} user_id={} error={}",
                        task_ref.task_id, task_ref.user_id, sync_err
                    );
                } else {
                    let summary = summarize_tasks(scheduler.tasks(), Utc::now());
                    log_task_snapshot(&task_ref.user_id, "after_execute_error", &summary);
                }
                Err(Box::new(err))
            }
        }
    } else {
        // Task was not executed (disabled or not due), sync index to remove stale entries
        index_store.sync_user_tasks(&task_ref.user_id, scheduler.tasks())?;
        Ok(())
    }
}

struct TaskSummary {
    total: usize,
    enabled: usize,
    due: usize,
    completed: usize,
    disabled: usize,
    lines: Vec<String>,
}

fn summarize_tasks(tasks: &[ScheduledTask], now: DateTime<Utc>) -> TaskSummary {
    let mut summary = TaskSummary {
        total: tasks.len(),
        enabled: 0,
        due: 0,
        completed: 0,
        disabled: 0,
        lines: Vec::new(),
    };

    for task in tasks {
        let due = is_task_due(task, now);
        if task.enabled {
            summary.enabled += 1;
            if due {
                summary.due += 1;
            }
        } else if task.last_run.is_some() {
            summary.completed += 1;
        } else {
            summary.disabled += 1;
        }
        summary.lines.push(format_task_line(task, now));
    }

    summary
}

fn log_task_snapshot(user_id: &str, phase: &str, summary: &TaskSummary) {
    if summary.total == 0 {
        info!(
            "scheduler task snapshot user_id={} phase={} total=0",
            user_id, phase
        );
        return;
    }
    let tasks = summary.lines.join(" | ");
    info!(
        "scheduler task snapshot user_id={} phase={} total={} enabled={} due={} completed={} disabled={} tasks=[{}]",
        user_id,
        phase,
        summary.total,
        summary.enabled,
        summary.due,
        summary.completed,
        summary.disabled,
        tasks
    );
}

fn format_task_line(task: &ScheduledTask, now: DateTime<Utc>) -> String {
    let next_run = schedule_next_run(&task.schedule).to_rfc3339();
    let last_run = format_datetime_opt(task.last_run.clone());
    format!(
        "id={} kind={} status={} next_run={} last_run={}",
        task.id,
        task_kind_label(&task.kind),
        task_status(task, now),
        next_run,
        last_run
    )
}

fn task_status(task: &ScheduledTask, now: DateTime<Utc>) -> &'static str {
    if !task.enabled {
        if task.last_run.is_some() {
            return "completed";
        }
        return "disabled";
    }
    if is_task_due(task, now) {
        "due"
    } else {
        "scheduled"
    }
}

fn is_task_due(task: &ScheduledTask, now: DateTime<Utc>) -> bool {
    match &task.schedule {
        Schedule::Cron { next_run, .. } => *next_run <= now,
        Schedule::OneShot { run_at } => *run_at <= now,
    }
}

fn schedule_next_run(schedule: &Schedule) -> DateTime<Utc> {
    match schedule {
        Schedule::Cron { next_run, .. } => next_run.clone(),
        Schedule::OneShot { run_at } => run_at.clone(),
    }
}

fn format_datetime_opt(value: Option<DateTime<Utc>>) -> String {
    value
        .map(|value| value.to_rfc3339())
        .unwrap_or_else(|| "-".to_string())
}

fn task_kind_label(kind: &TaskKind) -> &'static str {
    match kind {
        TaskKind::SendReply(_) => "send_email",
        TaskKind::RunTask(_) => "run_task",
        TaskKind::Noop => "noop",
    }
}

pub fn cancel_pending_thread_tasks<E: crate::TaskExecutor>(
    scheduler: &mut Scheduler<E>,
    workspace: &Path,
    current_epoch: u64,
) -> Result<usize, SchedulerError> {
    let thread_state_path = default_thread_state_path(workspace);
    scheduler.disable_tasks_by(|task| {
        if !task.enabled {
            return false;
        }
        match &task.kind {
            TaskKind::RunTask(run) => {
                run.workspace_dir == workspace && run.thread_epoch.unwrap_or(0) < current_epoch
            }
            TaskKind::SendReply(send) => {
                let same_thread = send
                    .thread_state_path
                    .as_ref()
                    .map(|path| path == &thread_state_path)
                    .unwrap_or_else(|| send.html_path.starts_with(workspace));
                same_thread && send.thread_epoch.unwrap_or(0) < current_epoch
            }
            _ => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::employee_config::{EmployeeDirectory, EmployeeProfile};
    use crate::index_store::IndexStore;
    use crate::service::DEFAULT_INBOUND_BODY_MAX_BYTES;
    use crate::user_store::UserStore;
    use std::collections::{HashMap, HashSet};
    use std::env;
    use std::fs;
    use std::sync::Arc;
    use std::time::Instant;
    use tempfile::TempDir;

    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let prev = env::var(key).ok();
            env::set_var(key, value);
            Self { key, prev }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(value) => env::set_var(self.key, value),
                None => env::remove_var(self.key),
            }
        }
    }

    fn build_test_config(temp: &TempDir) -> ServiceConfig {
        let workspace_root = temp.path().join("workspaces");
        let users_root = temp.path().join("users");
        let state_dir = temp.path().join("state");
        fs::create_dir_all(&workspace_root).expect("create workspaces");
        fs::create_dir_all(&users_root).expect("create users");
        fs::create_dir_all(&state_dir).expect("create state");

        let addresses = vec!["test@example.com".to_string()];
        let address_set: HashSet<String> = addresses.iter().cloned().collect();
        let employee_profile = EmployeeProfile {
            id: "test-employee".to_string(),
            display_name: None,
            runner: "local".to_string(),
            model: None,
            addresses,
            address_set: address_set.clone(),
            runtime_root: None,
            agents_path: None,
            claude_path: None,
            soul_path: None,
            skills_dir: None,
            discord_enabled: false,
            slack_enabled: false,
            bluebubbles_enabled: false,
        };
        let mut employee_by_id = HashMap::new();
        employee_by_id.insert(employee_profile.id.clone(), employee_profile.clone());
        let employee_directory = EmployeeDirectory {
            employees: vec![employee_profile.clone()],
            employee_by_id,
            default_employee_id: Some(employee_profile.id.clone()),
            service_addresses: address_set,
        };

        dotenvy::dotenv().ok();
        let ingestion_db_url =
            std::env::var("SUPABASE_DB_URL").expect("SUPABASE_DB_URL required for tests");

        ServiceConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
            employee_id: employee_profile.id.clone(),
            employee_config_path: temp.path().join("employee.toml"),
            employee_profile,
            employee_directory,
            workspace_root: workspace_root.clone(),
            scheduler_state_path: state_dir.join("tasks.db"),
            processed_ids_path: state_dir.join("postmark_processed_ids.txt"),
            ingestion_db_url,
            ingestion_poll_interval: Duration::from_millis(50),
            users_root: users_root.clone(),
            users_db_path: state_dir.join("users.db"),
            task_index_path: state_dir.join("task_index.db"),
            codex_model: "test".to_string(),
            codex_disabled: true,
            scheduler_poll_interval: Duration::from_millis(20),
            scheduler_max_concurrency: 1,
            scheduler_user_max_concurrency: 1,
            inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
            skills_source_dir: None,
            slack_bot_token: None,
            slack_bot_user_id: None,
            slack_store_path: state_dir.join("slack.db"),
            slack_client_id: None,
            slack_client_secret: None,
            slack_redirect_uri: None,
            discord_bot_token: None,
            discord_bot_user_id: None,
            google_docs_enabled: false,
            bluebubbles_url: None,
            bluebubbles_password: None,
            telegram_bot_token: None,
            whatsapp_access_token: None,
            whatsapp_phone_number_id: None,
            whatsapp_verify_token: None,
        }
    }

    #[test]
    fn stop_and_join_returns_quickly_with_short_watchdog_interval() {
        let _guard = EnvGuard::set("WATCHDOG_INTERVAL_MS", "100");
        let temp = TempDir::new().expect("tempdir");
        let config = build_test_config(&temp);
        let user_store = Arc::new(UserStore::new(&config.users_db_path).expect("user store"));
        let index_store = Arc::new(IndexStore::new(&config.task_index_path).expect("index store"));

        let start = Instant::now();
        let mut control =
            start_scheduler_threads(Arc::new(config), user_store.clone(), index_store.clone());
        control.stop_and_join();

        let elapsed = start.elapsed();
        assert!(
            elapsed < Duration::from_secs(1),
            "stop_and_join took too long: {:?}",
            elapsed
        );
    }
}
