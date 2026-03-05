# Azure VM Worker Guide (Docker-Isolated RunTask)

This guide describes running `rust_service` on an Azure VM with host Docker and Azure Files-backed workspace storage.

Use this for environments where worker tasks may run with dockerized execution while keeping user workspace state durable.

For full staging/production branch/env/runbook policy, see:
- `DoWhiz_service/docs/staging_production_deploy.md`

## 1) Prerequisites

- Azure resource group + VM
- Azure Storage Account + Azure Files share
- Service Bus namespace/queue
- ACR image for DoWhiz worker runtime (optional but typical)

## 2) VM Setup

Install Docker + CIFS tooling:

```bash
sudo apt-get update
sudo apt-get install -y docker.io cifs-utils
sudo systemctl enable --now docker
```

Mount Azure Files share (example):

```bash
sudo mkdir -p /mnt/dowhiz-workspace
sudo mount -t cifs //<storage_account>.file.core.windows.net/<file_share> /mnt/dowhiz-workspace \
  -o vers=3.0,username=<storage_account>,password=<storage_account_key>,dir_mode=0777,file_mode=0777

sudo mkdir -p /mnt/dowhiz-workspace/run_task/{state,users,workspaces}
```

## 3) Worker `.env` (Unprefixed Runtime Keys)

Create runtime env file (example path `/opt/dowhiz/.env`) and set at least:

- `MONGODB_URI`
- `SUPABASE_DB_URL`
- `AZURE_OPENAI_API_KEY_BACKUP`
- `INGESTION_QUEUE_BACKEND=servicebus`
- Service Bus auth:
  - `SERVICE_BUS_CONNECTION_STRING=...`, or
  - `SERVICE_BUS_NAMESPACE=...` + `SERVICE_BUS_POLICY_NAME=...` + `SERVICE_BUS_POLICY_KEY=...`
- `SERVICE_BUS_QUEUE_NAME=ingestion`
- `RAW_PAYLOAD_STORAGE_BACKEND=azure`
- `AZURE_STORAGE_CONTAINER_INGEST=ingestion-raw`
- Azure auth for raw payload storage (`AZURE_STORAGE_CONTAINER_SAS_URL` or account+sas)

If using dockerized local task execution path in worker:
- `RUN_TASK_USE_DOCKER=1`
- `RUN_TASK_DOCKER_REQUIRED=1`
- `RUN_TASK_DOCKER_IMAGE=<acr>/dowhiz-service:<tag>`

If using Azure ACI backend for task execution instead:
- configure `RUN_TASK_EXECUTION_BACKEND=azure_aci` + required `RUN_TASK_AZURE_ACI_*` keys.

## 4) Run Worker Container

```bash
docker login <acr_login_server> -u <acr_username> -p <acr_password>

docker run -d --name dowhiz-worker \
  --restart unless-stopped \
  -p 9001:9001 \
  --env-file /opt/dowhiz/.env \
  -e EMPLOYEE_ID=little_bear \
  -e RUST_SERVICE_PORT=9001 \
  -v /mnt/dowhiz-workspace:/app/.workspace \
  -v /var/run/docker.sock:/var/run/docker.sock \
  <acr_login_server>/dowhiz-service:<tag>
```

Notes:
- Worker container uses host Docker socket if docker run_task mode is enabled.
- Keep `RUN_TASK_DOCKER_IMAGE` pinned to an explicit version tag.

## 5) Load / Smoke Testing

Module load-test helper:

```bash
cd DoWhiz_service
cargo run -p run_task_module --bin load_test -- \
  --count 200 \
  --concurrency 200 \
  --reply-required \
  --employee little_bear
```

Service Bus fanout generator:

```bash
python3 DoWhiz_service/scripts/load_tests/servicebus_fanout.py --count 200 --employee-id little_bear
```

## 6) Important Notes

- Runtime env should use unprefixed keys in `.env`.
- Some code paths still support `SCALE_OLIVER_*` aliases, but new deployments should treat unprefixed keys as source of truth.
- Validate queue/storage credentials before enabling high-concurrency loads.
