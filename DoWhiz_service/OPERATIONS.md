# DoWhiz Operations Guide

This runbook covers VM operations for:
- `inbound_gateway`
- `rust_service`
- optional ngrok ingress for local/live testing

For complete staging/production rollout and rollback details, see:
- `DoWhiz_service/docs/staging_production_deploy.md`

## 1) Deployment Policy

- Production deploy branch: `main`
- Staging deploy branch: `dev`

Environment policy:
- Services read unprefixed keys from `DoWhiz_service/.env`.
- Production VM `.env` is built from `ENV_COMMON + ENV_PROD`.
- Staging VM `.env` is built from `ENV_COMMON + ENV_STAGING`.
- Do not use `STAGING_*`/`PROD_*` keys in runtime `.env`.
- `DEPLOY_TARGET` is optional and used only for runtime policy (for example run_task backend behavior).

## 2) VM Paths and Logs

Common repo paths:
- `/home/azureuser/server/.dowhiz/DoWhiz` (current)
- `/home/azureuser/server/DoWhiz` (legacy)

Service directory:
- `/home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service`

Script logs:
- `DoWhiz_service/gateway.log`
- `DoWhiz_service/worker.log`
- `/tmp/ngrok.log` or `/tmp/ngrok-dowhiz.log`

PM2 logs (if used):
- `/home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log`
- `/home/azureuser/server/.pm2/logs/dowhiz-rust-service-out.log`
- `/home/azureuser/server/.pm2/logs/dowhiz-rust-service-error.log`

## 3) Azure Files Mount (Required for ACI Worker)

When `RUN_TASK_EXECUTION_BACKEND=azure_aci`, worker state/workspaces must live on Azure Files at:
- `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT`

One-time bootstrap on VM:
```bash
sudo mkdir -p /etc/smbcredentials
sudo tee /etc/smbcredentials/<storage-account-name> >/dev/null <<'EOF_CRED'
username=<storage-account-name>
password=<storage-account-key>
EOF_CRED
sudo chmod 600 /etc/smbcredentials/<storage-account-name>

sudo mkdir -p /home/azureuser/server/.dowhiz/DoWhiz/run_task
echo '//<storage-account-name>.file.core.windows.net/<file-share> /home/azureuser/server/.dowhiz/DoWhiz/run_task cifs vers=3.0,credentials=/etc/smbcredentials/<storage-account-name>,dir_mode=0777,file_mode=0777,serverino,uid=1000,gid=1000,mfsymlinks,_netdev,nofail 0 0' | sudo tee -a /etc/fstab
sudo mount /home/azureuser/server/.dowhiz/DoWhiz/run_task
findmnt -T /home/azureuser/server/.dowhiz/DoWhiz/run_task
```

Guardrails:
- `DoWhiz_service/scripts/ensure_aci_share_mount.sh` runs in local scripts and CI deploy flows.
- If backend is `azure_aci`, startup fails fast when mount is unavailable.

## 4) Safety Rules

- Do not run destructive git commands on shared VMs.
- Do not restart production unless planned.
- Do not run `start_all.sh` on production unless you explicitly want ngrok + webhook overwrite.

## 5) Staging Runbook (`dowhizstaging`)

```bash
ssh dowhizstaging
cd /home/azureuser/server/.dowhiz/DoWhiz

git fetch origin
git checkout dev
git pull --ff-only origin dev

./DoWhiz_service/scripts/start_all.sh
```

Health checks:
```bash
curl -sS http://127.0.0.1:9100/health
curl -sS http://127.0.0.1:9001/health
```

Expected staging profile:
- `GATEWAY_CONFIG_PATH` points to `gateway.staging.toml`
- `EMPLOYEE_CONFIG_PATH` points to `employee.staging.toml`
- staging mailbox defaults to `dowhiz@deep-tutor.com`

## 6) Production Runbook (`dowhizprod1`)

```bash
ssh dowhizprod1
cd /home/azureuser/server/.dowhiz/DoWhiz

git fetch origin
git checkout main
git pull --ff-only origin main

./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```

Health checks:
```bash
curl -sS http://127.0.0.1:9100/health
curl -sS http://127.0.0.1:9001/health
```

Expected production profile:
- `GATEWAY_CONFIG_PATH` points to `gateway.toml`
- `EMPLOYEE_CONFIG_PATH` points to `employee.toml`

## 7) Quick Verification

Check runtime env file:
```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
grep -E '^(GATEWAY_CONFIG_PATH|EMPLOYEE_CONFIG_PATH|RUN_TASK_EXECUTION_BACKEND|DEPLOY_TARGET)=' .env
if grep -Eq '^(STAGING_|PROD_)' .env; then echo 'unexpected prefixed keys'; fi
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

Example:
```bash
RUN_CODEX_E2E=1 POSTMARK_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
```

## 9) Common Failure Patterns

1. Gateway backend error
- Cause: `INGESTION_QUEUE_BACKEND` is not `servicebus`.
- Fix: verify queue-related unprefixed keys in `.env`.

2. Enqueue works but worker does not process
- Cause: queue mismatch between gateway and worker.
- Fix: verify `SERVICE_BUS_CONNECTION_STRING` and `SERVICE_BUS_QUEUE_NAME` in `.env`.

3. Worker startup fails before tasks
- Cause: `RUN_TASK_EXECUTION_BACKEND=azure_aci` but Azure Files mount missing.
- Fix: verify `/etc/fstab` entry and run `scripts/ensure_aci_share_mount.sh`.

4. No outbound email in live tests
- Cause: SMTP blocked or sender not verified.
- Fix: set `POSTMARK_SMTP_PORT=2525` and verify Postmark sender/domain.

## 10) Rollback

Minimal rollback:
```bash
./DoWhiz_service/scripts/stop_all.sh
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```
