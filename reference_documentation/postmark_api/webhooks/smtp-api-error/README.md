# SMTP API Error Documentation

## Overview

SMTP API Error represents an **internal rejection mechanism** that Postmark activates when attempting to deliver messages via SMTP to recipients on suppression lists. The system identifies addresses previously marked as inactive due to bounces, spam complaints, unsubscribes, or manual suppressions.

**Key Scope:** This feature applies exclusively to SMTP-based message transmission and must be configured at the server level. REST API submissions receive rejection details directly in API responses.

## Why SMTP API Errors Exist

When sending via SMTP, you don't receive immediate feedback about recipient status like you do with the REST API. SMTP accepts all messages at submission time, even if the recipient is on the suppression list. The SMTP API Error feature provides visibility into these rejections through the bounce system.

## Configuration

### Enabling SMTP API Errors

To activate this feature, configure it individually per server through the API (UI configuration unavailable).

**Requirements:**
- Account API authentication token
- PUT request to the servers endpoint
- Server ID in the request URL

**Example Request:**
```bash
curl "https://api.postmarkapp.com/servers/SERVERID" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: ACCOUNTAPITOKEN" \
  -d '{
    "EnableSmtpApiErrorHooks": true
  }'
```

## How It Works

1. You send an email via SMTP to a suppressed address
2. Postmark accepts the message (SMTP can't reject synchronously)
3. Postmark detects the recipient is on the suppression list
4. An SMTP API Error is generated
5. The error appears in bounce webhooks and the dashboard

## Error Response Data

When transmission to a suppressed recipient occurs, Postmark generates an error notification visible in:

- The Postmark dashboard interface
- Bounce webhooks (if configured)
- Bounce API queries

The rejection displays the suppression reason (e.g., HardBounce, SpamComplaint, ManualSuppression).

## Webhook Payload

SMTP API Errors appear as bounce events with TypeCode for SMTP API errors:

```json
{
  "RecordType": "Bounce",
  "Type": "InternalError",
  "TypeCode": 4003,
  "Name": "SMTP API Error",
  "MessageID": "message-id",
  "Description": "Recipient is on the suppression list",
  "Email": "suppressed@example.com",
  "From": "sender@example.com",
  "BouncedAt": "2024-01-15T10:30:00Z"
}
```

## Testing Procedures

Implementation verification follows this sequence:

1. **Suppress a recipient** using the Suppressions API endpoint:
   ```bash
   curl "https://api.postmarkapp.com/message-streams/outbound/suppressions" \
     -X POST \
     -H "Accept: application/json" \
     -H "Content-Type: application/json" \
     -H "X-Postmark-Server-Token: server token" \
     -d '{"Suppressions": [{"EmailAddress": "test@example.com"}]}'
   ```

2. **Transmit a message** to the suppressed address via your configured SMTP connection

3. **Observe the rejection** in Postmark's interface or through your bounce webhook

## Practical Applications

This feature enables:

- **Synchronization:** Maintain alignment between external recipient lists and Postmark suppression records
- **Awareness:** Receive notifications when contacting inactive addresses via SMTP
- **Recovery:** Identify suppressed recipients and reactivate when appropriate
- **Monitoring:** Track SMTP delivery attempts to suppressed addresses

## Reactivating Recipients

If a recipient's status warrants removal from suppression, use the Suppressions API delete endpoint:

```bash
curl "https://api.postmarkapp.com/message-streams/outbound/suppressions/delete" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{"Suppressions": [{"EmailAddress": "test@example.com"}]}'
```

## SMTP vs REST API Comparison

| Aspect | SMTP | REST API |
|--------|------|----------|
| Immediate rejection feedback | No (requires webhook) | Yes (in response) |
| Suppression check timing | Asynchronous | Synchronous |
| Error delivery method | Via bounce webhook | In API response |
| Configuration required | Yes (EnableSmtpApiErrorHooks) | No (built-in) |

## Related Documentation

- [Sending Email with SMTP](../../user-guide/sending-email-smtp/README.md)
- [Bounce Webhook](../bounce-webhook/README.md)
- [Suppressions API](../../api-reference/suppressions-api/README.md)
- [Server API](../../api-reference/server-api/README.md)
