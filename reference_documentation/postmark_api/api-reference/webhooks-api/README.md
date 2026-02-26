# Webhooks API Documentation

## Overview

The Webhooks API enables management of webhooks for Transactional and Broadcast Message Streams. Webhooks allow you to receive HTTP POST notifications for various email events.

## Authentication

All webhook endpoints require:
- **Header**: `X-Postmark-Server-Token` (server-level API token)
- **Content-Type**: `application/json`
- **Accept**: `application/json`

---

## Endpoints

### 1. List Webhooks

**Endpoint**: `GET /webhooks`

Retrieves all webhooks for a server, optionally filtered by message stream.

**Query Parameters**:

| Parameter | Description |
|-----------|-------------|
| MessageStream | Filter results by stream name (optional) |

**Example Request**:
```bash
curl "https://api.postmarkapp.com/webhooks?MessageStream=outbound" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Response Structure**:
```json
{
  "Webhooks": [
    {
      "ID": 1234567,
      "Url": "https://www.example.com/webhook-test",
      "MessageStream": "outbound",
      "HttpAuth": {
        "Username": "user",
        "Password": "pass"
      },
      "HttpHeaders": [
        {
          "Name": "header-name",
          "Value": "header-value"
        }
      ],
      "Triggers": {
        "Open": { "Enabled": true, "PostFirstOpenOnly": false },
        "Click": { "Enabled": true },
        "Delivery": { "Enabled": true },
        "Bounce": { "Enabled": false, "IncludeContent": false },
        "SpamComplaint": { "Enabled": false, "IncludeContent": false },
        "SubscriptionChange": { "Enabled": true }
      }
    }
  ]
}
```

---

### 2. Get a Webhook

**Endpoint**: `GET /webhooks/{Id}`

Retrieves details for a specific webhook by ID.

---

### 3. Create a Webhook

**Endpoint**: `POST /webhooks`

Creates a new webhook for event notifications.

**Request Body Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| Url | string | Yes | Target webhook URL |
| MessageStream | string | No | Stream association (defaults to "outbound") |
| HttpAuth | object | No | Username/password credentials |
| HttpHeaders | array | No | Custom headers to include |
| Triggers | object | No | Event types to enable |

**Trigger Options**:

| Trigger | Properties |
|---------|------------|
| Open | `Enabled`, `PostFirstOpenOnly` |
| Click | `Enabled` |
| Delivery | `Enabled` |
| Bounce | `Enabled`, `IncludeContent` |
| SpamComplaint | `Enabled`, `IncludeContent` |
| SubscriptionChange | `Enabled` |

**Example Request**:
```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://www.example.com/webhook-handler",
    "MessageStream": "outbound",
    "HttpAuth": {
      "Username": "user",
      "Password": "pass"
    },
    "HttpHeaders": [
      {
        "Name": "Authorization",
        "Value": "Bearer token123"
      }
    ],
    "Triggers": {
      "Open": { "Enabled": true, "PostFirstOpenOnly": false },
      "Click": { "Enabled": true },
      "Delivery": { "Enabled": true },
      "Bounce": { "Enabled": true, "IncludeContent": false },
      "SpamComplaint": { "Enabled": true, "IncludeContent": false },
      "SubscriptionChange": { "Enabled": true }
    }
  }'
```

---

### 4. Edit a Webhook

**Endpoint**: `PUT /webhooks/{Id}`

Updates an existing webhook configuration. Only specified fields are modified.

**Request Body Parameters** (all optional):
- `Url`: New webhook URL
- `HttpAuth`: Updated credentials
- `HttpHeaders`: New header pairs
- `Triggers`: Event configuration changes

---

### 5. Delete a Webhook

**Endpoint**: `DELETE /webhooks/{Id}`

Permanently removes a webhook from your server.

**Example Response**:
```json
{
  "ErrorCode": 0,
  "Message": "Webhook 1234 removed."
}
```

---

## Webhook Event Types

| Event | Triggered When |
|-------|----------------|
| Open | Recipient opens email |
| Click | Recipient clicks tracked link |
| Delivery | Email successfully delivered |
| Bounce | Email bounces (hard/soft) |
| SpamComplaint | Recipient marks as spam |
| SubscriptionChange | List subscription status changes |

---

## Key Features

- **Custom HTTP Auth**: Secure webhooks with username/password authentication
- **Custom Headers**: Add custom headers to webhook requests
- **Selective Triggers**: Enable/disable specific event types per webhook
- **Content Options**: Include full email content in bounce and spam complaint notifications
- **First Open Only**: Optionally limit open tracking to first recipient open
- **Message Streams**: Associate webhooks with specific transactional or broadcast streams

---

## Notes

- Default message stream is `outbound` if not specified during creation
- When editing a webhook, only provided fields are updated; omitted fields remain unchanged
- Server-level API tokens are required for all webhook operations
- Webhook endpoints must be publicly accessible HTTP(S) URLs

## Related Documentation

- [Webhooks Overview](../../webhooks/overview/README.md)
- [Bounce Webhook](../../webhooks/bounce-webhook/README.md)
- [Server API](../server-api/README.md)
