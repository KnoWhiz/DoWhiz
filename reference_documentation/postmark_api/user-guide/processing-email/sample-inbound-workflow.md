# Sample Inbound Workflow Guide

## Overview

This documentation outlines a practical approach to processing incoming emails through Postmark's inbound service. The scenario focuses on handling email replies to comment notifications, enabling users to respond without logging into your application.

## Prerequisites

Before implementing this workflow, ensure you have:
- Configured an inbound server within Postmark
- Obtained your unique inbound email address
- Established your webhook URL

## Workflow Steps

### Step 1: Configure a Forwarding Address

Your inbound mailbox follows this format: `abc123@inbound.postmarkapp.com`

To maintain brand consistency, set up email forwarding from your custom domain to the Postmark address. For example, forward emails sent to `reply@yourdomain.com` to your Postmark inbound address.

**Two configuration options available:**
- Postmark's Inbound Domain Forwarding feature
- Custom Gmail/Google Apps forwarding (for those without DNS access)

This approach allows users to reply using your branded email address while Postmark captures and processes the content.

### Step 2: Identify Unique Replies

To associate replies with specific users and conversations, implement an identifier system using the `+` character in your reply-to address.

**Example header structure:**
```
From: reply@yourapp.com
Reply-to: reply+userIDthreadID@yourapp.com
To: youruser@domain.com
```

When recipients reply, Postmark extracts information after the `+` symbol and includes it in the `MailboxHash` JSON field:

```json
{
  "MailboxHash": "userIDthreadID"
}
```

**Security recommendation:** Hash sensitive identifiers using SHA1 or MD5 before including them in the mailbox hash. Maintain a lookup table connecting hashes to original identifiers for quick retrieval.

### Step 3: Process Webhook Data

Upon receiving a reply, Postmark sends parsed email contents and metadata as JSON to your webhook endpoint.

**Retry mechanism:**
- Non-200 response codes trigger automatic retries
- Maximum 10 retry attempts with intervals escalating from 1 minute to 6 hours
- Failed messages display as "Inbound Error" in your Postmark dashboard

**Implementation considerations:**
- Response handling depends on your specific use case
- Programming language and environment affect processing approach
- Postmark provides code examples for common scenarios

## Complete Workflow Sequence

```
1. User posts a comment on your site or application
         ↓
2. Notification email containing the comment distributes to subscribers
         ↓
3. Recipients click reply in their email client, sending to the forwarded address
         ↓
4. Postmark parses the email into JSON format
         ↓
5. JSON payload transmits to your configured webhook URL
         ↓
6. Application processes the data and posts the reply as a comment
```

This architecture eliminates login requirements while maintaining proper message threading and attribution.

## Example Implementation

### Sending the Original Notification

```json
{
  "From": "notifications@yourapp.com",
  "To": "subscriber@example.com",
  "ReplyTo": "reply+abc123def456@yourapp.com",
  "Subject": "New comment on your post",
  "TextBody": "John said: Great article!\\n\\nReply to this email to respond.",
  "HtmlBody": "<p><strong>John said:</strong> Great article!</p><p>Reply to this email to respond.</p>"
}
```

### Processing the Reply

```javascript
// Express.js webhook handler
app.post('/webhook/inbound', (req, res) => {
  const { MailboxHash, StrippedTextReply, From } = req.body;

  // Decode the hash to get user and thread IDs
  const { userId, threadId } = decodeHash(MailboxHash);

  // Create the reply comment
  createComment({
    threadId,
    authorEmail: From,
    content: StrippedTextReply
  });

  res.status(200).send('OK');
});
```

## Best Practices

1. **Hash sensitive data** - Never expose raw database IDs in email addresses
2. **Validate senders** - Verify the From address matches expected users
3. **Handle edge cases** - Process out-of-office replies and bounce notifications gracefully
4. **Log everything** - Maintain records of processed emails for debugging
5. **Test thoroughly** - Simulate various email clients and response formats

## Related Documentation

- [Inbound Domain Forwarding](inbound-domain-forwarding.md)
- [Parse an Email](parse-an-email.md)
- [Inbound Webhook](../../webhooks/inbound-webhook/README.md)
