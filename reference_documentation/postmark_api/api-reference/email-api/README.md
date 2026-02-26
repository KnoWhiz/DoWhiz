# Postmark Email API Documentation

## Overview

The Postmark Email API serves as the primary interface for sending emails through a designated server. The service accommodates batch operations with a maximum of 500 messages per API call and a 50 MB payload limit (including attachments).

## Send a Single Email

**Endpoint:** `POST /email`

### Request Headers

| Header | Required | Value |
|--------|----------|-------|
| Content-Type | Yes | application/json |
| Accept | Yes | application/json |
| X-Postmark-Server-Token | Yes | Server authentication token |

### Request Body Parameters

| Parameter | Type | Notes |
|-----------|------|-------|
| From | string | Sender address; must have registered Sender Signature. Format: "Name <email@domain>" |
| To | string | Recipient(s); comma-separated; max 50 addresses |
| Cc | string | Carbon copy recipients; comma-separated; max 50 addresses |
| Bcc | string | Blind carbon copy recipients; comma-separated; max 50 addresses |
| Subject | string | Email subject line |
| Tag | string | Categorization tag for statistics; max 1000 characters |
| HtmlBody | string | HTML message (required if TextBody absent) |
| TextBody | string | Plain text message (required if HtmlBody absent) |
| ReplyTo | string | Override reply address; defaults to Sender Signature setting |
| Headers | array | Custom header objects with Name and Value properties |
| TrackOpens | boolean | Enable open tracking |
| TrackLinks | string | Link tracking mode: None, HtmlAndText, HtmlOnly, TextOnly |
| Metadata | object | Custom key/value pairs |
| Attachments | array | File attachment objects |
| MessageStream | string | Stream ID; defaults to "outbound" |

### Example Request

```bash
curl "https://api.postmarkapp.com/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
  "From": "sender@example.com",
  "To": "receiver@example.com",
  "Subject": "Postmark test",
  "TextBody": "Hello dear Postmark user.",
  "HtmlBody": "<html><body><strong>Hello</strong> dear Postmark user.</body></html>",
  "MessageStream": "outbound"
}'
```

### Example Response

```json
{
  "To": "receiver@example.com",
  "SubmittedAt": "2014-02-17T07:25:01.4178645-05:00",
  "MessageID": "0a129aee-e1cd-480d-b08d-4f48548ff48d",
  "ErrorCode": 0,
  "Message": "OK"
}
```

---

## Send Batch Emails

**Endpoint:** `POST /email/batch`

### Request Headers

Same as single email endpoint.

### Request Body

An array of email objects using the same parameters as the single email endpoint. Each message object follows the identical structure.

### Key Constraints

- Maximum 500 messages per request
- Maximum 50 MB total payload
- Each message uses standard email object parameters

### Example Request

```bash
curl "https://api.postmarkapp.com/email/batch" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '[
  {
    "From": "sender@example.com",
    "To": "receiver1@example.com",
    "Subject": "Postmark test #1",
    "TextBody": "Hello dear Postmark user.",
    "MessageStream": "outbound"
  },
  {
    "From": "sender@example.com",
    "To": "receiver2@example.com",
    "Subject": "Postmark test #2",
    "TextBody": "Hello dear Postmark user.",
    "MessageStream": "outbound"
  }
]'
```

### Example Response

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

**Important:** The batch endpoint returns HTTP 200 even when individual message validation fails. Each message object includes its own error code and status.

---

## Attachments

Include files using these parameters:

| Parameter | Description |
|-----------|-------------|
| Name | Filename |
| Content | Base64-encoded file content |
| ContentType | MIME type (e.g., text/plain, application/pdf) |
| ContentID | Optional; for embedding in HTML (format: "cid:filename") |

### Example

```json
{
  "Attachments": [
    {
      "Name": "document.pdf",
      "Content": "base64-encoded-content",
      "ContentType": "application/pdf"
    }
  ]
}
```

---

## Metadata & Custom Headers

**Metadata:** Store custom key-value data for analytics and reference purposes.

```json
{
  "Metadata": {
    "order_id": "12345",
    "campaign": "summer-sale"
  }
}
```

**Headers:** Add custom email headers as name-value pairs for advanced email handling.

```json
{
  "Headers": [
    {
      "Name": "X-Custom-Header",
      "Value": "custom-value"
    }
  ]
}
```

---

## Response Fields

| Field | Type | Description |
|-------|------|-------------|
| To | string | Recipient address |
| MessageID | string | Unique message identifier |
| SubmittedAt | string | ISO 8601 timestamp |
| ErrorCode | integer | Status code (0 = success) |
| Message | string | Status or error description |

## Related Documentation

- [Templates API](../templates-api/README.md)
- [Bounce API](../bounce-api/README.md)
- [Messages API](../messages-api/README.md)
