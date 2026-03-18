# DoWhiz_service Test Plan (Canonical)

Scope: `DoWhiz_service` only.

Status legend:
- `AUTO`: runs in CI/local without external paid services (may still require local toolchain)
- `LIVE`: requires real external credentials/services/accounts
- `MANUAL`: script-assisted or exploratory verification
- `PLANNED`: coverage gap to implement later

## 1) Policy

1. For every `DoWhiz_service` code change, run all **relevant AUTO** suites below.
2. For `LIVE`/`MANUAL`/`PLANNED`, report `SKIP` with reason unless explicitly executed.
3. If env/infra prevents a relevant AUTO suite from running, report `SKIP` with blocker details.
4. Runtime env policy in tests follows production behavior:
   - runtime `.env` uses unprefixed keys
   - `DEPLOY_TARGET` is policy metadata, not shell remapping

## 2) Test Suites

### 2.1 AUTO suites

| Test ID | Command | Scope | When Required |
|---|---|---|---|
| AUTO-RUN-01 | `cargo test -p run_task_module` | run_task core/unit/integration coverage | Any `run_task_module` change |
| AUTO-MAIL-01 | `cargo test -p send_emails_module` | Postmark payload construction + module behavior | Any `send_emails_module` change |
| AUTO-SCH-01 | `cargo test -p scheduler_module --test scheduler_basic` | scheduler core lifecycle | Any scheduler logic change |
| AUTO-SCH-02 | `cargo test -p scheduler_module --test scheduler_agent_e2e` | scheduler + run_task integration path | scheduler/run_task orchestration changes |
| AUTO-SCH-03 | `cargo test -p scheduler_module --test email_html_e2e` | inbound email HTML handling | email ingress/sanitization changes |
| AUTO-SCH-04 | `cargo test -p scheduler_module --test email_html_e2e_2` | advanced HTML/body fallback behavior | email ingress/sanitization changes |
| AUTO-SCH-05 | `cargo test -p scheduler_module --test github_env_e2e` | GitHub/x402 env propagation | env injection / github/x402 changes |
| AUTO-SCH-06 | `cargo test -p scheduler_module --test memory_e2e` | workspace memory sync | memory sync changes |
| AUTO-SCH-07 | `cargo test -p scheduler_module --test secrets_e2e` | per-user secrets sync | secret sync changes |
| AUTO-SCH-08 | `cargo test -p scheduler_module --test scheduler_followups` | scheduled follow-up persistence | follow-up/scheduler action changes |
| AUTO-SCH-09 | `cargo test -p scheduler_module --test scheduler_concurrency` | scheduler concurrency behavior | concurrency/throughput changes |
| AUTO-SCH-10 | `cargo test -p scheduler_module --test send_reply_outbound_e2e` | multi-channel outbound adapters (mocked) | outbound adapter changes |
| AUTO-SCH-11 | `cargo test -p scheduler_module --test scheduler_retry_notifications_e2e` | retry + notification behavior, including transient Codex retry alerts | retry/notification changes (`MONGODB_URI` required) |
| AUTO-SCH-12 | `cargo test -p scheduler_module --test scheduler_retry_notifications_slack_e2e` | Slack retry notifications | slack retry changes (`MONGODB_URI` required) |
| AUTO-SCH-13 | `cargo test -p scheduler_module --test scheduler_x402_env_e2e` | scheduler x402 env bridge | x402/env bridge changes |
| AUTO-SCH-14 | `cargo test -p scheduler_module --test thread_latest_epoch_e2e` | stale-thread cancellation, latest-epoch rule | thread state / email race changes |
| AUTO-SCH-15 | `cargo test -p scheduler_module` | broad scheduler_module sweep (includes unit + integration with env-gated skips) | major scheduler/gateway refactors |

### 2.2 LIVE suites

| Test ID | Command / Script | Scope | Required Env |
|---|---|---|---|
| LIVE-SCH-01 | `cargo test -p scheduler_module --test service_real_email -- --nocapture` | real inbound/outbound email flow | `RUST_SERVICE_LIVE_TEST=1` + Postmark + public hook URL (`POSTMARK_TEST_HOOK_URL`/`POSTMARK_INBOUND_HOOK_URL`; ngrok only for local tunneling) |
| LIVE-SCH-02 | `cargo test -p scheduler_module --test google_docs_cli_e2e -- --nocapture` | real Google Docs CLI behavior | Google OAuth creds + target docs |
| LIVE-SCH-03 | `cargo test -p scheduler_module --test unified_memo_e2e -- --ignored --nocapture` | unified memo/account/blob flow | service URL + supabase + azure blob creds + test account |
| LIVE-SCH-04 | `cargo test -p scheduler_module --test billing_e2e -- --nocapture` | billing/account db logic vs real DB | `SUPABASE_DB_URL` + test account data |
| LIVE-SCH-05 | `cargo test -p scheduler_module --test email_verification_e2e -- --nocapture` | email verification token flows | `SUPABASE_DB_URL` + test account data |
| LIVE-MAIL-01 | `POSTMARK_LIVE_TEST=1 cargo test -p send_emails_module -- --nocapture` | real Postmark delivery tests | Postmark credentials |

### 2.3 MANUAL suites

| Test ID | Script / Action | Scope |
|---|---|---|
| MAN-OPS-01 | `DoWhiz_service/scripts/test_auth_api.sh` | `/auth/*` endpoint roundtrip |
| MAN-OPS-02 | `DoWhiz_service/scripts/test_auth_link_only.sh` | link/verify flow without full account deletion |
| MAN-OPS-03 | `DoWhiz_service/scripts/test_blob_store.sh` | Azure Blob upload/download/list roundtrip |
| MAN-GWS-01 | `DoWhiz_service/scheduler_module/tests/google_workspace_cli_test.sh` | Google Workspace CLI smoke test |
| MAN-GWS-02 | `DoWhiz_service/scheduler_module/tests/google_workspace_e2e_test.sh` | Google Workspace comment workflow smoke |

## 3) Planned Gaps

| Gap ID | Priority | Gap |
|---|---|---|
| GAP-01 | P0 | Explicit stress test for ingestion queue multi-worker claim race |
| GAP-02 | P0 | Deterministic coverage for Azure ACI end-to-end lifecycle (create/run/cleanup) under CI-like conditions |
| GAP-03 | P1 | Automated failure-injection tests for outbound Slack/Discord/SMS API 5xx retry mapping |
| GAP-04 | P1 | Supabase raw payload backend upload/download integration tests (currently mostly env-dependent path checks) |
| GAP-05 | P2 | DST/timezone boundary cron behavior regression suite |

## 4) Reporting Template

Use this table in verification summaries when requested:

| Test ID | Status (PASS/FAIL/SKIP) | Evidence (short) | Notes / Reason |
|---|---|---|---|
| AUTO-... |  |  |  |

Rules:
- Include every relevant AUTO suite touched by the change.
- Mark each LIVE/MANUAL/PLANNED row as `SKIP` with reason unless executed.
