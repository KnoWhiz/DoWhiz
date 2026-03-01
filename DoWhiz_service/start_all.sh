#!/bin/bash
# DoWhiz 一键启动脚本

cd /home/liuxt/deeptutor/DoWhiz/DoWhiz_service
source ../.env

echo "=== DoWhiz Service Startup ==="

# 停止已有服务
echo "[1/4] Stopping existing services..."
pkill -f "rust_service" 2>/dev/null
pkill -f "inbound_gateway" 2>/dev/null
pkill -f "cloudflared" 2>/dev/null
sleep 2

# 启动 Cloudflare Tunnel
echo "[2/4] Starting Cloudflare Tunnel..."
~/bin/cloudflared tunnel --url http://localhost:9100 > cloudflared.log 2>&1 &
sleep 3

# 获取 Cloudflare URL
CF_URL=$(grep -o 'https://[^[:space:]]*\.trycloudflare\.com' cloudflared.log | head -1)
if [ -n "$CF_URL" ]; then
    echo "    Tunnel URL: $CF_URL"

    # 自动更新 Postmark Webhook
    if [ -n "$POSTMARK_SERVER_TOKEN" ]; then
        echo "    Updating Postmark webhook..."
        WEBHOOK_URL="${CF_URL}/inbound"
        RESPONSE=$(curl -s -X PUT "https://api.postmarkapp.com/server" \
            -H "Accept: application/json" \
            -H "Content-Type: application/json" \
            -H "X-Postmark-Server-Token: $POSTMARK_SERVER_TOKEN" \
            -d "{\"InboundHookUrl\": \"$WEBHOOK_URL\"}")

        if echo "$RESPONSE" | grep -q "InboundHookUrl"; then
            echo "    ✓ Postmark webhook updated to: $WEBHOOK_URL"
        else
            echo "    ✗ Failed to update Postmark webhook"
            echo "    Response: $RESPONSE"
        fi
    else
        echo "    Warning: POSTMARK_SERVER_TOKEN not set, skipping webhook update"
    fi
else
    echo "    Warning: Could not get tunnel URL, check cloudflared.log"
fi

# 启动 Inbound Gateway
echo "[3/4] Starting Inbound Gateway on port 9100..."
cargo run --release --bin inbound_gateway > inbound_gateway.log 2>&1 &
sleep 3

# 启动 Rust Service
echo "[4/4] Starting Rust Service on port 9001..."
cargo run --release --bin rust_service -- --host 0.0.0.0 --port 9001 > rust_service.log 2>&1 &
sleep 2

# 检查状态
echo ""
echo "=== Service Status ==="
if pgrep -f "rust_service" > /dev/null; then
    echo "✓ rust_service: Running"
else
    echo "✗ rust_service: Failed to start"
fi

if pgrep -f "inbound_gateway" > /dev/null; then
    echo "✓ inbound_gateway: Running"
else
    echo "✗ inbound_gateway: Failed to start"
fi

if pgrep -f "cloudflared" > /dev/null; then
    echo "✓ cloudflared: Running"
else
    echo "✗ cloudflared: Failed to start"
fi

echo ""
echo "=== Logs ==="
echo "View logs: tail -f rust_service.log inbound_gateway.log"
echo ""
echo "Ready for testing!"
