# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Try to keep the whole codebase modular and easy to maintain. If a file is too long (say more than 500 / 1000 lines), consider separate the code file into separate code files and make sure each one has its own well defined functionality.

## Overview

DoWhiz Service is a Rust-based email processing and task scheduling system. It receives inbound email webhooks from Postmark, runs AI agent tasks (via Codex CLI) to process emails, and sends reply emails back via Postmark. The system uses SQLite for persistence with per-user task isolation.

## Build and Test Commands

```bash
# Build all modules
cargo build

# Run main HTTP server
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# Run all tests
cargo test

# Run tests for a specific module
cargo test -p scheduler_module
cargo test -p send_emails_module
cargo test -p run_task_module

# Run a single test
cargo test -p scheduler_module --test scheduler_basic

# Lint
cargo clippy --all-targets --all-features
cargo fmt --check
```

### Live Tests (require credentials)
```bash
# Live Postmark email test
POSTMARK_LIVE_TEST=1 cargo test -p send_emails_module

# Full E2E test with real Postmark
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK-URL.ngrok-free.dev \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

## Architecture

### Workspace Structure (Cargo workspace with 3 modules)

- **scheduler_module**: Core task scheduler, HTTP server (Axum), SQLite persistence, webhook handling
- **send_emails_module**: Postmark API wrapper for sending emails with attachments
- **run_task_module**: Codex CLI runner for AI-generated email replies

### Data Flow

1. **Inbound email** → `POST /postmark/inbound` webhook
2. **Deduplication** → Check processed IDs in `postmark_processed_ids.txt`
3. **User registration** → Get/create user in `users.db`
4. **RunTask scheduled** → Creates task in user's `tasks.db` with `next_run = now`
5. **Scheduler loop** → Polls `task_index.db` for due tasks (every 1 sec)
6. **Task execution** → Runs Codex CLI, generates reply HTML
7. **SendEmail scheduled** → Queues email task (immediate or delayed)
8. **Email sent** → Postmark API, then archived to user's `mail/` directory

### Key Files

| File | Purpose |
|------|---------|
| `scheduler_module/src/bin/rust_service.rs` | Main HTTP server entry point |
| `scheduler_module/src/service.rs` | Webhook handler, scheduler loop |
| `scheduler_module/src/lib.rs` | Core Scheduler, TaskKind, Schedule definitions |
| `scheduler_module/src/user_store/mod.rs` | Per-user data management |
| `scheduler_module/src/index_store/mod.rs` | Global task index |
| `send_emails_module/src/lib.rs` | Postmark API wrapper |
| `run_task_module/src/lib.rs` | Codex CLI invocation |

### Runtime State (`.workspace/run_task/`)

```
.workspace/run_task/
├── state/
│   ├── tasks.db              # Global scheduler state
│   ├── users.db              # User registry
│   ├── task_index.db         # Index for due task polling
│   └── postmark_processed_ids.txt
├── workspaces/               # Per-task execution directories
└── users/<user_id>/
    ├── state/tasks.db        # User's task queue
    ├── memory/               # Agent context
    └── mail/                 # Email archive
```

## Environment Variables

Required in `.env`:
- `POSTMARK_SERVER_TOKEN` - Postmark API key

Replies are sent from the inbound service address detected in the received email.

For Codex AI:
- `AZURE_OPENAI_API_KEY_BACKUP`, `AZURE_OPENAI_ENDPOINT_BACKUP`
- `CODEX_MODEL` - Model name
- `CODEX_DISABLED=1` - Bypass Codex (uses placeholder replies)

Scheduler tuning:
- `SCHEDULER_POLL_INTERVAL_SECS` (default: 1)
- `SCHEDULER_MAX_CONCURRENCY` (default: 10) - Global max concurrent tasks
- `SCHEDULER_USER_MAX_CONCURRENCY` (default: 3) - Per-user limit

## Key Concepts

### Task Kinds
- **SendEmail**: Send HTML email with attachments
- **RunTask**: Invoke Codex CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime

### Follow-up Scheduling
Agents emit scheduled tasks in stdout:
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"...","html_path":"...","to":[...]}]
SCHEDULED_TASKS_JSON_END
```

### Per-User Isolation
Each user gets separate SQLite databases and workspace directories. Concurrency is enforced at both global and per-user levels.

### Email Blacklist
Inbound emails from `little-bear@dowhiz.com`, `agent@dowhiz.com`, `oliver@dowhiz.com`, `mini-mouse@dowhiz.com`, and `maggie@dowhiz.com` are ignored (prevents loops).

<SOUL>
Your name is Mini-Mouse, a tiny mouse, who is curious and quick and capable. You always get task done.
Go mice!
</SOUL>
