# DoWhiz - A lightweight Rust replica of OpenClaw🦞

<p align="center">
  <img src="assets/readme-banner.svg" alt="DoWhiz - Any-channel digital employees" width="1200" />
</p>

<p align="center">
  <a href="LICENSE">
    <img alt="License: Apache 2.0" src="https://img.shields.io/badge/License-Apache%202.0-0f172a?style=for-the-badge" />
  </a>
  <a href="DoWhiz_service/README.md">
    <img alt="Rust service" src="https://img.shields.io/badge/Rust-Service-0ea5e9?style=for-the-badge&logo=rust&logoColor=white" />
  </a>
  <a href="website/README.md">
    <img alt="React website" src="https://img.shields.io/badge/React-Website-3b82f6?style=for-the-badge&logo=react&logoColor=white" />
  </a>
</p>

A lightweight Rust replica of OpenClaw🦞 with **better security, accessibility, and token usage**. Serve as your digital employee team, message us any task over email, Discord, Slack, Telegram, WhatsApp, iMessage, or any other channel. 🧸🐭🐙🐘👾🦞🐦🐉

## Quick Start

### 1. Prerequisites

**macOS:**
```bash
brew install node@20 openssl@3 sqlite pkg-config
npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get update && sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest
```

### 2. Configure Environment

```bash
cp .env.example DoWhiz_service/.env
# Edit DoWhiz_service/.env and add your POSTMARK_SERVER_TOKEN
```

### 3. Start Service

**Option A: Start Inbound Gateway + Workers (recommended)**
```bash
# Terminal 1: Build image (once)
docker build -t dowhiz-service .

# Terminal 2: Oliver worker (Docker)
docker run --rm -p 9001:9001 \
  -e EMPLOYEE_ID=little_bear \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_DOCKER_IMAGE= \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace-oliver:/app/.workspace \
  dowhiz-service

# Terminal 3: Maggie worker (Docker)
docker run --rm -p 9002:9001 \
  -e EMPLOYEE_ID=mini_mouse \
  -e RUST_SERVICE_PORT=9001 \
  -e RUN_TASK_DOCKER_IMAGE= \
  -v "$PWD/DoWhiz_service/.env:/app/.env:ro" \
  -v dowhiz-workspace-maggie:/app/.workspace \
  dowhiz-service

# Terminal 4: Gateway (host)
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
# Edit gateway.toml targets to match the workers you started.
./DoWhiz_service/scripts/run_gateway_local.sh

# Terminal 5: Expose the gateway
ngrok http 9100 --url https://YOUR-DOMAIN.ngrok.app

# Terminal 6: Point Postmark at the gateway
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

Note: when running workers inside Docker, clear `RUN_TASK_DOCKER_IMAGE` to avoid nested Docker usage.

**Option B: Start Single Employee (auto ngrok + hook)**
```bash
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
```

This single command starts ngrok, updates the Postmark hook, and runs the service.

**Available employees:**
| Employee | Port | Runner | Email |
|----------|------|--------|-------|
| `little_bear` | 9001 | Codex | oliver@dowhiz.com |
| `mini_mouse` | 9002 | Claude | maggie@dowhiz.com |
| `sticky_octopus` | 9003 | Codex | devin@dowhiz.com |
| `boiled_egg` | 9004 | Codex | proto@dowhiz.com |

Now send an email to `oliver@dowhiz.com` (or any employee) and watch the magic happen!

## VM Deployment (Gateway + ngrok)

This is the production flow we used for `oliver` on `dowhizprod1`: run a single worker behind the inbound gateway and expose the gateway with ngrok.

1. Provision an Ubuntu VM and open inbound TCP ports `22`, `80`, `443`.
Outbound SMTP (`25`) is often blocked on cloud VMs; run E2E senders from your local machine if needed.

2. Install dependencies and ngrok:
```bash
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl git python3
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
curl https://sh.rustup.rs -sSf | sh -s -- -y
sudo snap install ngrok
```

3. Clone repo and place `.env`:
```bash
git clone https://github.com/KnoWhiz/DoWhiz.git
cd DoWhiz
cp .env.example DoWhiz_service/.env
# Fill in secrets inside DoWhiz_service/.env
```

4. Configure gateway targets:
```bash
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
cat > DoWhiz_service/gateway.toml <<'EOF'
[targets]
little_bear = "http://127.0.0.1:9001"
EOF
```

5. Start services (tmux):
```bash
tmux new-session -d -s oliver "bash -lc 'cd ~/DoWhiz/DoWhiz_service && set -a && source .env && set +a && EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 RUN_TASK_DOCKER_IMAGE= cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001'"
tmux new-session -d -s gateway "bash -lc 'cd ~/DoWhiz/DoWhiz_service && set -a && source .env && set +a && ./scripts/run_gateway_local.sh'"
ngrok config add-authtoken "$NGROK_AUTHTOKEN"
tmux new-session -d -s ngrok "ngrok http 9100 --url https://oliver.dowhiz.prod.ngrok.app"
```

6. Point Postmark to the gateway:
```bash
cd ~/DoWhiz/DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://oliver.dowhiz.prod.ngrok.app/postmark/inbound
```

7. Run E2E from your local machine (recommended if VM blocks SMTP 25):
```
POSTMARK_INBOUND_HOOK_URL=https://oliver.dowhiz.prod.ngrok.app/postmark/inbound
POSTMARK_TEST_FROM=mini-mouse@deep-tutor.com
POSTMARK_TEST_SERVICE_ADDRESS=oliver@dowhiz.com
```
See `DoWhiz_service/README.md` for the full E2E driver script.

## Architecture

```
Inbound message -> Scheduler -> Task runner -> Tools -> Outbound message
```

**Core capabilities:**
- Any-channel task intake and replies (email, Discord, Slack, Telegram, WhatsApp, iMessage)
- Role-based agents with isolated, user-specific memory and data
- Scheduling and orchestration for long-running or recurring work
- Tool-backed execution for reliable outputs

## Repository Layout

| Directory | Description |
|-----------|-------------|
| `DoWhiz_service/` | Rust backend service (scheduler, email handling, task execution) |
| `website/` | React frontend (Vite + React 19) |
| `DoWhiz_service/skills/` | 20+ agent skills (playwright-cli, pdf, docx, pptx, canvas-design, etc.) |
| `DoWhiz_service/employees/` | Employee persona configs |
| `external/openclaw/` | Reference implementation for multi-agent patterns |

## Documentation

- **[DoWhiz Service - Full Documentation](DoWhiz_service/README.md)** - Detailed setup, configuration, environment variables, Docker, E2E testing
- **[Website](website/README.md)** - Frontend development
- **[Contributing](CONTRIBUTING.md)** - Development workflow and guidelines
- **[Vision](vision.md)** - Long-term product direction
- **[Developer Docs](https://docs.google.com/document/d/1MRU00FTJIlCJno2yj9jrlnXNPq1TJ34B5jldg687fSg/edit?tab=t.0)** - Internal documentation and task board

## License

See [LICENSE](LICENSE).
