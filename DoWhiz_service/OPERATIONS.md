# DoWhiz Operations Guide

This document is the operational runbook for VM-based deployment of:
- `inbound_gateway`
- `rust_service`
- ngrok ingress (when needed)

For full deployment matrix and rollback steps, see:
- `DoWhiz_service/docs/staging_production_deploy.md`

## 1) Deployment Policy

- Production deploy branch: `main` (CI/CD baseline)
- Staging deploy branch: `dev` (CI/CD rollout target)
- Optional staging hotfix branch: `staging-vm-setup`

Environment policy:
- Keep one `DoWhiz_service/.env`
- Production uses base keys
- Staging uses `STAGING_` keys
- Switch with `DEPLOY_TARGET=production|staging`
- Mapping is applied by `DoWhiz_service/scripts/load_env_target.sh`

## 2) VM Paths and Logs

Common repo paths in use:
- `/home/azureuser/server/.dowhiz/DoWhiz` (current)
- `/home/azureuser/server/DoWhiz` (legacy)

Service directory:
- `/home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service`

Script logs:
- `DoWhiz_service/gateway.log`
- `DoWhiz_service/worker.log`
- `/tmp/ngrok.log` or `/tmp/ngrok-dowhiz.log`

PM2 logs (if PM2 is used):
- `/home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log`
- `/home/azureuser/server/.pm2/logs/dowhiz-rust-service-out.log`
- `/home/azureuser/server/.pm2/logs/dowhiz-rust-service-error.log`

## 3) Azure Files Mount (Required for ACI Worker)

When `RUN_TASK_EXECUTION_BACKEND=azure_aci`, worker state/workspaces must live on the shared Azure Files mount configured by:
- `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT`

One-time bootstrap on VM:
```bash
sudo mkdir -p /etc/smbcredentials
sudo tee /etc/smbcredentials/<storage-account-name> >/dev/null <<'EOF'
username=<storage-account-name>
password=<storage-account-key>
EOF
sudo chmod 600 /etc/smbcredentials/<storage-account-name>

sudo mkdir -p /home/azureuser/server/.dowhiz/DoWhiz/run_task
echo '//<storage-account-name>.file.core.windows.net/<file-share> /home/azureuser/server/.dowhiz/DoWhiz/run_task cifs vers=3.0,credentials=/etc/smbcredentials/<storage-account-name>,dir_mode=0777,file_mode=0777,serverino,uid=1000,gid=1000,mfsymlinks,_netdev,nofail 0 0' | sudo tee -a /etc/fstab
sudo mount /home/azureuser/server/.dowhiz/DoWhiz/run_task
findmnt -T /home/azureuser/server/.dowhiz/DoWhiz/run_task
```

Startup guardrails:
- `DoWhiz_service/scripts/ensure_aci_share_mount.sh` is executed by:
  - `scripts/run_employee.sh`
  - `scripts/start_all.sh`
  - production/staging CI deploy workflows before PM2 restart
- If backend is `azure_aci`, the script will fail fast when mount is unavailable or `/etc/fstab` is missing the mount entry.

Live migration note (local run_task -> Azure Files):
- Keep a backup first, e.g. `run_task.local_backup_<timestamp>`.
- Do not use `rsync --ignore-existing` for live state (it can preserve stale/corrupt runtime artifacts forever).
- Use two-phase migration:
  1) Warm copy while services are online (exclude `*.db-journal`, `*.db-wal`, `*.db-shm`, `*.lock`)
  2) Short maintenance window: stop writer processes, run final sync without `--ignore-existing`, then restart.
- Validate before traffic restore:
  - required runtime directories (for example `workspaces/`, `users/`, `state/`) must exist
  - worker can connect to MongoDB (`MONGODB_URI`) and Supabase Postgres (`SUPABASE_DB_URL`)
- Verify worker health after migration: `curl -sS http://127.0.0.1:9001/health`.

## 4) Safety Rules

- Do not run destructive git commands on shared VMs.
- Do not restart production processes unless explicitly planned.
- Do not run `start_all.sh` on production unless you explicitly want to start ngrok and overwrite Postmark inbound hook to ngrok.

## 5) Staging Runbook (`dowhizstaging`)

Default branch for staging deploys: `dev`

```bash
ssh dowhizstaging
cd /home/azureuser/server/.dowhiz/DoWhiz

git fetch origin
git checkout dev
git pull --ff-only origin dev

export DEPLOY_TARGET=staging
./DoWhiz_service/scripts/start_all.sh
```

Optional hotfix/testing branch on staging:
```bash
git checkout staging-vm-setup
git pull --ff-only origin staging-vm-setup
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

## 6) Production Runbook (`dowhizprod1`)

Default branch for production deploys: `main`

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

## 7) Quick Verification

Resolve target-mapped runtime values:
```bash
DEPLOY_TARGET=staging bash -lc 'source DoWhiz_service/scripts/load_env_target.sh; echo "$DEPLOY_TARGET|$SERVICE_BUS_QUEUE_NAME|$GATEWAY_CONFIG_PATH|$EMPLOYEE_CONFIG_PATH"'
DEPLOY_TARGET=production bash -lc 'source DoWhiz_service/scripts/load_env_target.sh; echo "$DEPLOY_TARGET|$SERVICE_BUS_QUEUE_NAME|$GATEWAY_CONFIG_PATH|$EMPLOYEE_CONFIG_PATH"'
```

Check processes:
```bash
pgrep -af inbound_gateway
pgrep -af rust_service
pgrep -af "ngrok http"
```

If PM2 is used:
```bash
HOME=/home/azureuser/server pm2 list
pm2 logs dowhiz-inbound-gateway
pm2 logs dowhiz-rust-service
```

## 8) Live E2E Notes

- `scheduler_module/tests/service_real_email.rs` supports SMTP port override via `POSTMARK_SMTP_PORT`.
- On cloud VMs where SMTP port `25` is blocked, use `POSTMARK_SMTP_PORT=2525`.
- For staging, use `STAGING_POSTMARK_SMTP_PORT` with `DEPLOY_TARGET=staging`.

Example:
```bash
export DEPLOY_TARGET=staging
RUN_CODEX_E2E=1 POSTMARK_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
```

## 9) Common Failure Patterns

1. Gateway exits with backend error
- Cause: target-resolved `INGESTION_QUEUE_BACKEND` is not `servicebus`.
- Fix: verify `DEPLOY_TARGET` and corresponding `STAGING_`/base queue backend values.

2. Enqueue works but worker does not process
- Cause: queue mismatch between gateway and worker target config.
- Fix: verify `SERVICE_BUS_CONNECTION_STRING` and `SERVICE_BUS_QUEUE_NAME` after env mapping.

3. Staging accidentally using production `SCALE_OLIVER_*`
- `load_env_target.sh` syncs staging aliases, but validate with quick mapping commands above.

4. No outbound email in live tests
- Cause: SMTP blocked or sender not verified in Postmark.
- Fix: set SMTP port override and verify sender/domain signatures.

## 10) Rollback

Use:
- `DoWhiz_service/docs/staging_production_deploy.md` -> `Rollback (staging -> production)`

Minimal rollback:
```bash
./DoWhiz_service/scripts/stop_all.sh
export DEPLOY_TARGET=production
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```
