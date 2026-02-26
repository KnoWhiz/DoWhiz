# Staging vs Production Deployment (Single `.env`)

This guide keeps one `DoWhiz_service/.env` file and uses:
- base keys for production
- `STAGING_`-prefixed keys for staging
- `DEPLOY_TARGET=production|staging` to switch at startup time

`scripts/load_env_target.sh` loads `.env` and, in staging mode, maps `STAGING_FOO -> FOO`.

## 1) Isolated staging Service Bus (created)

Staging Service Bus resources:
- Resource group: `dowhiz-staging-rg-260226124234`
- Namespace: `dowhizsbstg260226124234`
- Queues: `ingestion-little_bear`, `ingestion-test`
- SAS rule: `dowhiz-staging-app` (Listen + Send)

Provisioning pattern (repeatable):
```bash
az group create -n <staging-rg> -l westus2
az servicebus namespace create -g <staging-rg> -n <staging-namespace> -l westus2 --sku Standard --min-tls 1.2 --zone-redundant true
az servicebus queue create -g <staging-rg> --namespace-name <staging-namespace> -n ingestion-little_bear --enable-duplicate-detection true --duplicate-detection-history-time-window PT10M
az servicebus queue create -g <staging-rg> --namespace-name <staging-namespace> -n ingestion-test --enable-duplicate-detection true --duplicate-detection-history-time-window PT10M
az servicebus namespace authorization-rule create -g <staging-rg> --namespace-name <staging-namespace> -n dowhiz-staging-app --rights Listen Send
az servicebus namespace authorization-rule keys list -g <staging-rg> --namespace-name <staging-namespace> -n dowhiz-staging-app
```

## 2) Required key split (single `.env`)

Most keys can be shared. Keep these isolated by environment:

| Purpose | Production key | Staging key |
|---|---|---|
| Postmark server token | `POSTMARK_SERVER_TOKEN` | `STAGING_POSTMARK_SERVER_TOKEN` |
| Postmark inbound hook URL | `POSTMARK_INBOUND_HOOK_URL` | `STAGING_POSTMARK_INBOUND_HOOK_URL` |
| Ingestion backend | `INGESTION_QUEUE_BACKEND` | `STAGING_INGESTION_QUEUE_BACKEND` |
| Service Bus connection | `SERVICE_BUS_CONNECTION_STRING` | `STAGING_SERVICE_BUS_CONNECTION_STRING` |
| Service Bus queue | `SERVICE_BUS_QUEUE_NAME` | `STAGING_SERVICE_BUS_QUEUE_NAME` |
| Service Bus test queue | `SERVICE_BUS_TEST_QUEUE_NAME` | `STAGING_SERVICE_BUS_TEST_QUEUE_NAME` |
| Queue-per-employee flag | `SERVICE_BUS_QUEUE_PER_EMPLOYEE` | `STAGING_SERVICE_BUS_QUEUE_PER_EMPLOYEE` |
| Gateway route config | `GATEWAY_CONFIG_PATH` | `STAGING_GATEWAY_CONFIG_PATH` |
| Employee config file | `EMPLOYEE_CONFIG_PATH` | `STAGING_EMPLOYEE_CONFIG_PATH` |
| Raw payload blob path prefix | `RAW_PAYLOAD_PATH_PREFIX` | `STAGING_RAW_PAYLOAD_PATH_PREFIX` |

Optional but recommended split:
- `WORKER_INSTANCE_ID_OLIVER` / `STAGING_WORKER_INSTANCE_ID_OLIVER`

## 3) Gateway routing isolation

Use separate gateway configs:
- Production: `gateway.toml`
- Staging: `gateway.staging.toml`

Use separate employee configs:
- Production: `employee.toml`
- Staging: `employee.staging.toml` (current default sender/receiver is only `dowhiz@deep-tutor.com`)

`gateway.staging.toml` currently routes to `little_bear` for:
- `dowhiz@deep-tutor.com`

## 4) Deploy commands

Run from repo root unless noted.

### Staging gateway + worker
```bash
export DEPLOY_TARGET=staging

./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```

If using one-command startup on VM:
```bash
export DEPLOY_TARGET=staging
./DoWhiz_service/scripts/start_all.sh
```

### Production gateway + worker
```bash
export DEPLOY_TARGET=production

./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```

If using one-command startup on VM:
```bash
export DEPLOY_TARGET=production
./DoWhiz_service/scripts/start_all.sh
```

## 5) Webhook notes (current staging URL)

Current staging inbound hook:
- `https://oliver.dowhiz.prod.ngrok.app/postmark/inbound`

When `DEPLOY_TARGET=staging`, scripts use `STAGING_POSTMARK_SERVER_TOKEN` and `STAGING_POSTMARK_INBOUND_HOOK_URL` automatically via env mapping.

## 6) Shared container with staging folder prefix

Using a dedicated staging folder in the same Azure Blob container is supported by:
- `STAGING_RAW_PAYLOAD_PATH_PREFIX="staging/ingestion_raw"`

This keeps payload object paths separated while sharing the same container and SAS credentials.

Tradeoffs:
- Pro: easy setup, no extra Azure resources.
- Con: not a hard security boundary (same SAS can read/write both prod and staging paths).
- Con: lifecycle/retention policies apply at container level unless you add prefix-aware jobs.

If you need stricter isolation later, move staging to a separate container or storage account.

## 7) Rollback (staging -> production)

1. Stop staging processes:
```bash
./DoWhiz_service/scripts/stop_all.sh
```
2. Switch target:
```bash
export DEPLOY_TARGET=production
```
3. Restart gateway/worker:
```bash
./DoWhiz_service/scripts/run_gateway_local.sh
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
```
4. Verify:
- `curl http://127.0.0.1:9100/health`
- `curl http://127.0.0.1:9001/health`
- confirm Postmark inbound hook points to production endpoint

## 8) Sanity checks

Queue counts:
```bash
az servicebus queue show -g dowhiz-staging-rg-260226124234 --namespace-name dowhizsbstg260226124234 -n ingestion-little_bear --query countDetails
```

Confirm active environment mapping at runtime:
```bash
DEPLOY_TARGET=staging bash -lc 'source DoWhiz_service/scripts/load_env_target.sh; echo \"$DEPLOY_TARGET|$SERVICE_BUS_QUEUE_NAME|$GATEWAY_CONFIG_PATH\"'
```
