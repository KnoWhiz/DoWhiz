# Push Notifications

## Overview

The Gmail API offers server push notifications for monitoring mailbox changes. This feature improves application performance by eliminating the extra network and compute costs involved with polling resources. For user-owned devices, the sync-based approach remains recommended.

## Initial Cloud Pub/Sub Setup

Gmail API uses Cloud Pub/Sub to deliver notifications via webhooks or polling.

### Prerequisites

- Complete Cloud Pub/Sub prerequisites
- Set up a Cloud Pub/Sub client

### Create a Topic

Use your Cloud Pub/Sub client to create a topic matching the pattern `projects/myproject/topics/*`.

### Create a Subscription

Follow Cloud Pub/Sub Subscriber Guide to establish a subscription—either webhook push (HTTP POST) or pull (app-initiated).

### Grant Publishing Rights

Grant `publish` privileges to `gmail-api-push@system.gserviceaccount.com` via Cloud Pub/Sub Developer Console. Domain restricted sharing may require configuring exceptions.

## Getting Gmail Mailbox Updates

### Watch Request

Call the `watch` endpoint to configure notifications:

```
POST https://www.googleapis.com/gmail/v1/users/me/watch
Content-type: application/json

{
  topicName: "projects/myproject/topics/mytopic",
  labelIds: ["INBOX"],
  labelFilterBehavior: "INCLUDE"
}
```

### Python Example

```python
request = {
  'labelIds': ['INBOX'],
  'topicName': 'projects/myproject/topics/mytopic',
  'labelFilterBehavior': 'INCLUDE'
}
gmail.users().watch(userId='me', body=request).execute()
```

### Watch Response

Success returns:

```json
{
  "historyId": 1234567890,
  "expiration": 1431990098200
}
```

All changes after this `historyId` trigger notifications.

### Renewing Watches

Call `watch` at least every 7 days; daily renewal is recommended.

## Receiving Notifications

Webhook notifications conform to `PubsubMessage` format:

```json
{
  "message": {
    "data": "eyJlbWFpbEFkZHJlc3MiOiAidXNlckBleGFtcGxlLmNvbSIsICJoaXN0b3J5SWQiOiAiMTIzNDU2Nzg5MCJ9",
    "messageId": "2070443601311540",
    "publishTime": "2021-02-26T19:13:55.749Z"
  },
  "subscription": "projects/myproject/subscriptions/mysubscription"
}
```

The base64url-decoded `message.data` contains:

```json
{
  "emailAddress": "user@example.com",
  "historyId": "9876543210"
}
```

Use `history.list` with the previous `historyId` to retrieve change details.

### Acknowledging Notifications

- **Webhook**: HTTP 200 response acknowledges
- **Pull delivery**: Use REST or RPC acknowledge calls

Failed acknowledgments trigger retries.

## Stopping Updates

Call the `stop` endpoint to cease receiving notifications within minutes.

## Limitations

- **Max rate**: 1 event/second per user; excess notifications drop
- **Reliability**: Typically delivered within seconds; gracefully handle delays or drops via fallback `history.list` polling
- **Cloud Pub/Sub**: Subject to API pricing and quota limits
