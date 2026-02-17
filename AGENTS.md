# Repository Guidelines

## Project Structure & Module Organization
- `DoWhiz_service/`: Rust backend (scheduler, task runner, email/webhook handling). Modules live under `*_module/`, with shared assets in `skills/` and employee configs in `employees/` plus `employee.toml`.
- `website/`: React 19 + Vite marketing site (`src/`, `public/`, `eslint.config.js`).
- `function_app/`: Azure Functions custom-handler wrapper for the Rust service (build scripts, `host.json`, `HttpEntry/`).
- `assets/`, `api_reference_documentation/`, `example_files/`, `external/openclaw/`: supporting docs, reference material, and design assets.

## Build, Test, and Development Commands
Backend (from repo root):
```bash
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
# or manual
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```
Frontend:
```bash
cd website
npm install
npm run dev
```
Azure Functions wrapper:
```bash
./function_app/scripts/build_binary.sh
cd function_app && func host start --port 7071
```
Docker image (service):
```bash
docker build -t dowhiz-service .
```
For gateway and multi-employee setups, see `DoWhiz_service/README.md`.

## Coding Style & Naming Conventions
- Rust: format with `cargo fmt`, lint with `cargo clippy --all-targets --all-features`. Use `snake_case` for modules/functions, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
- Web: run `npm run lint` (ESLint config in `website/eslint.config.js`). Use `PascalCase` for React components and `camelCase` for variables.

## Testing Guidelines
- Rust unit/integration tests live under `DoWhiz_service/*_module/tests` and inline `#[cfg(test)]` modules. Run all tests with `cargo test` or module-specific with `cargo test -p run_task_module`.
- Live E2E (Postmark + ngrok) is documented in `DoWhiz_service/README.md` and `DoWhiz_service/run_task_module/tests/README.md`.
- Azure Functions local E2E: `./function_app/scripts/e2e_local.sh`.

## Testing Expectations
After completing code changes, you must design targeted, detailed unit tests and end-to-end tests to ensure both new and existing functionality behave as expected. Debug and resolve any issues found during test runs. If certain issues require manual intervention, provide a detailed report and follow-up steps.

## Commit & Pull Request Guidelines
- Commit history favors short, imperative summaries; optional Conventional prefixes appear (e.g., `feat:`, `fix(scope):`). Keep messages concise and scoped.
- PRs should include a short summary, tests run, and screenshots for UI changes (per `CONTRIBUTING.md`).

## Configuration & Secrets
- Service secrets live in `DoWhiz_service/.env` (copy from `.env.example`).
- Azure Functions uses `function_app/local.settings.json` for local-only settings.
- Never commit tokens, API keys, or Postmark credentials.

**Contributor principle:** Keep the codebase modular and easy to maintain. If a file grows too large (roughly 500–1000 lines), consider splitting it into smaller, well-defined modules with clear responsibilities.
