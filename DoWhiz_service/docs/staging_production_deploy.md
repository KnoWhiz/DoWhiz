# Staging vs Production Deployment (Unprefixed Keys)

This guide uses one runtime file (`DoWhiz_service/.env`) per VM, with **unprefixed** keys only.

- Staging VM `.env`: `${ENV_COMMON}` + `${ENV_STAGING}`
- Production VM `.env`: `${ENV_COMMON}` + `${ENV_PROD}`

`STAGING_*` and `PROD_*` key mapping is removed.

## 1) Deployment Contract

1. Runtime code reads unprefixed keys only.
2. `DEPLOY_TARGET` is optional and only affects runtime policy (for example `RUN_TASK_EXECUTION_BACKEND=auto` behavior).
3. Config file selection is explicit via `.env`:
- `GATEWAY_CONFIG_PATH`
- `EMPLOYEE_CONFIG_PATH`

Expected values:
- Staging: `gateway.staging.toml`, `employee.staging.toml`
- Production: `gateway.toml`, `employee.toml`

## 2) Required Environment Keys

Typical keys that differ by environment (still unprefixed):
- `POSTMARK_SERVER_TOKEN`
- `POSTMARK_INBOUND_HOOK_URL`
- `POSTMARK_TEST_SERVICE_ADDRESS`
- `INGESTION_QUEUE_BACKEND`
- `SERVICE_BUS_CONNECTION_STRING` (or `SERVICE_BUS_NAMESPACE` + `SERVICE_BUS_POLICY_NAME` + `SERVICE_BUS_POLICY_KEY`)
- `SERVICE_BUS_QUEUE_NAME`
- `SERVICE_BUS_TEST_QUEUE_NAME`
- `GATEWAY_CONFIG_PATH`
- `EMPLOYEE_CONFIG_PATH`
- `RAW_PAYLOAD_PATH_PREFIX`
- `RUN_TASK_EXECUTION_BACKEND`
- `RUN_TASK_AZURE_ACI_RESOURCE_GROUP`
- `RUN_TASK_AZURE_ACI_IMAGE`
- `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT`
- `RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT`
- `RUN_TASK_AZURE_ACI_STORAGE_KEY`

Raw payload download auth for Azure Blob can use any one of:
- `AZURE_STORAGE_CONTAINER_SAS_URL`
- `AZURE_STORAGE_CONTAINER_INGEST` + `AZURE_STORAGE_SAS_TOKEN` + `AZURE_STORAGE_ACCOUNT`
- `AZURE_STORAGE_CONNECTION_STRING_INGEST` (or `AZURE_STORAGE_CONNECTION_STRING`)

## 3) VM Deployment

### Staging (`dev`)
```bash
ssh dowhizstaging
cd /home/azureuser/server/.dowhiz/DoWhiz
git fetch origin
git checkout dev
git pull --ff-only origin dev
./DoWhiz_service/scripts/start_all.sh
```

### Production (`main`)
```bash
ssh dowhizprod1
cd /home/azureuser/server/.dowhiz/DoWhiz
git fetch origin
git checkout main
git pull --ff-only origin main
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```

## 4) CI/CD Expectations

Deployment workflows should:
1. Write `.env` from `ENV_COMMON + ENV_STAGING/ENV_PROD`.
2. Fail if `.env` contains keys matching `^(STAGING_|PROD_)`.
3. Validate `GATEWAY_CONFIG_PATH` and `EMPLOYEE_CONFIG_PATH` exist and match expected target files.
4. Source `.env` before PM2 restarts and use `pm2 restart --update-env` so runtime env changes (for example `EMPLOYEE_ID`) are applied to existing processes.

## 5) Health Checks

```bash
curl -sS http://127.0.0.1:9100/health
curl -sS http://127.0.0.1:9001/health
```

If PM2 is used:
```bash
pm2 list
```

## 6) Live Email E2E

```bash
RUN_CODEX_E2E=1 POSTMARK_LIVE_TEST=1 cargo test -p scheduler_module --test service_real_email -- --nocapture
```

If SMTP 25 is blocked on the VM, set `POSTMARK_SMTP_PORT=2525` in `.env`.

## 7) Rollback

1. Revert the deployment commit.
2. Re-run target environment workflow.
3. Re-check health endpoints.
4. If needed, restore previous `ENV_COMMON` / `ENV_STAGING` / `ENV_PROD` secret values.
