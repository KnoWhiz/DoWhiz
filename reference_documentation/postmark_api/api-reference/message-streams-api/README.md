# Message Streams API Documentation

## Overview

The Message Streams API enables management of message streams for a specific server. Key constraints include:

- Maximum of 10 streams per server (including defaults)
- Default streams cannot be deleted
- Only 1 inbound stream allowed per server

## Authentication

All endpoints require:
- **Header**: `X-Postmark-Server-Token` (server-level API token)
- **Header**: `Accept: application/json`

---

## Endpoints

### 1. List Message Streams

**Request**: `GET /message-streams`

**Query Parameters**:

| Parameter | Description | Default |
|-----------|-------------|---------|
| MessageStreamType | Filter by type (`All`, `Inbound`, `Transactional`, `Broadcasts`) | `All` |
| IncludeArchivedStreams | Include archived streams | `False` |

**Response Fields**:
- `MessageStreams[]`: Array of stream objects
- `TotalCount`: Number of streams retrieved

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Example Response:**
```json
{
  "TotalCount": 3,
  "MessageStreams": [
    {
      "ID": "outbound",
      "ServerID": 12345,
      "Name": "Transactional Stream",
      "Description": "Default transactional stream",
      "MessageStreamType": "Transactional",
      "CreatedAt": "2020-01-01T00:00:00Z",
      "UpdatedAt": null,
      "ArchivedAt": null,
      "ExpectedPurgeDate": null,
      "SubscriptionManagementConfiguration": {
        "UnsubscribeHandlingType": "none"
      }
    }
  ]
}
```

---

### 2. Get Single Message Stream

**Request**: `GET /message-streams/{stream_ID}`

Returns detailed information about a specific stream with the same response structure as listing.

---

### 3. Create Message Stream

**Request**: `POST /message-streams`

**Required Body Fields**:

| Field | Type | Description |
|-------|------|-------------|
| ID | string | Unique identifier for sending messages to this stream |
| Name | string | Display name |
| MessageStreamType | string | `Broadcasts` or `Transactional` |

**Optional Fields**:
- `Description`: Stream description
- `SubscriptionManagementConfiguration`: Unsubscribe handling preferences

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "ID": "marketing-emails",
    "Name": "Marketing Emails",
    "MessageStreamType": "Broadcasts",
    "Description": "Marketing and promotional emails"
  }'
```

---

### 4. Edit Message Stream

**Request**: `PATCH /message-streams/{stream_ID}`

**Body Parameters**:

| Field | Type | Description |
|-------|------|-------------|
| Name | string | Updated stream name |
| Description | string | Updated description |
| SubscriptionManagementConfiguration | object | Unsubscribe settings |

**Note**: "Broadcast streams require unsubscribe management, with `Postmark` as default."

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams/marketing-emails" \
  -X PATCH \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Name": "Marketing & Announcements",
    "SubscriptionManagementConfiguration": {
      "UnsubscribeHandlingType": "Postmark"
    }
  }'
```

---

### 5. Archive Message Stream

**Request**: `POST /message-streams/{stream_ID}/archive`

**Header**: `Content-Length: 0` (required)

**Response**:
- `ID`: Stream identifier
- `ServerID`: Associated server
- `ExpectedPurgeDate`: "Stream is deleted 45 days after archiving date. Until this date, it can be restored."

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams/marketing-emails/archive" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "Content-Length: 0" \
  -H "X-Postmark-Server-Token: server token"
```

---

### 6. Unarchive Message Stream

**Request**: `POST /message-streams/{stream_ID}/unarchive`

**Header**: `Content-Length: 0` (required)

Restores archived stream with full stream details returned.

---

## Message Stream Types

| Type | Description | Default Unsubscribe |
|------|-------------|---------------------|
| Transactional | For operational messages | `none` |
| Broadcasts | For marketing messages | `Postmark` (required) |
| Inbound | For receiving messages (only 1 per server) | `none` |

## Unsubscribe Handling Options

| Option | Description |
|--------|-------------|
| `none` | No unsubscribe management |
| `Postmark` | Native unsubscribe handling |
| `Custom` | Approved accounts only |

## Stream Object Fields

| Field | Type | Description |
|-------|------|-------------|
| ID | string | Unique stream identifier |
| ServerID | integer | Associated server ID |
| Name | string | Display name |
| Description | string | Optional description |
| MessageStreamType | string | Stream category |
| CreatedAt | string | Creation timestamp (ISO 8601) |
| UpdatedAt | string | Last modification timestamp (nullable) |
| ArchivedAt | string | Archive timestamp (nullable) |
| ExpectedPurgeDate | string | Deletion date for archived streams |
| SubscriptionManagementConfiguration | object | Unsubscribe handling settings |

## Related Documentation

- [Server API](../server-api/README.md)
- [Email API](../email-api/README.md)
- [Webhooks API](../webhooks-api/README.md)
