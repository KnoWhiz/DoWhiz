#!/usr/bin/env bash
# DoWhiz 服务停止脚本

echo "停止 DoWhiz 服务..."

pkill -f "inbound_gateway" 2>/dev/null && echo "✓ Gateway 已停止" || echo "- Gateway 未运行"
pkill -f "rust_service" 2>/dev/null && echo "✓ Worker 已停止" || echo "- Worker 未运行"
pkill -f "ngrok http" 2>/dev/null && echo "✓ Ngrok 已停止" || echo "- Ngrok 未运行"

echo ""
echo "所有服务已停止"
