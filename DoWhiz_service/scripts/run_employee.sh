#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Run the Rust email service for a single employee with optional ngrok + Postmark hook.

Usage:
  scripts/run_employee.sh <employee_id> [port]
  scripts/run_employee.sh --employee <id> --port <port> [--public-url <url>] [--skip-hook] [--skip-ngrok] [--host <host>]

Options:
  --employee, -e   Employee id from employee.toml (default: use service default)
  --port, -p       Port to bind (default: 9001)
  --host           Host to bind (default: 0.0.0.0)
  --public-url     Public base URL (or full /postmark/inbound). Skips ngrok.
  --skip-hook      Do not update Postmark inbound hook.
  --skip-ngrok     Do not start ngrok (requires --public-url or --skip-hook).
  --help, -h       Show this help.

Examples:
  scripts/run_employee.sh little_bear 9001
  scripts/run_employee.sh --employee mini_mouse --port 9002
  scripts/run_employee.sh --employee little_bear --public-url https://example.com
USAGE
}

employee_id=""
port=""
host="0.0.0.0"
public_url=""
skip_hook="false"
skip_ngrok="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --employee|-e)
      employee_id="${2:-}"
      shift 2
      ;;
    --port|-p)
      port="${2:-}"
      shift 2
      ;;
    --host)
      host="${2:-}"
      shift 2
      ;;
    --public-url)
      public_url="${2:-}"
      shift 2
      ;;
    --skip-hook)
      skip_hook="true"
      shift
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
      if [[ -z "$employee_id" ]]; then
        employee_id="$1"
        shift
      elif [[ -z "$port" ]]; then
        port="$1"
        shift
      else
        echo "Unknown argument: $1" >&2
        usage
        exit 1
      fi
      ;;
  esac
done

if [[ -z "$port" ]]; then
  port="9001"
fi

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
service_root="$(cd "${script_dir}/.." && pwd)"

ngrok_pid=""
service_pid=""

cleanup() {
  if [[ -n "$service_pid" ]] && kill -0 "$service_pid" 2>/dev/null; then
    kill "$service_pid" 2>/dev/null || true
  fi
  if [[ -n "$ngrok_pid" ]] && kill -0 "$ngrok_pid" 2>/dev/null; then
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
  if ! command -v python3 >/dev/null 2>&1; then
    echo "python3 not found. Install python3 or pass --public-url to skip ngrok." >&2
    exit 1
  fi

  echo "Starting ngrok on port ${port}..."
  ngrok http "$port" --log=stdout >/tmp/dowhiz-ngrok.log 2>&1 &
  ngrok_pid=$!

  echo "Waiting for ngrok public URL..."
  if ! hook_base_url="$(wait_for_ngrok_url 20)"; then
    echo "Failed to obtain ngrok public URL from http://127.0.0.1:4040." >&2
    exit 1
  fi
fi

if [[ -n "$hook_base_url" ]]; then
  hook_base_url="${hook_base_url%/}"
  if [[ "$hook_base_url" != */postmark/inbound ]]; then
    hook_base_url="${hook_base_url}/postmark/inbound"
  fi

  if [[ "$skip_hook" != "true" ]]; then
    echo "Updating Postmark inbound hook to: ${hook_base_url}"
    if [[ -n "$employee_id" ]]; then
      EMPLOYEE_ID="$employee_id" \
        cargo run -p scheduler_module --bin set_postmark_inbound_hook -- --hook-url "$hook_base_url"
    else
      cargo run -p scheduler_module --bin set_postmark_inbound_hook -- --hook-url "$hook_base_url"
    fi
  fi
else
  if [[ "$skip_hook" != "true" ]]; then
    echo "No public URL available for hook. Pass --public-url or remove --skip-ngrok." >&2
    exit 1
  fi
fi

echo "Starting service on ${host}:${port}..."
if [[ -n "$employee_id" ]]; then
  EMPLOYEE_ID="$employee_id" RUST_SERVICE_PORT="$port" \
    cargo run -p scheduler_module --bin rust_service -- --host "$host" --port "$port" &
else
  RUST_SERVICE_PORT="$port" \
    cargo run -p scheduler_module --bin rust_service -- --host "$host" --port "$port" &
fi
service_pid=$!

wait "$service_pid"
