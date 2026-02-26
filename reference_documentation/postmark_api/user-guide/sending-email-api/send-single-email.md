# Sending a Single Email with Postmark

## Overview

Postmark enables single email delivery through an HTTP POST request to the `/email` endpoint with a JSON-formatted message in the request body.

## Basic Request Structure

### Authentication

All requests require the `X-Postmark-Server-Token` header for authentication. This header is case-insensitive and provides server-level access control. Requests without valid credentials receive an HTTP 401 (Unauthorized) response.

**Header format:**
```
X-Postmark-Server-Token: [your-server-token]
```

For testing, use `POSTMARK_API_TEST` as the token value to validate message structure without delivery.

### cURL Example

```bash
curl "https://api.postmarkapp.com/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
  "From": "sender@example.com",
  "To": "receiver@example.com",
  "Subject": "Postmark test",
  "TextBody": "Hello dear Postmark user.",
  "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
  "MessageStream": "outbound"
}'
```

## JSON Message Format

### Required and Core Fields

| Field | Purpose | Notes |
|-------|---------|-------|
| `From` | Sender address | Must have registered, confirmed sender signature |
| `To` | Recipient address(es) | Comma-separated; max 50 total recipients with Cc/Bcc |
| `Subject` | Email subject | Maximum 2000 characters (UTF-16 code points) |
| `TextBody` | Plain text content | Up to 5MB |
| `HtmlBody` | HTML content | Up to 5MB |

### Optional Fields

- **`Cc`**: Carbon copy recipients (comma-separated)
- **`Bcc`**: Blind carbon copy recipients (comma-separated)
- **`ReplyTo`**: Reply-to address
- **`Tag`**: Categorization label (max 1000 characters)
- **`MessageStream`**: Stream identifier; defaults to outbound transactional stream
- **`TrackOpens`**: Enable open tracking (boolean)
- **`TrackLinks`**: Enable link tracking ("HtmlOnly", "HtmlAndText", etc.)
- **`Metadata`**: Custom key-value pairs for tracking
- **`Headers`**: Array of custom headers

### Complete Example

```json
{
  "From": "sender@example.com",
  "To": "receiver@example.com",
  "Cc": "copied@example.com",
  "Bcc": "blank-copied@example.com",
  "Subject": "Test",
  "Tag": "Invitation",
  "HtmlBody": "<b>Hello</b>",
  "TextBody": "Hello",
  "ReplyTo": "reply@example.com",
  "Metadata": {
    "Color": "blue",
    "Client-Id": "12345"
  },
  "Headers": [
    {
      "Name": "CUSTOM-HEADER",
      "Value": "value"
    }
  ],
  "TrackOpens": true,
  "TrackLinks": "HtmlOnly",
  "MessageStream": "outbound"
}
```

## Sender Information

- Sender names can override registered signatures: `"From": "John Smith <sender@example.com>"`
- Names with punctuation require quotes: `"To": "\"Joe Receiver, Jr.\" <receiver@example.com>"`
- From address character limit: 255 characters

## Tracking Features

### Open Tracking

Include `"TrackOpens": true` to monitor message opens. Configure at the server or per-message level.

### Link Tracking

Enable with the `TrackLinks` property:
```json
{
  "TrackLinks": "HtmlAndText"
}
```

Options include "HtmlOnly", "HtmlAndText", or "TextOnly".

## Attachments

### Structure

Attachments appear in an array with the following properties:

| Property | Description |
|----------|-------------|
| `Name` | Display filename |
| `Content` | Base64-encoded file data |
| `ContentType` | MIME type |
| `ContentID` | Optional; required for inline images |

### Forbidden File Types

vbs, exe, bin, bat, chm, com, cpl, crt, hlp, hta, inf, ins, isp, jse, lnk, mdb, pcd, pif, reg, scr, sct, shs, vbe, vba, wsf, wsh, wsl, msc, msi, msp, mst

### Size Limitations

- Individual bodies: Up to 5MB each
- Total message size: Up to 10MB (including all attachments)
- Base64-encoding counts toward size limits

### Example with Attachments

```json
{
  "Attachments": [
    {
      "Name": "readme.txt",
      "Content": "dGVzdCBjb250ZW50",
      "ContentType": "text/plain"
    },
    {
      "Name": "report.pdf",
      "Content": "dGVzdCBjb250ZW50",
      "ContentType": "application/octet-stream"
    }
  ]
}
```

## Inline Images

For HTML emails with embedded images:

```json
{
  "Attachments": [
    {
      "Name": "bacon.jpg",
      "Content": "/9j/4AAQSkZJRgABAgEAAAAAAAD/4",
      "ContentType": "image/jpeg",
      "ContentID": "cid:part1.01030607.06070005@gmail.com"
    }
  ]
}
```

HTML reference:
```html
<img src="cid:part1.01030607.06070005@gmail.com">
```

## Success Response

A successful request returns:

```json
{
  "ErrorCode": 0,
  "Message": "OK",
  "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
  "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
  "To": "receiver@example.com"
}
```

The `MessageID` field enables correlation with bounce webhooks and bounce API responses.

## Best Practices

- Send emails with attachments from background jobs rather than request handlers
- Store the `MessageID` for tracking bounce events
- Configure sender signatures before sending production emails
- Use multipart messages (both `TextBody` and `HtmlBody`) for broader client compatibility
