# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz is an email-first digital employee platform. Users send tasks via email to role-based AI agents (Oliver/Codex or Maggie/Claude), which process requests and reply with structured results. The system handles scheduling, orchestration, and per-user isolation.

## Build and Run Commands

### Rust Service
```bash
# Build all modules
cargo build

# Run HTTP server (single employee)
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# One-command local run (auto ngrok + Postmark hook setup)
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
./DoWhiz_service/scripts/run_employee.sh mini_mouse 9002

# Docker production
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 -v "$PWD/.env:/app/.env:ro" -v dowhiz-workspace:/app/.workspace dowhiz-service
```

### Tests
```bash
# Unit tests
cargo test -p scheduler_module
cargo test -p run_task_module

# Single test
cargo test -p scheduler_module --test scheduler_basic

# Live Postmark E2E (requires ngrok + credentials)
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK-URL.ngrok-free.dev \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

### Linting
```bash
cargo clippy --all-targets --all-features
cargo fmt --check
```

### Website
```bash
cd website && npm install
npm run dev      # Dev server on port 5173
npm run build    # Production build
npm run lint     # ESLint
```

## Architecture

### Data Flow
```
Inbound email (Postmark webhook)
  → POST /postmark/inbound
  → Deduplication (postmark_processed_ids.txt)
  → User registration (users.db)
  → RunTask scheduled (user's tasks.db)
  → Scheduler loop polls task_index.db (1 sec interval)
  → Task execution (Codex CLI or Claude CLI)
  → SendEmail scheduled
  → Postmark API sends reply
  → Archive to user's mail/ directory
```

### Rust Workspace Modules
- **scheduler_module**: HTTP server (Axum), task scheduler, SQLite persistence, webhook handling
- **send_emails_module**: Postmark API wrapper for sending emails with attachments
- **run_task_module**: CLI runner for Codex (OpenAI) and Claude (Anthropic) agents

### Runtime State (`$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/`)
```
state/
├── tasks.db                    # Global scheduler state
├── users.db                    # User registry
├── task_index.db               # Due task index for polling
└── postmark_processed_ids.txt  # Deduplication list
workspaces/<message_id>/
├── workspace/                  # Agent workspace
├── references/past_emails/     # Hydrated email history
├── reply_email_draft.html      # Generated reply
└── reply_email_attachments/
users/<user_id>/
├── state/tasks.db              # Per-user task queue
├── memory/                     # Agent context/memory
└── mail/                       # Email archive
```

### Employee Profiles (`DoWhiz_service/employee.toml`)
- **Oliver (little_bear)**: Codex runner, addresses: oliver@dowhiz.com, little-bear@dowhiz.com, agent@dowhiz.com
- **Maggie (mini_mouse)**: Claude runner, addresses: maggie@dowhiz.com, mini-mouse@dowhiz.com

Each employee has isolated state directories. Employee configs include AGENTS.md, CLAUDE.md, and SOUL.md in `employees/<id>/`.

### Key Concepts
- **Task Kinds**: SendEmail, RunTask, Noop
- **Schedules**: Cron (6-field: sec min hour day month weekday UTC) or OneShot (single DateTime)
- **Follow-up Scheduling**: Agents emit `SCHEDULED_TASKS_JSON_BEGIN...END` blocks in stdout
- **Per-User Isolation**: Separate SQLite databases and workspaces per user
- **Email Blacklist**: Employee addresses ignored as senders (prevents loops)

## Required Environment Variables (`.env`)
```bash
POSTMARK_SERVER_TOKEN=          # Postmark API key
AZURE_OPENAI_API_KEY_BACKUP=    # For Codex runner
AZURE_OPENAI_ENDPOINT_BACKUP=   # For Codex runner
```

### Key Optional Variables
- `EMPLOYEE_ID`: Profile selector (little_bear, mini_mouse)
- `CODEX_DISABLED=1`: Bypass Codex (placeholder replies)
- `CLAUDE_MODEL`: Model for Claude runner (default: claude-opus-4-5)
- `RUN_TASK_DOCKER_IMAGE`: Enable per-task Docker isolation
- `SCHEDULER_MAX_CONCURRENCY`: Global max concurrent tasks (default: 10)
- `SCHEDULER_USER_MAX_CONCURRENCY`: Per-user limit (default: 3)

## Repository Structure
- `DoWhiz_service/`: Rust backend service (scheduler, email handling, task execution)
- `website/`: React frontend (Vite + React 19)
- `DoWhiz_service/skills/`: 20+ agent skills (playwright-cli, pdf, docx, pptx, canvas-design, etc.)
- `DoWhiz_service/employees/`: Employee persona configs
- `external/openclaw/`: Reference implementation for multi-agent patterns
- `function_app/`: Azure Functions deployment wrapper

## Code Style
- Keep modules under ~500-1000 lines; split into separate files with well-defined functionality
- Run `cargo fmt` before commits
- Run `npm run lint` for website changes
