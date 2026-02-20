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

A lightweight Rust replica of OpenClaw🦞 with **better security, accessibility, and token usage**. Serve as your digital employee team, message us any task over email, Slack, Discord, SMS (Twilio), Telegram, WhatsApp, Google Docs comments, iMessage (BlueBubbles), or any other channel. 🧸🐭🐙🐘👾🦞🐦🐉

## Quick Start

### 1. Prerequisites
- Rust toolchain (via `rustup`)
- Node.js 20 + npm
- `ngrok`
- Docker (optional; for containerized workers)
- `python3` (used by scripts to discover ngrok URLs)

For full dependency install steps, see `DoWhiz_service/README.md`.

### 2. Configure Environment

```bash
cp .env.example DoWhiz_service/.env
# Edit DoWhiz_service/.env and add your POSTMARK_SERVER_TOKEN
# For Codex/Claude runners, also set AZURE_OPENAI_API_KEY_BACKUP
```

### 3. Start Service

Set a shared ingestion queue database URL (add to `DoWhiz_service/.env` or export in each terminal before starting gateway/workers):

```bash
export SUPABASE_DB_URL="postgresql://..."
# or
export INGESTION_DB_URL="postgresql://..."
```
Also set Supabase Storage credentials for raw payload blobs:
```bash
export SUPABASE_PROJECT_URL="https://<project>.supabase.co"
export SUPABASE_SECRET_KEY="sb_secret_..."
export SUPABASE_STORAGE_BUCKET="ingestion-raw"
```
If your Supabase DB hostname resolves to IPv6-only, ensure the VM has IPv6 outbound enabled (see VM deployment notes in `DoWhiz_service/README.md`). For Supabase DB TLS, you may also need:
```bash
export INGESTION_QUEUE_TLS_ALLOW_INVALID_CERTS=true
```
For VM/pm2 deployments, prefer writing these into `DoWhiz_service/.env` so they survive restarts.

Run a gateway + worker (local, single employee):

```bash
# Terminal 1: worker
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok

# Terminal 2: gateway
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
# Edit gateway.toml routes to map your service address to little_bear
# Ensure SUPABASE_DB_URL (or INGESTION_DB_URL) is set in this terminal
./DoWhiz_service/scripts/run_gateway_local.sh

# Terminal 3: expose gateway + set Postmark hook
ngrok http 9100
cd DoWhiz_service
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-DOMAIN.ngrok.app/postmark/inbound
```

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
Inbound message -> Ingestion Gateway -> Ingestion Queue -> Scheduler -> Task runner -> Tools -> Outbound message
```

**Core capabilities:**
- Any-channel task intake and replies (email, Slack, Discord, SMS/Twilio, Telegram, WhatsApp, Google Docs comments, iMessage/BlueBubbles)
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
