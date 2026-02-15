#!/bin/bash
#
# Diagnostic script to check Google OAuth token scopes and API access.
# This helps diagnose 403 PERMISSION_DENIED errors.
#

set -e

# Load .env if exists
if [ -f ../.env ]; then
    source ../.env
elif [ -f .env ]; then
    source .env
fi

echo "=============================================="
echo "Google OAuth Token Diagnostic"
echo "=============================================="
echo ""

# Check environment variables
echo "1. Checking environment variables..."
if [ -z "$GOOGLE_CLIENT_ID" ]; then
    echo "   ERROR: GOOGLE_CLIENT_ID not set"
    exit 1
fi
echo "   GOOGLE_CLIENT_ID: ${GOOGLE_CLIENT_ID:0:20}..."

if [ -z "$GOOGLE_CLIENT_SECRET" ]; then
    echo "   ERROR: GOOGLE_CLIENT_SECRET not set"
    exit 1
fi
echo "   GOOGLE_CLIENT_SECRET: ${GOOGLE_CLIENT_SECRET:0:10}..."

# Check for employee-specific token first
REFRESH_TOKEN="${GOOGLE_REFRESH_TOKEN_BOILED_EGG:-$GOOGLE_REFRESH_TOKEN}"
if [ -z "$REFRESH_TOKEN" ]; then
    echo "   ERROR: Neither GOOGLE_REFRESH_TOKEN_BOILED_EGG nor GOOGLE_REFRESH_TOKEN is set"
    exit 1
fi
echo "   Using refresh token: ${REFRESH_TOKEN:0:20}..."
echo ""

# Exchange refresh token for access token
echo "2. Exchanging refresh token for access token..."
RESPONSE=$(curl -s -X POST "https://oauth2.googleapis.com/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "client_id=${GOOGLE_CLIENT_ID}" \
    -d "client_secret=${GOOGLE_CLIENT_SECRET}" \
    -d "refresh_token=${REFRESH_TOKEN}" \
    -d "grant_type=refresh_token")

# Check for errors
if echo "$RESPONSE" | grep -q '"error"'; then
    echo "   ERROR: Token refresh failed"
    echo "$RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$RESPONSE"
    exit 1
fi

ACCESS_TOKEN=$(echo "$RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('access_token', ''))")
SCOPE=$(echo "$RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('scope', 'NOT PROVIDED'))")
EXPIRES_IN=$(echo "$RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('expires_in', 0))")

echo "   Token obtained successfully!"
echo "   Expires in: ${EXPIRES_IN} seconds"
echo ""

# Display scopes
echo "3. Token scopes:"
echo "   $SCOPE" | tr ' ' '\n' | while read scope; do
    if [ -n "$scope" ]; then
        echo "   - $scope"
    fi
done
echo ""

# Check for required scopes
echo "4. Checking required scopes..."
REQUIRED_SCOPES=(
    "https://www.googleapis.com/auth/documents"
    "https://www.googleapis.com/auth/drive"
)

MISSING_SCOPES=0
for required in "${REQUIRED_SCOPES[@]}"; do
    if echo "$SCOPE" | grep -q "$required"; then
        echo "   [OK] $required"
    else
        echo "   [MISSING] $required"
        MISSING_SCOPES=1
    fi
done
echo ""

if [ $MISSING_SCOPES -eq 1 ]; then
    echo "WARNING: Some required scopes are missing!"
    echo "You need to re-authorize with the correct scopes."
    echo "Run: ./scripts/get_google_refresh_token.sh"
    echo ""
fi

# Test token info endpoint
echo "5. Token info from Google..."
TOKEN_INFO=$(curl -s "https://www.googleapis.com/oauth2/v1/tokeninfo?access_token=${ACCESS_TOKEN}")
if echo "$TOKEN_INFO" | grep -q '"error"'; then
    echo "   Could not get token info"
else
    echo "$TOKEN_INFO" | python3 -m json.tool 2>/dev/null || echo "$TOKEN_INFO"
fi
echo ""

# Try to identify the user
echo "5b. Identifying OAuth user..."
USER_INFO=$(curl -s "https://www.googleapis.com/oauth2/v2/userinfo" \
    -H "Authorization: Bearer $ACCESS_TOKEN")
if echo "$USER_INFO" | grep -q '"email"'; then
    USER_EMAIL=$(echo "$USER_INFO" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('email', 'unknown'))")
    USER_NAME=$(echo "$USER_INFO" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('name', 'unknown'))")
    echo "   OAuth user: $USER_NAME <$USER_EMAIL>"
    echo ""
    echo "   IMPORTANT: To enable write access, share the document"
    echo "   with $USER_EMAIL as an EDITOR."
else
    echo "   Could not identify user (email scope may not be granted)"
    echo "   Re-run ./scripts/get_google_refresh_token.sh to get a new token with email scope"
fi
echo ""

# Test Drive API read access
echo "6. Testing Drive API (read - list files)..."
DRIVE_RESPONSE=$(curl -s "https://www.googleapis.com/drive/v3/files?pageSize=1" \
    -H "Authorization: Bearer $ACCESS_TOKEN")

if echo "$DRIVE_RESPONSE" | grep -q '"error"'; then
    echo "   [FAIL] Drive API read access"
    echo "$DRIVE_RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$DRIVE_RESPONSE"
else
    echo "   [OK] Drive API read access works"
fi
echo ""

# Get first document ID for testing
echo "7. Finding a Google Doc for testing..."
DOC_QUERY="mimeType='application/vnd.google-apps.document'"
DOC_RESPONSE=$(curl -s "https://www.googleapis.com/drive/v3/files?q=$(python3 -c "import urllib.parse; print(urllib.parse.quote('''$DOC_QUERY'''))")&pageSize=1&fields=files(id,name)" \
    -H "Authorization: Bearer $ACCESS_TOKEN")

if echo "$DOC_RESPONSE" | grep -q '"error"'; then
    echo "   [FAIL] Could not list documents"
    echo "$DOC_RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$DOC_RESPONSE"
    exit 1
fi

DOC_ID=$(echo "$DOC_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); files=d.get('files',[]); print(files[0]['id'] if files else '')")
DOC_NAME=$(echo "$DOC_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); files=d.get('files',[]); print(files[0].get('name','') if files else '')")

if [ -z "$DOC_ID" ]; then
    echo "   No documents found. Cannot test Docs API."
    exit 0
fi

echo "   Found document: $DOC_NAME ($DOC_ID)"
echo ""

# Test Docs API read access
echo "8. Testing Docs API (read - get document structure)..."
DOCS_READ_RESPONSE=$(curl -s "https://docs.googleapis.com/v1/documents/$DOC_ID" \
    -H "Authorization: Bearer $ACCESS_TOKEN")

if echo "$DOCS_READ_RESPONSE" | grep -q '"error"'; then
    echo "   [FAIL] Docs API read access"
    ERROR_CODE=$(echo "$DOCS_READ_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('error',{}).get('code',''))")
    ERROR_MSG=$(echo "$DOCS_READ_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('error',{}).get('message',''))")
    echo "   Error $ERROR_CODE: $ERROR_MSG"

    if [ "$ERROR_CODE" = "403" ]; then
        echo ""
        echo "   DIAGNOSIS: Google Docs API may not be enabled in your Google Cloud project."
        echo "   Go to: https://console.cloud.google.com/apis/library/docs.googleapis.com"
        echo "   and enable the Google Docs API."
    fi
else
    echo "   [OK] Docs API read access works"
    TITLE=$(echo "$DOCS_READ_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('title',''))")
    echo "   Document title: $TITLE"
fi
echo ""

# Test Docs API write access (insert a space and delete it)
echo "9. Testing Docs API (write - batchUpdate)..."
echo "   Attempting to update document style (harmless operation)..."

# Try a minimal batchUpdate - just update paragraph style which is always valid
BATCH_REQUEST='{
  "requests": [
    {
      "updateParagraphStyle": {
        "range": {
          "startIndex": 1,
          "endIndex": 2
        },
        "paragraphStyle": {
          "alignment": "START"
        },
        "fields": "alignment"
      }
    }
  ]
}'

DOCS_WRITE_RESPONSE=$(curl -s -X POST "https://docs.googleapis.com/v1/documents/$DOC_ID:batchUpdate" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d "$BATCH_REQUEST")

if echo "$DOCS_WRITE_RESPONSE" | grep -q '"error"'; then
    echo "   [FAIL] Docs API write access"
    ERROR_CODE=$(echo "$DOCS_WRITE_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('error',{}).get('code',''))")
    ERROR_MSG=$(echo "$DOCS_WRITE_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('error',{}).get('message',''))")
    ERROR_STATUS=$(echo "$DOCS_WRITE_RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('error',{}).get('status',''))")
    echo "   Error $ERROR_CODE ($ERROR_STATUS): $ERROR_MSG"

    echo ""
    echo "   =============================================="
    echo "   DIAGNOSIS FOR 403 PERMISSION_DENIED:"
    echo "   =============================================="
    echo ""
    echo "   Possible causes:"
    echo "   1. The document is not shared with Editor permissions"
    echo "      - Open the document in Google Docs"
    echo "      - Click 'Share' and check permissions"
    echo "      - The OAuth user must have 'Editor' access (not just 'Viewer')"
    echo ""
    echo "   2. The Google Docs API is not enabled"
    echo "      - Go to: https://console.cloud.google.com/apis/library/docs.googleapis.com"
    echo "      - Click 'Enable' if not already enabled"
    echo ""
    echo "   3. The OAuth token doesn't have write scopes"
    echo "      - Current scopes: $SCOPE"
    echo "      - Required: https://www.googleapis.com/auth/documents"
    echo "      - Run ./scripts/get_google_refresh_token.sh to re-authorize"
    echo ""
else
    echo "   [OK] Docs API write access works!"
    echo ""
    echo "   =============================================="
    echo "   SUCCESS! All API access is working correctly."
    echo "   =============================================="
fi

echo ""
echo "Done."
