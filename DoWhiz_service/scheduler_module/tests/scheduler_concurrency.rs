use scheduler_module::employee_config::{EmployeeDirectory, EmployeeProfile};
use scheduler_module::index_store::IndexStore;
use scheduler_module::service::{run_server, ServiceConfig, DEFAULT_INBOUND_BODY_MAX_BYTES};
use scheduler_module::user_store::UserStore;
use scheduler_module::{ModuleExecutor, RunTaskTask, Scheduler, TaskKind};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

const TASK_COUNT: usize = 20;
const CONCURRENCY_LIMIT: usize = 10;
const TASK_SLEEP_SECS: f64 = 0.5;

fn test_employee_directory() -> (EmployeeProfile, EmployeeDirectory) {
    let addresses = vec!["service@example.com".to_string()];
    let address_set: HashSet<String> =
        addresses.iter().map(|value| value.to_ascii_lowercase()).collect();
    let employee = EmployeeProfile {
        id: "test-employee".to_string(),
        display_name: None,
        runner: "codex".to_string(),
        model: None,
        addresses: addresses.clone(),
        address_set: address_set.clone(),
        agents_path: None,
        claude_path: None,
        soul_path: None,
        skills_dir: None,
    };
    let mut employee_by_id = HashMap::new();
    employee_by_id.insert(employee.id.clone(), employee.clone());
    let mut service_addresses = HashSet::new();
    service_addresses.extend(address_set);
    let directory = EmployeeDirectory {
        employees: vec![employee.clone()],
        employee_by_id,
        default_employee_id: Some(employee.id.clone()),
        service_addresses,
    };
    (employee, directory)
}

#[test]
fn scheduler_parallelism_reduces_wall_clock_time() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt().with_target(false).try_init();

    let temp = TempDir::new()?;
    let home_dir = temp.path().join("home");
    fs::create_dir_all(&home_dir)?;
    env::set_var("HOME", &home_dir);
    env::set_var("AZURE_OPENAI_API_KEY_BACKUP", "test-key");
    env::set_var("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.com");
    env::set_var("CODEX_TEST_SLEEP_SECS", format!("{TASK_SLEEP_SECS}"));
    env::set_var("GH_AUTH_DISABLED", "1");
    env::set_var("RUN_TASK_DOCKER_IMAGE", "");
    env::set_var("RUN_TASK_USE_DOCKER", "0");

    let fake_bin_dir = temp.path().join("bin");
    fs::create_dir_all(&fake_bin_dir)?;
    write_fake_codex(&fake_bin_dir)?;
    let path_env = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", format!("{}:{}", fake_bin_dir.display(), path_env));

    let workspace_root = temp.path().join("workspaces");
    let users_root = temp.path().join("users");
    let state_dir = temp.path().join("state");
    fs::create_dir_all(&workspace_root)?;
    fs::create_dir_all(&users_root)?;
    fs::create_dir_all(&state_dir)?;

    let user_store = UserStore::new(state_dir.join("users.db"))?;
    let index_store = IndexStore::new(state_dir.join("task_index.db"))?;

    for i in 0..TASK_COUNT {
        let email = format!("user{}@example.com", i);
        let user = user_store.get_or_create_user(&email)?;
        let paths = user_store.user_paths(&users_root, &user.user_id);
        user_store.ensure_user_dirs(&paths)?;
        let workspace = paths.workspaces_root.join(format!("workspace_{i}"));
        prepare_workspace(&workspace)?;

        let run_task = RunTaskTask {
            workspace_dir: workspace,
            input_email_dir: PathBuf::from("incoming_email"),
            input_attachments_dir: PathBuf::from("incoming_attachments"),
            memory_dir: PathBuf::from("memory"),
            reference_dir: PathBuf::from("references"),
            model_name: "gpt-5.2-codex".to_string(),
            runner: "codex".to_string(),
            codex_disabled: false,
            reply_to: Vec::new(),
            reply_from: None,
            archive_root: None,
            thread_id: None,
            thread_epoch: None,
            thread_state_path: None,
            channel: scheduler_module::channel::Channel::default(),
        };

        let mut scheduler = Scheduler::load(&paths.tasks_db_path, ModuleExecutor::default())?;
        scheduler.add_one_shot_in(Duration::from_secs(0), TaskKind::RunTask(run_task))?;
        index_store.sync_user_tasks(&user.user_id, scheduler.tasks())?;
    }

    let port = pick_free_port()?;
    let (employee_profile, employee_directory) = test_employee_directory();
    let config = ServiceConfig {
        host: "127.0.0.1".to_string(),
        port,
        employee_id: employee_profile.id.clone(),
        employee_config_path: temp.path().join("employee.toml"),
        employee_profile,
        employee_directory,
        workspace_root,
        scheduler_state_path: state_dir.join("tasks.db"),
        processed_ids_path: state_dir.join("postmark_processed_ids.txt"),
        users_root: users_root.clone(),
        users_db_path: state_dir.join("users.db"),
        task_index_path: state_dir.join("task_index.db"),
        codex_model: "gpt-5.2-codex".to_string(),
        codex_disabled: false,
        scheduler_poll_interval: Duration::from_millis(100),
        scheduler_max_concurrency: CONCURRENCY_LIMIT,
        scheduler_user_max_concurrency: 3,
        inbound_body_max_bytes: DEFAULT_INBOUND_BODY_MAX_BYTES,
        skills_source_dir: None,
        slack_bot_token: None,
        slack_bot_user_id: None,
    };

    let rt = Runtime::new()?;
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let server_handle = rt.spawn(async move {
        run_server(config, async {
            let _ = shutdown_rx.await;
        })
        .await
    });

    rt.block_on(async {
        tokio::time::sleep(Duration::from_millis(200)).await;
    });

    let start = Instant::now();
    wait_for_all_tasks_complete(&user_store, &users_root, Duration::from_secs(20))?;
    let elapsed = start.elapsed();

    let sequential = TASK_COUNT as f64 * TASK_SLEEP_SECS;
    let max_allowed = Duration::from_secs_f64(sequential * 0.6);
    assert!(
        elapsed < max_allowed,
        "expected parallel speedup: elapsed {:?} >= {:?}",
        elapsed,
        max_allowed
    );

    let _ = shutdown_tx.send(());
    match rt.block_on(async { server_handle.await }) {
        Ok(Ok(())) => {}
        Ok(Err(err)) => return Err(err),
        Err(err) => return Err(Box::new(err)),
    }

    Ok(())
}

fn prepare_workspace(workspace: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(workspace)?;
    for dir in [
        "incoming_email",
        "incoming_attachments",
        "memory",
        "references",
    ] {
        fs::create_dir_all(workspace.join(dir))?;
    }
    Ok(())
}

fn write_fake_codex(dir: &Path) -> Result<PathBuf, std::io::Error> {
    let script = dir.join("codex");
    let body = r#"#!/bin/sh
sleep "${CODEX_TEST_SLEEP_SECS:-0.5}"
cat > reply_email_draft.html <<'EOF'
<html><body>ok</body></html>
EOF
"#;
    fs::write(&script, body)?;
    let mut perms = fs::metadata(&script)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms)?;
    Ok(script)
}

fn pick_free_port() -> Result<u16, std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

fn wait_for_all_tasks_complete(
    user_store: &UserStore,
    users_root: &Path,
    timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    loop {
        let user_ids = user_store.list_user_ids()?;
        let mut all_done = !user_ids.is_empty();
        for user_id in user_ids {
            let paths = user_store.user_paths(users_root, &user_id);
            if !paths.tasks_db_path.exists() {
                all_done = false;
                break;
            }
            let scheduler = Scheduler::load(&paths.tasks_db_path, ModuleExecutor::default())?;
            if scheduler.tasks().is_empty()
                || scheduler
                    .tasks()
                    .iter()
                    .any(|task| task.enabled || task.last_run.is_none())
            {
                all_done = false;
                break;
            }
        }

        if all_done {
            return Ok(());
        }
        if start.elapsed() >= timeout {
            return Err("timed out waiting for tasks to complete".into());
        }
        thread::sleep(Duration::from_millis(100));
    }
}
