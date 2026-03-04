# CLAUDE.md

`external/` folder contains information about other projects that we can use as reference but we never need to touch the code in it.

By default open RUN_CODEX_E2E=1 and POSTMARK_LIVE_TEST=1 to use real emails address (mini-mouse@dowhiz.com / deep-tutor@deep-tutor.com to send emails to proto@dowhiz.com / oliver@dowhiz.com / maggie@dowhiz.com). For ngrok domain you can use: `https://shayne-laminar-lillian.ngrok-free.dev`

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz is a multi-tenant, email-first digital employee platform. Users send tasks to digital employees via email (and other channels like Slack, Discord, SMS via Twilio, Telegram, WhatsApp, Google Docs comments, iMessage via BlueBubbles), and AI agents (Codex CLI or Claude Code) process and respond. The system emphasizes per-user isolation, role-based agents, and tool-backed execution.

## Build and Development Commands

### Rust Backend (DoWhiz_service)
```bash
cargo build                                    # Build all modules
cargo test                                     # Run all tests
cargo test -p scheduler_module                 # Test specific module
cargo test -p scheduler_module --test scheduler_basic  # Single test
cargo clippy --all-targets --all-features     # Lint
cargo fmt --check                             # Format check

# Run HTTP server
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

### React Website
```bash
cd website
npm install && npm run dev    # Dev server (port 5173)
npm run build                 # Production build -> website/dist/
npm run lint                  # ESLint
```

### Live E2E Tests (require credentials)
```bash
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK.ngrok.app \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

### Running Employees
```bash
# Worker-only local run (recommended with inbound gateway)
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
./DoWhiz_service/scripts/run_employee.sh mini_mouse 9002 --skip-hook --skip-ngrok

# Docker build and run
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -e EMPLOYEE_ID=little_bear \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  dowhiz-service
```

## Architecture

### Workspace Structure (Cargo workspace)
- **scheduler_module**: Core HTTP worker server (Axum), inbound gateway binary (webhooks + dedupe), scheduler and user/index stores backed by MongoDB, plus account/auth/billing flows backed by Supabase Postgres
- **send_emails_module**: Postmark API wrapper for email delivery
- **run_task_module**: Codex/Claude CLI invocation for task execution
- **website**: React 19 + Vite marketing site

### Data Flow
```
Inbound (Email/Slack/Discord/SMS/Telegram/WhatsApp/Google Docs/iMessage)
    → Ingestion Gateway (dedupe + raw payload storage in Azure Blob)
    → Ingestion Queue (Service Bus for gateway; Postgres optional/legacy)
    → Worker Service (per-employee)
    → Scheduler/User Index (MongoDB) → Task Execution (Codex/Claude) → Outbound Reply
```

### Key Files
| File | Purpose |
|------|---------|
| `scheduler_module/src/service/server.rs` | Worker HTTP server, scheduler loop |
| `scheduler_module/src/bin/inbound_gateway.rs` | Inbound gateway entrypoint (webhooks + dedupe) |
| `scheduler_module/src/lib.rs` | Core Scheduler, TaskKind, Schedule definitions |
| `scheduler_module/src/user_store/mod.rs` | Per-user data management |
| `send_emails_module/src/lib.rs` | Postmark API wrapper |
| `run_task_module/src/lib.rs` | Codex/Claude CLI invocation |
| `scheduler_module/src/adapters/whatsapp.rs` | WhatsApp inbound/outbound adapter |
| `DoWhiz_service/employee.toml` | Employee registry (addresses, runners, models) |

### Runtime State
```
$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/
├── state/
│   ├── tasks.db              # Scope key path for scheduler records in MongoDB
│   ├── users.db              # Scope key path for user records in MongoDB
│   ├── task_index.db         # Scope key path for due-task index records in MongoDB
│   └── postmark_processed_ids.txt
├── workspaces/<message_id>/  # Per-task execution
└── users/<user_id>/
    ├── state/tasks.db        # User-scope key path for per-user scheduler records in MongoDB
    ├── memory/               # Agent context
    └── mail/                 # Email archive
```

### Employees
| ID | Name | Runner | Primary Email |
|----|------|--------|---------------|
| `little_bear` | Oliver | Codex | oliver@dowhiz.com |
| `mini_mouse` | Maggie | Claude | maggie@dowhiz.com |
| `sticky_octopus` | Devin | Codex | devin@dowhiz.com |
| `boiled_egg` | Boiled-Egg | Codex | proto@dowhiz.com |

## Key Concepts

### Task Kinds
- **SendReply**: Send outbound reply on a channel (email/slack/discord/sms/telegram/whatsapp/google workspace)
- **RunTask**: Invoke Codex/Claude CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime

### Per-User Isolation
Each user gets isolated workspace/memory/mail directories plus Mongo owner scopes derived from per-user paths. Default concurrency limits are global 200 and per-user 1 (configurable via env).

### Follow-up Scheduling
Agents emit scheduled tasks in stdout:
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"...","html_path":"...","to":[...]}]
SCHEDULED_TASKS_JSON_END
```

## Required Environment Variables

Copy `.env.example` to `DoWhiz_service/.env` and configure:
- `POSTMARK_SERVER_TOKEN` - Postmark API key (required)
- `AZURE_OPENAI_API_KEY_BACKUP` - Required for Codex and Claude runners (Foundry config)
- `AZURE_OPENAI_ENDPOINT_BACKUP` - Optional endpoint override for components that use Azure OpenAI directly (Codex runner endpoint is fixed in code)
- `MONGODB_URI` - Required for scheduler/user/index/slack persistence
- `SUPABASE_DB_URL` - Required by `AccountStore` (auth/account-link flows in worker and gateway; billing routes in worker when Stripe is enabled)

Optional:
- `CODEX_DISABLED=1` - Bypass Codex CLI (uses placeholder replies)
- `RUN_TASK_DOCKER_IMAGE` - Enable per-task Docker isolation
- `GITHUB_USERNAME`, `GITHUB_PERSONAL_ACCESS_TOKEN` - GitHub access for agents
- `OPENAI_API_KEY` - Enable message router quick replies
- `INGESTION_QUEUE_BACKEND=servicebus` + `SERVICE_BUS_CONNECTION_STRING` + `SERVICE_BUS_QUEUE_NAME` - Required when running the inbound gateway
- `AZURE_STORAGE_CONTAINER_INGEST` + `AZURE_STORAGE_SAS_TOKEN` (and optionally `AZURE_STORAGE_ACCOUNT` or `AZURE_STORAGE_CONTAINER_SAS_URL`) - Raw payload storage when `RAW_PAYLOAD_STORAGE_BACKEND=azure`
- `SUPABASE_JWT_SECRET` + `SUPABASE_ANON_KEY` + `SUPABASE_PROJECT_URL` - Recommended for token validation in `/auth/*` and `/billing/*`

## Testing Expectations

After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Test Checklist and Report (DoWhiz_service)
- Canonical checklist: `reference_documentation/test_plans/DoWhiz_service_tests.md`
- After any DoWhiz_service change, run all relevant AUTO tests from the checklist.
- For LIVE/MANUAL/PLANNED entries, mark SKIP with a reason unless explicitly run.
- If the user asks so, include the Test Report table from the checklist in your final response (PASS/FAIL/SKIP per Test ID). Otherwise by default summarize the tests results.
- For real end-to-end tests requiring a public ngrok URL, use `https://shayne-laminar-lillian.ngrok-free.dev` and set up the environment accordingly.

## Coding Guidelines

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.

Use standard Rust formatting (rustfmt) with `snake_case` for functions/modules, `CamelCase` for types.

Tests live in `*/tests/*.rs` for integration and in-module `#[test]` for unit coverage. Live Postmark tests are opt-in via environment variables.

When opening PRs, include a short summary, tests run, and any required env/config changes.

---

# Operations & Deployment Notes (Azure VM)

Use `DoWhiz_service/OPERATIONS.md` as the source of truth for VM paths, PM2 commands, deployment runbooks, and troubleshooting.

Current deployment policy:
- Production VM deploy target branch: `main`
- Staging VM deploy target branch: `dev`

Single `.env` split policy:
- Production uses base keys
- Staging uses `STAGING_` keys with `DEPLOY_TARGET=staging`
- Runtime mapping is handled by `DoWhiz_service/scripts/load_env_target.sh`

For exact staging/prod commands, key split table, and rollback:
- `DoWhiz_service/docs/staging_production_deploy.md`

Quick checks:
```bash
HOME=/home/azureuser/server pm2 list
pm2 logs dowhiz-inbound-gateway
pm2 logs dowhiz-rust-service
grep -i "service bus\\|enqueue\\|error" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -50
```

Key env names to verify first:
- `SERVICE_BUS_CONNECTION_STRING` (or `SCALE_OLIVER_SERVICE_BUS_CONNECTION_STRING`)
- `SERVICE_BUS_QUEUE_NAME`
- `INGESTION_QUEUE_BACKEND=servicebus`
- `RAW_PAYLOAD_STORAGE_BACKEND=azure` (recommended for gateway production flow)
- `AZURE_STORAGE_CONTAINER_INGEST`
- `AZURE_STORAGE_SAS_TOKEN` (or `AZURE_STORAGE_CONTAINER_SAS_URL`)
