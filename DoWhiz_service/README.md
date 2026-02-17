# DoWhiz Service

Rust service for inbound webhooks (Postmark, Slack, Discord), task scheduling, AI agent execution (Codex/Claude), and outbound replies.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Install Dependencies](#install-dependencies)
- [Employee Configuration](#employee-configuration)
- [Running the Service](#running-the-service)
  - [One-Command Local Run](#one-command-local-run)
  - [Manual Multi-Employee Setup](#manual-multi-employee-setup)
  - [Inbound Gateway (Recommended)](#inbound-gateway-recommended)
  - [VM Deployment (Gateway + ngrok)](#vm-deployment-gateway--ngrok)
  - [Fanout Gateway (Legacy)](#fanout-gateway-legacy)
  - [Docker Production](#docker-production)
- [Per-Task Docker Execution](#per-task-docker-execution)
- [Testing](#testing)
  - [Unit Tests](#unit-tests)
  - [Live E2E Tests](#live-e2e-tests)
  - [Slack Local Testing](#slack-local-testing)
- [Message Router (Ollama)](#message-router-ollama)
- [Environment Variables](#environment-variables)
- [Runtime State](#runtime-state)
- [Database Schema](#database-schema)
- [Past Email Hydration](#past-email-hydration)
- [Scheduled Follow-ups](#scheduled-follow-ups)

---

## Prerequisites

- Rust toolchain
- System libs: `libsqlite3`, `libssl`, `pkg-config`, `ca-certificates`
- Node.js 20 + npm
- `codex` CLI on your PATH (only required for local execution; optional when `RUN_TASK_DOCKER_IMAGE` is set)
- `claude` CLI on your PATH (only required for employees with `runner = "claude"`)
- `playwright-cli` + Chromium (required for browser automation skills)
- `ngrok` (for exposing local service to webhooks)
- `python3` (for ngrok URL discovery)
- Ollama (optional; required for local message routing via phi3:mini)

**Required in `.env`** (copy from repo-root `.env.example` to `DoWhiz_service/.env`):
- `POSTMARK_SERVER_TOKEN`
- `AZURE_OPENAI_API_KEY_BACKUP` and `AZURE_OPENAI_ENDPOINT_BACKUP` (required when Codex is enabled)

**Optional in `.env`**:
- `GITHUB_USERNAME` + `GITHUB_PERSONAL_ACCESS_TOKEN` (enables Codex/agent GitHub access via `GH_TOKEN`/`GITHUB_TOKEN` + git askpass)
- `RUN_TASK_DOCKER_IMAGE` (run each task inside a disposable Docker container; use `dowhiz-service` for the repo image)
- `RUN_TASK_DOCKER_AUTO_BUILD=1` to auto-build the image when missing (set `0` to disable)

---

## Install Dependencies

### Linux (Debian/Ubuntu)

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

Optional: Install Ollama for local message routing:
```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama pull phi3:mini
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
brew install node@20 openssl@3 sqlite pkg-config ollama
npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
npx playwright install chromium
ollama pull phi3:mini
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
| `boiled_egg` | Proto | Codex | proto@dowhiz.com, boiled-egg@dowhiz.com |

`employee.toml` also supports `runtime_root` per employee to override the default runtime location (for repo-local runs, use `.workspace/<employee_id>` relative to `DoWhiz_service/employee.toml`).

Skills are copied per workspace: the base repo skills are always included, and `skills_dir` can optionally add overrides or extra skills.

---

## Running the Service

### One-Command Local Run

From the repo root:
```bash
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
./DoWhiz_service/scripts/run_employee.sh mini_mouse 9002
./DoWhiz_service/scripts/run_employee.sh sticky_octopus 9003
./DoWhiz_service/scripts/run_employee.sh boiled_egg 9004
```

This command:
- Starts ngrok and discovers the public URL
- Updates the Postmark inbound hook to `https://.../postmark/inbound`
- Runs the Rust service bound to the selected host/port

Requires `POSTMARK_SERVER_TOKEN` in `DoWhiz_service/.env`, plus `ngrok` and `python3` installed.

**Optional flags:**
- `--public-url https://example.com` uses an existing public URL and skips ngrok
- `--skip-hook` leaves the Postmark hook unchanged
- `--skip-ngrok` disables ngrok (requires `--public-url` or `--skip-hook`)

When running with the inbound gateway, start workers with `--skip-hook` so they do not overwrite the gateway's Postmark hook.

**Full usage:**
```
scripts/run_employee.sh <employee_id> [port]
scripts/run_employee.sh --employee <id> --port <port> [--public-url <url>] [--skip-hook] [--skip-ngrok] [--host <host>]
```

### Manual Multi-Employee Setup

**Step 1: Start the Rust service (Terminal 1)**
```bash
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

**Step 2: Expose the service with ngrok (Terminal 2)**
```bash
ngrok http 9001   # or 9002 for mini_mouse
```

**Step 3: Set the Postmark inbound hook (Terminal 3)**
```bash
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

**Step 4: Send an email**
```
oliver@dowhiz.com   # or mini-mouse@dowhiz.com, devin@dowhiz.com, proto@dowhiz.com
```

**Step 5: Watch logs for task execution**

Outputs appear under:
- `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/reply_email_draft.html`
- `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/workspaces/<message_id>/reply_email_attachments/`
- Scheduler state: `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/state/tasks.db`

### Inbound Gateway (Recommended)

The inbound gateway (`inbound_gateway`) is the recommended way to run multiple employees without webhook collisions. It accepts inbound webhooks, deduplicates, routes to a single employee worker, and returns immediately. Each worker sends the actual reply back to the channel.

**Phase 1 local gateway test (2 Docker workers + 1 local worker)**

This matches the staging/dev flow with Oliver + Maggie in Docker and Proto on your host:

**Step 1: Build the Docker image (once)**
```bash
docker build -t dowhiz-service .
```

**Step 2: Start Oliver worker (Docker)**
```bash
docker run --rm -p 9001:9001 \
  -e EMPLOYEE_ID=little_bear \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_DOCKER_IMAGE= \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace-oliver:/app/.workspace \
  dowhiz-service
```

**Step 3: Start Maggie worker (Docker)**
```bash
docker run --rm -p 9002:9001 \
  -e EMPLOYEE_ID=mini_mouse \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_DOCKER_IMAGE= \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace-maggie:/app/.workspace \
  dowhiz-service
```

Note: when running workers inside Docker, clear `RUN_TASK_DOCKER_IMAGE` to avoid nested Docker usage.

**Step 4: Start Proto worker (host)**
```bash
EMPLOYEE_ID=boiled_egg RUST_SERVICE_PORT=9004 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9004
```

**Step 5: Configure the gateway**
```bash
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
```
Edit `DoWhiz_service/gateway.toml` to point at your local ports:
```toml
[targets]
little_bear = "http://127.0.0.1:9001"
mini_mouse  = "http://127.0.0.1:9002"
boiled_egg  = "http://127.0.0.1:9004"

[slack]
default_employee_id = "boiled_egg"
```

**Step 6: Start the gateway (host)**
```bash
./DoWhiz_service/scripts/run_gateway_local.sh
```

**Step 7: Expose the gateway with ngrok**
```bash
ngrok http 9100
```

**Step 8: Point Postmark to the gateway**
```bash
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

**Step 9: Send test emails**
Use Postmark inbound with the test sender identities:
- From: `mini-mouse@deep-tutor.com` → To: `mini-mouse@dowhiz.com`
- From: `deep-tutor@deep-tutor.com` → To: `proto@dowhiz.com`

If you want a scripted smoke test, reuse the live Postmark script below and set:
```
POSTMARK_INBOUND_HOOK_URL=http://127.0.0.1:9100/postmark/inbound
POSTMARK_TEST_FROM=mini-mouse@deep-tutor.com
POSTMARK_TEST_SERVICE_ADDRESS=mini-mouse@dowhiz.com
```
Repeat with `POSTMARK_TEST_FROM=deep-tutor@deep-tutor.com` and `POSTMARK_TEST_SERVICE_ADDRESS=proto@dowhiz.com`.

### VM Deployment (Gateway + ngrok)

This is the current production flow (oliver on `dowhizprod1`): run a single worker behind the inbound gateway and expose the gateway with ngrok.

1. Provision an Ubuntu VM and open inbound TCP ports `22`, `80`, `443`.
Outbound SMTP (`25`) is often blocked on cloud VMs; run E2E senders from your local machine if needed.

2. Install dependencies + ngrok (VM):
```bash
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl git python3
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
curl https://sh.rustup.rs -sSf | sh -s -- -y
sudo snap install ngrok
```

3. Clone the repo and configure `.env` (VM):
```bash
git clone https://github.com/KnoWhiz/DoWhiz.git
cd DoWhiz
cp .env.example DoWhiz_service/.env
# Edit DoWhiz_service/.env with production secrets
```

Optional: copy your local `.env` directly to the VM:
```bash
scp /path/to/DoWhiz_service/.env azureuser@<vm>:/home/azureuser/DoWhiz/DoWhiz_service/.env
```

4. Configure the gateway to route only Oliver:
```bash
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
cat > DoWhiz_service/gateway.toml <<'EOF'
[targets]
little_bear = "http://127.0.0.1:9001"
EOF
```

5. Start services (tmux recommended):
```bash
tmux new-session -d -s oliver "bash -lc 'cd ~/DoWhiz/DoWhiz_service && set -a && source .env && set +a && EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 RUN_TASK_DOCKER_IMAGE= cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001'"
tmux new-session -d -s gateway "bash -lc 'cd ~/DoWhiz/DoWhiz_service && set -a && source .env && set +a && ./scripts/run_gateway_local.sh'"
ngrok config add-authtoken "$NGROK_AUTHTOKEN"
tmux new-session -d -s ngrok "ngrok http 9100 --url https://oliver.dowhiz.prod.ngrok.app"
```

6. Health checks (VM):
```bash
curl -sS http://127.0.0.1:9001/health && echo
curl -sS http://127.0.0.1:9100/health && echo
curl -sS https://oliver.dowhiz.prod.ngrok.app/health && echo
```

7. Point Postmark to the gateway (VM):
```bash
cd ~/DoWhiz/DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://oliver.dowhiz.prod.ngrok.app/postmark/inbound
```

8. Run live E2E (from your local machine if VM blocks SMTP 25):
```
POSTMARK_INBOUND_HOOK_URL=https://oliver.dowhiz.prod.ngrok.app/postmark/inbound
POSTMARK_TEST_FROM=mini-mouse@deep-tutor.com
POSTMARK_TEST_SERVICE_ADDRESS=oliver@dowhiz.com
```
Use the Live E2E driver script in [Live E2E Tests](#live-e2e-tests).

#### Nginx + systemd (optional)

If you prefer terminating HTTPS on the VM directly (no ngrok), use Nginx + systemd and point Postmark to the VM domain.

Environment file path (update to match current layout):
```
EnvironmentFile=/home/azureuser/DoWhiz/DoWhiz_service/.env
```

Example systemd service:
```ini
[Unit]
Description=DoWhiz Rust Service (Oliver)
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

If you need a broadcast-style gateway that forwards every inbound request to **all** employee services, use the legacy fanout gateway. For most deployments, prefer the inbound gateway so each message is routed to a single worker.

**Local (fanout only):**
```bash
./DoWhiz_service/scripts/run_fanout_local.sh
```

**Override targets/port:**
```bash
FANOUT_TARGETS="http://127.0.0.1:9001,http://127.0.0.1:9002,http://127.0.0.1:9003" \
FANOUT_PORT=9100 \
./DoWhiz_service/scripts/run_fanout_local.sh
```

**Docker (fanout + all employees in one command):**
```bash
./DoWhiz_service/scripts/run_all_employees_docker.sh
```

Point Postmark inbound hook and Slack event subscriptions at the **fanout** URL:
- `https://<ngrok>.ngrok.app/postmark/inbound`
- `https://<ngrok>.ngrok.app/slack/events`

**Proto (boiled_egg) debug:**
```bash
./DoWhiz_service/scripts/run_proto_docker.sh
```

**Local (no Docker):**
```bash
./DoWhiz_service/scripts/run_employee.sh boiled_egg 9004 --skip-hook
```

### Docker Production

Build the image from the repo root and run it with the same `.env` file mounted:

```bash
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace:/app/.workspace \
  dowhiz-service
```

If `RUN_TASK_DOCKER_IMAGE` is set in your `.env`, each task runs inside a fresh Docker container and the image auto-builds on first use (unless disabled with `RUN_TASK_DOCKER_AUTO_BUILD=0`).

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
    codex exec --skip-git-repo-check -c web_search=\"disabled\" --cd \"$WORKDIR\" --dangerously-bypass-approvals-and-sandbox \
    \"Test the \\\"add todo\\\" flow on https://demo.playwright.dev/todomvc using playwright-cli. Check playwright-cli --help for available commands.\""
```

---

## Per-Task Docker Execution

When `RUN_TASK_DOCKER_IMAGE` is set, each RunTask spins up a fresh container, mounts the task workspace at `/workspace`, runs Codex inside the container, and removes the container when done.

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
- `POSTMARK_SERVER_TOKEN`, `POSTMARK_TEST_FROM`, `AZURE_OPENAI_API_KEY_BACKUP`, and `AZURE_OPENAI_ENDPOINT_BACKUP` set
- `RUN_CODEX_E2E=1` if you want Codex to execute real tasks (otherwise it is disabled in the live test)

**Docker flow (run service in Docker, then drive the live email from the host):**

1. Start ngrok:
```bash
ngrok http 9002   # mini_mouse
ngrok http 9001   # little_bear
```

2. Start the container (match the same port you exposed with ngrok):
```bash
docker run --rm -p 9002:9002 \
  -e EMPLOYEE_ID=mini_mouse \
  -e RUST_SERVICE_PORT=9002 \
  -e RUN_TASK_SKIP_WORKSPACE_REMAP=1 \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace:/app/.workspace \
  dowhiz-service
```

For `little_bear` (Codex), add `-e CODEX_BYPASS_SANDBOX=1` if Codex fails with Landlock sandbox errors inside Docker.

3. Run the live driver:
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
ngrok http 9002   # mini_mouse
ngrok http 9001   # little_bear
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

1. Set up the ngrok tunnel on port 9100:
```bash
ngrok http 9100 --authtoken=${NGROK_AUTHTOKEN} --url https://YOUR-DOMAIN.ngrok.app
```

2. Start the dev employee (`boiled_egg`):
```bash
cargo build
cd DoWhiz/DoWhiz_service && cargo run --bin inbound_gateway --release
./DoWhiz_service/scripts/run_employee.sh boiled_egg 9001 --public-url https://YOUR-DOMAIN.ngrok.app --skip-hook
```
3. Go to OAuth URL: `https://YOUR-DOMAIN.ngrok.app/slack/oauth/callback`
   - This endpoint may be unreachable on workstations with SafeBrowse
   - To bypass this, go to this URL on your mobile device with Wi-Fi turned off (use mobile data)

4. After OAuth, invite the bot to the channel:
   - In Slack, go to the channel and type `/invite @DoWhiz` (or click the channel settings → Integrations → Add apps)

### Discord Local Testing

1. Start the dev employee (`boiled_egg`):
```bash
cargo build
cd DoWhiz/DoWhiz_service && cargo run --bin inbound_gateway --release
./DoWhiz_service/scripts/run_employee.sh boiled_egg 9001 --public-url https://YOUR-DOMAIN.ngrok.app --skip-hook
```

2. Add the bot to server by going to this URL:
   - https://discord.com/oauth2/authorize?client_id=1472013251553525983&permissions=0&integration_type=0&scope=bot

### iMessage Local Testing via BlueBubbles
1. Download BlueBubbles (e.g. ```brew install --cask bluebubbles```)
2. In API & WebHooks, create a new webhook at http://127.0.0.1:9100/bluebubbles/webhook. Give it the New Messages Event Subscription.
3. Start the dev employee (`boiled_egg`):
```bash
cargo build
cd DoWhiz/DoWhiz_service && cargo run --bin inbound_gateway --release
./DoWhiz_service/scripts/run_employee.sh boiled_egg 9001 --public-url https://YOUR-DOMAIN.ngrok.app --skip-hook
```

### Google Docs Integration

Digital employees can collaborate on Google Docs with color-coded revision marks (suggesting mode).

### Features
- **@Mention Detection**: Employees detect when mentioned in document comments
- **Suggesting Mode**: Edits appear as color-coded revisions:
  - 🔴 **Red strikethrough** = Deletions
  - 🔵 **Blue text** = Insertions
- **Apply/Discard**: Users can accept or reject all suggestions in batch
- **Comment Replies**: Employees can respond to document comments

#### Quick Setup

##### Step 1: Create Google Cloud Project

1. Go to [Google Cloud Console](https://console.cloud.google.com)
2. Create a new project or select existing
3. Enable APIs:
   - Google Docs API
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
```

#### Testing

##### Manual CLI Test
```bash
cd DoWhiz_service

# Build the CLI
cargo build --release --bin google-docs

# List accessible documents
./target/release/google-docs list-documents

# Read a document
./target/release/google-docs read-document <doc_id>

# Test suggesting mode (find and replace with revision marks)
./target/release/google-docs suggest-replace <doc_id> --find="old text" --replace="new text"

# Apply all suggestions (removes red, normalizes blue)
./target/release/google-docs apply-suggestions <doc_id>

# Discard all suggestions (removes blue, restores red)
./target/release/google-docs discard-suggestions <doc_id>
```

##### E2E Tests
```bash
cargo test --package scheduler_module --test google_docs_cli_e2e
```

#### Multi-Employee Setup

Each employee needs their own Google account and refresh token:

| Employee | Env Variable | Google Account |
|----------|-------------|----------------|
| Proto (boiled_egg) | `GOOGLE_REFRESH_TOKEN_BOILED_EGG` | proto@dowhiz.com |
| Oliver (little_bear) | `GOOGLE_REFRESH_TOKEN_LITTLE_BEAR` | oliver@dowhiz.com |

Run `get_google_refresh_token.sh` once per employee, logging into the appropriate Google account each time.

#### Troubleshooting

| Issue | Solution |
|-------|----------|
| `DNS lookup failed` | Ensure sandbox bypass is enabled for GoogleDocs tasks |
| `Token refresh failed` | Re-run `get_google_refresh_token.sh` to get a new token |
| `\n` appearing literally | Upgrade to latest CLI with escape sequence support |
| `Permission denied` | Share the document with the employee's Google account |

---
## Message Router (Ollama)

The service includes a local LLM message router that classifies incoming Discord messages using Ollama. Simple queries (greetings, casual chat) are handled directly by a local model (phi3:mini), while complex queries are forwarded to the full Codex/Claude pipeline.

### Configuration

Environment variables:
- `OLLAMA_URL`: Ollama server URL (default: `http://localhost:11434`)
- `OLLAMA_MODEL`: Model to use (default: `phi3:mini`)
- `OLLAMA_ENABLED`: Set to `"false"` to disable routing (default: enabled)

### Docker Setup

The `docker-compose.fanout.yml` includes an Ollama sidecar container. After starting the containers, pull the model:

```bash
docker exec dowhiz_service-ollama-1 ollama pull phi3:mini
```

The model is persisted in a Docker volume (`ollama-models`) so it only needs to be pulled once.

### How it works

1. Incoming Discord messages are classified by phi3:mini (~200-500ms)
2. Simple queries get a quick local response
3. Complex queries are forwarded to the full pipeline (Codex/Claude)

This reduces API costs and latency for simple interactions while preserving full capability for complex tasks.

---

## Environment Variables

### Service Configuration
| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_SERVICE_HOST` | `0.0.0.0` | Host to bind |
| `RUST_SERVICE_PORT` | `9001` | Port to bind |
| `EMPLOYEE_ID` | - | Selects employee profile from `employee.toml` |
| `EMPLOYEE_CONFIG_PATH` | `DoWhiz_service/employee.toml` | Path to employee config |

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
| `SCHEDULER_MAX_CONCURRENCY` | `10` | Global max concurrent tasks |
| `SCHEDULER_USER_MAX_CONCURRENCY` | `3` | Per-user max concurrent tasks |

### Codex (OpenAI)
| Variable | Default | Description |
|----------|---------|-------------|
| `CODEX_MODEL` | - | Model name |
| `CODEX_DISABLED` | `0` | Set to `1` to bypass Codex CLI |
| `CODEX_SANDBOX` | `workspace-write` | Sandbox mode |
| `CODEX_BYPASS_SANDBOX` | `0` | Set to `1` to bypass sandbox (sometimes required inside Docker) |
| `AZURE_OPENAI_API_KEY_BACKUP` | - | Azure OpenAI API key |
| `AZURE_OPENAI_ENDPOINT_BACKUP` | - | Azure OpenAI endpoint |

### Claude (Anthropic)
| Variable | Default | Description |
|----------|---------|-------------|
| `CLAUDE_MODEL` | `claude-opus-4-5` | Model name |
| `ANTHROPIC_FOUNDRY_RESOURCE` | `knowhiz-service-openai-backup-2` | Foundry resource |

### Docker Execution
| Variable | Default | Description |
|----------|---------|-------------|
| `RUN_TASK_DOCKER_IMAGE` | - | Enable per-task containers |
| `RUN_TASK_DOCKER_REQUIRED` | `0` | Fail when Docker CLI is missing instead of falling back to host execution |
| `RUN_TASK_DOCKER_AUTO_BUILD` | `1` | Auto-build missing images |
| `RUN_TASK_DOCKERFILE` | - | Override Dockerfile path |
| `RUN_TASK_DOCKER_BUILD_CONTEXT` | - | Override build context directory |
| `RUN_TASK_DOCKER_NETWORK` | - | Docker network mode (e.g., `host`) |
| `RUN_TASK_DOCKER_DNS` | - | Override Docker DNS servers (comma/space-separated) |
| `RUN_TASK_DOCKER_DNS_SEARCH` | - | Add DNS search domains (comma/space-separated) |
| `RUN_TASK_SKIP_WORKSPACE_REMAP` | `0` | Disable legacy workspace path migration |

### Inbound Gateway
| Variable | Default | Description |
|----------|---------|-------------|
| `GATEWAY_CONFIG_PATH` | `gateway.toml` | Path to gateway config file |
| `GATEWAY_DEDUPE_PATH` | `$HOME/.dowhiz/DoWhiz/gateway/state/processed_ids.txt` | Gateway dedupe file |
| `GATEWAY_HOST` | `0.0.0.0` | Gateway bind host |
| `GATEWAY_PORT` | `9100` | Gateway bind port |

### Fanout Gateway (Legacy)
| Variable | Default | Description |
|----------|---------|-------------|
| `FANOUT_HOST` | - | Gateway host |
| `FANOUT_PORT` | - | Gateway port |
| `FANOUT_TARGETS` | - | Comma-separated list of target URLs |
| `FANOUT_TIMEOUT_SECS` | - | Request timeout |

### Ollama
| Variable | Default | Description |
|----------|---------|-------------|
| `OLLAMA_URL` | `http://localhost:11434` | Ollama server URL |
| `OLLAMA_MODEL` | `phi3:mini` | Model to use |
| `OLLAMA_ENABLED` | `true` | Set to `false` to disable |

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
│   ├── reply_email_draft.html      # Generated reply
│   └── reply_email_attachments/
└── users/<user_id>/
    ├── state/tasks.db              # Per-user task queue
    ├── memory/                     # Agent context/memory
    └── mail/                       # Email archive
```

---

## Database Schema

### users.db
Table `users(id, email, created_at, last_seen_at)` stores normalized email, creation time, and last activity time (RFC3339 UTC). `last_seen_at` updates on inbound email.

### task_index.db
Global task index for due work. Table `task_index(task_id, user_id, next_run, enabled)` plus indexes on `next_run` and `user_id`. This is a derived index synced from each user's `tasks.db` and used by the scheduler thread to query due tasks efficiently.

### tasks.db (per-user)
Per-user scheduler store (SQLite with foreign keys on). Key tables:

- `tasks(id, kind, enabled, created_at, last_run, schedule_type, cron_expression, next_run, run_at)` - Scheduling metadata. `schedule_type` is `cron` or `one_shot`; cron uses `cron_expression` + `next_run`, one-shot uses `run_at`.
- `send_email_tasks(task_id, subject, html_path, attachments_dir, in_reply_to, references_header[, archive_root])` - Email task payloads. `archive_root` may be added by auto-migration.
- `send_email_recipients(id, task_id, recipient_type, address)` - `to`/`cc`/`bcc` recipients.
- `run_task_tasks(task_id, workspace_dir, input_email_dir, input_attachments_dir, memory_dir, reference_dir, model_name, runner, codex_disabled, reply_to, reply_from[, archive_root])` - RunTask parameters. `reply_to` is newline-separated; `reply_from` carries the inbound service mailbox used for replies.
- `task_executions(id, task_id, started_at, finished_at, status, error_message)` - Execution history and errors.

---

## Past Email Hydration

Each new workspace populates `references/past_emails/` from the user archive under `$HOME/.dowhiz/DoWhiz/run_task/<employee_id>/users/<user_id>/mail`.

The hydrator copies `incoming_email/` and any attachments <= 50MB; larger attachments are referenced via `attachments_manifest.json` (set `*.azure_url` sidecar files to supply the Azure blob URL if needed).

Outgoing agent replies are archived after successful `send_email` execution and appear in `past_emails` with `direction: "outbound"`.

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

If the agent needs to send a follow-up later, it should emit a schedule block to stdout at the end of its response. The scheduler parses the block and stores follow-up tasks in SQLite.

**Example schedule block:**
```
SCHEDULED_TASKS_JSON_BEGIN
[{"type":"send_email","delay_minutes":15,"subject":"Quick reminder","html_path":"reminder_email_draft.html","attachments_dir":"reminder_email_attachments","to":["you@example.com"],"cc":[],"bcc":[]}]
SCHEDULED_TASKS_JSON_END
```

### Task Kinds
- **SendEmail**: Send HTML email with attachments
- **RunTask**: Invoke Codex/Claude CLI to generate reply
- **Noop**: Testing placeholder

### Schedules
- **Cron**: 6-field format `sec min hour day month weekday` (UTC)
- **OneShot**: Single execution at specific DateTime
