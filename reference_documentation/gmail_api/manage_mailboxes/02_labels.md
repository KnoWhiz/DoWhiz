# Manage Labels

## Overview

Labels enable you to organize and categorize messages and threads in Gmail. They operate on a many-to-many relationship: multiple labels can apply to a single message, and a single label can apply across many messages or threads.

## Required Authorization

Managing labels requires the `https://www.googleapis.com/auth/gmail.labels` scope. Consult the Gmail API authentication documentation for scope details.

## Label Types

Gmail supports two label categories:

### System Labels (Reserved)

System labels correspond to Gmail interface elements and cannot be duplicated with custom labels. Common examples include:

| Label | Manual Application | Notes |
|-------|-------------------|-------|
| `INBOX` | Yes | |
| `SPAM` | Yes | |
| `TRASH` | Yes | |
| `UNREAD` | Yes | |
| `STARRED` | Yes | |
| `IMPORTANT` | Yes | |
| `SENT` | No | Applied automatically to messages that are sent with `drafts.send` or `messages.send` |
| `DRAFT` | No | Applied to all draft messages automatically |
| `CATEGORY_PERSONAL` | Yes | Personal tab content |
| `CATEGORY_SOCIAL` | Yes | Social tab content |
| `CATEGORY_PROMOTIONS` | Yes | Promotions tab content |
| `CATEGORY_UPDATES` | Yes | Updates tab content |
| `CATEGORY_FORUMS` | Yes | Forums tab content |

**Note:** Attempting to create a custom label matching a reserved name results in an HTTP 400 error.

### User Labels (Custom)

Custom labels are user-created organizational tags for messages and threads.

## Managing Labels on Messages and Threads

### Key Distinction

Labels only exist on messages. When listing thread labels, you receive labels from any message within that thread, though not all messages necessarily carry each label.

### Applying Labels to Threads

Using the `threads.modify` endpoint, you can add or remove labels across all existing thread messages. Important caveat: newly added messages don't automatically inherit existing thread labels.

### Applying Labels to Messages

The `messages.modify` endpoint adds or removes labels on specific messages. When a label is removed from a message and no other thread messages carry it, the label is removed from the thread association.

**Draft Limitation:** Labels cannot be applied to draft messages.

## API Reference

For operations, see the Labels reference documentation:
- Create labels
- Retrieve label details
- List labels
- Update labels
- Delete labels
