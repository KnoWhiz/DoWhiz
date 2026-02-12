# DoWhiz - Lightweight Rust replica of OpenClaw

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
  <img alt="Any channel" src="https://img.shields.io/badge/Any-Channel-14b8a6?style=for-the-badge" />
</p>

DoWhiz is a lightweight Rust replica of OpenClaw that serves as your digital
employee team. Message us any task over Telegram, WhatsApp, iMessage, email,
Discord, Slack, or any other channel, and get structured work back in the same
place.

## Overview
DoWhiz is a lightweight Rust replica of OpenClaw that lets users send tasks to
role-based agents over any message type (Telegram, WhatsApp, iMessage, email,
Discord, Slack, and more). The platform routes, schedules, executes, and
replies in-channel with results, follow-ups, and scheduled work.

## Core capabilities
- Any-channel task intake and replies across email, chat, and messaging apps.
- Role-based agents with isolated, user-specific memory and data.
- Scheduling and orchestration for long-running or recurring work.
- Tool-backed execution for reliable outputs.

## High-level architecture
```
Inbound message -> Scheduler -> Task runner -> Tools -> Outbound message
```

## Vision
The long-term product direction lives in `vision.md`: a multi-agent,
multi-tenant platform where each user has isolated memory/data, agents act
like teammates, and the system escalates when human input is needed.

## Repository layout
- `DoWhiz_service/`: Rust service for inbound email, scheduling, task execution,
  and outbound replies.
- `website/`: Product website.
- `landing/`: Landing page experiments/assets.
- `api_reference_documentation/`: Postmark/Gmail API references (reference-only).
- `example_files/`: Sample inputs/outputs used for testing and demos.
- `external/`: Vendored references (including OpenClaw).
- `scripts/`: Helper scripts.

## Local dependencies (Dockerfile parity for agents)
The Docker image installs extra tooling so Codex can run the `playwright-cli`
skill. For the same capabilities locally, install the Dockerfile deps and set
the skills source dir.

Linux (Debian/Ubuntu, matches Dockerfile):
```
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

Optional (match Dockerfile's chrome-channel lookup used by E2E):
```
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.cache/ms-playwright"
chromium_path="$(ls -d "$PLAYWRIGHT_BROWSERS_PATH"/chromium-*/chrome-linux/chrome | head -n1)"
sudo mkdir -p /opt/google/chrome
sudo ln -sf "$chromium_path" /opt/google/chrome/chrome
```

macOS (Homebrew):
```
brew install node@20 openssl@3 sqlite pkg-config
npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
npx playwright install chromium
```

Skills are copied from `DoWhiz_service/skills` automatically when preparing workspaces.
Postmark outbound requires each employee address to be verified as a Sender Signature (or domain) because replies are sent from the inbound mailbox.

## Getting started
Local config lives in a repo-root `.env` file. Use `.env.example` as a template.

Rust service:
```
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```
Employee profiles (addresses, runner, persona, skills) are defined in `DoWhiz_service/employee.toml`. Each server only processes emails addressed to its configured employee.
Replies are sent from the employee address that the inbound email targeted. For forwarded mail, the service checks `To`/`Cc`/`Bcc` plus headers like `X-Original-To`, `Delivered-To`, and `X-Forwarded-To` to determine which employee address was targeted.
If `RUN_TASK_DOCKER_IMAGE` is set in your `.env`, each task runs
inside a fresh Docker container and the image auto-builds on first use (unless
disabled with `RUN_TASK_DOCKER_AUTO_BUILD=0`).

One-command local run (auto ngrok + Postmark hook):
```
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
```
Requires `POSTMARK_SERVER_TOKEN` in your `.env`, plus `ngrok` and `python3` installed. Use `--public-url`, `--skip-hook`, or `--skip-ngrok` for advanced flows.

Multi-employee local run (from repo root):
```
# Oliver / Little-Bear (Codex)
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# Maggie / Mini-Mouse (Claude)
EMPLOYEE_ID=mini_mouse RUST_SERVICE_PORT=9002 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9002
```

Docker (production image):
```
docker build -t dowhiz-service .
docker run --rm -p 9001:9001 \
  -v "$PWD/.env:/app/.env:ro" \
  -v dowhiz-workspace:/app/.workspace \
  dowhiz-service
```

Docker E2E (Codex + playwright-cli):
```
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

Website:
```
cd website
npm run dev
```

More detail:
- `DoWhiz_service/README.md`
- `website/README.md`
- `CONTRIBUTING.md`

## Contributing
See `CONTRIBUTING.md` for environment setup, tests, and local development workflow.

## Testing
Rust unit tests:
```
cargo test -p scheduler_module
cargo test -p run_task_module
```

Live Postmark E2E:
See `DoWhiz_service/README.md` for the full Docker + local live E2E workflow, including ngrok port mapping for `mini_mouse` (9002) and `little_bear` (9001).

Website lint:
```
cd website
npm run lint
```

## License
See `LICENSE`.
