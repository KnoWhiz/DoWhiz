# scheduler_module

Core orchestration module for DoWhiz backend.

Responsibilities:
- task scheduling (`cron` + `one-shot`)
- queue-consumer execution path in `rust_service`
- ingress path in `inbound_gateway`
- outbound delivery (`SendReply`) across channels
- user/task index integration with Mongo-backed stores
- startup workspace product-layer bootstrap planning (blueprint validation, resource mapping, starter agents/tasks/artifacts, provisioning snapshot)

## Task Model

`TaskKind`:
- `SendReply` (serialized as `send_email` for backward compatibility)
- `RunTask`
- `Noop`

Schedules:
- `Cron` (6 fields: `sec min hour day month weekday`, UTC)
- `OneShot` (`run_at` timestamp)

## Channels

Supported channel enum values:
- `email`, `slack`, `discord`, `sms`, `telegram`, `whatsapp`, `wechat`
- `google_docs`, `google_sheets`, `google_slides`
- `bluebubbles`

## Key Entry Points

- `src/bin/inbound_gateway.rs`
- `src/bin/rust_service.rs`
- `src/service/*`
- `src/scheduler/*`
- `src/ingestion_queue.rs`
- `src/domain/workspace_blueprint.rs`
- `src/domain/resource_model.rs`
- `src/domain/agent_roster.rs`
- `src/domain/starter_tasks.rs`
- `src/domain/artifact_queue.rs`
- `src/service/startup_workspace/*`
- `src/service/workspace.rs` (bootstrap artifact persistence under `startup_workspace/`)
- `src/service/auth.rs` (`GET /api/workspace/provider-state`)

## Startup Workspace Layer

The startup workspace layer is intentionally separated from run-task runner concerns.

Key boundaries:
- Product modeling and bootstrap policy are in `scheduler_module`:
  - `domain/*`: canonical blueprint/resource/task/roster/artifact schemas
  - `service/startup_workspace/*`: intake normalization + bootstrap orchestration + runtime provider-state snapshots
- Execution/runtime concerns remain in `run_task_module` (for example Codex/Claude execution and filesystem prep).

Bootstrap output artifacts are persisted into each workspace under:
- `startup_workspace/blueprint.json`
- `startup_workspace/resources.json`
- `startup_workspace/agent_roster.json`
- `startup_workspace/starter_tasks.json`
- `startup_workspace/artifact_queue.json`
- `startup_workspace/provisioning.json`
- `startup_workspace/workspace_home_snapshot.json`

## Test Commands

```bash
cd DoWhiz_service
cargo test -p scheduler_module
cargo test -p scheduler_module --test scheduler_basic
cargo test -p scheduler_module --test send_reply_outbound_e2e
cargo test -p scheduler_module startup_workspace::
```

Live tests and manual scripts are listed in:
- `reference_documentation/test_plans/DoWhiz_service_tests.md`

## Deployment Notes

For gateway/worker runtime and env policy, use:
- `DoWhiz_service/README.md`
- `DoWhiz_service/OPERATIONS.md`
