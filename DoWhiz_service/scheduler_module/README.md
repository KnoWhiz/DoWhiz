# scheduler_module

Core orchestration module for DoWhiz backend.

Responsibilities:
- task scheduling (`cron` + `one-shot`)
- queue-consumer execution path in `rust_service`
- ingress path in `inbound_gateway`
- outbound delivery (`SendReply`) across channels
- user/task index integration with Mongo-backed stores

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

## Test Commands

```bash
cd DoWhiz_service
cargo test -p scheduler_module
cargo test -p scheduler_module --test scheduler_basic
cargo test -p scheduler_module --test send_reply_outbound_e2e
```

Live tests and manual scripts are listed in:
- `reference_documentation/test_plans/DoWhiz_service_tests.md`

## Deployment Notes

For gateway/worker runtime and env policy, use:
- `DoWhiz_service/README.md`
- `DoWhiz_service/OPERATIONS.md`
