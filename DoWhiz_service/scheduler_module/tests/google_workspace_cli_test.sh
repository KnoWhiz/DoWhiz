#!/bin/bash
# Google Workspace CLI Test Script
#
# This script tests the CLI tools for Google Docs, Sheets, and Slides.
# Requires Google OAuth credentials to be set in environment variables.
#
# Usage:
#   export GOOGLE_CLIENT_ID="your_client_id"
#   export GOOGLE_CLIENT_SECRET="your_client_secret"
#   export GOOGLE_REFRESH_TOKEN="your_refresh_token"
#   ./google_workspace_cli_test.sh
#
# Or use a pre-generated access token:
#   export GOOGLE_ACCESS_TOKEN="your_token"
#   ./google_workspace_cli_test.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=========================================="
echo "Google Workspace CLI Test Suite"
echo "=========================================="
echo ""

# Check if credentials are available
if [ -z "$GOOGLE_ACCESS_TOKEN" ] && [ -z "$GOOGLE_REFRESH_TOKEN" ]; then
    echo -e "${RED}Error: No Google credentials found.${NC}"
    echo "Set GOOGLE_ACCESS_TOKEN or (GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REFRESH_TOKEN)"
    exit 1
fi

# Build the binaries
echo "Building CLI binaries..."
cd "$PROJECT_DIR"
cargo build --release --bin google-docs --bin google-sheets --bin google-slides 2>&1 | grep -E "(Compiling|Finished|error)" || true

GOOGLE_DOCS="$PROJECT_DIR/target/release/google-docs"
GOOGLE_SHEETS="$PROJECT_DIR/target/release/google-sheets"
GOOGLE_SLIDES="$PROJECT_DIR/target/release/google-slides"

# Check binaries exist
for bin in "$GOOGLE_DOCS" "$GOOGLE_SHEETS" "$GOOGLE_SLIDES"; do
    if [ ! -f "$bin" ]; then
        echo -e "${RED}Error: Binary not found: $bin${NC}"
        exit 1
    fi
done

echo -e "${GREEN}Binaries built successfully.${NC}"
echo ""

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

run_test() {
    local name="$1"
    local cmd="$2"

    echo -n "Testing: $name... "

    if output=$(eval "$cmd" 2>&1); then
        echo -e "${GREEN}PASSED${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        if [ "$VERBOSE" = "1" ]; then
            echo "Output: $output"
        fi
        return 0
    else
        echo -e "${RED}FAILED${NC}"
        echo "Command: $cmd"
        echo "Output: $output"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# ==========================================
# Google Docs Tests
# ==========================================
echo ""
echo "=========================================="
echo "1. Google Docs CLI Tests"
echo "=========================================="

run_test "google-docs --help" "'$GOOGLE_DOCS' --help"
run_test "google-docs list-documents" "'$GOOGLE_DOCS' list-documents"

# Get first document ID for further tests
DOC_ID=$("$GOOGLE_DOCS" list-documents 2>&1 | grep -oP '\(\K[a-zA-Z0-9_-]+(?=\))' | head -1)
if [ -n "$DOC_ID" ]; then
    echo -e "${YELLOW}Found document ID: $DOC_ID${NC}"
    run_test "google-docs read-document" "'$GOOGLE_DOCS' read-document '$DOC_ID'"
    run_test "google-docs list-comments" "'$GOOGLE_DOCS' list-comments '$DOC_ID'"
    run_test "google-docs get-styles" "'$GOOGLE_DOCS' get-styles '$DOC_ID'"
else
    echo -e "${YELLOW}No documents found, skipping document-specific tests${NC}"
fi

# ==========================================
# Google Sheets Tests
# ==========================================
echo ""
echo "=========================================="
echo "2. Google Sheets CLI Tests"
echo "=========================================="

run_test "google-sheets --help" "'$GOOGLE_SHEETS' --help"
run_test "google-sheets list-spreadsheets" "'$GOOGLE_SHEETS' list-spreadsheets"

# Get first spreadsheet ID for further tests
SHEET_ID=$("$GOOGLE_SHEETS" list-spreadsheets 2>&1 | grep -oP '\(\K[a-zA-Z0-9_-]+(?=\))' | head -1)
if [ -n "$SHEET_ID" ]; then
    echo -e "${YELLOW}Found spreadsheet ID: $SHEET_ID${NC}"
    run_test "google-sheets read-spreadsheet" "'$GOOGLE_SHEETS' read-spreadsheet '$SHEET_ID'"
    run_test "google-sheets get-metadata" "'$GOOGLE_SHEETS' get-metadata '$SHEET_ID'"
    run_test "google-sheets list-comments" "'$GOOGLE_SHEETS' list-comments '$SHEET_ID'"
    run_test "google-sheets read-values" "'$GOOGLE_SHEETS' read-values '$SHEET_ID' 'Sheet1!A1:C5'"
else
    echo -e "${YELLOW}No spreadsheets found, skipping spreadsheet-specific tests${NC}"
fi

# ==========================================
# Google Slides Tests
# ==========================================
echo ""
echo "=========================================="
echo "3. Google Slides CLI Tests"
echo "=========================================="

run_test "google-slides --help" "'$GOOGLE_SLIDES' --help"
run_test "google-slides list-presentations" "'$GOOGLE_SLIDES' list-presentations"

# Get first presentation ID for further tests
SLIDES_ID=$("$GOOGLE_SLIDES" list-presentations 2>&1 | grep -oP '\(\K[a-zA-Z0-9_-]+(?=\))' | head -1)
if [ -n "$SLIDES_ID" ]; then
    echo -e "${YELLOW}Found presentation ID: $SLIDES_ID${NC}"
    run_test "google-slides read-presentation" "'$GOOGLE_SLIDES' read-presentation '$SLIDES_ID'"
    run_test "google-slides get-presentation" "'$GOOGLE_SLIDES' get-presentation '$SLIDES_ID'"
    run_test "google-slides list-comments" "'$GOOGLE_SLIDES' list-comments '$SLIDES_ID'"
else
    echo -e "${YELLOW}No presentations found, skipping presentation-specific tests${NC}"
fi

# ==========================================
# Summary
# ==========================================
echo ""
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -gt 0 ]; then
    echo ""
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo ""
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
