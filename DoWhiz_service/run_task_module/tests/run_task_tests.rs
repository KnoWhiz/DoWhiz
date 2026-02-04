mod support;

use run_task_module::{run_task, RunTaskError, RunTaskParams};
use std::env;
use std::fs;
use std::path::Path;
use support::{
    build_params, create_workspace, write_fake_codex, EnvGuard, FakeCodexMode, TempDir, ENV_MUTEX,
};

#[test]
#[cfg(unix)]
fn run_task_success_with_fake_codex() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_success").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let home_dir = temp.path.join("home");
    let bin_dir = temp.path.join("bin");
    fs::create_dir_all(&home_dir).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_codex(&bin_dir, FakeCodexMode::Success).unwrap();

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.azure.com/"),
        ("CODEX_MODEL", "test-model"),
    ]);

    let params = build_params(&workspace);
    let result = run_task(&params).unwrap();
    assert!(result.reply_html_path.exists());
    assert!(result.reply_attachments_dir.is_dir());

    let config_path = home_dir.join(".codex").join("config.toml");
    let config = fs::read_to_string(config_path).unwrap();
    assert!(config.contains("model = \"test-model\""));
    assert!(config.contains("https://example.azure.com/openai/v1"));
}

#[test]
#[cfg(unix)]
fn run_task_reports_missing_output() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_missing_output").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let home_dir = temp.path.join("home");
    let bin_dir = temp.path.join("bin");
    fs::create_dir_all(&home_dir).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_codex(&bin_dir, FakeCodexMode::NoOutput).unwrap();

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.azure.com/"),
    ]);

    let params = build_params(&workspace);
    let err = run_task(&params).unwrap_err();
    assert!(matches!(err, RunTaskError::OutputMissing { .. }));
}

#[test]
#[cfg(unix)]
fn run_task_reports_codex_failure() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_failure").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let home_dir = temp.path.join("home");
    let bin_dir = temp.path.join("bin");
    fs::create_dir_all(&home_dir).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_codex(&bin_dir, FakeCodexMode::Fail).unwrap();

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.azure.com/"),
    ]);

    let params = build_params(&workspace);
    let err = run_task(&params).unwrap_err();
    assert!(matches!(
        err,
        RunTaskError::CodexFailed {
            status: Some(2),
            ..
        }
    ));
}

#[test]
#[cfg(unix)]
fn run_task_reports_missing_codex_cli() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_missing_cli").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let home_dir = temp.path.join("home");
    fs::create_dir_all(&home_dir).unwrap();
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", ""),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.azure.com/"),
    ]);

    let params = build_params(&workspace);
    let err = run_task(&params).unwrap_err();
    assert!(matches!(err, RunTaskError::CodexNotFound));
}

#[test]
#[cfg(unix)]
fn run_task_reports_missing_env() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_missing_env").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let home_dir = temp.path.join("home");
    let bin_dir = temp.path.join("bin");
    fs::create_dir_all(&home_dir).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_codex(&bin_dir, FakeCodexMode::Success).unwrap();

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", ""),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.azure.com/"),
    ]);

    let params = build_params(&workspace);
    let err = run_task(&params).unwrap_err();
    assert!(matches!(
        err,
        RunTaskError::MissingEnv {
            key: "AZURE_OPENAI_API_KEY_BACKUP"
        }
    ));
}

#[test]
#[cfg(unix)]
fn run_task_rejects_absolute_input_dir() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_absolute").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let request = RunTaskParams {
        workspace_dir: workspace,
        input_email_dir: Path::new("/absolute/path").to_path_buf(),
        input_attachments_dir: Path::new("incoming_attachments").to_path_buf(),
        memory_dir: Path::new("memory").to_path_buf(),
        reference_dir: Path::new("references").to_path_buf(),
        model_name: "test-model".to_string(),
        codex_disabled: false,
    };

    let err = run_task(&request).unwrap_err();
    assert!(matches!(err, RunTaskError::InvalidPath { .. }));
}

#[test]
fn run_task_codex_disabled_writes_placeholder() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_disabled").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let mut params = build_params(&workspace);
    params.codex_disabled = true;

    let result = run_task(&params).unwrap();
    let html = fs::read_to_string(&result.reply_html_path).unwrap();
    assert!(html.contains("Codex disabled"));
    assert!(result.reply_attachments_dir.is_dir());
}
