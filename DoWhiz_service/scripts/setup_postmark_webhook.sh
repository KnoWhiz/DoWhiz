#!/usr/bin/env bash
set -euo pipefail

# Setup Postmark inbound webhook with ngrok
# Usage: ./scripts/setup_postmark_webhook.sh [--port 9001]

PORT="${1:-9001}"
POSTMARK_TOKEN="${POSTMARK_SERVER_TOKEN:-7a14b884-2e10-440b-81d2-62b5aaa725ca}"

echo "=== Postmark Webhook Setup ==="
echo ""

# Check if ngrok is already running
if curl -s http://127.0.0.1:4040/api/tunnels >/dev/null 2>&1; then
    echo "Ngrok is already running. Getting existing URL..."
else
    echo "Starting ngrok on port $PORT..."
    ngrok http "$PORT" --log=stdout > /tmp/ngrok-dowhiz.log 2>&1 &
    NGROK_PID=$!
    echo "Ngrok started (PID: $NGROK_PID)"
    echo "Waiting for ngrok to initialize..."
    sleep 3
fi

# Get ngrok public URL
echo ""
echo "Fetching ngrok public URL..."
NGROK_URL=$(curl -s http://127.0.0.1:4040/api/tunnels | python3 -c "
import sys, json
data = json.load(sys.stdin)
for tunnel in data.get('tunnels', []):
    url = tunnel.get('public_url', '')
    if url.startswith('https://'):
        print(url)
        break
" 2>/dev/null)

if [[ -z "$NGROK_URL" ]]; then
    echo "ERROR: Failed to get ngrok URL. Check if ngrok is running properly."
    echo "Try running: ngrok http $PORT"
    exit 1
fi

WEBHOOK_URL="${NGROK_URL}/postmark/inbound"
echo ""
echo "Ngrok URL: $NGROK_URL"
echo "Webhook URL: $WEBHOOK_URL"

# Update Postmark webhook
echo ""
echo "Updating Postmark inbound webhook..."
RESPONSE=$(curl -s -X PUT "https://api.postmarkapp.com/server" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "X-Postmark-Server-Token: $POSTMARK_TOKEN" \
    -d "{\"InboundHookUrl\": \"$WEBHOOK_URL\"}")

# Check result
if echo "$RESPONSE" | grep -q "InboundHookUrl"; then
    NEW_URL=$(echo "$RESPONSE" | python3 -c "import sys, json; print(json.load(sys.stdin).get('InboundHookUrl', 'unknown'))")
    echo ""
    echo "=== SUCCESS ==="
    echo "Postmark webhook updated to: $NEW_URL"
    echo ""
    echo "You can now send emails to proto@dowhiz.com"
    echo "The service should receive them at: http://localhost:$PORT/postmark/inbound"
    echo ""
    echo "To monitor ngrok traffic: http://127.0.0.1:4040"
else
    echo ""
    echo "ERROR: Failed to update Postmark webhook"
    echo "Response: $RESPONSE"
    exit 1
fi
