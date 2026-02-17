# Repository Guidelines

## Project Structure & Module Organization
- `scheduler_module/`: inbound webhooks, scheduling, and service binaries.
- `send_emails_module/`: Postmark integration and email sending.
- `run_task_module/`: task execution and workspace orchestration (`run_task.rs`).
- `scripts/`: local run helpers (gateway, employees, E2E).
- `employees/`: per-employee agent configs (`AGENTS.md`, `CLAUDE.md`, `SOUL.md`).
- `skills/`: bundled agent skills.
- Config at repo root: `employee.toml`, `gateway.toml` (copy from `gateway.example.toml`).
- Build output: `target/` (generated).

## Build, Test, and Development Commands
- `cargo build`: build the workspace.
- `cargo test`: run all unit/integration tests.
- `cargo test -p scheduler_module`: module-specific tests.
- `cargo clippy --all-targets --all-features`: lint.
- `cargo fmt --check`: formatting check.
- `./scripts/run_employee.sh little_bear 9001`: run a local worker with ngrok + Postmark hook update.
- `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`: run the service directly.
- `./scripts/run_gateway_local.sh`: start the inbound gateway locally.
- `docker build -t dowhiz-service .`: build the container image.

## Coding Style & Naming Conventions
Use rustfmt defaults. Follow Rust naming: `snake_case` for functions/modules, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Keep files and modules focused; split large files instead of growing monoliths. Prefer explicit error handling and structured logging via `tracing`.

## Testing Guidelines
Unit tests live in `src` with `#[test]`; integration tests live in `*/tests/*.rs`. Live E2E email tests are opt-in and require env vars like `RUST_SERVICE_LIVE_TEST=1`, `POSTMARK_SERVER_TOKEN`, and `POSTMARK_INBOUND_HOOK_URL`. Example: `cargo test -p scheduler_module --test service_real_email -- --nocapture`.

## Testing Expectations
After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Commit & Pull Request Guidelines
Recent history uses short, imperative, sentence-case messages (e.g., “Update .env.example”, “Refactor inbound gateway logic”). Conventional Commit prefixes appear occasionally (e.g., `fix(runtime): ...`, `feat: ...`); use them when helpful, but keep subject lines concise. PRs should include a summary, the exact test commands run, and any config/env changes. Update `.env.example` when adding new required variables.

## Configuration & Secrets
Copy `.env.example` to `.env` and keep secrets out of git. `employee.toml` selects employee profiles; `gateway.toml` routes inbound webhooks. For agent behavior changes, check `employees/<id>/AGENTS.md` and `employees/<id>/CLAUDE.md`.

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.
