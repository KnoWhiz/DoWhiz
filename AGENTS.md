# Repository Guidelines

## Project Structure & Module Organization
DoWhiz is split between a Rust service, a React website, and an Azure Functions wrapper. Key locations:
- `DoWhiz_service/`: Rust workspace (`scheduler_module/`, `run_task_module/`, `send_emails_module/`) plus `employee.toml` and `skills/`.
- `website/`: Vite + React site (`src/`, `public/`, `vite.config.js`).
- `function_app/`: Azure Functions custom handler, build scripts, and local settings.
- `api_reference_documentation/`: Postmark/Gmail reference docs (read-only).
- `assets/`, `example_files/`, `scripts/`, `external/`: shared assets, samples, helper scripts, vendored code.

## Build, Test, and Development Commands
- `EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`: run the Rust service locally.
- `./DoWhiz_service/scripts/run_employee.sh little_bear 9001`: one-command local run (ngrok + Postmark hook).
- `cargo test -p scheduler_module` and `cargo test -p run_task_module`: core Rust tests.
- `POSTMARK_LIVE_TEST=1 POSTMARK_SERVER_TOKEN=... cargo test -p scheduler_module --test service_real_email -- --nocapture`: live Postmark E2E.
- `cd website && npm install && npm run dev`: start the website (Vite dev server).
- `cd website && npm run lint`: lint website sources (ESLint).
- `./function_app/scripts/build_binary.sh`: build the Azure Functions Linux binary.
- `cd function_app && func host start --port 7071`: run the Functions wrapper locally.

## Coding Style & Naming Conventions
Rust follows rustfmt defaults; run `cargo fmt` before submitting. Use Rust naming conventions (`snake_case` for modules/functions, `CamelCase` for types). Website code follows ESLint rules in `website/eslint.config.js`; React components use `PascalCase`, hooks follow `useX`, and CSS class names are kebab-case (see `website/src/index.css`). Match existing formatting in each folder.

## Testing Guidelines
Unit tests live alongside modules with `#[test]`; integration tests live in `DoWhiz_service/*/tests/*.rs`. Name tests for behavior (e.g., `scheduler_concurrency`). Live Postmark tests are opt-in and require environment variables; keep them isolated and document the exact command in PRs.

## Commit & Pull Request Guidelines
Recent history uses short, imperative, capitalized summaries (e.g., "Add run_employee.sh", "Update openclaw"); follow that style and include PR numbers when applicable. PRs should include a concise summary, test commands run, and any required env/config changes. Add screenshots for UI changes.

## Configuration & Agent Notes
Create a local `.env` from `.env.example` and never commit secrets. `DoWhiz_service/employee.toml` selects employee profiles (set `EMPLOYEE_ID`). Per-employee agent instructions live in `DoWhiz_service/employees/*/AGENTS.md` and `DoWhiz_service/employees/*/CLAUDE.md`; shared skills live in `DoWhiz_service/skills/`.
