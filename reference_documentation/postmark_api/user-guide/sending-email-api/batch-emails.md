# Postmark Batch Email API

## Overview

Postmark provides a batching endpoint for developers needing to send messages at higher volumes or with time constraints. This feature allows sending up to 500 well-formatted messages in a single API call.

## Key Specifications

**Limits:**
- Maximum messages per call: 500
- Maximum payload size: up to 50 MB payload size, including attachments
- Format: JSON array containing multiple message requests

**Broadcasting Support:**
When sending Broadcast (bulk) messages via batch, use the `MessageStream` field to designate your sending stream.

## Request Format

### Endpoint

```
POST https://api.postmarkapp.com/email/batch
```

### Headers Required

- `Accept: application/json`
- `Content-Type: application/json`
- `X-Postmark-Server-Token: [your-server-token]`

### Request Body Structure

The request uses a JSON array format with multiple message objects:

```json
[
  {
    "From": "sender@example.com",
    "To": "receiver@example.com",
    "Subject": "Email subject",
    "TextBody": "Plain text content",
    "HtmlBody": "<html>HTML content</html>",
    "MessageStream": "outbound"
  }
]
```

## Example: cURL Request

```bash
curl "https://api.postmarkapp.com/email/batch" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '[{
    "From": "sender@example.com",
    "To": "receiver1@example.com",
    "Subject": "Postmark test #1",
    "TextBody": "Hello dear Postmark user.",
    "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
    "MessageStream": "outbound"
  },
  {
    "From": "sender@example.com",
    "To": "receiver2@example.com",
    "Subject": "Postmark test #2",
    "TextBody": "Hello dear Postmark user.",
    "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
    "MessageStream": "outbound"
  }]'
```

## Response Format

The API returns a matching JSON array with individual response objects for each message sent:

```json
[
  {
    "ErrorCode": 0,
    "Message": "OK",
    "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
    "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
    "To": "receiver1@example.com"
  },
  {
    "ErrorCode": 0,
    "Message": "OK",
    "MessageID": "e2ecbbfc-fe12-463d-b933-9fe22915106d",
    "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
    "To": "receiver2@example.com"
  }
]
```

### Response Fields

| Field | Description |
|-------|-------------|
| `ErrorCode` | Status code (0 = success) |
| `Message` | Status message |
| `MessageID` | Unique identifier for tracking |
| `SubmittedAt` | Timestamp of submission |
| `To` | Recipient email address |

## Error Handling

**Important:** The batch endpoint returns HTTP 200 even when individual message validation fails. Each message object includes its own error code and status.

Example with one failed message:

```json
[
  {
    "ErrorCode": 0,
    "Message": "OK",
    "MessageID": "b7bc2f4a-e38e-4336-af7d-e6c392c2f817",
    "SubmittedAt": "2010-11-26T12:01:05.1794748-05:00",
    "To": "receiver1@example.com"
  },
  {
    "ErrorCode": 406,
    "Message": "Recipient marked as inactive",
    "To": "receiver2@example.com"
  }
]
```

Refer to the complete [API error codes documentation](../../api-reference/overview/README.md) for detailed error information and troubleshooting guidance.

## Best Practices

1. **Process responses individually** - Check each message's `ErrorCode` in the response array
2. **Implement retry logic** - Retry failed messages separately
3. **Use appropriate message streams** - Set `MessageStream` for broadcast vs. transactional emails
4. **Monitor payload size** - Keep track of total size including attachments
5. **Background processing** - Execute batch sends asynchronously for better user experience
