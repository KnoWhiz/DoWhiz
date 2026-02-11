# Contributing to DoWhiz

Thanks for your interest in contributing. This guide focuses on local setup, testing, and the fastest paths to verify changes.

**Quick Start**
1. Install prerequisites for Rust + Node (see **Prerequisites**).
2. Create a local `.env` based on `.env.example` and fill required values.
3. Start the Rust service:
```
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```
4. Run tests:
```
cargo test -p scheduler_module
cargo test -p run_task_module
```
5. Start the website:
```
cd website
npm run dev
```

**Prerequisites**
- Rust toolchain (stable).
- Node.js 20+ and npm.
- System libraries (Linux): `libsqlite3`, `libssl`, `pkg-config`, `ca-certificates`.
- System libraries (macOS): Homebrew `sqlite` + `openssl@3` + `pkg-config`.
- Optional (agent tooling): `codex`, `claude`, `playwright-cli` with Chromium.

Linux (Debian/Ubuntu):
```
sudo apt-get update
sudo apt-get install -y ca-certificates libsqlite3-dev libssl-dev pkg-config curl
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
sudo npx playwright install --with-deps chromium
```

macOS (Homebrew):
```
brew install node@20 openssl@3 sqlite pkg-config
npm install -g @openai/codex@latest @anthropic-ai/claude-code@latest @playwright/cli@latest
npx playwright install chromium
```

**Environment Setup**
The service loads environment variables from the current working directory. Recommended workflow:
1. Use the repo root as your working directory.
2. Create a local `.env` based on `.env.example` and set at least:
```
AZURE_OPENAI_API_KEY_BACKUP=
AZURE_OPENAI_ENDPOINT_BACKUP=
POSTMARK_SERVER_TOKEN=
```
3. Optional toggles include `CODEX_DISABLED=1` (run without Codex) and `RUN_TASK_DOCKER_IMAGE=dowhiz-service` (run each task in Docker).

**Run the Rust Service**
One-command local run (auto ngrok + Postmark hook):
```
./DoWhiz_service/scripts/run_employee.sh little_bear 9001
```
Requires `POSTMARK_SERVER_TOKEN` in your `.env`, plus `ngrok` and `python3` installed. Use `--public-url`, `--skip-hook`, or `--skip-ngrok` for advanced flows.

Single employee (from repo root):
```
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

Multi-employee (run two terminals):
```
# Oliver / Little-Bear (Codex)
EMPLOYEE_ID=little_bear RUST_SERVICE_PORT=9001 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001

# Maggie / Mini-Mouse (Claude)
EMPLOYEE_ID=mini_mouse RUST_SERVICE_PORT=9002 \
  cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9002
```

**Real Email Flow (Postmark + ngrok)**
1. Start the service.
2. Expose the port with ngrok:
```
ngrok http 9001
```
3. Update the Postmark inbound hook:
```
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound
```
4. Send an email to the employee address configured in `DoWhiz_service/employee.toml`.

**Tests**
Rust unit tests:
```
cargo test -p scheduler_module
cargo test -p run_task_module
```

Live email tests (require Postmark):
```
POSTMARK_LIVE_TEST=1 POSTMARK_SERVER_TOKEN=... cargo test -p scheduler_module --test service_real_email -- --nocapture
```

**Website**
```
cd website
npm install
npm run dev
```

**Code Style**
- Rust: run `cargo fmt` before PRs.
- Website: run `npm run lint` from `website/`.

**Pull Requests**
- Include a summary, how to test, and any env/config changes.
- Add screenshots for UI changes.
