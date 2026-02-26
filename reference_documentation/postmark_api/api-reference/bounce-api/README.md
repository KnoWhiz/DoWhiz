# Postmark Bounce API Documentation

## Overview

The Bounce API provides access to bounce reports for a specific server. Bounce records remain available for 45 days by default (the standard retention period).

## API Endpoints

### 1. Get Delivery Stats

**Endpoint:** `GET /deliverystats`

**Purpose:** Retrieve overall bounce statistics and inactive mail counts

**Required Headers:**
- `Accept: application/json`
- `X-Postmark-Server-Token` (server-level authentication token)

**Response Fields:**
- `InactiveMails` (integer): Count of deactivated email addresses
- `Bounces` (array): Collection of bounce type counts

**Example Request:**
```bash
curl "https://api.postmarkapp.com/deliverystats" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Example Response:**
```json
{
  "InactiveMails": 192,
  "Bounces": [
    {
      "Name": "Hard bounce",
      "Type": "HardBounce",
      "Count": 82
    },
    {
      "Name": "Soft bounce",
      "Type": "SoftBounce",
      "Count": 110
    }
  ]
}
```

---

### 2. Get Bounces

**Endpoint:** `GET /bounces`

**Purpose:** Search and retrieve up to 10,000 bounces with filtering options

**Query Parameters:**

| Parameter | Required | Description |
|-----------|----------|-------------|
| count | Yes | Results per request (max 500) |
| offset | Yes | Number of records to skip |
| type | No | Filter by bounce classification |
| inactive | No | Boolean to filter deactivated addresses |
| emailFilter | No | Target specific email address |
| tag | No | Filter by message tag |
| messageID | No | Retrieve by message identifier |
| fromdate | No | Start timestamp (Eastern Time, format: YYYY-MM-DDT12:00:00) |
| todate | No | End timestamp (Eastern Time, format: YYYY-MM-DDT23:00:00) |
| messagestream | No | Filter by stream ID (defaults to outbound) |

**Example Request:**
```bash
curl "https://api.postmarkapp.com/bounces?count=50&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Response Fields:**
- `TotalCount`: Total matching records
- `Bounces`: Array of bounce objects

---

### 3. Get Single Bounce

**Endpoint:** `GET /bounces/{bounceid}`

**Purpose:** Retrieve detailed information about a specific bounce event

**Example Request:**
```bash
curl "https://api.postmarkapp.com/bounces/12345" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Response Fields:**
- Complete bounce details including type code, description, raw SMTP details, subject line, and full message content

---

### 4. Get Bounce Dump

**Endpoint:** `GET /bounces/{bounceid}/dump`

**Purpose:** Access raw SMTP source data (available for 30 days)

**Response:**
- `Body`: Raw bounce message source or empty string if unavailable

---

### 5. Activate a Bounce

**Endpoint:** `PUT /bounces/{bounceid}/activate`

**Purpose:** Reactivate a deactivated email address for future sending

**Required Headers:**
- `Content-Type: application/json`
- `Accept: application/json`
- `X-Postmark-Server-Token`

**Example Request:**
```bash
curl "https://api.postmarkapp.com/bounces/12345/activate" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Response:**
- `Message`: Confirmation status
- `Bounce`: Updated bounce object with reactivation status

---

## Bounce Type Classifications

| Type | Code | Description |
|------|------|-------------|
| HardBounce | 1 | Permanent delivery failure (unknown user, invalid mailbox) |
| Transient | 2 | Temporary delivery issue or network delay |
| Unsubscribe | 16 | Unsubscribe/removal request |
| Subscribe | 32 | Subscription request |
| AutoResponder | 64 | Automatic reply message |
| AddressChange | 128 | Recipient requested address update |
| DnsError | 256 | Temporary DNS lookup failure |
| SpamNotification | 512 | Message blocked or classified as bulk/spam |
| SoftBounce | 4096 | Temporary delivery failure (full mailbox, disabled account) |
| SpamComplaint | 100001 | Subscriber marked as spam |
| BadEmailAddress | 100000 | Invalid email format |
| DMARCPolicy | 100009 | Rejected due to DMARC authentication policy |
| TemplateRenderingFailed | 100010 | Template processing error |

---

## Key Features

- **Search Limits:** Maximum 10,000 results per query; use date filters for larger datasets
- **Data Retention:** Standard 45-day retention window
- **Reactivation:** Some bounces allow reactivation via the activate endpoint
- **Raw Data Access:** Bounce dumps provide complete SMTP transaction records (30-day availability)
- **Filtering:** Comprehensive filtering by type, date range, email, tags, and message streams

---

## Authentication

All endpoints require the `X-Postmark-Server-Token` header, which should be obtained from the API Tokens section within your Postmark server settings.

## Related Documentation

- [Bounce Webhook](../../webhooks/bounce-webhook/README.md)
- [Email API](../email-api/README.md)
- [Suppressions API](../suppressions-api/README.md)
