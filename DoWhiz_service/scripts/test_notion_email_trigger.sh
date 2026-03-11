#!/bin/bash
# Test Notion email trigger E2E
# This script simulates a Notion notification email to test the new flow

set -e
cd "$(dirname "$0")/.."
source .env 2>/dev/null || true

echo "=== Notion Email Trigger Test ==="

# Create a test payload simulating Notion email notification
TEST_PAYLOAD=$(cat <<'JSONEOF'
{
  "From": "Notion <notify@mail.notion.so>",
  "To": "oliver@dowhiz.com",
  "Subject": "Test User mentioned you in Test Page",
  "TextBody": "Test User mentioned you in a comment on Test Page.\n\n@Oliver please review this section.\n\nView in Notion: https://www.notion.so/workspace/Test-Page-abc123def456789012345678901234ab?d=comment123",
  "HtmlBody": "<p>Test User mentioned you in a comment on <a href=\"https://www.notion.so/workspace/Test-Page-abc123def456789012345678901234ab\">Test Page</a>.</p><p>@Oliver please review this section.</p>",
  "MessageID": "<notion-test-$(date +%s)@mail.notion.so>"
}
JSONEOF
)

echo ""
echo "Test payload:"
echo "$TEST_PAYLOAD" | jq .

# Check if worker is running
if pgrep -f "rust_service.*worker" > /dev/null 2>&1 || pgrep -f "run_employee" > /dev/null 2>&1; then
    echo ""
    echo "✓ Worker appears to be running"
else
    echo ""
    echo "⚠ Worker not detected. Start with:"
    echo "  ./scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok"
fi

# Save test payload for manual testing
TEST_FILE="/tmp/notion_email_test_payload.json"
echo "$TEST_PAYLOAD" > "$TEST_FILE"
echo ""
echo "Test payload saved to: $TEST_FILE"

# If we have curl and the worker webhook endpoint
WORKER_PORT="${WORKER_PORT:-9001}"
WEBHOOK_URL="http://localhost:$WORKER_PORT/webhook/postmark"

echo ""
echo "To test locally (if worker is running with webhook):"
echo "  curl -X POST '$WEBHOOK_URL' -H 'Content-Type: application/json' -d @$TEST_FILE"

echo ""
echo "=== Manual E2E Test Steps ==="
echo "1. Ensure Notion email notifications are forwarded to your service email"
echo "2. In Notion, @mention Oliver (or your employee) in a page comment"
echo "3. Watch worker logs for: 'detected Notion email notification'"
echo "4. Check workspace for .notion_email_context.json"
echo ""
echo "Log command: pm2 logs dw_worker --lines 50 | grep -i notion"
