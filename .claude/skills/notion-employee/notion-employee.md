# Notion Employee Integration Skill

This skill enables digital employees (Oliver, Maggie, etc.) to interact with Notion as real team members - receiving @mentions, replying to comments, and editing pages.

## Workflow Overview (OAuth + API)

DoWhiz uses **Notion Public Integration (OAuth)** to access Notion workspaces. This replaces the old browser automation approach.

```
1. User authorizes DoWhiz at dowhiz.com/settings (OAuth)
2. OAuth token is stored in MongoDB
3. Email notifications from Notion trigger tasks
4. Agent uses notion_api_cli to read/write pages
5. No browser automation required
```

## Trigger Detection

**Email-triggered tasks:** Check for `.notion_email_context.json` in your workspace root. If present, this task was triggered by a Notion email notification.

```bash
# Check if this is a Notion task
cat .notion_email_context.json
```

## Email-Triggered Workflow

### Step 1: Check API Access

```bash
# Source the Notion API token (if available)
if [ -f .notion_env ]; then
    source .notion_env
    echo "API token available"
else
    echo "No API token - user needs to authorize at dowhiz.com/settings"
fi
```

### Step 2: Read Context

```bash
cat .notion_email_context.json
```

The context contains:
| Field | Description |
|-------|-------------|
| `page_url` | Direct URL to the Notion page |
| `page_id` | 32-char UUID for API access |
| `actor_name` | Who mentioned you |
| `comment_preview` | Preview of the comment/mention |
| `notification_type` | Type: comment_mention, page_mention, comment_reply, page_comment |
| `has_api_access` | Whether OAuth token is available |
| `instructions` | How to proceed |

### Step 3: Read Page Content

```bash
# Read page metadata
notion_api_cli read-page $PAGE_ID

# Read page blocks/content
notion_api_cli read-blocks $PAGE_ID

# Get comments on the page
notion_api_cli get-comments $PAGE_ID
```

### Step 4: Complete the Task

Based on the comment/mention request, perform the necessary actions:
- Research information
- Create documents
- Update content
- etc.

### Step 5: Reply via API

```bash
# Create a new comment on the page
notion_api_cli create-comment $PAGE_ID "Your reply message here"

# Or reply to an existing comment thread
notion_api_cli reply $DISCUSSION_ID "Your reply message"

# Append a block to the page
notion_api_cli append-block $PAGE_ID "Content to add"
```

### Step 6: Write reply_message.txt

Write your response summary for the task system:
```bash
echo "I've replied to the Notion comment with: [summary of your reply]" > reply_message.txt
```

## notion_api_cli Reference

```bash
# Get bot info (verify token works)
notion_api_cli me

# Search pages
notion_api_cli search "query"

# Read page metadata
notion_api_cli read-page PAGE_ID

# Read page blocks/content
notion_api_cli read-blocks PAGE_ID

# Get comments on a page
notion_api_cli get-comments PAGE_ID

# Reply to a comment thread
notion_api_cli reply DISCUSSION_ID "message"

# Create new comment on page
notion_api_cli create-comment PAGE_ID "message"

# Append a paragraph block
notion_api_cli append-block PAGE_ID "content"
```

## No API Access?

If `.notion_env` is missing or `has_api_access` is false:

1. Inform the user that you cannot access the page
2. Write to reply_message.txt:
   ```
   I was mentioned in Notion but cannot access the page. The workspace owner needs to authorize DoWhiz at dowhiz.com/settings.
   ```

## Multi-Workspace Support

OAuth tokens are stored per-workspace. When a Notion email arrives:

1. The system extracts the workspace name from the email URL
2. Looks up the matching OAuth token in MongoDB
3. Passes the token to the agent via `.notion_env`

Users can connect multiple Notion workspaces at dowhiz.com/settings.

## Configuration

Required for OAuth flow (set in DoWhiz_service/.env):
```bash
NOTION_CLIENT_ID=<from Notion integration settings>
NOTION_CLIENT_SECRET=<from Notion integration settings>
NOTION_REDIRECT_URI=https://dowhiz.com/auth/notion/callback
```

For manual testing without OAuth:
```bash
# Get an Internal Integration token from https://www.notion.so/my-integrations
NOTION_API_TOKEN=secret_xxx
```

## Employee Accounts

| Employee | Primary Email |
|----------|---------------|
| Oliver (little_bear) | oliver@dowhiz.com |
| Maggie (mini_mouse) | maggie@dowhiz.com |
| Boiled-Egg (boiled_egg) | proto@dowhiz.com |

## Error Handling

### API Errors
- `401 Unauthorized`: Token expired or invalid - user needs to re-authorize
- `403 Forbidden`: Page not shared with the integration
- `404 Not Found`: Page deleted or moved

### Workspace Not Found
If no OAuth token matches the workspace:
- The email trigger will still create a task
- `has_api_access` will be false
- Inform the user to authorize at dowhiz.com/settings

## Testing

### Test with Internal Integration (no OAuth needed)

1. Create an Internal Integration at https://www.notion.so/my-integrations
2. Share your test page with the integration
3. Run the E2E test:
   ```bash
   cd DoWhiz_service/scripts
   ./test_notion_e2e.sh secret_xxx PAGE_ID
   ```

### Test the CLI directly

```bash
export NOTION_API_TOKEN="secret_xxx"
notion_api_cli me
notion_api_cli read-page PAGE_ID
notion_api_cli create-comment PAGE_ID "Test comment from CLI"
```

## Migration from Browser Automation

The old browser automation workflow has been deprecated:

| Old (Browser) | New (API) |
|---------------|-----------|
| `browser-use` + Google OAuth login | OAuth token from dowhiz.com/settings |
| Cookie persistence | Token stored in MongoDB |
| Inbox polling via browser | Email notifications from Notion |
| Reply via browser click/type | `notion_api_cli create-comment` |

Benefits of the new approach:
- More reliable (no DOM changes to break automation)
- Faster (no browser startup overhead)
- Multi-workspace support (tokens per workspace)
- No rate limiting from Google login
