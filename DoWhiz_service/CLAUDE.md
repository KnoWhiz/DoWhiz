# Claude Code Instructions for DoWhiz

## IMPORTANT: Read This First

Before starting any task related to DoWhiz deployment or debugging, read `OPERATIONS.md` in this directory for:
- Azure VM paths and PM2 commands
- Common issues and solutions
- Recent optimizations

## Quick Reference

### Azure VM Info
- **Server root**: `/home/azureuser/server/`
- **PM2 logs**: `/home/azureuser/server/.pm2/logs/`
- **PM2 requires**: `export HOME=/home/azureuser/server`

### Common Issues

#### 1. Azure Service Bus Enqueue Error
```
gateway enqueue error: service bus error: HttpResponse(400,unknown)
```
**Cause**: Service Bus connection string misconfigured or expired
**Fix**: Check `AZURE_SERVICE_BUS_CONNECTION_STRING` in `.env`

#### 2. "missing policy name in connection string"
**Cause**: Azure Service Bus SAS policy not specified
**Fix**: Connection string format should be:
```
Endpoint=sb://<namespace>.servicebus.windows.net/;SharedAccessKeyName=<policy>;SharedAccessKey=<key>
```

#### 3. Slides/Sheets comments not processed
**Check order**:
1. Is `GOOGLE_SLIDES_ENABLED=true` in `.env`?
2. Are there Service Bus errors in logs?
3. Is `scheduler_user_max_concurrency` causing blocking?
4. Check `google_workspace_processed.db` for already-processed comments

#### 4. 20+ minute delay for tasks
**Root cause**: `SCHEDULER_USER_MAX_CONCURRENCY=1`
- Only one task per user runs at a time
- Affects ALL channels (Email, Slack, Discord, Google Docs/Sheets/Slides)
- Team discussion needed before increasing

### Diagnostic Commands for VM

```bash
# Check PM2 status
HOME=/home/azureuser/server pm2 list

# Check recent errors
tail -100 /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | grep -i error

# Check Slides polling
grep -i "slides|presentation" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -30

# Check Service Bus errors
grep -i "service bus|enqueue" /home/azureuser/server/.pm2/logs/dowhiz-inbound-gateway-out.log | tail -20
```

---

## Session Notes (Update After Each Session)

### 2026-02-25: Slides Debug Session

**Problem**: User's Slides comments not getting replies

**Root Cause Found**: Azure Service Bus enqueue failing with `HttpResponse(400,unknown)`
- Comments are being detected but cannot be queued for processing
- Need to fix Service Bus connection string configuration

**Files Modified**:
- `scheduler_module/src/adapters/google_common/comments.rs` - Added timeout + retry
- `scheduler_module/src/google_workspace_poller.rs` - Added file list cache, reduced polling to 15s
- `scheduler_module/src/bin/inbound_gateway/google_workspace.rs` - Parallelized Sheets/Slides polling
- `scheduler_module/src/google_drive_changes.rs` - New file for future push notifications
- `OPERATIONS.md` - Created deployment guide
- `CLAUDE.md` - This file

**Commits**:
- `294c0bd` - Optimize Google Workspace polling for reduced latency
- `bfdbabf` - Add OPERATIONS.md for Azure VM deployment guide

**Next Steps**:
1. Fix Azure Service Bus connection string on VM
2. Deploy new code after testing locally
3. Consider increasing `SCHEDULER_USER_MAX_CONCURRENCY` (needs team discussion)

---

## Code Patterns

### Google Workspace Comment Flow
```
inbound_gateway/google_workspace.rs
  └─> spawn_google_workspace_poller()
       └─> poll_workspace_comments() [every 15s]
            └─> GoogleWorkspacePoller.poll_sheets() / poll_slides()
                 └─> GoogleCommentsClient.list_comments()
                      └─> filter_actionable_comments()
                           └─> resolve_route()
                                └─> state.queue.enqueue()  <-- THIS IS FAILING
```

### Key Environment Variables
- `GOOGLE_DOCS_ENABLED` / `GOOGLE_SHEETS_ENABLED` / `GOOGLE_SLIDES_ENABLED`
- `GOOGLE_WORKSPACE_POLL_INTERVAL_SECS` (default: 15)
- `SCHEDULER_USER_MAX_CONCURRENCY` (default: 1)
- `GOOGLE_DRIVE_PUSH_ENABLED` (future feature)
- `AZURE_SERVICE_BUS_CONNECTION_STRING`
