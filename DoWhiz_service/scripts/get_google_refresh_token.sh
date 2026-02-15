#!/bin/bash
#
# Script to get a Google OAuth refresh token with full Google Docs write access.
# Uses localhost redirect URI with a temporary HTTP server.
#
# Prerequisites:
#   1. You need a Google Cloud project with OAuth 2.0 credentials
#   2. Set GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET environment variables
#   3. Add http://localhost:8085 to your OAuth client's Authorized redirect URIs
#      in Google Cloud Console -> APIs & Services -> Credentials
#
# Usage:
#   ./scripts/get_google_refresh_token.sh
#

set -e

# Load .env if exists
if [ -f ../.env ]; then
    source ../.env
elif [ -f .env ]; then
    source .env
fi

if [ -z "$GOOGLE_CLIENT_ID" ] || [ -z "$GOOGLE_CLIENT_SECRET" ]; then
    echo "Error: GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET must be set"
    exit 1
fi

# Configuration
PORT=8085
REDIRECT_URI="http://localhost:${PORT}"

# Required scopes for full Google Docs access
# Note: email scope added to identify the user
SCOPES="https://www.googleapis.com/auth/documents https://www.googleapis.com/auth/drive https://www.googleapis.com/auth/drive.file https://www.googleapis.com/auth/userinfo.email"

# URL encode the scopes
ENCODED_SCOPES=$(python3 -c "import urllib.parse; print(urllib.parse.quote('$SCOPES'))")
ENCODED_REDIRECT=$(python3 -c "import urllib.parse; print(urllib.parse.quote('$REDIRECT_URI'))")

# Generate authorization URL
AUTH_URL="https://accounts.google.com/o/oauth2/v2/auth?client_id=${GOOGLE_CLIENT_ID}&redirect_uri=${ENCODED_REDIRECT}&response_type=code&scope=${ENCODED_SCOPES}&access_type=offline&prompt=consent"

echo "=============================================="
echo "Google OAuth Token Generator"
echo "=============================================="
echo ""
echo "This will get a refresh token with WRITE access to Google Docs."
echo ""
echo "IMPORTANT: Make sure you have added this redirect URI to your"
echo "Google Cloud Console -> APIs & Services -> Credentials -> OAuth 2.0 Client IDs:"
echo ""
echo "  $REDIRECT_URI"
echo ""
echo "Required scopes:"
echo "  - https://www.googleapis.com/auth/documents (Docs read/write)"
echo "  - https://www.googleapis.com/auth/drive (Drive full access)"
echo "  - https://www.googleapis.com/auth/drive.file (Drive file access)"
echo ""
echo "Press Enter to open the authorization URL in your browser..."
read

# Open browser
if command -v xdg-open &> /dev/null; then
    xdg-open "$AUTH_URL" 2>/dev/null &
elif command -v open &> /dev/null; then
    open "$AUTH_URL" &
else
    echo "Could not open browser automatically."
    echo "Please open this URL manually:"
    echo ""
    echo "$AUTH_URL"
    echo ""
fi

echo ""
echo "Waiting for OAuth callback on http://localhost:${PORT}..."
echo "(If your browser didn't open, copy the URL above and paste it in your browser)"
echo ""

# Start a simple HTTP server to catch the callback
# Using Python's http.server module
AUTH_CODE=$(python3 << 'PYEOF'
import http.server
import socketserver
import urllib.parse
import sys

PORT = 8085
code = None

class OAuthHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        global code
        parsed = urllib.parse.urlparse(self.path)
        params = urllib.parse.parse_qs(parsed.query)

        if 'code' in params:
            code = params['code'][0]
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(b'''
                <html>
                <body style="font-family: Arial, sans-serif; text-align: center; padding: 50px;">
                    <h1 style="color: green;">Authorization Successful!</h1>
                    <p>You can close this window and return to the terminal.</p>
                </body>
                </html>
            ''')
        elif 'error' in params:
            error = params.get('error', ['unknown'])[0]
            self.send_response(400)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(f'''
                <html>
                <body style="font-family: Arial, sans-serif; text-align: center; padding: 50px;">
                    <h1 style="color: red;">Authorization Failed</h1>
                    <p>Error: {error}</p>
                </body>
                </html>
            '''.encode())
            code = f"ERROR:{error}"
        else:
            self.send_response(404)
            self.end_headers()
            return

        # Signal to stop the server
        raise KeyboardInterrupt()

    def log_message(self, format, *args):
        pass  # Suppress logging

try:
    with socketserver.TCPServer(("", PORT), OAuthHandler) as httpd:
        httpd.handle_request()
except KeyboardInterrupt:
    pass

if code:
    print(code)
else:
    print("ERROR:no_code")
PYEOF
)

if [[ "$AUTH_CODE" == ERROR:* ]]; then
    echo ""
    echo "Error: Authorization failed - ${AUTH_CODE#ERROR:}"
    exit 1
fi

if [ -z "$AUTH_CODE" ]; then
    echo ""
    echo "Error: No authorization code received"
    exit 1
fi

echo ""
echo "Authorization code received!"
echo "Exchanging for tokens..."
echo ""

# Exchange auth code for refresh token
RESPONSE=$(curl -s -X POST "https://oauth2.googleapis.com/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "client_id=${GOOGLE_CLIENT_ID}" \
    -d "client_secret=${GOOGLE_CLIENT_SECRET}" \
    -d "code=${AUTH_CODE}" \
    -d "grant_type=authorization_code" \
    -d "redirect_uri=${REDIRECT_URI}")

# Check for errors
if echo "$RESPONSE" | grep -q '"error"'; then
    echo "Error: Failed to exchange code for token"
    echo "$RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$RESPONSE"
    exit 1
fi

# Extract tokens
REFRESH_TOKEN=$(echo "$RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('refresh_token', ''))")
ACCESS_TOKEN=$(echo "$RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('access_token', ''))")
SCOPE=$(echo "$RESPONSE" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('scope', ''))")

if [ -z "$REFRESH_TOKEN" ]; then
    echo "Error: No refresh token in response"
    echo "$RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$RESPONSE"
    exit 1
fi

echo "=============================================="
echo "SUCCESS! Token obtained."
echo "=============================================="
echo ""
echo "Granted scopes:"
echo "$SCOPE"
echo ""
echo "Refresh token:"
echo "$REFRESH_TOKEN"
echo ""
echo "=============================================="
echo "Add this to your .env file:"
echo "=============================================="
echo ""
echo "GOOGLE_REFRESH_TOKEN_BOILED_EGG=$REFRESH_TOKEN"
echo ""
echo "=============================================="
echo ""

# Test the token
echo "Testing access token..."
TEST_RESPONSE=$(curl -s "https://www.googleapis.com/drive/v3/files?pageSize=1" \
    -H "Authorization: Bearer $ACCESS_TOKEN")

if echo "$TEST_RESPONSE" | grep -q '"error"'; then
    echo "Warning: Token test failed"
    echo "$TEST_RESPONSE" | python3 -m json.tool 2>/dev/null || echo "$TEST_RESPONSE"
else
    echo "Token test passed - can access Google Drive"
fi

echo ""
echo "To test Google Docs write access, run:"
echo "  GOOGLE_REFRESH_TOKEN=$REFRESH_TOKEN ./bin/google-docs suggest-replace <doc_id> --find='test' --replace='TEST'"
