# Managing POP and IMAP

## Overview

The Gmail Settings API enables configuration of how Gmail behaves when accessed through POP and IMAP protocols. The API provides methods to get and update these settings.

## Available Methods

The Settings resource offers four main operations:
- `getPop` - Retrieve POP settings
- `updatePop` - Modify POP configuration
- `getImap` - Retrieve IMAP settings
- `updateImap` - Modify IMAP configuration

## POP Configuration

### Access Control

The `accessWindow` property controls whether POP is enabled and which messages are accessible:

| Value | Behavior |
|-------|----------|
| `disabled` | POP access is turned off |
| `allMail` | All existing messages become retrievable |
| `fromNowOn` | Only newly arriving messages are available |

### Message Handling

The `disposition` property determines what happens to messages after retrieval via POP:

| Value | Behavior |
|-------|----------|
| `archive` | Messages are removed from inbox |
| `leaveInInbox` | Messages remain unread in inbox |
| `markRead` | Messages are marked as read |
| `trash` | Messages move to trash folder |

## IMAP Configuration

### Access Control

Enable or disable IMAP by setting the `enabled` property to `true` or `false`.

### Deleted Message Handling

The `expungeBehavior` property specifies the fate of messages deleted via IMAP:

| Value | Behavior |
|-------|----------|
| `archive` | Messages move out of inbox |
| `deleteForever` | Messages are permanently deleted |
| `trash` | Messages move to trash |

The `autoExpunge` property, when set to `true`, immediately applies these actions. Otherwise, deletions remain pending until explicitly requested by the IMAP client.
