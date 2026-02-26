# Click Webhook Documentation

## Overview

The Click webhook enables Postmark to push notifications to your application whenever a recipient clicks a tracked link in an email. This provides real-time click event data in JSON format, complementing the Messages API's ability to pull historical click data.

## Key Characteristics

**Uniqueness Definition:** A click registers as unique when a recipient clicks a particular link within a message for the first time during your retention period (default: 45 days; customizable from 7-365 days).

**Example Scenario:** If you send one email with tracked links to two recipients, and both click the same link, your server receives two webhook calls. Subsequent clicks from either recipient on that same link do not trigger additional events. However, clicks on different links within the same message each generate separate webhook events.

**Retention Period Behavior:** Links older than your retention window remain functional for redirects but won't generate click events due to inability to determine uniqueness.

## Configuration

### Via Postmark Dashboard

1. Log into Postmark and select your Server
2. Navigate to your Message Stream
3. Access the **Webhooks** tab
4. Click **Add webhook**
5. Enter your webhook URL
6. Check the **Link Click** checkbox

### Via API

```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://example.com/click-webhook",
    "Triggers": {
      "Click": {"Enabled": true}
    }
  }'
```

## Webhook Payload Structure

### Key Fields

| Field | Description |
|-------|-------------|
| RecordType | Always "Click" |
| Recipient | Email address of the person who clicked |
| OriginalLink | The URL that was clicked |
| ClickLocation | Whether click originated from HTML or Text section |
| Tag | Delivery tag from message send |
| Metadata | Custom metadata from the original email |
| ReceivedAt | ISO 8601 timestamp of click event |
| MessageID | Unique message identifier |
| MessageStream | Message stream identifier |
| Client | Browser information (name, company, family) |
| OS | Operating system details |
| Platform | Device type (Desktop, Mobile, WebMail, Unknown) |
| UserAgent | Full browser user agent string |
| Geo | Geographic data (country, region, city, coordinates, IP) |

### Complete Example Payload

```json
{
  "RecordType": "Click",
  "MessageStream": "outbound",
  "ClickLocation": "HTML",
  "Client": {
    "Name": "Chrome 35.0.1916.153",
    "Company": "Google",
    "Family": "Chrome"
  },
  "OS": {
    "Name": "OS X 10.7 Lion",
    "Company": "Apple Computer, Inc.",
    "Family": "OS X 10"
  },
  "Platform": "Desktop",
  "UserAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.153 Safari/537.36",
  "OriginalLink": "https://example.com",
  "Geo": {
    "CountryISOCode": "RS",
    "Country": "Serbia",
    "RegionISOCode": "VO",
    "Region": "Autonomna Pokrajina Vojvodina",
    "City": "Novi Sad",
    "Zip": "21000",
    "Coords": "45.2517,19.8369",
    "IP": "8.8.8.8"
  },
  "MessageID": "00000000-0000-0000-0000-000000000000",
  "Metadata": {
    "a_key": "a_value",
    "b_key": "b_value"
  },
  "ReceivedAt": "2017-10-25T15:21:11.9065619Z",
  "Tag": "welcome-email",
  "Recipient": "john@example.com"
}
```

## Local Testing with curl

To test your webhook endpoint during development:

```bash
curl <your-url> \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "RecordType": "Click",
    "ClickLocation": "HTML",
    "Client": {
      "Name": "Chrome 35.0.1916.153",
      "Company": "Google",
      "Family": "Chrome"
    },
    "OS": {
      "Name": "OS X 10.7 Lion",
      "Company": "Apple Computer, Inc.",
      "Family": "OS X 10"
    },
    "Platform": "Desktop",
    "UserAgent": "Mozilla/5.0",
    "OriginalLink": "https://example.com",
    "Geo": {
      "CountryISOCode": "RS",
      "Country": "Serbia",
      "City": "Novi Sad",
      "IP": "127.0.0.1"
    },
    "MessageID": "00000000-0000-0000-0000-000000000000",
    "ReceivedAt": "2017-10-25T15:21:11.9065619Z",
    "Tag": "welcome-email",
    "Recipient": "john@example.com"
  }'
```

## Use Cases

- **Performance Analysis:** Determine timing and geographic origin of recipient engagement with links
- **A/B Testing:** Compare effectiveness of different email layouts based on click patterns
- **Security:** Deactivate one-time links (like password resets) after first use
- **Fraud Detection:** Alert users to suspicious clicks from unfamiliar locations in near real-time
- **Engagement Tracking:** Assess transactional email effectiveness when open tracking isn't available

## Related Documentation

- [Tracking Links](../../user-guide/tracking-links/README.md)
- [Webhooks Overview](../overview/README.md)
- [Stats API](../../api-reference/stats-api/README.md)
