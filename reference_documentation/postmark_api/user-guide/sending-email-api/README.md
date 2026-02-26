# Sending Email with Postmark API

## Overview

Postmark provides straightforward HTTP POST endpoints for sending emails via their API. This guide covers both single email and batch email sending capabilities.

## Single Email Sending

### Endpoint

```
POST https://api.postmarkapp.com/email
```

### Authentication

All requests require the `X-Postmark-Server-Token` header for authentication. Each Postmark server has its own API token, enabling data isolation across applications.

**Header format:**
```
X-Postmark-Server-Token: your_server_token
```

The header name and value are case-insensitive. Missing or incorrect authentication returns an HTTP 401 (Unauthorized) response.

**Testing Note:** Use `POSTMARK_API_TEST` as your server token to send test emails that won't be delivered to recipients, useful for validating data formats.

### Example cURL Request

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

### JSON Message Format

#### Core Fields

| Field | Description | Required |
|-------|-------------|----------|
| `From` | Sender email address; supports format: `"John Doe <email@example.com>"` | Yes |
| `To` | Recipient email (comma-separated for multiple, max 50 total recipients across To/Cc/Bcc) | Yes |
| `Cc` | Carbon copy recipients | No |
| `Bcc` | Blind carbon copy recipients | No |
| `Subject` | Email subject (max 2000 characters) | Yes |
| `TextBody` | Plain text message body (max 5MB) | No* |
| `HtmlBody` | HTML message body (max 5MB) | No* |
| `ReplyTo` | Reply-to email address | No |
| `MessageStream` | Stream ID for categorization (defaults to outbound) | No |

*At least one body type (TextBody or HtmlBody) is required. Providing both enables multipart messages.

#### Optional Fields

- **Tag**: Categorize emails for detailed statistics. Single tag per message, max 1000 characters.
- **Metadata**: Custom key-value pairs for tracking purposes.
- **Headers**: Array of custom email headers with Name/Value pairs.
- **TrackOpens**: Boolean to enable open tracking.
- **TrackLinks**: Options include "HtmlOnly", "HtmlAndText", or "None".

#### Important Sender Requirements

A registered and confirmed sender signature must exist for the `From` address, or the request returns HTTP 422 (Unprocessable Entity).

You can override the name in a sender signature: use `"John Smith <sender@example.com>"` to customize the display name while maintaining the verified email address.

#### Name Handling

For addresses with punctuation in names: `"To": "\"Joe Receiver, Jr.\" <receiver@example.com>"`

#### Custom Message-ID

Include custom Message-ID headers via the Headers array:
```json
"Headers": [{"Name":"Message-ID", "Value": "<my-id-123@example.com>"}]
```

### Complete Message Example

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

### Attachments

Attachments are submitted as an array with individual attachment objects:

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

#### Attachment Fields

- **Name**: Filename displayed to recipients
- **Content**: Base64-encoded binary data
- **ContentType**: MIME type for client interpretation

#### Forbidden File Types

Postmark blocks attachments with these extensions to prevent malware distribution: vbs, exe, bin, bat, chm, com, cpl, crt, hlp, hta, inf, ins, isp, jse, lnk, mdb, pcd, pif, reg, scr, sct, shs, vbe, vba, wsf, wsh, wsl, msc, msi, msp, mst.

#### Size Limitations

- TextBody and HtmlBody: up to 5MB each
- Total message size including attachments: up to 10MB
- Base64-encoding increases apparent size; Postmark calculates sizes post-encoding
- Recommendation: Send emails with attachments from background jobs rather than synchronous web request handlers

### Inline Images

Embed images directly in HTML messages using base64-encoding and Content IDs:

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

Reference in HTML:
```html
<img src="cid:part1.01030607.06070005@gmail.com">
```

**Note:** If referencing an image multiple times, attach it once; Postmark reuses it automatically.

### Successful Response

```json
{
  "ErrorCode": 0,
  "Message": "OK",
  "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
  "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
  "To": "receiver@example.com"
}
```

The `MessageID` field enables tracking and linking messages to bounce webhooks or bounce API results.

### Attachment Retrieval

Postmark does not store attachments after sending. They are not retrievable via UI, API, or webhooks.

---

## Batch Email Sending

### Endpoint

```
POST https://api.postmarkapp.com/email/batch
```

### Capabilities

- Send up to 500 messages per API call
- Maximum payload size: 50 MB (including attachments)
- Use `MessageStream` field to set sending stream for broadcast messages
- Same JSON message format as single email sending

### Example cURL Request

```bash
curl "https://api.postmarkapp.com/email/batch" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '[{
    "From": "sender@example.com",
    "To": "receiver1@example.com",
    "Subject": "Postmark test #1",
    "TextBody": "Hello dear Postmark user.",
    "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
    "MessageStream": "outbound"
  },
  {
    "From": "sender@example.com",
    "To": "receiver2@example.com",
    "Subject": "Postmark test #2",
    "TextBody": "Hello dear Postmark user.",
    "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
    "MessageStream": "outbound"
  }
]'
```

### Batch Response

The API returns a matching JSON array with individual responses for each submitted message:

```json
[
  {
    "ErrorCode": 0,
    "Message": "OK",
    "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
    "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
    "To": "receiver1@example.com"
  },
  {
    "ErrorCode": 0,
    "Message": "OK",
    "MessageID": "e2ecbbfc-fe12-463d-b933-9fe22915106d",
    "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
    "To": "receiver2@example.com"
  }
]
```

Each message in the batch receives individual response data, enabling granular error handling.

---

## Tracking Features

### Open Tracking

Enable per-message by setting `"TrackOpens": true` in the JSON payload. Alternatively, configure at the server level for all emails.

### Link Tracking

Enable via `"TrackLinks"` with values:
- `"HtmlAndText"`: Track clicks in both HTML and plain text links
- `"HtmlOnly"`: Track HTML links only
- `"None"`: Disable tracking

Configure per-message or at the server level.

---

## Key Implementation Notes

1. **Sender Signature Requirement**: Every sender email must have a registered, confirmed signature; unverified addresses cause HTTP 422 errors.

2. **Recipient Limits**: Maximum 50 total recipients per message (To + Cc + Bcc combined).

3. **Character Encoding**: Subject length uses UTF-16 code point counting; points 0000-FFFF = 1 character, above FFFF = 2 characters.

4. **Default Message Stream**: If no `MessageStream` is specified, emails route to the default transactional stream.

5. **Background Job Recommendation**: Process large attachments asynchronously to avoid blocking user-facing requests.

6. **From Address Limit**: Maximum 255 characters.

---

## Error Handling

Missing or incorrect authentication headers return HTTP 401. Invalid sender signatures generate HTTP 422 responses. Consult the [API error codes documentation](../../api-reference/overview/README.md) for comprehensive error reference.

## Related Documentation

- [Send a Single Email](send-single-email.md)
- [Batch Emails](batch-emails.md)
- [Templates API](../../api-reference/templates-api/README.md)
