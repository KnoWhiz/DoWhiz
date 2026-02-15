---
name: google-docs
description: Work with Google Docs - read document content, propose edits via comments, and apply changes when confirmed. Use this skill when handling comments from Google Docs that mention you (proto, oliver, maggie, etc.).
allowed-tools: Bash(google-docs:*)
---

# Google Docs Collaboration Skill

## Overview

This skill enables you to collaborate on Google Docs as a digital co-author. Users can share documents with you and request help via comments. You respond through comment replies and can propose or apply document edits.

## When to Use This Skill

Use this skill when:
- You receive a task from Google Docs (check `incoming_email/*_gdocs_comment.json`)
- The user has mentioned you in a document comment (proto, oliver, @proto, etc.)
- You need to read document content, propose edits, or respond to feedback

## Workflow

### 1. Understanding the Request

When triggered by a Google Docs comment:

1. Read the incoming comment from `incoming_email/email.html` or `incoming_email/*_gdocs_comment.json`
2. Note the **document ID**, **comment ID**, and **quoted text** (if any)
3. The quoted text shows what part of the document the comment references

### 2. Reading Document Content

**The full document content is pre-fetched and available at:**
```
incoming_email/document_content.txt
```

This file contains the complete document as plain text. **Always read this file first** to understand the full context before responding to comments or making edit suggestions.

### 3. Ask User's Preference (IMPORTANT!)

**Before making any edits, always ask the user which mode they prefer:**

```html
<!-- reply_email_draft.html -->
<p>I understand you'd like me to [summarize the request]. Before I proceed, would you prefer:</p>
<ul>
  <li><strong>Direct editing</strong> - I'll make the changes directly to your document</li>
  <li><strong>Suggesting mode</strong> - I'll show my proposed changes with revision marks (red strikethrough for deletions, blue for additions) so you can review before applying</li>
</ul>
<p>Reply "direct" or "suggest" to let me know your preference.</p>
```

### 4. Editing Modes

#### Mode A: Direct Editing

If user chooses "direct", make the changes immediately:

```bash
# Replace text directly
google-docs apply-edit <document_id> --find="original text" --replace="new text"
```

Then confirm completion:
```html
<p>Done! I've updated the document with the changes you requested.</p>
```

#### Mode B: Suggesting Mode (Word-style Revision Marks)

If user chooses "suggest", apply changes with visual revision marks:

**Color Coding (Word-style):**
- 🔴 **Red + Strikethrough** = Text to be deleted
- 🔵 **Blue** = New/added text

```bash
# Mark text for deletion (red strikethrough)
google-docs mark-deletion <document_id> --find="text to delete"

# Insert new text with suggestion formatting (blue)
google-docs insert-suggestion <document_id> --after="anchor text" --text="new text to add"

# Replace with revision marks (marks old as deleted, adds new as blue)
google-docs suggest-replace <document_id> --find="old text" --replace="new text"
```

Example reply after applying suggestions:
```html
<p>I've added my suggested changes to the document with revision marks:</p>
<ul>
  <li><span style="color:red;text-decoration:line-through;">Red strikethrough</span> = text I suggest removing</li>
  <li><span style="color:blue;">Blue text</span> = text I suggest adding</li>
</ul>
<p>Please review the changes in the document. When you're satisfied, reply "apply" and I'll finalize all the changes (remove the formatting and make them permanent).</p>
```

### 5. Applying Suggestions

When user replies "apply" or "accept" after reviewing suggestions:

```bash
# Apply all pending suggestions (remove red text, convert blue to black)
google-docs apply-suggestions <document_id>
```

This command:
1. Deletes all text with red strikethrough formatting
2. Converts blue text to the document's default formatting (black)
3. Results in a clean, final document

Reply after applying:
```html
<p>All changes have been applied! The document is now updated with the final text.</p>
```

## Available Commands

### Read Operations (Safe)

```bash
# List all documents shared with you
google-docs list-documents

# Read document content
google-docs read-document <document_id>

# List comments on a document
google-docs list-comments <document_id>

# Read a specific comment and its replies
google-docs read-comment <document_id> <comment_id>
```

### Direct Edit Operations

```bash
# Apply a text replacement directly
google-docs apply-edit <document_id> --find="text to find" --replace="replacement text"

# Insert text at a position
google-docs insert-text <document_id> --after="anchor text" --text="text to insert"

# Delete text
google-docs delete-text <document_id> --find="text to delete"
```

### Suggesting Mode Operations

```bash
# Mark text for deletion (red strikethrough)
google-docs mark-deletion <document_id> --find="text to mark as deleted"

# Insert suggestion text (blue)
google-docs insert-suggestion <document_id> --after="anchor text" --text="suggested new text"

# Replace with revision marks (old=red strikethrough, new=blue)
google-docs suggest-replace <document_id> --find="old text" --replace="new text"

# Apply all suggestions (finalize the document)
google-docs apply-suggestions <document_id>

# Discard all suggestions (revert to original)
google-docs discard-suggestions <document_id>
```

## Response Format Examples

### Example 1: Initial Response to Edit Request

User comment: "Proto, please improve this paragraph"

```html
<!-- reply_email_draft.html -->
<p>I'd be happy to improve this paragraph! Before I proceed, would you prefer:</p>
<ul>
  <li><strong>Direct editing</strong> - I'll make the changes directly</li>
  <li><strong>Suggesting mode</strong> - I'll show revision marks so you can review first</li>
</ul>
<p>Reply "direct" or "suggest".</p>
```

### Example 2: After User Chooses "suggest"

```html
<!-- reply_email_draft.html -->
<p>I've added my suggestions to the document with revision marks:</p>
<ul>
  <li><span style="color:red;text-decoration:line-through;">Red strikethrough</span> = remove</li>
  <li><span style="color:blue;">Blue text</span> = add</li>
</ul>
<p>Changes made:</p>
<ol>
  <li>Replaced "very good" with "excellent" for more professional tone</li>
  <li>Added transition sentence between paragraphs</li>
  <li>Removed redundant phrase "in order to"</li>
</ol>
<p>Reply "apply" to finalize these changes, or let me know if you'd like adjustments.</p>
```

### Example 3: After User Says "apply"

```html
<!-- reply_email_draft.html -->
<p>Done! All suggestions have been applied. The document now contains the finalized text without any revision marks.</p>
```

### Example 4: Complex Multi-Section Edit

User comment: "Proto, review the entire document for grammar and clarity"

```html
<!-- reply_email_draft.html -->
<p>I've reviewed the document and found 5 areas for improvement. Would you like me to:</p>
<ul>
  <li><strong>Direct editing</strong> - Apply all fixes immediately</li>
  <li><strong>Suggesting mode</strong> - Show all changes with revision marks for your review</li>
</ul>
<p>Here's a summary of what I'll change:</p>
<ol>
  <li>Section 1: Fix passive voice (3 instances)</li>
  <li>Section 2: Improve sentence flow</li>
  <li>Section 3: Remove redundant words</li>
  <li>Section 4: Clarify technical explanation</li>
  <li>Conclusion: Strengthen call-to-action</li>
</ol>
<p>Reply "direct" or "suggest".</p>
```

## Important Guidelines

1. **Read document_content.txt first** - Always read `incoming_email/document_content.txt` before responding
2. **Always ask for editing mode preference** - Let user choose between direct editing and suggesting mode
3. **Use revision marks in suggesting mode** - Red strikethrough for deletions, blue for additions
4. **Explain changes clearly** - List what changes were made and why
5. **Wait for "apply" in suggesting mode** - Don't finalize until user explicitly approves
6. **Keep replies concise** - Google Docs comments have limited space
7. **Match existing document formatting** - When applying changes, preserve the user's text formatting (font, size, etc.)

## Error Handling

If you encounter errors:
- Document not accessible: Ask user to verify sharing permissions
- Comment not found: The comment may have been resolved or deleted
- Edit failed: The document content may have changed; re-read and retry
- Formatting issues: Ensure the find text matches exactly (including whitespace)

## Color Reference

| Action | Color | Style | Meaning |
|--------|-------|-------|---------|
| Delete | Red (#FF0000) | Strikethrough | Text to be removed |
| Add | Blue (#0000FF) | Normal | New text to be added |
| Final | Black (default) | Normal | Applied/accepted text |
