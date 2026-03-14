#!/bin/bash
# Manually store a Notion token in MongoDB for testing
#
# Usage:
#   ./store_notion_token.sh <WORKSPACE_ID> <ACCESS_TOKEN> [EMPLOYEE_ID]
#
# Example:
#   ./store_notion_token.sh "abc123-def456" "secret_xxx" "boiled_egg"

set -e

source "$(dirname "$0")/../.env"

WORKSPACE_ID="${1}"
ACCESS_TOKEN="${2}"
EMPLOYEE_ID="${3:-boiled_egg}"
WORKSPACE_NAME="${4:-Test Workspace}"

if [ -z "$WORKSPACE_ID" ] || [ -z "$ACCESS_TOKEN" ]; then
    echo "Usage: $0 <WORKSPACE_ID> <ACCESS_TOKEN> [EMPLOYEE_ID] [WORKSPACE_NAME]"
    echo ""
    echo "To get your workspace ID:"
    echo "  1. Go to your Notion workspace"
    echo "  2. Open any page, copy the URL"
    echo "  3. The URL looks like: notion.so/WORKSPACE_ID/page-name-PAGE_ID"
    echo "     Or use 'test-workspace' as a placeholder"
    echo ""
    echo "To get your access token:"
    echo "  1. Go to https://www.notion.so/my-integrations"
    echo "  2. Create an Internal Integration"
    echo "  3. Copy the 'Internal Integration Secret'"
    exit 1
fi

echo "Storing Notion token in MongoDB..."
echo "  Workspace ID: $WORKSPACE_ID"
echo "  Employee ID: $EMPLOYEE_ID"
echo "  Token: ${ACCESS_TOKEN:0:20}..."

# Use mongosh to insert the token
mongosh "$MONGODB_URI" --quiet --eval "
db = db.getSiblingDB('${MONGODB_DATABASE:-dowhiz_local_boiled_egg}');
db.notion_oauth_tokens.updateOne(
  { workspace_id: '$WORKSPACE_ID', employee_id: '$EMPLOYEE_ID' },
  {
    \$set: {
      workspace_id: '$WORKSPACE_ID',
      workspace_name: '$WORKSPACE_NAME',
      access_token: '$ACCESS_TOKEN',
      bot_id: 'manual-test-bot',
      owner_user_id: 'manual-test-user',
      employee_id: '$EMPLOYEE_ID',
      created_at: new Date()
    }
  },
  { upsert: true }
);
print('Token stored successfully!');

// List all tokens
print('');
print('Current tokens for employee $EMPLOYEE_ID:');
db.notion_oauth_tokens.find({ employee_id: '$EMPLOYEE_ID' }).forEach(function(doc) {
  print('  - ' + doc.workspace_name + ' (' + doc.workspace_id + ')');
});
"

echo ""
echo "Done! You can now test the Notion API client."
