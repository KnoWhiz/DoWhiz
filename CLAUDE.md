# CLAUDE.md

`external/` folder contains information about other projects that we can use as reference bu we never need to touch the code in it.

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz is a multi-tenant, email-first digital employee platform. Users send tasks to digital employees via email (and other channels like Discord, Slack, iMessage), and AI agents (Codex CLI or Claude Code) process and respond. The system emphasizes per-user isolation, role-based agents, and tool-backed execution.

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
- **scheduler_module**: Core HTTP server (Axum), task scheduler, SQLite persistence, webhook handlers
- **send_emails_module**: Postmark API wrapper for email delivery
- **run_task_module**: Codex/Claude CLI invocation for task execution
- **website**: React 19 + Vite marketing site

### Data Flow
```
Inbound (Email/Discord/Slack/iMessage) → Gateway → Deduplication → Routing
    → Scheduler (SQLite) → Task Execution (Codex/Claude) → Outbound Reply
```

### Key Files
| File | Purpose |
|------|---------|
| `scheduler_module/src/service.rs` | Webhook handlers, scheduler loop |
| `scheduler_module/src/lib.rs` | Core Scheduler, TaskKind, Schedule definitions |
| `scheduler_module/src/user_store/mod.rs` | Per-user data management |
| `send_emails_module/src/lib.rs` | Postmark API wrapper |
| `run_task_module/src/lib.rs` | Codex/Claude CLI invocation |
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
| `sticky_octopus` | Devin | Codex | devin@dowhiz.com |
| `boiled_egg` | Proto | Codex | proto@dowhiz.com |

## Key Concepts

### Task Kinds
- **SendEmail**: Send HTML email with attachments
- **RunTask**: Invoke Codex/Claude CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime

### Per-User Isolation
Each user gets separate SQLite databases and workspace directories. Concurrency limits: global max 10, per-user max 3.

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
- `AZURE_OPENAI_API_KEY_BACKUP`, `AZURE_OPENAI_ENDPOINT_BACKUP` - For Codex employees
- `ANTHROPIC_API_KEY` - For Claude employees (mini_mouse)

Optional:
- `CODEX_DISABLED=1` - Bypass Codex CLI (uses placeholder replies)
- `RUN_TASK_DOCKER_IMAGE` - Enable per-task Docker isolation
- `GITHUB_USERNAME`, `GITHUB_PERSONAL_ACCESS_TOKEN` - GitHub access for agents

## Testing Expectations

After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Test Checklist and Report (DoWhiz_service)
- Canonical checklist: `test_plans/DoWhiz_service_tests.md`
- After any DoWhiz_service change, run all relevant AUTO tests from the checklist.
- For LIVE/MANUAL/PLANNED entries, mark SKIP with a reason unless explicitly run.
- Always include the Test Report table from the checklist in your final response (PASS/FAIL/SKIP per Test ID).
- For real end-to-end tests requiring a public ngrok URL, use `https://shayne-laminar-lillian.ngrok-free.dev` and set up the environment accordingly.

## Coding Guidelines

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.

Use standard Rust formatting (rustfmt) with `snake_case` for functions/modules, `CamelCase` for types.

Tests live in `*/tests/*.rs` for integration and in-module `#[test]` for unit coverage. Live Postmark tests are opt-in via environment variables.

When opening PRs, include a short summary, tests run, and any required env/config changes.
