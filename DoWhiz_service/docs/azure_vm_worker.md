# Azure VM Worker (Docker-Isolated RunTask)

This guide runs `rust_service` on an Azure VM with host Docker so each task can spawn its own
Codex container (single image). It keeps per-user workspaces on Azure Files for isolation +
durability.

For staging/prod deployment with separate secret sets and unprefixed runtime keys, plus
gateway + worker deploy/rollback steps, see:
`DoWhiz_service/docs/staging_production_deploy.md`.

## Prereqs
- Azure resources (RG, Storage account + Azure Files share, Service Bus, ACR).
- Image pushed to ACR (e.g., `acrdwhzoliverdev.azurecr.io/dowhiz-service:dev`).
- VM with Docker installed.

## VM Setup (host Docker + Azure Files)
1. Install Docker and CIFS tooling:
```bash
sudo apt-get update
sudo apt-get install -y docker.io cifs-utils
sudo systemctl enable --now docker
```

2. Mount the Azure Files share (example):
```bash
sudo mkdir -p /mnt/dowhiz-workspace
sudo mount -t cifs //<storage_account>.file.core.windows.net/<file_share> /mnt/dowhiz-workspace \
  -o vers=3.0,username=<storage_account>,password=<storage_account_key>,dir_mode=0777,file_mode=0777

sudo mkdir -p /mnt/dowhiz-workspace/run_task/{state,users,workspaces}
```

3. Create the env file for the worker (example path):
```bash
sudo mkdir -p /opt/dowhiz
sudo nano /opt/dowhiz/.env
```

At minimum, set:
- `AZURE_OPENAI_API_KEY_BACKUP`
- `AZURE_OPENAI_ENDPOINT_BACKUP`
- `SCALE_OLIVER_INGESTION_QUEUE_BACKEND=servicebus`
- `SCALE_OLIVER_SERVICE_BUS_CONNECTION_STRING=...`
- `SCALE_OLIVER_SERVICE_BUS_QUEUE_NAME=ingestion`
- `SCALE_OLIVER_RAW_PAYLOAD_STORAGE_BACKEND=azure`
- `SCALE_OLIVER_AZURE_STORAGE_ACCOUNT=...`
- `SCALE_OLIVER_AZURE_STORAGE_CONTAINER_INGEST=ingestion-raw`
- `SCALE_OLIVER_AZURE_STORAGE_SAS_TOKEN=...`
- `AZURE_STORAGE_CONNECTION_STRING=...`
- `AZURE_STORAGE_CONTAINER=memo`
- `SCHEDULER_MAX_CONCURRENCY=200`
- `SCHEDULER_USER_MAX_CONCURRENCY=5`

The worker now prefers `SCALE_OLIVER_*` keys and falls back to legacy key names.

## Run the Worker Container
```bash
docker login <acr_login_server> -u <acr_username> -p <acr_password>

docker run -d --name dowhiz-oliver \
  --restart unless-stopped \
  -p 9001:9001 \
  --env-file /opt/dowhiz/.env \
  -e EMPLOYEE_ID=little_bear \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_USE_DOCKER=1 \
  -e RUN_TASK_DOCKER_REQUIRED=1 \
  -e RUN_TASK_DOCKER_IMAGE=<acr_login_server>/dowhiz-service:dev \
  -v /mnt/dowhiz-workspace:/app/.workspace \
  -v /var/run/docker.sock:/var/run/docker.sock \
  <acr_login_server>/dowhiz-service:dev
```

Notes:
- The worker container uses the host Docker socket to launch per-task containers.
- `RUN_TASK_DOCKER_IMAGE` should match the same tag in ACR.
- The workspace mount keeps user memory/history durable and shared.

## Load Test (Codex Tasks)
Use the load test binary from `run_task_module`:
```bash
cargo run -p run_task_module --bin load_test -- \
  --count 200 \
  --concurrency 200 \
  --reply-required \
  --employee little_bear
```

If running on the VM, install Rust (rustup) first, or run the load test on a build VM
and point it at the same Azure resources.

To drive a full Service Bus -> worker flow, send load-test envelopes:
```bash
export SCALE_OLIVER_SERVICE_BUS_CONNECTION_STRING="..."
export SCALE_OLIVER_SERVICE_BUS_QUEUE_NAME="ingestion"
python3 DoWhiz_service/scripts/load_tests/servicebus_fanout.py --count 200 --employee-id little_bear
```
