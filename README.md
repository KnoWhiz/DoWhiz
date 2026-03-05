# DoWhiz - OpenClaw🦞, but scalable, accessible, and safe.

<p align="center">
  <img src="website/public/assets/DoWhiz.jpeg" alt="Do icon" width="96" />
</p>

<p align="center"><strong>Product Shorts</strong></p>

<table align="center">
  <tr>
    <td align="center">
      <a href="https://www.youtube.com/shorts/SI9mxW_Top0">
        <img src="assets/readme-shorts/short-1.gif" alt="DoWhiz short 1 preview" width="170" />
      </a>
    </td>
    <td align="center">
      <a href="https://www.youtube.com/shorts/PSsJ7WBk71w">
        <img src="assets/readme-shorts/short-2.gif" alt="DoWhiz short 2 preview" width="170" />
      </a>
    </td>
    <td align="center">
      <a href="https://www.youtube.com/shorts/5H9g3LOGkMc">
        <img src="assets/readme-shorts/short-3.gif" alt="DoWhiz short 3 preview" width="170" />
      </a>
    </td>
  </tr>
  <tr>
    <td align="center"><sub><a href="https://www.youtube.com/shorts/SI9mxW_Top0">DoWhiz employee in Notion</a></sub></td>
    <td align="center"><sub><a href="https://www.youtube.com/shorts/PSsJ7WBk71w">DoWhiz employee in Email</a></sub></td>
    <td align="center"><sub><a href="https://www.youtube.com/shorts/5H9g3LOGkMc">DoWhiz employee in Discord</a></sub></td>
  </tr>
</table>

<p align="center"><sub>Tap any preview to watch the full Shorts video.</sub></p>

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

A lightweight Rust replica of OpenClaw🦞 with **better security, accessibility, and token usage**. Serve as your digital employee team, message us any task over email, Slack, Discord, SMS (Twilio), Telegram, WhatsApp, Google Docs/Sheets/Slides comments, iMessage (BlueBubbles), or any other channel. 🧸🐭🐙🐘👾🦞🐦🐉

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
# Edit DoWhiz_service/.env and set required keys:
# - POSTMARK_SERVER_TOKEN
# - AZURE_OPENAI_API_KEY_BACKUP
# - MONGODB_URI
# - SUPABASE_DB_URL
```

### 3. Start Service

Configure Service Bus + Azure Blob (add to `DoWhiz_service/.env` or export in each terminal before starting gateway/workers):

```bash
export INGESTION_QUEUE_BACKEND=servicebus
export SERVICE_BUS_CONNECTION_STRING="Endpoint=sb://..."
export SERVICE_BUS_QUEUE_NAME="ingestion"
export RAW_PAYLOAD_STORAGE_BACKEND=azure
export AZURE_STORAGE_CONTAINER_INGEST="ingestion-raw"
export AZURE_STORAGE_SAS_TOKEN="..."
```
For VM/pm2 deployments, prefer writing these into `DoWhiz_service/.env` so they survive restarts.

Run a gateway + worker (local, single employee):

```bash
# Terminal 1: worker
./DoWhiz_service/scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok

# Terminal 2: gateway
cp DoWhiz_service/gateway.example.toml DoWhiz_service/gateway.toml
# Edit gateway.toml routes to map your service address to little_bear
# Ensure Service Bus + Azure Blob env vars are set in this terminal
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

## Single-VM Deployment (Gateway + Worker)

For a full single-VM deployment (no Docker) that runs the inbound gateway + an Oliver worker end-to-end, follow the **VM Deployment (Gateway + ngrok)** section in `DoWhiz_service/README.md`. Summary:

1. Build release binaries (`inbound_gateway`, `rust_service`) on the VM.
2. Start gateway + worker under `pm2`/`systemd` (recommended for long-running services).
3. Expose the gateway (ngrok or Nginx) and set Postmark’s inbound hook to `https://<public>/postmark/inbound`.
4. Ensure `.env` includes Service Bus + Azure Blob settings, plus `EMPLOYEE_ID=little_bear` for GitHub auth when creating PRs.
5. Run the live E2E email test.

For single `.env` staging/prod split (`DEPLOY_TARGET` + `STAGING_` keys), plus VM runbooks and rollback:
- `DoWhiz_service/docs/staging_production_deploy.md`
- Production deploy branch: `main`
- Staging CI target branch: `dev` (planned rollout)

## Azure Deployment (Production)

For Azure-managed ingress, queues, and storage, follow `DoWhiz_service/README.md` under **Azure Deployment (Rust Gateway + Service Bus + Blob + Workers)**. This flow uses the Rust inbound gateway for **all** ingress (including email), Azure Service Bus for ingestion queues, Azure Blob for raw payloads, and worker services running on Azure VMs or containers.

## Architecture

```
Inbound message -> Ingress (Rust gateway) -> Raw payload storage (Azure Blob; Supabase legacy) -> Ingestion Queue (Service Bus for gateway; Postgres optional) -> Scheduler -> Task runner -> Tools -> Outbound message
```

**Core capabilities:**
- Any-channel task intake and replies (email, Slack, Discord, SMS/Twilio, Telegram, WhatsApp, Google Docs/Sheets/Slides comments, iMessage/BlueBubbles)
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

## DoWhiz Account [Slack and Discord]
- Account/auth APIs run in both `rust_service` and `inbound_gateway` (`/auth/*`).
- Billing APIs (`/billing/*`) are exposed by `rust_service` when Stripe env is configured.
- DoWhiz accounts use Supabase access-token validation (local JWT verification with `SUPABASE_JWT_SECRET`, with Supabase API fallback).
- Account, identifier, email verification, and payment records are stored in Supabase Postgres (`SUPABASE_DB_URL`).
- Support for Slack and Discord OAuth for DoWhiz accounts in Integration panel
- Simultaneous Slack support for `boiled_egg` (Proto) and `little_bear` (Oliver)

## Documentation

- **[DoWhiz Service - Full Documentation](DoWhiz_service/README.md)** - Detailed setup, configuration, environment variables, Docker, E2E testing
- **[Website](website/README.md)** - Frontend development
- **[Contributing](CONTRIBUTING.md)** - Development workflow and guidelines
- **[Vision](reference_documentation/vision.md)** - Long-term product direction
- **[Developer Docs](https://docs.google.com/document/d/1MRU00FTJIlCJno2yj9jrlnXNPq1TJ34B5jldg687fSg/edit?tab=t.0)** - Internal documentation and task board

## License

See [LICENSE](LICENSE).
