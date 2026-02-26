# Postmark Spam Complaint Webhook Documentation

## Overview

A spam complaint occurs when an email recipient marks a message as spam in clients like Yahoo, Hotmail, or AOL. Once recorded, Postmark deactivates the recipient address permanently to maintain sending reputation.

## What Constitutes a Spam Complaint

When users click "This is Spam" or "Mark as Spam" in their email clients, Postmark documents this as a complaint. The system treats these reports seriously as indicators of sending abuse or poor practices. As noted in the documentation: "spam complaints are a clear metric to determine abuse and poor sending practices."

**Important:** Reactivation of flagged addresses requires direct contact with Postmark support.

## Configuration

### Via Web Dashboard

1. Navigate to your selected Server → Stream → Webhooks
2. Select "Add webhook"
3. Enter your endpoint URL
4. Check the spam complaint option
5. Optionally enable "Include content"

### Via API

Use the Webhooks API to create or modify webhook configurations:

```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://example.com/spam-webhook",
    "Triggers": {
      "SpamComplaint": {
        "Enabled": true,
        "IncludeContent": false
      }
    }
  }'
```

## Webhook Payload Structure

### Key Fields

| Field | Type | Description |
|-------|------|-------------|
| RecordType | string | Always "SpamComplaint" |
| ID | integer | Complaint identifier |
| Type | string | Always "SpamComplaint" |
| TypeCode | integer | Always 512 |
| Name | string | "Spam complaint" |
| Email | string | Recipient's address that marked as spam |
| From | string | Sender address |
| Tag | string | Delivery tag from original message |
| BouncedAt | string | ISO 8601 timestamp of complaint |
| Subject | string | Email subject line |
| Metadata | object | Custom key-value pairs from original email |
| MessageID | string | Unique message identifier |
| ServerID | integer | Server identifier |
| MessageStream | string | Message stream identifier |
| DumpAvailable | boolean | Whether abuse report content exists |
| Inactive | boolean | Account status after complaint |
| CanActivate | boolean | Whether address can be reactivated |

### Example Payload

```json
{
  "RecordType": "SpamComplaint",
  "MessageStream": "outbound",
  "ID": 42,
  "Type": "SpamComplaint",
  "TypeCode": 512,
  "Name": "Spam complaint",
  "Tag": "Test",
  "MessageID": "00000000-0000-0000-0000-000000000000",
  "ServerID": 1234,
  "Email": "john@example.com",
  "From": "sender@example.com",
  "BouncedAt": "2019-11-05T16:33:54.9070259Z",
  "Subject": "Test subject",
  "DumpAvailable": true,
  "Inactive": true,
  "CanActivate": false,
  "Metadata": {
    "a_key": "a_value",
    "b_key": "b_value"
  }
}
```

## Testing with cURL

```bash
curl <your-url> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
  "RecordType": "SpamComplaint",
  "ID": 42,
  "Type": "SpamComplaint",
  "TypeCode": 512,
  "Name": "Spam complaint",
  "Email": "john@example.com",
  "From": "sender@example.com",
  "BouncedAt": "2018-02-20T12:54:23.3396434-05:00",
  "Subject": "Test subject",
  "DumpAvailable": true,
  "Inactive": true,
  "CanActivate": false
}'
```

## Use Cases

- Receive immediate notification of spam complaints to trigger downstream actions
- Generate application-specific complaint statistics
- Display complaint history to users through your application interface
- Update user records to prevent future sends to complainers
- Alert teams about potential deliverability issues

## Impact on Sending

When a spam complaint is recorded:

1. The recipient address is automatically deactivated
2. Future sends to that address will fail
3. The complaint affects your overall sender reputation
4. Statistics are updated in your dashboard

## Best Practices

1. **Monitor complaint rates** - High rates indicate potential issues
2. **Review content** - Check if email content may be triggering complaints
3. **Verify opt-in** - Ensure recipients consented to receive emails
4. **Honor unsubscribes** - Provide easy unsubscribe options
5. **Segment audiences** - Send relevant content to engaged recipients

## Related Documentation

- [Webhooks Overview](../overview/README.md)
- [Bounce API](../../api-reference/bounce-api/README.md)
- [Suppressions API](../../api-reference/suppressions-api/README.md)
