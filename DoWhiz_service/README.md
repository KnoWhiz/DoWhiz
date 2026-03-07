# DoWhiz_service

Rust backend for DoWhiz digital employees.

This service layer currently runs as:
- `inbound_gateway`: ingress/webhook router + dedupe + raw payload storage + queue enqueue
- `rust_service`: queue consumer + scheduler + task execution + outbound replies

## Table of Contents

- [1) Architecture](#1-architecture)
- [2) Components and Binaries](#2-components-and-binaries)
- [3) Config Files](#3-config-files)
- [4) Environment Variables](#4-environment-variables)
- [5) Local Run Workflows](#5-local-run-workflows)
- [6) Staging / Production Deployment](#6-staging--production-deployment)
- [7) Testing](#7-testing)
- [8) Runtime State and Data Stores](#8-runtime-state-and-data-stores)
- [9) Troubleshooting](#9-troubleshooting)

## 1) Architecture

### 1.1 Runtime boundary

- Gateway handles inbound HTTP/webhook and Discord gateway ingress.
- Worker does **not** host inbound webhook routes; it consumes ingestion queue messages.
- Both gateway and worker expose account/auth routes (`/auth/*`) and agent market routes.
- Billing routes (`/billing/*`) are mounted on worker only when Stripe config exists.

### 1.2 End-to-end flow

```text
Inbound (email/slack/discord/sms/telegram/whatsapp/google workspace/bluebubbles)
  -> inbound_gateway
  -> build route + dedupe key + raw payload ref
  -> ingestion queue
  -> rust_service worker claim_next(employee_id)
  -> process channel-specific inbound
  -> enqueue RunTask
  -> run_task_module (codex/claude)
  -> SendReply task(s) + optional follow-ups
  -> outbound channel adapter
```

### 1.3 Queue and storage behavior

- Ingestion queue backend resolver defaults to `postgres`.
- `inbound_gateway` enforces `INGESTION_QUEUE_BACKEND=servicebus` (or alias equivalent).
- Raw payload storage defaults to Supabase; Azure Blob backend is recommended for gateway production.
- Scheduler/user/index state is Mongo-backed.

## 2) Components and Binaries

Cargo workspace members:
- `scheduler_module`
- `run_task_module`
- `send_emails_module`

Key binaries (from `scheduler_module/src/bin`):

| Binary | Purpose |
|---|---|
| `inbound_gateway` | Main ingress gateway (webhooks + queue enqueue) |
| `rust_service` | Worker service (queue consumer + scheduler + auth routes) |
| `set_postmark_inbound_hook` | Utility to update Postmark inbound webhook |
| `inbound_fanout` | Legacy fanout ingress helper |
| `google-docs` / `google-sheets` / `google-slides` | Workspace integration CLI tools |

Key scripts:

| Script | Purpose |
|---|---|
| `scripts/run_gateway_local.sh` | Start `inbound_gateway` |
| `scripts/run_employee.sh` | Start `rust_service` for one employee (uses configured public hook URL; ngrok optional for local only) |
| `scripts/start_all.sh` | Local-only stack bootstrap (gateway + worker + ngrok + hook) |
| `scripts/run_e2e.sh` | Live email E2E harness (uses `POSTMARK_TEST_HOOK_URL`/`POSTMARK_INBOUND_HOOK_URL` when available) |
| `scripts/ensure_aci_share_mount.sh` | Validate/mount Azure Files for ACI backend |

## 3) Config Files

### 3.1 Employee config

Default path resolution:
- `EMPLOYEE_CONFIG_PATH` if set
- otherwise `DoWhiz_service/employee.toml`

Primary files:
- `employee.toml` (production/default)
- `employee.staging.toml` (staging profile)

Each employee can define:
- `id`, `display_name`, `runner` (`codex` / `claude`), `model`
- `addresses` (first address is default outbound from)
- optional `runtime_root`
- optional `agents_path`, `claude_path`, `soul_path`, `skills_dir`
- channel toggles: `discord_enabled`, `slack_enabled`, `bluebubbles_enabled`

### 3.2 Gateway config

Default path resolution:
- `GATEWAY_CONFIG_PATH` if set
- otherwise `DoWhiz_service/gateway.toml`

Files:
- `gateway.toml`
- `gateway.staging.toml`
- `gateway.example.toml`

Route model (`channel + key -> employee_id + tenant_id`):
- exact key match has highest priority
- `key = "*"` acts as channel default
- email fallback can route by service address from `employee.toml`
- global defaults (`[defaults]`) are fallback of last resort

Notes:
- Discord message routing uses bot-token-to-employee mapping for selected client; route table is mainly used to enable channel defaults/tenant defaults.

## 4) Environment Variables

Copy base template:

```bash
cp .env.example DoWhiz_service/.env
```

### 4.1 Runtime policy

- Runtime `.env` should use **unprefixed** keys.
- `DEPLOY_TARGET` is optional (`production`/`staging`/others) and affects runtime policy decisions.
- Some ingestion/storage paths support `SCALE_OLIVER_*` fallback aliases; keep unprefixed keys authoritative.

### 4.2 Required for typical gateway + worker flow

| Key | Why |
|---|---|
| `MONGODB_URI` | Scheduler/user/index persistence |
| `SUPABASE_DB_URL` (or `SUPABASE_POOLER_URL` fallback in some paths) | Account/auth/billing store |
| `AZURE_OPENAI_API_KEY_BACKUP` | Required by Codex/Claude task execution |
| `POSTMARK_SERVER_TOKEN` | Email outbound and webhook utility |
| `INGESTION_QUEUE_BACKEND=servicebus` | Required by gateway |
| `SERVICE_BUS_CONNECTION_STRING` **or** `SERVICE_BUS_NAMESPACE` + `SERVICE_BUS_POLICY_NAME` + `SERVICE_BUS_POLICY_KEY` | Service Bus queue auth |
| `SERVICE_BUS_QUEUE_NAME` | Service Bus queue target |

### 4.3 Raw payload storage backend

Default backend is Supabase. Recommended gateway production backend is Azure.

If using Supabase raw payload storage:
- `SUPABASE_PROJECT_URL`
- `SUPABASE_SECRET_KEY`
- optional `SUPABASE_STORAGE_BUCKET` (default `ingestion-raw`)

If using Azure raw payload storage (`RAW_PAYLOAD_STORAGE_BACKEND=azure`):
- `AZURE_STORAGE_CONTAINER_INGEST`
- one auth option:
  - `AZURE_STORAGE_CONTAINER_SAS_URL`, or
  - `AZURE_STORAGE_ACCOUNT` + `AZURE_STORAGE_SAS_TOKEN`, or
  - `AZURE_STORAGE_CONNECTION_STRING_INGEST`/`AZURE_STORAGE_CONNECTION_STRING`

### 4.4 RunTask backend controls

- `RUN_TASK_EXECUTION_BACKEND=local|azure_aci|auto`
- `auto` behavior:
  - `DEPLOY_TARGET in {staging,production}` -> Azure ACI
  - otherwise local

In staging/production targets, local codex execution is blocked unless you explicitly avoid that policy.

Docker execution path (local worker):
- `RUN_TASK_USE_DOCKER=1`
- `RUN_TASK_DOCKER_IMAGE=<image>`
- optional `RUN_TASK_DOCKER_REQUIRED=1`

Azure ACI execution path (required vars):
- `RUN_TASK_AZURE_ACI_RESOURCE_GROUP`
- `RUN_TASK_AZURE_ACI_IMAGE`
- `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT`
- `RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT`
- `RUN_TASK_AZURE_ACI_STORAGE_KEY`
- optional: location/registry/cpu/memory/share/container root vars

### 4.5 Channel-specific integrations (optional)

- Slack: `SLACK_*`, `SLACK_SIGNING_SECRET`
- Discord: `DISCORD_*` and/or employee-specific Discord token envs
- Telegram: `TELEGRAM_BOT_TOKEN` or employee-derived env keys
- WhatsApp: `WHATSAPP_ACCESS_TOKEN`, `WHATSAPP_PHONE_NUMBER_ID`, `WHATSAPP_VERIFY_TOKEN`
- Twilio SMS: `TWILIO_*`
- Google Workspace: `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, refresh tokens, `GOOGLE_*_ENABLED`
- Google Drive push: `GOOGLE_DRIVE_PUSH_ENABLED`, `GOOGLE_DRIVE_WEBHOOK_URL`

## 5) Local Run Workflows

### 5.1 Fast path: one worker + one gateway

From repo root:

```bash
# worker
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok

# gateway (new terminal)
./DoWhiz_service/scripts/run_gateway_local.sh
```

Optional local public webhook:

```bash
ngrok http 9100
cd DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

### 5.2 Manual multi-employee worker setup

```bash
cd DoWhiz_service

EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

EMPLOYEE_ID=mini_mouse RUST_SERVICE_PORT=9002 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9002

EMPLOYEE_ID=sticky_octopus RUST_SERVICE_PORT=9003 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9003

EMPLOYEE_ID=boiled_egg RUST_SERVICE_PORT=9004 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9004
```

Then run gateway using configured routes in `gateway.toml`.

### 5.3 Legacy fanout ingress

`inbound_fanout` is still available for legacy fanout testing:

```bash
./DoWhiz_service/scripts/run_fanout_local.sh
```

Preferred ingress path remains `inbound_gateway`.

## 6) Staging / Production Deployment

Deployment branch policy:
- staging VM deploys from `dev`
- production VM deploys from `main`

Runtime env policy:
- VM runtime file is `DoWhiz_service/.env` with unprefixed keys.
- CI/CD merges `ENV_COMMON + ENV_STAGING` (staging) or `ENV_COMMON + ENV_PROD` (production).

Expected config selections:
- staging: `GATEWAY_CONFIG_PATH=gateway.staging.toml`, `EMPLOYEE_CONFIG_PATH=employee.staging.toml`
- production: `GATEWAY_CONFIG_PATH=gateway.toml`, `EMPLOYEE_CONFIG_PATH=employee.toml`
- staging expected worker identity: `boiled_egg`
- production expected worker identity: `little_bear`
- on staging/production VMs, use existing public webhook endpoint (`POSTMARK_INBOUND_HOOK_URL`) and do not run ngrok

Use these runbooks:
- `DoWhiz_service/OPERATIONS.md`
- `DoWhiz_service/docs/staging_production_deploy.md`

## 7) Testing

### 7.1 Core test commands

```bash
cd DoWhiz_service
cargo test -p run_task_module
cargo test -p send_emails_module
cargo test -p scheduler_module
```

Module-targeted examples:

```bash
cargo test -p scheduler_module --test scheduler_basic
cargo test -p scheduler_module --test send_reply_outbound_e2e
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

### 7.2 Live E2E

Full email E2E helper script:

```bash
./DoWhiz_service/scripts/run_e2e.sh
```

On staging/production, prefer configured public hook URL via `POSTMARK_TEST_HOOK_URL` or `POSTMARK_INBOUND_HOOK_URL` (or pass `--public-url`) and keep ngrok disabled.

Manual live run example:

```bash
RUN_CODEX_E2E=1 POSTMARK_LIVE_TEST=1 \
  cargo test -p scheduler_module --test service_real_email -- --nocapture
```

Canonical test checklist:
- `reference_documentation/test_plans/DoWhiz_service_tests.md`

## 8) Runtime State and Data Stores

Default runtime root:

```text
$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/
```

Common directories:
- `state/` (scheduler/user/index scope keys and processed IDs)
- `users/<user_id>/memory`
- `users/<user_id>/mail`
- `users/<user_id>/workspaces/<thread_or_message>`

Data store split:
- MongoDB: task scheduler state, user/index data, several operational collections
- Supabase Postgres: account/auth/billing records
- Raw payload: Supabase storage or Azure Blob (by backend config)
- Queue: Service Bus (gateway flow) or Postgres (legacy/optional)

## 9) Troubleshooting

### Gateway exits immediately with backend error

Symptom:
- `inbound gateway requires ... INGESTION_QUEUE_BACKEND=servicebus`

Fix:
- set `INGESTION_QUEUE_BACKEND=servicebus`
- set either `SERVICE_BUS_CONNECTION_STRING`
  or `SERVICE_BUS_NAMESPACE` + `SERVICE_BUS_POLICY_NAME` + `SERVICE_BUS_POLICY_KEY`
- set `SERVICE_BUS_QUEUE_NAME`

### Gateway enqueue works but worker does not process

Check:
- same Service Bus credentials and `SERVICE_BUS_QUEUE_NAME` in worker env
- worker `EMPLOYEE_ID` matches routed employee
- worker logs for `claim_next`/processing errors

### Raw payload store upload/download failures

Check:
- backend selection `RAW_PAYLOAD_STORAGE_BACKEND`
- matching credentials for selected backend
- container/bucket names and permissions

### Local run_task blocked in staging/production target

Symptom:
- local execution forbidden error

Fix:
- use `RUN_TASK_EXECUTION_BACKEND=azure_aci` with required ACI env,
  or run with non-staging/production `DEPLOY_TARGET` for local dev.

### Azure ACI backend fails before execution

Check:
- Azure Files share mounted at `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT`
- run `scripts/ensure_aci_share_mount.sh`
- verify ACI resource group/image/storage credentials
