# Postmark Delivery Webhook Documentation

## Overview

The Delivery webhook notifies your application when emails are "successfully delivered to the receiving server." This differs from inbox placement—Postmark considers delivery complete when "the destination email server returns an OK response after delivery is attempted."

## Key Characteristics

- **Real-time notifications**: Webhooks push confirmation data instantly versus polling via the Messages API
- **Data format**: JSON payloads delivered via HTTP POST
- **Timestamp format**: ISO 8601 standard for `DeliveredAt` field
- **Scope**: Tracks individual recipient deliveries across multiple recipients per message

## Configuration

### Via Web Interface

1. Access your Server settings
2. Navigate to Message Stream
3. Click the Webhooks tab
4. Click "Add webhook"
5. Enter your URL
6. Enable the Delivery checkbox

### Via API

Use the Webhooks API to create or modify delivery webhooks:

```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://example.com/delivery-webhook",
    "Triggers": {
      "Delivery": {"Enabled": true}
    }
  }'
```

## Webhook Payload Structure

```json
{
  "RecordType": "Delivery",
  "MessageStream": "outbound",
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "Recipient": "john@example.com",
  "DeliveredAt": "2019-11-05T16:33:54.9070259Z",
  "Details": "Test delivery webhook details",
  "Tag": "welcome-email",
  "ServerID": 23,
  "Metadata": {
    "a_key": "a_value",
    "b_key": "b_value"
  }
}
```

### Key Fields

| Field | Type | Description |
|-------|------|-------------|
| RecordType | string | Always "Delivery" |
| Recipient | string | Target email address |
| Tag | string | Custom identifier from send request |
| DeliveredAt | string | Delivery timestamp (ISO 8601) |
| Details | string | Response from receiving mail server |
| Metadata | object | Custom key-value pairs included with message |
| MessageID | string | Unique message identifier |
| ServerID | integer | Server identifier |
| MessageStream | string | Message stream identifier |

## Testing

Use this curl command to simulate webhook requests:

```bash
curl <your-url> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
  "RecordType": "Delivery",
  "MessageStream": "outbound",
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "Recipient": "john@example.com",
  "DeliveredAt": "2014-08-01T13:28:10.2735393-04:00",
  "Details": "Test delivery webhook details",
  "Tag": "welcome-email",
  "ServerID": 23,
  "Metadata": {"a_key": "a_value"}
}'
```

## Use Cases

- Trigger downstream processes upon delivery confirmation
- Generate application-specific delivery analytics
- Build user interfaces displaying email status
- Measure delivery speed metrics
- Update database records with delivery timestamps

## Important Notes

- Delivery confirmation means the receiving server accepted the email
- It does not guarantee inbox placement (email may go to spam)
- Use in combination with open/click tracking for engagement insights

## Related Documentation

- [Webhooks Overview](../overview/README.md)
- [Email API](../../api-reference/email-api/README.md)
- [Messages API](../../api-reference/messages-api/README.md)
