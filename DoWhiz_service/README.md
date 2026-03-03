# DoWhiz Service

<p align="center">
  <img src="../website/public/assets/DoWhiz.jpeg" alt="Do icon" width="96" />
</p>

Rust service for inbound channels (Postmark email, Slack, Discord, Twilio SMS, Telegram, WhatsApp, Google Docs/Sheets/Slides, BlueBubbles/iMessage), task scheduling, AI agent execution (Codex/Claude), and outbound replies.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Install Dependencies](#install-dependencies)
- [Employee Configuration](#employee-configuration)
- [Running the Service](#running-the-service)
  - [One-Command Local Run](#one-command-local-run)
  - [Staging/Production Target Switching](#stagingproduction-target-switching)
  - [Manual Multi-Employee Setup](#manual-multi-employee-setup)
  - [Inbound Gateway (Recommended)](#inbound-gateway-recommended)
  - [Azure Deployment (Rust Gateway + Service Bus + Blob + Workers)](#azure-deployment-rust-gateway--service-bus--blob--workers)
  - [VM Deployment (Gateway + ngrok)](#vm-deployment-gateway--ngrok)
  - [Fanout Gateway (Legacy)](#fanout-gateway-legacy)
  - [Docker Production](#docker-production)
- [Per-Task Docker Execution](#per-task-docker-execution)
- [Testing](#testing)
  - [Unit Tests](#unit-tests)
  - [Live E2E Tests](#live-e2e-tests)
  - [Slack Local Testing](#slack-local-testing)
  - [Discord Local Testing](#discord-local-testing)
  - [Telegram Local Testing](#telegram-local-testing)
  - [SMS (Twilio) Local Testing](#sms-twilio-local-testing)
  - [iMessage Local Testing via BlueBubbles](#imessage-local-testing-via-bluebubbles)
- [Message Router (OpenAI)](#message-router-openai)
- [Environment Variables](#environment-variables)
- [Runtime State](#runtime-state)
- [Database Schema](#database-schema)
- [Past Email Hydration](#past-email-hydration)
- [Scheduled Follow-ups](#scheduled-follow-ups)

---

## Prerequisites

- Rust toolchain
- System libs: `libssl`, `pkg-config`, `ca-certificates`
- Node.js 20 + npm
- `codex` CLI on your PATH (only required for local execution; optional when `RUN_TASK_USE_DOCKER=1` and the image includes Codex)
- `claude` CLI on your PATH (only required for employees with `runner = "claude"`)
- `playwright-cli` + Chromium (required for browser automation skills)
- `ngrok` (for exposing local service to webhooks)
- `python3` (for ngrok URL discovery)
- OpenAI API key (optional; enables message router quick replies)

**Required in `.env`** (copy from repo-root `.env.example` to `DoWhiz_service/.env`):
- `POSTMARK_SERVER_TOKEN`
- `AZURE_OPENAI_API_KEY_BACKUP` (required for Codex and Claude runners; Codex base URL is fixed in code)

**Optional in `.env`**:
- GitHub auth: `GH_TOKEN`/`GITHUB_TOKEN`/`GITHUB_PERSONAL_ACCESS_TOKEN` + `GITHUB_USERNAME`. Per-employee prefixes are supported (`OLIVER_`, `MAGGIE_`, `DEVIN_`, `PROTO_`) and can be overridden with `EMPLOYEE_GITHUB_ENV_PREFIX` or `GITHUB_ENV_PREFIX`.
- `RUN_TASK_USE_DOCKER=1` + `RUN_TASK_DOCKER_IMAGE` (run each task inside a disposable Docker container; use `dowhiz-service` for the repo image)
- `RUN_TASK_DOCKER_AUTO_BUILD=1` to auto-build the image when missing (set `0` to disable)
- `INGESTION_QUEUE_BACKEND=servicebus` + `SERVICE_BUS_CONNECTION_STRING` + `SERVICE_BUS_QUEUE_NAME` (ingestion queue)
- `RAW_PAYLOAD_STORAGE_BACKEND=azure` + `AZURE_STORAGE_CONTAINER_INGEST` + `AZURE_STORAGE_SAS_TOKEN` (raw payload storage)
- `OPENAI_API_KEY` (enables message router quick replies)

---

## Install Dependencies

### Linux (Debian/Ubuntu)

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

Optional (match Dockerfile's chrome-channel lookup used by E2E):
```bash
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.cache/ms-playwright"
chromium_path="$(ls -d "$PLAYWRIGHT_BROWSERS_PATH"/chromium-*/chrome-linux/chrome | head -n1)"
sudo mkdir -p /opt/google/chrome
sudo ln -sf "$chromium_path" /opt/google/chrome/chrome
```

### macOS (Homebrew)

```bash
brew install node@20 openssl@3 pkg-config
npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
npx playwright install chromium
```

Skills are copied from `DoWhiz_service/skills` automatically when preparing workspaces.
Postmark outbound requires each employee address to be a verified Sender Signature (or Domain) because replies are sent from the inbound mailbox.

---

## Employee Configuration

`employee.toml` defines each employee (addresses, runner, models, AGENTS/CLAUDE/SOUL files, and skills directory). Set `EMPLOYEE_ID` to pick which employee profile this server instance runs.

The server only processes inbound mail addressed to its configured addresses; other emails are ignored, so multiple servers can receive the same webhook safely.

Replies are sent from the employee address that the inbound email targeted (no `OUTBOUND_FROM` override needed).

For forwarded mail, the service checks `To`/`Cc`/`Bcc` plus headers such as `X-Original-To`, `Delivered-To`, and `X-Forwarded-To` to determine which employee address was targeted.

**Available employees:**

| ID | Name | Runner | Addresses |
|----|------|--------|-----------|
| `little_bear` | Oliver | Codex | oliver@dowhiz.com, little-bear@dowhiz.com, agent@dowhiz.com |
| `mini_mouse` | Maggie | Claude | maggie@dowhiz.com, mini-mouse@dowhiz.com |
| `sticky_octopus` | Devin | Codex | devin@dowhiz.com, sticky-octopus@dowhiz.com, coder@dowhiz.com |
| `boiled_egg` | Boiled-Egg | Codex | proto@dowhiz.com, boiled-egg@dowhiz.com |

`employee.toml` also supports `runtime_root` per employee to override the default runtime location (for repo-local runs, use `.workspace/<employee_id>` relative to `DoWhiz_service/employee.toml`).

Skills are copied per workspace: the base repo skills are always included, and `skills_dir` can optionally add overrides or extra skills.

---

## Running the Service

### One-Command Local Run

From the repo root:
```bash
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
./DoWhiz_service/scripts/run_employee.sh mini_mouse 9002 --skip-hook --skip-ngrok
./DoWhiz_service/scripts/run_employee.sh sticky_octopus 9003 --skip-hook --skip-ngrok
./DoWhiz_service/scripts/run_employee.sh boiled_egg 9004 --skip-hook --skip-ngrok
```

This command:
- Starts a worker process (the Rust service bound to the selected host/port).
- (Legacy) Can start ngrok and update the Postmark inbound hook, but the worker no longer serves `/postmark/inbound`.

For the current inbound flow, run the inbound gateway separately and use `--skip-hook --skip-ngrok` so workers do not overwrite the gateway hook.

**Optional flags:**
- `--public-url https://example.com` uses an existing public URL and skips ngrok
- `--skip-hook` leaves the Postmark hook unchanged
- `--skip-ngrok` disables ngrok (requires `--public-url` or `--skip-hook`)

When running with the inbound gateway, start workers with `--skip-hook --skip-ngrok`.

**Full usage:**
```
scripts/run_employee.sh <employee_id> [port]
scripts/run_employee.sh --employee <id> --port <port> [--public-url <url>] [--skip-hook] [--skip-ngrok] [--host <host>]
```

### Staging/Production Target Switching

Use one `.env` and switch targets with:
```bash
export DEPLOY_TARGET=production   # or staging
```

All startup scripts now load `DoWhiz_service/.env` and, when `DEPLOY_TARGET=staging`,
automatically map `STAGING_FOO -> FOO` (for example, `STAGING_SERVICE_BUS_CONNECTION_STRING`
overrides `SERVICE_BUS_CONNECTION_STRING`).

Current staging profile defaults:
- sender + receiver mailbox: `dowhiz@deep-tutor.com` (via `employee.staging.toml`)
- raw payload storage prefix: `staging/ingestion_raw` (same container, separated folder path)

Branch policy for VM deployments:
- Production branch: `main` (CI/CD baseline)
- Staging branch: `dev` (CI/CD rollout target)

For full split-key matrix, gateway/worker commands, and rollback steps, see:
`DoWhiz_service/docs/staging_production_deploy.md`.

### Manual Multi-Employee Setup

**Step 0: Configure Azure ingestion (required for gateway)**
Add these to `DoWhiz_service/.env` (recommended) or export in each terminal before starting gateway/workers.
```bash
export INGESTION_QUEUE_BACKEND=servicebus
export SERVICE_BUS_CONNECTION_STRING="Endpoint=sb://..."
export SERVICE_BUS_QUEUE_NAME="ingestion"
export RAW_PAYLOAD_STORAGE_BACKEND=azure
export AZURE_STORAGE_CONTAINER_INGEST="ingestion-raw"
export AZURE_STORAGE_SAS_TOKEN="..."
```

**Step 1: Start workers (one per employee)**
Run each worker in its own terminal.
```bash
cd DoWhiz_service

# Oliver / Little-Bear (Codex)
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# Maggie / Mini-Mouse (Claude)
EMPLOYEE_ID=mini_mouse RUST_SERVICE_PORT=9002 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9002

# Sticky-Octopus (Codex)
EMPLOYEE_ID=sticky_octopus RUST_SERVICE_PORT=9003 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9003

# Boiled-Egg (Codex)
EMPLOYEE_ID=boiled_egg RUST_SERVICE_PORT=9004 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9004
```

**Step 2: Configure the gateway routes**
```bash
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
# Edit gateway.toml to map service addresses to employees. Example:
cat > DoWhiz_service/gateway.toml <<'EOF'
[defaults]
tenant_id = "default"
employee_id = "little_bear"

[[routes]]
channel = "email"
key = "oliver@dowhiz.com"
employee_id = "little_bear"
tenant_id = "default"
EOF
```

**Step 3: Start the inbound gateway (Terminal 2)**
```bash
# Ensure Service Bus + Azure Blob env vars are set in this terminal
./DoWhiz_service/scripts/run_gateway_local.sh
```

**Step 4: Expose the gateway with ngrok (Terminal 3)**
```bash
ngrok http 9100
```

**Step 5: Set the Postmark inbound hook to the gateway**
```bash
cd DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

**Step 6: Send an email**
```
oliver@dowhiz.com   # or mini-mouse@dowhiz.com, devin@dowhiz.com, proto@dowhiz.com
```

**Step 7: Watch logs for task execution**

Outputs appear under:
- Email / Google Workspace replies:
  - `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/reply_email_draft.html`
  - `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/reply_email_attachments/`
- Chat/SMS channel replies:
  - `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/reply_message.txt`
  - `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/reply_attachments/`
- Scheduler state: `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/state/tasks.db`

### Inbound Gateway (Recommended)

The inbound gateway (`inbound_gateway`) handles Postmark/Slack/Discord/BlueBubbles/Twilio SMS/Telegram/WhatsApp/Google Docs/Sheets/Slides inbound traffic, deduplicates it, and enqueues messages into Azure Service Bus. Raw payload storage backend is configurable (`supabase` by default, `azure` recommended for production). Workers poll the shared queue and filter by `employee_id`; workers no longer expose `/postmark/inbound`.

HTTP endpoints:
- `/postmark/inbound` (email)
- `/slack/events`
- `/bluebubbles/webhook`
- `/telegram/webhook`
- `/sms/twilio`
- `/whatsapp/webhook`
- `/webhooks/google-drive-changes` (Google Drive push notifications for Docs/Sheets file changes)

Discord is handled via bot gateway clients (requires `DISCORD_BOT_TOKEN` or per-employee Discord bot tokens). Discord routing is determined by the bot token to employee mapping (not by `gateway.toml` route keys). Google Docs/Sheets/Slides comments are handled by the workspace poller when the corresponding `GOOGLE_*_ENABLED` flags are set.

Optional webhook verification:
- `POSTMARK_INBOUND_TOKEN` (validates `X-Postmark-Token`)
- `SLACK_SIGNING_SECRET`
- `BLUEBUBBLES_WEBHOOK_TOKEN`
- `TWILIO_AUTH_TOKEN` + `TWILIO_WEBHOOK_URL`
- `WHATSAPP_VERIFY_TOKEN` (validates WhatsApp webhook verification handshake)
- `GATEWAY_MAX_BODY_BYTES` to override the default 25MB request limit

### Azure Deployment (Rust Gateway + Service Bus + Blob + Workers)

This is the recommended Azure production flow. The Rust inbound gateway handles **all ingress** (email + Slack/Discord/etc), stores raw payloads in Azure Blob, and enqueues messages into Azure Service Bus. Workers (`rust_service`) run on VMs or containers and poll Service Bus.

For staging/prod split deployment with a single `.env` (`DEPLOY_TARGET` + `STAGING_` overrides), see:
`DoWhiz_service/docs/staging_production_deploy.md`.

**Step 1: Provision Azure resources**
```bash
# Example (use your own names/location)
az group create -n <rg> -l westus2
az storage account create -g <rg> -n <storage> -l westus2 --sku Standard_LRS --kind StorageV2
az storage container create --account-name <storage> --name ingestion-raw
az servicebus namespace create -g <rg> -n <sb-namespace> -l westus2 --sku Standard
az servicebus queue create -g <rg> --namespace-name <sb-namespace> -n ingestion --enable-duplicate-detection true --duplicate-detection-history-time-window PT10M
az servicebus queue create -g <rg> --namespace-name <sb-namespace> -n ingestion-test --enable-duplicate-detection true --duplicate-detection-history-time-window PT10M
az apim create -g <rg> -n <apim> --location westus2 --publisher-email proto@dowhiz.com --publisher-name DoWhiz --sku-name Consumption
```

For production workers, set queue `LockDuration` to `PT5M` (5 minutes) to reduce lock-expiry risk during slow processing bursts.

**Step 2: Configure gateway + worker configs**
Update `DoWhiz_service/gateway.toml` and `DoWhiz_service/employee.toml` with your service addresses and routing targets. These same files are used by the gateway and by workers.

**Step 3: Deploy the Rust gateway**
```bash
export INGESTION_QUEUE_BACKEND=servicebus
export SERVICE_BUS_CONNECTION_STRING="Endpoint=sb://..."
export SERVICE_BUS_QUEUE_NAME="ingestion"
export RAW_PAYLOAD_STORAGE_BACKEND=azure
export AZURE_STORAGE_CONTAINER_INGEST="ingestion-raw"
export AZURE_STORAGE_SAS_TOKEN="..."

./DoWhiz_service/scripts/run_gateway_local.sh
```

**Step 4: Point inbound webhooks to the gateway**
- Postmark: `https://<public>/postmark/inbound`
- Slack: `https://<public>/slack/events`
- Telegram: `https://<public>/telegram/webhook`
- SMS (Twilio): `https://<public>/sms/twilio`
- WhatsApp: `https://<public>/whatsapp/webhook`
- BlueBubbles: `https://<public>/bluebubbles/webhook`

**Step 5: Deploy workers against Service Bus**
Set these on the worker host (VM or container) and start one worker per employee:
```bash
export INGESTION_QUEUE_BACKEND=servicebus
export SERVICE_BUS_CONNECTION_STRING="Endpoint=sb://..."
export SERVICE_BUS_QUEUE_NAME="ingestion"
export RAW_PAYLOAD_STORAGE_BACKEND=azure
export AZURE_STORAGE_CONTAINER_INGEST="ingestion-raw"
export AZURE_STORAGE_SAS_TOKEN="..."

./DoWhiz_service/scripts/run_employee.sh boiled_egg 9004 --skip-hook --skip-ngrok
```

Notes:
- All employees share the same Service Bus queue; workers filter by `employee_id` in the envelope.
- Keep `gateway.toml` and `employee.toml` consistent across the gateway and workers.

**Local gateway + Docker workers (Service Bus + Azure Blob)**

**Step 1: Build the Docker image (once)**
```bash
docker build -t dowhiz-service .
```

**Step 2: Configure Service Bus + Azure Blob**
```bash
export INGESTION_QUEUE_BACKEND=servicebus
export SERVICE_BUS_CONNECTION_STRING="Endpoint=sb://..."
export SERVICE_BUS_QUEUE_NAME="ingestion"
export RAW_PAYLOAD_STORAGE_BACKEND=azure
export AZURE_STORAGE_CONTAINER_INGEST="ingestion-raw"
export AZURE_STORAGE_SAS_TOKEN="..."
```

**Step 3: Start workers in Docker (mount shared ingestion dir)**
```bash
docker run --rm -p 9001:9001 \
  -e EMPLOYEE_ID=little_bear \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_USE_DOCKER=0 \
  -e INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
  -e SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
  -e SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
  -e RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
  -e AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
  -e AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace-oliver:/app/.workspace \
  dowhiz-service

docker run --rm -p 9002:9001 \
  -e EMPLOYEE_ID=mini_mouse \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_USE_DOCKER=0 \
  -e INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
  -e SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
  -e SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
  -e RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
  -e AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
  -e AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace-maggie:/app/.workspace \
  dowhiz-service
```

Note: when running workers inside Docker, keep `RUN_TASK_USE_DOCKER=0` to avoid nested Docker usage.

**Step 4: Configure the gateway routes**
```bash
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
# Edit gateway.toml routes to map service addresses to employees.
```

**Step 5: Start the gateway (host)**
```bash
INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  ./DoWhiz_service/scripts/run_gateway_local.sh
```

**Step 6: Expose the gateway with ngrok**
```bash
ngrok http 9100
```

**Step 7: Point Postmark to the gateway**
```bash
cd DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

**Step 8: Send test emails**
Use Postmark inbound with the test sender identities:
- From: `mini-mouse@deep-tutor.com` → To: `mini-mouse@dowhiz.com`

If you want a scripted smoke test, reuse the live Postmark script below and set:
```
POSTMARK_INBOUND_HOOK_URL=http://127.0.0.1:9100/postmark/inbound
POSTMARK_TEST_FROM=mini-mouse@deep-tutor.com
POSTMARK_TEST_SERVICE_ADDRESS=mini-mouse@dowhiz.com
```

### VM Deployment (Gateway + ngrok)

Single-VM deployment that runs a worker and the Rust inbound gateway, exposed via ngrok. This flow still uses Azure Service Bus + Blob for ingestion/payload storage; ngrok only provides public ingress.

1. Provision an Ubuntu VM and open inbound TCP ports `22`, `80`, `443`.
Outbound SMTP (`25`) is often blocked on cloud VMs; run E2E senders from your local machine if needed.

2. Ensure the VM can reach Azure Service Bus and Azure Blob Storage over HTTPS (port 443).

3. Install dependencies + ngrok (VM):
```bash
sudo apt-get update
sudo apt-get install -y ca-certificates libssl-dev pkg-config curl git python3
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
curl https://sh.rustup.rs -sSf | sh -s -- -y
sudo snap install ngrok
```

4. Clone the repo and configure `.env` (VM):
```bash
git clone https://github.com/KnoWhiz/DoWhiz.git
cd DoWhiz
cp .env.example DoWhiz_service/.env
# Edit DoWhiz_service/.env with production secrets
# Add Service Bus + Azure Blob settings (used by gateway + worker):
INGESTION_QUEUE_BACKEND=servicebus
SERVICE_BUS_CONNECTION_STRING=Endpoint=sb://...
SERVICE_BUS_QUEUE_NAME=ingestion
RAW_PAYLOAD_STORAGE_BACKEND=azure
AZURE_STORAGE_CONTAINER_INGEST=ingestion-raw
AZURE_STORAGE_SAS_TOKEN=...
# For GitHub PR creation from email (recommended):
EMPLOYEE_ID=little_bear
# Optional (dev only): if Supabase TLS fails with self-signed certs
# INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS=true
```

Optional: copy your local `.env` directly to the VM:
```bash
scp /path/to/DoWhiz_service/.env azureuser@<vm>:/home/azureuser/DoWhiz/DoWhiz_service/.env
```

5. Configure the gateway to route only Oliver:
```bash
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
cat > DoWhiz_service/gateway.toml <<'EOF'
[defaults]
tenant_id = "default"
employee_id = "little_bear"

[[routes]]
channel = "email"
key = "oliver@dowhiz.com"
employee_id = "little_bear"
tenant_id = "default"
EOF
```

6. Build release binaries (recommended on VM):
```bash
source ~/.cargo/env
cd ~/DoWhiz/DoWhiz_service
cargo build -p scheduler_module --bin rust_service --bin inbound_gateway --release
```

7. Start services (pm2 recommended):
```bash
pm2 start ~/DoWhiz/DoWhiz_service/target/release/inbound_gateway \
  --name dowhiz_gateway \
  --cwd ~/DoWhiz/DoWhiz_service

pm2 start ~/DoWhiz/DoWhiz_service/target/release/rust_service \
  --name dowhiz_rust_service \
  --cwd ~/DoWhiz/DoWhiz_service -- --host 0.0.0.0 --port 9001

pm2 save
```

tmux alternative:
```bash
tmux new-session -d -s oliver "bash -lc 'cd ~/DoWhiz/DoWhiz_service && set -a && source .env && set +a && ~/DoWhiz/DoWhiz_service/target/release/rust_service --host 0.0.0.0 --port 9001'"
tmux new-session -d -s gateway "bash -lc 'cd ~/DoWhiz/DoWhiz_service && set -a && source .env && set +a && ~/DoWhiz/DoWhiz_service/target/release/inbound_gateway'"
```
Note: if you run services under pm2/systemd (non-interactive shells), ensure PATH includes `~/.cargo/bin` when building, or use absolute binary paths as above.

8. Expose the gateway with ngrok (VM):
```bash
ngrok config add-authtoken "$NGROK_AUTHTOKEN"
ngrok http 9100 --domain=shayne-laminar-lillian.ngrok-free.dev
```

9. Health checks (VM):
```bash
curl -sS http://127.0.0.1:9001/health && echo
curl -sS http://127.0.0.1:9100/health && echo
curl -sS https://shayne-laminar-lillian.ngrok-free.dev/health && echo
```

10. Point Postmark to the gateway (VM):
```bash
cd ~/DoWhiz/DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://shayne-laminar-lillian.ngrok-free.dev/postmark/inbound
```

11. Run live E2E (from your local machine if VM blocks SMTP 25):
```
POSTMARK_INBOUND_HOOK_URL=https://shayne-laminar-lillian.ngrok-free.dev/postmark/inbound
POSTMARK_TEST_FROM=mini-mouse@deep-tutor.com
POSTMARK_TEST_SERVICE_ADDRESS=oliver@dowhiz.com
```
Use the Live E2E driver script in [Live E2E Tests](#live-e2e-tests).

#### Nginx + systemd (optional)

If you prefer terminating HTTPS on the VM directly (no ngrok), run both the inbound gateway and a worker behind Nginx and point Postmark to the gateway URL.

Environment file path (update to match current layout). Ensure it includes the shared ingestion paths:
```
EnvironmentFile=/home/azureuser/DoWhiz/DoWhiz_service/.env
```

Example systemd services:
Build binaries first: `cargo build -p scheduler_module --bin rust_service --bin inbound_gateway --release`.

```ini
[Unit]
Description=DoWhiz Inbound Gateway
After=network.target

[Service]
Type=simple
User=azureuser
Group=azureuser
WorkingDirectory=/home/azureuser/DoWhiz/DoWhiz_service
EnvironmentFile=/home/azureuser/DoWhiz/DoWhiz_service/.env
Environment=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ExecStart=/home/azureuser/DoWhiz/DoWhiz_service/target/release/inbound_gateway
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

```ini
[Unit]
Description=DoWhiz Worker (Oliver)
After=network.target

[Service]
Type=simple
User=azureuser
Group=azureuser
WorkingDirectory=/home/azureuser/DoWhiz/DoWhiz_service
EnvironmentFile=/home/azureuser/DoWhiz/DoWhiz_service/.env
Environment=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ExecStart=/home/azureuser/DoWhiz/DoWhiz_service/target/release/rust_service --host 127.0.0.1 --port 9001
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

### Fanout Gateway (Legacy)

The fanout gateway predates the ingestion-queue architecture and expects workers to accept inbound HTTP routes. Current workers do not expose `/postmark/inbound`, so the fanout flow is not compatible with the default setup.

Use the inbound gateway instead. Legacy scripts remain for reference:
- `./DoWhiz_service/scripts/run_fanout_local.sh`
- `./DoWhiz_service/scripts/run_all_employees_docker.sh`
- `./DoWhiz_service/scripts/run_proto_docker.sh`

### Docker Production

Build the image from the repo root and run it with the same `.env` file mounted:

```bash
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -e INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
  -e SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
  -e SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
  -e RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
  -e AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
  -e AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace:/app/.workspace \
  dowhiz-service
```

This runs a worker only. For inbound webhooks, run the inbound gateway separately and point it at the same ingestion queue:
```bash
docker run --rm -p 9100:9100 \
  --entrypoint /app/inbound_gateway \
  -e GATEWAY_PORT=9100 \
  -e INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
  -e SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
  -e SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
  -e RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
  -e AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
  -e AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v "$PWD/DoWhiz_service/gateway.toml:/app/DoWhiz_service/gateway.toml:ro" \
  dowhiz-service
```

If `RUN_TASK_USE_DOCKER=1` and `RUN_TASK_DOCKER_IMAGE` is set in your `.env`, each task runs inside a fresh Docker container and the image auto-builds on first use (unless disabled with `RUN_TASK_DOCKER_AUTO_BUILD=0`).

**Docker E2E (Codex + playwright-cli):**
```bash
export AZURE_OPENAI_API_KEY_BACKUP=...
mkdir -p .workspace_docker_test
docker run --rm --entrypoint bash --user 10001:10001 \
  -e AZURE_OPENAI_API_KEY_BACKUP \
  -e HOME=/workspace \
  -e TMPDIR=/workspace/tmp \
  -v "$HOME/.codex:/codex-config:ro" \
  -v "$PWD/.workspace_docker_test:/workspace" \
  dowhiz-service -lc "set -euo pipefail; \
    WORKDIR=/workspace/skill_e2e_test_docker; \
    mkdir -p /workspace/.codex /workspace/tmp \"$WORKDIR/.agents/skills\" \"$WORKDIR/.playwright\"; \
    cp -R /codex-config/* /workspace/.codex/; \
    cat > \"$WORKDIR/.playwright/cli.config.json\" <<'EOF'
{ \"browser\": { \"browserName\": \"chromium\", \"userDataDir\": \"/workspace/tmp/playwright-user-data\", \"launchOptions\": { \"channel\": \"chrome\", \"chromiumSandbox\": false } } }
EOF
    codex exec --skip-git-repo-check \
      -m gpt-5.3-codex \
      -c model_provider=\"azure\" \
      -c web_search=\"live\" \
      -c ask_for_approval=\"never\" \
      -c sandbox=\"workspace-write\" \
      -c model_providers.azure.base_url=\"https://knowhiz-service-openai-backup-2.openai.azure.com/openai/v1\" \
      -c model_providers.azure.env_key=\"AZURE_OPENAI_API_KEY_BACKUP\" \
      -c model_providers.azure.wire_api=\"responses\" \
      --cd \"$WORKDIR\" \
    \"Test the \\\"add todo\\\" flow on https://demo.playwright.dev/todomvc using playwright-cli. Check playwright-cli --help for available commands.\""
```

---

## Per-Task Docker Execution

When `RUN_TASK_USE_DOCKER=1`, each RunTask spins up a fresh container, mounts the task workspace at `/workspace`, runs Codex inside the container, and removes the container when done.

If the image is missing, the service will auto-build it (unless `RUN_TASK_DOCKER_AUTO_BUILD=0`).

Override build inputs:
- `RUN_TASK_DOCKERFILE` - Override the Dockerfile path
- `RUN_TASK_DOCKER_BUILD_CONTEXT` - Override the docker build context directory

---

## Testing

### Unit Tests

```bash
# All tests
cargo test

# Module-specific
cargo test -p scheduler_module
cargo test -p send_emails_module
cargo test -p run_task_module

# Single test
cargo test -p scheduler_module --test scheduler_basic

# Linting
cargo clippy --all-targets --all-features
cargo fmt --check
```

### Live E2E Tests

**Prerequisites:**
- ngrok installed and authenticated
- Postmark inbound address configured on the server
- Sender signatures for all employee addresses and the `POSTMARK_TEST_FROM` address
- `POSTMARK_SERVER_TOKEN`, `POSTMARK_TEST_FROM`, and `AZURE_OPENAI_API_KEY_BACKUP` set
- `RUN_CODEX_E2E=1` if you want Codex to execute real tasks (otherwise it is disabled in the live test)

**Docker flow (worker in Docker, gateway on host):**

1. Configure Service Bus + Azure Blob:
```bash
export INGESTION_QUEUE_BACKEND=servicebus
export SERVICE_BUS_CONNECTION_STRING="Endpoint=sb://..."
export SERVICE_BUS_QUEUE_NAME="ingestion"
export RAW_PAYLOAD_STORAGE_BACKEND=azure
export AZURE_STORAGE_CONTAINER_INGEST="ingestion-raw"
export AZURE_STORAGE_SAS_TOKEN="..."
```

2. Start the worker container:
```bash
docker run --rm -p 9002:9002 \
  -e EMPLOYEE_ID=mini_mouse \
  -e RUST_SERVICE_PORT=9002 \
  -e RUN_TASK_SKIP_WORKSPACE_REMAP=1 \
  -e INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
  -e SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
  -e SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
  -e RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
  -e AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
  -e AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace:/app/.workspace \
  dowhiz-service
```

For `little_bear` (Codex), add `-e CODEX_BYPASS_SANDBOX=1` if Codex fails with Landlock sandbox errors inside Docker.

3. Ensure `DoWhiz_service/gateway.toml` routes the test address to your worker, then start the inbound gateway on the host:
```bash
INGESTION_QUEUE_BACKEND="$INGESTION_QUEUE_BACKEND" \
SERVICE_BUS_CONNECTION_STRING="$SERVICE_BUS_CONNECTION_STRING" \
SERVICE_BUS_QUEUE_NAME="$SERVICE_BUS_QUEUE_NAME" \
RAW_PAYLOAD_STORAGE_BACKEND="$RAW_PAYLOAD_STORAGE_BACKEND" \
AZURE_STORAGE_CONTAINER_INGEST="$AZURE_STORAGE_CONTAINER_INGEST" \
AZURE_STORAGE_SAS_TOKEN="$AZURE_STORAGE_SAS_TOKEN" \
  ./DoWhiz_service/scripts/run_gateway_local.sh
```

4. Start ngrok (gateway port):
```bash
ngrok http 9100
```

5. Run the live driver:
```bash
POSTMARK_INBOUND_HOOK_URL="https://<ngrok>.ngrok.app/postmark/inbound" \
POSTMARK_TEST_SERVICE_ADDRESS="mini-mouse@dowhiz.com" \
POSTMARK_TEST_FROM="mini-mouse@deep-tutor.com" \
python - <<'PY'
import os, time, json, urllib.request, urllib.parse, smtplib
from email.message import EmailMessage

TOKEN = os.environ.get("POSTMARK_SERVER_TOKEN")
HOOK = os.environ.get("POSTMARK_INBOUND_HOOK_URL")
FROM_ADDR = os.environ.get("POSTMARK_TEST_FROM") or "oliver@dowhiz.com"
SERVICE_ADDR = os.environ.get("POSTMARK_TEST_SERVICE_ADDRESS") or "oliver@dowhiz.com"

if not TOKEN or not HOOK:
    raise SystemExit("Missing POSTMARK_SERVER_TOKEN or POSTMARK_INBOUND_HOOK_URL")

base_url = HOOK.rstrip("/")
if base_url.endswith("/postmark/inbound"):
    base_url = base_url[: -len("/postmark/inbound")]
health_url = base_url + "/health"

def request(method, url, payload=None, timeout=30):
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(url, data=data, method=method)
    req.add_header("Accept", "application/json")
    req.add_header("Content-Type", "application/json")
    req.add_header("X-Postmark-Server-Token", TOKEN)
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        body = resp.read().decode("utf-8")
        if resp.status < 200 or resp.status >= 300:
            raise RuntimeError(f"Postmark request failed: {resp.status} {body}")
        return json.loads(body) if body else {}

with urllib.request.urlopen(health_url, timeout=10) as resp:
    if resp.status < 200 or resp.status >= 300:
        raise SystemExit(f"Health check failed: {resp.status}")

server_info = request("GET", "https://api.postmarkapp.com/server")
prev_hook = server_info.get("InboundHookUrl", "") or ""
inbound_address = server_info.get("InboundAddress", "") or ""
if not inbound_address:
    raise SystemExit("Postmark server missing inbound address")

hook_url = base_url + "/postmark/inbound"
request("PUT", "https://api.postmarkapp.com/server", {"InboundHookUrl": hook_url})

subject = f"Live E2E test {int(time.time())}"
msg = EmailMessage()
msg["From"] = FROM_ADDR
msg["To"] = inbound_address
msg["Subject"] = subject
msg["X-Original-To"] = SERVICE_ADDR
msg.set_content("Rust service live email test.")

with smtplib.SMTP("inbound.postmarkapp.com", 25, timeout=30) as smtp:
    smtp.send_message(msg)

subject_hint = f"Re: {subject}"
start = time.time()
found = None
while time.time() - start < 300:
    params = urllib.parse.urlencode({"recipient": FROM_ADDR, "count": 50, "offset": 0})
    data = request("GET", "https://api.postmarkapp.com/messages/outbound?" + params)
    for message in data.get("Messages", []) or []:
        subj = message.get("Subject") or ""
        if subject_hint in subj:
            found = message
            break
    if found:
        break
    time.sleep(2)

request("PUT", "https://api.postmarkapp.com/server", {"InboundHookUrl": prev_hook})

if not found:
    raise SystemExit("Timed out waiting for outbound reply")
status = (found.get("Status") or "")
if status not in ("Sent", "Delivered"):
    raise SystemExit(f"Unexpected outbound status: {status}")

print("live e2e ok")
PY
```

**Local flow (service spawned by the test, no Docker required):**

1. Start ngrok:
```bash
ngrok http 9100
```

2. Run the live test (do not start `rust_service` separately; the test binds to the port itself):
```bash
RUST_SERVICE_PORT=9002 \
POSTMARK_INBOUND_HOOK_URL="https://<ngrok>.ngrok.app/postmark/inbound" \
POSTMARK_TEST_SERVICE_ADDRESS="mini-mouse@dowhiz.com" \
POSTMARK_TEST_FROM="mini-mouse@deep-tutor.com" \
RUST_SERVICE_LIVE_TEST=1 RUN_CODEX_E2E=1 \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

**Rust E2E test (generic):**
```bash
RUST_SERVICE_LIVE_TEST=1 \
POSTMARK_INBOUND_HOOK_URL=https://YOUR-DOMAIN.ngrok.app \
POSTMARK_TEST_FROM=you@example.com \
cargo test -p scheduler_module --test service_real_email -- --nocapture
```

### Slack Local Testing

Slack events are handled by the inbound gateway (`/slack/events`); OAuth callbacks are handled by the worker (`/slack/oauth/callback`). You need two public URLs (or a reverse proxy that splits paths).

1. Start a worker (typically `boiled_egg`) and the inbound gateway with a shared ingestion queue.
2. Start two ngrok tunnels:
```bash
ngrok http 9100  # gateway events
ngrok http 9004  # worker OAuth/install
```
3. Configure your Slack app:
Event Subscriptions Request URL: `https://<gateway-ngrok>.ngrok.app/slack/events`
OAuth Redirect URL: `https://<worker-ngrok>.ngrok.app/slack/oauth/callback`
Set `SLACK_REDIRECT_URI` in `.env` to the OAuth Redirect URL.
4. Visit `https://<worker-ngrok>.ngrok.app/slack/install` to authorize.
5. Invite the bot to a channel (`/invite @DoWhiz`).

### Discord Local Testing

1. Set either:
   - `DISCORD_BOT_TOKEN` + `DISCORD_BOT_USER_ID` for a single bot, or
   - `BOILED_EGG_DISCORD_BOT_TOKEN`/`BOILED_EGG_DISCORD_BOT_USER_ID` and/or `LITTLE_BEAR_DISCORD_BOT_TOKEN`/`LITTLE_BEAR_DISCORD_BOT_USER_ID` for per-employee bots.
2. Start a worker and the inbound gateway with a shared ingestion queue.
   - Discord routing is based on bot token -> employee mapping; `gateway.toml` Discord route keys are not used.
3. Add the bot to your server:
`https://discord.com/oauth2/authorize?client_id=1472013251553525983&permissions=0&integration_type=0&scope=bot`

### Telegram Local Testing

1. Set `TELEGRAM_BOT_TOKEN` (or per-employee `DO_WHIZ_<EMPLOYEE>_BOT`) in `.env`.
2. Start a worker and the inbound gateway with a shared ingestion queue.
3. Expose the gateway and set the Telegram webhook to `https://<gateway-ngrok>.ngrok.app/telegram/webhook`.
4. Ensure `gateway.toml` has a `telegram` route (use `key = "*"`, or a specific chat id).
5. Message the bot.

### SMS (Twilio) Local Testing

1. Set `TWILIO_ACCOUNT_SID` and `TWILIO_AUTH_TOKEN` in `.env`.
2. Start a worker and the inbound gateway with a shared ingestion queue.
3. Expose the gateway and set the Twilio webhook to `https://<gateway-ngrok>.ngrok.app/sms/twilio`.
4. If you want signature verification, set `TWILIO_WEBHOOK_URL` to the public webhook URL.
5. Ensure `gateway.toml` has an `sms` route (use `key = "*"`, or the Twilio phone number).
6. Send an SMS to the Twilio number.

### iMessage Local Testing via BlueBubbles
1. Download BlueBubbles (e.g. `brew install --cask bluebubbles`).
2. Start a worker and the inbound gateway with a shared ingestion queue.
3. In BlueBubbles → API & WebHooks, create a webhook at `http://127.0.0.1:9100/bluebubbles/webhook` (gateway). If BlueBubbles runs remotely, expose the gateway with ngrok and use that URL.

### Google Workspace Integration (Docs/Sheets/Slides)

Digital employees can collaborate on Google Docs (suggesting mode) and respond to comment threads in Google Docs, Sheets, and Slides.

### Features
- **Docs @Mention Detection**: Employees detect when mentioned in document comments
- **Docs Suggesting Mode**: Edits appear as color-coded revisions:
  - 🔴 **Red strikethrough** = Deletions
  - 🔵 **Blue text** = Insertions
- **Docs Apply/Discard**: Users can accept or reject all suggestions in batch
- **Docs Comment Replies**: Employees can respond to document comments
- **Sheets/Slides Comment Replies**: Employees can respond to comment threads in Sheets and Slides

#### Quick Setup

##### Step 1: Create Google Cloud Project

1. Go to [Google Cloud Console](https://console.cloud.google.com)
2. Create a new project or select existing
3. Enable APIs:
   - Google Docs API
   - Google Sheets API
   - Google Slides API
   - Google Drive API
4. Create OAuth 2.0 credentials:
   - Go to **APIs & Services → Credentials**
   - Click **Create Credentials → OAuth client ID**
   - Select **Desktop app** type
   - Download the credentials JSON

##### Step 2: Configure Redirect URI

In Google Cloud Console → Credentials → Your OAuth Client:
- Add `http://localhost:8085` to **Authorized redirect URIs**

##### Step 3: Get Refresh Token

```bash
# Set your credentials
export GOOGLE_CLIENT_ID="your-client-id.apps.googleusercontent.com"
export GOOGLE_CLIENT_SECRET="your-client-secret"

# Run the token generator script
cd DoWhiz_service
./scripts/get_google_refresh_token.sh
```

The script will:
1. Open browser for Google OAuth consent
2. Catch the callback on localhost:8085
3. Exchange code for refresh token
4. Print the token to add to your `.env`

##### Step 4: Add to .env

```bash
# Google OAuth credentials
GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-client-secret

# Per-employee refresh tokens (replace EMPLOYEE_ID with uppercase ID)
GOOGLE_REFRESH_TOKEN_BOILED_EGG=your-refresh-token-here
GOOGLE_REFRESH_TOKEN_LITTLE_BEAR=your-refresh-token-here

# Enable Google Docs polling
GOOGLE_DOCS_ENABLED=true
GOOGLE_DOCS_POLL_INTERVAL_SECS=30

# Enable Google Sheets/Slides polling
GOOGLE_SHEETS_ENABLED=true
GOOGLE_SLIDES_ENABLED=true
GOOGLE_WORKSPACE_POLL_INTERVAL_SECS=15

# Optional: additional employee emails to ignore (comma-separated)
GOOGLE_EMPLOYEE_EMAILS=oliver@dowhiz.com,proto@dowhiz.com
```

#### Testing

##### Manual CLI Test
```bash
cd DoWhiz_service

# Build the CLI tools
cargo build --release --bin google_docs_cli --bin google_sheets_cli --bin google_slides_cli

# Docs
./target/release/google_docs_cli list-documents
./target/release/google_docs_cli read-document <doc_id>
./target/release/google_docs_cli suggest-replace <doc_id> --find="old text" --replace="new text"
./target/release/google_docs_cli apply-suggestions <doc_id>
./target/release/google_docs_cli discard-suggestions <doc_id>

# Sheets
./target/release/google_sheets_cli list-spreadsheets
./target/release/google_sheets_cli list-comments <sheet_id>

# Slides
./target/release/google_slides_cli list-presentations
./target/release/google_slides_cli list-comments <slides_id>
```

##### E2E Tests
```bash
cargo test --package scheduler_module --test google_docs_cli_e2e

# Manual scripts (requires live credentials + shared files)
./scheduler_module/tests/google_workspace_cli_test.sh
./scheduler_module/tests/google_workspace_e2e_test.sh
```

#### Multi-Employee Setup

Each employee needs their own Google account and refresh token:

| Employee | Env Variable | Google Account |
|----------|-------------|----------------|
| Boiled-Egg (boiled_egg) | `GOOGLE_REFRESH_TOKEN_BOILED_EGG` | proto@dowhiz.com |
| Oliver (little_bear) | `GOOGLE_REFRESH_TOKEN_LITTLE_BEAR` | oliver@dowhiz.com |

Run `get_google_refresh_token.sh` once per employee, logging into the appropriate Google account each time.

#### Troubleshooting

| Issue | Solution |
|-------|----------|
| `DNS lookup failed` | Ensure sandbox bypass is enabled for GoogleDocs tasks |
| `Token refresh failed` | Re-run `get_google_refresh_token.sh` to get a new token |
| `\n` appearing literally | Upgrade to latest CLI with escape sequence support |
| `Permission denied` | Share the document/sheet/slides deck with the employee's Google account |

---
## Message Router (OpenAI)

The service includes a lightweight message router that can answer simple queries directly and forward complex ones to the full Codex/Claude pipeline. The router is enabled only when `OPENAI_API_KEY` is set.

### Configuration

Environment variables:
- `AZURE_OPENAI_API_KEY_BACKUP`: Azure OpenAI key (preferred when paired with endpoint)
- `AZURE_OPENAI_ENDPOINT_BACKUP`: Azure OpenAI endpoint (base URL; `/openai/v1` auto-appended if missing)
- `OPENAI_API_KEY`: Required to enable routing when Azure vars are not set
- `OPENAI_API_URL`: Override OpenAI base URL (default: `https://api.openai.com/v1`)
- `ROUTER_MODEL`: Model name (default: `gpt-5.2`)
- `ROUTER_ENABLED`: Set to `"false"` to disable routing (default: enabled)

### How it works

1. Short/simple messages are classified by the OpenAI model
2. Simple queries get a quick local response
3. Complex queries are forwarded to the full pipeline (Codex/Claude)

This reduces API costs and latency for simple interactions while preserving full capability for complex tasks.

---

## Environment Variables

Single-file env split:
- Use one `DoWhiz_service/.env`.
- Base keys are production values.
- Put staging-specific keys under `STAGING_*` and run with `DEPLOY_TARGET=staging`.

### Service Configuration
| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_SERVICE_HOST` | `0.0.0.0` | Host to bind |
| `RUST_SERVICE_PORT` | `9001` | Port to bind |
| `EMPLOYEE_ID` | - | Selects employee profile from `employee.toml` |
| `EMPLOYEE_CONFIG_PATH` | `DoWhiz_service/employee.toml` | Path to employee config |
| `FRONTEND_URL` | `http://localhost:5173` | Frontend base URL for OAuth redirects |
| `POSTMARK_INBOUND_MAX_BYTES` | `26214400` | Max inbound body size for worker endpoints |

### Paths
| Variable | Default | Description |
|----------|---------|-------------|
| `WORKSPACE_ROOT` | `<runtime_root>/workspaces` | Task workspace directory |
| `SCHEDULER_STATE_PATH` | `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/state/tasks.db` | Scheduler state |
| `PROCESSED_IDS_PATH` | `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/state/postmark_processed_ids.txt` | Deduplication list |
| `USERS_ROOT` | `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/users` | User data root |
| `USERS_DB_PATH` | `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/state/users.db` | User registry |
| `TASK_INDEX_PATH` | `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/state/task_index.db` | Task index |

### Scheduler
| Variable | Default | Description |
|----------|---------|-------------|
| `SCHEDULER_POLL_INTERVAL_SECS` | `1` | Poll interval for due tasks |
| `SCHEDULER_MAX_CONCURRENCY` | `200` | Global max concurrent tasks |
| `SCHEDULER_USER_MAX_CONCURRENCY` | `1` | Per-user max concurrent tasks |

### Ingestion Queue
| Variable | Default | Description |
|----------|---------|-------------|
| `SUPABASE_DB_URL` | - | Legacy Postgres ingestion backend (not used by inbound gateway) |
| `INGESTION_DB_URL` | - | Legacy alias for `SUPABASE_DB_URL` |
| `DATABASE_URL` | - | Legacy Postgres connection string for ingestion |
| `SUPABASE_POOLER_URL` | - | Legacy PgBouncer/Pooler URL for Postgres ingestion |
| `SUPABASE_PROJECT_URL` | - | Supabase project URL (used when `RAW_PAYLOAD_STORAGE_BACKEND=supabase`) |
| `SUPABASE_SECRET_KEY` | - | Supabase service key (used when `RAW_PAYLOAD_STORAGE_BACKEND=supabase`) |
| `SUPABASE_STORAGE_BUCKET` | `ingestion-raw` | Supabase storage bucket for raw payloads |
| `INGESTION_QUEUE_TABLE` | `ingestion_queue` | Postgres table name for the queue |
| `INGESTION_QUEUE_POOL_SIZE` | `8` | Max size for the ingestion queue Postgres pool |
| `INGESTION_QUEUE_LEASE_SECS` | `60` | Lease timeout before reclaiming stuck jobs |
| `INGESTION_QUEUE_MAX_ATTEMPTS` | `5` | Max retry attempts before marking failed |
| `INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS` | `0` | Set to `1` to allow self-signed Postgres certificates (local/dev only) |
| `INGESTION_POLL_INTERVAL_SECS` | `1` | Poll interval for ingestion consumer |

### Codex (OpenAI)
| Variable | Default | Description |
|----------|---------|-------------|
| `CODEX_MODEL` | `gpt-5.3-codex` | Default model when a RunTask payload does not provide `model_name` |
| `CODEX_DISABLED` | `0` | Set to `1` to bypass Codex CLI |
| `CODEX_SANDBOX` | `workspace-write` (fixed) | Historical var; run_task currently uses fixed sandbox in code |
| `CODEX_BYPASS_SANDBOX` | `0` | Set to `1` to pass `--yolo` (bypass approvals/sandbox) |
| `AZURE_OPENAI_API_KEY_BACKUP` | - | Azure OpenAI API key |
| `AZURE_OPENAI_ENDPOINT_BACKUP` | `https://knowhiz-service-openai-backup-2.openai.azure.com/openai/v1` (fixed) | Endpoint used by Codex runner is fixed in code |

### Claude (Anthropic)
| Variable | Default | Description |
|----------|---------|-------------|
| `CLAUDE_MODEL` | `claude-opus-4-5` | Model name |
| `ANTHROPIC_FOUNDRY_RESOURCE` | `knowhiz-service-openai-backup-2` | Foundry resource |

### Docker Execution
| Variable | Default | Description |
|----------|---------|-------------|
| `RUN_TASK_DOCKER_IMAGE` | - | Docker image to use when `RUN_TASK_USE_DOCKER=1` |
| `RUN_TASK_USE_DOCKER` | `0` | Enable per-task Docker execution (requires `RUN_TASK_DOCKER_IMAGE`) |
| `RUN_TASK_DOCKER_REQUIRED` | `0` | Fail when Docker CLI is missing instead of falling back to host execution |
| `RUN_TASK_DOCKER_AUTO_BUILD` | `1` | Auto-build missing images |
| `RUN_TASK_DOCKERFILE` | - | Override Dockerfile path |
| `RUN_TASK_DOCKER_BUILD_CONTEXT` | - | Override build context directory |
| `RUN_TASK_DOCKER_NETWORK` | - | Docker network mode (e.g., `host`) |
| `RUN_TASK_DOCKER_DNS` | - | Override Docker DNS servers (comma/space-separated) |
| `RUN_TASK_DOCKER_DNS_SEARCH` | - | Add DNS search domains (comma/space-separated) |
| `RUN_TASK_SKIP_WORKSPACE_REMAP` | `0` | Disable legacy workspace path migration |

### Azure ACI Execution (Serverless Per Task)
| Variable | Default | Description |
|----------|---------|-------------|
| `RUN_TASK_EXECUTION_BACKEND` | `auto` | `azure_aci`/`local`/`auto`. In `DEPLOY_TARGET=staging|production`, local Codex execution is blocked. |
| `RUN_TASK_AZURE_ACI_RESOURCE_GROUP` | - | Resource group used by `az container create` |
| `RUN_TASK_AZURE_ACI_IMAGE` | `RUN_TASK_DOCKER_IMAGE` fallback | Container image for task execution |
| `RUN_TASK_AZURE_ACI_LOCATION` | - | Optional ACI region override |
| `RUN_TASK_AZURE_ACI_CPU` | `2.0` | vCPU per task container |
| `RUN_TASK_AZURE_ACI_MEMORY_GB` | `4.0` | RAM (GB) per task container |
| `RUN_TASK_AZURE_ACI_FILE_SHARE` | `dowhiz-run-task` | Azure Files share name mounted into task containers |
| `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT` | - | VM-side mount root for the shared workspace data |
| `RUN_TASK_AZURE_ACI_CONTAINER_SHARE_ROOT` | `/mnt/dowhiz-share` | In-container mount root for the same Azure Files share |
| `RUN_TASK_AZURE_ACI_STORAGE_ACCOUNT` | `AZURE_STORAGE_ACCOUNT` fallback | Storage account for Azure Files mount |
| `RUN_TASK_AZURE_ACI_STORAGE_KEY` | - | Storage key for Azure Files mount |
| `RUN_TASK_AZURE_ACI_STORAGE_CONNECTION_STRING` | - | Optional source of `AccountKey` if `RUN_TASK_AZURE_ACI_STORAGE_KEY` unset |
| `RUN_TASK_AZURE_ACI_REGISTRY_SERVER` | derived from image | Optional private registry login server |
| `RUN_TASK_AZURE_ACI_REGISTRY_USERNAME` | - | Optional private registry username (set with password) |
| `RUN_TASK_AZURE_ACI_REGISTRY_PASSWORD` | - | Optional private registry password (set with username) |

Important host prerequisite for `RUN_TASK_EXECUTION_BACKEND=azure_aci`:
- `RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT` must be a mounted Azure Files path on the VM before worker startup.
- Startup/deploy scripts now call `DoWhiz_service/scripts/ensure_aci_share_mount.sh` to enforce this.
- The script behavior:
  - skip for non-ACI backends,
  - verify mount already exists,
  - if not mounted, run `mount <RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT>` from `/etc/fstab`,
  - fail fast when `/etc/fstab` is missing that mount entry.

One-time bootstrap example on VM (production):
```bash
sudo mkdir -p /etc/smbcredentials
sudo tee /etc/smbcredentials/dwhzoliverdev >/dev/null <<'EOF'
username=dwhzoliverdev
password=<storage-account-key>
EOF
sudo chmod 600 /etc/smbcredentials/dwhzoliverdev

sudo mkdir -p /home/azureuser/server/.dowhiz/DoWhiz/run_task
echo '//dwhzoliverdev.file.core.windows.net/dowhiz-run-task-prod /home/azureuser/server/.dowhiz/DoWhiz/run_task cifs vers=3.0,credentials=/etc/smbcredentials/dwhzoliverdev,dir_mode=0777,file_mode=0777,serverino,uid=1000,gid=1000,mfsymlinks,_netdev,nofail 0 0' | sudo tee -a /etc/fstab
sudo mount /home/azureuser/server/.dowhiz/DoWhiz/run_task
```

### Inbound Gateway
| Variable | Default | Description |
|----------|---------|-------------|
| `GATEWAY_CONFIG_PATH` | `gateway.toml` | Path to gateway config file |
| `GATEWAY_HOST` | `0.0.0.0` | Gateway bind host |
| `GATEWAY_PORT` | `9100` | Gateway bind port |
| `GATEWAY_MAX_BODY_BYTES` | `26214400` | Max inbound body size (25MB) |
| `POSTMARK_INBOUND_TOKEN` | - | Verify Postmark webhook (`X-Postmark-Token`) |

### Azure Ingestion (Service Bus + Blob)
| Variable | Default | Description |
|----------|---------|-------------|
| `INGESTION_QUEUE_BACKEND` | `postgres` | `postgres` or `servicebus` (gateway requires `servicebus`) |
| `SERVICE_BUS_CONNECTION_STRING` | - | Service Bus namespace connection string |
| `SERVICE_BUS_QUEUE_NAME` | `ingestion` | Shared queue name for all employees |
| `SERVICE_BUS_TEST_QUEUE_NAME` | `ingestion-test` | Queue used by Service Bus tests |
| `SERVICE_BUS_PEEK_LOCK_TIMEOUT_SECS` | `30` | Peek-lock timeout for Service Bus receive |
| `SERVICE_BUS_LOCK_RENEW_INTERVAL_SECS` | `15` | Worker lock-renew cadence for claimed messages (bounded to 5-60s) |
| `RAW_PAYLOAD_STORAGE_BACKEND` | `supabase` | `supabase` or `azure` (`azure` recommended for gateway production flow) |
| `AZURE_STORAGE_ACCOUNT` | - | Azure Storage account name |
| `AZURE_STORAGE_CONNECTION_STRING_INGEST` | - | Optional connection string (used to derive account name for ingestion SAS URLs) |
| `AZURE_STORAGE_CONTAINER_INGEST` | - | Azure Blob container for raw payloads |
| `AZURE_STORAGE_SAS_TOKEN` | - | SAS token for container access |
| `AZURE_STORAGE_CONTAINER_SAS_URL` | - | Full container SAS URL (optional) |
| `AZURE_APIM_POSTMARK_URL` | - | APIM ingress URL |

### Azure Memo Storage
| Variable | Default | Description |
|----------|---------|-------------|
| `AZURE_STORAGE_CONNECTION_STRING` | - | Azure Blob connection string for memo storage |
| `AZURE_STORAGE_CONTAINER` | - | Azure Blob container for memo.md (e.g., `memos`) |

### Slack
| Variable | Default | Description |
|----------|---------|-------------|
| `SLACK_SIGNING_SECRET` | - | Verify Slack signatures (gateway) |
| `SLACK_BOT_TOKEN` | - | Bot token for outbound messages (legacy single-workspace) |
| `SLACK_BOT_USER_ID` | - | Bot user id (filter self messages) |
| `SLACK_CLIENT_ID` | - | Slack OAuth client id |
| `SLACK_CLIENT_SECRET` | - | Slack OAuth client secret |
| `SLACK_REDIRECT_URI` | - | Slack OAuth redirect URI |
| `SLACK_AUTH_REDIRECT_URI` | - | Slack account-linking OAuth redirect URI (`/auth/slack/callback`) |
| `SLACK_STORE_PATH` | `<runtime_root>/state/slack.db` | Slack installation store |
| `SLACK_API_BASE_URL` | `https://slack.com/api` | Override Slack API base URL |

### Discord
| Variable | Default | Description |
|----------|---------|-------------|
| `DISCORD_BOT_TOKEN` | - | Discord bot token |
| `DISCORD_BOT_USER_ID` | - | Bot user id (filter self messages) |
| `BOILED_EGG_DISCORD_BOT_TOKEN` | - | Per-employee Discord bot token (boiled_egg) |
| `BOILED_EGG_DISCORD_BOT_USER_ID` | - | Per-employee Discord bot user id (boiled_egg) |
| `LITTLE_BEAR_DISCORD_BOT_TOKEN` | - | Per-employee Discord bot token (little_bear) |
| `LITTLE_BEAR_DISCORD_BOT_USER_ID` | - | Per-employee Discord bot user id (little_bear) |
| `DISCORD_CLIENT_ID` | - | Discord OAuth client id (account linking) |
| `DISCORD_CLIENT_SECRET` | - | Discord OAuth client secret (account linking) |
| `DISCORD_REDIRECT_URI` | - | Discord OAuth redirect URI (account linking) |
| `DISCORD_API_BASE_URL` | `https://discord.com/api/v10` | Override Discord API base URL |

### BlueBubbles (iMessage)
| Variable | Default | Description |
|----------|---------|-------------|
| `BLUEBUBBLES_WEBHOOK_TOKEN` | - | Verify BlueBubbles webhook token |
| `BLUEBUBBLES_URL` | - | BlueBubbles server URL |
| `BLUEBUBBLES_PASSWORD` | - | BlueBubbles server password |

### SMS (Twilio)
| Variable | Default | Description |
|----------|---------|-------------|
| `TWILIO_ACCOUNT_SID` | - | Twilio account SID |
| `TWILIO_AUTH_TOKEN` | - | Twilio auth token (outbound + webhook verification) |
| `TWILIO_API_BASE_URL` | `https://api.twilio.com` | Override Twilio API base URL |
| `TWILIO_WEBHOOK_URL` | - | Public URL used to validate Twilio signatures |

### Telegram
| Variable | Default | Description |
|----------|---------|-------------|
| `TELEGRAM_BOT_TOKEN` | - | Global Telegram bot token |
| `DO_WHIZ_<EMPLOYEE>_BOT` | - | Per-employee bot token override (e.g., `DO_WHIZ_OLIVER_BOT`) |

### Google Workspace (Docs/Sheets/Slides)
| Variable | Default | Description |
|----------|---------|-------------|
| `GOOGLE_CLIENT_ID` | - | OAuth client id |
| `GOOGLE_CLIENT_SECRET` | - | OAuth client secret |
| `GOOGLE_REFRESH_TOKEN` | - | Global refresh token (fallback) |
| `GOOGLE_REFRESH_TOKEN_<EMPLOYEE>` | - | Per-employee refresh token (uppercase ID) |
| `GOOGLE_SERVICE_ACCOUNT_JSON` | - | Service account JSON (alternative auth) |
| `GOOGLE_ACCESS_TOKEN` | - | Pre-generated access token (sandbox/CLI) |
| `GOOGLE_DOCS_ENABLED` | `false` | Enable Google Docs comment polling in the gateway workspace poller |
| `GOOGLE_DOCS_POLL_INTERVAL_SECS` | `30` | Legacy docs-only poller interval (not used by default gateway flow) |
| `GOOGLE_SHEETS_ENABLED` | `false` | Enable Google Sheets polling |
| `GOOGLE_SLIDES_ENABLED` | `false` | Enable Google Slides polling |
| `GOOGLE_WORKSPACE_POLL_INTERVAL_SECS` | `15` | Poll interval for gateway workspace poller (Docs/Sheets/Slides) |
| `GOOGLE_EMPLOYEE_EMAILS` | - | Comma-separated emails to ignore as "self" in Google Workspace comments |
| `GOOGLE_DRIVE_PUSH_ENABLED` | `false` | Enable Google Drive push notifications for Docs/Sheets file changes (Slides unsupported by Drive watch API) |
| `GOOGLE_DRIVE_WEBHOOK_URL` | - | Public webhook URL for push notifications (`/webhooks/google-drive-changes`) |
| `GOOGLE_DRIVE_CHANNEL_EXPIRATION_SECS` | `3600` | Watch channel expiration interval |

### WhatsApp (Meta Cloud API)
| Variable | Default | Description |
|----------|---------|-------------|
| `WHATSAPP_ACCESS_TOKEN` | - | Cloud API access token for outbound sends |
| `WHATSAPP_PHONE_NUMBER_ID` | - | Phone number ID for the bot |
| `WHATSAPP_VERIFY_TOKEN` | - | Verify token for webhook subscription |

### Fanout Gateway (Legacy)
| Variable | Default | Description |
|----------|---------|-------------|
| `FANOUT_HOST` | - | Gateway host |
| `FANOUT_PORT` | - | Gateway port |
| `FANOUT_TARGETS` | - | Comma-separated list of target URLs |
| `FANOUT_TIMEOUT_SECS` | - | Request timeout |

### Message Router
| Variable | Default | Description |
|----------|---------|-------------|
| `ROUTER_ENABLED` | `true` | Set to `false` to disable |
| `ROUTER_MODEL` | `gpt-5.2` | Model name |
| `AZURE_OPENAI_API_KEY_BACKUP` | - | Azure OpenAI key (preferred) |
| `AZURE_OPENAI_ENDPOINT_BACKUP` | - | Azure OpenAI base URL |
| `OPENAI_API_KEY` | - | Required to enable routing |
| `OPENAI_API_URL` | `https://api.openai.com/v1` | OpenAI base URL |

### Inbound Blacklist
Any address listed in `employee.toml` is ignored as a sender (prevents loops; display names and `+tag` aliases are normalized).

---

## Runtime State

Default location: `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/`

Each employee gets isolated directories unless you override paths with environment variables.

```
$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/
├── state/
│   ├── tasks.db                    # Global scheduler state
│   ├── users.db                    # User registry
│   ├── task_index.db               # Due task index for polling
│   └── postmark_processed_ids.txt  # Deduplication list
├── workspaces/<message_id>/
│   ├── workspace/                  # Agent workspace
│   ├── references/past_emails/     # Hydrated email history
│   ├── reply_email_draft.html      # Email / Google Workspace reply
│   ├── reply_email_attachments/    # Email / Google Workspace attachments
│   ├── reply_message.txt           # Chat/SMS reply (Slack/Discord/Telegram/SMS/WhatsApp/BlueBubbles)
│   └── reply_attachments/          # Chat/SMS attachments
└── users/<user_id>/
    ├── state/tasks.db              # Per-user task queue
    ├── memory/                     # Agent context/memory
    └── mail/                       # Email archive
```

---

## Database Schema

### MongoDB Collections
Scheduler/runtime state is stored in MongoDB collections:

- `users` - normalized user identifiers and last-seen timestamps.
- `tasks` - scheduled task payloads and scheduling metadata.
- `task_executions` - task run history/status/error.
- `task_index` - due-task index for scheduler scanning.
- `slack_installations` - Slack OAuth installations.
- `processed_comments` / `workspace_files` - Google Docs/Workspace polling state.
- `collaboration_sessions` / `collaboration_messages` / `collaboration_artifacts` - multi-channel collaboration context.

Owner/user isolation is enforced with scoped fields (for example `owner_scope` on tasks and per-store scope keys), so one user's tasks cannot modify or read another user's scheduler records.

---

## Past Email Hydration

Each new workspace populates `references/past_emails/` from the user archive under `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/users/<user_id>/mail`.

The hydrator copies `incoming_email/` and any attachments <= 50MB; larger attachments are referenced via `attachments_manifest.json` (set `*.azure_url` sidecar files to supply the Azure blob URL if needed).

Outgoing agent replies are archived after successful `send_reply` execution and appear in `past_emails` with `direction: "outbound"` (for channels that maintain email-style archives).

**Manual run:**
```bash
cargo run -p scheduler_module --bin hydrate_past_emails -- \
  --archive-root $HOME/.dowhiz/DoWhiz/run_task/<employee_id>/users/<user_id>/mail \
  --references-dir /path/to/workspace/references \
  --user-id <user_id>
```

### index.json schema

Entry directories are named `YYYY-MM-DD_<action>_<topic>_<shortid>`. `direction` is `"inbound"` or `"outbound"`.

```json
{
  "version": 1,
  "generated_at": "RFC3339 timestamp",
  "user_id": "uuid",
  "entries": [
    {
      "entry_id": "message-id",
      "display_name": "2026-02-03_message_archive-hello_abc123",
      "path": "2026-02-03_message_archive-hello_abc123",
      "direction": "inbound",
      "subject": "Archive hello",
      "from": "Sender <sender@example.com>",
      "to": "Recipient <recipient@example.com>",
      "cc": "",
      "bcc": "",
      "date": "RFC3339 timestamp",
      "message_id": "message-id",
      "attachments_manifest": "2026-02-03_message_archive-hello_abc123/attachments_manifest.json",
      "attachments_count": 1,
      "large_attachments_count": 0
    }
  ]
}
```

### attachments_manifest.json schema

```json
{
  "version": 1,
  "generated_at": "RFC3339 timestamp",
  "message_id": "message-id",
  "attachments": [
    {
      "file_name": "report.pdf",
      "original_name": "Report.pdf",
      "content_type": "application/pdf",
      "size_bytes": 12345,
      "storage": "local",
      "relative_path": "incoming_attachments/report.pdf",
      "azure_blob_url": null
    }
  ]
}
```

---

## Scheduled Follow-ups

If the agent needs to send a follow-up later, it should emit a schedule block to stdout at the end of its response. The scheduler parses the block and stores follow-up tasks in MongoDB.
`"type":"send_email"` remains the wire-format tag for backward compatibility, and is mapped to `SendReply` internally.

**Example schedule block:**
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"Quick reminder","html_path":"reminder_email_draft.html","attachments_dir":"reminder_email_attachments","to":["you@example.com"],"cc":[],"bcc":[]}]
SCHEDULED_TASKS_JSON_END
```

### Task Kinds
- **SendReply**: Send outbound reply by channel (email/chat/sms/google workspace)
- **RunTask**: Invoke Codex/Claude CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime
