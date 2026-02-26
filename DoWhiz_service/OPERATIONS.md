# DoWhiz Operations Guide

## Azure VM Deployment Info

### Paths
- **Server root**: `/home/azureuser/server/`
- **DoWhiz Service**: `/home/azureuser/server/DoWhiz_service/`
- **PM2 logs**: `/home/azureuser/server/.pm2/logs/`
  - `dowhiz-rust-service-out.log` - Rust service stdout
  - `dowhiz-rust-service-error.log` - Rust service errors
  - `dowhiz-inbound-gateway-out.log` - Inbound gateway logs
- **Processed comments DB**: `/home/azureuser/server/DoWhiz_service/.workspace/*/run_task/google_workspace_processed.db`
- **Codex logs**: `/home/azureuser/server/.codex/log/codex-tui.log`

### PM2 Commands
```bash
# PM2 requires HOME to be set correctly
export HOME=/home/azureuser/server

# List services
pm2 list

# View logs
pm2 logs dowhiz-rust-service
pm2 logs dowhiz-inbound-gateway

# Restart services
pm2 restart dowhiz-rust-service
pm2 restart dowhiz-inbound-gateway

# Restart all
pm2 restart all
```

### Common Diagnostic Commands

#### Check if services are running
```bash
ps aux | grep -E "inbound|rust_service|dowhiz" | grep -v grep
```

#### Check Google Slides issues
```bash
# Check if Slides poller is working
grep -i "slides\|presentation" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -50

# Check for "no route" errors (common issue)
grep -i "no route" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -20

# Check processed Slides comments
sqlite3 /home/azureuser/server/DoWhiz_service/.workspace/*/run_task/google_workspace_processed.db \
  "SELECT * FROM google_workspace_processed_comments WHERE file_type='slides' ORDER BY processed_at DESC LIMIT 10;"
```

#### Check for errors
```bash
tail -100 /home/azureuser/server/.pm2/logs/dowhiz-rust-service-error.log
grep -i "error\|failed" /home/azureuser/server/.pm2/logs/dowhiz-rust-service-out.log | tail -50
```

#### Check environment variables
```bash
cat /home/azureuser/server/DoWhiz_service/.env | grep -i "SLIDES\|SHEETS\|GOOGLE"
```

---

## Known Issues & Fixes

### Issue 1: Slides comment not receiving reply
**Symptoms**: User comments on Google Slides mentioning @oliver but no reply is received.

**Possible causes**:
1. **No route configured** - The Slides file_id is not registered in the routing system
   - Check: `grep "no route" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log`

2. **Comment already processed** - The comment ID is in the processed_comments database
   - Check: Query the `google_workspace_processed_comments` table

3. **Mention not detected** - The mention pattern doesn't match
   - The system looks for: proto, oliver, maggie, little-bear, @proto, etc.

4. **Service not running** - PM2 service crashed
   - Check: `HOME=/home/azureuser/server pm2 list`

5. **scheduler_user_max_concurrency = 1** - Task blocked by another running task
   - This affects ALL channels (Email, Slack, Discord, Google Docs/Sheets/Slides)
   - If a Docs task is running, Slides task waits

### Issue 2: Long delay (20+ minutes) for Slides while Docs works
**Root cause**: `scheduler_user_max_concurrency = 1` means only one task per user runs at a time.

**Fix**: Increase `SCHEDULER_USER_MAX_CONCURRENCY` to 2 or 3 in `.env`
- **Warning**: This affects ALL channels, discuss with team first

### Issue 3: 5+ minute response time
**Breakdown**:
- Polling delay: ~15-30 seconds (reduced from 30s to 15s)
- Azure OpenAI API: 2-3 minutes (gpt-5.2-codex)
- Web search (if enabled): 30-60 seconds
- Google API calls: 10-30 seconds

---

## Recent Optimizations (Feb 2026)

1. **Parallel polling** - Sheets and Slides now poll in separate threads
2. **HTTP timeout + retry** - 30s timeout, 3 retries with exponential backoff
3. **Polling interval reduced** - 30s → 15s default
4. **File list cache** - 5 minute TTL to reduce API calls
5. **Google Drive Change API** - Foundation added (not yet enabled)

### Environment Variables for Tuning
```bash
# Polling interval (default: 15 seconds)
GOOGLE_WORKSPACE_POLL_INTERVAL_SECS=15

# Enable Sheets/Slides
GOOGLE_SHEETS_ENABLED=true
GOOGLE_SLIDES_ENABLED=true

# User concurrency (affects all channels!)
SCHEDULER_USER_MAX_CONCURRENCY=1

# Push notifications (future)
GOOGLE_DRIVE_PUSH_ENABLED=false
GOOGLE_DRIVE_WEBHOOK_URL=https://your-domain.com/webhooks/google-drive-changes
```

---

## Deployment Notes

### Do NOT restart without team coordination
The VM is shared and runs production services. Avoid:
- `pm2 restart all` without warning
- Kernel upgrades that require reboot
- Any destructive git operations

### After code changes
1. Build locally: `cargo build --release`
2. Test locally with `CODEX_DISABLED=1`
3. Create PR to main
4. On VM: `git pull && cargo build --release && pm2 restart all`

---

## Contact

For issues, check:
1. This document
2. PM2 logs
3. GitHub issues: https://github.com/KnoWhiz/DoWhiz/issues
