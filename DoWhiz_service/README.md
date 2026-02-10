# MVP Rust Email Service

This Rust service receives inbound Postmark webhooks, schedules a RunTask job,
then schedules a SendEmail job and sends the reply via Postmark.

## Prereqs
- Rust toolchain
- System libs: `libsqlite3`, `libssl`, `pkg-config`, `ca-certificates`
- Node.js 20 + npm
- `codex` CLI on your PATH (only required for local execution; optional when `RUN_TASK_DOCKER_IMAGE` is set)
- `playwright-cli` + Chromium (required for browser automation skills)
- `.env` includes:
  - `POSTMARK_SERVER_TOKEN`
  - `OUTBOUND_FROM` (optional, defaults to `oliver@dowhiz.com`)
  - `AZURE_OPENAI_API_KEY_BACKUP` and `AZURE_OPENAI_ENDPOINT_BACKUP` (required when Codex is enabled)
  - `GITHUB_USERNAME` + `GITHUB_PERSONAL_ACCESS_TOKEN` (optional; enables Codex/agent GitHub access via `GH_TOKEN`/`GITHUB_TOKEN` + git askpass)
  - `RUN_TASK_DOCKER_IMAGE` (run each task inside a disposable Docker container; use `dowhiz-service` for the repo image)
  - `RUN_TASK_DOCKER_AUTO_BUILD=1` to auto-build the image when missing (set `0` to disable)

### Install dependencies (Dockerfile parity)
Linux (Debian/Ubuntu):
```
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

Optional (match Dockerfile's chrome-channel lookup used by E2E):
```
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.cache/ms-playwright"
chromium_path="$(ls -d "$PLAYWRIGHT_BROWSERS_PATH"/chromium-*/chrome-linux/chrome | head -n1)"
sudo mkdir -p /opt/google/chrome
sudo ln -sf "$chromium_path" /opt/google/chrome/chrome
```

macOS (Homebrew):
```
brew install node@20 openssl@3 sqlite pkg-config
npm install -g @openai/codex@latest @playwright/cli@latest
npx playwright install chromium
```

Skills are copied from `DoWhiz_service/skills` automatically when preparing workspaces.

## Per-task Docker execution
When `RUN_TASK_DOCKER_IMAGE` is set, each RunTask spins up a fresh container,
mounts the task workspace at `/workspace`, runs Codex inside the container, and
removes the container when done. If the image is missing, the service will
auto-build it (unless `RUN_TASK_DOCKER_AUTO_BUILD=0`). You can override the
build inputs with `RUN_TASK_DOCKERFILE` and `RUN_TASK_DOCKER_BUILD_CONTEXT`.

## Docker (production image)
Build the image from the repo root and run it with the same `.env` file mounted
so `dotenv` can load it inside the container:
```
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace:/app/.workspace \
  dowhiz-service
```

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
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound
```

4) Send an email to:
```
oliver@dowhiz.com
```

5) Watch logs for task execution. Outputs appear under:
- `.workspace/run_task/workspaces/<message_id>/reply_email_draft.html`
- `.workspace/run_task/workspaces/<message_id>/reply_email_attachments/`
- Scheduler state: `.workspace/run_task/state/tasks.db`

## Environment knobs
- `RUST_SERVICE_HOST` / `RUST_SERVICE_PORT`
- `WORKSPACE_ROOT` (default: `.workspace/run_task/workspaces`)
- `SCHEDULER_STATE_PATH` (default: `.workspace/run_task/state/tasks.db`)
- `PROCESSED_IDS_PATH` (default: `.workspace/run_task/state/postmark_processed_ids.txt`)
- `USERS_ROOT` (default: `.workspace/run_task/users`)
- `USERS_DB_PATH` (default: `.workspace/run_task/state/users.db`)
- `TASK_INDEX_PATH` (default: `.workspace/run_task/state/task_index.db`)
- `SCHEDULER_POLL_INTERVAL_SECS` (default: `1`)
- `SCHEDULER_MAX_CONCURRENCY` (default: `10`)
- `SCHEDULER_USER_MAX_CONCURRENCY` (default: `3`)
- `CODEX_MODEL`
- `CODEX_DISABLED=1` to bypass Codex CLI
- `RUN_TASK_DOCKER_IMAGE` to enable per-task containers
- `RUN_TASK_DOCKER_AUTO_BUILD=1` to auto-build missing images
- `RUN_TASK_DOCKERFILE` to override the Dockerfile path
- `RUN_TASK_DOCKER_BUILD_CONTEXT` to override the docker build context directory
- `RUN_TASK_DOCKER_NETWORK` to set Docker's network mode (for example, `host`)
- `RUN_TASK_DOCKER_DNS` to override Docker DNS servers (comma/space-separated)
- `RUN_TASK_DOCKER_DNS_SEARCH` to add DNS search domains (comma/space-separated)
- Inbound blacklist: `little-bear@dowhiz.com`, `agent@dowhiz.com`, `oliver@dowhiz.com`, `mini-mouse@dowhiz.com`, `maggie@dowhiz.com` are ignored (display names and `+tag` aliases are normalized).

## Database files
- `DoWhiz_service/.workspace/run_task/state/users.db`: user registry. Table `users(id, email, created_at, last_seen_at)` stores normalized email, creation time, and last activity time (RFC3339 UTC). `last_seen_at` updates on inbound email.
- `DoWhiz_service/.workspace/run_task/state/task_index.db`: global task index for due work. Table `task_index(task_id, user_id, next_run, enabled)` plus indexes on `next_run` and `user_id`. This is a derived index synced from each user's `tasks.db` and used by the scheduler thread to query due tasks efficiently.
- `DoWhiz_service/.workspace/run_task/users/<user_id>/state/tasks.db`: per-user scheduler store (SQLite with foreign keys on). Key tables:
  - `tasks(id, kind, enabled, created_at, last_run, schedule_type, cron_expression, next_run, run_at)` holds scheduling metadata. `schedule_type` is `cron` or `one_shot`; cron uses `cron_expression` + `next_run`, one-shot uses `run_at`.
  - `send_email_tasks(task_id, subject, html_path, attachments_dir, in_reply_to, references_header[, archive_root])` stores email task payloads. `archive_root` may be added by auto-migration.
  - `send_email_recipients(id, task_id, recipient_type, address)` stores `to`/`cc`/`bcc` recipients.
  - `run_task_tasks(task_id, workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, codex_disabled, reply_to[, archive_root])` stores RunTask parameters. `reply_to` is newline-separated; `archive_root` may be added by auto-migration.
  - `task_executions(id, task_id, started_at, finished_at, status, error_message)` records execution history and errors.

## Past email hydration
Each new workspace populates `references/past_emails/` from the user archive under
`.workspace/run_task/users/<user_id>/mail`. The hydrator copies `incoming_email/`
and any attachments <= 50MB; larger attachments are referenced via
`attachments_manifest.json` (set `*.azure_url` sidecar files to supply the Azure
blob URL if needed). Outgoing agent replies are archived after successful
`send_email` execution and appear in `past_emails` with `direction: "outbound"`.

Manual run:
```
cargo run -p scheduler_module --bin hydrate_past_emails -- \
  --archive-root .workspace/run_task/users/<user_id>/mail \
  --references-dir /path/to/workspace/references \
  --user-id <user_id>
```

### `index.json` schema
Entry directories are named `YYYY-MM-DD_<action>_<topic>_<shortid>`.
`direction` is `"inbound"` or `"outbound"`.
```
{
  "version": 1,
  "generated_at": "RFC3339 timestamp",
  "user_id": "uuid",
  "entries": [
    {
      "entry_id": "message-id",
      "display_name": "2026-02-03_message_archive-hello_abc123",
      "path": "2026-02-03_message_archive-hello_abc123",
      "direction": "inbound",
      "subject": "Archive hello",
      "from": "Sender <sender@example.com>",
      "to": "Recipient <recipient@example.com>",
      "cc": "",
      "bcc": "",
      "date": "RFC3339 timestamp",
      "message_id": "message-id",
      "attachments_manifest": "2026-02-03_message_archive-hello_abc123/attachments_manifest.json",
      "attachments_count": 1,
      "large_attachments_count": 0
    }
  ]
}
```

### `attachments_manifest.json` schema
```
{
  "version": 1,
  "generated_at": "RFC3339 timestamp",
  "message_id": "message-id",
  "attachments": [
    {
      "file_name": "report.pdf",
      "original_name": "Report.pdf",
      "content_type": "application/pdf",
      "size_bytes": 12345,
      "storage": "local",
      "relative_path": "incoming_attachments/report.pdf",
      "azure_blob_url": null
    }
  ]
}
```

## Scheduled follow-ups
If the agent needs to send a follow-up later, it should emit a schedule block to stdout at the end
of its response. The scheduler parses the block and stores follow-up tasks in SQLite.

Example schedule block:
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"Quick reminder","html_path":"reminder_email_draft.html","attachments_dir":"reminder_email_attachments","to":["you@example.com"],"cc":[],"bcc":[]}]
SCHEDULED_TASKS_JSON_END
```

## Real email end-to-end test (Rust)

This test starts the Rust service, sets the Postmark inbound hook to your public
URL, sends a real email to the Postmark inbound address, and verifies the reply.
It sends a single inbound email and expects a single reply (no batch sends).

Prereqs:
- `RUST_SERVICE_LIVE_TEST=1`
- `POSTMARK_SERVER_TOKEN`
- `POSTMARK_INBOUND_HOOK_URL` (public URL, e.g. ngrok base or full `/postmark/inbound` endpoint)
- `POSTMARK_TEST_FROM` (your inbox for replies)
- `RUST_SERVICE_TEST_PORT` (optional, defaults to `9001`; ensure ngrok forwards to this port)

Run:
```
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK-URL.ngrok-free.dev \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

Set `RUN_CODEX_E2E=1` if you want the test to call Codex; otherwise it uses the
Codex-disabled placeholder reply.
