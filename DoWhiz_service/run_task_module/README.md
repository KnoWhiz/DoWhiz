# run_task_module

Workspace-based task executor used by scheduler `RunTask` jobs.

Runners:
- `codex`
- `claude`

## Inputs / Outputs

Required input directories (relative to `workspace_dir`):
- `incoming_email`
- `incoming_attachments`
- `memory`
- `references`

Output files are channel-aware:
- email/google workspace channels -> `reply_email_draft.html` + `reply_email_attachments/`
- chat channels (slack/discord/telegram/sms/whatsapp/bluebubbles) -> `reply_message.txt` + `reply_attachments/`

## Execution Backend

Control via `RUN_TASK_EXECUTION_BACKEND=local|azure_aci|auto`.

`auto` behavior:
- `DEPLOY_TARGET=staging|production` -> Azure ACI
- otherwise -> local

Safety rule in code:
- local codex execution is blocked when `DEPLOY_TARGET` is `staging` or `production`.

Optional dockerized local path:
- `RUN_TASK_USE_DOCKER=1`
- `RUN_TASK_DOCKER_IMAGE=<image>`
- optional `RUN_TASK_DOCKER_REQUIRED=1`

## Required Env

Minimum practical requirement:
- `AZURE_OPENAI_API_KEY_BACKUP`
- `AZURE_OPENAI_ENDPOINT_BACKUP` (required for `codex` runner; for example `https://<resource>.openai.azure.com/`)

Common optional controls:
- `CODEX_MODEL`, `CLAUDE_MODEL`
- `RUN_TASK_TIMEOUT_SECS`
- `CODEX_SANDBOX_MODE`, `CODEX_BYPASS_SANDBOX`
- Bright Data social scraping:
  - `BRIGHT_DATA_API_KEY`
  - optional `BRIGHT_DATA_XIAOHONGSHU_COLLECTOR`
  - optional `BRIGHT_DATA_XIAOHONGSHU_TRIGGER_URL`
  - run_task forwards these vars into docker and Azure ACI Codex executions so
    shared Bright Data skills can authenticate inside remote task containers
- Google Workspace CLI (`gws`) auth:
  - preferred: `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE`
  - or components: `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE_CLIENT_ID`,
    `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE_CLIENT_SECRET`,
    `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE_REFRESH_TOKEN`,
    optional `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE_TYPE`
  - when using component keys, run_task writes
    `.secrets/google_workspace_cli_credentials.json` inside each workspace and
    injects `GOOGLE_WORKSPACE_CLI_CREDENTIALS_FILE` for local/docker/Azure ACI runs

## Example Usage

```rust
use run_task_module::{run_task, RunTaskParams};
use std::path::PathBuf;

let params = RunTaskParams {
    workspace_dir: PathBuf::from("/path/to/workspace"),
    input_email_dir: PathBuf::from("incoming_email"),
    input_attachments_dir: PathBuf::from("incoming_attachments"),
    memory_dir: PathBuf::from("memory"),
    reference_dir: PathBuf::from("references"),
    reply_to: vec!["user@example.com".to_string()],
    model_name: "gpt-5.4".to_string(),
    runner: "codex".to_string(),
    codex_disabled: false,
    channel: "email".to_string(),
    google_access_token: None,
    has_unified_account: true,
};

let out = run_task(&params)?;
println!("reply file: {}", out.reply_html_path.display());
```

## Tests

```bash
cd DoWhiz_service
cargo test -p run_task_module
```

See also:
- `DoWhiz_service/run_task_module/tests/README.md`
