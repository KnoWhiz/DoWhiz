# Suppressions API Documentation

## Overview

The Suppressions API allows management of suppressed email addresses within a Message Stream. When an address is suppressed, it becomes inactive for sending purposes. Deleting a `HardBounce` suppression effectively reactivates the associated bounce record.

## Authentication

All endpoints require the `X-Postmark-Server-Token` header with server-level API credentials.

---

## Endpoints

### 1. Suppression Dump

**Endpoint:** `GET /message-streams/{stream_id}/suppressions/dump`

Retrieves a list of all suppressed addresses in a message stream.

**Query Parameters:**

| Parameter | Description |
|-----------|-------------|
| SuppressionReason | Filter by reason: `HardBounce`, `SpamComplaint`, or `ManualSuppression` |
| Origin | Filter by source: `Recipient`, `Customer`, or `Admin` |
| fromdate | Start date filter (format: YYYY-MM-DD) |
| todate | End date filter (format: YYYY-MM-DD) |
| EmailAddress | Filter by specific email address |

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams/outbound/suppressions/dump" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Response:**
```json
{
  "Suppressions": [
    {
      "EmailAddress": "address@example.com",
      "SuppressionReason": "ManualSuppression",
      "Origin": "Recipient",
      "CreatedAt": "2019-12-17T08:58:33-05:00"
    },
    {
      "EmailAddress": "bounced@example.com",
      "SuppressionReason": "HardBounce",
      "Origin": "Customer",
      "CreatedAt": "2020-01-15T14:22:10-05:00"
    }
  ]
}
```

---

### 2. Create Suppression

**Endpoint:** `POST /message-streams/{stream_id}/suppressions`

Adds email addresses to the suppression list. Supports up to 50 addresses per request.

**Request Body:**
```json
{
  "Suppressions": [
    {
      "EmailAddress": "address@example.com"
    },
    {
      "EmailAddress": "another@example.com"
    }
  ]
}
```

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams/outbound/suppressions" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Suppressions": [
      {"EmailAddress": "address@example.com"},
      {"EmailAddress": "another@example.com"}
    ]
  }'
```

**Response:**
```json
{
  "Suppressions": [
    {
      "EmailAddress": "address@example.com",
      "Status": "Suppressed",
      "Message": null
    },
    {
      "EmailAddress": "another@example.com",
      "Status": "Suppressed",
      "Message": null
    }
  ]
}
```

**Status Values:**
- `Suppressed` - Successfully added to suppression list
- `Failed` - Could not be suppressed (check Message field)

---

### 3. Delete Suppression

**Endpoint:** `POST /message-streams/{stream_id}/suppressions/delete`

Removes addresses from the suppression list. Maximum 50 per request.

> **Important:** "SpamComplaint suppressions cannot be deleted" directly, reflecting compliance constraints.

**Request Body:**
```json
{
  "Suppressions": [
    {
      "EmailAddress": "address@example.com"
    }
  ]
}
```

**Example Request:**
```bash
curl "https://api.postmarkapp.com/message-streams/outbound/suppressions/delete" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Suppressions": [
      {"EmailAddress": "address@example.com"}
    ]
  }'
```

**Response:**
```json
{
  "Suppressions": [
    {
      "EmailAddress": "address@example.com",
      "Status": "Deleted",
      "Message": null
    }
  ]
}
```

**Status Values:**
- `Deleted` - Successfully removed from suppression list
- `Failed` - Could not be deleted (check Message field for reason)

---

## Suppression Reasons

| Reason | Description |
|--------|-------------|
| HardBounce | Email permanently undeliverable |
| SpamComplaint | Recipient marked email as spam |
| ManualSuppression | Manually added to suppression list |

## Origins

| Origin | Description |
|--------|-------------|
| Recipient | Suppression originated from recipient action |
| Customer | Suppression created by customer/user |
| Admin | Suppression created by administrator |

---

## Key Considerations

- All requests require `Content-Type: application/json`
- Server token authentication is mandatory
- Batch operations limited to 50 items per call
- Failed operations include descriptive error messages in response
- SpamComplaint suppressions cannot be deleted (compliance requirement)
- Deleting a HardBounce suppression reactivates the email address for sending

## Use Cases

1. **Maintain compliance** - Prevent sending to addresses that have bounced or complained
2. **Proactive suppression** - Add addresses you know are invalid or unsubscribed
3. **Re-engagement** - Delete suppressions for addresses that have been fixed or re-engaged
4. **Audit** - Review suppression list for compliance reporting

## Related Documentation

- [Bounce API](../bounce-api/README.md)
- [Message Streams API](../message-streams-api/README.md)
- [Subscription Change Webhook](../../webhooks/subscription-change-webhook/README.md)
