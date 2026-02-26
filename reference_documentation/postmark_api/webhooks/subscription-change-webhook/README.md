# Subscription Change Webhook Documentation

## Overview

A subscription change is recorded when an email address is added to or removed from a Message Stream's Suppression list. This typically occurs following hard bounces, spam complaints, unsubscribes, or manual suppressions.

## What Triggers a Subscription Change

Email addresses enter the suppression list through:

| Trigger | Description |
|---------|-------------|
| Hard Bounce | Permanent delivery failure |
| Spam Complaint | Recipient marked as spam |
| Unsubscribe | Recipient opted out |
| Manual Suppression | Added via UI or API |

Addresses are removed manually through the Postmark UI or via the Suppressions API.

## Configuration

### Via Postmark Dashboard

1. Log into your Postmark account
2. Select your Server and Message Stream
3. Navigate to the **Webhooks** tab
4. Click **Add webhook**
5. Enter your webhook URL
6. Check the **Subscription Change** checkbox

### Via API

```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://example.com/subscription-webhook",
    "Triggers": {
      "SubscriptionChange": {"Enabled": true}
    }
  }'
```

## Webhook Payload Structure

```json
{
  "RecordType": "SubscriptionChange",
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "ServerID": 123456,
  "MessageStream": "outbound",
  "ChangedAt": "2020-02-01T10:53:34.416071Z",
  "Recipient": "bounced-address@example.com",
  "Origin": "Recipient",
  "SuppressSending": true,
  "SuppressionReason": "HardBounce",
  "Tag": "my-tag",
  "Metadata": {
    "example": "value",
    "example_2": "value"
  }
}
```

## Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| RecordType | string | Always "SubscriptionChange" |
| SuppressSending | boolean | `true` = suppressed, `false` = reactivated |
| MessageID | string | Null for manual suppressions or reactivations |
| SuppressionReason | string | `HardBounce`, `SpamComplaint`, or `ManualSuppression` |
| Origin | string | Source of change (`Recipient`, `Customer`, or `Admin`) |
| ChangedAt | string | ISO 8601 timestamp of the change |
| Recipient | string | Email address affected |
| Tag | string | Null when reactivating (SuppressSending = false) |
| Metadata | object | Empty object during reactivation |
| ServerID | integer | Server identifier |
| MessageStream | string | Message stream identifier |

## Suppression vs Reactivation

### Suppression Event (SuppressSending = true)

Indicates an address was added to the suppression list:

```json
{
  "RecordType": "SubscriptionChange",
  "SuppressSending": true,
  "SuppressionReason": "HardBounce",
  "Tag": "welcome-email",
  "Metadata": {"campaign": "onboarding"}
}
```

### Reactivation Event (SuppressSending = false)

Indicates an address was removed from the suppression list:

```json
{
  "RecordType": "SubscriptionChange",
  "SuppressSending": false,
  "SuppressionReason": "ManualSuppression",
  "Tag": null,
  "Metadata": {}
}
```

## Testing with cURL

```bash
curl <your-url> \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{
  "RecordType": "SubscriptionChange",
  "MessageID": "883953f4-6105-42a2-a16a-77a8eac79483",
  "ServerID": 4509041,
  "MessageStream": "outbound",
  "ChangedAt": "2020-02-01T10:53:34.416071Z",
  "Recipient": "hard-bounce@example.com",
  "Origin": "Customer",
  "SuppressSending": true,
  "SuppressionReason": "HardBounce",
  "Tag": "my-tag",
  "Metadata": {
    "example": "value"
  }
}'
```

## Use Cases

- **Replace bounce webhooks** - Get inactive address notifications in one place
- **Maintain internal suppression list** - Sync with your own database
- **Notify users** - Alert users when their email status changes
- **Compliance tracking** - Record suppression events for audit purposes
- **Re-engagement workflows** - Trigger processes when addresses are reactivated

## Related Documentation

- [Suppressions API](../../api-reference/suppressions-api/README.md)
- [Webhooks Overview](../overview/README.md)
- [Bounce Webhook](../bounce-webhook/README.md)
