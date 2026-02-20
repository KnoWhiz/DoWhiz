# run_task_module

Run Codex or Claude CLI to generate `reply_email_draft.html` and optional `reply_email_attachments/` based on workspace inputs.

## Usage

Requirements:
- Codex CLI installed and available on PATH (for `runner = "codex"`).
- Claude CLI installed and available on PATH (for `runner = "claude"`).
- Node.js 20 + npm.
- `playwright-cli` + Chromium (required when Codex calls browser automation skills).
- Environment variables:
  - `AZURE_OPENAI_API_KEY_BACKUP`

Install (Linux, Dockerfile parity):
```
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

Example:
```rust
use run_task_module::{run_task, RunTaskParams};
use std::path::PathBuf;

let params = RunTaskParams {
    workspace_dir: PathBuf::from("/path/to/workspace"),
    input_email_dir: PathBuf::from("incoming_email"),
    input_attachments_dir: PathBuf::from("input_attachments"),
    memory_dir: PathBuf::from("memory"),
    reference_dir: PathBuf::from("references"),
    reply_to: vec!["user@example.com".to_string()],
    model_name: "gpt-5.2-codex".to_string(),
    runner: "codex".to_string(),
    codex_disabled: false,
};

// runner: "codex" (default) or "claude"
// For Claude runs, install @anthropic-ai/claude-code and ensure
// AZURE_OPENAI_API_KEY_BACKUP is set so the Foundry settings are written.
// For Codex runs, model/base_url/sandbox are fixed in code; overrides are ignored.

let result = run_task(&params)?;
println!("Reply saved at: {}", result.reply_html_path.display());
```

## Folder structure

- `DoWhiz_service/run_task_module/src/lib.rs` : Codex CLI runner and prompt builder.
- `DoWhiz_service/run_task_module/tests/` : Basic test that verifies output file creation when Codex is disabled.

## Notes

- Input paths must be relative to `workspace_dir`.
- The module creates `reply_email_draft.html` and `reply_email_attachments/` inside the workspace.
- When `codex_disabled` is true, it writes a placeholder reply instead of calling Codex (unless `reply_to` is empty).
- When `reply_to` is empty, the prompt skips email drafting and `reply_email_draft.html` is optional.
- Skills are copied from `DoWhiz_service/skills` automatically when preparing workspaces.
- Codex runs always use `gpt-5.2-codex` + `workspace-write` sandbox + `https://knowhiz-service-openai-backup-2.openai.azure.com/openai/v1`.
- Codex exec adds `--add-dir $HOME/.config/gh` to allow GitHub CLI state writes under sandbox.

## VM Deployment Workflow

For production VM setup, HTTPS, and systemd service configuration, follow the workflow in `DoWhiz_service/README.md` under “VM Deployment Workflow”.
