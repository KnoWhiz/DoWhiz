# CLAUDE.md

`external/` folder contains information about other projects that we can use as reference but we never need to touch the code in it.

By default open RUN_CODEX_E2E=1 and POSTMARK_LIVE_TEST=1 to use real emails address (mini-mouse@dowhiz.com / deep-tutor@deep-tutor.com to send emails to proto@dowhiz.com / oliver@dowhiz.com / maggie@dowhiz.com). For ngrok domain you can use: `https://shayne-laminar-lillian.ngrok-free.dev`

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz is a multi-tenant, email-first digital employee platform. Users send tasks to digital employees via email (and other channels like Slack, Discord, SMS via Twilio, Telegram, WhatsApp, Google Docs/Sheets/Slides comments, iMessage via BlueBubbles), and AI agents (Codex CLI or Claude Code) process and respond. The system emphasizes per-user isolation, role-based agents, and tool-backed execution.

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
# One-command local run (starts ngrok, updates Postmark hook, runs service)
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
./DoWhiz_service/scripts/run_employee.sh mini_mouse 9002

# Docker build and run
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -e EMPLOYEE_ID=little_bear \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  dowhiz-service
```

## Architecture

### Workspace Structure (Cargo workspace)
- **scheduler_module**: Core HTTP worker server (Axum), task scheduler, SQLite persistence, inbound gateway binary (webhooks + dedupe)
- **send_emails_module**: Postmark API wrapper for email delivery
- **run_task_module**: Codex/Claude CLI invocation for task execution
- **website**: React 19 + Vite marketing site

### Data Flow
```
Inbound (Email/Slack/Discord/SMS/Telegram/WhatsApp/Google Docs/Sheets/Slides/iMessage)
    → Ingestion Gateway (dedupe + raw payload storage in Azure Blob)
    → Ingestion Queue (Service Bus for gateway; Postgres optional/legacy)
    → Worker Service (per-employee)
    → Scheduler (SQLite) → Task Execution (Codex/Claude) → Outbound Reply
```

### Key Files
| File | Purpose |
|------|---------|
| `scheduler_module/src/service/server.rs` | Worker HTTP server, scheduler loop |
| `scheduler_module/src/bin/inbound_gateway.rs` | Inbound gateway entrypoint (webhooks + dedupe) |
| `scheduler_module/src/bin/inbound_gateway/google_workspace.rs` | Google Sheets/Slides pollers (workspace comments) |
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
│   ├── tasks.db              # Global scheduler state
│   ├── users.db              # User registry
│   ├── task_index.db         # Due task index
│   └── postmark_processed_ids.txt
├── workspaces/<message_id>/  # Per-task execution
└── users/<user_id>/
    ├── state/tasks.db        # User's task queue
    ├── memory/               # Agent context
    └── mail/                 # Email archive
```

### Employees
| ID | Name | Runner | Primary Email |
|----|------|--------|---------------|
| `little_bear` | Oliver | Codex | oliver@dowhiz.com |
| `mini_mouse` | Maggie | Claude | maggie@dowhiz.com |
| `sticky_octopus` | Sticky-Octopus | Codex | devin@dowhiz.com |
| `boiled_egg` | Boiled-Egg | Codex | proto@dowhiz.com |

## Key Concepts

### Task Kinds
- **SendEmail**: Send HTML email with attachments
- **RunTask**: Invoke Codex/Claude CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime

### Per-User Isolation
Each user gets separate SQLite databases and workspace directories. Concurrency limits are configurable (defaults: global max 200, per-user max 200).

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
- `AZURE_OPENAI_ENDPOINT_BACKUP` - Required for Codex runner

Optional:
- `CODEX_DISABLED=1` - Bypass Codex CLI (uses placeholder replies)
- `RUN_TASK_DOCKER_IMAGE` - Enable per-task Docker isolation
- `GITHUB_USERNAME`, `GITHUB_PERSONAL_ACCESS_TOKEN` - GitHub access for agents
- `OPENAI_API_KEY` - Enable message router quick replies
- `INGESTION_QUEUE_BACKEND=servicebus` + `SERVICE_BUS_CONNECTION_STRING` + `SERVICE_BUS_QUEUE_NAME` - Required when running the inbound gateway
- `AZURE_STORAGE_CONTAINER_INGEST` + `AZURE_STORAGE_SAS_TOKEN` (+ optional `AZURE_STORAGE_ACCOUNT` or `AZURE_STORAGE_CONNECTION_STRING_INGEST`) - Required for gateway raw payload storage (Azure Blob), unless using `AZURE_STORAGE_CONTAINER_SAS_URL`
- `AZURE_STORAGE_CONTAINER_SAS_URL` - Optional full SAS URL for the ingestion container (overrides account + container + SAS token)

## Testing Expectations

After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Test Checklist and Report (DoWhiz_service)
- Canonical checklist: `test_plans/DoWhiz_service_tests.md`
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

# Operations & Debugging (Azure VM)

## IMPORTANT: Read This Before Debugging

Before debugging DoWhiz deployment issues, read `DoWhiz_service/OPERATIONS.md` for:
- Azure VM paths and PM2 commands
- Common issues and solutions
- Recent optimizations

## Quick Reference

### Azure VM Info
- **Server root**: `/home/azureuser/server/`
- **DoWhiz Service**: `/home/azureuser/server/DoWhiz_service/`
- **PM2 logs**: `/home/azureuser/server/.pm2/logs/`
- **PM2 requires**: `export HOME=/home/azureuser/server`

### Common Issues

#### 1. Azure Service Bus Enqueue Error
```
gateway enqueue error: service bus error: HttpResponse(400,unknown)
```
**Cause**: Service Bus connection string misconfigured or expired
**Fix**: Check `AZURE_SERVICE_BUS_CONNECTION_STRING` in `.env`

#### 2. "missing policy name in connection string"
**Cause**: Azure Service Bus SAS policy not specified
**Fix**: Connection string format should be:
```
Endpoint=sb://<namespace>.servicebus.windows.net/;SharedAccessKeyName=<policy>;SharedAccessKey=<key>
```

#### 3. Slides/Sheets comments not processed
**Check order**:
1. Is `GOOGLE_SLIDES_ENABLED=true` in `.env`?
2. Are there Service Bus errors in logs?
3. Is `scheduler_user_max_concurrency` causing blocking?
4. Check `google_workspace_processed.db` for already-processed comments

#### 4. 20+ minute delay for tasks
**Root cause**: `SCHEDULER_USER_MAX_CONCURRENCY=1`
- Only one task per user runs at a time
- Affects ALL channels (Email, Slack, Discord, Google Docs/Sheets/Slides)
- Team discussion needed before increasing

### Diagnostic Commands for VM

```bash
# Check PM2 status
HOME=/home/azureuser/server pm2 list

# Check recent errors
tail -100 /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | grep -i error

# Check Slides polling
grep -i "slides|presentation" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -30

# Check Service Bus errors
grep -i "service bus|enqueue" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -20
```

---

## Session Notes (Update After Each Session)

### 2026-02-25: Slides Debug Session

**Problem**: User's Slides comments not getting replies

**Root Cause Found**: Azure Service Bus enqueue failing with `HttpResponse(400,unknown)`
- Comments are being detected but cannot be queued for processing
- Need to fix Service Bus connection string configuration

**Files Modified**:
- `DoWhiz_service/scheduler_module/src/adapters/google_common/comments.rs` - Added timeout + retry
- `DoWhiz_service/scheduler_module/src/google_workspace_poller.rs` - Added file list cache, reduced polling to 15s
- `DoWhiz_service/scheduler_module/src/bin/inbound_gateway/google_workspace.rs` - Parallelized Sheets/Slides polling
- `DoWhiz_service/scheduler_module/src/google_drive_changes.rs` - New file for future push notifications
- `DoWhiz_service/OPERATIONS.md` - Created deployment guide

**Commits**:
- `294c0bd` - Optimize Google Workspace polling for reduced latency
- `bfdbabf` - Add OPERATIONS.md for Azure VM deployment guide

**Next Steps**:
1. Fix Azure Service Bus connection string on VM
2. Deploy new code after testing locally
3. Consider increasing `SCHEDULER_USER_MAX_CONCURRENCY` (needs team discussion)

---

## Code Patterns

### Google Workspace Comment Flow
```
DoWhiz_service/scheduler_module/src/bin/inbound_gateway/google_workspace.rs
  └─> spawn_google_workspace_poller()
       └─> poll_workspace_comments() [every 15s]
            └─> GoogleWorkspacePoller.poll_sheets() / poll_slides()
                 └─> GoogleCommentsClient.list_comments()
                      └─> filter_actionable_comments()
                           └─> resolve_route()
                                └─> state.queue.enqueue()  <-- THIS IS FAILING
```

### Key Environment Variables
- `GOOGLE_DOCS_ENABLED` / `GOOGLE_SHEETS_ENABLED` / `GOOGLE_SLIDES_ENABLED`
- `GOOGLE_WORKSPACE_POLL_INTERVAL_SECS` (default: 15)
- `SCHEDULER_USER_MAX_CONCURRENCY` (default: 200; prod may set lower)
- `GOOGLE_DRIVE_PUSH_ENABLED` (future feature)
- `AZURE_SERVICE_BUS_CONNECTION_STRING`
