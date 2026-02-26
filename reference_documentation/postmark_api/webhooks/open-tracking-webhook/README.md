# Open Tracking Webhook Documentation

## Overview

The open tracking webhook enables real-time notifications when email recipients open messages. Rather than polling the API, Postmark pushes open data to your specified endpoint as events occur.

## What is an Open Webhook?

When emails are sent with open tracking enabled, Postmark monitors recipient activity. While the platform displays first-open data in the dashboard and via API queries, webhooks provide an alternative delivery method.

**Key distinction:** "If a recipient opens an email multiple times, Postmark will only save the first open" when using standard tracking. However, with the webhook configured to set `PostFirstOpenOnly` as false, the system will "POST the open information to your webhook URL every time an open occurs."

## Configuration

### Via Postmark Dashboard

1. Log into your Postmark account
2. Select your Server and Message Stream
3. Navigate to the **Webhooks** tab
4. Click **Add webhook**
5. Enter your webhook URL
6. Check the **Open** checkbox
7. Optionally configure "Post first open only"
8. Save your configuration

### Via Webhooks API

Use the Webhooks API to programmatically create or modify webhook endpoints with the `Open` trigger.

```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://example.com/open-webhook",
    "Triggers": {
      "Open": {
        "Enabled": true,
        "PostFirstOpenOnly": false
      }
    }
  }'
```

## Webhook Payload Format

### JSON Structure

```json
{
  "RecordType": "Open",
  "MessageStream": "outbound",
  "FirstOpen": true,
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
  "Platform": "WebMail",
  "UserAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_5) AppleWebKit/537.36",
  "Geo": {
    "CountryISOCode": "RS",
    "Country": "Serbia",
    "RegionISOCode": "VO",
    "Region": "Autonomna Pokrajina Vojvodina",
    "City": "Novi Sad",
    "Zip": "21000",
    "Coords": "45.2517,19.8369",
    "IP": "188.2.95.4"
  },
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "Metadata": {
    "a_key": "a_value",
    "b_key": "b_value"
  },
  "ReceivedAt": "2019-11-05T16:33:54.9070259Z",
  "Tag": "welcome-email",
  "Recipient": "john@example.com"
}
```

### Field Descriptions

| Field | Description |
|-------|-------------|
| RecordType | Always "Open" |
| Recipient | Email address of the person who opened the message |
| FirstOpen | Boolean indicating whether this was the initial open |
| Geo | Geographic data extracted from IP address (may be partially populated) |
| Metadata | Custom key-value data included with the original send |
| ReceivedAt | ISO 8601 formatted timestamp of the open event |
| Client | Browser/email client information (name, company, family) |
| OS | Operating system details |
| Platform | Access method (WebMail, Mobile, Desktop, Unknown) |
| UserAgent | Full browser user agent string |
| MessageID | Unique identifier for the opened message |
| Tag | Category label assigned to the email |
| MessageStream | The message stream through which the email was sent |

## Testing Your Webhook

Use curl to validate your endpoint accepts the proper payload format:

```bash
curl <your-url> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
  "RecordType": "Open",
  "FirstOpen": true,
  "Client": {"Name": "Chrome 35.0.1916.153", "Company": "Google"},
  "OS": {"Name": "OS X 10.7 Lion", "Company": "Apple Computer, Inc."},
  "Platform": "WebMail",
  "UserAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_5)",
  "Geo": {
    "CountryISOCode": "RS",
    "Country": "Serbia",
    "City": "Novi Sad",
    "IP": "188.2.95.4"
  },
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "ReceivedAt": "2019-05-21T18:24:47Z",
  "Tag": "welcome-email",
  "Recipient": "john@example.com"
}'
```

## Practical Applications

Open tracking data enables insights such as:

- **Engagement Analysis**: Identifying users who consistently ignore email communications
- **Geographic Intelligence**: Understanding regional distribution of your audience
- **Device Preferences**: Determining which devices and clients dominate your recipient base
- **Client Optimization**: Tailoring message formatting based on predominant clients and platforms

## Limitations

- **HTML required**: Open tracking only works with HTML emails
- **Image blocking**: Some clients block images, preventing tracking
- **Privacy features**: iOS 15+ Mail Privacy Protection may generate false positives
- **Gmail proxy**: Geographic data may be unavailable for Gmail users

## Related Documentation

- [Tracking Opens](../../user-guide/tracking-opens/README.md)
- [Webhooks Overview](../overview/README.md)
- [Stats API](../../api-reference/stats-api/README.md)
