# Repository Guidelines

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.

## Project Structure & Module Organization
- `scheduler_module/`: core Rust service, HTTP handlers, and binaries in `src/bin/` (e.g., `rust_service`, `set_postmark_inbound_hook`); integration tests in `tests/`.
- `send_emails_module/`: outbound email delivery logic with `src/` and `tests/`.
- `run_task_module/`: task execution and workspace orchestration; wrapper entrypoint in `run_task.rs`, tests in `tests/`.
- `scripts/`: local/dev helper scripts such as `run_employee.sh`, `run_fanout_local.sh`, `run_all_employees_docker.sh`.
- `employees/`: per-employee configuration and agent instruction files (`AGENTS.md`, `CLAUDE.md`, `SOUL.md`).
- `skills/`: bundled agent skills referenced by employees.
- `employee.toml`: shared employee definitions and runtime settings.

## Build, Test, and Development Commands
- `cargo build -p scheduler_module --release`: build the service binary.
- `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`: run the service locally.
- `./scripts/run_employee.sh little_bear 9001`: one-command local run (ngrok + hook wiring).
- `./scripts/run_fanout_local.sh`: local fanout gateway for multiple employees.
- `cargo test`: run all tests in the workspace.
- `cargo test -p scheduler_module --test scheduler_basic`: run one integration test.
- `cargo clippy --all-targets --all-features`: lint checks.
- `cargo fmt --check`: formatting verification.

## Coding Style & Naming Conventions
- Rust style is standard `rustfmt` with default settings (4-space indent).
- Naming: `snake_case` for modules/functions/variables, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants.
- Keep binaries in `scheduler_module/src/bin/` with filenames matching the binary name.

## Testing Guidelines
- Primary framework is Rust’s built-in test runner (`cargo test`).
- Integration/E2E tests live under `*/tests/*.rs` (e.g., `scheduler_module/tests/service_real_email.rs`).
- Live E2E flows require Postmark credentials, ngrok, and optional AI keys; keep them opt-in and document required env vars when adding new ones.
- After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Commit & Pull Request Guidelines
- Recent history shows short, imperative commit summaries; some use Conventional Commit prefixes like `feat:`. Follow that pattern when appropriate and include issue/PR numbers if relevant.
- PRs should describe behavioral changes, note config/env updates, and include test evidence (commands + results). Update docs if you change run scripts or onboarding steps.

## Agent & Configuration Notes
- Agent behavior is scoped by `employee.toml` and per-employee instructions in `employees/<id>/AGENTS.md`.
- Keep secrets in local `.env` only; do not commit credentials.
