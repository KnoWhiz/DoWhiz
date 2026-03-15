#!/bin/bash
# Test Notion API directly with a token
#
# Usage:
#   ./test_notion_api.sh <NOTION_TOKEN>
#
# Get your token:
# 1. Go to https://www.notion.so/my-integrations
# 2. Create an Internal Integration
# 3. Copy the "Internal Integration Secret" (starts with secret_)
# 4. Share your Notion page with the integration

set -e

NOTION_TOKEN="${1:-$NOTION_API_KEY}"

if [ -z "$NOTION_TOKEN" ]; then
    echo "Usage: $0 <NOTION_TOKEN>"
    echo ""
    echo "Get your token from: https://www.notion.so/my-integrations"
    echo "Or set NOTION_API_KEY environment variable"
    exit 1
fi

echo "=== Testing Notion API ==="
echo ""

# Test 1: Get current user (bot)
echo "1. Testing /users/me (bot info)..."
curl -s "https://api.notion.com/v1/users/me" \
    -H "Authorization: Bearer $NOTION_TOKEN" \
    -H "Notion-Version: 2022-06-28" | jq .

echo ""

# Test 2: Search for pages
echo "2. Searching for pages..."
curl -s "https://api.notion.com/v1/search" \
    -H "Authorization: Bearer $NOTION_TOKEN" \
    -H "Notion-Version: 2022-06-28" \
    -H "Content-Type: application/json" \
    -d '{"page_size": 5}' | jq '.results | length' | xargs -I{} echo "Found {} accessible pages/databases"

echo ""

# Test 3: If PAGE_ID is provided, read it
if [ -n "$2" ]; then
    PAGE_ID="$2"
    echo "3. Reading page $PAGE_ID..."
    curl -s "https://api.notion.com/v1/pages/$PAGE_ID" \
        -H "Authorization: Bearer $NOTION_TOKEN" \
        -H "Notion-Version: 2022-06-28" | jq '{id, url, created_time}'

    echo ""
    echo "4. Getting page blocks..."
    curl -s "https://api.notion.com/v1/blocks/$PAGE_ID/children" \
        -H "Authorization: Bearer $NOTION_TOKEN" \
        -H "Notion-Version: 2022-06-28" | jq '.results | length' | xargs -I{} echo "Page has {} blocks"

    echo ""
    echo "5. Getting comments on page..."
    curl -s "https://api.notion.com/v1/comments?block_id=$PAGE_ID" \
        -H "Authorization: Bearer $NOTION_TOKEN" \
        -H "Notion-Version: 2022-06-28" | jq '.results | length' | xargs -I{} echo "Page has {} comments"
fi

echo ""
echo "=== API Test Complete ==="
echo ""
echo "To reply to a comment, use:"
echo 'curl -X POST "https://api.notion.com/v1/comments" \'
echo '  -H "Authorization: Bearer $NOTION_TOKEN" \'
echo '  -H "Notion-Version: 2022-06-28" \'
echo '  -H "Content-Type: application/json" \'
echo '  -d '\''{"discussion_id": "...", "rich_text": [{"text": {"content": "Your reply"}}]}'\'''
