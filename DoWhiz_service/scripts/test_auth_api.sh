#!/bin/bash
# Test script for the auth API endpoints

set -e

# Load specific environment variables from .env
if [ -f .env ]; then
    SUPABASE_ANON_KEY=$(grep '^SUPABASE_ANON_KEY=' .env | cut -d'=' -f2- | tr -d '"')
    SUPABASE_PROJECT_URL=$(grep '^SUPABASE_PROJECT_URL=' .env | cut -d'=' -f2- | tr -d '"')
fi

# Configuration
SERVICE_URL="${SERVICE_URL:-http://localhost:9001}"
SUPABASE_URL="${SUPABASE_PROJECT_URL:-https://resmseutzmwumflevfqw.supabase.co}"
ANON_KEY="${SUPABASE_ANON_KEY}"

# Use email from argument or default
TEST_EMAIL="${1:-dylantang12@gmail.com}"
TEST_PASSWORD="${2:-testpassword123}"

echo "=== Auth API Test Script ==="
echo "Service URL: $SERVICE_URL"
echo "Supabase URL: $SUPABASE_URL"
echo "Test email: $TEST_EMAIL"
echo ""

# Check if anon key is set
if [ -z "$ANON_KEY" ]; then
    echo "ERROR: SUPABASE_ANON_KEY not set. Please check your .env file."
    exit 1
fi

# Step 1: Try to login first, if fails then signup
echo "1. Logging in with Supabase Auth..."
LOGIN_RESPONSE=$(curl -s -X POST "${SUPABASE_URL}/auth/v1/token?grant_type=password" \
    -H "apikey: ${ANON_KEY}" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"${TEST_EMAIL}\", \"password\": \"${TEST_PASSWORD}\"}")

# Extract access token from login
ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$ACCESS_TOKEN" ]; then
    echo "Login failed, trying signup..."
    SIGNUP_RESPONSE=$(curl -s -X POST "${SUPABASE_URL}/auth/v1/signup" \
        -H "apikey: ${ANON_KEY}" \
        -H "Content-Type: application/json" \
        -d "{\"email\": \"${TEST_EMAIL}\", \"password\": \"${TEST_PASSWORD}\"}")

    echo "Signup response: $SIGNUP_RESPONSE"

    # Check if email confirmation is required
    if echo "$SIGNUP_RESPONSE" | grep -q "confirmation_sent_at"; then
        echo ""
        echo "=========================================="
        echo "EMAIL CONFIRMATION REQUIRED"
        echo "=========================================="
        echo "1. Check your email ($TEST_EMAIL) for confirmation link"
        echo "2. Click the link to confirm your account"
        echo "3. Run this script again to login"
        echo ""
        echo "Or disable email confirmation in Supabase Dashboard:"
        echo "  Auth → Settings → Email Auth → Disable 'Confirm email'"
        exit 0
    fi

    ACCESS_TOKEN=$(echo "$SIGNUP_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
fi

if [ -z "$ACCESS_TOKEN" ]; then
    echo "ERROR: Failed to get access token"
    echo "Login response: $LOGIN_RESPONSE"
    exit 1
fi

echo "Access token obtained: ${ACCESS_TOKEN:0:20}..."
echo ""

# Step 2: Create DoWhiz account
echo "2. Creating DoWhiz account (POST /auth/signup)..."
ACCOUNT_RESPONSE=$(curl -s -X POST "${SERVICE_URL}/auth/signup" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_RESPONSE"
echo ""

# Step 3: Get account info
echo "3. Getting account info (GET /auth/account)..."
ACCOUNT_INFO=$(curl -s "${SERVICE_URL}/auth/account" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_INFO"
echo ""

# Step 4: Link a phone identifier
echo "4. Linking phone identifier (POST /auth/link)..."
LINK_RESPONSE=$(curl -s -X POST "${SERVICE_URL}/auth/link" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" \
    -H "Content-Type: application/json" \
    -d '{"identifier_type": "phone", "identifier": "+14155550100"}')

echo "Response: $LINK_RESPONSE"
echo ""

# Step 5: Get account info again (should show the linked identifier)
echo "5. Getting account info again (should show linked identifier)..."
ACCOUNT_INFO2=$(curl -s "${SERVICE_URL}/auth/account" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_INFO2"
echo ""

# Step 6: Verify the identifier
echo "6. Verifying identifier (POST /auth/verify)..."
VERIFY_RESPONSE=$(curl -s -X POST "${SERVICE_URL}/auth/verify" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" \
    -H "Content-Type: application/json" \
    -d '{"identifier_type": "phone", "identifier": "+14155550100", "code": "123456"}')

echo "Response: $VERIFY_RESPONSE"
echo ""

# Step 7: Get account info again (should show verified)
echo "7. Getting account info (should show verified=true)..."
ACCOUNT_INFO3=$(curl -s "${SERVICE_URL}/auth/account" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_INFO3"
echo ""

# Step 8: Unlink the identifier
echo "8. Unlinking identifier (DELETE /auth/unlink)..."
UNLINK_RESPONSE=$(curl -s -X DELETE "${SERVICE_URL}/auth/unlink" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" \
    -H "Content-Type: application/json" \
    -d '{"identifier_type": "phone", "identifier": "+14155550100"}')

echo "Response: $UNLINK_RESPONSE"
echo ""

# Step 9: Final account info (should have no identifiers)
echo "9. Final account info (should have no identifiers)..."
ACCOUNT_INFO_FINAL=$(curl -s "${SERVICE_URL}/auth/account" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_INFO_FINAL"
echo ""

echo "=== Test Complete ==="
