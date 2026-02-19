#!/bin/bash
# Test script that links an identifier without unlinking
# So you can inspect the database afterwards

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
TEST_PHONE="${3:-+14155551234}"

echo "=== Auth API Link Test (No Unlink) ==="
echo "Service URL: $SERVICE_URL"
echo "Supabase URL: $SUPABASE_URL"
echo "Test email: $TEST_EMAIL"
echo "Test phone: $TEST_PHONE"
echo ""

# Check if anon key is set
if [ -z "$ANON_KEY" ]; then
    echo "ERROR: SUPABASE_ANON_KEY not set. Please check your .env file."
    exit 1
fi

# Step 1: Login with Supabase
echo "1. Logging in with Supabase Auth..."
LOGIN_RESPONSE=$(curl -s -X POST "${SUPABASE_URL}/auth/v1/token?grant_type=password" \
    -H "apikey: ${ANON_KEY}" \
    -H "Content-Type: application/json" \
    -d "{\"email\": \"${TEST_EMAIL}\", \"password\": \"${TEST_PASSWORD}\"}")

ACCESS_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$ACCESS_TOKEN" ]; then
    echo "ERROR: Failed to get access token"
    echo "Response: $LOGIN_RESPONSE"
    exit 1
fi

echo "Access token obtained: ${ACCESS_TOKEN:0:20}..."
echo ""

# Step 2: Get or create DoWhiz account
echo "2. Creating/Getting DoWhiz account (POST /auth/signup)..."
ACCOUNT_RESPONSE=$(curl -s -X POST "${SERVICE_URL}/auth/signup" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_RESPONSE"
echo ""

# Step 3: Link a phone identifier
echo "3. Linking phone identifier: $TEST_PHONE (POST /auth/link)..."
LINK_RESPONSE=$(curl -s -X POST "${SERVICE_URL}/auth/link" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" \
    -H "Content-Type: application/json" \
    -d "{\"identifier_type\": \"phone\", \"identifier\": \"${TEST_PHONE}\"}")

echo "Response: $LINK_RESPONSE"
echo ""

# Step 4: Verify the identifier
echo "4. Verifying identifier (POST /auth/verify)..."
VERIFY_RESPONSE=$(curl -s -X POST "${SERVICE_URL}/auth/verify" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}" \
    -H "Content-Type: application/json" \
    -d "{\"identifier_type\": \"phone\", \"identifier\": \"${TEST_PHONE}\", \"code\": \"123456\"}")

echo "Response: $VERIFY_RESPONSE"
echo ""

# Step 5: Get final account info
echo "5. Final account info..."
ACCOUNT_INFO=$(curl -s "${SERVICE_URL}/auth/account" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

echo "Response: $ACCOUNT_INFO"
echo ""

echo "=== Test Complete ==="
echo ""
echo "Now check Supabase Dashboard:"
echo "  1. Go to Table Editor"
echo "  2. Look at 'accounts' table - should see your account"
echo "  3. Look at 'account_identifiers' table - should see the linked phone"
