# Gmail API Reference Documentation

## Overview

The Gmail API enables developers to view and manage Gmail mailbox data including threads, messages, and labels through REST endpoints.

**Service Endpoint:** `https://gmail.googleapis.com`

**Discovery Document:** `https://gmail.googleapis.com/$discovery/rest?version=v1`

---

## Core REST Resources

### Users (`v1.users`)

Manage user account information and settings.

**Methods:**
- `getProfile` - Retrieves the current user's Gmail profile
- `watch` - Establishes push notification monitoring on a user mailbox
- `stop` - Disables push notifications for a mailbox

### Drafts (`v1.users.drafts`)

Handle draft message creation and management.

**Methods:**
- `create` - Generates a new draft with the DRAFT label
- `list` - Displays all drafts in the user's mailbox
- `get` - Retrieves a specific draft
- `update` - Replaces draft content
- `send` - Transmits an existing draft to recipients
- `delete` - Permanently removes a draft

### Messages (`v1.users.messages`)

Manage individual email messages.

**Methods:**
- `list` - Retrieves messages from the mailbox
- `get` - Fetches a specific message
- `send` - Transmits a message to recipients
- `import` - Inserts messages with standard delivery processing
- `insert` - Directly adds messages bypassing most scanning
- `modify` - Updates labels on messages
- `batchModify` - Changes labels across multiple messages
- `delete` - Permanently removes a message
- `batchDelete` - Removes multiple messages
- `trash` - Moves messages to trash
- `untrash` - Restores messages from trash

### Message Attachments (`v1.users.messages.attachments`)

Access message attachment data.

**Methods:**
- `get` - Retrieves a specific message attachment

### Labels (`v1.users.labels`)

Create and manage custom mailbox labels.

**Methods:**
- `list` - Shows all user labels
- `get` - Retrieves a specific label
- `create` - Generates a new label
- `update` - Modifies label properties
- `patch` - Partially updates a label
- `delete` - Removes a label permanently

### Threads (`v1.users.threads`)

Organize and manage message conversations.

**Methods:**
- `list` - Shows all threads in the mailbox
- `get` - Retrieves a specific thread
- `modify` - Updates labels on threads
- `trash` - Moves threads to trash
- `untrash` - Restores threads from trash
- `delete` - Permanently removes threads

### History (`v1.users.history`)

Track mailbox changes over time.

**Methods:**
- `list` - Displays all mailbox modifications and changes

### Settings (`v1.users.settings`)

Configure account preferences and behavior.

**Methods:**
- `getAutoForwarding`/`updateAutoForwarding` - Manage forwarding rules
- `getImap`/`updateImap` - Configure IMAP access
- `getPop`/`updatePop` - Configure POP access
- `getLanguage`/`updateLanguage` - Set language preferences
- `getVacation`/`updateVacation` - Configure vacation responders

### Delegates (`v1.users.settings.delegates`)

Grant mailbox access to other users.

**Methods:**
- `list` - Shows all delegates
- `get` - Retrieves delegate information
- `create` - Adds a new delegate
- `delete` - Removes delegate access

### Filters (`v1.users.settings.filters`)

Automate message handling rules.

**Methods:**
- `list` - Shows all filters
- `get` - Retrieves a specific filter
- `create` - Generates a new filter
- `delete` - Removes a filter

### Send-As Addresses (`v1.users.settings.sendAs`)

Manage alternate sender identities.

**Methods:**
- `list` - Shows all send-as addresses
- `get` - Retrieves address details
- `create` - Adds a new sender identity
- `update`/`patch` - Modifies sender settings
- `verify` - Sends verification email
- `delete` - Removes a sender identity

### Client-Side Encryption

**Identities** (`v1.users.settings.cse.identities`) - Configure encrypted sender profiles
**Keypairs** (`v1.users.settings.cse.keypairs`) - Manage encryption certificate chains

---

## Client Libraries

Supported across multiple programming languages:
- JavaScript/Node.js
- Python
- Java
- Go
- Ruby
- PHP
- .NET

---

## Additional Resources

- **API Usage Limits:** Available quota documentation
- **Postmaster Tools API:** Domain reputation monitoring (v1 and v2beta versions)
- **Email Markup:** Schema.org structured data for enhanced email display
- **Android Content Provider:** GmailContract for native Android integration
