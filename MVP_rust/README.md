# MVP Rust Email Service

This Rust service receives inbound Postmark webhooks, schedules a RunTask job,
then schedules a SendEmail job and sends the reply via Postmark.

## Prereqs
- Rust toolchain
- `codex` CLI on your PATH (unless `CODEX_DISABLED=1`)
- `.env` includes:
  - `POSTMARK_SERVER_TOKEN`
  - `OUTBOUND_FROM` (optional, defaults to `oliver@dowhiz.com`)
  - `AZURE_OPENAI_API_KEY_BACKUP` and `AZURE_OPENAI_ENDPOINT_BACKUP` (required when Codex is enabled)

## Step-by-step: start the Rust service and send real email

1) Start the Rust service (Terminal 1):
```
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

2) Expose the service with ngrok (Terminal 2):
```
ngrok http 9001
```

3) Set the Postmark inbound hook to the **new** ngrok URL (Terminal 3):
```
python -m mvp_python.email_pipeline.set_postmark_inbound_hook \
  --hook-url https://YOUR-NEW-NGROK-URL.ngrok-free.dev/postmark/inbound
```

4) Send an email to:
```
oliver@dowhiz.com
```

5) Watch logs for task execution. Outputs appear under:
- `.workspace/run_task/workspaces/<message_id>/reply_email_draft.html`
- `.workspace/run_task/workspaces/<message_id>/reply_email_attachments/`
- Scheduler state: `.workspace/run_task/state/tasks.json`

## Environment knobs
- `RUST_SERVICE_HOST` / `RUST_SERVICE_PORT`
- `WORKSPACE_ROOT` (default: `.workspace/run_task/workspaces`)
- `SCHEDULER_STATE_PATH` (default: `.workspace/run_task/state/tasks.json`)
- `PROCESSED_IDS_PATH` (default: `.workspace/run_task/state/postmark_processed_ids.txt`)
- `CODEX_MODEL`
- `CODEX_DISABLED=1` to bypass Codex CLI

## Real email end-to-end test (Rust)

This test starts the Rust service, sets the Postmark inbound hook to your public
URL, sends a real email to the Postmark inbound address, and verifies the reply.

Prereqs:
- `RUST_SERVICE_LIVE_TEST=1`
- `POSTMARK_SERVER_TOKEN`
- `POSTMARK_INBOUND_HOOK_URL` (public URL, e.g. ngrok)
- `POSTMARK_TEST_FROM` (your inbox for replies)

Run:
```
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK-URL.ngrok-free.dev \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

Set `RUN_CODEX_E2E=1` if you want the test to call Codex; otherwise it uses the
Codex-disabled placeholder reply.
