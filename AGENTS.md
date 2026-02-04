# Repository Guidelines

## Project Structure & Module Organization
- `mvp_python/email_pipeline/`: Python MVP for inbound email → Codex → reply flow (SMTP server, Postmark webhook helpers, local outbox).
- `DoWhiz_service/`: Rust workspace (`scheduler_module`, `run_task_module`, `send_emails_module`) for Postmark webhook intake and scheduled replies.
- `external/openclaw/`: Vendored upstream project with its own tooling and contributor guide; follow `external/openclaw/AGENTS.md` when working there.
- `api_reference_documentation/postmark_api/`: Postmark API reference notes used by the email pipeline.
- `example_files/`: Sample attachments used for local testing.

## Build, Test, and Development Commands
- Python deps: `pip install -r mvp_python/email_pipeline/requirements.txt`
- Run local pipeline server: `python -m mvp_python.email_pipeline.server`
- Send a local test email: `python -m mvp_python.email_pipeline.send_test_email --from you@… --to you@…`
- Read captured replies: `python -m mvp_python.email_pipeline.read_outbox`
- Rust service (from `DoWhiz_service`): `cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001`
- Rust tests: `cargo test -p scheduler_module` (add `-- --nocapture` for integration logs)
- For OpenClaw build/test commands, see `external/openclaw/AGENTS.md`.

## Coding Style & Naming Conventions
- Python: 4-space indentation; `snake_case` for modules/functions; `CamelCase` for classes.
- Rust: `rustfmt` defaults; `snake_case` for modules/functions; `UpperCamelCase` for types.
- Keep filenames aligned with existing patterns (e.g., `email_pipeline/*.py`, `*_module/src/*.rs`).
- OpenClaw-specific lint/formatting is defined in `external/openclaw/AGENTS.md`.

## Testing Guidelines
- Rust: prefer crate-scoped runs (`cargo test -p run_task_module`) before full workspace tests.
- Python: use the provided scripts (`send_test_email`, `real_email_test.py`) for functional checks; no pytest suite is wired up.
- OpenClaw tests (Vitest, etc.) are documented in `external/openclaw/AGENTS.md`.

## Commit & Pull Request Guidelines
- Git history shows short, descriptive messages (often concise phrases); keep commits focused and scoped.
- PRs should describe purpose, list test commands run, and note any new/changed env vars or webhook configuration.

## Security & Configuration Tips
- Keep secrets in `.env`; never commit `.env`, Postmark tokens, or Gmail credentials (ignored by `.gitignore`).
- Generated artifacts live in `mvp_python/email_pipeline/workspaces/`, `mvp_python/email_pipeline/outbox/`, and `DoWhiz_service/.workspace`; don’t add them to git.
