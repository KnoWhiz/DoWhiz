use axum::extract::{DefaultBodyLimit, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use axum::Router;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use kuchiki::traits::*;
use kuchiki::NodeRef;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::adapters::bluebubbles::send_quick_bluebubbles_response;
use crate::adapters::slack::SlackEventWrapper;
use crate::employee_config::{load_employee_directory, EmployeeDirectory, EmployeeProfile};
use crate::ingestion::IngestionEnvelope;
use crate::ingestion_queue::IngestionQueue;
use crate::message_router::{MessageRouter, RouterDecision};
use crate::index_store::{IndexStore, TaskRef};
use crate::mailbox;
use crate::slack_store::{SlackInstallation, SlackStore};
// Re-export thread_state functions for use by discord_gateway
use crate::channel::Channel;
pub use crate::thread_state::{bump_thread_state, default_thread_state_path};
use crate::user_store::{extract_emails, UserStore};
use crate::{
    ModuleExecutor, RunTaskTask, Schedule, ScheduledTask, Scheduler, SchedulerError, TaskKind,
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub const DEFAULT_INBOUND_BODY_MAX_BYTES: usize = 25 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub employee_id: String,
    pub employee_config_path: PathBuf,
    pub employee_profile: EmployeeProfile,
    pub employee_directory: EmployeeDirectory,
    pub workspace_root: PathBuf,
    pub scheduler_state_path: PathBuf,
    pub processed_ids_path: PathBuf,
    pub ingestion_db_path: PathBuf,
    pub ingestion_dedupe_path: PathBuf,
    pub ingestion_poll_interval: Duration,
    pub users_root: PathBuf,
    pub users_db_path: PathBuf,
    pub task_index_path: PathBuf,
    pub codex_model: String,
    pub codex_disabled: bool,
    pub scheduler_poll_interval: Duration,
    pub scheduler_max_concurrency: usize,
    pub scheduler_user_max_concurrency: usize,
    pub inbound_body_max_bytes: usize,
    pub skills_source_dir: Option<PathBuf>,
    /// Slack bot OAuth token for sending messages (legacy single-workspace)
    pub slack_bot_token: Option<String>,
    /// Slack bot user ID for filtering out bot's own messages (legacy single-workspace)
    pub slack_bot_user_id: Option<String>,
    /// Path to slack installations database
    pub slack_store_path: PathBuf,
    /// Slack OAuth client ID (for multi-workspace support)
    pub slack_client_id: Option<String>,
    /// Slack OAuth client secret (for multi-workspace support)
    pub slack_client_secret: Option<String>,
    /// Slack OAuth redirect URI
    pub slack_redirect_uri: Option<String>,
    /// Discord bot token
    pub discord_bot_token: Option<String>,
    /// Discord bot application ID (for filtering out bot's own messages)
    pub discord_bot_user_id: Option<u64>,
    /// Google Docs polling enabled
    pub google_docs_enabled: bool,
    /// BlueBubbles server URL (e.g., http://localhost:1234)
    pub bluebubbles_url: Option<String>,
    /// BlueBubbles server password
    pub bluebubbles_password: Option<String>,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, BoxError> {
        dotenvy::dotenv().ok();

        let host = env::var("RUST_SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("RUST_SERVICE_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(9001);

        let employee_config_path =
            resolve_path(env::var("EMPLOYEE_CONFIG_PATH").unwrap_or_else(|_| {
                default_employee_config_path()
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let employee_directory = load_employee_directory(&employee_config_path)?;
        let employee_id = env::var("EMPLOYEE_ID")
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .or_else(|| employee_directory.default_employee_id.clone())
            .or_else(|| {
                employee_directory
                    .employees
                    .first()
                    .map(|emp| emp.id.clone())
            })
            .ok_or_else(|| "employee config has no employees".to_string())?;
        let employee_profile = employee_directory
            .employee(&employee_id)
            .ok_or_else(|| {
                format!(
                    "employee '{}' not found in {}",
                    employee_id,
                    employee_config_path.display()
                )
            })?
            .clone();

        let runtime_root = default_runtime_root()?;
        let employee_runtime_root = employee_profile
            .runtime_root
            .clone()
            .unwrap_or_else(|| runtime_root.join(&employee_id));
        let workspace_root = resolve_path(env::var("WORKSPACE_ROOT").unwrap_or_else(|_| {
            employee_runtime_root
                .join("workspaces")
                .to_string_lossy()
                .into_owned()
        }))?;
        let scheduler_state_path =
            resolve_path(env::var("SCHEDULER_STATE_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("tasks.db")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let processed_ids_path =
            resolve_path(env::var("PROCESSED_IDS_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("postmark_processed_ids.txt")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let ingestion_db_path =
            resolve_path(env::var("INGESTION_DB_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("ingestion.db")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let ingestion_dedupe_path =
            resolve_path(env::var("INGESTION_DEDUPE_PATH").unwrap_or_else(|_| {
                employee_runtime_root
                    .join("state")
                    .join("ingestion_processed_ids.txt")
                    .to_string_lossy()
                    .into_owned()
            }))?;
        let users_root = resolve_path(env::var("USERS_ROOT").unwrap_or_else(|_| {
            employee_runtime_root
                .join("users")
                .to_string_lossy()
                .into_owned()
        }))?;
        let users_db_path = resolve_path(env::var("USERS_DB_PATH").unwrap_or_else(|_| {
            employee_runtime_root
                .join("state")
                .join("users.db")
                .to_string_lossy()
                .into_owned()
        }))?;
        let task_index_path = resolve_path(env::var("TASK_INDEX_PATH").unwrap_or_else(|_| {
            employee_runtime_root
                .join("state")
                .join("task_index.db")
                .to_string_lossy()
                .into_owned()
        }))?;
        let codex_model = env::var("CODEX_MODEL").unwrap_or_else(|_| "gpt-5.2-codex".to_string());
        let codex_disabled = env_flag("CODEX_DISABLED", false);
        let scheduler_poll_interval = env::var("SCHEDULER_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(1));
        let scheduler_max_concurrency = env::var("SCHEDULER_MAX_CONCURRENCY")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(10);
        let scheduler_user_max_concurrency = env::var("SCHEDULER_USER_MAX_CONCURRENCY")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(3);
        let ingestion_poll_interval = env::var("INGESTION_POLL_INTERVAL_SECS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
            .unwrap_or_else(|| Duration::from_secs(1));
        let inbound_body_max_bytes = env::var("POSTMARK_INBOUND_MAX_BYTES")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(DEFAULT_INBOUND_BODY_MAX_BYTES);
        let skills_source_dir = Some(repo_skills_source_dir());

        // Slack configuration
        let slack_bot_token = env::var("SLACK_BOT_TOKEN").ok().filter(|s| !s.is_empty());
        let slack_bot_user_id = env::var("SLACK_BOT_USER_ID").ok().filter(|s| !s.is_empty());
        let slack_store_path = resolve_path(env::var("SLACK_STORE_PATH").unwrap_or_else(|_| {
            employee_runtime_root
                .join("state")
                .join("slack.db")
                .to_string_lossy()
                .into_owned()
        }))?;
        let slack_client_id = env::var("SLACK_CLIENT_ID").ok().filter(|s| !s.is_empty());
        let slack_client_secret = env::var("SLACK_CLIENT_SECRET")
            .ok()
            .filter(|s| !s.is_empty());
        let slack_redirect_uri = env::var("SLACK_REDIRECT_URI")
            .ok()
            .filter(|s| !s.is_empty());

        // Discord configuration
        let discord_bot_token = env::var("DISCORD_BOT_TOKEN").ok().filter(|s| !s.is_empty());
        let discord_bot_user_id = env::var("DISCORD_BOT_USER_ID")
            .ok()
            .and_then(|s| s.parse::<u64>().ok());

        // Google Docs configuration
        let google_docs_enabled = env::var("GOOGLE_DOCS_ENABLED")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        // BlueBubbles configuration
        let bluebubbles_url = env::var("BLUEBUBBLES_URL").ok().filter(|s| !s.is_empty());
        let bluebubbles_password = env::var("BLUEBUBBLES_PASSWORD")
            .ok()
            .filter(|s| !s.is_empty());

        Ok(Self {
            host,
            port,
            employee_id,
            employee_config_path,
            employee_profile,
            employee_directory,
            workspace_root,
            scheduler_state_path,
            processed_ids_path,
            ingestion_db_path,
            ingestion_dedupe_path,
            ingestion_poll_interval,
            users_root,
            users_db_path,
            task_index_path,
            codex_model,
            codex_disabled,
            scheduler_poll_interval,
            scheduler_max_concurrency,
            scheduler_user_max_concurrency,
            inbound_body_max_bytes,
            skills_source_dir,
            slack_bot_token,
            slack_bot_user_id,
            slack_store_path,
            slack_client_id,
            slack_client_secret,
            slack_redirect_uri,
            discord_bot_token,
            discord_bot_user_id,
            google_docs_enabled,
            bluebubbles_url,
            bluebubbles_password,
        })
    }
}

#[derive(Clone)]
struct AppState {
    config: Arc<ServiceConfig>,
    slack_store: Arc<SlackStore>,
}

/// Information about a running task for monitoring
#[derive(Clone, Debug)]
struct TaskClaim {
    task_id: String,
    user_id: String,
    started_at: DateTime<Utc>,
    thread_id: Option<String>,
    retry_count: u32,
}

/// Default task timeout in seconds (10 minutes)
const DEFAULT_TASK_TIMEOUT_SECS: u64 = 600;
/// Maximum number of retries before giving up
const MAX_TASK_RETRIES: u32 = 3;
/// Watchdog check interval in seconds
const WATCHDOG_INTERVAL_SECS: u64 = 30;

#[derive(Default)]
struct SchedulerClaims {
    running_tasks: HashMap<String, TaskClaim>,
    running_users: HashMap<String, usize>,
}

enum ClaimResult {
    Claimed,
    UserBusy,
    TaskBusy,
}

impl SchedulerClaims {
    fn try_claim(&mut self, task_ref: &TaskRef, user_limit: usize, retry_count: u32) -> ClaimResult {
        let active = self
            .running_users
            .get(&task_ref.user_id)
            .copied()
            .unwrap_or(0);
        if active >= user_limit {
            return ClaimResult::UserBusy;
        }
        if self.running_tasks.contains_key(&task_ref.task_id) {
            return ClaimResult::TaskBusy;
        }
        self.running_users
            .insert(task_ref.user_id.clone(), active + 1);
        let claim = TaskClaim {
            task_id: task_ref.task_id.clone(),
            user_id: task_ref.user_id.clone(),
            started_at: Utc::now(),
            thread_id: None, // TaskRef doesn't have thread_id; it's tracked in the task itself
            retry_count,
        };
        self.running_tasks.insert(task_ref.task_id.clone(), claim);
        ClaimResult::Claimed
    }

    fn release(&mut self, task_ref: &TaskRef) {
        if let Some(active) = self.running_users.get_mut(&task_ref.user_id) {
            if *active <= 1 {
                self.running_users.remove(&task_ref.user_id);
            } else {
                *active -= 1;
            }
        }
        self.running_tasks.remove(&task_ref.task_id);
    }

    /// Find tasks that have been running longer than the timeout
    fn find_stale_tasks(&self, timeout_secs: u64) -> Vec<TaskClaim> {
        let now = Utc::now();
        let timeout = chrono::Duration::seconds(timeout_secs as i64);
        self.running_tasks
            .values()
            .filter(|claim| now - claim.started_at > timeout)
            .cloned()
            .collect()
    }

    /// Force release a task by task_id (used by watchdog)
    fn force_release(&mut self, task_id: &str) -> Option<TaskClaim> {
        if let Some(claim) = self.running_tasks.remove(task_id) {
            if let Some(active) = self.running_users.get_mut(&claim.user_id) {
                if *active <= 1 {
                    self.running_users.remove(&claim.user_id);
                } else {
                    *active -= 1;
                }
            }
            Some(claim)
        } else {
            None
        }
    }
}

struct ConcurrencyLimiter {
    max: usize,
    in_flight: Mutex<usize>,
}

impl ConcurrencyLimiter {
    fn new(max: usize) -> Self {
        Self {
            max,
            in_flight: Mutex::new(0),
        }
    }

    fn try_acquire(&self) -> bool {
        let mut in_flight = self
            .in_flight
            .lock()
            .expect("concurrency limiter lock poisoned");
        if *in_flight >= self.max {
            return false;
        }
        *in_flight += 1;
        true
    }

    fn release(&self) {
        let mut in_flight = self
            .in_flight
            .lock()
            .expect("concurrency limiter lock poisoned");
        if *in_flight > 0 {
            *in_flight -= 1;
        }
    }
}

pub async fn run_server(
    config: ServiceConfig,
    shutdown: impl std::future::Future<Output = ()> + Send + 'static,
) -> Result<(), BoxError> {
    // Export SLACK_STORE_PATH so execute_slack_send can find the OAuth tokens
    env::set_var("SLACK_STORE_PATH", &config.slack_store_path);
    let config = Arc::new(config);
    let user_store = Arc::new(UserStore::new(&config.users_db_path)?);
    let index_store = Arc::new(IndexStore::new(&config.task_index_path)?);
    let slack_store = Arc::new(SlackStore::new(&config.slack_store_path)?);
    let ingestion_queue = Arc::new(IngestionQueue::new(&config.ingestion_db_path)?);
    let message_router = Arc::new(MessageRouter::new());
    if let Ok(user_ids) = user_store.list_user_ids() {
        for user_id in user_ids {
            let paths = user_store.user_paths(&config.users_root, &user_id);
            let scheduler = Scheduler::load(&paths.tasks_db_path, ModuleExecutor::default());
            match scheduler {
                Ok(scheduler) => {
                    if let Err(err) = index_store.sync_user_tasks(&user_id, scheduler.tasks()) {
                        error!("index bootstrap failed for {}: {}", user_id, err);
                    }
                }
                Err(err) => {
                    error!("scheduler bootstrap failed for {}: {}", user_id, err);
                }
            }
        }
    }
    let scheduler_stop = Arc::new(AtomicBool::new(false));
    let scheduler_poll_interval = config.scheduler_poll_interval;
    let scheduler_max_concurrency = config.scheduler_max_concurrency;
    let scheduler_user_max_concurrency = config.scheduler_user_max_concurrency;
    let claims = Arc::new(Mutex::new(SchedulerClaims::default()));
    let running_threads = Arc::new(Mutex::new(HashSet::new()));
    let limiter = Arc::new(ConcurrencyLimiter::new(scheduler_max_concurrency));
    {
        let config = config.clone();
        let user_store = user_store.clone();
        let index_store = index_store.clone();
        let scheduler_stop = scheduler_stop.clone();
        let claims = claims.clone();
        let running_threads = running_threads.clone();
        let limiter = limiter.clone();
        let query_limit = scheduler_max_concurrency.saturating_mul(4).max(1);
        thread::spawn(move || {
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

        thread::spawn(move || {
            info!(
                "Task watchdog started (timeout={}s, check_interval={}s)",
                task_timeout_secs, WATCHDOG_INTERVAL_SECS
            );

            while !scheduler_stop.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(WATCHDOG_INTERVAL_SECS));

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
    }
    info!(
        "Inbound webhooks are handled by the ingestion gateway; worker {} will only consume queued messages",
        config.employee_id
    );

    spawn_ingestion_consumer(
        config.clone(),
        ingestion_queue.clone(),
        user_store.clone(),
        index_store.clone(),
        slack_store.clone(),
        message_router.clone(),
    )?;

    let state = AppState {
        config: config.clone(),
        slack_store,
    };

    let host: IpAddr = config
        .host
        .parse()
        .map_err(|_| format!("invalid host: {}", config.host))?;
    let addr = SocketAddr::new(host, config.port);
    info!("Rust email service listening on {}", addr);

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/slack/install", get(slack_install))
        .route("/slack/oauth/callback", get(slack_oauth_callback))
        .with_state(state)
        .layer(DefaultBodyLimit::max(config.inbound_body_max_bytes));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;
    scheduler_stop.store(true, Ordering::Relaxed);
    Ok(())
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
    fs::create_dir_all(&notification_dir)?;

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

    fs::write(&notification_file, &notification_content)?;
    info!(
        "Task failure notification written to: {}",
        notification_file.display()
    );

    // Log the failure for monitoring
    error!(
        "TASK_FAILURE_ALERT: task_id={} user_id={} thread_id={:?} retries={}",
        stale_claim.task_id,
        stale_claim.user_id,
        stale_claim.thread_id,
        stale_claim.retry_count
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

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

fn spawn_ingestion_consumer(
    config: Arc<ServiceConfig>,
    queue: Arc<IngestionQueue>,
    user_store: Arc<UserStore>,
    index_store: Arc<IndexStore>,
    slack_store: Arc<SlackStore>,
    message_router: Arc<MessageRouter>,
) -> Result<(), BoxError> {
    let poll_interval = config.ingestion_poll_interval;
    let employee_id = config.employee_id.clone();
    let dedupe_path = config.ingestion_dedupe_path.clone();
    let runtime = tokio::runtime::Handle::current();

    thread::spawn(move || {
        let mut dedupe_store = match ProcessedMessageStore::load(&dedupe_path) {
            Ok(store) => store,
            Err(err) => {
                error!("ingestion dedupe store load failed: {}", err);
                return;
            }
        };

        loop {
            match queue.claim_next(&employee_id) {
                Ok(Some(item)) => {
                    let is_new = match dedupe_store.mark_if_new(&[item.envelope.dedupe_key.clone()])
                    {
                        Ok(value) => value,
                        Err(err) => {
                            error!("ingestion dedupe store error: {}", err);
                            true
                        }
                    };

                    if !is_new {
                        if let Err(err) = queue.mark_done(&item.id) {
                            warn!("failed to mark duplicate envelope done: {}", err);
                        }
                        continue;
                    }

                    match process_ingestion_envelope(
                        &config,
                        &user_store,
                        &index_store,
                        &slack_store,
                        &message_router,
                        &runtime,
                        &item.envelope,
                    ) {
                        Ok(_) => {
                            if let Err(err) = queue.mark_done(&item.id) {
                                warn!("failed to mark envelope done: {}", err);
                            }
                        }
                        Err(err) => {
                            if let Err(mark_err) =
                                queue.mark_failed(&item.id, &err.to_string())
                            {
                                warn!("failed to mark envelope failed: {}", mark_err);
                            }
                        }
                    }
                }
                Ok(None) => {
                    thread::sleep(poll_interval);
                }
                Err(err) => {
                    warn!("ingestion queue claim error: {}", err);
                    thread::sleep(poll_interval);
                }
            }
        }
    });

    Ok(())
}

fn process_ingestion_envelope(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    slack_store: &SlackStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    envelope: &IngestionEnvelope,
) -> Result<(), BoxError> {
    match envelope.channel {
        Channel::Email => {
            let raw_payload = envelope.raw_payload_bytes();
            let payload: PostmarkInbound = serde_json::from_slice(&raw_payload)?;
            process_inbound_payload(config, user_store, index_store, &payload, &raw_payload)
        }
        Channel::Slack => {
            let message = envelope.to_inbound_message();
            if try_quick_response_slack(
                config,
                user_store,
                slack_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            if raw_payload.is_empty() {
                return Err("missing slack raw payload".into());
            }
            process_slack_event(config, user_store, index_store, slack_store, &raw_payload)
        }
        Channel::BlueBubbles => {
            let message = envelope.to_inbound_message();
            if try_quick_response_bluebubbles(
                config,
                user_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            if raw_payload.is_empty() {
                return Err("missing bluebubbles raw payload".into());
            }
            process_bluebubbles_event(config, user_store, index_store, &raw_payload)
        }
        Channel::Discord => {
            let message = envelope.to_inbound_message();
            if try_quick_response_discord(
                config,
                user_store,
                message_router,
                runtime,
                &message,
            )? {
                return Ok(());
            }
            let raw_payload = envelope.raw_payload_bytes();
            process_discord_inbound_message(config, index_store, &message, &raw_payload)
        }
        Channel::Sms => {
            let message = envelope.to_inbound_message();
            let raw_payload = envelope.raw_payload_bytes();
            process_sms_message(config, user_store, index_store, &message, &raw_payload)
        }
        Channel::GoogleDocs => {
            let message = envelope.to_inbound_message();
            let raw_payload = envelope.raw_payload_bytes();
            process_google_docs_message(config, user_store, index_store, &message, &raw_payload)
        }
        Channel::Telegram => Ok(()),
    }
}

/// Read memo.md from a user's memory directory
fn read_user_memo(memory_dir: &Path) -> Option<String> {
    let memo_path = memory_dir.join("memo.md");
    fs::read_to_string(&memo_path).ok()
}

/// Append memory update to a user's memo.md
fn append_user_memo(memory_dir: &Path, update: &str) -> io::Result<()> {
    fs::create_dir_all(memory_dir)?;
    let memo_path = memory_dir.join("memo.md");
    let existing = fs::read_to_string(&memo_path).unwrap_or_default();
    let new_content = if existing.trim().is_empty() {
        format!("# Memo\n\n{}\n", update.trim())
    } else {
        format!("{}\n\n{}\n", existing.trim_end(), update.trim())
    };
    fs::write(&memo_path, new_content)
}

fn try_quick_response_slack(
    config: &ServiceConfig,
    user_store: &UserStore,
    slack_store: &SlackStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let channel_id = match message.metadata.slack_channel_id.as_deref() {
        Some(value) => value,
        None => return Ok(false),
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("slack", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let cleaned_text = text
        .split_whitespace()
        .filter(|word| !(word.starts_with("<@") && word.ends_with(">")))
        .collect::<Vec<_>>()
        .join(" ");

    let decision = runtime.block_on(message_router.classify(&cleaned_text, memory.as_deref()));
    match decision {
        RouterDecision::Simple { response, memory_update } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            let token = resolve_slack_bot_token(config, slack_store, message.metadata.slack_team_id.as_deref());
            if let Some(token) = token {
                let thread_ts = Some(message.thread_id.as_str());
                if runtime
                    .block_on(send_quick_slack_response(&token, channel_id, thread_ts, &response))
                    .is_ok()
                {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

fn resolve_slack_bot_token(
    config: &ServiceConfig,
    slack_store: &SlackStore,
    team_id: Option<&str>,
) -> Option<String> {
    if let Some(team_id) = team_id {
        if let Ok(installation) = slack_store.get_installation_or_env(team_id) {
            if !installation.bot_token.trim().is_empty() {
                return Some(installation.bot_token);
            }
        }
    }
    config.slack_bot_token.clone()
}

fn try_quick_response_bluebubbles(
    config: &ServiceConfig,
    user_store: &UserStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let Some(chat_guid) = message.metadata.bluebubbles_chat_guid.as_deref() else {
        return Ok(false);
    };
    let Some(url) = config.bluebubbles_url.as_deref() else {
        return Ok(false);
    };
    let Some(password) = config.bluebubbles_password.as_deref() else {
        return Ok(false);
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("phone", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let decision = runtime.block_on(message_router.classify(text, memory.as_deref()));
    match decision {
        RouterDecision::Simple { response, memory_update } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            if runtime
                .block_on(send_quick_bluebubbles_response(url, password, chat_guid, &response))
                .is_ok()
            {
                return Ok(true);
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

fn try_quick_response_discord(
    config: &ServiceConfig,
    user_store: &UserStore,
    message_router: &MessageRouter,
    runtime: &tokio::runtime::Handle,
    message: &crate::channel::InboundMessage,
) -> Result<bool, BoxError> {
    let Some(text) = message.text_body.as_deref() else {
        return Ok(false);
    };
    let channel_id = match message.metadata.discord_channel_id {
        Some(value) => value,
        None => return Ok(false),
    };
    let message_id = message.message_id.as_deref();
    let token = match config.discord_bot_token.as_deref() {
        Some(token) => token,
        None => return Ok(false),
    };

    // Look up user and get memory
    let user = user_store.get_or_create_user("discord", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    let memory = read_user_memo(&user_paths.memory_dir);

    let decision = runtime.block_on(message_router.classify(text, memory.as_deref()));
    match decision {
        RouterDecision::Simple { response, memory_update } => {
            // Write memory update if present
            if let Some(update) = memory_update {
                if let Err(e) = append_user_memo(&user_paths.memory_dir, &update) {
                    warn!("Failed to write memory update: {}", e);
                } else {
                    info!("Updated memory for user {}", user.user_id);
                }
            }

            if send_quick_discord_response_simple(token, channel_id, message_id, &response).is_ok()
            {
                return Ok(true);
            }
            Ok(false)
        }
        RouterDecision::Complex | RouterDecision::Passthrough => Ok(false),
    }
}

/// Redirect to Slack OAuth authorization page.
/// GET /slack/install
async fn slack_install(State(state): State<AppState>) -> impl IntoResponse {
    let client_id = match &state.config.slack_client_id {
        Some(id) => id.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Slack OAuth not configured (missing SLACK_CLIENT_ID)",
            )
                .into_response();
        }
    };

    let redirect_uri = state.config.slack_redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "http://localhost:{}/slack/oauth/callback",
            state.config.port
        )
    });

    let scopes = "chat:write,channels:history,groups:history,im:history,mpim:history";

    let auth_url = format!(
        "https://slack.com/oauth/v2/authorize?client_id={}&scope={}&redirect_uri={}",
        urlencoding::encode(&client_id),
        urlencoding::encode(scopes),
        urlencoding::encode(&redirect_uri)
    );

    Redirect::temporary(&auth_url).into_response()
}

/// Query parameters for OAuth callback.
#[derive(Debug, Deserialize)]
struct SlackOAuthCallbackParams {
    code: Option<String>,
    error: Option<String>,
}

/// Handle Slack OAuth callback.
/// GET /slack/oauth/callback?code=...
async fn slack_oauth_callback(
    State(state): State<AppState>,
    Query(params): Query<SlackOAuthCallbackParams>,
) -> impl IntoResponse {
    // Check for OAuth errors
    if let Some(error) = params.error {
        return (
            StatusCode::BAD_REQUEST,
            format!("Slack OAuth error: {}", error),
        )
            .into_response();
    }

    let code = match params.code {
        Some(c) => c,
        None => {
            return (StatusCode::BAD_REQUEST, "Missing OAuth code").into_response();
        }
    };

    let client_id = match &state.config.slack_client_id {
        Some(id) => id.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "SLACK_CLIENT_ID not configured",
            )
                .into_response();
        }
    };

    let client_secret = match &state.config.slack_client_secret {
        Some(secret) => secret.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "SLACK_CLIENT_SECRET not configured",
            )
                .into_response();
        }
    };

    let redirect_uri = state.config.slack_redirect_uri.clone().unwrap_or_else(|| {
        format!(
            "http://localhost:{}/slack/oauth/callback",
            state.config.port
        )
    });

    // Exchange code for token
    let client = reqwest::Client::new();
    let token_response = match client
        .post("https://slack.com/api/oauth.v2.access")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            error!("Slack OAuth token exchange failed: {}", e);
            return (StatusCode::BAD_GATEWAY, "Failed to contact Slack API").into_response();
        }
    };

    let token_json: serde_json::Value = match token_response.json().await {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse Slack OAuth response: {}", e);
            return (StatusCode::BAD_GATEWAY, "Invalid response from Slack").into_response();
        }
    };

    // Check for API errors
    if token_json.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        let error_msg = token_json
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        error!("Slack OAuth error: {}", error_msg);
        return (
            StatusCode::BAD_REQUEST,
            format!("Slack API error: {}", error_msg),
        )
            .into_response();
    }

    // Extract installation details
    let team_id = token_json
        .get("team")
        .and_then(|t| t.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let team_name = token_json
        .get("team")
        .and_then(|t| t.get("name"))
        .and_then(|v| v.as_str());
    let bot_token = token_json
        .get("access_token")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let bot_user_id = token_json
        .get("bot_user_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if team_id.is_empty() || bot_token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            "Missing team_id or access_token in Slack response",
        )
            .into_response();
    }

    // Save installation
    let installation = SlackInstallation {
        team_id: team_id.to_string(),
        team_name: team_name.map(|s| s.to_string()),
        bot_token: bot_token.to_string(),
        bot_user_id: bot_user_id.to_string(),
        installed_at: Utc::now(),
    };

    if let Err(e) = state.slack_store.upsert_installation(&installation) {
        error!("Failed to save Slack installation: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save installation",
        )
            .into_response();
    }

    info!(
        "Slack app installed for team {} ({})",
        team_id,
        team_name.unwrap_or("unknown")
    );

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8" />
  <meta
    name="description"
    content="DoWhiz Slack integration is installed. Confirm your workspace, learn next steps, and start chatting with digital employees right away."
  />
  <title>DoWhiz Slack Integration | Install Complete and Next Steps</title>
</head>
<body style="font-family: sans-serif; text-align: center; padding: 50px;">
    <h1>Installation Complete!</h1>
    <p>DoWhiz has been successfully installed to <strong>{}</strong>.</p>
    <p>You can now close this window and start chatting with the bot in Slack.</p>
</body>
</html>"#,
        team_name.unwrap_or(team_id)
    );

    (StatusCode::OK, axum::response::Html(html)).into_response()
}

fn process_slack_event(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    slack_store: &SlackStore,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::adapters::slack::SlackInboundAdapter;
    use crate::channel::InboundAdapter;

    info!("processing slack event");

    // Parse wrapper first to get team_id
    let wrapper: SlackEventWrapper = serde_json::from_slice(raw_payload)?;

    // Look up bot_user_id from SlackStore (with fallback to env var)
    let team_id = wrapper.team_id.as_deref().unwrap_or("");
    let mut bot_user_ids = HashSet::new();
    if let Ok(installation) = slack_store.get_installation_or_env(team_id) {
        if !installation.bot_user_id.is_empty() {
            bot_user_ids.insert(installation.bot_user_id);
        }
    } else if let Some(ref bot_id) = config.slack_bot_user_id {
        // Legacy fallback
        bot_user_ids.insert(bot_id.clone());
    }
    let adapter = SlackInboundAdapter::new(bot_user_ids);

    // Check if this is a bot message (should be ignored)
    if let Some(ref event) = wrapper.event {
        if adapter.is_bot_message(event) {
            info!("ignoring bot message from user {:?}", event.user);
            return Ok(());
        }
    }

    let message = adapter.parse(raw_payload)?;

    info!(
        "slack message from {} in channel {:?}: {:?}",
        message.sender, message.metadata.slack_channel_id, message.text_body
    );

    // Get channel ID (required for Slack)
    let channel_id = message
        .metadata
        .slack_channel_id
        .as_ref()
        .ok_or("missing slack_channel_id")?;

    let user = user_store.get_or_create_user("slack", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Thread key: channel_id + thread_id for grouping conversations
    let thread_key = format!("slack:{}:{}", channel_id, message.thread_id);

    // Create/get workspace for this thread
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    // Bump thread state
    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    // Save the incoming Slack message to workspace
    append_slack_message(
        &workspace,
        &message,
        raw_payload,
        thread_state.last_email_seq,
    )?;

    // Determine model and runner
    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config
                .employee_profile
                .runner
                .eq_ignore_ascii_case("claude")
            {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };

    info!(
        "workspace ready at {} for user {} thread={} epoch={}",
        workspace.display(),
        user.user_id,
        thread_key,
        thread_state.epoch
    );

    // Create RunTask to process the message
    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![channel_id.clone()], // Reply to the same channel
        reply_from: None,                   // Slack uses bot token, not a "from" address
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Slack,
        slack_team_id: message.metadata.slack_team_id.clone(),
        employee_id: Some(config.employee_profile.id.clone()),
    };

    // Schedule the task
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

fn process_bluebubbles_event(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::adapters::bluebubbles::BlueBubblesInboundAdapter;
    use crate::channel::InboundAdapter;

    info!("processing BlueBubbles event");

    let adapter = BlueBubblesInboundAdapter::new();
    let message = adapter.parse(raw_payload)?;

    info!(
        "iMessage from {} in chat {:?}: {:?}",
        message.sender, message.metadata.bluebubbles_chat_guid, message.text_body
    );

    // Get chat GUID (required for BlueBubbles)
    let chat_guid = message
        .metadata
        .bluebubbles_chat_guid
        .as_ref()
        .ok_or("missing bluebubbles_chat_guid")?;

    let user = user_store.get_or_create_user("phone", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    // Thread key: chat_guid for grouping conversations
    let thread_key = format!("imessage:{}", chat_guid);

    // Create/get workspace for this thread
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    // Bump thread state
    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    // Save the incoming BlueBubbles message to workspace
    append_bluebubbles_message(
        &workspace,
        &message,
        raw_payload,
        thread_state.last_email_seq.try_into().unwrap_or(u32::MAX),
    )?;

    // Determine model and runner
    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config
                .employee_profile
                .runner
                .eq_ignore_ascii_case("claude")
            {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };

    info!(
        "workspace ready at {} for user {} thread={} epoch={}",
        workspace.display(),
        user.user_id,
        thread_key,
        thread_state.epoch
    );

    // Create RunTask to process the message
    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![chat_guid.clone()],
        reply_from: None,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::BlueBubbles,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    // Schedule the task
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

/// Append a BlueBubbles message to the workspace inbox.
fn append_bluebubbles_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u32,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;

    // Save raw payload for debugging/archival
    let raw_path = incoming_dir.join(format!("{:05}_bluebubbles_raw.json", seq));
    fs::write(&raw_path, raw_payload)?;

    // Save message text as a simple text file
    let text_path = incoming_dir.join(format!("{:05}_bluebubbles_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    fs::write(&text_path, &text_content)?;

    // Create a metadata file with sender info
    let meta_path = incoming_dir.join(format!("{:05}_bluebubbles_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "bluebubbles",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "chat_guid": message.metadata.bluebubbles_chat_guid,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved BlueBubbles message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

fn process_discord_inbound_message(
    config: &ServiceConfig,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::discord_gateway::DiscordGuildPaths;

    let channel_id = message
        .metadata
        .discord_channel_id
        .ok_or("missing discord_channel_id")?;
    let guild_id = message
        .metadata
        .discord_guild_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "dm".to_string());

    let guild_paths = DiscordGuildPaths::new(&config.workspace_root, &guild_id);
    guild_paths.ensure_dirs()?;

    let thread_key = format!("discord:{}:{}:{}", guild_id, channel_id, message.thread_id);

    let workspace = ensure_discord_workspace(
        &guild_paths,
        channel_id,
        &message.thread_id,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    append_discord_message_payload(&workspace, message, raw_payload, thread_state.last_email_seq)?;

    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config
                .employee_profile
                .runner
                .eq_ignore_ascii_case("claude")
            {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![channel_id.to_string()],
        reply_from: None,
        archive_root: None,
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Discord,
        slack_team_id: None,
        employee_id: Some(config.employee_id.clone()),
    };

    let mut scheduler = Scheduler::load(&guild_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;

    let synthetic_user_id = DiscordGuildPaths::user_id(&guild_id);
    index_store.sync_user_tasks(&synthetic_user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued guild={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        guild_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

fn ensure_discord_workspace(
    guild_paths: &crate::discord_gateway::DiscordGuildPaths,
    channel_id: u64,
    thread_id: &str,
    employee_profile: &EmployeeProfile,
    skills_source_dir: Option<&Path>,
) -> Result<PathBuf, BoxError> {
    let thread_hash = &md5::compute(thread_id.as_bytes())
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()[..8];

    let workspace = guild_paths
        .workspaces_root
        .join(channel_id.to_string())
        .join(thread_hash);

    if !workspace.exists() {
        fs::create_dir_all(&workspace)?;
        fs::create_dir_all(workspace.join("incoming_email"))?;
        fs::create_dir_all(workspace.join("incoming_attachments"))?;
        fs::create_dir_all(workspace.join("memory"))?;
        fs::create_dir_all(workspace.join("references"))?;

        ensure_workspace_employee_files(&workspace, employee_profile)?;

        let agents_skills_dir = workspace.join(".agents").join("skills");
        if let Some(skills_src) = skills_source_dir {
            if let Err(err) = copy_skills_directory(skills_src, &agents_skills_dir) {
                error!("failed to copy base skills to workspace: {}", err);
            }
        }
        if let Some(employee_skills) = employee_profile.skills_dir.as_deref() {
            let should_copy = skills_source_dir
                .map(|base| base != employee_skills)
                .unwrap_or(true);
            if should_copy {
                if let Err(err) = copy_skills_directory(employee_skills, &agents_skills_dir) {
                    error!("failed to copy employee skills to workspace: {}", err);
                }
            }
        }

        info!("created Discord workspace at {}", workspace.display());
    }

    Ok(workspace)
}

fn append_discord_message_payload(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;

    let raw_path = incoming_dir.join(format!("{:05}_discord_raw.json", seq));
    fs::write(&raw_path, raw_payload)?;

    let text_path = incoming_dir.join(format!("{:05}_discord_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    fs::write(&text_path, &text_content)?;

    let meta_path = incoming_dir.join(format!("{:05}_discord_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "discord",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "guild_id": message.metadata.discord_guild_id,
        "channel_id": message.metadata.discord_channel_id,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved Discord message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

fn process_sms_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let normalized_from = normalize_phone_number(&message.sender);
    let user = user_store.get_or_create_user("phone", &message.sender)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!(
        "sms:{}:{}",
        normalize_phone_number(&message.recipient),
        normalized_from
    );

    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state =
        bump_thread_state(&thread_state_path, &thread_key, message.message_id.clone())?;

    append_sms_message(&workspace, message, raw_payload, thread_state.last_email_seq)?;

    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config
                .employee_profile
                .runner
                .eq_ignore_ascii_case("claude")
            {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };

    let reply_from = message
        .metadata
        .sms_to
        .clone()
        .or_else(|| Some(message.recipient.clone()));

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![message.sender.clone()],
        reply_from,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Sms,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

fn append_sms_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;

    let raw_path = incoming_dir.join(format!("{:05}_sms_raw.txt", seq));
    fs::write(&raw_path, raw_payload)?;

    let text_path = incoming_dir.join(format!("{:05}_sms_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    fs::write(&text_path, &text_content)?;

    let meta_path = incoming_dir.join(format!("{:05}_sms_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "sms",
        "sender": message.sender,
        "recipient": message.recipient,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved SMS message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

fn normalize_phone_number(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '+')
        .collect()
}

fn process_google_docs_message(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    use crate::adapters::google_docs::{ActionableComment, GoogleDocsInboundAdapter};
    use crate::google_auth::GoogleAuth;

    let actionable: ActionableComment = serde_json::from_slice(raw_payload)?;
    let document_id = message
        .metadata
        .google_docs_document_id
        .as_deref()
        .ok_or("missing google_docs_document_id")?;
    let user_email = extract_emails(&message.sender)
        .into_iter()
        .next()
        .unwrap_or_else(|| format!("gdocs_{}@local", message.sender.replace(' ', "_")));
    let user = user_store.get_or_create_user("google_docs", &user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = format!("gdocs:{}:{}", document_id, actionable.comment.id);

    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;

    let thread_state_path = default_thread_state_path(&workspace);
    let thread_state = bump_thread_state(
        &thread_state_path,
        &thread_key,
        Some(actionable.tracking_id.clone()),
    )?;

    append_google_docs_comment(&workspace, message, &actionable, thread_state.last_email_seq)?;

    if let Ok(auth) = GoogleAuth::from_env() {
        let adapter = GoogleDocsInboundAdapter::new(auth, HashSet::new());
        match adapter.read_document_content(document_id) {
            Ok(doc_content) => {
                let doc_content_path =
                    workspace.join("incoming_email").join("document_content.txt");
                if let Err(e) = fs::write(&doc_content_path, &doc_content) {
                    warn!(
                        "Failed to save document content for {}: {}",
                        document_id, e
                    );
                }
            }
            Err(e) => {
                warn!("Failed to fetch document content for {}: {}", document_id, e);
            }
        }
    }

    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config.employee_profile.runner.eq_ignore_ascii_case("claude") {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: vec![message.sender.clone()],
        reply_from: config.employee_profile.addresses.first().cloned(),
        archive_root: None,
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::GoogleDocs,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;

    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={:?} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message.message_id,
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
}

/// Save an incoming Google Docs comment or reply to the workspace.
fn append_google_docs_comment(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    actionable: &crate::adapters::google_docs::ActionableComment,
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;

    // Save the raw comment JSON (includes all replies)
    let raw_path = incoming_dir.join(format!("{:05}_gdocs_comment.json", seq));
    let raw_json = serde_json::to_string_pretty(&actionable.comment)?;
    fs::write(&raw_path, &raw_json)?;

    // Create HTML representation for the agent
    let doc_name = message.metadata.google_docs_document_name.as_deref().unwrap_or("Document");
    let doc_id = message.metadata.google_docs_document_id.as_deref().unwrap_or("");
    let sender_name = message.sender_name.as_deref().unwrap_or(&message.sender);
    let quoted_text = actionable.comment.quoted_file_content.as_ref()
        .and_then(|q| q.value.as_deref())
        .unwrap_or("");

    let item_type = if actionable.triggering_reply.is_some() { "Reply" } else { "Comment" };

    // Build conversation thread HTML if this is a reply
    let thread_html = if let Some(ref reply) = actionable.triggering_reply {
        let original_author = actionable.comment.author.as_ref()
            .and_then(|a| a.display_name.as_deref())
            .unwrap_or("Someone");

        format!(
            r#"<h3>Conversation Thread:</h3>
<div style="margin-bottom: 10px;">
    <p><strong>{} (original comment):</strong></p>
    <p>{}</p>
</div>
<div style="margin-left: 20px; border-left: 2px solid #ccc; padding-left: 10px;">
    <p><strong>{} (reply that mentions you):</strong></p>
    <p>{}</p>
</div>"#,
            original_author,
            actionable.comment.content,
            sender_name,
            reply.content
        )
    } else {
        format!(
            r#"<h3>Comment:</h3>
<p>{}</p>"#,
            actionable.comment.content
        )
    };

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>Google Docs {}</title></head>
<body>
<h2>{} on: {}</h2>
<p><strong>Document ID:</strong> {}</p>
<p><strong>From:</strong> {} ({})</p>
<p><strong>Comment ID:</strong> {}</p>
<p><strong>Tracking ID:</strong> {}</p>
{}
{}
<hr>
<h3>Document Content:</h3>
<p>The full document content is available in: <code>incoming_email/document_content.txt</code></p>
<p>Read this file to understand the document context and make appropriate edits or suggestions.</p>
<hr>
<p><em>Respond by writing to reply_email_draft.html</em></p>
</body>
</html>"#,
        item_type, item_type, doc_name, doc_id, sender_name, message.sender,
        actionable.comment.id, actionable.tracking_id,
        if quoted_text.is_empty() {
            String::new()
        } else {
            format!("<h3>Quoted text:</h3><blockquote>{}</blockquote>", quoted_text)
        },
        thread_html
    );

    let html_path = incoming_dir.join(format!("{:05}_email.html", seq));
    fs::write(&html_path, &html_content)?;

    // Create metadata file
    let meta_path = incoming_dir.join(format!("{:05}_gdocs_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "google_docs",
        "sender": message.sender,
        "sender_name": message.sender_name,
        "document_id": doc_id,
        "document_name": doc_name,
        "comment_id": actionable.comment.id,
        "tracking_id": actionable.tracking_id,
        "is_reply": actionable.triggering_reply.is_some(),
        "thread_id": message.thread_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    let item_type_lower = if actionable.triggering_reply.is_some() { "reply" } else { "comment" };
    info!(
        "saved Google Docs {} seq={} tracking_id={} to {}",
        item_type_lower, seq, actionable.tracking_id, incoming_dir.display()
    );
    Ok(())
}

/// Save an incoming Slack message to the workspace.
fn append_slack_message(
    workspace: &Path,
    message: &crate::channel::InboundMessage,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_dir = workspace.join("incoming_email");
    fs::create_dir_all(&incoming_dir)?;

    // Save the raw JSON payload
    let raw_path = incoming_dir.join(format!("{:05}_slack_raw.json", seq));
    fs::write(&raw_path, raw_payload)?;

    // Save message text as a simple text file (similar to email body)
    let text_path = incoming_dir.join(format!("{:05}_slack_message.txt", seq));
    let text_content = message.text_body.clone().unwrap_or_default();
    fs::write(&text_path, &text_content)?;

    // Create a metadata file with sender info
    let meta_path = incoming_dir.join(format!("{:05}_slack_meta.json", seq));
    let meta = serde_json::json!({
        "channel": "slack",
        "sender": message.sender,
        "channel_id": message.metadata.slack_channel_id,
        "team_id": message.metadata.slack_team_id,
        "thread_id": message.thread_id,
        "message_id": message.message_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;

    info!(
        "saved Slack message seq={} to {}",
        seq,
        incoming_dir.display()
    );
    Ok(())
}

/// Send a quick response via Slack for locally-handled queries (async version).
async fn send_quick_slack_response(
    bot_token: &str,
    channel: &str,
    thread_ts: Option<&str>,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_base = std::env::var("SLACK_API_BASE_URL")
        .unwrap_or_else(|_| "https://slack.com/api".to_string());
    let url = format!("{}/chat.postMessage", api_base.trim_end_matches('/'));

    let mut request = serde_json::json!({
        "channel": channel,
        "text": response_text,
        "mrkdwn": true
    });

    if let Some(ts) = thread_ts {
        request["thread_ts"] = serde_json::Value::String(ts.to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", bot_token))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Slack API returned {}: {}", status, body).into());
    }

    let api_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if api_response.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        let error = api_response
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or("unknown error");
        return Err(format!("Slack API error: {}", error).into());
    }

    Ok(())
}

fn send_quick_discord_response_simple(
    bot_token: &str,
    channel_id: u64,
    message_id: Option<&str>,
    response_text: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use crate::adapters::discord::DiscordOutboundAdapter;
    use crate::channel::{ChannelMetadata, OutboundAdapter, OutboundMessage};

    let adapter = DiscordOutboundAdapter::new(bot_token.to_string());

    let message = OutboundMessage {
        channel: Channel::Discord,
        from: None,
        to: vec![channel_id.to_string()],
        cc: vec![],
        bcc: vec![],
        subject: String::new(),
        text_body: response_text.to_string(),
        html_body: String::new(),
        html_path: None,
        attachments_dir: None,
        thread_id: message_id.map(|value| value.to_string()),
        metadata: ChannelMetadata {
            discord_channel_id: Some(channel_id),
            ..Default::default()
        },
    };

    let result = adapter.send(&message)?;
    if !result.success {
        return Err(result.error.unwrap_or_else(|| "discord send failed".to_string()).into());
    }
    Ok(())
}

pub fn process_inbound_payload(
    config: &ServiceConfig,
    user_store: &UserStore,
    index_store: &IndexStore,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    info!("processing inbound payload into workspace");

    let sender = payload.from.as_deref().unwrap_or("").trim();
    if is_blacklisted_sender(sender, &config.employee_directory.service_addresses) {
        info!("skipping blacklisted sender: {}", sender);
        return Ok(());
    }
    let user_email = payload.from.as_deref().unwrap_or("").trim();
    let user_email = extract_emails(user_email)
        .into_iter()
        .next()
        .ok_or_else(|| "missing sender email".to_string())?;
    let user = user_store.get_or_create_user("email", &user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let reply_to_raw = payload.reply_to.as_deref().unwrap_or("");
    let from_raw = payload.from.as_deref().unwrap_or("");
    let mut to_list = replyable_recipients(reply_to_raw);
    if to_list.is_empty() {
        to_list = replyable_recipients(from_raw);
    }
    if to_list.is_empty() {
        info!(
            "no replyable recipients found (reply_to='{}', from='{}')",
            reply_to_raw, from_raw
        );
    }

    let inbound_candidates = collect_service_address_candidates(payload);
    let inbound_service_mailbox = mailbox::select_inbound_service_mailbox(
        &inbound_candidates,
        &config.employee_profile.address_set,
    );
    let inbound_service_mailbox = match inbound_service_mailbox {
        Some(mailbox) => mailbox,
        None => {
            info!("no service address found in inbound payload; skipping");
            return Ok(());
        }
    };

    let thread_key = thread_key(payload, raw_payload);
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        &config.employee_profile,
        config.skills_source_dir.as_deref(),
    )?;
    let reply_from = Some(inbound_service_mailbox.formatted());
    let model_name = match config.employee_profile.model.clone() {
        Some(model) => model,
        None => {
            if config
                .employee_profile
                .runner
                .eq_ignore_ascii_case("claude")
            {
                String::new()
            } else {
                config.codex_model.clone()
            }
        }
    };
    let thread_state_path = default_thread_state_path(&workspace);
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .map(|value| value.trim().to_string());
    let thread_state = bump_thread_state(&thread_state_path, &thread_key, message_id.clone())?;
    append_inbound_payload(
        &workspace,
        payload,
        raw_payload,
        thread_state.last_email_seq,
    )?;
    if let Err(err) = archive_inbound(&user_paths, payload, raw_payload) {
        error!("failed to archive inbound email: {}", err);
    }
    info!(
        "workspace ready at {} for user {} thread={} epoch={}",
        workspace.display(),
        user.user_id,
        thread_key,
        thread_state.epoch
    );

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name,
        runner: config.employee_profile.runner.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: to_list.clone(),
        reply_from,
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
        channel: Channel::Email,
        slack_team_id: None,
        employee_id: Some(config.employee_profile.id.clone()),
    };

    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
    if let Err(err) = cancel_pending_thread_tasks(&mut scheduler, &workspace, thread_state.epoch) {
        warn!(
            "failed to cancel pending thread tasks for {}: {}",
            workspace.display(),
            err
        );
    }
    let task_id = scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
    index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;
    info!(
        "scheduler tasks enqueued user_id={} task_id={} message_id={} workspace={} thread_epoch={}",
        user.user_id,
        task_id,
        message_id.unwrap_or_else(|| "-".to_string()),
        workspace.display(),
        thread_state.epoch
    );

    Ok(())
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

fn is_blacklisted_sender(sender: &str, service_addresses: &HashSet<String>) -> bool {
    if sender.is_empty() {
        return false;
    }
    let mut matched = false;
    let addresses = extract_emails(sender);
    for address in addresses {
        if is_blacklisted_address(&address, service_addresses) {
            matched = true;
            break;
        }
    }
    matched
}

fn is_blacklisted_address(address: &str, service_addresses: &HashSet<String>) -> bool {
    mailbox::is_service_address(address, service_addresses)
}

fn thread_key(payload: &PostmarkInbound, raw_payload: &[u8]) -> String {
    if let Some(value) = payload.header_value("References") {
        if let Some(id) = extract_first_message_id(value) {
            return id;
        }
    }
    if let Some(value) = payload.header_value("In-Reply-To") {
        if let Some(id) = extract_first_message_id(value) {
            return id;
        }
    }
    if let Some(id) = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .and_then(normalize_message_id)
    {
        return id;
    }
    format!("{:x}", md5::compute(raw_payload))
}

fn extract_first_message_id(value: &str) -> Option<String> {
    for token in value.split(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | ',' | ';')) {
        let trimmed = token.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(id) = normalize_message_id(trimmed) {
            return Some(id);
        }
    }
    None
}

fn thread_workspace_name(thread_key: &str) -> String {
    let hash = format!("{:x}", md5::compute(thread_key.as_bytes()));
    format!("thread_{}", hash)
}

fn copy_skills_directory(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let skill_src = entry.path();
        let skill_dest = dest.join(entry.file_name());

        if skill_src.is_dir() {
            copy_dir_recursive(&skill_src, &skill_dest)?;
        }
    }
    Ok(())
}

pub fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    Ok(())
}

fn ensure_workspace_employee_files(workspace: &Path, employee: &EmployeeProfile) -> io::Result<()> {
    if let Some(path) = employee.agents_path.as_ref() {
        if path.exists() {
            fs::copy(path, workspace.join("AGENTS.md"))?;
        }
    }
    if let Some(path) = employee.claude_path.as_ref() {
        if path.exists() {
            fs::copy(path, workspace.join("CLAUDE.md"))?;
        }
    }
    if let Some(path) = employee.soul_path.as_ref() {
        if path.exists() {
            fs::copy(path, workspace.join("SOUL.md"))?;
        }
    }
    Ok(())
}

fn ensure_thread_workspace(
    user_paths: &crate::user_store::UserPaths,
    user_id: &str,
    thread_key: &str,
    employee: &EmployeeProfile,
    skills_source_dir: Option<&Path>,
) -> Result<PathBuf, BoxError> {
    fs::create_dir_all(&user_paths.workspaces_root)?;

    let workspace_name = thread_workspace_name(thread_key);
    let workspace = user_paths.workspaces_root.join(workspace_name);
    let is_new = !workspace.exists();
    if is_new {
        fs::create_dir_all(&workspace)?;
    }

    let incoming_email = workspace.join("incoming_email");
    let incoming_attachments = workspace.join("incoming_attachments");
    let memory = workspace.join("memory");
    let references = workspace.join("references");

    fs::create_dir_all(&incoming_email)?;
    fs::create_dir_all(&incoming_attachments)?;
    fs::create_dir_all(&memory)?;
    fs::create_dir_all(&references)?;

    if is_new || !references.join("past_emails").exists() {
        if let Err(err) = crate::past_emails::hydrate_past_emails(
            &user_paths.mail_root,
            &references,
            user_id,
            None,
        ) {
            error!("failed to hydrate past_emails: {}", err);
        }
    }

    ensure_workspace_employee_files(&workspace, employee)?;

    // Copy skills to workspace for Codex/Claude runners.
    let agents_skills_dir = workspace.join(".agents").join("skills");
    if let Some(skills_src) = skills_source_dir {
        if let Err(err) = copy_skills_directory(skills_src, &agents_skills_dir) {
            error!("failed to copy base skills to workspace: {}", err);
        }
    }
    if let Some(employee_skills) = employee.skills_dir.as_deref() {
        let should_copy = skills_source_dir
            .map(|base| base != employee_skills)
            .unwrap_or(true);
        if should_copy {
            if let Err(err) = copy_skills_directory(employee_skills, &agents_skills_dir) {
                error!("failed to copy employee skills to workspace: {}", err);
            }
        }
    }

    Ok(workspace)
}

fn append_inbound_payload(
    workspace: &Path,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
    seq: u64,
) -> Result<(), BoxError> {
    let incoming_email = workspace.join("incoming_email");
    let incoming_attachments = workspace.join("incoming_attachments");
    let entries_email = incoming_email.join("entries");
    let entries_attachments = incoming_attachments.join("entries");
    fs::create_dir_all(&entries_email)?;
    fs::create_dir_all(&entries_attachments)?;

    let entry_name = build_inbound_entry_name(payload, seq);
    let entry_email_dir = entries_email.join(&entry_name);
    let entry_attachments_dir = entries_attachments.join(&entry_name);
    fs::create_dir_all(&entry_email_dir)?;
    fs::create_dir_all(&entry_attachments_dir)?;
    write_inbound_payload(
        payload,
        raw_payload,
        &entry_email_dir,
        &entry_attachments_dir,
    )?;

    clear_dir_except(&incoming_attachments, &entries_attachments)?;
    write_inbound_payload(payload, raw_payload, &incoming_email, &incoming_attachments)?;
    if let Err(err) = write_thread_history(&incoming_email, &incoming_attachments) {
        warn!("failed to write thread history: {}", err);
    }
    Ok(())
}

fn clear_dir_except(root: &Path, keep: &Path) -> Result<(), io::Error> {
    if !root.exists() {
        fs::create_dir_all(root)?;
        return Ok(());
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path == keep {
            continue;
        }
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn archive_inbound(
    user_paths: &crate::user_store::UserPaths,
    payload: &PostmarkInbound,
    raw_payload: &[u8],
) -> Result<(), BoxError> {
    let fallback = format!("email_{}", Utc::now().timestamp());
    let message_id = payload
        .header_message_id()
        .or(payload.message_id.as_deref())
        .unwrap_or("");
    let base = sanitize_token(message_id, &fallback);
    let year = Utc::now().format("%Y").to_string();
    let month = Utc::now().format("%m").to_string();
    let mail_root = user_paths.mail_root.join(year).join(month);
    fs::create_dir_all(&mail_root)?;
    let mail_dir = create_unique_dir(&mail_root, &base)?;
    let incoming_email = mail_dir.join("incoming_email");
    let incoming_attachments = mail_dir.join("incoming_attachments");
    fs::create_dir_all(&incoming_email)?;
    fs::create_dir_all(&incoming_attachments)?;
    write_inbound_payload(payload, raw_payload, &incoming_email, &incoming_attachments)?;
    Ok(())
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

fn write_inbound_payload(
    payload: &PostmarkInbound,
    raw_payload: &[u8],
    incoming_email: &Path,
    incoming_attachments: &Path,
) -> Result<(), BoxError> {
    fs::write(incoming_email.join("postmark_payload.json"), raw_payload)?;
    let email_html = render_email_html(payload);
    fs::write(incoming_email.join("email.html"), email_html)?;

    if let Some(attachments) = payload.attachments.as_ref() {
        for attachment in attachments {
            let name = sanitize_token(&attachment.name, "attachment");
            let target = incoming_attachments.join(name);
            let data = BASE64_STANDARD
                .decode(attachment.content.as_bytes())
                .unwrap_or_default();
            fs::write(target, data)?;
        }
    }
    Ok(())
}

fn build_inbound_entry_name(payload: &PostmarkInbound, seq: u64) -> String {
    let subject = payload.subject.as_deref().unwrap_or("");
    let subject_token = sanitize_token(subject, "no_subject");
    let subject_token = truncate_ascii(&subject_token, 48);
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let base = format!("{}_{}", timestamp, subject_token);
    format!("{:04}_{}", seq, base)
}

fn truncate_ascii(value: &str, max_len: usize) -> String {
    if value.len() <= max_len {
        return value.to_string();
    }
    let mut out = value[..max_len].to_string();
    while out.ends_with(['.', '_', '-']) {
        out.pop();
    }
    if out.is_empty() {
        value.to_string()
    } else {
        out
    }
}

fn write_thread_history(
    incoming_email: &Path,
    incoming_attachments: &Path,
) -> Result<(), BoxError> {
    let entries_email = incoming_email.join("entries");
    if !entries_email.exists() {
        return Ok(());
    }

    let mut entry_dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(&entries_email)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            entry_dirs.push(entry.path());
        }
    }
    entry_dirs.sort_by_key(|path| {
        path.file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_default()
    });

    let mut output = String::new();
    output.push_str("# Thread history (inbound)\n");
    output.push_str("Auto-generated from incoming_email/entries. Latest entry is last.\n\n");

    for entry_dir in entry_dirs {
        let entry_name = entry_dir
            .file_name()
            .map(|value| value.to_string_lossy().to_string())
            .unwrap_or_else(|| "entry".to_string());
        let payload_path = entry_dir.join("postmark_payload.json");
        let summary = load_payload_summary(&payload_path);
        let attachments_dir = incoming_attachments.join("entries").join(&entry_name);
        let attachments = list_attachment_names(&attachments_dir).unwrap_or_default();
        let email_file = if entry_dir.join("email.html").exists() {
            "email.html"
        } else if entry_dir.join("email.txt").exists() {
            "email.txt"
        } else {
            "email.html"
        };

        output.push_str(&format!("## {entry_name}\n"));
        if let Some(summary) = summary {
            output.push_str(&format!("Subject: {}\n", summary.subject));
            output.push_str(&format!("From: {}\n", summary.from));
            output.push_str(&format!("To: {}\n", summary.to));
            if !summary.cc.is_empty() {
                output.push_str(&format!("Cc: {}\n", summary.cc));
            }
            if !summary.bcc.is_empty() {
                output.push_str(&format!("Bcc: {}\n", summary.bcc));
            }
            if let Some(date) = summary.date.as_deref() {
                output.push_str(&format!("Date: {}\n", date));
            }
            if !summary.message_id.is_empty() {
                output.push_str(&format!("Message-ID: {}\n", summary.message_id));
            }
            let preview = build_preview(&summary);
            if let Some(preview) = preview {
                output.push_str("Preview:\n```text\n");
                output.push_str(&preview);
                output.push_str("\n```\n");
            }
        }

        output.push_str("Files:\n");
        output.push_str(&format!(
            "- incoming_email/entries/{entry_name}/{email_file}\n"
        ));
        output.push_str(&format!(
            "- incoming_email/entries/{entry_name}/postmark_payload.json\n"
        ));
        if !attachments.is_empty() {
            output.push_str(&format!(
                "- incoming_attachments/entries/{entry_name}/ ({})\n",
                attachments.join(", ")
            ));
        } else {
            output.push_str("- incoming_attachments/entries/(none)\n");
        }
        output.push('\n');
    }

    fs::write(incoming_email.join("thread_history.md"), output)?;
    Ok(())
}

#[derive(Default)]
struct PayloadSummary {
    subject: String,
    from: String,
    to: String,
    cc: String,
    bcc: String,
    date: Option<String>,
    message_id: String,
    text_body: Option<String>,
    html_body: Option<String>,
}

fn load_payload_summary(payload_path: &Path) -> Option<PayloadSummary> {
    let payload_data = fs::read_to_string(payload_path).ok()?;
    let payload_json: serde_json::Value = serde_json::from_str(&payload_data).ok()?;
    Some(PayloadSummary {
        subject: json_string(&payload_json, "Subject").unwrap_or_default(),
        from: json_string(&payload_json, "From").unwrap_or_default(),
        to: json_string(&payload_json, "To").unwrap_or_default(),
        cc: json_string(&payload_json, "Cc").unwrap_or_default(),
        bcc: json_string(&payload_json, "Bcc").unwrap_or_default(),
        date: json_string(&payload_json, "Date")
            .or_else(|| json_string(&payload_json, "ReceivedAt")),
        message_id: json_string(&payload_json, "MessageID")
            .or_else(|| json_string(&payload_json, "MessageId"))
            .unwrap_or_default(),
        text_body: json_string(&payload_json, "TextBody")
            .or_else(|| json_string(&payload_json, "StrippedTextReply")),
        html_body: json_string(&payload_json, "HtmlBody"),
    })
}

fn json_string(payload: &serde_json::Value, key: &str) -> Option<String> {
    payload
        .get(key)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

fn list_attachment_names(dir: &Path) -> Result<Vec<String>, io::Error> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    names.sort();
    Ok(names)
}

fn build_preview(summary: &PayloadSummary) -> Option<String> {
    let mut preview = summary
        .text_body
        .as_deref()
        .unwrap_or("")
        .trim()
        .to_string();
    if preview.is_empty() {
        preview = summary
            .html_body
            .as_deref()
            .map(strip_html_tags)
            .unwrap_or_default();
    }
    let preview = preview.trim();
    if preview.is_empty() {
        return None;
    }
    Some(truncate_preview(preview, 1200))
}

fn truncate_preview(input: &str, max_len: usize) -> String {
    if input.len() <= max_len {
        return input.to_string();
    }
    let mut end = max_len;
    while end > 0 && !input.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &input[..end])
}

fn strip_html_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn create_unique_dir(root: &Path, base: &str) -> Result<PathBuf, io::Error> {
    let mut candidate = root.join(base);
    if !candidate.exists() {
        fs::create_dir_all(&candidate)?;
        return Ok(candidate);
    }
    for idx in 1..1000 {
        let name = format!("{}_{}", base, idx);
        candidate = root.join(name);
        if !candidate.exists() {
            fs::create_dir_all(&candidate)?;
            return Ok(candidate);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "failed to create unique workspace directory",
    ))
}

fn clean_inbound_html(html: &str) -> String {
    let document = kuchiki::parse_html().one(html);
    remove_html_comments(&document);
    remove_elements_by_selector(
        &document,
        "head, script, style, meta, link, title, noscript",
    );
    remove_hidden_elements(&document);
    remove_tracking_pixels(&document);
    remove_footer_blocks(&document);
    sanitize_allowed_elements(&document);
    extract_body_html(&document)
}

fn remove_html_comments(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        if node.as_comment().is_some() {
            node.detach();
        }
    }
}

fn remove_elements_by_selector(document: &NodeRef, selector: &str) {
    if let Ok(nodes) = document.select(selector) {
        for node in nodes {
            node.as_node().detach();
        }
    }
}

fn remove_hidden_elements(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        if is_hidden_element(element) {
            node.detach();
        }
    }
}

fn remove_tracking_pixels(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        if element.name.local.as_ref() == "img" && is_tracking_pixel(element) {
            node.detach();
        }
    }
}

fn remove_footer_blocks(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        let tag = element.name.local.as_ref();
        if !is_footer_candidate(tag) {
            continue;
        }
        if element_has_footer_marker(element) {
            node.detach();
            continue;
        }
        let text = node.text_contents();
        if text_contains_footer_hint(&text) {
            node.detach();
        }
    }
}

fn sanitize_allowed_elements(document: &NodeRef) {
    let nodes: Vec<NodeRef> = document.descendants().collect();
    for node in nodes {
        let element = match node.as_element() {
            Some(value) => value,
            None => continue,
        };
        let tag = element.name.local.as_ref();
        if is_drop_tag(tag) {
            node.detach();
            continue;
        }
        if !is_allowed_tag(tag) {
            unwrap_node(&node);
            continue;
        }
        prune_attributes(tag, element);
    }
}

fn extract_body_html(document: &NodeRef) -> String {
    if let Ok(mut bodies) = document.select("body") {
        if let Some(body) = bodies.next() {
            let mut out = String::new();
            for child in body.as_node().children() {
                out.push_str(&child.to_string());
            }
            return out;
        }
    }
    document.to_string()
}

fn unwrap_node(node: &NodeRef) {
    if node.parent().is_none() {
        return;
    }
    let children: Vec<NodeRef> = node.children().collect();
    for child in children {
        node.insert_before(child);
    }
    node.detach();
}

fn is_allowed_tag(tag: &str) -> bool {
    matches!(
        tag,
        "html"
            | "body"
            | "p"
            | "br"
            | "div"
            | "span"
            | "a"
            | "img"
            | "ul"
            | "ol"
            | "li"
            | "strong"
            | "em"
            | "b"
            | "i"
            | "u"
            | "blockquote"
            | "pre"
            | "code"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "table"
            | "thead"
            | "tbody"
            | "tr"
            | "td"
            | "th"
    )
}

fn is_drop_tag(tag: &str) -> bool {
    matches!(
        tag,
        "script" | "style" | "head" | "meta" | "link" | "title" | "noscript"
    )
}

fn is_footer_candidate(tag: &str) -> bool {
    matches!(
        tag,
        "div" | "p" | "span" | "td" | "li" | "section" | "footer"
    )
}

fn element_has_footer_marker(element: &kuchiki::ElementData) -> bool {
    let attrs = element.attributes.borrow();
    for key in ["class", "id"] {
        if let Some(value) = attrs.get(key) {
            let lower = value.to_ascii_lowercase();
            if lower.contains("footer")
                || lower.contains("unsubscribe")
                || lower.contains("notification")
                || lower.contains("preferences")
            {
                return true;
            }
        }
    }
    false
}

fn text_contains_footer_hint(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let hints = [
        "unsubscribe",
        "notification settings",
        "manage notifications",
        "email preferences",
        "manage your email",
        "view this email in your browser",
        "view in browser",
        "you are receiving this",
        "to stop receiving",
        "opt out",
        "reply to this email directly",
    ];
    hints.iter().any(|hint| lower.contains(hint))
}

fn is_hidden_element(element: &kuchiki::ElementData) -> bool {
    let attrs = element.attributes.borrow();
    if attrs.contains("hidden") {
        return true;
    }
    if let Some(value) = attrs.get("aria-hidden") {
        if value.trim().eq_ignore_ascii_case("true") {
            return true;
        }
    }
    if let Some(style) = attrs.get("style") {
        if style_contains_hidden(style) {
            return true;
        }
    }
    false
}

fn is_tracking_pixel(element: &kuchiki::ElementData) -> bool {
    let attrs = element.attributes.borrow();
    if let Some(style) = attrs.get("style") {
        if style_contains_hidden(style) {
            return true;
        }
    }
    let src = attrs.get("src").unwrap_or("");
    let src_lower = src.to_ascii_lowercase();
    if src_lower.contains("tracking")
        || src_lower.contains("pixel")
        || src_lower.contains("beacon")
        || src_lower.contains("open.gif")
    {
        return true;
    }
    let width = attrs.get("width").and_then(parse_dimension).or_else(|| {
        attrs
            .get("style")
            .and_then(|style| style_dimension(style, "width"))
    });
    let height = attrs.get("height").and_then(parse_dimension).or_else(|| {
        attrs
            .get("style")
            .and_then(|style| style_dimension(style, "height"))
    });
    matches_1x1(width, height)
}

fn matches_1x1(width: Option<u32>, height: Option<u32>) -> bool {
    match (width, height) {
        (Some(w), Some(h)) => w <= 1 && h <= 1,
        (Some(w), None) => w <= 1,
        (None, Some(h)) => h <= 1,
        (None, None) => false,
    }
}

fn style_contains_hidden(style: &str) -> bool {
    let normalized: String = style
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();
    normalized.contains("display:none")
        || normalized.contains("visibility:hidden")
        || normalized.contains("opacity:0")
        || normalized.contains("max-height:0")
}

fn style_dimension(style: &str, key: &str) -> Option<u32> {
    for part in style.split(';') {
        let mut iter = part.splitn(2, ':');
        let name = iter.next().unwrap_or("").trim().to_ascii_lowercase();
        if name == key {
            let value = iter.next().unwrap_or("").trim();
            return parse_dimension(value);
        }
    }
    None
}

fn parse_dimension(raw: &str) -> Option<u32> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let digits: String = trimmed
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn prune_attributes(tag: &str, element: &kuchiki::ElementData) {
    let mut attrs = element.attributes.borrow_mut();
    let mut to_remove = Vec::new();
    for (name, _) in attrs.map.iter() {
        let local = name.local.as_ref();
        let keep = match tag {
            "a" => matches!(local, "href"),
            "img" => matches!(local, "src" | "alt" | "width" | "height"),
            _ => false,
        };
        if !keep {
            to_remove.push(name.clone());
        }
    }
    for name in to_remove {
        attrs.map.remove(&name);
    }
    if tag == "a" {
        if let Some(href) = attrs.get("href").map(|value| value.to_string()) {
            if !is_safe_link(&href) {
                attrs.remove("href");
            }
        }
    }
    if tag == "img" {
        if let Some(src) = attrs.get("src").map(|value| value.to_string()) {
            if !is_safe_image_src(&src) {
                attrs.remove("src");
            }
        }
    }
}

fn is_safe_link(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !(lower.starts_with("javascript:") || lower.starts_with("vbscript:"))
}

fn is_safe_image_src(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    !(lower.starts_with("javascript:") || lower.starts_with("vbscript:"))
}

fn render_email_html(payload: &PostmarkInbound) -> String {
    if let Some(html) = payload
        .html_body
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        let cleaned = clean_inbound_html(html);
        if !cleaned.trim().is_empty() {
            return cleaned;
        }
    }

    let text_body = payload
        .text_body
        .as_deref()
        .or(payload.stripped_text_reply.as_deref())
        .unwrap_or("");
    if text_body.trim().is_empty() {
        return "<pre>(no content)</pre>".to_string();
    }
    wrap_text_as_html(text_body)
}

fn wrap_text_as_html(input: &str) -> String {
    format!("<pre>{}</pre>", escape_html(input))
}

fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let trimmed = value.trim().trim_start_matches('<').trim_end_matches('>');
    let mut out = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    let cleaned = out.trim_matches(&['.', '_', '-'][..]);
    if cleaned.is_empty() {
        fallback.to_string()
    } else {
        cleaned.to_string()
    }
}

fn split_recipients(value: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;

    for ch in value.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => {
                escaped = true;
                current.push(ch);
            }
            '"' => {
                in_quotes = !in_quotes;
                current.push(ch);
            }
            ',' | ';' if !in_quotes => {
                let trimmed = current.trim();
                if !trimmed.is_empty() {
                    out.push(trimmed.to_string());
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }

    let trimmed = current.trim();
    if !trimmed.is_empty() {
        out.push(trimmed.to_string());
    }

    out
}

fn replyable_recipients(raw: &str) -> Vec<String> {
    split_recipients(raw)
        .into_iter()
        .filter(|recipient| contains_replyable_address(recipient))
        .collect()
}

fn contains_replyable_address(value: &str) -> bool {
    let emails = extract_emails(value);
    if emails.is_empty() {
        return false;
    }
    emails.iter().any(|address| !is_no_reply_address(address))
}

// Only local-part markers; avoid domain-based filtering.
const NO_REPLY_LOCAL_PARTS: [&str; 3] = ["noreply", "no-reply", "do-not-reply"];

fn is_no_reply_address(address: &str) -> bool {
    let normalized = address.trim().to_ascii_lowercase();
    let local = normalized.split('@').next().unwrap_or("");
    if local.is_empty() {
        return false;
    }
    NO_REPLY_LOCAL_PARTS.iter().any(|marker| local == *marker)
}

fn env_flag(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(
            value.trim().to_lowercase().as_str(),
            "1" | "true" | "yes" | "y"
        ),
        Err(_) => default,
    }
}

fn repo_skills_source_dir() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd
        .file_name()
        .map(|name| name == "DoWhiz_service")
        .unwrap_or(false)
    {
        cwd.join("skills")
    } else {
        cwd.join("DoWhiz_service").join("skills")
    }
}

fn default_employee_config_path() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd
        .file_name()
        .map(|name| name == "DoWhiz_service")
        .unwrap_or(false)
    {
        cwd.join("employee.toml")
    } else {
        cwd.join("DoWhiz_service").join("employee.toml")
    }
}

fn default_runtime_root() -> Result<PathBuf, io::Error> {
    let home =
        env::var("HOME").map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME not set"))?;
    Ok(PathBuf::from(home)
        .join(".dowhiz")
        .join("DoWhiz")
        .join("run_task"))
}

fn resolve_path(raw: String) -> Result<PathBuf, io::Error> {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Ok(path)
    } else {
        let cwd = env::current_dir()?;
        Ok(cwd.join(path))
    }
}

fn collect_service_address_candidates(payload: &PostmarkInbound) -> Vec<Option<&str>> {
    let mut candidates = Vec::new();
    if let Some(value) = payload.to.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.cc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(value) = payload.bcc.as_deref() {
        candidates.push(Some(value));
    }
    if let Some(list) = payload.to_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.cc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    if let Some(list) = payload.bcc_full.as_ref() {
        for entry in list {
            candidates.push(Some(entry.email.as_str()));
        }
    }
    for header in [
        "X-Original-To",
        "Delivered-To",
        "Envelope-To",
        "X-Envelope-To",
        "X-Forwarded-To",
        "X-Original-Recipient",
        "Original-Recipient",
    ] {
        for value in payload.header_values(header) {
            candidates.push(Some(value));
        }
    }
    candidates
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostmarkInbound {
    #[serde(rename = "From")]
    from: Option<String>,
    #[serde(rename = "To")]
    #[allow(dead_code)]
    to: Option<String>,
    #[serde(rename = "Cc")]
    #[allow(dead_code)]
    cc: Option<String>,
    #[serde(rename = "Bcc")]
    #[allow(dead_code)]
    bcc: Option<String>,
    #[serde(rename = "ToFull")]
    #[allow(dead_code)]
    to_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "CcFull")]
    #[allow(dead_code)]
    cc_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "BccFull")]
    #[allow(dead_code)]
    bcc_full: Option<Vec<PostmarkRecipient>>,
    #[serde(rename = "ReplyTo")]
    reply_to: Option<String>,
    #[serde(rename = "Subject")]
    subject: Option<String>,
    #[serde(rename = "TextBody")]
    text_body: Option<String>,
    #[serde(rename = "StrippedTextReply")]
    stripped_text_reply: Option<String>,
    #[serde(rename = "HtmlBody")]
    html_body: Option<String>,
    #[serde(rename = "MessageID", alias = "MessageId")]
    message_id: Option<String>,
    #[serde(rename = "Headers")]
    headers: Option<Vec<PostmarkHeader>>,
    #[serde(rename = "Attachments")]
    attachments: Option<Vec<PostmarkAttachment>>,
}

impl PostmarkInbound {
    fn header_value(&self, name: &str) -> Option<&str> {
        self.headers.as_ref().and_then(|headers| {
            headers
                .iter()
                .find(|header| header.name.eq_ignore_ascii_case(name))
                .map(|header| header.value.as_str())
        })
    }

    fn header_message_id(&self) -> Option<&str> {
        self.header_value("Message-ID")
    }

    fn header_values(&self, name: &str) -> Vec<&str> {
        self.headers
            .as_ref()
            .map(|headers| {
                headers
                    .iter()
                    .filter(|header| header.name.eq_ignore_ascii_case(name))
                    .map(|header| header.value.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkRecipient {
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "Name")]
    #[allow(dead_code)]
    name: Option<String>,
    #[serde(rename = "MailboxHash")]
    #[allow(dead_code)]
    mailbox_hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkHeader {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PostmarkAttachment {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Content")]
    content: String,
    #[serde(rename = "ContentType")]
    #[allow(dead_code)]
    content_type: String,
}

fn normalize_message_id(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_matches(|ch| matches!(ch, '<' | '>'));
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_ascii_lowercase())
}

struct ProcessedMessageStore {
    path: PathBuf,
    seen: HashSet<String>,
}

impl ProcessedMessageStore {
    fn load(path: &Path) -> Result<Self, io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut seen = HashSet::new();
        if path.exists() {
            for raw in fs::read_to_string(path)?.lines() {
                let line = raw.trim();
                if !line.is_empty() {
                    seen.insert(line.to_string());
                }
            }
        }
        Ok(Self {
            path: path.to_path_buf(),
            seen,
        })
    }

    fn mark_if_new(&mut self, ids: &[String]) -> Result<bool, io::Error> {
        let candidates: Vec<_> = ids
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .collect();
        if candidates.is_empty() {
            return Ok(true);
        }

        if candidates.iter().any(|value| self.seen.contains(*value)) {
            return Ok(false);
        }

        let mut handle = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        for value in candidates {
            self.seen.insert(value.to_string());
            use std::io::Write;
            writeln!(handle, "{}", value)?;
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{ChannelMetadata, InboundMessage};
    use tempfile::TempDir;

    #[test]
    fn create_workspace_hydrates_past_emails() {
        let temp = TempDir::new().expect("tempdir");
        let user_root = temp.path().join("user");
        let user_paths = crate::user_store::UserPaths {
            root: user_root.clone(),
            state_dir: user_root.join("state"),
            tasks_db_path: user_root.join("state/tasks.db"),
            memory_dir: user_root.join("memory"),
            secrets_dir: user_root.join("secrets"),
            mail_root: user_root.join("mail"),
            workspaces_root: user_root.join("workspaces"),
        };
        fs::create_dir_all(&user_paths.mail_root).expect("mail root");
        fs::create_dir_all(&user_paths.workspaces_root).expect("workspaces root");

        let archive_dir = user_paths.mail_root.join("2026").join("02").join("msg_1");
        let incoming_email = archive_dir.join("incoming_email");
        let incoming_attachments = archive_dir.join("incoming_attachments");
        fs::create_dir_all(&incoming_email).expect("incoming_email");
        fs::create_dir_all(&incoming_attachments).expect("incoming_attachments");
        fs::write(incoming_email.join("email.html"), "<pre>Hello</pre>").expect("email.html");
        let archived_payload = r#"{
  "From": "Alice <alice@example.com>",
  "To": "Bob <bob@example.com>",
  "Subject": "Archive hello",
  "Date": "Tue, 03 Feb 2026 20:10:44 -0800",
  "MessageID": "<msg-1@example.com>",
  "Attachments": [
    {"Name": "report.pdf", "ContentType": "application/pdf"}
  ]
}"#;
        fs::write(
            incoming_email.join("postmark_payload.json"),
            archived_payload,
        )
        .expect("postmark payload");
        fs::write(incoming_attachments.join("report.pdf"), "data").expect("attachment");

        let inbound_raw = r#"{
  "From": "New <new@example.com>",
  "To": "Service <service@example.com>",
  "Subject": "New request",
  "TextBody": "Hi"
}"#;
        let inbound_payload: PostmarkInbound =
            serde_json::from_str(inbound_raw).expect("parse inbound");
        let thread = thread_key(&inbound_payload, inbound_raw.as_bytes());
        let addresses = vec!["service@example.com".to_string()];
        let address_set: HashSet<String> = addresses
            .iter()
            .map(|value| value.to_ascii_lowercase())
            .collect();
        let employee = EmployeeProfile {
            id: "test-employee".to_string(),
            display_name: None,
            runner: "codex".to_string(),
            model: None,
            addresses,
            address_set,
            runtime_root: None,
            agents_path: None,
            claude_path: None,
            soul_path: None,
            skills_dir: None,
            discord_enabled: false,
            slack_enabled: false,
            bluebubbles_enabled: false,
        };
        let workspace = ensure_thread_workspace(&user_paths, "user123", &thread, &employee, None)
            .expect("create workspace");

        let past_root = workspace.join("references").join("past_emails");
        let index_path = past_root.join("index.json");
        assert!(index_path.exists(), "index.json created");

        let index_data = fs::read_to_string(index_path).expect("read index");
        let index_json: serde_json::Value = serde_json::from_str(&index_data).expect("parse index");
        let entries = index_json["entries"].as_array().expect("entries array");
        assert_eq!(entries.len(), 1, "one archived entry");
        let entry_path = entries[0]["path"].as_str().expect("entry path");
        assert!(past_root.join(entry_path).join("incoming_email").exists());
        assert!(past_root
            .join(entry_path)
            .join("attachments_manifest.json")
            .exists());
    }

    #[test]
    fn replyable_recipients_filters_no_reply_addresses() {
        let raw = "No Reply <noreply@example.com>, Real <user@example.com>";
        let recipients = replyable_recipients(raw);
        assert_eq!(recipients, vec!["Real <user@example.com>"]);
    }

    #[test]
    fn replyable_recipients_returns_empty_when_only_no_reply() {
        let raw = "No Reply <no-reply@example.com>";
        let recipients = replyable_recipients(raw);
        assert!(recipients.is_empty());
    }

    #[test]
    fn replyable_recipients_keeps_quoted_display_name_commas() {
        let raw =
            "\"Zoom Video Communications, Inc\" <reply@example.com>, Other <other@example.com>";
        let recipients = replyable_recipients(raw);
        assert_eq!(
            recipients,
            vec![
                "\"Zoom Video Communications, Inc\" <reply@example.com>",
                "Other <other@example.com>"
            ]
        );
    }

    #[test]
    fn no_reply_detection_matches_common_variants() {
        assert!(is_no_reply_address("noreply@example.com"));
        assert!(is_no_reply_address("no-reply@example.com"));
        assert!(is_no_reply_address("do-not-reply@example.com"));
        assert!(!is_no_reply_address("reply@example.com"));
    }

    #[test]
    fn no_reply_detection_requires_exact_local_part() {
        assert!(!is_no_reply_address("noreplying@example.com"));
        assert!(!is_no_reply_address("reply-noreply@example.com"));
        assert!(!is_no_reply_address("no-reply-bot@example.com"));
    }

    #[test]
    fn no_reply_detection_ignores_domain_markers() {
        assert!(!is_no_reply_address("notifications@github.com"));
        assert!(!is_no_reply_address("octocat@users.noreply.github.com"));
    }

    #[test]
    fn process_sms_message_creates_run_task() -> Result<(), BoxError> {
        let temp = TempDir::new()?;
        let root = temp.path();
        let users_root = root.join("users");
        let state_root = root.join("state");
        fs::create_dir_all(&users_root)?;
        fs::create_dir_all(&state_root)?;

        let addresses = vec!["service@example.com".to_string()];
        let address_set: HashSet<String> = addresses
            .iter()
            .map(|value| value.to_ascii_lowercase())
            .collect();
        let employee = EmployeeProfile {
            id: "test-employee".to_string(),
            display_name: None,
            runner: "codex".to_string(),
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
        employee_by_id.insert(employee.id.clone(), employee.clone());
        let employee_directory = EmployeeDirectory {
            employees: vec![employee.clone()],
            employee_by_id,
            default_employee_id: Some(employee.id.clone()),
            service_addresses: address_set,
        };

        let config = ServiceConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
            employee_id: employee.id.clone(),
            employee_config_path: root.join("employee.toml"),
            employee_profile: employee,
            employee_directory,
            workspace_root: root.join("workspaces"),
            scheduler_state_path: state_root.join("tasks.db"),
            processed_ids_path: state_root.join("processed_ids.txt"),
            ingestion_db_path: state_root.join("ingestion.db"),
            ingestion_dedupe_path: state_root.join("ingestion_processed_ids.txt"),
            ingestion_poll_interval: Duration::from_millis(50),
            users_root: users_root.clone(),
            users_db_path: state_root.join("users.db"),
            task_index_path: state_root.join("task_index.db"),
            codex_model: "gpt-5.2-codex".to_string(),
            codex_disabled: true,
            scheduler_poll_interval: Duration::from_millis(50),
            scheduler_max_concurrency: 1,
            scheduler_user_max_concurrency: 1,
            inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
            skills_source_dir: None,
            slack_bot_token: None,
            slack_bot_user_id: None,
            slack_store_path: state_root.join("slack.db"),
            slack_client_id: None,
            slack_client_secret: None,
            slack_redirect_uri: None,
            discord_bot_token: None,
            discord_bot_user_id: None,
            google_docs_enabled: false,
            bluebubbles_url: None,
            bluebubbles_password: None,
        };

        let user_store = UserStore::new(&config.users_db_path)?;
        let index_store = IndexStore::new(&config.task_index_path)?;

        let sender = "+1 (555) 123-4567".to_string();
        let recipient = "+1 555-999-0000".to_string();
        let raw_payload = b"From=%2B15551234567&To=%2B15559990000&Body=Hello".to_vec();
        let message = InboundMessage {
            channel: Channel::Sms,
            sender: sender.clone(),
            sender_name: None,
            recipient: recipient.clone(),
            subject: None,
            text_body: Some("Hello".to_string()),
            html_body: None,
            thread_id: "sms:test".to_string(),
            message_id: Some("SM123".to_string()),
            attachments: Vec::new(),
            reply_to: vec![sender.clone()],
            raw_payload: raw_payload.clone(),
            metadata: ChannelMetadata {
                sms_from: Some(sender.clone()),
                sms_to: Some(recipient.clone()),
                ..Default::default()
            },
        };

        process_sms_message(&config, &user_store, &index_store, &message, &raw_payload)?;

        let user = user_store.get_or_create_user("phone", &sender)?;
        let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
        let scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;

        let run_task = scheduler
            .tasks()
            .iter()
            .find_map(|task| match &task.kind {
                TaskKind::RunTask(run) => Some(run),
                _ => None,
            })
            .expect("run task created");

        assert_eq!(run_task.channel, Channel::Sms);
        assert_eq!(run_task.reply_to, vec![sender]);
        assert_eq!(run_task.reply_from.as_deref(), Some(recipient.as_str()));
        Ok(())
    }
}
