#!/usr/bin/env bash
# DoWhiz 服务停止脚本

echo "停止 DoWhiz 服务..."

# 核心服务
pkill -f "inbound_gateway" 2>/dev/null && echo "✓ Gateway 已停止" || echo "- Gateway 未运行"
pkill -f "rust_service" 2>/dev/null && echo "✓ Worker 已停止" || echo "- Worker 未运行"
pkill -f "ngrok http" 2>/dev/null && echo "✓ Ngrok 已停止" || echo "- Ngrok 未运行"

# 轮询服务
pkill -9 -f "notion_poller" 2>/dev/null && echo "✓ Notion Poller 已停止" || echo "- Notion Poller 未运行"
pkill -9 -f "google_docs_poller" 2>/dev/null && echo "✓ Google Docs Poller 已停止" || echo "- Google Docs Poller 未运行"

# Browser-use 及相关浏览器进程
pkill -9 -f "browser_use.skill_cli.server" 2>/dev/null && echo "✓ Browser-use 已停止" || echo "- Browser-use 未运行"
pkill -9 -f "chrome.*browser-use-user-data-dir" 2>/dev/null && echo "✓ Browser-use Chrome 已停止" || echo "- Browser-use Chrome 未运行"
pkill -9 -f "chromium.*browser-use" 2>/dev/null && echo "✓ Browser-use Chromium 已停止" || echo "- Browser-use Chromium 未运行"

echo ""
echo "所有服务已停止"
