# Parsing Inbound Emails with Postmark

## Overview

When emails arrive at your Postmark inbound mailbox, they're immediately forwarded to your configured webhook URL as JSON data. This enables you to process incoming messages within your application.

## Setup Requirements

Before parsing emails, you need:

1. **Inbound Server Configuration** - Set up your inbound server and obtain your unique `InboundHash`
2. **Email Forwarding** - Route incoming emails to your hash address via domain forwarding or custom configuration
3. **Webhook URL** - Configure where Postmark sends the JSON payload

The unique hash appears in the `InboundHash` property returned from the Servers API.

## Webhook Response Handling

When Postmark POSTs to your webhook:

| Response | Behavior |
|----------|----------|
| HTTP 200 | Success - message processed |
| Non-200 | Trigger retries (up to 10 attempts) |
| HTTP 403 | Halt all retry attempts |
| Timeout | 2 minutes maximum wait time |

**Retry schedule**: 1, 5, 10 (×3), 15, 30 minutes, then 1, 2, and 6 hours

## JSON Response Structure

The webhook delivers comprehensive email data:

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
  "Cc": "cc@domain.com",
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
  "TextBody": "[Plain text content]",
  "HtmlBody": "[HTML content]",
  "StrippedTextReply": "Parsed reply text",
  "Tag": "",
  "Headers": [],
  "Attachments": []
}
```

## Key Fields

### Contact Information

**Full Contact Format** (Recommended):
```json
{
  "Email": "user@domain.com",
  "Name": "Full Name",
  "MailboxHash": "optional-hash"
}
```

Fields `FromFull`, `ToFull`, and `CcFull` provide structured contact data with email, name, and mailbox hash separated into distinct properties.

**Legacy Format** (Deprecated):
Use `From`, `To`, `Cc` for backward compatibility, though the "Full" variations are preferred.

### Custom Mailbox Hash

When recipients use addresses like `user+hash@yourdomain.com`, Postmark extracts the hash portion. This feature enables linking replies to specific application records.

### Stripped Text Reply

The `StrippedTextReply` field contains the plain text portion of email replies, simplifying reply parsing. This works when your inbound address appears in To or CC fields and includes "In-Reply-To" or "References" headers.

**Limitations**: English text only, tested on Yahoo, iCloud, Gmail, Outlook.com, iOS Mail, Apple Mail, Microsoft Outlook, and Thunderbird.

## Attachments

Attachments arrive as base64-encoded content:

```json
{
  "Attachments": [
    {
      "Name": "document.pdf",
      "Content": "[BASE64-ENCODED DATA]",
      "ContentType": "application/pdf",
      "ContentLength": 8192,
      "ContentID": "document.pdf@id123"
    }
  ]
}
```

**Constraints**: Total cumulative size cannot exceed 35 MB.

## Security Headers

### SpamAssassin Headers

Postmark scans for spam and adds these headers:

| Header | Description |
|--------|-------------|
| `X-Spam-Checker-Version` | SpamAssassin version used |
| `X-Spam-Status` | "Yes" or "No" |
| `X-Spam-Score` | Numeric score (default spam threshold: >5) |
| `X-Spam-Tests` | Test names triggered |

**Note**: Legitimate messages may receive high scores; test thoroughly before filtering.

### SPF Verification

SPF header results indicate sender domain authenticity:

| Result | Meaning |
|--------|---------|
| `pass` | IP approved by SPF record |
| `softfail` | IP not approved, but messages accepted |
| `fail` | IP not approved for sending |
| `neutral` | No SPF record exists |

Example: `"Received-SPF: Pass (sender SPF authorized)"`

## Code Examples by Language

### Rails Gem
- Griddler Postmark integration

### Ruby Gem
- Randy Schmidt's postmark-mitt library with example application

### PHP
- Postmark inbound mail helper by We Create Solutions
- Postmark Inbound PHP by Joffrey Jaffeux

### Python
- Postmark Inbound Python by Jose Padilla
- postmarker library by Dmitry Dygalo
- Django Anymail support

### .NET
- C# + MVC inbound demo
- Visual Basic reply text parser

### JavaScript
- Postmark Webhooks Lab (Meteor.js)

## BCC Handling

For privacy, BCC information appears in JSON only under specific conditions:

| Scenario | BCC Visible |
|----------|-------------|
| Inbound address is the BCC recipient | Yes |
| Inbound address is both To and BCC | Yes (appears as To) |
| BCC is a different address than inbound | No |
| Forwarding address/domain is the BCC | Yes |

## Processing Best Practices

1. Validate webhook responses return 200 status
2. Parse `StrippedTextReply` for simplified reply handling
3. Extract and decode base64 attachments
4. Check SpamAssassin headers for filtering decisions
5. Verify SPF results for sender authentication
6. Use mailbox hash extraction to link replies to specific records

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
    "MessageID": "test-123"
  }'
```

## Related Documentation

- [Inbound Domain Forwarding](inbound-domain-forwarding.md)
- [Sample Inbound Workflow](sample-inbound-workflow.md)
- [Inbound Webhook](../../webhooks/inbound-webhook/README.md)
