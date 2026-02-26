# Email Open Tracking Guide

## Overview

Postmark's open tracking feature embeds an invisible pixel into HTML emails, allowing you to monitor when recipients view your messages. The system automatically handles pixel insertion and data collection.

## Key Capabilities

Open tracking provides valuable business insights:

- **Individual recipient tracking**: Monitor specific user engagement patterns
- **Tag-based analytics**: Compare open rates across different email campaigns
- **Geographic data**: Identify regional distribution of email opens
- **Client insights**: Determine which email applications your audience uses
- **Platform detection**: See whether readers use mobile or desktop devices
- **OS information**: Track operating system usage among your recipients

## Important Limitations

### Technical Constraints

| Limitation | Description |
|------------|-------------|
| **HTML requirement** | "We can only add a tracking pixel to HTML emails. If you send plain text only emails or if a user only reads the plain text version, Postmark can't track opens for those emails." |
| **Image blocking** | Emails viewed in clients that block image loading cannot be tracked |
| **Ad blocker interference** | Users with ad blockers may prevent pixel requests from being recorded |

### Platform-Specific Considerations

**Gmail**: Gmail's image proxy server allows open detection but prevents geographic and OS data collection.

**Apple Mail**: iOS 15+ Mail Privacy Protection can generate false-positive open reports.

### Data Retention

- Individual open records: stored for 45 days
- Aggregated statistics: persist indefinitely

## Enabling Open Tracking

### Server-Level Configuration

Enable open tracking for all emails sent through a server via the Postmark dashboard or Server API:

```bash
curl "https://api.postmarkapp.com/server" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{"TrackOpens": true}'
```

### Per-Message Configuration

Override server settings on individual messages:

```json
{
  "From": "sender@example.com",
  "To": "recipient@example.com",
  "Subject": "Hello",
  "HtmlBody": "<html><body>Hello!</body></html>",
  "TrackOpens": true
}
```

### SMTP Header

For SMTP sending, add the header:
```
X-PM-TrackOpens: true
```

## Accessing Open Data

### Via API

Use the Messages API to retrieve open information:

```bash
curl "https://api.postmarkapp.com/messages/outbound/opens?count=100&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

### Via Webhook

Configure an Open Tracking Webhook to receive real-time notifications when emails are opened.

### Via Dashboard

View open statistics in the Postmark dashboard under your server's activity feed.

## Open Event Data

When an open is detected, the following data is captured:

| Field | Description |
|-------|-------------|
| `Recipient` | Email address that opened |
| `ReceivedAt` | Timestamp of the open |
| `Client` | Email client information (name, company, family) |
| `OS` | Operating system details |
| `Platform` | Access type (WebMail, Desktop, Mobile, Unknown) |
| `UserAgent` | Full user agent string |
| `Geo` | Geographic data (country, region, city, coordinates, IP) |
| `MessageID` | Unique identifier of the opened message |
| `Tag` | Tag assigned to the email |

## Best Practices

1. **Use HTML emails** - Plain text emails cannot be tracked
2. **Set realistic expectations** - Open rates are underreported due to image blocking
3. **Consider privacy** - Some users and organizations block tracking
4. **Combine with click tracking** - Get a more complete picture of engagement
5. **Monitor trends, not absolutes** - Open rates are best used for comparative analysis

## Related Documentation

- [Open Tracking Webhook](../../webhooks/open-tracking-webhook/README.md)
- [Stats API](../../api-reference/stats-api/README.md)
- [Messages API](../../api-reference/messages-api/README.md)
