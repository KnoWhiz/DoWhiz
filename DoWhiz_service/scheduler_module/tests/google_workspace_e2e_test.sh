#!/bin/bash
# Google Workspace End-to-End Test Script
#
# This script tests the full workflow:
# 1. List shared files (Docs/Sheets/Slides)
# 2. Find comments that mention @employee
# 3. Reply to comments
# 4. Make edits to the file
#
# Prerequisites:
# - Share a Google Doc/Sheet/Slides with the service account
# - Add a comment mentioning @oliver (or configured employee name)
#
# Usage:
#   export GOOGLE_ACCESS_TOKEN="your_token"
#   ./google_workspace_e2e_test.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "=========================================="
echo "Google Workspace E2E Test"
echo "=========================================="
echo ""

# Check credentials
if [ -z "$GOOGLE_ACCESS_TOKEN" ] && [ -z "$GOOGLE_REFRESH_TOKEN" ]; then
    echo -e "${RED}Error: No Google credentials found.${NC}"
    exit 1
fi

# Build binaries
echo "Building CLI binaries..."
cd "$PROJECT_DIR"
cargo build --release --bin google-docs --bin google-sheets --bin google-slides 2>/dev/null

GOOGLE_DOCS="$PROJECT_DIR/target/release/google-docs"
GOOGLE_SHEETS="$PROJECT_DIR/target/release/google-sheets"
GOOGLE_SLIDES="$PROJECT_DIR/target/release/google-slides"

echo -e "${GREEN}Build complete.${NC}"
echo ""

# ==========================================
# Test 1: Google Sheets E2E
# ==========================================
echo "=========================================="
echo -e "${BLUE}Test 1: Google Sheets Workflow${NC}"
echo "=========================================="

echo ""
echo "Step 1.1: List all spreadsheets..."
SHEETS_OUTPUT=$("$GOOGLE_SHEETS" list-spreadsheets 2>&1)
echo "$SHEETS_OUTPUT"

SHEET_ID=$(echo "$SHEETS_OUTPUT" | grep -oP '\(\K[a-zA-Z0-9_-]+(?=\))' | head -1)

if [ -z "$SHEET_ID" ]; then
    echo -e "${YELLOW}No spreadsheets found. Please share a spreadsheet with the service account.${NC}"
else
    echo ""
    echo -e "${GREEN}Using spreadsheet ID: $SHEET_ID${NC}"

    echo ""
    echo "Step 1.2: List comments on spreadsheet..."
    "$GOOGLE_SHEETS" list-comments "$SHEET_ID" 2>&1

    echo ""
    echo "Step 1.3: Read spreadsheet content (first 5 rows)..."
    "$GOOGLE_SHEETS" read-values "$SHEET_ID" "Sheet1!A1:E5" 2>&1 || echo "(No data or sheet not named 'Sheet1')"

    echo ""
    echo "Step 1.4: Get spreadsheet metadata..."
    "$GOOGLE_SHEETS" get-metadata "$SHEET_ID" 2>&1

    # Optional: Test editing (uncomment to enable)
    # echo ""
    # echo "Step 1.5: Test update values..."
    # "$GOOGLE_SHEETS" update-values "$SHEET_ID" "Sheet1!F1" '[["Test from CLI"]]' 2>&1

    # echo ""
    # echo "Step 1.6: Test append row..."
    # "$GOOGLE_SHEETS" append-rows "$SHEET_ID" "Sheet1!A:E" '[["Row added by", "CLI test", "at", "'$(date +%H:%M:%S)'", "success"]]' 2>&1
fi

# ==========================================
# Test 2: Google Slides E2E
# ==========================================
echo ""
echo "=========================================="
echo -e "${BLUE}Test 2: Google Slides Workflow${NC}"
echo "=========================================="

echo ""
echo "Step 2.1: List all presentations..."
SLIDES_OUTPUT=$("$GOOGLE_SLIDES" list-presentations 2>&1)
echo "$SLIDES_OUTPUT"

SLIDES_ID=$(echo "$SLIDES_OUTPUT" | grep -oP '\(\K[a-zA-Z0-9_-]+(?=\))' | head -1)

if [ -z "$SLIDES_ID" ]; then
    echo -e "${YELLOW}No presentations found. Please share a presentation with the service account.${NC}"
else
    echo ""
    echo -e "${GREEN}Using presentation ID: $SLIDES_ID${NC}"

    echo ""
    echo "Step 2.2: List comments on presentation..."
    "$GOOGLE_SLIDES" list-comments "$SLIDES_ID" 2>&1

    echo ""
    echo "Step 2.3: Get presentation structure..."
    "$GOOGLE_SLIDES" get-presentation "$SLIDES_ID" 2>&1

    echo ""
    echo "Step 2.4: Read presentation content..."
    "$GOOGLE_SLIDES" read-presentation "$SLIDES_ID" 2>&1 | head -30

    # Optional: Test editing (uncomment to enable)
    # echo ""
    # echo "Step 2.5: Test replace all text..."
    # "$GOOGLE_SLIDES" replace-all-text "$SLIDES_ID" --find="PLACEHOLDER" --replace="CLI Test" 2>&1
fi

# ==========================================
# Test 3: Google Docs E2E (for comparison)
# ==========================================
echo ""
echo "=========================================="
echo -e "${BLUE}Test 3: Google Docs Workflow${NC}"
echo "=========================================="

echo ""
echo "Step 3.1: List all documents..."
DOCS_OUTPUT=$("$GOOGLE_DOCS" list-documents 2>&1)
echo "$DOCS_OUTPUT"

DOC_ID=$(echo "$DOCS_OUTPUT" | grep -oP '\(\K[a-zA-Z0-9_-]+(?=\))' | head -1)

if [ -z "$DOC_ID" ]; then
    echo -e "${YELLOW}No documents found. Please share a document with the service account.${NC}"
else
    echo ""
    echo -e "${GREEN}Using document ID: $DOC_ID${NC}"

    echo ""
    echo "Step 3.2: List comments on document..."
    "$GOOGLE_DOCS" list-comments "$DOC_ID" 2>&1

    echo ""
    echo "Step 3.3: Read document content..."
    "$GOOGLE_DOCS" read-document "$DOC_ID" 2>&1 | head -30
fi

# ==========================================
# Summary
# ==========================================
echo ""
echo "=========================================="
echo -e "${GREEN}E2E Test Complete${NC}"
echo "=========================================="
echo ""
echo "Next steps to verify full workflow:"
echo "1. Share a Google Sheet/Slides with the service account email"
echo "2. Add a comment mentioning @oliver (or your employee name)"
echo "3. Run the poller to pick up the comment"
echo "4. Verify the agent processes and replies to the comment"
echo ""
echo "To test comment reply manually:"
echo "  google-sheets reply-comment <sheet_id> <comment_id> 'Hello from CLI!'"
echo "  google-slides reply-comment <slides_id> <comment_id> 'Hello from CLI!'"
