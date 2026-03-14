# DoWhiz Operations Guide

Operational runbook for VM-based DoWhiz service operation.

Covers:
- `inbound_gateway`
- `rust_service` worker
- shared runtime `.env` policy
- health checks and incident triage

For deployment pipeline details, see:
- `DoWhiz_service/docs/staging_production_deploy.md`

## 1) Deployment Policy

- Staging deploy branch: `dev`
- Production deploy branch: `main`

Runtime environment policy:
- Runtime services read unprefixed keys from `DoWhiz_service/.env`.
- VM `.env` is generated from `ENV_COMMON + ENV_STAGING/ENV_PROD` in CI/CD.
- Runtime `.env` must not include `STAGING_*`/`PROD_*` keys.
- `DEPLOY_TARGET` is optional and used for runtime policy decisions.
- `POSTMARK_INBOUND_HOOK_URL` should point to the VM public endpoint; ngrok is local-only and should not run on staging/production VMs.

## 2) Expected Config Selection

- Staging:
  - `GATEWAY_CONFIG_PATH=gateway.staging.toml`
  - `EMPLOYEE_CONFIG_PATH=employee.staging.toml`
- Production:
  - `GATEWAY_CONFIG_PATH=gateway.toml`
  - `EMPLOYEE_CONFIG_PATH=employee.toml`

## 3) Common Paths

Typical repo location on VM:
- `/home/azureuser/server/.dowhiz/DoWhiz`

Service directory:
- `/home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service`

Common logs:
- `DoWhiz_service/gateway.log`
- `DoWhiz_service/worker.log`
- `/tmp/ngrok.log` (if ngrok used)

PM2 logs (if PM2-managed):
- `/home/azureuser/server/.pm2/logs/dw_gateway-out.log`
- `/home/azureuser/server/.pm2/logs/dw_worker-out.log`
- `/home/azureuser/server/.pm2/logs/dw_worker-error.log`

## 4) Start / Restart Patterns

On staging/production VMs, PM2 is the canonical runtime manager. Treat any leftover `systemd`
unit such as `dowhiz-oliver.service` as legacy and disable it so it cannot compete with PM2 or
confuse incident response.

### 4.1 Script-based (foreground/local style)

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh <employee_id> 9001 --skip-hook --skip-ngrok
```

Use `boiled_egg` on staging and `little_bear` on production.

### 4.2 PM2-based (recommended on VM)

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
set -a
source .env
set +a

# worker
pm2 restart dw_worker --update-env || \
  pm2 start ./target/release/rust_service --name dw_worker --cwd "$PWD" -- --host 0.0.0.0 --port "${RUST_SERVICE_PORT:-9001}"

# gateway
pm2 restart dw_gateway --update-env || \
  pm2 start ./target/release/inbound_gateway --name dw_gateway --cwd "$PWD"

pm2 save
pm2 list
```

If a legacy worker unit exists on a VM, disable it once:

```bash
sudo systemctl disable --now dowhiz-oliver.service || true
```

## 5) Health Checks

```bash
curl -sS http://127.0.0.1:9100/health
curl -sS http://127.0.0.1:9001/health
```

Queue/config sanity:

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
grep -E '^(INGESTION_QUEUE_BACKEND|SERVICE_BUS_CONNECTION_STRING|SERVICE_BUS_NAMESPACE|SERVICE_BUS_POLICY_NAME|SERVICE_BUS_POLICY_KEY|SERVICE_BUS_QUEUE_NAME|GATEWAY_CONFIG_PATH|EMPLOYEE_CONFIG_PATH|RUN_TASK_EXECUTION_BACKEND|DEPLOY_TARGET)=' .env
```

Process sanity:

```bash
pgrep -af inbound_gateway
pgrep -af rust_service
HOME=/home/azureuser/server pm2 list
```

## 6) Azure ACI Prerequisite (Worker)

When `RUN_TASK_EXECUTION_BACKEND=azure_aci`, the worker requires Azure Files mount at `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT`.

Check/mount helper:

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
./scripts/ensure_aci_share_mount.sh
```

If the mount is missing, worker startup should fail fast.

## 7) Live Email E2E Notes

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
RUN_CODEX_E2E=1 POSTMARK_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
```

If SMTP 25 is blocked by cloud policy, set:
- `POSTMARK_SMTP_PORT=2525`

## 8) Common Failure Patterns

1. Gateway startup error about backend
- Cause: `INGESTION_QUEUE_BACKEND` is not `servicebus`.
- Fix: set queue backend + Service Bus credentials.

2. Messages enqueued but worker idle
- Cause: queue mismatch or wrong `EMPLOYEE_ID` routing target.
- Fix: align queue/env and route targets.

3. Worker run_task fails immediately in staging/prod
- Cause: local backend selected while target policy forbids local execution.
- Fix: configure Azure ACI backend vars or adjust dev target for local environments.

4. Raw payload fetch/store failures
- Cause: storage backend credentials incomplete.
- Fix: verify selected backend and full credential set.

## 9) Rollback

Operational rollback (same code, restart services):

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
pm2 restart dw_gateway --update-env
pm2 restart dw_worker --update-env
```

Code rollback:
1. Checkout previous known-good commit on target branch.
2. Redeploy binaries and `.env` via CI/CD workflow.
3. Re-run health checks.
