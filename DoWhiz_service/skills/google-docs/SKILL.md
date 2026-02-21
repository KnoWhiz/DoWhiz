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
- **You receive an email containing a Google Docs link** (check `collaboration/` directory for artifact info)
- You need to read document content, propose edits, or respond to feedback

### Email → Google Docs Direct Modification

When a user sends you an email with a Google Docs link (instead of commenting in the doc), you can directly modify the document:

1. **Check for collaboration context**: Look for `collaboration/session_info.json` which contains the linked document ID
2. **Extract document ID**: The `artifact_id` field contains the Google Docs document ID
3. **Read the document**: Use `google-docs read-document <document_id>` or check `incoming_email/document_content.txt` if pre-fetched
4. **Apply changes**: Follow the user's instructions from the email and modify the document directly

**Example workflow:**

User email: "Proto, please review this doc and suggest grammar improvements: https://docs.google.com/document/d/1abc123..."

```bash
# 1. Read the document
google-docs read-document 1abc123

# 2. Detect mode from email ("suggest" mentioned)
# → Use suggesting mode

# 3. Apply suggestions
google-docs suggest-replace 1abc123 --find="original text" --replace="improved text"
```

Then reply via email confirming the changes were made.

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

### 3. Determine Editing Mode (Intelligent Detection)

**Intelligently detect the user's preference from context. Only ask if unclear.**

#### Check for Explicit Preference First:

Look for preference indicators in the user's message:

| User Says | Detected Mode |
|-----------|---------------|
| "suggest", "suggestion", "track changes", "revision marks", "review first", "let me review", "show me changes" | **Suggesting Mode** |
| "direct", "just do it", "apply directly", "make the changes", "edit directly", "no need to review" | **Direct Mode** |
| "help me improve", "fix this" (ambiguous - no explicit preference) | **Ask User** |

#### Examples of Intelligent Detection:

**Example 1 - User says "suggest" in email:**
> "Proto, please review this document and suggest improvements. I want to see what you'd change before applying."

→ **Auto-detect: Suggesting Mode** (no need to ask)

**Example 2 - User says "just fix it" in email:**
> "Proto, there are some grammar errors in this doc. Just fix them directly."

→ **Auto-detect: Direct Mode** (no need to ask)

**Example 3 - Ambiguous request:**
> "Proto, can you help improve this paragraph?"

→ **Ask User** (preference unclear)

#### When to Ask (only if genuinely unclear):

```html
<!-- reply_email_draft.html -->
<p>I understand you'd like me to [summarize the request]. How would you like me to proceed?</p>
<ul>
  <li><strong>Direct editing</strong> - I'll make the changes directly to your document</li>
  <li><strong>Suggesting mode</strong> - I'll show my proposed changes with revision marks so you can review first</li>
</ul>
<p>Reply "direct" or "suggest", or I'll default to suggesting mode for safety.</p>
```

#### Remember User Preferences:

If this is a **continuation of a collaboration session** (check `collaboration/session_info.json`), the user may have already expressed their preference in an earlier message. Check:
1. `collaboration/context_summary.md` - Original request may contain preference
2. `collaboration/message_history.json` - Previous interactions may indicate preference
3. If user said "suggest" in the original email, continue using suggesting mode for follow-up comments

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

### Style Operations (NEW)

```bash
# Get existing styles from document (ALWAYS run this first!)
google-docs get-styles <document_id>

# Set text style - color, font, size, bold, italic
google-docs set-style <document_id> --find="Heading Text" --color="#1B263B" --bold
google-docs set-style <document_id> --find="Body text" --font="Arial" --size=11
google-docs set-style <document_id> --find="Important" --color="#FF0000" --bold --italic
```

#### IMPORTANT: Match Existing Document Styles

**Before applying any style changes, ALWAYS:**

1. Run `google-docs get-styles <document_id>` to see the document's existing style patterns
2. Note the colors, fonts, and sizes used for each heading level (H1, H2, H3)
3. Apply styles that **match the existing patterns** in the document

**Example Workflow:**

```bash
# Step 1: Check existing styles
google-docs get-styles 1lkQLGy4_sVTNpMogVJcDnqCZnBS_AUcdBJh_sTINaGc

# Output shows:
# Heading 1: color=#1B263B, font="Georgia", size=24pt, bold
# Heading 2: color=#415A77, font="Georgia", size=18pt, bold
# Heading 3: color=#778DA9, font="Georgia", size=14pt, bold

# Step 2: Apply matching styles to new headings
google-docs set-style <doc_id> --find="New Section Title" --color="#1B263B" --font="Georgia" --size=24 --bold
google-docs set-style <doc_id> --find="Subsection" --color="#415A77" --font="Georgia" --size=18 --bold
```

**Never invent new styles** - always match what the user has already defined in their document.

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
2. **Intelligently detect editing mode** - Check if user already specified "suggest"/"direct" preference; only ask if genuinely unclear
3. **Check collaboration context** - If `collaboration/` directory exists, read it for prior preferences and context
4. **Use revision marks in suggesting mode** - Red strikethrough for deletions, blue for additions
5. **Explain changes clearly** - List what changes were made and why
6. **Wait for "apply" in suggesting mode** - Don't finalize until user explicitly approves
7. **Keep replies concise** - Google Docs comments have limited space
8. **Match existing document formatting** - When applying changes, preserve the user's text formatting (font, size, etc.)
9. **Default to suggesting mode** - When in doubt, use suggesting mode (safer, non-destructive)
10. **For style changes, ALWAYS check existing styles first** - Run `google-docs get-styles <doc_id>` before applying any formatting. Match the document's existing heading colors, fonts, and sizes. Never invent new styles - follow the user's established patterns.

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
