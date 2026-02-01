# Searching for Messages in Gmail API

## Overview

The Gmail API provides methods to search and filter messages using the `messages.list` and `threads.list` methods. These endpoints accept a `q` parameter supporting most of the same advanced search syntax as the Gmail web-interface.

## Core Methods

- **`messages.list`**: Search individual messages
- **`threads.list`**: Search conversation threads

Both support the `q` parameter for advanced queries and `labelIds` parameter for filtering by labels.

## Search Query Syntax

The API accepts advanced search operators similar to Gmail's web interface. Example query to retrieve messages sent in January 2014:

```
GET https://www.googleapis.com/gmail/v1/users/me/messages?q=in:sent after:2014/01/01 before:2014/02/01
```

## Important Date Handling

**Warning**: All dates in search queries are interpreted as midnight PST. For accurate results in other timezones, pass values in seconds:

```
?q=in:sent after:1388552400 before:1391230800
```

## Label-Based Filtering

Beyond text queries, you can filter using the `labelIds` parameter to search messages with specific system or user-applied labels.

## Key Differences: Gmail UI vs. API

The API differs from Gmail's web interface in two significant ways:

1. **No Alias Expansion**: The API doesn't infer account aliases—only exact matches return results
2. **No Thread-Wide Searches**: The API performs message-level searches, not thread-level searches

Consult the official method reference documentation for complete parameter details.
