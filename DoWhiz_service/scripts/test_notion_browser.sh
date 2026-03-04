#!/bin/bash
# Test script for Notion browser integration
#
# Prerequisites:
# 1. geckodriver or chromedriver running on port 4444
# 2. MongoDB accessible (local or remote)
# 3. Environment variables configured

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Load environment
if [[ -f "$PROJECT_ROOT/.env" ]]; then
    export $(grep -v '^#' "$PROJECT_ROOT/.env" | xargs)
fi

# Notion browser integration env vars
export NOTION_BROWSER_ENABLED="${NOTION_BROWSER_ENABLED:-true}"
export NOTION_EMPLOYEE_EMAIL="${NOTION_EMPLOYEE_EMAIL:-agent@dowhiz.com}"
export NOTION_EMPLOYEE_PASSWORD="${NOTION_EMPLOYEE_PASSWORD:-}"
export NOTION_POLL_INTERVAL_SECS="${NOTION_POLL_INTERVAL_SECS:-30}"
export NOTION_BROWSER_HEADLESS="${NOTION_BROWSER_HEADLESS:-false}"
export NOTION_BROWSER_SLOW_MO="${NOTION_BROWSER_SLOW_MO:-100}"
export WEBDRIVER_URL="${WEBDRIVER_URL:-http://localhost:4444}"
export NOTION_EMPLOYEE_NAME="${NOTION_EMPLOYEE_NAME:-Oliver}"

echo "=== Notion Browser Integration Test ==="
echo ""
echo "Configuration:"
echo "  NOTION_EMPLOYEE_EMAIL: $NOTION_EMPLOYEE_EMAIL"
echo "  NOTION_BROWSER_HEADLESS: $NOTION_BROWSER_HEADLESS"
echo "  WEBDRIVER_URL: $WEBDRIVER_URL"
echo "  NOTION_POLL_INTERVAL_SECS: $NOTION_POLL_INTERVAL_SECS"
echo ""

# Check if geckodriver/chromedriver is running
check_webdriver() {
    echo "Checking WebDriver at $WEBDRIVER_URL..."
    if curl -s -o /dev/null -w "%{http_code}" "$WEBDRIVER_URL/status" | grep -q "200"; then
        echo "  WebDriver is running"
        return 0
    else
        echo "  ERROR: WebDriver is not running at $WEBDRIVER_URL"
        echo ""
        echo "  To start geckodriver (Firefox):"
        echo "    geckodriver --port 4444"
        echo ""
        echo "  To start chromedriver (Chrome):"
        echo "    chromedriver --port=4444"
        echo ""
        return 1
    fi
}

# Check MongoDB connectivity
check_mongodb() {
    echo "Checking MongoDB..."
    if [[ -z "${MONGODB_URI:-}" ]]; then
        echo "  WARNING: MONGODB_URI not set"
        return 1
    fi
    echo "  MONGODB_URI is configured"
    return 0
}

# Check Notion credentials
check_notion_creds() {
    echo "Checking Notion credentials..."
    if [[ -z "${NOTION_EMPLOYEE_PASSWORD:-}" ]]; then
        echo "  ERROR: NOTION_EMPLOYEE_PASSWORD not set"
        return 1
    fi
    echo "  Credentials configured for $NOTION_EMPLOYEE_EMAIL"
    return 0
}

# Run Rust tests for Notion module
run_rust_tests() {
    echo ""
    echo "=== Running Notion module tests ==="
    cd "$PROJECT_ROOT"
    cargo test --release -p scheduler_module notion -- --nocapture 2>&1 || true
}

# Main
echo "=== Pre-flight checks ==="
echo ""

CHECKS_PASSED=true

if ! check_webdriver; then
    CHECKS_PASSED=false
fi

if ! check_mongodb; then
    CHECKS_PASSED=false
fi

if ! check_notion_creds; then
    CHECKS_PASSED=false
fi

echo ""

if [[ "$CHECKS_PASSED" != "true" ]]; then
    echo "Some checks failed. Please fix the issues above before testing."
    echo ""
    echo "To run the integration anyway (will likely fail):"
    echo "  cargo run --release -p scheduler_module --bin inbound_gateway"
    exit 1
fi

echo "=== All checks passed ==="
echo ""

# Ask user what to do
echo "Available actions:"
echo "  1) Run Rust unit tests for Notion module"
echo "  2) Start inbound gateway with Notion polling"
echo "  3) Start worker service"
echo "  4) Exit"
echo ""
read -p "Select action [1-4]: " ACTION

case "$ACTION" in
    1)
        run_rust_tests
        ;;
    2)
        echo "Starting inbound gateway..."
        cd "$PROJECT_ROOT"
        cargo run --release -p scheduler_module --bin inbound_gateway
        ;;
    3)
        echo "Starting worker service..."
        cd "$PROJECT_ROOT"
        cargo run --release -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
        ;;
    *)
        echo "Exiting."
        ;;
esac
