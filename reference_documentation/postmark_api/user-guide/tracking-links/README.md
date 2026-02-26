# Postmark Link Tracking Documentation

## Overview

Link Tracking in Postmark enables collection of engagement metrics about how recipients interact with links in emails. Unlike open tracking, this method works across both HTML and text email formats and doesn't depend on image loading permissions.

## How Link Tracking Works

When enabled, Postmark modifies links in messages to route through their servers. Recipients clicking tracked links trigger the following process:

1. The click request reaches Postmark servers
2. System records user location, browser information, and email section (HTML or Text)
3. User is immediately redirected to the original URL
4. The tracking domain displayed is `click.pstmrk.it`

### Link Persistence

"Links tracked by Postmark do not expire. Since services like Gmail retain emails for years, it's possible that your" recipients may access links long after receipt. Each tracked link contains sufficient data for redirection even if your Postmark server is deleted, ensuring continued functionality.

## Security Considerations

All tracked links use HTTPS/TLS encryption. This protects recipient-specific URLs in transactional emails from being captured through unencrypted HTTP redirects—a critical distinction from competitors using non-encrypted connections.

## Requirements for Link Tracking

### Account Prerequisites

Account approval is mandatory before link tracking activates. "Until your account is approved, no links will be encoded/tracked when sending emails through Postmark."

### URL Format Requirements

Links must meet these specifications:

| Requirement | Description |
|-------------|-------------|
| RFC-3986 compliance | Well-formed according to web standards |
| HTML encoding | Links in `<a>` elements must be HTML-encoded (Postmark auto-decodes during processing) |
| Text encoding | Do NOT HTML-encode links in TextBody |
| URL encoding | Proper percent-encoding required |
| Unicode domains | Use punycode for ASCII representation |
| Protocol requirement | Must include `http://` or `https://` |

### Structural Limitations

- Only "Open Tag" format supported (not closing tag attributes)
- Only `http` and `https` protocols tracked
- Other protocols (ftp, ssh, mailto) are not supported
- Malformed or improperly encoded links are skipped automatically

## Enabling Link Tracking

### Configuration Options

| Option | Behavior |
|--------|----------|
| `None` | Default; no link replacement or tracking |
| `HtmlAndText` | Replaces links in both formats; identical links count as single click |
| `HtmlOnly` | Tracks only HTML body links |
| `TextOnly` | Tracks only text body links |

### Implementation Levels

**Server-level configuration**: Set `TrackLinks` property on the server (applies to all messages, can be overridden per-message)

```bash
curl "https://api.postmarkapp.com/server" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{"TrackLinks": "HtmlAndText"}'
```

**API message-level**: Set `TrackLinks` property when sending via API (overrides server setting)

```json
{
  "From": "sender@example.com",
  "To": "recipient@example.com",
  "Subject": "Check out our site",
  "HtmlBody": "<a href='https://example.com'>Click here</a>",
  "TrackLinks": "HtmlAndText"
}
```

**SMTP message-level**: Add `X-PM-TrackLinks` header in SMTP transmission (overrides server setting)

```
X-PM-TrackLinks: HtmlAndText
```

## Per-Link Opt-Out

For selective link exclusion within tracked messages, add the `data-pm-no-track` attribute:

```html
<a data-pm-no-track href="https://example.com">Non-Tracked Link</a>
```

**Important**: This attribute applies to both HTML and corresponding TextBody instances of the same link.

## Click Event Data

When a click is recorded, the following data is captured:

| Field | Description |
|-------|-------------|
| `Recipient` | Email address of the clicker |
| `OriginalLink` | The URL that was clicked |
| `ClickLocation` | HTML or Text |
| `Client` | Browser information (name, company, family) |
| `OS` | Operating system details |
| `Platform` | Device type (Desktop, Mobile, WebMail, Unknown) |
| `UserAgent` | Full browser user agent string |
| `Geo` | Geographic data (country, region, city, coordinates, IP) |
| `ReceivedAt` | Timestamp of the click |
| `MessageID` | Unique identifier of the message |
| `Tag` | Tag assigned to the email |

## Accessing Click Data

### Via API

```bash
curl "https://api.postmarkapp.com/messages/outbound/clicks?count=100&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

### Via Webhook

Configure a Click Webhook to receive real-time notifications when links are clicked.

### Via Dashboard

View click statistics in the Postmark dashboard under your server's activity feed.

## Best Practices

1. **Use meaningful link text** - Helps with analytics and user experience
2. **Test before sending** - Verify tracked links redirect correctly
3. **Consider opt-out links** - Use `data-pm-no-track` for sensitive URLs
4. **Monitor click patterns** - Identify popular content and optimize
5. **Combine with open tracking** - Get a complete engagement picture

## Related Documentation

- [Click Webhook](../../webhooks/click-webhook/README.md)
- [Stats API](../../api-reference/stats-api/README.md)
- [Messages API](../../api-reference/messages-api/README.md)
