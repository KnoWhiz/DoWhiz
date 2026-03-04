# Notion Browser Integration - Testing Guide

## Overview

This guide covers testing the Notion browser automation integration, which allows the digital employee to:
- Monitor Notion notifications for @mentions
- Read page content for context
- Reply to comments via browser automation

## Prerequisites

### 1. WebDriver Setup

The integration uses WebDriver (fantoccini) for browser automation. You need either:

**Option A: geckodriver (Firefox)**
```bash
# macOS
brew install geckodriver

# Linux
wget https://github.com/mozilla/geckodriver/releases/latest/download/geckodriver-v0.34.0-linux64.tar.gz
tar -xvzf geckodriver-v0.34.0-linux64.tar.gz
sudo mv geckodriver /usr/local/bin/

# Start geckodriver
geckodriver --port 4444
```

**Option B: chromedriver (Chrome)**
```bash
# macOS
brew install chromedriver

# Linux
# Download from https://chromedriver.chromium.org/downloads

# Start chromedriver
chromedriver --port=4444
```

### 2. MongoDB

Ensure MongoDB is accessible. The integration uses MongoDB for:
- Tracking processed notifications (deduplication)
- User/task storage

### 3. Environment Variables

Add to your `.env` file:

```bash
# Notion Browser Integration
NOTION_BROWSER_ENABLED=true
NOTION_EMPLOYEE_EMAIL=agent@dowhiz.com
NOTION_EMPLOYEE_PASSWORD=A2da74ae9e06496088e3b385702ca55b
NOTION_POLL_INTERVAL_SECS=30
NOTION_BROWSER_HEADLESS=false
NOTION_BROWSER_SLOW_MO=100
WEBDRIVER_URL=http://localhost:4444
NOTION_EMPLOYEE_NAME=Oliver

# MongoDB
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=dowhiz
```

## Testing Steps

### Step 1: Verify WebDriver

```bash
# Check if WebDriver is running
curl http://localhost:4444/status

# Should return JSON with "ready": true
```

### Step 2: Run Unit Tests

```bash
cd DoWhiz_service
cargo test --release -p scheduler_module notion -- --nocapture
```

### Step 3: Manual Browser Login Test

Create a test script to verify browser login:

```bash
# From project root
cd DoWhiz_service

# Run a simple test
NOTION_BROWSER_ENABLED=true \
NOTION_EMPLOYEE_EMAIL=agent@dowhiz.com \
NOTION_EMPLOYEE_PASSWORD=A2da74ae9e06496088e3b385702ca55b \
NOTION_BROWSER_HEADLESS=false \
cargo test --release -p scheduler_module test_notion_browser_login -- --nocapture --ignored
```

### Step 4: End-to-End Test

1. **Start the inbound gateway:**
   ```bash
   ./scripts/run_gateway_local.sh
   ```

2. **Start the worker:**
   ```bash
   ./scripts/run_employee.sh little_bear 9001 --skip-hook --skip-ngrok
   ```

3. **Create a test @mention:**
   - Go to a Notion workspace where agent@dowhiz.com is a member
   - Create a comment mentioning @agent@dowhiz.com
   - Wait for the poll interval (30 seconds)

4. **Verify:**
   - Check gateway logs for "Processing notification"
   - Check worker logs for task execution
   - Check if reply was posted (browser reply)

## Testing Checklist

### Inbound Flow
- [ ] Browser connects to WebDriver
- [ ] Login succeeds (session persists)
- [ ] Notifications page loads
- [ ] @mentions are detected
- [ ] Processed notifications are tracked in MongoDB
- [ ] InboundMessage is created correctly
- [ ] Task is scheduled for worker

### Outbound Flow
- [ ] Worker generates reply_message.txt
- [ ] .notion_context.json is read
- [ ] .notion_reply_request.json is created
- [ ] Poller picks up reply request (TODO: implement)
- [ ] Browser posts reply to correct comment

## Troubleshooting

### WebDriver Connection Failed
```
Error: Failed to create browser session
```
- Ensure geckodriver/chromedriver is running on port 4444
- Check firewall settings
- Try restarting WebDriver

### Login Failed
```
Error: Login appeared to fail
```
- Verify NOTION_EMPLOYEE_EMAIL and NOTION_EMPLOYEE_PASSWORD
- Check if Notion requires 2FA (not supported yet)
- Try manual login in browser first

### Notifications Not Detected
```
No new Notion notifications
```
- Verify the account has unread notifications
- Check Notion notifications page manually
- Verify HTML parsing selectors in parser.rs

### MongoDB Connection Issues
```
Error: MongoDB config error
```
- Verify MONGODB_URI is correct
- Check MongoDB is running and accessible
- Verify network/firewall settings

## Current Limitations

1. **2FA Not Supported**: The browser automation doesn't handle 2FA flows yet.

2. **Reply Posting**: The reply mechanism writes to `.notion_reply_request.json` but the poller doesn't yet process these requests. This is a TODO.

3. **Session Management**: Browser sessions are recreated on each poll cycle. A persistent session mechanism would be more efficient.

4. **Selector Fragility**: HTML selectors in parser.rs may break if Notion updates their UI.

## Files Reference

| File | Purpose |
|------|---------|
| `notion_browser/browser.rs` | WebDriver session management |
| `notion_browser/poller.rs` | Polling loop for notifications |
| `notion_browser/parser.rs` | HTML parsing for notifications/pages |
| `notion_browser/store.rs` | MongoDB deduplication store |
| `notion_browser/models.rs` | Data structures |
| `service/inbound/notion.rs` | Inbound message handler |
| `scheduler/outbound.rs` | Reply executor (execute_notion_send) |
