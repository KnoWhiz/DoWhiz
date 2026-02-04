# Tests for run_task_module

This module includes a lightweight Cargo test harness that runs offline using a
fake Codex CLI script.

## Run tests

```
cd DoWhiz_service/run_task_module
cargo test
```

The tests spin up a temporary workspace, inject a fake `codex` executable, and
verify that `run_task` writes `reply_email_draft.html`, creates the
attachments directory, and handles error cases (missing env, missing CLI,
failed CLI, missing output, invalid paths).

## Manual smoke test (real Codex)

1) Prepare a workspace with the required folders:

```
workspace/
  incoming_email/
  incoming_attachments/
  memory/
  references/
```

2) Ensure AZURE_OPENAI_API_KEY_BACKUP and AZURE_OPENAI_ENDPOINT_BACKUP are set.
3) Run a small Rust harness that calls run_task from run_task.rs.
4) Verify the outputs:
   - reply_email_draft.html exists in the workspace root.
   - reply_email_attachments/ exists (and contains any generated attachments).

## Future automated tests

- Add a gated test that uses the real Codex CLI when RUN_CODEX_E2E=1 is set.
