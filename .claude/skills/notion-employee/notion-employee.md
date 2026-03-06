# Notion Employee Integration Skill

This skill enables digital employees (Oliver, Maggie, etc.) to interact with Notion as real team members - receiving @mentions, replying to comments, and editing pages.

## Employee Accounts

| Employee | Email | Notion Display Name |
|----------|-------|---------------------|
| Oliver (little_bear) | agent@dowhiz.com | Oliver |
| Maggie (mini_mouse) | maggie@dowhiz.com | Maggie |

Password: Stored in environment variable `NOTION_EMPLOYEE_PASSWORD`

## Workflow Overview

```
1. Login to Notion (Google OAuth)
2. Check Inbox for @mentions
3. For each mention:
   a. Navigate to the page/comment
   b. Read context
   c. Reply or edit as requested
4. Wait and repeat
```

## Login Flow

### CRITICAL: Cookie-First Authentication

**NEVER attempt automated Google OAuth login unless absolutely necessary.** Frequent automated login attempts will trigger Google security mechanisms and may block the account.

**Authentication Priority:**
1. **Always try cookies first** - Import from `~/.dowhiz/notion/cookies.json`
2. **If cookies work** - Continue with polling, no login needed
3. **If cookies expired** - Request manual login (see below)
4. **Only if NOTION_FORCE_LOGIN=true** - Attempt automated Google OAuth

### Manual Login Procedure (Preferred)

When cookies expire, use the manual login helper:

```bash
# Run the manual login script
./DoWhiz_service/scripts/notion_manual_login.sh

# Or manually with browser-use:
IN_DOCKER=true browser-use --session notion_manual_login --browser chromium --headed open "https://www.notion.so/login"
# Complete login manually in the browser window
# Then export cookies:
IN_DOCKER=true browser-use --session notion_manual_login cookies export ~/.dowhiz/notion/cookies.json
IN_DOCKER=true browser-use --session notion_manual_login close
```

### Session Persistence
- Cookies saved at: `~/.dowhiz/notion/cookies.json`
- Cookies typically last 30 days
- Poller automatically exports cookies after successful login

### Rate Limiting Guidelines (STRICT)
- **NEVER** retry failed logins within 5 minutes
- **NEVER** attempt more than 3 automated logins per day
- Wait at least 30-60 seconds between poll cycles
- Use human-like delays (1-3 seconds) between browser actions
- If login fails with "500 error", wait 24 hours before retrying

## Inbox Navigation

### Opening Inbox
1. From any Notion page, click "Inbox" in the left sidebar
2. Inbox panel slides open showing notifications
3. Notifications are grouped by time: "This week", "Earlier"

### Notification Types
- **Comment mentions**: "@Oliver: [message]" - requires response
- **Page mentions**: "mentioned you in [Page]" - informational
- **Task assignments**: "assigned you to [Task]" - action needed

### Identifying @mentions
Look for notifications containing:
- Employee name after "@" symbol
- "commented in" or "mentioned you"
- Clickable page name

## Replying to Comments

### Flow
1. Click on the notification to open the page
2. The comment thread should be visible/highlighted
3. Find the "Reply" input field in the thread
4. Type the response
5. Click "Send" or press Enter

### Expected Page Structure
```
Page Content
├── Block with comment indicator
│   └── Comment thread (expanded)
│       ├── Original comment: "@Oliver please review"
│       ├── [Reply input field]
│       └── [Send button]
```

## Editing Pages

### Flow
1. Navigate to the target page
2. Click on the block to edit (contenteditable)
3. Type new content
4. Click outside or press Escape to save

### Common Editable Elements
- Text blocks: Direct click and type
- Titles: Click on page title to edit
- Databases: Click cell to edit
- Toggle lists: Click arrow to expand, then edit

## Multi-Workspace Support

Oliver may be invited to multiple workspaces. **The poller automatically checks all workspaces:**

### Automatic Polling (Implemented in `poller.rs`)
```
For each poll cycle:
1. List all accessible workspaces
2. For each workspace:
   a. Switch to workspace
   b. Open Inbox
   c. Parse @mentions
   d. Filter already-processed notifications
   e. Navigate to each mention and extract context
3. Enqueue all new mentions for processing
```

### Manual Workspace Navigation (for debugging)
1. Click workspace name (top-left corner)
2. Workspace dropdown shows all available workspaces
3. Switch to each workspace and check Inbox
4. Return to primary workspace when done

## Error Handling

### Login Errors
- "500 error from Google": Account may be rate-limited, wait 5+ minutes
- "Invalid credentials": Check password in env vars
- "Unusual activity": May require manual verification

### Navigation Errors
- Element not found: Page may not have loaded, wait and retry
- Stale elements: Page updated, refresh and retry

### Comment Errors
- Cannot find reply input: Comment may be resolved, skip
- Send failed: Check network, retry once

## Browser-Use Agent Instructions

When using `browser-use run` for Notion tasks:

```bash
# Login and check inbox
browser-use -b remote run "
  1. Go to notion.so/login
  2. Click Google login
  3. Enter email agent@dowhiz.com and password [from env]
  4. After login, click Inbox in sidebar
  5. Report any @Oliver mentions found
" --wait

# Reply to a specific comment
browser-use -b remote run "
  1. Go to notion.so (already logged in)
  2. Navigate to page [PAGE_ID]
  3. Find the comment thread mentioning Oliver
  4. Type reply: [REPLY_TEXT]
  5. Click Send
" --session-id [existing_session]
```

## Integration with DoWhiz Scheduler

The Notion poller runs as part of the inbound gateway:

1. **Poll Interval**: Every 30-60 seconds
2. **Deduplication**: MongoDB stores processed notification IDs
3. **Task Queue**: New mentions create RunTask entries
4. **Outbound**: Agent responses sent via browser automation

## Configuration

Environment variables:
```bash
NOTION_EMPLOYEE_EMAIL=agent@dowhiz.com
NOTION_EMPLOYEE_PASSWORD=<google_password>
NOTION_POLL_INTERVAL_SECS=45
NOTION_BROWSER_SESSION=notion_oliver
```
