# DoWhiz

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

A lightweight Rust replica of OpenClaw with better security, accessibility, and token usage. Serve as your digital employee team, message us any task over email, Discord, Slack, Telegram, WhatsApp, iMessage, or any other channel.

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
