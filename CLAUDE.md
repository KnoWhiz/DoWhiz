# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Try to keep the whole codebase modular and easy to maintain. If a file is too long (say more than 500 / 1000 lines), consider separate the code file into separate code files and make sure each one has its own well defined functionality.

## Project Overview

DoWhiz is a Digital Employee Platform - a SaaS system where users delegate tasks via email to AI-powered digital workers (agents named Oliver, Mini-Mouse, etc.). Users interact through their inbox; no UI is required.

**Architecture Flow:**
```
Email (Postmark) → Inbound Webhook → Triage & Queue → Agent Runtime (Codex CLI) → Email Reply
                                                     ↓
                                               Scheduler (CRON support)
```

## Build & Run Commands

### Backend (Rust Service)

```bash
# Build all modules
cargo build

# Run all tests
cargo test

# Run specific test
cargo test scheduler_concurrency

# Run tests with output
cargo test -- --nocapture

# Start the service
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# Update Postmark webhook URL
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound

# Hydrate email history for a user
cargo run -p scheduler_module --bin hydrate_past_emails -- \
  --archive-root .workspace/run_task/users/<user_id>/mail \
  --references-dir /path/to/workspace/references \
  --user-id <user_id>
```

### Live End-to-End Test (requires Postmark credentials)

```bash
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK-URL.ngrok-free.dev \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

### Frontend (React + Vite)

```bash
cd website
npm run dev      # Development server
npm run build    # Production build
npm run lint     # ESLint
npm run preview  # Preview production build
```

## Project Structure

```
DoWhiz/
├── DoWhiz_service/              # Main Rust workspace
│   ├── scheduler_module/        # Webhooks, scheduling, service binary
│   │   ├── src/lib.rs          # Core scheduler logic, task definitions
│   │   ├── src/service.rs      # HTTP server (Axum), webhook handling
│   │   ├── src/past_emails.rs  # Email archive hydration
│   │   ├── src/user_store/     # Per-user SQLite management
│   │   ├── src/index_store/    # Global task index
│   │   └── tests/              # Integration tests
│   ├── send_emails_module/      # Postmark email sending
│   └── run_task_module/         # Codex CLI execution
├── website/                     # React + Vite landing page
└── .workspace/run_task/         # Runtime state (not in git)
    ├── workspaces/<msg_id>/     # Per-task execution dirs
    ├── users/<user_id>/         # Per-user data, memory, mail archive
    └── state/                   # Global DBs (users.db, task_index.db)
```

## Key Architectural Patterns

### Multi-tenant Isolation
- Each user has a UUID; separate SQLite databases per user
- Email normalization handles case and `+tag` aliases
- Strict workspace isolation per task execution

### Task Scheduling
- Schedule types: `cron` (recurring) or `one_shot` (single execution)
- Global index store for O(log n) due-task queries
- Concurrency limits: global (`SCHEDULER_MAX_CONCURRENCY=10`) and per-user (`SCHEDULER_USER_MAX_CONCURRENCY=3`)

### Scheduled Follow-ups
Agents emit follow-up tasks via stdout blocks that the scheduler parses:
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"Reminder",...}]
SCHEDULED_TASKS_JSON_END
```

## Environment Variables

**Required:**
- `POSTMARK_SERVER_TOKEN` - Email service
- `AZURE_OPENAI_API_KEY_BACKUP`, `AZURE_OPENAI_ENDPOINT_BACKUP` - AI inference

**Service Config:**
- `RUST_SERVICE_HOST` / `RUST_SERVICE_PORT` - Server binding (default: 0.0.0.0:9001)
- `WORKSPACE_ROOT` - Task workspaces (default: `.workspace/run_task/workspaces`)
- `USERS_ROOT` - User data (default: `.workspace/run_task/users`)
- `CODEX_DISABLED=1` - Bypass Codex CLI for local testing

## Code Style

- Standard Rust formatting (rustfmt defaults)
- `snake_case` for functions/modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants
- Commit messages: short, imperative, capitalized (e.g., "Update service.rs")

## Testing

- Unit tests: `#[test]` blocks in source files
- Integration tests: `*/tests/*.rs`
- Live Postmark tests: opt-in via `RUST_SERVICE_LIVE_TEST=1`
