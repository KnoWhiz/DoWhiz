# run_task_module

Run Codex CLI to generate `reply_email_draft.html` and optional `reply_email_attachments/` based on workspace inputs.

## Usage

Requirements:
- Codex CLI installed and available on PATH.
- Environment variables:
  - `AZURE_OPENAI_API_KEY_BACKUP`
  - `AZURE_OPENAI_ENDPOINT_BACKUP`

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
    model_name: "gpt-5.2-codex".to_string(),
    codex_disabled: false,
};

let result = run_task(&params)?;
println!("Reply saved at: {}", result.reply_html_path.display());
```

## Folder structure

- `DoWhiz_service/run_task_module/src/lib.rs` : Codex CLI runner and prompt builder.
- `DoWhiz_service/run_task_module/tests/` : Basic test that verifies output file creation when Codex is disabled.

## Notes

- Input paths must be relative to `workspace_dir`.
- The module creates `reply_email_draft.html` and `reply_email_attachments/` inside the workspace.
- When `codex_disabled` is true, it writes a placeholder reply instead of calling Codex.
