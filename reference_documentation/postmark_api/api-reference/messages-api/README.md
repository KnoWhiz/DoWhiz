# Postmark Messages API Documentation

## Overview

The Messages API enables retrieval of detailed information about outbound and inbound messages processed through a Postmark server. "Messages expire after your retention period, which is 45 days by default (but retention can be customized from 7 to 365 days)."

---

## Outbound Message Search

**Endpoint:** `GET /messages/outbound`

Retrieve up to 10,000 messages matching search criteria. For larger result sets, use date filtering.

### Required Headers
- `Accept: application/json`
- `X-Postmark-Server-Token`: Server-level API token

### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| count | integer | Messages per request (max 500) |
| offset | integer | Messages to skip |
| recipient | string | Filter by recipient email |
| fromemail | string | Filter by sender address |
| tag | string | Filter by message tag |
| status | string | Filter by status: `queued`, `sent`, or `processed` |
| todate | string | Filter until date (ISO format, Eastern Time) |
| fromdate | string | Filter from date (ISO format, Eastern Time) |
| subject | string | Filter by email subject |
| messagestream | string | Filter by stream ID (defaults to 'outbound') |
| metadata_* | string | Filter by custom metadata field |

### Example Request
```bash
curl "https://api.postmarkapp.com/messages/outbound?count=100&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

### Response
```json
{
  "TotalCount": 150,
  "Messages": [
    {
      "Tag": "welcome-email",
      "MessageID": "abc123",
      "MessageStream": "outbound",
      "To": [{"Email": "user@example.com", "Name": "User"}],
      "Cc": [],
      "Bcc": [],
      "Recipients": ["user@example.com"],
      "ReceivedAt": "2023-01-15T10:30:00Z",
      "From": "sender@example.com",
      "Subject": "Welcome!",
      "Attachments": [],
      "Status": "Sent",
      "TrackOpens": true,
      "TrackLinks": "HtmlAndText",
      "Metadata": {"campaign": "onboarding"},
      "Sandboxed": false
    }
  ]
}
```

---

## Outbound Message Details

**Endpoint:** `GET /messages/outbound/{messageid}/details`

Retrieve comprehensive details including message bodies and event history.

### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| TextBody | string | Plain text version |
| HtmlBody | string | HTML version |
| Body | string | Raw SMTP source |
| MessageEvents | array | Event history (Delivered, Bounced, Opened, etc.) |
| Status | string | Current status |
| TrackOpens | boolean | Open tracking enabled |
| TrackLinks | string | Link tracking mode |

MessageEvents array includes objects with Recipient, Type, ReceivedAt, and Details fields.

---

## Outbound Message Dump

**Endpoint:** `GET /messages/outbound/{messageid}/dump`

Retrieve raw SMTP source of the message.

### Response
- **Body**: Raw message source (empty string if unavailable)

---

## Inbound Message Search

**Endpoint:** `GET /messages/inbound`

Search received messages with filtering options.

### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| count | integer | Messages per request (max 500) |
| offset | integer | Messages to skip |
| recipient | string | Filter by recipient |
| fromemail | string | Filter by sender |
| tag | string | Filter by tag |
| subject | string | Filter by subject |
| mailboxhash | string | Filter by mailbox hash |
| status | string | Filter by status (blocked, processed, queued, failed, scheduled) |
| todate | string | Filter until date |
| fromdate | string | Filter from date |

Default status filter is `processed` when unspecified.

---

## Inbound Message Details

**Endpoint:** `GET /messages/inbound/{messageid}/details`

Retrieve full inbound message content and metadata.

### Response Fields

| Field | Description |
|-------|-------------|
| From/FromName/FromFull | Sender information |
| To/ToFull | Recipient details |
| Subject, Date | Message metadata |
| TextBody/HtmlBody | Message content |
| Headers | Array of email headers |
| Attachments | Array with Name, ContentType, ContentLength |
| Status | Message status |
| BlockedReason | If blocked, the reason provided |

---

## Inbound Message Management

### Bypass Rules for Blocked Message

**Endpoint:** `PUT /messages/inbound/{messageid}/bypass`

Override blocking rules for a specific message.

### Retry Failed Inbound Message

**Endpoint:** `PUT /messages/inbound/{messageid}/retry`

Reschedule a failed inbound message for reprocessing.

---

## Message Opens Tracking

### Search All Opens

**Endpoint:** `GET /messages/outbound/opens`

Search email opens with filtering by client, OS, platform, and geography.

**Key Parameters:**
- `recipient`, `tag`, `messagestream`
- `client_name`, `client_company`, `client_family`
- `os_name`, `os_family`, `os_company`
- `platform`, `country`, `region`, `city`

### Opens for Single Message

**Endpoint:** `GET /messages/outbound/opens/{messageid}`

**Note:** "Postmark API only stores the first open, so TotalCount will always equal 1."

### Response Fields
- **TotalCount**: Number of opens
- **Opens**: Array containing client info, OS details, platform, UserAgent, Geo data, ReceivedAt timestamp

---

## Message Clicks Tracking

### Search All Clicks

**Endpoint:** `GET /messages/outbound/clicks`

Track link clicks with extensive filtering options (recipient, tag, client, OS, platform, geography).

**Response includes:** ClickLocation (HTML/Text), Client, OS, Platform, OriginalLink, Geo, MessageID, ReceivedAt

### Clicks for Single Message

**Endpoint:** `GET /messages/outbound/clicks/{messageid}`

Retrieve all clicks on links within a specific message.

---

## Key Constraints

| Constraint | Value |
|------------|-------|
| Maximum messages per search | 10,000 |
| Maximum results per API call | 500 |
| Count + Offset maximum | 10,000 |
| Default timezone | Eastern Time Zone |
| Metadata fields searchable | One at a time |

## Related Documentation

- [Email API](../email-api/README.md)
- [Bounce API](../bounce-api/README.md)
- [Stats API](../stats-api/README.md)
