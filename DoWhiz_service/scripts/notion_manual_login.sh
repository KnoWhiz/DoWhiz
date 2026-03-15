#!/bin/bash
# Manual Notion Login Script
# This script opens a headed browser for manual login, then saves cookies for automated use.

set -e

COOKIE_PATH="${HOME}/.dowhiz/notion/cookies.json"
SESSION_NAME="notion_manual_login"

echo "=== Notion Manual Login Helper ==="
echo ""
echo "This script will:"
echo "  1. Open a browser window to Notion login"
echo "  2. Wait for you to complete the login (including any 2FA)"
echo "  3. Save cookies for automated use"
echo ""
echo "Cookie save path: ${COOKIE_PATH}"
echo ""

# Ensure browser-use is available
if ! command -v browser-use &> /dev/null; then
    echo "ERROR: browser-use CLI not found. Please install it first."
    exit 1
fi

# Close any existing sessions
echo "Closing any existing browser sessions..."
IN_DOCKER=true browser-use close --all 2>/dev/null || true

# Open headed browser to Notion login
echo ""
echo "Opening browser to Notion login page..."
echo "Please complete the login process in the browser window."
echo ""
IN_DOCKER=true browser-use --session "${SESSION_NAME}" --browser chromium --headed open "https://www.notion.so/login"

echo ""
echo "Browser opened. Please complete login manually."
echo "After you're logged in and see the Notion workspace, press ENTER to continue..."
read -r

# Verify login
echo ""
echo "Checking login status..."
STATE=$(IN_DOCKER=true browser-use --session "${SESSION_NAME}" state 2>&1)

if echo "$STATE" | grep -q "notion-sidebar\|notion-topbar\|notion-scroller\|data-block-id"; then
    echo "Login verified successfully!"
else
    # Check URL
    URL=$(echo "$STATE" | grep -oP 'url:\s*\K\S+' || true)
    if [[ "$URL" == *"notion.so"* ]] && [[ "$URL" != *"/login"* ]]; then
        echo "Login appears successful (URL: $URL)"
    else
        echo "WARNING: Could not verify login. Current URL: $URL"
        echo "If you believe you're logged in, press ENTER to continue, or Ctrl+C to abort."
        read -r
    fi
fi

# Export cookies
echo ""
echo "Exporting cookies..."
mkdir -p "$(dirname "${COOKIE_PATH}")"
IN_DOCKER=true browser-use --session "${SESSION_NAME}" cookies export "${COOKIE_PATH}"

echo ""
echo "Cookies saved to: ${COOKIE_PATH}"
echo ""

# Count cookies
if [ -f "${COOKIE_PATH}" ]; then
    COOKIE_COUNT=$(grep -c '"name"' "${COOKIE_PATH}" 2>/dev/null || echo "0")
    echo "Exported ${COOKIE_COUNT} cookies."
fi

# Close browser
echo ""
echo "Closing browser..."
IN_DOCKER=true browser-use --session "${SESSION_NAME}" close

echo ""
echo "=== Done ==="
echo ""
echo "You can now run the Notion poller without NOTION_FORCE_LOGIN."
echo "The poller will use the saved cookies for authentication."
echo ""
echo "To test, run:"
echo "  NOTION_EMPLOYEE_EMAIL=agent@dowhiz.com cargo run --release -p scheduler_module --bin notion_poller"
