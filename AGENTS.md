# Repository Guidelines

Try to keep the whole codebase modular and easy to maintain. If a file is too long (say more than 500 / 1000 lines), consider separate the code file into separate code files and make sure each one has its own well defined functionality.

## Project Structure & Module Organization
- `DoWhiz_service/` Rust workspace for the email service; main crates are `scheduler_module/`, `send_emails_module/`, and `run_task_module/`. Runtime artifacts live under `DoWhiz_service/.workspace/`.
- `website/` Vite + React site (`src/` for components, `public/` for static assets).
- `api_reference_documentation/` vendor API docs (Postmark/Gmail) used for reference only.
- `external/` vendored dependencies (OpenClaw); avoid edits unless intentionally updating upstream code.
- `example_files/`, `scripts/`, and `vision.md` hold sample inputs, helpers, and product notes.

## Build, Test, and Development Commands
- Rust service: `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`
- Update inbound hook when testing email flows: `ngrok http 9001` and then run `cargo run -p scheduler_module --bin set_postmark_inbound_hook -- --hook-url <public-url>/postmark/inbound`
- Unit tests: `cargo test -p scheduler_module`, `cargo test -p run_task_module`
- Live email E2E: `RUST_SERVICE_LIVE_TEST=1 POSTMARK_INBOUND_HOOK_URL=... POSTMARK_TEST_FROM=... cargo test -p scheduler_module --test service_real_email -- --nocapture`
- Website dev/build: `npm run dev`, `npm run build`, `npm run preview`, `npm run lint` (run from `website/`)

## Coding Style & Naming Conventions
- Rust: follow `rustfmt` defaults (`cargo fmt`); use `snake_case` for modules/functions and `UpperCamelCase` for types.
- Frontend: 2-space indentation in `.jsx` files, PascalCase React components (e.g., `App.jsx`), and ES module imports.
- Linting: `website/eslint.config.js` is the source of truth; fix lint errors before opening a PR.

## Testing Guidelines
- Tests live under each crate’s `tests/` directory and inline `#[test]` modules; keep test names descriptive.
- Postmark tests are live and require env vars (`POSTMARK_LIVE_TEST=1`, `POSTMARK_SERVER_TOKEN`); avoid running them without credentials.
- No JS test runner is configured yet, so rely on lint + manual verification for UI changes.

## Commit & Pull Request Guidelines
- Commit messages are short, imperative, and sentence-case (e.g., `Update service.rs`, `Implement webpage`).
- PRs should include: a concise summary, how to test, any env/config changes, and screenshots for website/UI updates.
