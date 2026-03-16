#!/usr/bin/env bash
# 清理 browser-use 和轮询服务
# 用于测试后快速清理所有浏览器自动化进程

set -e

echo "清理浏览器自动化服务..."
echo ""

# 计数器
KILLED=0

# Notion Poller
if pkill -9 -f "notion_poller" 2>/dev/null; then
    echo "✓ Notion Poller 已停止"
    ((KILLED++))
fi

# Google Docs Poller
if pkill -9 -f "google_docs_poller" 2>/dev/null; then
    echo "✓ Google Docs Poller 已停止"
    ((KILLED++))
fi

# Browser-use server
if pkill -9 -f "browser_use.skill_cli.server" 2>/dev/null; then
    echo "✓ Browser-use server 已停止"
    ((KILLED++))
fi

# 等待进程退出
sleep 1

# Browser-use 启动的 Chrome/Chromium
if pkill -9 -f "chrome.*browser-use-user-data-dir" 2>/dev/null; then
    echo "✓ Browser-use Chrome 已停止"
    ((KILLED++))
fi

if pkill -9 -f "chromium.*browser-use" 2>/dev/null; then
    echo "✓ Browser-use Chromium 已停止"
    ((KILLED++))
fi

# Chrome crashpad handlers (orphaned)
pkill -9 -f "chrome_crashpad.*browser-use" 2>/dev/null || true

echo ""
if [ $KILLED -gt 0 ]; then
    echo "已清理 $KILLED 个服务"
else
    echo "没有发现运行中的浏览器自动化服务"
fi

# 验证清理结果
echo ""
echo "验证清理结果..."
REMAINING=$(ps aux | grep -E "(notion_poller|browser_use|chrome.*browser-use)" | grep -v grep | wc -l)
if [ "$REMAINING" -eq 0 ]; then
    echo "✓ 所有浏览器自动化服务已清理完毕"
else
    echo "⚠ 仍有 $REMAINING 个相关进程在运行:"
    ps aux | grep -E "(notion_poller|browser_use|chrome.*browser-use)" | grep -v grep
fi
