# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz is a lightweight Rust replica of OpenClaw that creates "digital employees" - autonomous AI agents that receive tasks via email, Slack, Discord, Telegram, WhatsApp, and other channels, execute tasks using Codex or Claude CLI, and respond intelligently.

## Code Style

Keep the codebase modular and easy to maintain. If a file exceeds 500-1000 lines, split it into separate files with well-defined, single-responsibility functionality.

## Build Commands

### Rust Backend (DoWhiz_service/)

```bash
# Build
cargo build                                    # Dev build
cargo build --release                          # Release build
cargo build -p scheduler_module --release      # Just scheduler

# Test
cargo test                                     # All tests
cargo test -p scheduler_module                 # Scheduler tests only
cargo test -p run_task_module                  # Run task module tests
cargo test -p scheduler_module --test scheduler_basic  # Single test file

# Lint
cargo clippy --all-targets --all-features
cargo fmt --check
```

### Frontend (website/)

```bash
cd website
npm install
npm run dev      # Dev server (port 5173)
npm run build    # Production build
npm run lint     # ESLint
```

### Running the Service

```bash
# Single employee (auto ngrok + hook setup)
./DoWhiz_service/scripts/run_employee.sh little_bear 9001

# All employees via Docker
./DoWhiz_service/scripts/run_all_employees_docker.sh

# Manual single employee
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

## Architecture

```
Inbound message -> Scheduler -> Task runner (Codex/Claude) -> Tools -> Outbound message
```

### Workspace Structure (Rust)

```
DoWhiz_service/
├── scheduler_module/       # HTTP server (Axum), webhook handlers, task scheduling
├── run_task_module/        # Spawns Codex/Claude CLI, manages workspace context
├── send_emails_module/     # Postmark API client, email composition
├── employees/              # Employee personas (AGENTS.md, CLAUDE.md, SOUL.md per employee)
├── skills/                 # 20+ agent skills (playwright-cli, pdf, docx, pptx, etc.)
└── employee.toml           # Employee configuration (runners, addresses, models)
```

### Key Binaries

- `rust_service` - Main HTTP server handling webhooks and scheduling
- `set_postmark_inbound_hook` - Updates Postmark webhook URL

### Data Flow

1. **Webhook intake**: Postmark/Slack/Discord webhooks hit `/postmark/inbound`, `/slack/events`, `/discord/webhooks`
2. **User lookup**: Scheduler identifies user, deduplicates messages
3. **Task creation**: One-shot or cron-scheduled tasks stored in SQLite
4. **Execution**: run_task_module spawns Codex or Claude CLI with workspace context
5. **Reply**: Generated HTML + attachments sent via Postmark

### Employee System

Each employee runs as a separate process with isolated state:
- `little_bear` (Oliver): Codex runner, port 9001
- `mini_mouse` (Maggie): Claude runner, port 9002
- `sticky_octopus` (Devin): Codex runner, port 9003
- `boiled_egg` (Proto): Codex runner, port 9004

Configuration in `DoWhiz_service/employee.toml`.

### Runtime State Location

```
$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/
├── state/                  # tasks.db, users.db, task_index.db
├── workspaces/<msg_id>/    # Per-task workspace, reply drafts
└── users/<user_id>/        # Per-user memory, mail archive
```

## Key Environment Variables

Required:
- `POSTMARK_SERVER_TOKEN` - Postmark API token
- `EMPLOYEE_ID` - Selects employee profile from employee.toml
- `AZURE_OPENAI_API_KEY_BACKUP` - Required for Codex employees

Optional:
- `RUN_TASK_DOCKER_IMAGE` - Run each task in a disposable Docker container
- `OLLAMA_ENABLED` - Local LLM message routing for simple queries
- `CODEX_DISABLED=1` - Bypass Codex CLI for testing

## CI/CD

GitHub Actions workflow (`.github/workflows/rust.yml`) runs on push/PR to main:
- `cargo build` in DoWhiz_service/
- `cargo test -p scheduler_module`
- `cargo test -p run_task_module`

## Skills System

Skills in `DoWhiz_service/skills/` are automatically copied to task workspaces. Each skill has a `SKILL.md` with instructions for agents. Notable skills: playwright-cli (browser automation), pdf/docx/pptx/xlsx (document processing), google-docs (collaboration).

## Testing

Live E2E tests require ngrok + Postmark account. Set `RUST_SERVICE_LIVE_TEST=1` and `RUN_CODEX_E2E=1` to run full pipeline tests:

```bash
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-NGROK-URL.ngrok-free.dev \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

## Testing Expectations

After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.
