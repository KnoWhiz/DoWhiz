# Migrating from the Email Settings API

## Overview

This guide outlines the transition from Google's deprecated Email Settings API to the Gmail API. The document presents key differences and mapping for developers updating their applications.

## Authorization Changes

**Key Difference**: Gmail API permissions scope to individual users rather than entire domains.

The old scope:
```
https://apps-apis.google.com/a/feeds/emailsettings/2.0/
```

New equivalent scopes:
```
https://www.googleapis.com/auth/gmail.settings.basic
https://www.googleapis.com/auth/gmail.settings.sharing
```

**Important**: Domain administrator authorization doesn't grant access to migrate mail for other users. Instead, use service accounts with domain-wide authority, whitelisted in the Admin console.

## Protocol Migration

The Email Settings API used XML-based GDATA format; Gmail API uses JSON.

### Example: Creating a Label

**Email Settings API**:
```xml
POST https://apps-apis.google.com/a/feeds/emailsettings/2.0/{domain}/{username}/label
<?xml version="1.0" encoding="utf-8"?>
<atom:entry xmlns:atom="http://www.w3.org/2005/Atom"
    xmlns:apps="http://schemas.google.com/apps/2006">
  <apps:property name="label" value="status updates" />
</atom:entry>
```

**Gmail API**:
```json
POST https://www.googleapis.com/gmail/v1/users/{username}/labels
{
   "name": "status updates"
}
```

## Settings Mapping Tables

### Labels

| Old Setting | New Setting | Notes |
|---|---|---|
| labelId | id | — |
| label | name | — |
| unreadCount | messagesUnread | — |
| visibility | labelListVisibility | SHOW→labelShow; HIDE→labelHide |

Labels are now referenced by ID rather than name.

### Filters

| Old Setting | New Setting | Notes |
|---|---|---|
| from | criteria.from | — |
| to | criteria.to | — |
| subject | criteria.subject | — |
| hasTheWord | criteria.query | — |
| doesNotHaveTheWord | criteria.negatedQuery | — |
| hasAttachment | criteria.hasAttachment | — |
| shouldArchive | action.removeLabelIds | Use INBOX label ID |
| shouldMarkAsRead | action.removeLabelIds | Use UNREAD label ID |
| shouldStar | action.addLabelIds | Use STARRED label ID |
| label | action.addLabelIds | Use label ID |
| forwardTo | action.forward | — |
| shouldTrash | action.addLabelIds | Use TRASH label ID |
| neverSpam | action.removeLabelIds | Use SPAM label ID |

User labels must be created explicitly using the labels.create method if they don't exist.

### Send-As Aliases

| Old Setting | New Setting |
|---|---|
| name | displayName |
| address | sendAsEmail |
| replyTo | replyToAddress |
| makeDefault | isDefault |

### Auto-Forwarding Settings

| Old Setting | New Setting | Notes |
|---|---|---|
| enable | enabled | — |
| forwardTo | emailAddress | — |
| action | disposition | KEEP→leaveInInbox; ARCHIVE→archive; DELETE→trash; MARK_READ→markRead |

Forwarding addresses require creation and verification before use.

### POP Settings

| Old Setting | New Setting | Notes |
|---|---|---|
| enable | accessWindow | Set to "disabled" when off |
| enableFor | accessWindow | ALL_MAIL→allMail; MAIL_FROM_NOW_ON→fromNowOn |
| action | disposition | Same as auto-forwarding |

### IMAP Settings

| Old Setting | New Setting |
|---|---|
| enable | enabled |

### Vacation Auto-Reply

| Old Setting | New Setting |
|---|---|
| contactsOnly | restrictToContacts |
| domainOnly | restrictToDomain |
| enable | enableAutoReply |
| endDate | endTime |
| message | responseBodyHtml / responseBodyPlainText |
| startDate | startTime |
| subject | responseSubject |

### Signatures

| Old Setting | New Setting | Notes |
|---|---|---|
| signature | signature | Now managed per alias |

### Language Settings

| Old Setting | New Setting |
|---|---|
| language | displayLanguage |

### Delegation Settings

| Old Setting | New Setting | Notes |
|---|---|---|
| address | delegateEmail | — |
| status | verificationStatus | — |

**delegates.create** updates:
- Works across multiple domains in same organization
- Supports users requiring password change at next login
- Returns delegate resource in response body (previously empty)
- Returns HTTP 4XX for disabled users (previously 500)

**delegates.delete** now works with any verification status.

**delegates.get** is a new method available for individual delegate queries.

## Removed Features

Web Clip settings and general settings are no longer available via API.
