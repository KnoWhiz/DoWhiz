#!/bin/bash
# E2E test for Notion integration with Codex
#
# This script simulates the full flow:
# 1. Store OAuth token in MongoDB (simulating successful OAuth)
# 2. Create a simulated Notion @mention task
# 3. Trigger the worker to process the task
# 4. Verify Codex can read/write to Notion
#
# Prerequisites:
# - Worker running on localhost:9001
# - Notion Internal Integration token
# - Page shared with the integration
#
# Usage:
#   ./test_notion_e2e.sh <NOTION_TOKEN> <PAGE_ID>

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/../.env"

NOTION_TOKEN="${1}"
PAGE_ID="${2}"
EMPLOYEE_ID="${EMPLOYEE_ID:-boiled_egg}"
WORKSPACE_ID="${3:-test-workspace-$(date +%s)}"
WORKER_URL="${WORKER_URL:-http://localhost:9001}"

if [ -z "$NOTION_TOKEN" ] || [ -z "$PAGE_ID" ]; then
    echo "Notion E2E Test"
    echo ""
    echo "Usage: $0 <NOTION_TOKEN> <PAGE_ID> [WORKSPACE_ID]"
    echo ""
    echo "Prerequisites:"
    echo "  1. Create a Notion Internal Integration at https://www.notion.so/my-integrations"
    echo "  2. Share your test page with the integration"
    echo "  3. Copy the page ID from the URL (the 32-char string after the page name)"
    echo ""
    echo "Example:"
    echo "  $0 secret_xxx abc123def456..."
    exit 1
fi

echo "=== Notion E2E Test ==="
echo ""
echo "Configuration:"
echo "  Employee: $EMPLOYEE_ID"
echo "  Workspace ID: $WORKSPACE_ID"
echo "  Page ID: $PAGE_ID"
echo "  Token: ${NOTION_TOKEN:0:20}..."
echo ""

# Step 1: Test the token directly
echo "Step 1: Verifying Notion token..."
BOT_INFO=$(NOTION_API_TOKEN="$NOTION_TOKEN" notion_api_cli me 2>&1) || {
    echo "ERROR: Token verification failed"
    echo "$BOT_INFO"
    exit 1
}
BOT_NAME=$(echo "$BOT_INFO" | jq -r '.name // "Unknown"')
echo "  Bot name: $BOT_NAME"
echo "  Token is valid!"
echo ""

# Step 2: Store token in MongoDB
echo "Step 2: Storing token in MongoDB..."
if command -v mongosh &> /dev/null; then
    mongosh "$MONGODB_URI" --quiet --eval "
    db = db.getSiblingDB('${MONGODB_DATABASE:-dowhiz_local_boiled_egg}');
    db.notion_oauth_tokens.updateOne(
      { workspace_id: '$WORKSPACE_ID', employee_id: '$EMPLOYEE_ID' },
      {
        \$set: {
          workspace_id: '$WORKSPACE_ID',
          workspace_name: 'E2E Test Workspace',
          access_token: '$NOTION_TOKEN',
          bot_id: '$BOT_NAME',
          owner_user_id: 'test-user',
          employee_id: '$EMPLOYEE_ID',
          created_at: new Date()
        }
      },
      { upsert: true }
    );
    print('  Token stored successfully!');
    "
else
    echo "  WARNING: mongosh not found, skipping MongoDB storage"
    echo "  Token will be passed via environment variable instead"
fi
echo ""

# Step 3: Test reading the page
echo "Step 3: Reading Notion page..."
PAGE_INFO=$(NOTION_API_TOKEN="$NOTION_TOKEN" notion_api_cli read-page "$PAGE_ID" 2>&1) || {
    echo "ERROR: Failed to read page"
    echo "$PAGE_INFO"
    exit 1
}
PAGE_URL=$(echo "$PAGE_INFO" | jq -r '.url // "N/A"')
echo "  Page URL: $PAGE_URL"
echo ""

# Step 4: Get page content
echo "Step 4: Reading page blocks..."
BLOCKS=$(NOTION_API_TOKEN="$NOTION_TOKEN" notion_api_cli read-blocks "$PAGE_ID" 2>&1)
BLOCK_COUNT=$(echo "$BLOCKS" | jq -r '.results | length')
echo "  Found $BLOCK_COUNT blocks"
echo ""

# Step 5: Create test workspace for Codex
echo "Step 5: Creating test workspace for Codex..."
WORKSPACE_DIR="/tmp/notion_e2e_test_$(date +%s)"
mkdir -p "$WORKSPACE_DIR"

# Create Notion context file (like email trigger would)
cat > "$WORKSPACE_DIR/.notion_context.json" << EOF
{
    "page_id": "$PAGE_ID",
    "page_url": "$PAGE_URL",
    "workspace_id": "$WORKSPACE_ID",
    "actor_name": "E2E Test",
    "notification_type": "test_mention",
    "task": "Please read this page and add a comment saying 'E2E test completed successfully at [current time]'"
}
EOF

echo "  Created: $WORKSPACE_DIR/.notion_context.json"

# Create test prompt
cat > "$WORKSPACE_DIR/prompt.txt" << EOF
You have been mentioned in a Notion page. Please:

1. Read the Notion context from .notion_context.json
2. Use notion_api_cli to read the page content
3. Add a comment to the page saying "E2E test completed at $(date)"

Available commands:
- notion_api_cli read-page PAGE_ID
- notion_api_cli read-blocks PAGE_ID
- notion_api_cli create-comment PAGE_ID "message"

The NOTION_API_TOKEN environment variable is set.
EOF

echo "  Created: $WORKSPACE_DIR/prompt.txt"
echo ""

# Step 6: Test with Codex (manual or automated)
echo "Step 6: Ready for Codex test"
echo ""
echo "=== Manual Test Instructions ==="
echo ""
echo "Option A: Run Codex manually in the workspace:"
echo "  cd $WORKSPACE_DIR"
echo "  export NOTION_API_TOKEN='$NOTION_TOKEN'"
echo "  codex --model claude-sonnet-4-20250514 \"\$(cat prompt.txt)\""
echo ""
echo "Option B: Use the worker API to enqueue a task:"
echo "  curl -X POST '$WORKER_URL/tasks' \\"
echo "    -H 'Content-Type: application/json' \\"
echo "    -d '{\"type\": \"notion_test\", \"page_id\": \"$PAGE_ID\"}'"
echo ""
echo "Option C: Quick API test (no Codex):"
echo "  export NOTION_API_TOKEN='$NOTION_TOKEN'"
echo "  notion_api_cli create-comment '$PAGE_ID' 'Test comment from E2E script'"
echo ""

# Step 7: Optional - run quick API test
read -p "Run quick API test to add a comment? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Adding test comment..."
    RESULT=$(NOTION_API_TOKEN="$NOTION_TOKEN" notion_api_cli create-comment "$PAGE_ID" "E2E test completed at $(date)" 2>&1)
    if echo "$RESULT" | jq -e '.id' > /dev/null 2>&1; then
        COMMENT_ID=$(echo "$RESULT" | jq -r '.id')
        echo "SUCCESS! Comment created with ID: $COMMENT_ID"
        echo "Check your Notion page for the new comment."
    else
        echo "Result: $RESULT"
    fi
fi

echo ""
echo "=== E2E Test Setup Complete ==="
