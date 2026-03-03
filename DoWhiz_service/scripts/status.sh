#!/usr/bin/env bash
# DoWhiz 服务状态检查

SERVICE_DIR="/home/liuxt/deeptutor/DoWhiz/DoWhiz_service"

echo "=========================================="
echo "       DoWhiz 服务状态检查"
echo "=========================================="
echo ""

# 1. 进程状态
echo "=== 进程状态 ==="
if pgrep -f "inbound_gateway" > /dev/null; then
    echo "✅ Gateway: 运行中 (PID: $(pgrep -f inbound_gateway))"
else
    echo "❌ Gateway: 未运行"
fi

if pgrep -f "rust_service" > /dev/null; then
    echo "✅ Worker:  运行中 (PID: $(pgrep -f rust_service))"
else
    echo "❌ Worker:  未运行"
fi

if pgrep -f "ngrok http" > /dev/null; then
    NGROK_URL=$(curl -s http://127.0.0.1:4040/api/tunnels 2>/dev/null | python3 -c "import sys,json; print([t['public_url'] for t in json.load(sys.stdin).get('tunnels',[]) if t.get('public_url','').startswith('https')][0])" 2>/dev/null || echo "无法获取")
    echo "✅ Ngrok:   运行中 → $NGROK_URL"
else
    echo "❌ Ngrok:   未运行"
fi

# 2. 健康检查
echo ""
echo "=== 健康检查 ==="
curl -s http://localhost:9100/health > /dev/null 2>&1 && echo "✅ Gateway (9100): OK" || echo "❌ Gateway (9100): 无响应"
curl -s http://localhost:9001/health > /dev/null 2>&1 && echo "✅ Worker  (9001): OK" || echo "❌ Worker  (9001): 无响应"

# 3. 队列状态
echo ""
echo "=== 队列与存储 ==="
echo "当前版本默认使用 MongoDB + Service Bus/PG 队列，不再读取本地 state DB 文件。"
echo "请通过日志、Azure Portal 或对应数据库工具检查消息与任务状态。"

# 4. 正在执行的任务
echo ""
echo "=== 任务队列 ==="
echo "任务状态已迁移到 MongoDB（collections: tasks, task_executions, task_index）。"
echo "可通过服务 API 或 Mongo shell 查询。"

# 5. 最新日志
echo ""
echo "=== Worker 最新日志 (最后 5 行) ==="
tail -5 "$SERVICE_DIR/worker.log" 2>/dev/null | sed 's/\x1b\[[0-9;]*m//g' || echo "无日志"

echo ""
echo "=========================================="
echo "监控命令："
echo "  实时日志: tail -f $SERVICE_DIR/worker.log"
echo "  Ngrok:    http://127.0.0.1:4040"
echo "=========================================="
