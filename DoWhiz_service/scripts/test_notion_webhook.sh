#!/bin/bash
# Test Notion webhook handler locally
#
# Usage:
#   ./scripts/test_notion_webhook.sh [gateway_url]
#
# Default gateway URL: http://localhost:8080
# For staging: ./scripts/test_notion_webhook.sh https://api.staging.dowhiz.com

set -e

GATEWAY_URL="${1:-http://localhost:8080}"
WEBHOOK_ENDPOINT="$GATEWAY_URL/webhook/notion"

echo "Testing Notion webhook at: $WEBHOOK_ENDPOINT"
echo ""

# Generate test data
COMMENT_ID="test-comment-$(date +%s)"
PAGE_ID="3196a52c-d8a0-80a7-b375-f2dd2ca2aa1d"
WORKSPACE_ID="2be6a52c-d8a0-812a-8684-0003b0ffbf46"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%S.000Z")

# Build webhook payload (matches Notion's comment.created event format)
PAYLOAD=$(cat <<EOF
{
  "type": "comment.created",
  "timestamp": "$TIMESTAMP",
  "workspace_id": "$WORKSPACE_ID",
  "subscription_id": "sub-test-123",
  "integration_id": "324d872b-594c-815b-b12f-0037b4861a85",
  "data": {
    "id": "$COMMENT_ID",
    "parent": {
      "type": "page_id",
      "page_id": "$PAGE_ID",
      "workspace_id": "$WORKSPACE_ID"
    },
    "discussion_id": "disc-test-123",
    "created_time": "$TIMESTAMP",
    "created_by": {
      "id": "user-test-123",
      "name": "Test User",
      "type": "person"
    },
    "rich_text": [
      {
        "type": "mention",
        "mention": {
          "type": "user",
          "user": {
            "id": "bot-id",
            "name": "Proto-DoWhiz"
          }
        },
        "plain_text": "@Proto-DoWhiz"
      },
      {
        "type": "text",
        "text": {
          "content": " please help me test the webhook integration"
        },
        "plain_text": " please help me test the webhook integration"
      }
    ]
  }
}
EOF
)

echo "Payload:"
echo "$PAYLOAD" | jq .
echo ""

# Compute HMAC signature if NOTION_WEBHOOK_SECRET is set
if [ -n "$NOTION_WEBHOOK_SECRET" ]; then
    SIGNATURE=$(echo -n "$PAYLOAD" | openssl dgst -sha256 -hmac "$NOTION_WEBHOOK_SECRET" | sed 's/^.* //')
    SIGNATURE_HEADER="X-Notion-Signature: v1=$SIGNATURE"
    echo "Using signature: v1=$SIGNATURE"
else
    SIGNATURE_HEADER=""
    echo "Warning: NOTION_WEBHOOK_SECRET not set, sending without signature"
fi

echo ""
echo "Sending webhook..."
echo ""

# Send the webhook
if [ -n "$SIGNATURE_HEADER" ]; then
    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$WEBHOOK_ENDPOINT" \
        -H "Content-Type: application/json" \
        -H "$SIGNATURE_HEADER" \
        -d "$PAYLOAD")
else
    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$WEBHOOK_ENDPOINT" \
        -H "Content-Type: application/json" \
        -d "$PAYLOAD")
fi

# Extract body and status code
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "Response (HTTP $HTTP_CODE):"
echo "$BODY" | jq . 2>/dev/null || echo "$BODY"
echo ""

if [ "$HTTP_CODE" = "200" ]; then
    echo "SUCCESS: Webhook accepted"
else
    echo "FAILED: HTTP $HTTP_CODE"
    exit 1
fi
