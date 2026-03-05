#!/usr/bin/env bash
# Link-only auth test flow (keeps linked identifier for manual inspection).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
ENV_FILE="${ENV_FILE:-$PROJECT_DIR/.env}"

load_env_value() {
    local key="$1"
    local file="$2"
    awk -F'=' -v key="$key" '
        $0 ~ "^[[:space:]]*"key"=" {
            sub(/^[^=]*=/, "", $0);
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", $0);
            gsub(/^"|"$/, "", $0);
            print $0;
            exit;
        }
    ' "$file"
}

json_get() {
    local path="$1"
    python3 -c '
import json, sys
path = sys.argv[1]
try:
    data = json.load(sys.stdin)
except Exception:
    sys.exit(1)
cur = data
for part in path.split("."):
    if isinstance(cur, dict) and part in cur:
        cur = cur[part]
    else:
        sys.exit(1)
if cur is None:
    sys.exit(1)
if isinstance(cur, (dict, list)):
    print(json.dumps(cur))
else:
    print(cur)
' "$path"
}

curl_json() {
    local method="$1"
    local url="$2"
    local data="${3:-}"
    shift 3
    local extra_headers=("$@")
    local response
    if [[ -n "$data" ]]; then
        response="$(curl -sS -X "$method" "$url" \
            -H "Content-Type: application/json" \
            "${extra_headers[@]}" \
            -d "$data" \
            -w $'\n%{http_code}')"
    else
        response="$(curl -sS -X "$method" "$url" \
            "${extra_headers[@]}" \
            -w $'\n%{http_code}')"
    fi
    HTTP_STATUS="${response##*$'\n'}"
    HTTP_BODY="${response%$'\n'*}"
}

ensure_2xx() {
    local step="$1"
    if [[ "${HTTP_STATUS:0:1}" != "2" ]]; then
        echo "ERROR: ${step} failed (status=${HTTP_STATUS})"
        echo "Body: ${HTTP_BODY}"
        exit 1
    fi
}

if [[ -f "$ENV_FILE" ]]; then
    SUPABASE_ANON_KEY="${SUPABASE_ANON_KEY:-$(load_env_value SUPABASE_ANON_KEY "$ENV_FILE")}"
    SUPABASE_PROJECT_URL="${SUPABASE_PROJECT_URL:-$(load_env_value SUPABASE_PROJECT_URL "$ENV_FILE")}"
fi

SERVICE_URL="${SERVICE_URL:-http://localhost:9001}"
SUPABASE_URL="${SUPABASE_PROJECT_URL:-}"
ANON_KEY="${SUPABASE_ANON_KEY:-}"
TEST_EMAIL="${1:-${TEST_EMAIL:-}}"
TEST_PASSWORD="${2:-${TEST_PASSWORD:-}}"
TEST_PHONE="${3:-${TEST_PHONE:-+14155551234}}"
AUTH_VERIFY_CODE="${AUTH_VERIFY_CODE:-123456}"

if [[ -z "$TEST_EMAIL" || -z "$TEST_PASSWORD" ]]; then
    echo "Usage: $0 <test_email> <test_password> [test_phone]"
    echo "Or set TEST_EMAIL/TEST_PASSWORD env vars."
    exit 2
fi

if [[ -z "$SUPABASE_URL" || -z "$ANON_KEY" ]]; then
    echo "ERROR: SUPABASE_PROJECT_URL and SUPABASE_ANON_KEY are required."
    echo "Set env vars directly or provide them in $ENV_FILE."
    exit 1
fi

echo "=== Auth API Link Test (No Unlink) ==="
echo "Service URL: $SERVICE_URL"
echo "Supabase URL: $SUPABASE_URL"
echo "Test email: $TEST_EMAIL"
echo "Test phone: $TEST_PHONE"
echo ""

echo "1. Login with Supabase..."
curl_json "POST" "${SUPABASE_URL}/auth/v1/token?grant_type=password" \
    "{\"email\":\"${TEST_EMAIL}\",\"password\":\"${TEST_PASSWORD}\"}" \
    -H "apikey: ${ANON_KEY}"
ensure_2xx "Supabase login"

ACCESS_TOKEN="$(printf '%s' "$HTTP_BODY" | json_get "access_token" 2>/dev/null || true)"
if [[ -z "$ACCESS_TOKEN" ]]; then
    echo "ERROR: missing access_token in Supabase login response."
    echo "Body: ${HTTP_BODY}"
    exit 1
fi
echo "Access token obtained: ${ACCESS_TOKEN:0:20}..."
echo ""

echo "2. POST /auth/signup"
curl_json "POST" "${SERVICE_URL}/auth/signup" "" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}"
ensure_2xx "/auth/signup"
echo "status=${HTTP_STATUS}"
echo "body=${HTTP_BODY}"
echo ""

echo "3. POST /auth/link"
curl_json "POST" "${SERVICE_URL}/auth/link" \
    "{\"identifier_type\":\"phone\",\"identifier\":\"${TEST_PHONE}\"}" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}"
ensure_2xx "/auth/link"
echo "status=${HTTP_STATUS}"
echo "body=${HTTP_BODY}"
echo ""

echo "4. POST /auth/verify"
curl_json "POST" "${SERVICE_URL}/auth/verify" \
    "{\"identifier_type\":\"phone\",\"identifier\":\"${TEST_PHONE}\",\"code\":\"${AUTH_VERIFY_CODE}\"}" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}"
ensure_2xx "/auth/verify"
echo "status=${HTTP_STATUS}"
echo "body=${HTTP_BODY}"
echo ""

echo "5. GET /auth/account"
curl_json "GET" "${SERVICE_URL}/auth/account" "" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}"
ensure_2xx "/auth/account"
echo "status=${HTTP_STATUS}"
echo "body=${HTTP_BODY}"
echo ""

echo "=== Test Complete: PASS ==="
echo ""
echo "Manual verification:"
echo "1. Check accounts table for this user"
echo "2. Check account_identifiers for linked phone: ${TEST_PHONE}"
