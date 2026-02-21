# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DoWhiz Service is a Rust microservice handling inbound channels (Postmark email, Slack, Discord, Google Docs, BlueBubbles/iMessage, Twilio SMS, Telegram, WhatsApp), AI-powered task execution (Codex/Claude CLI), and outbound message delivery. It supports multiple "employee" profiles (Oliver, Maggie, Devin, Boiled-Egg) with isolated workspaces and configurable AI runners.

## Build & Test Commands

```bash
# Build
cargo build --release

# Run all tests
cargo test

# Module-specific tests
cargo test -p scheduler_module
cargo test -p send_emails_module
cargo test -p run_task_module

# Single test file
cargo test -p scheduler_module --test scheduler_basic

# Linting
cargo clippy --all-targets --all-features
cargo fmt --check

# Run service locally (per-employee)
./scripts/run_employee.sh little_bear 9001
./scripts/run_employee.sh mini_mouse 9002 --skip-hook

# Run inbound gateway
./scripts/run_gateway_local.sh

# Live E2E test (requires ngrok + Postmark setup)
RUST_SERVICE_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
```

## Architecture

### Message Flow
```
External Events (Postmark/Slack/Discord/GoogleDocs/BlueBubbles/Twilio SMS/Telegram/WhatsApp)
    ↓
Inbound Gateway (port 9100) - deduplicates, stores raw payloads in Azure Blob, routes to single employee
    ↓
Ingestion Queue (Service Bus for gateway; Postgres optional/legacy)
    ↓
Worker Service (ports 9001-9004) - per-employee consumer + HTTP server
    ↓
Scheduler (SQLite) - persists tasks, creates RunTask
    ↓
AI Agent Execution - codex CLI or claude CLI with workspace setup
    ↓
Reply Generation - parse schedule blocks, generate HTML, attach files
    ↓
Outbound Delivery - send via channel adapter, archive to user mail
```

### Key Modules

- **scheduler_module/src/lib.rs**: Core types (`TaskKind`, `Schedule`, `Scheduler<E>`)
- **scheduler_module/src/service/server.rs**: Axum HTTP server setup (worker)
- **scheduler_module/src/channel.rs**: `Channel` enum abstracting Email/Slack/Discord/SMS/Telegram/WhatsApp/GoogleDocs/BlueBubbles
- **scheduler_module/src/ingestion.rs**: `InboundMessage` envelope structure
- **scheduler_module/src/message_router.rs**: quick-response classifier (OpenAI)
- **scheduler_module/src/adapters/**: Channel-specific implementations (postmark.rs, slack.rs, discord.rs, google_docs.rs, bluebubbles.rs)
- **scheduler_module/src/adapters/whatsapp.rs**: WhatsApp inbound/outbound adapter
- **scheduler_module/src/bin/rust_service.rs**: Main entry point
- **scheduler_module/src/bin/inbound_gateway.rs**: Message router/deduplicator
- **scheduler_module/src/service_bus_queue.rs**: Azure Service Bus ingestion queue
- **scheduler_module/src/raw_payload_store.rs**: Raw payload storage (Azure Blob / Supabase)

### Configuration Files

- **employee.toml**: Employee profiles (id, runner, addresses, personality files)
- **gateway.toml**: Inbound gateway routing targets
- **.env**: Secrets (POSTMARK_SERVER_TOKEN, AZURE_OPENAI_*, SLACK_*, DISCORD_*, GOOGLE_*, WHATSAPP_*)

### Database Schema (SQLite)

- `tasks.db`: Scheduler state with `tasks`, `send_email_tasks`, `run_task_tasks`, `task_executions` tables
- `users.db`: User registry (normalized email, timestamps)
- `task_index.db`: Global index for due task polling

### Workspace Isolation

Each task runs in isolated workspace at `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/`:
- `workspace/`: Agent working directory
- `references/past_emails/`: Hydrated email history
- `reply_email_draft.html`: Generated reply
- `reply_email_attachments/`: Output files

### Employee System

| ID | Name | Runner | Primary Channel |
|----|------|--------|-----------------|
| `little_bear` | Oliver | codex | Email |
| `mini_mouse` | Maggie | claude | Email |
| `sticky_octopus` | Devin | codex | Email |
| `boiled_egg` | Boiled-Egg | codex | Email, Slack, Discord, BlueBubbles |

### Skills System

Skills in `skills/` directory provide agent capabilities (playwright-cli, pdf, docx, xlsx, google-docs, etc.). Each skill has `SKILL.md` metadata and optional `references/` guides.

## Testing Expectations

After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Key Patterns

- **Channel Abstraction**: Unified `InboundMessage` struct with `Channel` enum for platform independence
- **Multi-Tenant Design**: Each employee isolated under `.workspace/<id>` with shared skills pool
- **Deduplication**: Message IDs + hash lists prevent duplicate processing
- **Cron Scheduling**: 6-field format `sec min hour day month weekday` (UTC)
- **Per-Task Docker**: Optional isolated container execution via `RUN_TASK_DOCKER_IMAGE`

## Important Environment Variables

- `EMPLOYEE_ID`: Selects employee profile
- `RUST_SERVICE_PORT`: HTTP server port (default 9001)
- `CODEX_DISABLED=1`: Bypass Codex CLI for testing
- `CODEX_BYPASS_SANDBOX=1`: Required inside Docker sometimes
- `RUN_TASK_DOCKER_IMAGE`: Enable per-task container execution
- `INGESTION_QUEUE_BACKEND`: `servicebus` for gateway, `postgres` for legacy worker-only setups
- `SERVICE_BUS_CONNECTION_STRING` / `SERVICE_BUS_QUEUE_NAME`: Service Bus ingestion queue config
- `RAW_PAYLOAD_STORAGE_BACKEND`: `azure` for gateway, `supabase` for legacy payload storage
- `AZURE_STORAGE_ACCOUNT` / `AZURE_STORAGE_CONTAINER_INGEST` / `AZURE_STORAGE_SAS_TOKEN`: Azure Blob raw payload storage config
- `AZURE_STORAGE_CONNECTION_STRING` / `AZURE_STORAGE_CONTAINER`: Azure Blob memo storage config
- `SUPABASE_DB_URL`: Postgres ingestion queue (legacy)
- `OPENAI_API_KEY`: Enable message router quick replies
- `TELEGRAM_BOT_TOKEN`: Telegram bot token (or per-employee `DO_WHIZ_<EMPLOYEE>_BOT`)
- `TWILIO_ACCOUNT_SID`: Twilio account SID (SMS outbound)
- `TWILIO_AUTH_TOKEN`: Twilio auth token (SMS outbound + webhook verification)
- `TWILIO_WEBHOOK_URL`: Public URL used to validate Twilio signatures
- `WHATSAPP_ACCESS_TOKEN`: WhatsApp Cloud API access token
- `WHATSAPP_PHONE_NUMBER_ID`: WhatsApp phone number ID for outbound sends
- `WHATSAPP_VERIFY_TOKEN`: WhatsApp webhook verification token
- `BLUEBUBBLES_URL`: BlueBubbles server URL (iMessage outbound)
- `BLUEBUBBLES_PASSWORD`: BlueBubbles server password
- `SLACK_CLIENT_ID`: Slack OAuth client id
- `SLACK_CLIENT_SECRET`: Slack OAuth client secret
- `SLACK_REDIRECT_URI`: Slack OAuth redirect URI
- `DISCORD_BOT_TOKEN`: Discord bot token
- `DISCORD_BOT_USER_ID`: Discord bot user id (filter bot messages)

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.
