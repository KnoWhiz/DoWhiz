# CLAUDE.md

This file guides Claude/Codex style agents working in this repository.

## Core Rules

- `external/` is reference-only; do not edit it.
- Keep code and docs consistent in the same change.
- For non-trivial work, branch from latest `dev`, commit in logical steps, and push.

## Architecture Summary (Current Code)

DoWhiz is a multi-channel digital employee platform.

Primary runtime split:
- `inbound_gateway` receives webhook/event ingress and enqueues ingestion envelopes.
- `rust_service` (worker) consumes queue items and executes scheduler/tasks.

Queue/storage behavior:
- Ingestion queue backend defaults to Postgres in code, but gateway requires `servicebus`.
- Raw payload storage defaults to Supabase; Azure Blob is recommended for gateway production flow.
- Scheduler/user/index state is Mongo-backed.
- Account/auth/billing data is stored in Supabase Postgres (`AccountStore`).

Channels in code:
- email, slack, discord, sms, telegram, whatsapp, google_docs/google_sheets/google_slides, bluebubbles.

## Runtime Config Policy

- Runtime services read unprefixed keys from `DoWhiz_service/.env`.
- VM `.env` should be merged from:
  - staging: `ENV_COMMON + ENV_STAGING`
  - production: `ENV_COMMON + ENV_PROD`
- Do not put `STAGING_*` / `PROD_*` keys into runtime `.env`.
- `DEPLOY_TARGET` is optional and affects runtime policy only.

Compatibility note:
- Some ingestion/raw-payload code paths also read `SCALE_OLIVER_*` aliases.
- Treat unprefixed keys as source of truth for current deployment and docs.

## Key Commands

Service build/test:
```bash
cd DoWhiz_service
cargo build
cargo test -p scheduler_module
cargo test -p run_task_module
cargo test -p send_emails_module
cargo fmt
cargo clippy --all-targets --all-features
```

Run worker:
```bash
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```

Run gateway:
```bash
./DoWhiz_service/scripts/run_gateway_local.sh
```

Frontend:
```bash
cd website
npm install
npm run dev
npm run lint
```

## RunTask Behavior (Important)

- `run_task_module` supports `codex` and `claude` runners.
- Execution backend:
  - `RUN_TASK_EXECUTION_BACKEND=local|azure_aci|auto`
  - `auto` resolves to `azure_aci` when `DEPLOY_TARGET` is `staging` or `production`, otherwise local.
- In staging/production targets, local codex execution is blocked by code unless backend is explicit Azure ACI flow.

## Testing Expectations

- Use `reference_documentation/test_plans/DoWhiz_service_tests.md` as canonical checklist.
- Run all relevant AUTO entries for touched areas.
- For LIVE/MANUAL/PLANNED entries, mark SKIP with reason unless run explicitly.

## Deployment Pointers

- Production deploy branch: `main`
- Staging deploy branch: `dev`
- Operational runbooks:
  - `DoWhiz_service/OPERATIONS.md`
  - `DoWhiz_service/docs/staging_production_deploy.md`
