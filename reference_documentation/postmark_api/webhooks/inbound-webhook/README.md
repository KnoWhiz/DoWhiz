# Inbound Webhook Documentation

## Overview

The inbound webhook is a core component of Postmark's email processing system. It enables applications to receive and process incoming emails as JSON data, transforming email into a usable data source.

## How It Works

Postmark accepts emails sent to your server's unique inbound address or forwarding domain, parses them, and delivers the parsed content as JSON via HTTP POST to your configured webhook URL. This allows developers to build applications where "email becomes an incoming data source."

## Configuration

### Via Web Dashboard

1. Login to Postmark
2. Select your Server
3. Navigate to Inbound Message Stream
4. Click **Settings**
5. Enter your webhook URL in the **Webhook** field

### Via API

Use the Servers API to set `InboundHookUrl` when creating or editing a server.

## Webhook Payload Structure

### Key Fields

| Field | Type | Description |
|-------|------|-------------|
| From | string | Sender email address |
| FromName | string | Sender display name |
| FromFull | object | Structured sender info (Email, Name, MailboxHash) |
| To | string | Recipient email address |
| ToFull | array | Structured recipient info |
| Cc | string | CC recipients |
| CcFull | array | Structured CC info |
| Bcc | string | BCC recipients |
| BccFull | array | Structured BCC info |
| Subject | string | Email subject |
| MessageID | string | Unique message identifier |
| MessageStream | string | Always "inbound" |
| Date | string | Original message timestamp |
| MailboxHash | string | Hash from plus addressing |
| TextBody | string | Plain text content |
| HtmlBody | string | HTML content |
| StrippedTextReply | string | Extracted reply text |
| Tag | string | Custom tag |
| Headers | array | Email headers including SpamAssassin |
| Attachments | array | Base64-encoded attachments |

### Example Payload

```json
{
  "From": "user@domain.com",
  "FromName": "User Name",
  "FromFull": {
    "Email": "user@domain.com",
    "Name": "User Name",
    "MailboxHash": ""
  },
  "To": "hash@inbound.postmarkapp.com",
  "ToFull": [
    {
      "Email": "hash@inbound.postmarkapp.com",
      "Name": "",
      "MailboxHash": "custom-hash"
    }
  ],
  "Cc": "",
  "CcFull": [],
  "Bcc": "",
  "BccFull": [],
  "OriginalRecipient": "hash@inbound.postmarkapp.com",
  "ReplyTo": "reply@domain.com",
  "Subject": "Email Subject",
  "MessageID": "unique-id-uuid",
  "MessageStream": "inbound",
  "Date": "Thu, 5 Apr 2012 16:59:01 +0200",
  "MailboxHash": "custom-hash",
  "TextBody": "Plain text content",
  "HtmlBody": "<p>HTML content</p>",
  "StrippedTextReply": "Parsed reply text",
  "Tag": "",
  "Headers": [
    {"Name": "X-Spam-Status", "Value": "No"},
    {"Name": "X-Spam-Score", "Value": "1.2"}
  ],
  "Attachments": [
    {
      "Name": "document.pdf",
      "Content": "base64-encoded-content",
      "ContentType": "application/pdf",
      "ContentLength": 8192,
      "ContentID": ""
    }
  ]
}
```

## SpamAssassin Headers

Postmark runs incoming mail through SpamAssassin and includes these headers:

| Header | Description |
|--------|-------------|
| X-Spam-Checker-Version | SpamAssassin version |
| X-Spam-Status | "Yes" or "No" |
| X-Spam-Score | Numeric score |
| X-Spam-Tests | Tests triggered (e.g., DKIM_VALID, SPF_PASS) |

## Error Handling & Retries

If your webhook doesn't return HTTP 200:

| Attempt | Delay |
|---------|-------|
| 1 | Immediate |
| 2 | 1 minute |
| 3 | 5 minutes |
| 4-6 | 10 minutes |
| 7 | 15 minutes |
| 8 | 30 minutes |
| 9 | 1 hour |
| 10 | 2 hours |
| 11 | 6 hours |

- **403 responses** halt retries immediately
- Failed messages appear as "Inbound Error" in the Postmark dashboard
- Failed messages can be manually retried via the API

## Testing Your Webhook

Use curl to test locally:

```bash
curl <your-url> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "From": "test@example.com",
    "FromName": "Test User",
    "To": "hash@inbound.postmarkapp.com",
    "Subject": "Test Email",
    "TextBody": "This is a test",
    "MessageID": "test-123",
    "MessageStream": "inbound"
  }'
```

## Use Cases

Applications can leverage inbound webhooks to:

- Build comment-by-email systems
- Process support tickets from email
- Enable email-driven data entry
- Create automated workflows triggered by incoming mail
- Extract structured data from email content

## Related Documentation

- [Processing Email](../../user-guide/processing-email/README.md)
- [Parse an Email](../../user-guide/processing-email/parse-an-email.md)
- [Webhooks Overview](../overview/README.md)
