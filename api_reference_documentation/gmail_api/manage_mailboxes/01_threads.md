# Managing Threads

## Overview

The Gmail API uses Thread resources to organize email conversations. Threads group email replies with their original message into a single conversation or thread. This enables developers to retrieve all messages in a conversation sequentially, providing context and refining search results.

Key characteristics of threads:
- Labels can be applied to threads (similar to messages)
- Threads cannot be created directly, only deleted
- Messages can be inserted into existing threads

## Retrieving Threads

### Methods Available

Two primary methods exist for thread retrieval:

1. **`threads.list`** - Retrieves a set of thread IDs
2. **`threads.get`** - Retrieves a specific thread with all its messages

### Filtering Capabilities

Threads support the same query parameters as the Message resource. If any message in a thread matches the query, that thread is returned in the result.

### Code Example (Python)

The documentation provides a Python sample demonstrating how to display threads with 3+ messages:

```python
import google.auth
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

def show_chatty_threads():
  """Display threads with long conversations(>= 3 messages)"""
  creds, _ = google.auth.default()

  try:
    service = build("gmail", "v1", credentials=creds)
    threads = (
        service.users().threads().list(userId="me").execute().get("threads", [])
    )
    for thread in threads:
      tdata = (
          service.users().threads().get(userId="me", id=thread["id"]).execute()
      )
      nmsgs = len(tdata["messages"])

      if nmsgs > 2:
        msg = tdata["messages"][0]["payload"]
        subject = ""
        for header in msg["headers"]:
          if header["name"] == "Subject":
            subject = header["value"]
            break
        if subject:
          print(f"- {subject}, {nmsgs}")
    return threads

  except HttpError as error:
    print(f"An error occurred: {error}")
```

## Adding Messages to Threads

### Methods for Thread Assignment

Messages and drafts can be added to threads during:
- Creating a draft
- Updating a draft
- Sending a draft
- Inserting a message
- Sending a message

### Required Criteria

For a message or draft to be part of a thread, three conditions must be met:

1. The `threadId` must be specified on the Message or Draft.Message object
2. The `References` and `In-Reply-To` headers must comply with RFC 2822 standard
3. The `Subject` headers must match

### Implementation

To add a message to a thread, simply include a `threadId` key paired with the thread ID in the message metadata object.
