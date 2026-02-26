# Sandbox Mode Documentation

## Overview

Sandbox mode provides a secure testing environment within Postmark, allowing developers to experiment with various features without affecting live email delivery to actual recipients.

## Key Purpose

Sandbox mode enables you to "safely test out different parts of Postmark without accidentally sending email to real recipients." It supports:

- Integration with CI processes
- API experimentation
- Webhook testing
- Development and staging environments

## Important Usage Considerations

### Volume Counting

> **Critical Note**: Test messages processed through Sandbox Servers still consume your monthly sending quota. While these emails "aren't delivered to real inboxes," they "do count towards your monthly sending volume" since Postmark processes them through its full infrastructure.

## Creating a Sandbox Server

When creating a server, set the `DeliveryType` to "Sandbox":

```bash
curl "https://api.postmarkapp.com/servers" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "Name": "Test Server",
    "DeliveryType": "Sandbox"
  }'
```

**Important:** The delivery type cannot be changed after server creation.

## Available Features

### Server Sandbox Mode

Set up isolated test environments at the server level:
- All emails sent through sandbox servers are processed but not delivered
- Full API functionality remains available
- Statistics and activity are tracked normally
- Webhooks fire as expected

### Generate Fake Bounces

Create simulated bounce responses for testing webhook and bounce-handling functionality:
- Test your bounce processing logic
- Verify webhook endpoints handle bounces correctly
- Simulate various bounce types (hard, soft, etc.)

## Testing Scenarios

### API Integration Testing

```bash
# Send a test email through sandbox server
curl "https://api.postmarkapp.com/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: sandbox_server_token" \
  -d '{
    "From": "sender@example.com",
    "To": "test@example.com",
    "Subject": "Test Email",
    "TextBody": "This is a test."
  }'
```

### Webhook Testing

1. Configure webhook URLs on your sandbox server
2. Send test emails
3. Verify your webhook endpoints receive the expected payloads

### Using Test Token

For quick validation without delivery, use `POSTMARK_API_TEST` as your server token:

```bash
curl "https://api.postmarkapp.com/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: POSTMARK_API_TEST" \
  -d '{
    "From": "sender@example.com",
    "To": "test@example.com",
    "Subject": "Test Email",
    "TextBody": "This validates the request format."
  }'
```

This validates your request format without processing or counting against your quota.

## Sandbox vs. Live Servers

| Feature | Sandbox | Live |
|---------|---------|------|
| Email delivery | No | Yes |
| API functionality | Full | Full |
| Counts toward quota | Yes | Yes |
| Statistics tracking | Yes | Yes |
| Webhook triggers | Yes | Yes |
| Can be changed after creation | No | No |

## Best Practices

1. **Use sandbox for development** - Prevent accidental emails to real users
2. **Test webhooks thoroughly** - Verify all event handlers before going live
3. **Monitor quota usage** - Remember sandbox emails count toward your limits
4. **Use POSTMARK_API_TEST for validation** - Quick format checks without quota impact
5. **Create separate sandbox servers** - One for each development environment or team

## Related Documentation

- [Managing Servers](../managing-account/managing-servers.md)
- [Servers API](../../api-reference/servers-api/README.md)
- [Webhooks Overview](../../webhooks/overview/README.md)
