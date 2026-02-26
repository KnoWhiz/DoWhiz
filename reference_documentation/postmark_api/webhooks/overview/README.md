# Webhooks Overview - Postmark Developer Documentation

## What is a Webhook?

Postmark webhooks function as automated HTTP POST requests sent to your application's API when specific events occur. Rather than continuously querying Postmark's API to check for updates, webhooks enable immediate notification of event occurrences.

> "Postmark is able to immediately notify you when an event occurs and your application doesn't have to perform complicated polling"

**Note:** All webhook datetime values follow ISO 8601 format.

## Available Webhook Types

| Webhook | Description |
|---------|-------------|
| [Inbound Webhook](../inbound-webhook/README.md) | Incoming email received |
| [Bounce Webhook](../bounce-webhook/README.md) | Email delivery failed |
| [Spam Complaint Webhook](../spam-complaint-webhook/README.md) | Recipient marked as spam |
| [Open Tracking Webhook](../open-tracking-webhook/README.md) | Email opened |
| [Click Webhook](../click-webhook/README.md) | Link clicked |
| [Delivery Webhook](../delivery-webhook/README.md) | Email delivered successfully |
| [Subscription Change Webhook](../subscription-change-webhook/README.md) | Subscription status changed |

## Securing Your Webhook

### Firewall IP Configuration

Postmark maintains specific IP ranges for webhook requests. Organizations using firewalls should consult Postmark's support documentation for the current IP allowlist.

**Note:** Origin IP addresses may vary across retry attempts.

### Basic HTTP Authentication

Protect your webhook URL using standard HTTP authentication credentials:

```
https://<username>:<password>@example.com/webhook
```

### Custom Headers

Configure custom headers when creating webhooks via the API:

```json
{
  "HttpHeaders": [
    {
      "Name": "Authorization",
      "Value": "Bearer your-token"
    }
  ]
}
```

**Security Recommendation:** Always use HTTPS encryption for webhook endpoints to safeguard data transmission.

## Testing Webhooks

### Using curl

The curl utility replicates Postmark's HTTP requests to your API, allowing local testing before production deployment.

```bash
curl https://your-webhook-url.com/webhook \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"RecordType": "Delivery", "MessageID": "test-123"}'
```

### Using RequestBin

RequestBin provides temporary URLs for capturing and inspecting incoming HTTP requests, including headers, JSON payloads, and metadata—useful for development without public infrastructure.

## Retry Behavior

Postmark retries failed webhooks based on response status. A 403 response halts retry attempts; other non-200 responses trigger reattempts.

### Bounce and Inbound Webhooks

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

### Click, Open, Delivery, and Subscription Change Webhooks

| Attempt | Delay |
|---------|-------|
| 1 | Immediate |
| 2 | 1 minute |
| 3 | 5 minutes |
| 4 | 15 minutes |

## Configuring Webhooks

### Via Dashboard

1. Log into your Postmark account
2. Select your Server and Message Stream
3. Navigate to the **Webhooks** tab
4. Click **Add webhook**
5. Enter your webhook URL
6. Select the event types to enable
7. Save your configuration

### Via API

Use the [Webhooks API](../../api-reference/webhooks-api/README.md) to programmatically create, edit, and delete webhooks.

```bash
curl "https://api.postmarkapp.com/webhooks" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Url": "https://example.com/webhook",
    "Triggers": {
      "Delivery": {"Enabled": true},
      "Bounce": {"Enabled": true}
    }
  }'
```

## Best Practices

1. **Use HTTPS** - Always encrypt webhook traffic
2. **Authenticate** - Use HTTP auth or custom headers
3. **Return quickly** - Process webhooks asynchronously, return 200 immediately
4. **Handle retries** - Implement idempotency to handle duplicate deliveries
5. **Log everything** - Keep records of webhook payloads for debugging
6. **Test thoroughly** - Use curl or RequestBin before production

## Related Documentation

- [Webhooks API](../../api-reference/webhooks-api/README.md)
- [Server API](../../api-reference/server-api/README.md)
