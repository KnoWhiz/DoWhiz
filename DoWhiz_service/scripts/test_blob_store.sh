#!/bin/bash
# Test script for Azure Blob Storage memo.md operations

set -e

cd "$(dirname "$0")/.."

# Load env
if [ -f .env ]; then
    export $(grep -E '^AZURE_STORAGE' .env | xargs)
fi

echo "=== Blob Store Test ==="
echo "Container: ${AZURE_STORAGE_CONTAINER:-memos}"
echo ""

# Test account ID (fake UUID for testing)
TEST_ACCOUNT_ID="12345678-1234-1234-1234-123456789abc"
BLOB_PATH="${TEST_ACCOUNT_ID}/memo.md"

# Check if we have the connection string
if [ -z "$AZURE_STORAGE_CONNECTION_STRING" ]; then
    echo "ERROR: AZURE_STORAGE_CONNECTION_STRING not set"
    exit 1
fi

# Extract account name and key
ACCOUNT_NAME=$(echo "$AZURE_STORAGE_CONNECTION_STRING" | grep -o 'AccountName=[^;]*' | cut -d= -f2)
CONTAINER="${AZURE_STORAGE_CONTAINER:-memos}"

echo "Account: $ACCOUNT_NAME"
echo "Testing blob: $BLOB_PATH"
echo ""

# Test 1: Write a memo
echo "1. Writing test memo..."
TEST_CONTENT="# Test Memo

## Contacts
- Test User: 555-1234

## Preferences
- Theme: dark
"

az storage blob upload \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --name "$BLOB_PATH" \
    --data "$TEST_CONTENT" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --overwrite \
    --only-show-errors

echo "Write successful!"
echo ""

# Test 2: Read the memo back
echo "2. Reading memo back..."
READ_CONTENT=$(az storage blob download \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --name "$BLOB_PATH" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --only-show-errors \
    -o tsv 2>/dev/null || az storage blob download \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --name "$BLOB_PATH" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --file /dev/stdout \
    --only-show-errors 2>/dev/null)

echo "Read content:"
echo "$READ_CONTENT"
echo ""

# Test 3: List blobs
echo "3. Listing blobs in container..."
az storage blob list \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --query "[].name" \
    -o table \
    --only-show-errors

echo ""

# Test 4: Delete the test blob
echo "4. Deleting test blob..."
az storage blob delete \
    --account-name "$ACCOUNT_NAME" \
    --container-name "$CONTAINER" \
    --name "$BLOB_PATH" \
    --connection-string "$AZURE_STORAGE_CONNECTION_STRING" \
    --only-show-errors

echo "Delete successful!"
echo ""

echo "=== Test Complete ==="
