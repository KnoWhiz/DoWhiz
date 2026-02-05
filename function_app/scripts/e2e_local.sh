#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP_DIR="$ROOT_DIR/function_app"
LOG_DIR="$APP_DIR/.e2e"
FUNC_LOG="$LOG_DIR/func.log"
AZURITE_LOG="$LOG_DIR/azurite.log"
FUNC_PID=""
AZURITE_PID=""

cleanup() {
  if [[ -n "$FUNC_PID" ]]; then
    kill "$FUNC_PID" >/dev/null 2>&1 || true
    wait "$FUNC_PID" >/dev/null 2>&1 || true
  fi
  if [[ -n "$AZURITE_PID" ]]; then
    kill "$AZURITE_PID" >/dev/null 2>&1 || true
    wait "$AZURITE_PID" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

mkdir -p "$LOG_DIR"

if ! command -v func >/dev/null 2>&1; then
  echo "Azure Functions Core Tools (func) not found in PATH." >&2
  exit 1
fi

"$APP_DIR/scripts/build_binary.sh"

if [[ ! -f "$APP_DIR/local.settings.json" ]]; then
  cp "$APP_DIR/local.settings.example.json" "$APP_DIR/local.settings.json"
  echo "Created local.settings.json from example. Update secrets if needed." >&2
fi

if grep -q "UseDevelopmentStorage=true" "$APP_DIR/local.settings.json"; then
  if command -v azurite >/dev/null 2>&1; then
    azurite --silent --location "$LOG_DIR/azurite" --debug "$AZURITE_LOG" &
    AZURITE_PID="$!"
  else
    echo "Azurite is required when AzureWebJobsStorage=UseDevelopmentStorage=true." >&2
    exit 1
  fi
fi

pushd "$APP_DIR" >/dev/null
func host start --port 7071 >"$FUNC_LOG" 2>&1 &
FUNC_PID="$!"
popd >/dev/null

READY=""
for _ in $(seq 1 30); do
  if curl -fsS "http://localhost:7071/health" >/dev/null 2>&1; then
    READY="yes"
    break
  fi
  sleep 1
done

if [[ -z "$READY" ]]; then
  echo "Function host did not become ready. See $FUNC_LOG" >&2
  exit 1
fi

echo "Health check OK"

if command -v uuidgen >/dev/null 2>&1; then
  MSG_ID="$(uuidgen)"
else
  MSG_ID="$(date +%s)"
fi

PAYLOAD=$(cat <<JSON
{"From":"agent@dowhiz.com","To":"test@example.com","Subject":"E2E","MessageID":"<$MSG_ID@local>"}
JSON
)

RESP=$(curl -fsS -X POST -H "Content-Type: application/json" \
  -d "$PAYLOAD" http://localhost:7071/postmark/inbound)

echo "Inbound response: $RESP"
if ! echo "$RESP" | grep -Eq '"status"\s*:\s*"(accepted|duplicate)"'; then
  echo "Unexpected inbound response. See $FUNC_LOG" >&2
  exit 1
fi

echo "E2E OK"
