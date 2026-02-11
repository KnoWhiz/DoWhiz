#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Run the live Rust email service E2E test with ngrok.

Usage:
  scripts/run_e2e.sh [--port <port>] [--public-url <url>] [--skip-ngrok]

Options:
  --port, -p       Port to bind (default: 9001)
  --public-url     Public base URL (or full /postmark/inbound). Skips ngrok.
  --skip-ngrok     Do not start ngrok (requires --public-url).
  --help, -h       Show this help.

Examples:
  scripts/run_e2e.sh
  scripts/run_e2e.sh --port 9002
  scripts/run_e2e.sh --public-url https://example.com
USAGE
}

port=""
public_url=""
skip_ngrok="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --port|-p)
      port="${2:-}"
      shift 2
      ;;
    --public-url)
      public_url="${2:-}"
      shift 2
      ;;
    --skip-ngrok)
      skip_ngrok="true"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
service_root="$(cd "${script_dir}/.." && pwd)"
repo_root="$(cd "${service_root}/.." && pwd)"

# Load env
if [[ -f "${service_root}/.env" ]]; then
  set -a
  source "${service_root}/.env"
  set +a
elif [[ -f "${repo_root}/.env" ]]; then
  set -a
  source "${repo_root}/.env"
  set +a
fi

if [[ -z "$port" ]]; then
  port="${RUST_SERVICE_PORT:-9001}"
fi

if [[ -z "${POSTMARK_SERVER_TOKEN:-}" ]]; then
  echo "POSTMARK_SERVER_TOKEN is required for live E2E tests." >&2
  exit 1
fi

if [[ "${RUN_CODEX_E2E:-0}" == "1" ]]; then
  if [[ -z "${AZURE_OPENAI_API_KEY_BACKUP:-}" ]] || [[ -z "${AZURE_OPENAI_ENDPOINT_BACKUP:-}" ]]; then
    echo "AZURE_OPENAI_API_KEY_BACKUP and AZURE_OPENAI_ENDPOINT_BACKUP are required when RUN_CODEX_E2E=1." >&2
    exit 1
  fi
fi

if [[ -z "${RUN_TASK_DOCKERFILE:-}" ]] && [[ -f "${repo_root}/Dockerfile" ]]; then
  export RUN_TASK_DOCKERFILE="${repo_root}/Dockerfile"
fi

ngrok_pid=""
started_ngrok="false"

cleanup() {
  if [[ "$started_ngrok" == "true" ]] && [[ -n "$ngrok_pid" ]] && kill -0 "$ngrok_pid" 2>/dev/null; then
    kill "$ngrok_pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

wait_for_ngrok_url() {
  local timeout_secs="$1"
  local start
  start="$(date +%s)"
  while true; do
    local now
    now="$(date +%s)"
    if (( now - start > timeout_secs )); then
      return 1
    fi
    local url
    url="$(python3 - <<'PY'
import json
import urllib.request
try:
    with urllib.request.urlopen("http://127.0.0.1:4040/api/tunnels", timeout=2) as resp:
        data = json.load(resp)
except Exception:
    raise SystemExit(1)

urls = []
for tunnel in data.get("tunnels", []):
    public_url = tunnel.get("public_url")
    if public_url:
        urls.append(public_url)

preferred = None
for url in urls:
    if url.startswith("https://"):
        preferred = url
        break
if not preferred and urls:
    preferred = urls[0]

if not preferred:
    raise SystemExit(1)
print(preferred)
PY
)" || url=""

    if [[ -n "$url" ]]; then
      echo "$url"
      return 0
    fi
    sleep 1
  done
}

cd "$service_root"

hook_base_url=""
if [[ -n "$public_url" ]]; then
  hook_base_url="$public_url"
  skip_ngrok="true"
fi

if [[ "$skip_ngrok" != "true" ]]; then
  if ! command -v ngrok >/dev/null 2>&1; then
    echo "ngrok not found. Install ngrok or pass --public-url to skip it." >&2
    exit 1
  fi
  echo "Starting ngrok on port ${port}..."
  ngrok http "$port" --log=stdout >/tmp/dowhiz-ngrok-e2e.log 2>&1 &
  ngrok_pid=$!
  started_ngrok="true"

  echo "Waiting for ngrok public URL..."
  if ! hook_base_url="$(wait_for_ngrok_url 30)"; then
    echo "Failed to obtain ngrok public URL from http://127.0.0.1:4040." >&2
    exit 1
  fi
fi

if [[ -z "$hook_base_url" ]]; then
  echo "No public URL available. Pass --public-url or remove --skip-ngrok." >&2
  exit 1
fi

if [[ "$hook_base_url" != */postmark/inbound ]]; then
  hook_base_url="${hook_base_url%/}/postmark/inbound"
fi

export POSTMARK_INBOUND_HOOK_URL="$hook_base_url"
export RUST_SERVICE_LIVE_TEST=1
export RUST_SERVICE_PORT="$port"

cargo test -p scheduler_module --test service_real_email -- --nocapture
