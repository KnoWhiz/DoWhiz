# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Code Style Guidelines

**Modularity**: Keep the codebase modular and maintainable. If a file exceeds 500-1000 lines, split it into separate files with well-defined, single-purpose functionality.

## Build & Test Commands

```bash
# Build
cargo build                                    # Debug build
cargo build --release                          # Release build
cargo build -p scheduler_module --release      # Build specific module

# Test
cargo test                                     # All tests
cargo test -p scheduler_module                 # Module-specific tests
cargo test -p scheduler_module --test scheduler_basic  # Single test file

# Lint & Format
cargo fmt --check                              # Check formatting
cargo fmt                                      # Auto-format
cargo clippy --all-targets --all-features      # Run linter

# E2E Tests (requires environment setup)
RUST_SERVICE_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
RUN_CODEX_E2E=1 cargo test -p scheduler_module --test scheduler_agent_e2e -- --nocapture
```

## Running the Service

```bash
# Single employee with ngrok auto-exposure
./scripts/run_employee.sh little_bear 9001

# Manual start
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# Docker (all employees)
docker-compose -f docker-compose.fanout.yml up
```

## Architecture Overview

This is a Rust microservice managing AI agent-driven email and chat workflows.

```
Inbound Webhooks (Postmark/Slack/Discord/Google Docs)
                    ↓
           Axum HTTP Server (service.rs)
                    ↓
    ┌───────────────┼───────────────┬──────────────┐
    ↓               ↓               ↓              ↓
Scheduler       Run Task        Send Emails    Adapters
(lib.rs)        (Codex/Claude)  (Postmark)     (Slack/Discord)
    ↓               ↓               ↓
 SQLite         Subprocess      HTTP Client
(tasks.db)      Execution
```

### Workspace Structure

| Module | Purpose |
|--------|---------|
| `scheduler_module/` | Core HTTP service, task scheduling, persistence, adapters |
| `run_task_module/` | Codex/Claude CLI subprocess execution |
| `send_emails_module/` | Postmark email API client |
| `skills/` | 21 agent skill modules (pdf, playwright-cli, google-docs, etc.) |
| `employees/` | Agent personas and configurations |
| `scripts/` | Deployment and automation scripts |

### Key Source Files

- `scheduler_module/src/service.rs` - Axum HTTP server, webhook handlers
- `scheduler_module/src/lib.rs` - Core scheduler, task types, SQLite persistence
- `scheduler_module/src/employee_config.rs` - Employee profile loading from TOML
- `scheduler_module/src/adapters/` - Channel-specific adapters (postmark, slack, discord, google_docs)
- `scheduler_module/src/bin/rust_service.rs` - Main service entry point
- `scheduler_module/src/bin/inbound_fanout.rs` - Multi-agent gateway
- `employee.toml` - Employee/agent profiles (addresses, runners, models, skills)

## Key Patterns

### Multi-Employee Architecture
- Each `EMPLOYEE_ID` has isolated workspace, database, and configuration
- `employee.toml` defines all profiles (little_bear/Oliver, mini_mouse/Maggie, etc.)
- Fanout gateway routes inbound messages to all employees; each processes its own addresses

### Task Types
```rust
enum TaskKind {
    SendReply(SendReplyTask),   // Send email, Slack msg, etc.
    RunTask(RunTaskTask),       // Invoke Codex/Claude to generate reply
    Noop,                       // Testing placeholder
}
```

### Agent Runners
- **Codex** (default): `@openai/codex` CLI, subprocess-based
- **Claude**: `@anthropic-ai/claude-code` CLI, subprocess-based
- Workspace passed via `--cd` argument; output files at known paths
- Skills copied to `.agents/skills` in workspace before execution

### Cron Scheduling
- 6-field format with seconds: `sec min hour day month weekday`
- UTC-based scheduling
- One-shot tasks converted to UTC timestamp

### Channel Abstraction
`Channel` enum (Email, Slack, Discord, Telegram) allows same task types across platforms.

## Runtime State

Default location: `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/`

```
state/
├── tasks.db              # Scheduler state
├── users.db              # User registry
└── task_index.db         # Due task index
workspaces/<message_id>/
├── workspace/            # Agent workspace
├── references/           # Email history
└── reply_email_draft.html
users/<user_id>/
├── state/tasks.db        # Per-user task queue
├── memory/               # Agent context
└── mail/                 # Email archive
```

## Environment Variables

Critical variables in `.env`:
- `EMPLOYEE_ID` - Which employee to run
- `RUST_SERVICE_HOST`, `RUST_SERVICE_PORT` - Server binding
- `POSTMARK_SERVER_TOKEN` - Email API
- `AZURE_OPENAI_API_KEY_BACKUP`, `AZURE_OPENAI_ENDPOINT_BACKUP` - Codex
- `ANTHROPIC_API_KEY` - Claude
- `SLACK_BOT_TOKEN`, `DISCORD_BOT_TOKEN` - Chat integrations
- `RUN_TASK_DOCKER_IMAGE` - Per-task Docker execution
- `OLLAMA_URL`, `OLLAMA_MODEL`, `OLLAMA_ENABLED` - Local LLM routing
