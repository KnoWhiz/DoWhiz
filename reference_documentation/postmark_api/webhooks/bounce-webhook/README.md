# Bounce Webhook Documentation

## Overview

Postmark's bounce webhook mechanism enables applications to receive real-time notifications about email delivery failures. Rather than polling the Bounce API, webhooks allow Postmark to push bounce data to your application in JSON format as bounces are processed.

## What is a Bounce Webhook?

A bounce webhook is a push-based notification system that delivers information about emails that couldn't be delivered. It covers Hard Bounces, Soft Bounces, and Undeliverable classifications resulting from outgoing messages.

Key capabilities include:
- Immediate notification of bounce events
- JSON-formatted payload delivery
- Support for testing via fake bounces using black hole domains
- Timestamp data in ISO 8601 format

## Configuration

### Setting the Webhook URL

**Via Postmark Dashboard:**
1. Log into your Postmark account
2. Select your Server and Message Stream
3. Navigate to the Webhooks tab
4. Click "Add webhook"
5. Enter your webhook URL
6. Enable the Bounce checkbox

**Via API:**
Use the Webhooks API to either create a new webhook or modify an existing one by setting the `Bounce` field.

## Webhook Payload Structure

### Key Fields

| Field | Type | Description |
|-------|------|-------------|
| RecordType | string | Always "Bounce" |
| ID | integer | Identifier (int64) usable with Bounce API requests |
| Type | string | Classification of bounce (HardBounce, SoftBounce, etc.) |
| TypeCode | integer | Numeric code representing bounce type |
| Name | string | Human-readable bounce type name |
| Email | string | The address that bounced |
| From | string | Original message sender |
| BouncedAt | string | ISO 8601 timestamp of bounce occurrence |
| Inactive | boolean | Whether address was deactivated |
| CanActivate | boolean | Whether address can be reactivated |
| Metadata | object | Custom key-value pairs from original email |
| Content | string | Full bounce message (optional, disabled by default) |
| MessageStream | string | Stream identifier (e.g., "outbound") |
| Subject | string | Original message subject line |
| MessageID | string | Unique message identifier |
| Tag | string | Tag from original message |
| ServerID | integer | Server identifier |
| Description | string | Bounce description |
| Details | string | Additional bounce details |
| DumpAvailable | boolean | Whether raw bounce data is available |

### Example JSON Payload

```json
{
  "RecordType": "Bounce",
  "MessageStream": "outbound",
  "ID": 4323372036854775807,
  "Type": "HardBounce",
  "TypeCode": 1,
  "Name": "Hard bounce",
  "Tag": "Test",
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "Metadata": {
    "a_key": "a_value",
    "b_key": "b_value"
  },
  "ServerID": 23,
  "Description": "The server was unable to deliver your message (ex: unknown user, mailbox not found).",
  "Details": "Test bounce details",
  "Email": "john@example.com",
  "From": "sender@example.com",
  "BouncedAt": "2019-11-05T16:33:54.9070259Z",
  "DumpAvailable": true,
  "Inactive": true,
  "CanActivate": true,
  "Subject": "Test subject",
  "Content": "<Full dump of bounce>"
}
```

## Bounce Types

| Type | TypeCode | Description |
|------|----------|-------------|
| HardBounce | 1 | Permanent delivery failure |
| Transient | 2 | Temporary delivery issue |
| Unsubscribe | 16 | Unsubscribe request |
| Subscribe | 32 | Subscription request |
| AutoResponder | 64 | Auto-reply message |
| AddressChange | 128 | Address change request |
| DnsError | 256 | DNS lookup failure |
| SpamNotification | 512 | Spam classification |
| SoftBounce | 4096 | Temporary failure |
| SpamComplaint | 100001 | Marked as spam |
| BadEmailAddress | 100000 | Invalid email format |
| DMARCPolicy | 100009 | DMARC rejection |
| TemplateRenderingFailed | 100010 | Template error |

## Testing with cURL

Test your webhook endpoint locally using this curl command:

```bash
curl <your-url> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
  "RecordType": "Bounce",
  "ID": 4323372036854775807,
  "Type": "HardBounce",
  "TypeCode": 1,
  "Name": "Hard bounce",
  "Tag": "Test",
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "ServerID": 23,
  "Description": "The server was unable to deliver your message.",
  "Details": "Test bounce details",
  "Email": "john@example.com",
  "From": "sender@example.com",
  "BouncedAt": "2014-08-01T13:28:10.2735393-04:00",
  "DumpAvailable": true,
  "Inactive": true,
  "CanActivate": true,
  "Subject": "Test subject"
}'
```

## Use Cases

Bounce webhook data enables several practical implementations:

- **User Correction**: Help users update incorrect signup email addresses immediately
- **Statistics Generation**: Build application-specific bounce analytics
- **Address Reactivation**: Use the Bounce API to reactivate addresses when appropriate
- **Team Alerts**: Notify your team when specific emails fail delivery
- **Automated Handling**: Integrate with systems like Zapier for bounce notifications

## Additional Tools

- **Rebound**: A JavaScript snippet that checks for hard bounces and prompts customers to update their email addresses
- **Slack Integration**: Install the official Postmark Bot in Slack for direct bounce notifications
- **Zapier**: Automate bounce notifications to message senders without custom development

## Related Documentation

- [Bounce API](../../api-reference/bounce-api/README.md)
- [Webhooks Overview](../overview/README.md)
- [Suppressions API](../../api-reference/suppressions-api/README.md)
