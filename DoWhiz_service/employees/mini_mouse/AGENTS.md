# Repository Guidelines

Try to keep the whole codebase modular and easy to maintain. If a file is too long (say more than 500 / 1000 lines), consider separate the code file into separate code files and make sure each one has its own well defined functionality.

## Project Structure & Module Organization
This is a Rust workspace with three crates:
- `scheduler_module/`: inbound webhook handling, scheduling, and service binaries.
- `send_emails_module/`: Postmark email sending and integration tests.
- `run_task_module/`: task execution and workspace orchestration.
Runtime state lives under `.workspace/` (SQLite state, user archives, and generated drafts). Build outputs go to `target/`.

## Build, Test, and Development Commands
- `cargo build`: build the full workspace.
- `cargo test`: run all unit and integration tests.
- `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`: start the service locally.
- `ngrok http 9001`: expose the service for Postmark inbound webhooks.
- `cargo run -p scheduler_module --bin set_postmark_inbound_hook -- --hook-url https://<ngrok>/postmark/inbound`: update Postmark to the new URL.
- `RUST_SERVICE_LIVE_TEST=1 POSTMARK_INBOUND_HOOK_URL=... POSTMARK_TEST_FROM=... cargo test -p scheduler_module --test service_real_email -- --nocapture`: run the real email end-to-end test.

## Coding Style & Naming Conventions
Use standard Rust formatting (rustfmt defaults). Follow Rust naming: `snake_case` for functions/modules, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Keep modules focused around the workspace crates; prefer small helpers over large monolithic files.

## Testing Guidelines
Tests live in `*/tests/*.rs` for integration coverage and in-module `#[test]` blocks for unit coverage. Name tests descriptively (e.g., `scheduler_concurrency`). Live Postmark tests require environment variables and are opt-in; keep them isolated and documented in test output.

## Commit & Pull Request Guidelines
Commit messages in history are short, imperative, and capitalized (e.g., "Update service.rs"); follow that style and avoid verbose prefixes unless needed. PRs should include a concise summary, the exact test commands run, and any required env/config changes (Postmark hooks, `.env` keys, or `CODEX_DISABLED=1`). Add screenshots only if UI/log output changes are meaningful.

## Configuration & Secrets
Create a local `.env` with Postmark and Codex-related keys; never commit secrets. Set `CODEX_DISABLED=1` for local runs when the Codex CLI is unavailable.

<SOUL>
Your name is Mini-Mouse, a tiny mouse, who is curious and quick and capable. You always get task done.
Go mice!
</SOUL>
