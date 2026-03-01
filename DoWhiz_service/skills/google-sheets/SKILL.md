---
name: google-sheets
description: Work with Google Sheets - read spreadsheet content, update cells, append rows, and respond to comments. Use this skill when handling comments from Google Sheets that mention you (proto, oliver, maggie, etc.).
allowed-tools: Bash(google-sheets:*)
---

# Google Sheets Collaboration Skill

## Overview

This skill enables you to collaborate on Google Sheets as a digital co-author. Users can share spreadsheets with you and request help via comments. You respond through comment replies and can read/modify spreadsheet data.

## When to Use This Skill

Use this skill when:
- You receive a task from Google Sheets (check `incoming_email/*_sheets_comment.json`)
- The user has mentioned you in a spreadsheet comment (proto, oliver, @proto, etc.)
- You need to read spreadsheet content, update cells, append rows, or respond to feedback

## CLI Commands Reference

### Reading Spreadsheets

```bash
# List all spreadsheets shared with you
google-sheets list-spreadsheets

# Read entire spreadsheet as CSV
google-sheets read-spreadsheet <spreadsheet_id>

# Read specific range (e.g., "Sheet1!A1:D10")
google-sheets read-values <spreadsheet_id> "Sheet1!A1:D10"

# Get spreadsheet metadata (sheet names, properties)
google-sheets get-metadata <spreadsheet_id>
```

### Working with Comments

```bash
# List all comments on a spreadsheet
google-sheets list-comments <spreadsheet_id>

# Reply to a comment
google-sheets reply-comment <spreadsheet_id> <comment_id> "Your reply message"
```

### Editing Spreadsheets

```bash
# Update cell values (JSON array format)
google-sheets update-values <spreadsheet_id> "Sheet1!A1:B2" '[["Hello","World"],["Foo","Bar"]]'

# Append rows to a range
google-sheets append-rows <spreadsheet_id> "Sheet1!A:D" '[["new","row","data","here"]]'

# Batch update (advanced operations)
google-sheets batch-update <spreadsheet_id> '<json_requests>'
```

## Workflow

### 1. Understanding the Request

When triggered by a Google Sheets comment:

1. Read the incoming comment from `incoming_email/email.html` or the comment JSON file
2. Note the **spreadsheet ID**, **comment ID**, and **quoted cell content** (if any)
3. The quoted content shows what part of the spreadsheet the comment references

### 2. Reading Spreadsheet Content

**The spreadsheet content may be pre-fetched at:**
```
incoming_email/spreadsheet_content.csv
```

If not available, read directly:

```bash
# Read the full spreadsheet
google-sheets read-spreadsheet <spreadsheet_id>

# Or read a specific range
google-sheets read-values <spreadsheet_id> "Sheet1!A1:Z100"
```

### 3. Making Edits

**Example: Update specific cells**

```bash
# Update cells A1:B2 with new values
google-sheets update-values 1abc123xyz "Sheet1!A1:B2" '[["Name","Age"],["Alice","25"]]'
```

**Example: Append a new row**

```bash
# Add a new row at the end
google-sheets append-rows 1abc123xyz "Sheet1!A:D" '[["Bob","30","Engineer","NYC"]]'
```

### 4. Responding to the User

After completing the task, reply to the comment:

```bash
google-sheets reply-comment 1abc123xyz COMMENT_ID "Done! I've updated the spreadsheet with the requested changes. Please review."
```

## Example Interaction

**User Comment:** "@proto populate this table with sample employee data"

**Your Response:**

1. Read current spreadsheet structure:
```bash
google-sheets read-values 1abc123xyz "Sheet1!A1:D1"
```

2. Add sample data:
```bash
google-sheets update-values 1abc123xyz "Sheet1!A1:D4" '[["Name","Age","Department","Location"],["Alice","28","Engineering","NYC"],["Bob","32","Marketing","LA"],["Carol","25","Sales","Chicago"]]'
```

3. Reply to confirm:
```bash
google-sheets reply-comment 1abc123xyz AAABxyz123 "Done! I've added sample employee data with 3 entries. The table includes Name, Age, Department, and Location columns."
```

## Notes

- The `!` character in range notation (e.g., `Sheet1!A1:B2`) is handled automatically
- JSON values in update-values must be properly formatted as a 2D array
- Use `get-metadata` to discover sheet names if you're unsure of the structure
