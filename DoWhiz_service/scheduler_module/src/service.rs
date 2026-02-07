use axum::body::Bytes;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
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
use tokio::sync::Mutex as AsyncMutex;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::index_store::{IndexStore, TaskRef};
use crate::thread_state::{bump_thread_state, default_thread_state_path};
use crate::user_store::{extract_emails, UserStore};
use crate::{
    ModuleExecutor, RunTaskTask, Schedule, ScheduledTask, Scheduler, SchedulerError, TaskKind,
};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: u16,
    pub workspace_root: PathBuf,
    pub scheduler_state_path: PathBuf,
    pub processed_ids_path: PathBuf,
    pub users_root: PathBuf,
    pub users_db_path: PathBuf,
    pub task_index_path: PathBuf,
    pub codex_model: String,
    pub codex_disabled: bool,
    pub scheduler_poll_interval: Duration,
    pub scheduler_max_concurrency: usize,
    pub scheduler_user_max_concurrency: usize,
    pub skills_source_dir: Option<PathBuf>,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, BoxError> {
        dotenvy::dotenv().ok();

        let host = env::var("RUST_SERVICE_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("RUST_SERVICE_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(9001);

        let workspace_root = resolve_path(
            env::var("WORKSPACE_ROOT")
                .unwrap_or_else(|_| ".workspace/run_task/workspaces".to_string()),
        )?;
        let scheduler_state_path = resolve_path(
            env::var("SCHEDULER_STATE_PATH")
                .unwrap_or_else(|_| ".workspace/run_task/state/tasks.db".to_string()),
        )?;
        let processed_ids_path =
            resolve_path(env::var("PROCESSED_IDS_PATH").unwrap_or_else(|_| {
                ".workspace/run_task/state/postmark_processed_ids.txt".to_string()
            }))?;
        let users_root = resolve_path(
            env::var("USERS_ROOT").unwrap_or_else(|_| ".workspace/run_task/users".to_string()),
        )?;
        let users_db_path = resolve_path(
            env::var("USERS_DB_PATH")
                .unwrap_or_else(|_| ".workspace/run_task/state/users.db".to_string()),
        )?;
        let task_index_path = resolve_path(
            env::var("TASK_INDEX_PATH")
                .unwrap_or_else(|_| ".workspace/run_task/state/task_index.db".to_string()),
        )?;
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
        let skills_source_dir = Some(repo_skills_source_dir());

        Ok(Self {
            host,
            port,
            workspace_root,
            scheduler_state_path,
            processed_ids_path,
            users_root,
            users_db_path,
            task_index_path,
            codex_model,
            codex_disabled,
            scheduler_poll_interval,
            scheduler_max_concurrency,
            scheduler_user_max_concurrency,
            skills_source_dir,
        })
    }
}

#[derive(Clone)]
struct AppState {
    config: Arc<ServiceConfig>,
    dedupe_store: Arc<AsyncMutex<ProcessedMessageStore>>,
    user_store: Arc<UserStore>,
    index_store: Arc<IndexStore>,
}

#[derive(Default)]
struct SchedulerClaims {
    running_tasks: HashSet<String>,
    running_users: HashMap<String, usize>,
}

enum ClaimResult {
    Claimed,
    UserBusy,
    TaskBusy,
}

impl SchedulerClaims {
    fn try_claim(&mut self, task_ref: &TaskRef, user_limit: usize) -> ClaimResult {
        let active = self
            .running_users
            .get(&task_ref.user_id)
            .copied()
            .unwrap_or(0);
        if active >= user_limit {
            return ClaimResult::UserBusy;
        }
        if self.running_tasks.contains(&task_ref.task_id) {
            return ClaimResult::TaskBusy;
        }
        self.running_users
            .insert(task_ref.user_id.clone(), active + 1);
        self.running_tasks.insert(task_ref.task_id.clone());
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
    let config = Arc::new(config);
    let dedupe_store = ProcessedMessageStore::load(&config.processed_ids_path)?;
    let user_store = Arc::new(UserStore::new(&config.users_db_path)?);
    let index_store = Arc::new(IndexStore::new(&config.task_index_path)?);
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
                                claims.try_claim(&task_ref, scheduler_user_max_concurrency)
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
    let state = AppState {
        config: config.clone(),
        dedupe_store: Arc::new(AsyncMutex::new(dedupe_store)),
        user_store,
        index_store,
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
        .route("/postmark/inbound", post(postmark_inbound))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;
    scheduler_stop.store(true, Ordering::Relaxed);
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
    let user_paths = user_store.user_paths(&config.users_root, &task_ref.user_id);
    let mut scheduler = Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default())?;
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
        let refreshed_scheduler =
            Scheduler::load(&user_paths.tasks_db_path, ModuleExecutor::default());
        match refreshed_scheduler {
            Ok(refreshed_scheduler) => {
                index_store.sync_user_tasks(&task_ref.user_id, refreshed_scheduler.tasks())?;
                let summary = summarize_tasks(refreshed_scheduler.tasks(), Utc::now());
                log_task_snapshot(&task_ref.user_id, "after_execute", &summary);
            }
            Err(err) => {
                warn!(
                    "scheduler reload failed after execute task_id={} user_id={} error={}",
                    task_ref.task_id, task_ref.user_id, err
                );
                index_store.sync_user_tasks(&task_ref.user_id, scheduler.tasks())?;
                let summary = summarize_tasks(scheduler.tasks(), Utc::now());
                log_task_snapshot(&task_ref.user_id, "after_execute", &summary);
            }
        }
    } else {
        warn!(
            "scheduler task skipped task_id={} user_id={} status={}",
            task_ref.task_id, task_ref.user_id, status_label
        );
    }
    Ok(())
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn postmark_inbound(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    let payload: PostmarkInbound = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json(json!({"status": "bad_json"})));
        }
    };

    info!("postmark inbound payload received");

    let message_ids = extract_message_ids(&payload, &body);
    let is_new = {
        let mut store = state.dedupe_store.lock().await;
        match store.mark_if_new(&message_ids) {
            Ok(value) => value,
            Err(err) => {
                error!("dedupe store error: {err}");
                true
            }
        }
    };

    if !is_new {
        return (StatusCode::OK, Json(json!({"status": "duplicate"})));
    }

    let config = state.config.clone();
    let user_store = state.user_store.clone();
    let index_store = state.index_store.clone();
    let payload_clone = payload.clone();
    let body_bytes = body.to_vec();
    tokio::task::spawn_blocking(move || {
        if let Err(err) = process_inbound_payload(
            &config,
            &user_store,
            &index_store,
            &payload_clone,
            &body_bytes,
        ) {
            error!("failed to process inbound payload: {err}");
        }
    });

    (StatusCode::OK, Json(json!({"status": "accepted"})))
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
    if is_blacklisted_sender(sender) {
        info!("skipping blacklisted sender: {}", sender);
        return Ok(());
    }
    let user_email = payload.from.as_deref().unwrap_or("").trim();
    let user_email = extract_emails(user_email)
        .into_iter()
        .next()
        .ok_or_else(|| "missing sender email".to_string())?;
    let user = user_store.get_or_create_user(&user_email)?;
    let user_paths = user_store.user_paths(&config.users_root, &user.user_id);
    user_store.ensure_user_dirs(&user_paths)?;

    let thread_key = thread_key(payload, raw_payload);
    let workspace = ensure_thread_workspace(
        &user_paths,
        &user.user_id,
        &thread_key,
        config.skills_source_dir.as_deref(),
    )?;
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

    let reply_target = payload
        .reply_to
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| payload.from.as_deref())
        .unwrap_or("");
    let to_list = split_recipients(reply_target);
    if to_list.is_empty() {
        return Err("missing reply recipient".into());
    }

    let run_task = RunTaskTask {
        workspace_dir: workspace.clone(),
        input_email_dir: PathBuf::from("incoming_email"),
        input_attachments_dir: PathBuf::from("incoming_attachments"),
        memory_dir: PathBuf::from("memory"),
        reference_dir: PathBuf::from("references"),
        model_name: config.codex_model.clone(),
        codex_disabled: config.codex_disabled,
        reply_to: to_list.clone(),
        archive_root: Some(user_paths.mail_root.clone()),
        thread_id: Some(thread_key.clone()),
        thread_epoch: Some(thread_state.epoch),
        thread_state_path: Some(thread_state_path.clone()),
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
        TaskKind::SendEmail(_) => "send_email",
        TaskKind::RunTask(_) => "run_task",
        TaskKind::Noop => "noop",
    }
}

fn is_blacklisted_sender(sender: &str) -> bool {
    if sender.is_empty() {
        return false;
    }
    let mut matched = false;
    let addresses = extract_emails(sender);
    for address in addresses {
        if is_blacklisted_address(&address) {
            matched = true;
            break;
        }
    }
    matched
}

fn is_blacklisted_address(address: &str) -> bool {
    matches!(address, "agent@dowhiz.com" | "oliver@dowhiz.com")
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

fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
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

fn ensure_thread_workspace(
    user_paths: &crate::user_store::UserPaths,
    user_id: &str,
    thread_key: &str,
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

    // Copy skills to workspace for Codex CLI
    if let Some(skills_src) = skills_source_dir {
        let agents_skills_dir = workspace.join(".agents").join("skills");
        if let Err(err) = copy_skills_directory(skills_src, &agents_skills_dir) {
            error!("failed to copy skills to workspace: {}", err);
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

fn cancel_pending_thread_tasks<E: crate::TaskExecutor>(
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
            TaskKind::SendEmail(send) => {
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

fn write_thread_history(incoming_email: &Path, incoming_attachments: &Path) -> Result<(), BoxError> {
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
    output.push_str(
        "Auto-generated from incoming_email/entries. Latest entry is last.\n\n",
    );

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

fn render_email_html(payload: &PostmarkInbound) -> String {
    if let Some(html) = payload
        .html_body
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        return html.to_string();
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
    value
        .split(|ch| ch == ',' || ch == ';')
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
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

fn resolve_path(raw: String) -> Result<PathBuf, io::Error> {
    let path = PathBuf::from(raw);
    if path.is_absolute() {
        Ok(path)
    } else {
        let cwd = env::current_dir()?;
        Ok(cwd.join(path))
    }
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

fn extract_message_ids(payload: &PostmarkInbound, raw_payload: &[u8]) -> Vec<String> {
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    if let Some(header_id) = payload.header_message_id().and_then(normalize_message_id) {
        if seen.insert(header_id.clone()) {
            ids.push(header_id);
        }
    }
    if let Some(message_id) = payload
        .message_id
        .as_ref()
        .and_then(|value| normalize_message_id(value))
    {
        if seen.insert(message_id.clone()) {
            ids.push(message_id);
        }
    }
    let fallback = format!("{:x}", md5::compute(raw_payload));
    if seen.insert(fallback.clone()) {
        ids.push(fallback);
    }
    ids
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
        fs::write(incoming_email.join("email.html"), "<pre>Hello</pre>")
            .expect("email.html");
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
        let workspace = ensure_thread_workspace(&user_paths, "user123", &thread, None)
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
}
