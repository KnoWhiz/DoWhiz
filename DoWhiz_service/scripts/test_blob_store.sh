#!/usr/bin/env bash
# Test script for Azure Blob Storage memo.md operations.

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

require_cmd() {
    local name="$1"
    if ! command -v "$name" >/dev/null 2>&1; then
        echo "ERROR: required command not found: $name"
        exit 1
    fi
}

require_cmd az

if [[ -f "$ENV_FILE" ]]; then
    AZURE_STORAGE_CONNECTION_STRING="${AZURE_STORAGE_CONNECTION_STRING:-$(load_env_value AZURE_STORAGE_CONNECTION_STRING "$ENV_FILE")}"
    AZURE_STORAGE_CONTAINER="${AZURE_STORAGE_CONTAINER:-$(load_env_value AZURE_STORAGE_CONTAINER "$ENV_FILE")}"
fi

AZURE_STORAGE_CONNECTION_STRING="${AZURE_STORAGE_CONNECTION_STRING:-}"
CONTAINER="${AZURE_STORAGE_CONTAINER:-memos}"

if [[ -z "$AZURE_STORAGE_CONNECTION_STRING" ]]; then
    echo "ERROR: AZURE_STORAGE_CONNECTION_STRING not set (env or $ENV_FILE)."
    exit 1
fi

ACCOUNT_NAME="$(printf '%s' "$AZURE_STORAGE_CONNECTION_STRING" | awk -F';' '
    {
        for (i = 1; i <= NF; i++) {
            if ($i ~ /^AccountName=/) {
                sub(/^AccountName=/, "", $i);
                print $i;
                exit;
            }
        }
    }'
)"

if [[ -z "$ACCOUNT_NAME" ]]; then
    echo "ERROR: failed to parse AccountName from AZURE_STORAGE_CONNECTION_STRING."
    exit 1
fi

TEST_ACCOUNT_ID="${TEST_ACCOUNT_ID:-12345678-1234-1234-1234-123456789abc}"
BLOB_PATH="${TEST_ACCOUNT_ID}/memo.md"
TMP_DIR="$(mktemp -d)"
UPLOAD_FILE="${TMP_DIR}/memo_upload.md"
DOWNLOAD_FILE="${TMP_DIR}/memo_download.md"

cleanup() {
    az storage blob delete \
        --account-name "$ACCOUNT_NAME" \
        --container-name "$CONTAINER" \
        --name "$BLOB_PATH" \
        --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
        --only-show-errors >/dev/null 2>&1 || true
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

cat > "$UPLOAD_FILE" <<'EOF'
# Test Memo

## Contacts
- Test User: 555-1234

## Preferences
- Theme: dark
EOF

echo "=== Blob Store Test ==="
echo "Account: ${ACCOUNT_NAME}"
echo "Container: ${CONTAINER}"
echo "Blob path: ${BLOB_PATH}"
echo ""

echo "1. Upload test memo..."
az storage blob upload \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --name "$BLOB_PATH" \
    --file "$UPLOAD_FILE" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --overwrite \
    --only-show-errors
echo "Upload successful."
echo ""

echo "2. Download memo and verify roundtrip..."
az storage blob download \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --name "$BLOB_PATH" \
    --file "$DOWNLOAD_FILE" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --overwrite \
    --only-show-errors

if ! cmp -s "$UPLOAD_FILE" "$DOWNLOAD_FILE"; then
    echo "ERROR: downloaded blob content does not match uploaded content."
    echo "--- uploaded ---"
    cat "$UPLOAD_FILE"
    echo "--- downloaded ---"
    cat "$DOWNLOAD_FILE"
    exit 1
fi
echo "Roundtrip content verified."
echo ""

echo "3. List matching blobs..."
az storage blob list \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --prefix "$TEST_ACCOUNT_ID/" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --query "[].name" \
    -o table \
    --only-show-errors
echo ""

echo "=== Test Complete: PASS ==="
