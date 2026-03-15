#!/bin/bash
# Real Notion @mention E2E test
#
# This script helps test the full workflow with a real Notion @mention.
#
# Prerequisites:
# 1. Gateway running on port 9100
# 2. Worker running on port 9001 with RUN_TASK_EXECUTION_BACKEND=local
# 3. Internal Integration token stored (or in .env as NOTION_API_TOKEN)
# 4. Page shared with the integration
#
# Usage:
#   Step 1: Go to Notion and @mention proto on a shared page
#   Step 2: Wait for Notion email notification (check proto@dowhiz.com inbox)
#   Step 3: Copy the email content and run this script with the email file
#
#   ./test_notion_real_mention.sh <email_file.eml>
#
# Or use manual JSON mode:
#   ./test_notion_real_mention.sh --manual

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/../.env" 2>/dev/null || true

GATEWAY_URL="${GATEWAY_URL:-http://localhost:9100}"
NOTION_TOKEN="${NOTION_API_TOKEN:-$1}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "=============================================="
echo "  Notion Real @mention E2E Test"
echo "=============================================="
echo ""

# Check services
echo "Checking services..."
if ! curl -s "$GATEWAY_URL/health" > /dev/null 2>&1; then
    echo -e "${RED}ERROR: Gateway not running on $GATEWAY_URL${NC}"
    echo "Start with: source .env && /mnt/d/cargo-cache/release/inbound_gateway"
    exit 1
fi
echo -e "${GREEN}Gateway: OK${NC}"

if ! curl -s "http://localhost:9001/health" > /dev/null 2>&1; then
    echo -e "${RED}ERROR: Worker not running on port 9001${NC}"
    echo "Start with: RUN_TASK_EXECUTION_BACKEND=local /mnt/d/cargo-cache/release/rust_service --port 9001"
    exit 1
fi
echo -e "${GREEN}Worker: OK${NC}"
echo ""

if [ "$1" == "--manual" ]; then
    echo "=============================================="
    echo "  Manual Test Mode"
    echo "=============================================="
    echo ""
    echo "Step 1: Go to Notion and create a comment mentioning @proto"
    echo "        (on a page shared with the integration)"
    echo ""
    echo "Step 2: Wait for the email notification from notify@mail.notion.so"
    echo ""
    echo "Step 3: Forward the email or copy its content here."
    echo ""
    echo "Enter the email details below:"
    echo ""

    read -p "Email Subject: " SUBJECT
    read -p "Page URL (from email): " PAGE_URL
    read -p "Actor Name (who mentioned you): " ACTOR_NAME
    read -p "Comment text: " COMMENT_TEXT

    # Extract page ID from URL
    PAGE_ID=$(echo "$PAGE_URL" | grep -oE '[a-f0-9]{32}' | tail -1)

    if [ -z "$PAGE_ID" ]; then
        echo -e "${RED}ERROR: Could not extract page ID from URL${NC}"
        exit 1
    fi

    echo ""
    echo "Extracted page_id: $PAGE_ID"
    echo ""

    # Build the email payload
    MSG_ID="notion-real-test-$(date +%s)@mail.notion.so"

    PAYLOAD=$(cat <<EOF
{
  "From": "Notion <notify@mail.notion.so>",
  "To": "Proto <proto@dowhiz.com>",
  "Subject": "$SUBJECT",
  "TextBody": "$ACTOR_NAME mentioned you\\n\\n$COMMENT_TEXT\\n\\nView in Notion:\\n$PAGE_URL",
  "HtmlBody": "<p>$ACTOR_NAME mentioned you</p><p>$COMMENT_TEXT</p><p><a href='$PAGE_URL'>View in Notion</a></p>",
  "MessageID": "$MSG_ID",
  "Date": "$(date -R)"
}
EOF
)

    echo "Sending to gateway..."
    RESULT=$(curl -s -X POST "$GATEWAY_URL/postmark/inbound" \
        -H "Content-Type: application/json" \
        -d "$PAYLOAD")

    echo "Gateway response: $RESULT"

    if echo "$RESULT" | grep -q "accepted"; then
        echo -e "${GREEN}Email accepted!${NC}"
        echo ""
        echo "Now waiting for Codex to process..."
        echo "Check worker logs or wait ~60-120 seconds for completion."
        echo ""
        echo "To verify the comment was posted:"
        echo "  export NOTION_API_TOKEN='$NOTION_TOKEN'"
        echo "  $SCRIPT_DIR/notion_api_cli get-comments $PAGE_ID"
    else
        echo -e "${RED}Failed to send email${NC}"
        exit 1
    fi

elif [ -f "$1" ]; then
    echo "Processing email file: $1"
    # TODO: Parse .eml file and extract fields
    echo "EML file parsing not yet implemented"
    echo "Use --manual mode instead"
    exit 1

else
    echo "Usage:"
    echo "  $0 --manual           Interactive mode"
    echo "  $0 <email.eml>        Process saved email file"
    echo ""
    echo "Quick test (simulated email):"
    echo "  $0 --quick <PAGE_ID>  Send a quick test to a known page"

    if [ "$1" == "--quick" ] && [ -n "$2" ]; then
        PAGE_ID="$2"
        echo ""
        echo "Sending quick test to page $PAGE_ID..."

        MSG_ID="notion-quick-test-$(date +%s)@mail.notion.so"

        curl -s -X POST "$GATEWAY_URL/postmark/inbound" \
            -H "Content-Type: application/json" \
            -d "{
              \"From\": \"Notion <notify@mail.notion.so>\",
              \"To\": \"Proto <proto@dowhiz.com>\",
              \"Subject\": \"Test User mentioned you in Test Page\",
              \"TextBody\": \"Test User mentioned you\\n\\n@proto quick test - reply with current time\\n\\nView in Notion:\\nhttps://www.notion.so/Test-Page-$PAGE_ID\",
              \"MessageID\": \"$MSG_ID\",
              \"Date\": \"$(date -R)\"
            }"
        echo ""
    fi
fi
