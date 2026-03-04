# Tests for run_task_module

This module includes an offline Cargo test harness built around fake Codex/Claude
executables plus an optional live Codex E2E path.

## Run tests

```
cd DoWhiz_service/run_task_module
cargo test
```

The tests spin up temporary workspaces and verify:
- channel-aware output files (`reply_email_draft.html` vs `reply_message.txt`)
- config block generation/update
- sandbox/bypass flag wiring (`--yolo`, `sandbox`)
- timeout and runtime-failure handling
- env propagation for GitHub and x402 keys
- missing env/CLI/output/path validation errors

## Optional real Codex E2E test (RUN_CODEX_E2E)

When `RUN_CODEX_E2E=1` is set, `run_task_tests` runs a live Codex CLI test.

Prereqs:
- `codex` CLI installed.
- `AZURE_OPENAI_API_KEY_BACKUP` set.

Run:
```
RUN_CODEX_E2E=1 \
AZURE_OPENAI_API_KEY_BACKUP=... \
cargo test -p run_task_module --test run_task_tests -- --nocapture
```

## Manual smoke test (real Codex)

Prereqs (Dockerfile parity):
```
sudo apt-get update
sudo apt-get install -y ca-certificates libssl-dev pkg-config curl
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

2) Ensure AZURE_OPENAI_API_KEY_BACKUP is set.
3) Run a small Rust harness that calls run_task from run_task_module (see `src/run_task/`).
4) Verify the outputs:
   - email-style channels: `reply_email_draft.html` + `reply_email_attachments/`
   - chat-style channels: `reply_message.txt` + `reply_attachments/`
   - Skills are copied from `DoWhiz_service/skills` automatically when preparing workspaces.

## Production Deployment

For Azure deployment (Rust Gateway + Service Bus + Blob + workers) or VM-based setups, follow the workflows in `DoWhiz_service/README.md` under “Azure Deployment (Rust Gateway + Service Bus + Blob + Workers)” and “VM Deployment (Gateway + ngrok)”.
