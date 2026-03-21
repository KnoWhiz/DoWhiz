# Notion Integration - Testing Guide

## Overview

This guide covers testing the Notion OAuth + API integration, which allows the digital employee to:
- Receive @mentions via email notifications from Notion
- Read page content via Notion API
- Reply to comments via Notion API


## Prerequisites

### 1. Notion Integration Setup

**Option A: Internal Integration (for testing)**
1. Go to https://www.notion.so/my-integrations
2. Create a new Internal Integration
3. Copy the "Internal Integration Secret" (starts with `secret_`)
4. Share your test page with the integration

**Option B: Public Integration (for production)**
1. Configure OAuth at https://www.notion.so/my-integrations
2. Set redirect URI to `https://dowhiz.com/auth/notion/callback`
3. Add to `.env`:
   ```bash
   NOTION_CLIENT_ID=<from Notion>
   NOTION_CLIENT_SECRET=<from Notion>
   NOTION_REDIRECT_URI=https://dowhiz.com/auth/notion/callback
   ```

### 2. MongoDB

Ensure MongoDB is accessible. The integration uses MongoDB for:
- Storing OAuth tokens (NotionStore)
- Tracking processed notifications (deduplication)

### 3. Environment Variables

For testing with Internal Integration:
```bash
# Internal Integration token (testing only)
NOTION_API_TOKEN=secret_xxx

# MongoDB
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=dowhiz
```

For production OAuth:
```bash
# Notion OAuth
NOTION_CLIENT_ID=xxx
NOTION_CLIENT_SECRET=xxx
NOTION_REDIRECT_URI=https://dowhiz.com/auth/notion/callback

# MongoDB
MONGODB_URI=mongodb://cosmosdb-uri
MONGODB_DATABASE=dowhiz_staging_boiled_egg
```

## Testing Steps

### Step 1: Test notion_api_cli

```bash
cd DoWhiz_service/scripts

# Set token
export NOTION_API_TOKEN="secret_xxx"

# Verify token works
./notion_api_cli me

# Read a page
./notion_api_cli read-page YOUR_PAGE_ID

# Read page blocks
./notion_api_cli read-blocks YOUR_PAGE_ID

# Create a test comment
./notion_api_cli create-comment YOUR_PAGE_ID "Test comment from CLI"
```

### Step 2: Test Token Storage

```bash
cd DoWhiz_service/scripts

# Store a token manually (simulates OAuth result)
./store_notion_token.sh "test-workspace-id" "secret_xxx" "boiled_egg"

# Verify it's stored
mongosh "$MONGODB_URI" --eval "db.notion_credentials.find()"
```

### Step 3: E2E Test (Full Codex Flow)

```bash
cd DoWhiz_service/scripts

# Run the E2E test script
./test_notion_e2e.sh secret_xxx YOUR_PAGE_ID

# This will:
# 1. Verify the token works
# 2. Store token in MongoDB
# 3. Create a test workspace for Codex
# 4. Show manual test instructions
```

### Step 4: Run Unit Tests

```bash
cd DoWhiz_service
cargo test --release -p scheduler_module notion -- --nocapture
```

### Step 5: Integration Test on Production (with Services)

1. **Complete Notion OAuth:**
   - Go to `dowhiz.com/auth/index.html`
   - Click "Connect Notion" in the integrations section
   - Authorize the Notion integration and grant access to your workspace/pages
   - Verify the token is stored: check gateway logs for "Notion OAuth callback" success

2. **Create a test @mention:**
   - Share the Notion page with `oliver@dowhiz.com` (so the bot receives email notifications)
   - Go to the shared Notion page
   - Create a comment mentioning the bot (@Oliver or @Proto-DoWhiz)
   - Notion will send an email notification to `oliver@dowhiz.com`

3. **Verify:**
   - Check gateway logs for email processing
   - Check worker logs for task execution
   - Check Notion page for API reply

## Testing Checklist

### OAuth Flow
- [ ] User can initiate OAuth at /auth/notion
- [ ] Callback handles code exchange correctly
- [ ] Token is stored in NotionStore (MongoDB)
- [ ] Token can be retrieved by workspace_id
- [ ] Token can be retrieved by workspace_name (fuzzy match)

### Email Detection
- [ ] Emails from notify@mail.notion.so are detected
- [ ] NotionEmailNotification is parsed correctly
- [ ] workspace_name is extracted from URL
- [ ] page_id is extracted from URL
- [ ] actor_name is extracted (English and Chinese)

### Token Lookup
- [ ] Token found by exact workspace_name match
- [ ] Token found by fuzzy workspace_name match
- [ ] Token fallback to NOTION_API_TOKEN env var
- [ ] No token case handled gracefully

### Task Execution
- [ ] .notion_email_context.json is created
- [ ] .notion_env is created with token (when available)
- [ ] Codex can source .notion_env
- [ ] notion_api_cli commands work
- [ ] Reply is posted to correct page

## Troubleshooting

### Token Not Found

```
WARN No Notion token found for workspace 'myworkspace'
```

- Verify OAuth was completed for this workspace
- Check workspace_name matching in logs
- Try using exact NOTION_API_TOKEN env var for testing

### API Errors

```
Error: 401 Unauthorized
```
- Token may be expired - user needs to re-authorize
- Check token is correctly stored in MongoDB

```
Error: 403 Forbidden
```
- Page not shared with the integration
- For Internal Integration: share page explicitly
- For Public Integration: user needs to grant page access during OAuth

### Email Detection Failed

```
No Notion notification detected in email
```
- Check sender is from notion.so domain
- Verify email format matches expected patterns
- Check parser regex patterns in notion_email_detector.rs

## Files Reference

| File | Purpose |
|------|---------|
| `scheduler_module/src/notion_store.rs` | OAuth token storage (MongoDB) |
| `scheduler_module/src/notion_email_detector.rs` | Email parsing and detection |
| `scheduler_module/src/service/inbound/notion_email.rs` | Email → task processing |
| `scheduler_module/src/service/auth.rs` | OAuth callback handlers |
| `scripts/notion_api_cli` | CLI for Notion API calls |
| `scripts/test_notion_e2e.sh` | E2E test script |
| `scripts/store_notion_token.sh` | Manual token storage |
| `scripts/test_notion_api.sh` | Direct API test |

## Deprecated: Browser Automation

The browser automation approach (notion_browser/*) has been superseded by the OAuth + API integration. The old files remain for reference but are no longer used:

| Deprecated File | Replacement |
|----------------|-------------|
| `notion_browser/browser.rs` | OAuth + notion_api_cli |
| `notion_browser/poller.rs` | Email notifications |
| `notion_browser/parser.rs` | notion_email_detector.rs |

Benefits of the new approach:
- No WebDriver dependency
- No login credentials to manage
- No DOM selector fragility
- Multi-workspace support via OAuth tokens
- Faster and more reliable
