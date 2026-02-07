mod support;

use run_task_module::run_task;
use std::env;
use std::fs;
use support::{
    build_params, create_workspace, write_fake_codex, EnvGuard, FakeCodexMode, TempDir, ENV_MUTEX,
};

#[test]
#[cfg(unix)]
fn run_task_updates_existing_config_block() {
    let _lock = ENV_MUTEX.lock().unwrap();
    let temp = TempDir::new("codex_task_config_update").unwrap();
    let workspace = create_workspace(&temp.path).unwrap();

    let home_dir = temp.path.join("home");
    let bin_dir = temp.path.join("bin");
    fs::create_dir_all(&home_dir).unwrap();
    fs::create_dir_all(&bin_dir).unwrap();
    write_fake_codex(&bin_dir, FakeCodexMode::Success).unwrap();

    let config_dir = home_dir.join(".codex");
    fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.toml");
    let existing_config = r#"# preface
[other]
value = "keep"

# IMPORTANT: Use your Azure *deployment name* here (e.g., "old")
model = "old-model"
model_provider = "azure"
model_reasoning_effort = "xhigh"

[model_providers.azure]
name = "Azure OpenAI"
base_url = "https://old.azure.com/openai/v1"
env_key = "AZURE_OPENAI_API_KEY_BACKUP"
wire_api = "responses"

# footer
[extra]
value = "still"
"#;
    fs::write(&config_path, existing_config).unwrap();

    let old_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", bin_dir.display(), old_path);
    let _env = EnvGuard::set(&[
        ("HOME", home_dir.to_str().unwrap()),
        ("PATH", &new_path),
        ("AZURE_OPENAI_API_KEY_BACKUP", "test-key"),
        ("AZURE_OPENAI_ENDPOINT_BACKUP", "https://example.azure.com/"),
        ("CODEX_MODEL", "new-model"),
        ("GH_AUTH_DISABLED", "1"),
    ]);

    let mut params = build_params(&workspace);
    params.model_name = "new-model".to_string();
    let _result = run_task(&params).unwrap();

    let updated = fs::read_to_string(&config_path).unwrap();
    assert!(updated.contains("value = \"keep\""));
    assert!(updated.contains("value = \"still\""));
    assert!(updated.contains("model = \"new-model\""));
    assert!(!updated.contains("old-model"));
    assert!(updated.contains("https://example.azure.com/openai/v1"));
}
