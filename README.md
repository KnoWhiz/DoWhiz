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
cp .env.example .env
# Edit .env and add your POSTMARK_SERVER_TOKEN
```

### 3. Start Service

**Option A: Start All Employees (Docker, recommended)**
```bash
# Terminal 1: Start all services (Ollama + fanout + 4 employees)
./DoWhiz_service/scripts/run_all_employees_docker.sh

# Terminal 2: Expose fanout to the internet
ngrok http 9100

# Terminal 3: Set Postmark webhook to ngrok URL
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound
```

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

## VM Deployment Workflow (Production)

This workflow deploys a single employee per VM with HTTPS and systemd. Repeat these steps on a new VM and subdomain for each employee.

1. Provision an Ubuntu VM and open inbound TCP ports `22`, `80`, `443`.
For live email E2E tests, request outbound TCP `25` from your cloud provider.

2. Create a DNS A record for an API subdomain (example: `api.dowhiz.com`) that points to the VM public IP.

3. Install dependencies on the VM.
```bash
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl git python3
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
curl https://sh.rustup.rs -sSf | sh -s -- -y
sudo npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

4. Clone the repo and configure `.env`.
```bash
git clone https://github.com/KnoWhiz/DoWhiz.git
cd DoWhiz
cp .env.example .env
```
Set at least:
```
EMPLOYEE_ID=little_bear
RUST_SERVICE_HOST=127.0.0.1
RUST_SERVICE_PORT=9001
POSTMARK_INBOUND_HOOK_URL=https://api.dowhiz.com/postmark/inbound
SLACK_REDIRECT_URI=https://api.dowhiz.com/slack/oauth/callback
```

5. Build the service.
```bash
source ~/.cargo/env
cd DoWhiz/DoWhiz_service
cargo build -p scheduler_module --release
```

6. Configure Nginx reverse proxy for `/postmark/inbound`, `/slack/`, and `/health`.
```nginx
server {
    listen 80;
    server_name api.dowhiz.com;

    location /postmark/inbound {
        proxy_pass http://127.0.0.1:9001/postmark/inbound;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /slack/ {
        proxy_pass http://127.0.0.1:9001/slack/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    location /health {
        proxy_pass http://127.0.0.1:9001/health;
        proxy_set_header Host $host;
    }
}
```

7. Enable HTTPS with Let's Encrypt.
```bash
sudo apt-get install -y nginx certbot python3-certbot-nginx
sudo certbot --nginx -d api.dowhiz.com
```

8. Create a systemd service (example).
```ini
[Unit]
Description=DoWhiz Rust Service (Oliver)
After=network.target

[Service]
Type=simple
User=azureuser
Group=azureuser
WorkingDirectory=/home/azureuser/DoWhiz/DoWhiz_service
EnvironmentFile=/home/azureuser/DoWhiz/.env
Environment=PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ExecStart=/home/azureuser/DoWhiz/DoWhiz_service/target/release/rust_service --host 127.0.0.1 --port 9001
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

9. Update the Postmark inbound hook.
```bash
set -a; source /home/azureuser/DoWhiz/.env; set +a
/home/azureuser/DoWhiz/DoWhiz_service/target/release/set_postmark_inbound_hook \
  --hook-url https://api.dowhiz.com/postmark/inbound
```

10. Configure Slack and Discord.
- Slack Event URL: `https://api.dowhiz.com/slack/events`
- Slack OAuth Redirect: `https://api.dowhiz.com/slack/oauth/callback`
- For Discord, set `discord_enabled = true` for the employee in `DoWhiz_service/employee.toml`.

Slack and Discord tokens only support one active connection at a time. Use one VM per employee, or a fanout gateway VM if you want a shared Slack/Discord entry point.

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
