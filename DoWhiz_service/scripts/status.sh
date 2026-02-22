#!/usr/bin/env bash
# DoWhiz 服务状态检查

SERVICE_DIR="/home/liuxt/deeptutor/DoWhiz/DoWhiz_service"
DB_PATH="$SERVICE_DIR/.workspace/boiled_egg/state/ingestion.db"

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
echo "=== 最近 5 条消息 ==="
if [[ -f "$DB_PATH" ]]; then
    sqlite3 -header -column "$DB_PATH" \
        "SELECT
            substr(id, 1, 8) as id,
            status,
            attempts,
            substr(created_at, 12, 8) as time,
            CASE WHEN last_error IS NULL THEN '-' ELSE substr(last_error, 1, 30) END as error
         FROM ingestion_queue
         ORDER BY created_at DESC
         LIMIT 5;" 2>/dev/null || echo "无法读取数据库"
else
    echo "数据库不存在"
fi

# 4. 正在执行的任务
echo ""
echo "=== 任务队列 ==="
TASK_DB="$SERVICE_DIR/.workspace/boiled_egg/state/task_index.db"
if [[ -f "$TASK_DB" ]]; then
    COUNT=$(sqlite3 "$TASK_DB" "SELECT COUNT(*) FROM task_index WHERE enabled=1;" 2>/dev/null || echo "0")
    echo "待执行任务数: $COUNT"
    if [[ "$COUNT" -gt 0 ]]; then
        sqlite3 -header -column "$TASK_DB" \
            "SELECT substr(task_id, 1, 8) as task_id, substr(next_run, 12, 8) as next_run, enabled
             FROM task_index LIMIT 5;" 2>/dev/null
    fi
else
    echo "任务数据库不存在"
fi

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
