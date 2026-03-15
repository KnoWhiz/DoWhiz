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

Staging ingest isolation policy:
- Use a staging-dedicated storage account for raw payload ingress.
- Current staging account: `dwhzoliverstg26261234`
- Current staging container: `ingestion-raw`
- In `ENV_STAGING`, explicitly set `AZURE_STORAGE_ACCOUNT` and `AZURE_STORAGE_CONTAINER_SAS_URL` so staging does not fall back to shared/common storage credentials.

### Notion Browser Integration (Optional)

| Key | Description |
|-----|-------------|
| `NOTION_EMPLOYEE_EMAIL` | Notion account email |
| `NOTION_EMPLOYEE_PASSWORD` | Notion account password |
| `NOTION_BROWSER_ENABLED` | Enable browser automation (true/false) |
| `NOTION_POLL_INTERVAL_SECS` | Poll interval in seconds (default: 45) |
| `NOTION_BROWSER_PROFILE_DIR` | Browser profile directory |
| `NOTION_BROWSER_HEADLESS` | Headless mode (true/false, recommend: false) |
| `NOTION_BROWSER_SLOW_MO` | Slow-mo delay in ms (recommend: 100) |
| `WEBDRIVER_URL` | WebDriver server URL (default: http://localhost:4444) |

Note: Notion browser integration uses WebDriver (geckodriver/chromedriver) for browser automation.
Recommended settings for anti-detection: `NOTION_BROWSER_HEADLESS=false`, `NOTION_BROWSER_SLOW_MO=100`.

## 3) VM Deployment

PM2 is the canonical process manager on staging/production VMs. Do not use the foreground
`run_gateway_local.sh` / `run_employee.sh` pair as the steady-state VM runtime; those are for
local debugging and manual foreground sessions only.

If a legacy `systemd` worker unit such as `dowhiz-oliver.service` exists on a VM, disable it so
PM2 is the only process supervisor for DoWhiz.

### Staging (`dev`)
```bash
ssh dowhizstaging
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
set -a
source .env
set +a
sudo systemctl disable --now dowhiz-oliver.service || true
pm2 restart dw_gateway --update-env || pm2 start ./target/release/inbound_gateway --name dw_gateway --cwd "$PWD"
pm2 restart dw_worker --update-env || pm2 start ./target/release/rust_service --name dw_worker --cwd "$PWD" -- --host 0.0.0.0 --port "${RUST_SERVICE_PORT:-9001}"
pm2 save
pm2 list
```

### Production (`main`)
```bash
ssh dowhizprod1
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
set -a
source .env
set +a
sudo systemctl disable --now dowhiz-oliver.service || true
pm2 restart dw_gateway --update-env || pm2 start ./target/release/inbound_gateway --name dw_gateway --cwd "$PWD"
pm2 restart dw_worker --update-env || pm2 start ./target/release/rust_service --name dw_worker --cwd "$PWD" -- --host 0.0.0.0 --port "${RUST_SERVICE_PORT:-9001}"
pm2 save
pm2 list
```

Do not use `scripts/start_all.sh` on staging/production VMs; it is local-only and will start ngrok plus rewrite Postmark inbound hook.

## 4) CI/CD Expectations

Deployment workflows should:
1. Write `.env` from `ENV_COMMON + ENV_STAGING/ENV_PROD`.
2. Fail if `.env` contains keys matching `^(STAGING_|PROD_)`.
3. Validate `GATEWAY_CONFIG_PATH` and `EMPLOYEE_CONFIG_PATH` exist and match expected target files.
4. After release binaries and `.env` are installed, source `.env` and restart PM2-managed services immediately so live traffic moves onto the new worker/gateway before any long-running follow-up work.
5. Disable any legacy `systemd` worker unit (for example `dowhiz-oliver.service`) so PM2 remains the only supervisor.
6. Keep build/deploy deterministic: checkout by trigger commit and pass `deploy_sha` from build job into deploy job so VM and image build use the exact same code revision.
7. Restrict manual release safety gates by branch (`workflow_dispatch` must run from `dev` for staging and `main` for production).
8. Skip staging/production workflow runs when PR merge changes only files under `website/**`.
9. Use layered image strategy for Azure ACI:
- `Dockerfile.base` for heavy shared dependencies.
- `Dockerfile.aci` for runtime assembly from prebuilt binaries.
10. Gate base image rebuild by file hash:
- Compare `md5(Dockerfile.base)` with `version_base.md5`.
- If changed, auto-bump patch in `version_base.txt`, update `version_base.md5`, and commit back.
- Base image tag format: `<runtime_repo>-base:<version_base>-<dockerfile_base_md5_prefix12>`.
11. Reuse base image when present in ACR; build only when missing.
12. If Azure ACI backend is enabled (`RUN_TASK_EXECUTION_BACKEND=azure_aci` or `auto` with `DEPLOY_TARGET in {staging,production}`), build/push image from GitHub Runner (not VM):
- checkout `deploy_sha`
- stage temporary context with `Dockerfile.aci`, `Dockerfile.base`, `.dockerignore`, `version_base.txt`, `DoWhiz_service/` (exclude runtime `.env` and `.workspace`)
- inject artifact binaries (`rust_service`, `inbound_fanout`, `inbound_gateway`, `google-docs`)
- run `az acr build`
- do not run `cargo build` during image build
13. Use `pm2 restart --update-env` so runtime env changes (for example `EMPLOYEE_ID`) are applied to existing processes, and finish with local health checks that allow a short retry window while worker/gateway bind their ports.
14. Staging may skip ACI rebuild for gateway-only source changes to reduce unnecessary image churn.

### CI/CD Maintenance Notes

1. `RUN_TASK_AZURE_ACI_IMAGE` must include an explicit tag; mutable env tags (`:staging`, `:prod`) are overwritten on each successful release.
2. Base image tags are version/hash derived and effectively immutable for cache reuse.
3. `paths-ignore: website/**` applies only to PR-triggered workflow execution; manual `workflow_dispatch` can still run full deployment.
4. If runtime binaries change, update all related points together:
- artifact upload list
- VM install step
- ACI build-context binary injection
- `COPY` entries in `Dockerfile.aci`
5. Keep Dockerfile responsibilities clear:
- `Dockerfile.base`: base layer
- `Dockerfile.aci`: ACI runtime image
- `Dockerfile.local`: local/other development build path

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
