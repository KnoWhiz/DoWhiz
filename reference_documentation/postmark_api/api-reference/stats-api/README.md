# Postmark Stats API Documentation

## Overview

The Stats API provides comprehensive email statistics for outbound messages sent through Postmark. Statistics are stored permanently and do not expire. All stats use EST timezone.

**Note:** If no date range is provided, all-time statistics are returned.

---

## Authentication

All requests require the `X-Postmark-Server-Token` header with server-level privileges.

---

## Endpoints

### 1. Get Outbound Overview

**Endpoint:** `GET /stats/outbound`

Retrieves a brief overview of all outbound email statistics.

**Query Parameters:**

| Parameter | Description |
|-----------|-------------|
| tag | Filter by tag |
| fromdate | Start date (YYYY-MM-DD format, inclusive) |
| todate | End date (YYYY-MM-DD format, inclusive) |
| messagestream | Filter by message stream |

**Response Fields:**

| Field | Description |
|-------|-------------|
| Sent | Number of sent emails |
| Bounced | Number of bounced emails |
| SMTPApiErrors | Count of SMTP errors |
| BounceRate | Bounce rate percentage |
| SpamComplaints | Number of spam complaints |
| SpamComplaintsRate | Spam complaint percentage |
| Opens | Total opens |
| UniqueOpens | Unique opens |
| TotalClicks | Total link clicks |
| UniqueLinksClicked | Unique links clicked |
| Tracked | Emails with tracking enabled |
| WithLinkTracking | Emails with link tracking |
| WithOpenTracking | Emails with open tracking |

**Example Request:**
```bash
curl "https://api.postmarkapp.com/stats/outbound?fromdate=2024-01-01&todate=2024-02-01" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Example Response:**
```json
{
  "Sent": 615,
  "Bounced": 64,
  "SMTPApiErrors": 25,
  "BounceRate": 10.406,
  "SpamComplaints": 10,
  "SpamComplaintsRate": 1.626,
  "Opens": 166,
  "UniqueOpens": 26,
  "Tracked": 111,
  "WithLinkTracking": 90,
  "WithOpenTracking": 51
}
```

---

### 2. Get Sent Counts

**Endpoint:** `GET /stats/outbound/sends`

Returns total count of emails sent, broken down by date.

**Response:**
```json
{
  "Days": [
    { "Date": "2024-01-01", "Sent": 140 },
    { "Date": "2024-01-02", "Sent": 160 }
  ],
  "Sent": 615
}
```

---

### 3. Get Bounce Counts

**Endpoint:** `GET /stats/outbound/bounces`

Provides bounce statistics categorized by type.

**Response Bounce Types:**
- `HardBounce` - Permanent delivery failures
- `SoftBounce` - Temporary delivery issues
- `Transient` - Transient bounces
- `SMTPApiError` - SMTP-related errors

---

### 4. Get Spam Complaints

**Endpoint:** `GET /stats/outbound/spam`

Returns count of recipients marking emails as spam.

---

### 5. Get Tracked Email Counts

**Endpoint:** `GET /stats/outbound/tracked`

Retrieves count of emails sent with open or link tracking enabled.

---

### 6. Get Email Open Counts

**Endpoint:** `GET /stats/outbound/opens`

Returns email open statistics (requires open tracking enabled).

**Response Fields:**
- `Opens` - Total opens (including repeats)
- `Unique` - Unique opens per recipient

---

### 7. Get Email Platform Usage

**Endpoint:** `GET /stats/outbound/opens/platforms`

Breaks down opens by platform type.

**Response Categories:**
- `Desktop` - Desktop email client opens
- `Mobile` - Mobile app opens
- `WebMail` - Web-based email opens
- `Unknown` - Unidentified platform

---

### 8. Get Email Client Usage

**Endpoint:** `GET /stats/outbound/opens/emailclients`

Shows which email clients opened your messages (e.g., Apple Mail, Outlook).

---

### 9. Get Click Counts

**Endpoint:** `GET /stats/outbound/clicks`

Returns link click statistics.

**Response Fields:**
- `Clicks` - Total clicks (including repeats)
- `Unique` - Unique clicks per recipient

---

### 10. Get Browser Usage

**Endpoint:** `GET /stats/outbound/clicks/browserfamilies`

Shows which browsers opened tracked links (requires link tracking).

---

### 11. Get Browser Platform Usage

**Endpoint:** `GET /stats/outbound/clicks/platforms`

Categorizes link clicks by browser platform.

**Response Fields:**
- `Desktop` - Desktop browser clicks
- `Mobile` - Mobile browser clicks
- `Unknown` - Unidentified platform

---

### 12. Get Click Location

**Endpoint:** `GET /stats/outbound/clicks/location`

Identifies whether clicks came from HTML or text content.

**Response Fields:**
- `HTML` - Clicks from HTML links
- `Text` - Clicks from text links

---

## Common Parameters

All statistics endpoints accept these optional query parameters:

| Parameter | Description |
|-----------|-------------|
| tag | Filter results by email tag |
| fromdate | Start date (YYYY-MM-DD) |
| todate | End date (YYYY-MM-DD) |
| messagestream | Filter by message stream ID |

**Note:** "Days that didn't produce statistics won't appear in the JSON response."

---

## Response Format

All responses follow a consistent structure:
- Daily breakdowns in `Days[]` array
- Overall aggregated totals at the root level
- HTTP 200 status for successful requests
- Content-Type: application/json

---

## Example: Complete Stats Workflow

```bash
# Get overall statistics
curl "https://api.postmarkapp.com/stats/outbound" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"

# Get daily send counts
curl "https://api.postmarkapp.com/stats/outbound/sends?fromdate=2024-01-01&todate=2024-01-31" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"

# Get bounce breakdown
curl "https://api.postmarkapp.com/stats/outbound/bounces?fromdate=2024-01-01&todate=2024-01-31" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

## Related Documentation

- [Bounce API](../bounce-api/README.md)
- [Messages API](../messages-api/README.md)
- [Email API](../email-api/README.md)
