#!/usr/bin/env bash
set -euo pipefail

# DoWhiz 服务一键启动脚本
# 启动顺序：Gateway → Worker → Ngrok → 更新 Postmark Webhook

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# 加载环境变量，并根据 DEPLOY_TARGET 应用 STAGING_* 覆盖
# shellcheck source=./load_env_target.sh
source "${SCRIPT_DIR}/load_env_target.sh"
echo "✓ 已加载环境变量: ${ENV_FILE:-<shell environment only>}"
echo "✓ DEPLOY_TARGET=${DEPLOY_TARGET:-production}"

GATEWAY_CONFIG_PATH="${GATEWAY_CONFIG_PATH:-gateway.toml}"
GATEWAY_HOST="${GATEWAY_HOST:-0.0.0.0}"
GATEWAY_PORT="${GATEWAY_PORT:-9100}"
RUST_SERVICE_HOST="${RUST_SERVICE_HOST:-0.0.0.0}"
RUST_SERVICE_PORT="${RUST_SERVICE_PORT:-9001}"
EMPLOYEE_ID="${EMPLOYEE_ID:-little_bear}"

cd "$SERVICE_DIR"

# 检查二进制文件
if [[ ! -f "target/debug/inbound_gateway" ]] || [[ ! -f "target/debug/rust_service" ]]; then
    echo "编译服务..."
    cargo build
fi

# 停止已有进程
echo "停止已有进程..."
pkill -f "inbound_gateway" 2>/dev/null || true
pkill -f "rust_service" 2>/dev/null || true
pkill -f "ngrok http" 2>/dev/null || true
sleep 2

# 1. 启动 Gateway
echo ""
echo "=== 1. 启动 Gateway (端口 ${GATEWAY_PORT}) ==="
export GATEWAY_CONFIG_PATH
export GATEWAY_HOST
export GATEWAY_PORT
./target/debug/inbound_gateway > gateway.log 2>&1 &
GATEWAY_PID=$!
sleep 2

if curl -s "http://localhost:${GATEWAY_PORT}/health" | grep -q "ok"; then
    echo "✓ Gateway 启动成功 (PID: $GATEWAY_PID)"
else
    echo "✗ Gateway 启动失败，查看 gateway.log"
    exit 1
fi

# 2. 启动 Worker
echo ""
echo "=== 2. 启动 Worker (端口 ${RUST_SERVICE_PORT}, employee=${EMPLOYEE_ID}) ==="
EMPLOYEE_ID="${EMPLOYEE_ID}" RUST_SERVICE_PORT="${RUST_SERVICE_PORT}" \
  ./target/debug/rust_service --host "${RUST_SERVICE_HOST}" --port "${RUST_SERVICE_PORT}" > worker.log 2>&1 &
WORKER_PID=$!
sleep 2

if curl -s "http://localhost:${RUST_SERVICE_PORT}/health" | grep -q "ok"; then
    echo "✓ Worker 启动成功 (PID: $WORKER_PID)"
else
    echo "✗ Worker 启动失败，查看 worker.log"
    exit 1
fi

# 3. 启动 Ngrok
echo ""
echo "=== 3. 启动 Ngrok ==="
ngrok http "${GATEWAY_PORT}" --log=stdout > /tmp/ngrok.log 2>&1 &
NGROK_PID=$!
echo "Ngrok 启动中 (PID: $NGROK_PID)..."
sleep 4

# 获取 Ngrok URL
NGROK_URL=$(curl -s http://127.0.0.1:4040/api/tunnels 2>/dev/null | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    for tunnel in data.get('tunnels', []):
        url = tunnel.get('public_url', '')
        if url.startswith('https://'):
            print(url)
            break
except:
    pass
" 2>/dev/null)

if [[ -z "$NGROK_URL" ]]; then
    echo "✗ 无法获取 Ngrok URL"
    exit 1
fi
echo "✓ Ngrok URL: $NGROK_URL"

# 4. 更新 Postmark Webhook
echo ""
echo "=== 4. 更新 Postmark Webhook ==="
WEBHOOK_URL="${NGROK_URL}/postmark/inbound"

if [[ -z "${POSTMARK_SERVER_TOKEN:-}" ]]; then
    echo "✗ 缺少 POSTMARK_SERVER_TOKEN（当前 DEPLOY_TARGET=${DEPLOY_TARGET:-production}）"
    exit 1
fi

RESPONSE=$(curl -s -X PUT "https://api.postmarkapp.com/server" \
    -H "Accept: application/json" \
    -H "Content-Type: application/json" \
    -H "X-Postmark-Server-Token: ${POSTMARK_SERVER_TOKEN}" \
    -d "{\"InboundHookUrl\": \"$WEBHOOK_URL\"}")

if echo "$RESPONSE" | grep -q "InboundHookUrl"; then
    echo "✓ Postmark Webhook 已更新: $WEBHOOK_URL"
else
    echo "✗ Postmark 更新失败: $RESPONSE"
    exit 1
fi

# 完成
echo ""
echo "=========================================="
echo "✓ DoWhiz 服务启动完成！"
echo "=========================================="
echo ""
echo "组件状态："
echo "  Gateway:  http://localhost:${GATEWAY_PORT} (PID: $GATEWAY_PID)"
echo "  Worker:   http://localhost:${RUST_SERVICE_PORT} (PID: $WORKER_PID)"
echo "  Ngrok:    $NGROK_URL (PID: $NGROK_PID)"
echo "  Postmark: $WEBHOOK_URL"
echo ""
echo "日志文件："
echo "  Gateway: $SERVICE_DIR/gateway.log"
echo "  Worker:  $SERVICE_DIR/worker.log"
echo "  Ngrok:   /tmp/ngrok.log"
echo ""
echo "监控面板："
echo "  Ngrok:   http://127.0.0.1:4040"
echo ""
if [[ "${DEPLOY_TARGET:-production}" == "staging" ]]; then
  echo "测试发送邮件到: dowhiz@deep-tutor.com"
else
  echo "测试发送邮件到: proto@dowhiz.com 或 boiled-egg@dowhiz.com"
fi
echo ""
echo "停止所有服务: pkill -f 'inbound_gateway|rust_service|ngrok'"
