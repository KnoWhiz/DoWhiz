---
name: google-docs
description: Work with Google Docs - read document content, propose edits via comments, create new documents, and share with users. Use this skill when handling Google Docs comments or email requests to create/edit Google documents.
allowed-tools: Bash(google-docs:*)
---

# Google Docs Collaboration Skill

## When to Use This Skill

Use this skill when:
- You receive a task from Google Docs (check `incoming_email/*_gdocs_comment.json`)
- The user has mentioned you in a document comment
- **You receive an email containing a Google Docs link**
- You need to read document content, propose edits, or respond to feedback
- **User asks you to CREATE a new Google Doc** (e.g., "create a document", "draft a report")

## Creating New Documents (CRITICAL!)

When a user asks you to create a new Google Doc:

1. **Create the document**:
```bash
google-docs create-document --title="Document Title Here"
```

2. **Share with the requesting user** (ALWAYS do this!):
```bash
# Extract sender email from incoming_email/postmark_payload.json (the "From" field)
google-docs share <document_id> --email="sender@example.com" --role="writer" --notify
```

3. **Reply with the link** in reply_email_draft.html

**CRITICAL: ALWAYS share with the original email sender.** Extract their email from `incoming_email/postmark_payload.json`.

### Security Rules for Sharing

| Request Type | Action |
|-------------|--------|
| "Create a doc for me" | Share with sender ONLY |
| "Share with same-domain user" | Share with sender AND that user |
| "Share with external user" | Share with sender ONLY, ask for confirmation |

### Example Workflow

User email from `liuxt@umich.edu`: "Please create a research notes document"

```bash
# 1. Create doc
google-docs create-document --title="Research Notes"
# Output: Document ID: 1abc123

# 2. Share with sender (REQUIRED)
google-docs share 1abc123 --email="liuxt@umich.edu" --role="writer" --notify
```

Reply:
```html
<p>I've created your document: <a href="https://docs.google.com/document/d/1abc123">Research Notes</a></p>
<p>You have editor access!</p>
```

## Available Commands

### Document Management
```bash
google-docs create-document --title="Title"
google-docs share <file_id> --email="user@example.com" --role="writer" --notify
google-docs get-link <file_id>
google-docs list-permissions <file_id>
```

### Read Operations
```bash
google-docs list-documents
google-docs read-document <document_id>
google-docs list-comments <document_id>
```

### Edit Operations
```bash
google-docs apply-edit <document_id> --find="text" --replace="new text"
google-docs insert-text <document_id> --after="anchor" --text="content"
google-docs delete-text <document_id> --find="text"
```

## Important Guidelines

1. **ALWAYS share new documents with the sender** - Extract email from `incoming_email/postmark_payload.json`
2. **Security: Only share with verified recipients** - Default to sender only
3. Read `incoming_email/document_content.txt` before editing existing docs
