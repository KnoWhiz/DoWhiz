# DoWhiz Operations Guide

This document is the operational runbook for VM-based deployment of:
- `inbound_gateway`
- `rust_service`
- ngrok ingress (when needed)

For detailed deployment matrix and rollback between staging/prod targets, see:
- `DoWhiz_service/docs/staging_production_deploy.md`

## 1) Deployment Policy

- Production deploy branch: `main` (CI/CD baseline)
- Staging deploy branch: `dev` (CI/CD rollout target)
- Transition/hotfix branch for staging (manual): `staging-vm-setup`

Environment policy:
- Use one `DoWhiz_service/.env`
- Production uses base keys
- Staging uses `STAGING_` keys
- Runtime switching is controlled by `DEPLOY_TARGET=production|staging`
- Mapping is applied by `DoWhiz_service/scripts/load_env_target.sh`

## 2) VM Paths and Logs

Common VM repo path:
- `/home/azureuser/server/.dowhiz/DoWhiz`

Service directory:
- `/home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service`

Default logs when using helper scripts:
- `DoWhiz_service/gateway.log`
- `DoWhiz_service/worker.log`
- `/tmp/ngrok.log` (or `/tmp/ngrok-dowhiz.log`)

## 3) Safety Rules

- Do not run destructive git commands on shared VMs.
- Do not restart production processes unless explicitly planned.
- Do not run `start_all.sh` on production unless you intentionally want to start ngrok and overwrite Postmark inbound hook to ngrok.

## 4) Staging Runbook (`dowhizstaging`)

```bash
ssh dowhizstaging
cd /home/azureuser/server/.dowhiz/DoWhiz

git fetch origin
git checkout staging-vm-setup
git pull --ff-only origin staging-vm-setup

export DEPLOY_TARGET=staging
./DoWhiz_service/scripts/start_all.sh
```

Health checks:
```bash
curl -sS http://127.0.0.1:9100/health
curl -sS http://127.0.0.1:9001/health
```

Expected staging behavior:
- inbound route only for `dowhiz@deep-tutor.com`
- default outbound sender is `dowhiz@deep-tutor.com`
- queue/storage/postmark use `STAGING_` values

## 5) Production Runbook (`dowhizprod1`)

```bash
ssh dowhizprod1
cd /home/azureuser/server/.dowhiz/DoWhiz

git fetch origin
git checkout main
git pull --ff-only origin main

export DEPLOY_TARGET=production
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```

Health checks:
```bash
curl -sS http://127.0.0.1:9100/health
curl -sS http://127.0.0.1:9001/health
```

## 6) Quick Verification Commands

Check active env mapping:
```bash
DEPLOY_TARGET=staging bash -lc 'source DoWhiz_service/scripts/load_env_target.sh; echo "$DEPLOY_TARGET|$SERVICE_BUS_QUEUE_NAME|$GATEWAY_CONFIG_PATH|$EMPLOYEE_CONFIG_PATH"'
DEPLOY_TARGET=production bash -lc 'source DoWhiz_service/scripts/load_env_target.sh; echo "$DEPLOY_TARGET|$SERVICE_BUS_QUEUE_NAME|$GATEWAY_CONFIG_PATH|$EMPLOYEE_CONFIG_PATH"'
```

Check key processes:
```bash
pgrep -af inbound_gateway
pgrep -af rust_service
pgrep -af "ngrok http"
```

## 7) Live E2E Notes

- `scheduler_module/tests/service_real_email.rs` supports SMTP port override via `POSTMARK_SMTP_PORT`.
- On cloud VMs where SMTP port `25` is blocked, set `POSTMARK_SMTP_PORT=2525` (or staging equivalent through `STAGING_POSTMARK_SMTP_PORT` + `DEPLOY_TARGET=staging`).

Example:
```bash
export DEPLOY_TARGET=staging
RUN_CODEX_E2E=1 POSTMARK_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
```

## 8) Common Failure Patterns

1. Gateway exits immediately with backend error
- Cause: target-resolved `INGESTION_QUEUE_BACKEND` is not `servicebus`.
- Fix: verify `DEPLOY_TARGET` and `STAGING_INGESTION_QUEUE_BACKEND` / `INGESTION_QUEUE_BACKEND`.

2. Messages enqueue but worker does not process
- Cause: queue mismatch between gateway and worker target config.
- Fix: verify `SERVICE_BUS_CONNECTION_STRING` + `SERVICE_BUS_QUEUE_NAME` after `load_env_target.sh` mapping.

3. Staging accidentally using production `SCALE_OLIVER_*`
- Protection exists in `load_env_target.sh` (staging alias sync), but validate with the quick mapping command above.

4. No outbound email in live tests
- Cause: SMTP blocked or wrong sender signature.
- Fix: set SMTP port override and verify sender/domain in Postmark.

## 9) Rollback

Use:
- `DoWhiz_service/docs/staging_production_deploy.md` -> section `Rollback (staging -> production)`

Minimal rollback steps:
```bash
./DoWhiz_service/scripts/stop_all.sh
export DEPLOY_TARGET=production
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```
