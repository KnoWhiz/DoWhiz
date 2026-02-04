# Repository Guidelines

## Project Structure & Module Organization
This is a Rust workspace with three crates:
- `scheduler_module/` runs the webhook service and UTC task scheduler (binaries in `scheduler_module/src/bin/`).
- `run_task_module/` invokes the Codex CLI to generate reply drafts and attachments.
- `send_emails_module/` sends HTML email + attachments via Postmark.

Tests live under each crate’s `tests/` directory. Runtime artifacts and state are written to `.workspace/run_task/...` and `target/`.

## Build, Test, and Development Commands
- `cargo build` builds the workspace.
- `cargo test` runs all crate tests (live tests are gated by env vars).
- `cargo test -p scheduler_module`, `cargo test -p run_task_module`, `cargo test -p send_emails_module` run targeted suites.
- `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001` starts the service.
- `cargo run -p scheduler_module --bin set_postmark_inbound_hook -- --hook-url https://…/postmark/inbound` updates the Postmark hook.

Required env comes from `.env` (for example: `POSTMARK_SERVER_TOKEN`, `AZURE_OPENAI_API_KEY_BACKUP`, `AZURE_OPENAI_ENDPOINT_BACKUP`). Set `CODEX_DISABLED=1` to bypass Codex during runs.

## Coding Style & Naming Conventions
- Rust 2021 edition; format with `cargo fmt` (no custom rustfmt config).
- Use idiomatic naming: `snake_case` modules/functions, `UpperCamelCase` types, `SCREAMING_SNAKE_CASE` constants.
- Follow existing patterns for error enums (the codebase uses `thiserror`).

## Testing Guidelines
- Integration tests live under each crate’s `tests/`.
- Live Postmark tests are opt-in: set `POSTMARK_LIVE_TEST=1` plus `POSTMARK_SERVER_TOKEN` and `POSTMARK_TEST_TO` for `send_emails_module`.
- Live end-to-end service tests are opt-in: set `RUST_SERVICE_LIVE_TEST=1` plus `POSTMARK_INBOUND_HOOK_URL` and `POSTMARK_TEST_FROM` for `scheduler_module`.
- `run_task_module` tests run offline using a fake `codex` binary.

## Commit & Pull Request Guidelines
- Commit history uses short, direct subjects (for example, “Update gitignore”); follow that tone.
- PRs should include a concise summary, relevant env vars used, and exact test commands run (or “not run” with reason). If behavior affects email content, attach a sample output or log snippet.

## Security & Configuration Notes
- Keep credentials in `.env` and never commit secrets.
- `.workspace/` contains generated drafts, attachments, and scheduler state; treat it as local, transient data.
