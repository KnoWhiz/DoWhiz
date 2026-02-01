# List Gmail Messages

## Overview

This documentation explains how to use the Gmail API's `users.messages.list` method to retrieve Gmail messages for an authenticated user.

## Key Details

### Method Purpose

The `users.messages.list` method returns an array of Gmail `Message` resources that contain the message `id` and `threadId`. To access complete message details, developers should use the `users.messages.get` method instead.

### Query Parameters

The listing method supports several filtering options:

| Parameter | Description |
|-----------|-------------|
| `maxResults` | Limits the number of returned messages (default: 100, maximum: 500) |
| `pageToken` | Retrieves a specific results page |
| `q` | Filters messages using query syntax (e.g., `from:user@example.com is:unread`) |
| `labelIds` | Returns only messages matching specified label IDs |
| `includeSpamTrash` | Includes SPAM and TRASH folder messages in results |

### Response Structure

The API returns a response containing:

- `messages[]` — An array of Message resources
- `nextPageToken` — Token for accessing additional results pages
- `resultSizeEstimate` — Estimated total result count

## Python Code Example

```python
import os.path
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

SCOPES = ["https://www.googleapis.com/auth/gmail.readonly"]

def main():
    """Shows basic usage of the Gmail API.
    Lists the user's Gmail messages.
    """
    creds = None
    if os.path.exists("token.json"):
        creds = Credentials.from_authorized_user_file("token.json", SCOPES)
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file("credentials.json", SCOPES)
            creds = flow.run_local_server(port=0)
        with open("token.json", "w") as token:
            token.write(creds.to_json())

    try:
        service = build("gmail", "v1", credentials=creds)
        results = (
            service.users().messages().list(userId="me", labelIds=["INBOX"]).execute()
        )
        messages = results.get("messages", [])

        if not messages:
            print("No messages found.")
            return

        print("Messages:")
        for message in messages:
            print(f'Message ID: {message["id"]}')
            msg = (
                service.users().messages().get(userId="me", id=message["id"]).execute()
            )
            print(f'  Subject: {msg["snippet"]}')

    except HttpError as error:
        print(f"An error occurred: {error}")

if __name__ == "__main__":
    main()
```

## Prerequisites

Developers need a Google Cloud project with the Gmail API enabled and should complete the Gmail API Python quickstart guide.
