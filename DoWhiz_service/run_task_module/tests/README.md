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

## Optional real Codex E2E test (RUN_CODEX_E2E)

When `RUN_CODEX_E2E=1` is set, `run_task_tests` runs a live Codex CLI test.

Prereqs:
- `codex` CLI installed.
- `AZURE_OPENAI_API_KEY_BACKUP` and `AZURE_OPENAI_ENDPOINT_BACKUP` set.
- Optional: `CODEX_MODEL` to override the default model.

Run:
```
RUN_CODEX_E2E=1 \
AZURE_OPENAI_API_KEY_BACKUP=... \
AZURE_OPENAI_ENDPOINT_BACKUP=... \
cargo test -p run_task_module --test run_task_tests -- --nocapture
```

## Manual smoke test (real Codex)

Prereqs (Dockerfile parity):
```
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

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
   - Skills are copied from `DoWhiz_service/skills` automatically when preparing workspaces.

## VM Deployment Workflow

For production VM setup, HTTPS, and systemd service configuration, follow the workflow in `DoWhiz_service/README.md` under “VM Deployment Workflow”.
