# Repository Guidelines

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.

## Project Structure
- `DoWhiz_service/`: Rust backend (scheduler, run_task, email handling, agents). Key config: `DoWhiz_service/employee.toml`, skills in `DoWhiz_service/skills/`, personas in `DoWhiz_service/employees/`.
- `website/`: Vite + React product site (`website/src/`, `website/public/`, `website/eslint.config.js`).
- `external/openclaw/`: Reference implementation for multi-agent patterns (treat as vendor/reference unless a change is explicitly needed).
- `assets/`, `api_reference_documentation/`, `example_files/`: Static assets and docs/examples.
- Root files: `.env.example`, `Dockerfile`, `README.md`.

## Build, Test, and Development Commands
- `./DoWhiz_service/scripts/run_employee.sh little_bear 9001`: One-command local run (starts ngrok, updates Postmark hook, launches service).
- `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`: Manual service run.
- `cargo build -p scheduler_module --release`: Production build for the Rust service.
- `docker build -t dowhiz-service .`: Build the service container.
- `cd website && npm install && npm run dev`: Start the website dev server.
- `cd website && npm run build`: Production website build (output in `website/dist/`).

## Coding Style & Naming Conventions
- Rust: follow `rustfmt` and `clippy` (`cargo fmt --check`, `cargo clippy --all-targets --all-features`). Modules/crates use `snake_case` (e.g., `scheduler_module`).
- Web: follow ESLint rules in `website/eslint.config.js` (`npm run lint`). Keep JSX/components consistent with existing patterns.
- Config: environment variables are uppercase with underscores; employee IDs match `employee.toml` (e.g., `little_bear`).

## Testing Guidelines
- Unit tests: `cargo test` (or module-specific with `-p scheduler_module`, `-p send_emails_module`, `-p run_task_module`).
- Single test example: `cargo test -p scheduler_module --test scheduler_basic`.
- Live E2E (email + ngrok + Postmark) requires `RUST_SERVICE_LIVE_TEST=1` and often `RUN_CODEX_E2E=1` plus Postmark credentials; see `DoWhiz_service/README.md` before running.
- Website: `npm run lint` (no dedicated test runner currently).
- After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Commit & Pull Request Guidelines
- Commit messages are short and imperative; conventional prefixes appear occasionally (e.g., `feat:`/`fix:`) and PR/issue numbers are sometimes appended, e.g., `feat: Add Google Docs collaboration support for digital employees (#238)`.
- PRs must include a short summary, tests run, and screenshots for UI changes (`CONTRIBUTING.md`).

## Configuration & Secrets
- Start from `.env.example` and keep secrets out of git.
- Runtime state lives under `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/` and local workspaces may appear under `.workspace/`; do not commit generated data.
