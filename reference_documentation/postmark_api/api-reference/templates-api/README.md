# Postmark Templates API Documentation

## Overview

The Templates API allows you to manage email templates for a specific server. Each server supports up to 100 templates maximum. Contact support if you need additional capacity.

---

## API Endpoints

### 1. Send Email with Template

**Endpoint:** `POST /email/withTemplate`

Send a single email using a predefined template.

#### Required Headers
- `Accept: application/json`
- `Content-Type: application/json`
- `X-Postmark-Server-Token: [server token]`

#### Request Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| TemplateId | integer | Conditional | Template ID (required if TemplateAlias not provided) |
| TemplateAlias | string | Conditional | Template alias (required if TemplateId not provided) |
| TemplateModel | object | Yes | Data to apply to template variables |
| From | string | Yes | Sender email address (must have confirmed Sender Signature) |
| To | string | Yes | Recipient email (up to 50 comma-separated addresses) |
| Cc | string | No | CC recipients (up to 50 addresses) |
| Bcc | string | No | BCC recipients (up to 50 addresses) |
| Tag | string | No | Categorization tag (max 1000 characters) |
| ReplyTo | string | No | Reply-to address override |
| TrackOpens | boolean | No | Enable open tracking |
| TrackLinks | string | No | Link tracking: `None`, `HtmlAndText`, `HtmlOnly`, `TextOnly` |
| Headers | array | No | Custom headers list |
| Attachments | array | No | File attachments |
| Metadata | object | No | Custom key/value pairs |
| MessageStream | string | No | Message stream ID (defaults to `outbound`) |
| InlineCss | boolean | No | Inline CSS styles (defaults to true) |

#### Example Request
```bash
curl "https://api.postmarkapp.com/email/withTemplate" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "From": "sender@example.com",
    "To": "receiver@example.com",
    "TemplateId": 1234,
    "TemplateModel": {
      "user_name": "John Smith"
    }
  }'
```

#### Response
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

### 2. Send Batch with Templates

**Endpoint:** `POST /email/batchWithTemplates`

Send multiple emails using templates in a single request (up to 500 messages per call).

#### Request Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| Messages | array | Yes | List of message objects (max 500) |

Each message object follows the same parameters as single template send.

**Note:** The `/batchWithTemplates` endpoint will return a 200-level http status, even when validation for individual messages may fail. Check each message's error code in the response array.

#### Example Request
```bash
curl "https://api.postmarkapp.com/email/batchWithTemplates" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Messages": [
      {
        "From": "sender@example.com",
        "To": "receiver1@example.com",
        "TemplateId": 12345,
        "TemplateModel": {"name": "John"}
      },
      {
        "From": "sender@example.com",
        "To": "receiver2@example.com",
        "TemplateAlias": "welcome-email",
        "TemplateModel": {"name": "Jane"}
      }
    ]
  }'
```

---

### 3. Push Templates to Another Server

**Endpoint:** `PUT /templates/push`

Synchronize templates from one server to another.

#### Required Headers
- `Accept: application/json`
- `Content-Type: application/json`
- `X-Postmark-Account-Token: [account token]`

#### Request Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| SourceServerID | string | Yes | Source server ID |
| DestinationServerID | string | Yes | Destination server ID |
| PerformChanges | boolean | Yes | Execute push or dry-run only |

#### Example Request
```bash
curl "https://api.postmarkapp.com/templates/push" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "SourceServerID": 997287,
    "DestinationServerID": 997285,
    "PerformChanges": true
  }'
```

---

### 4. Get a Template

**Endpoint:** `GET /templates/{templateIdOrAlias}`

Retrieve a specific template by ID or alias.

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| TemplateId | integer | Unique template identifier |
| Name | string | Template name |
| Subject | string | Email subject template |
| HtmlBody | string | HTML content template |
| TextBody | string | Plain text content template |
| Alias | string | Optional identifier string |
| Active | boolean | Availability for sending |
| AssociatedServerId | integer | Server association |
| TemplateType | string | `Standard` or `Layout` |
| LayoutTemplate | string | Layout template alias if used |

---

### 5. Create a Template

**Endpoint:** `POST /templates`

Create a new template on the server.

#### Request Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| Name | string | Yes | Template name |
| Alias | string | No | Unique identifier (alphanumeric, dots, hyphens, underscores; starts with letter) |
| Subject | string | Conditional | Subject line (required for Standard, not for Layout) |
| HtmlBody | string | Conditional | HTML template (required if TextBody omitted) |
| TextBody | string | Conditional | Plain text template (required if HtmlBody omitted) |
| TemplateType | string | No | `Standard` or `Layout` (defaults to Standard) |
| LayoutTemplate | string | No | Layout template alias to apply |

#### Example Request
```bash
curl "https://api.postmarkapp.com/templates" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Name": "Welcome Email",
    "Alias": "welcome-v1",
    "HtmlBody": "<html><body>Hello {{name}}</body></html>",
    "TextBody": "Hello, {{name}}",
    "Subject": "Welcome, {{name}}!",
    "TemplateType": "Standard"
  }'
```

---

### 6. Edit a Template

**Endpoint:** `PUT /templates/{templateIdOrAlias}`

Update an existing template.

---

### 7. List Templates

**Endpoint:** `GET /templates`

Retrieve all templates associated with a server.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| Count | integer | Yes | Number of templates to return |
| Offset | integer | Yes | Number of templates to skip |
| TemplateType | string | No | Filter by type: `All`, `Standard`, `Layout` |
| LayoutTemplate | string | No | Filter by layout alias |

---

### 8. Delete a Template

**Endpoint:** `DELETE /templates/{templateIdOrAlias}`

Remove a template from the server.

---

### 9. Validate a Template

**Endpoint:** `POST /templates/validate`

Test template syntax and rendering without saving.

#### Request Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| Subject | string | Conditional | Subject to validate |
| HtmlBody | string | Conditional | HTML to validate |
| TextBody | string | Conditional | Text to validate |
| TestRenderModel | object | No | Sample data for rendering test |
| InlineCssForHtmlTestRender | boolean | No | Inline CSS in test render |
| TemplateType | string | No | `Standard` or `Layout` |
| LayoutTemplate | string | No | Layout template alias |

---

## Template Syntax

Templates use Postmark's Mustachio template language for variable substitution:

```
Hello {{name}},

Welcome to our service!

{{#orders}}
  - Order #{{order_id}}: {{total}}
{{/orders}}
```

## Rate Limits & Constraints

- **Maximum templates per server**: 100
- **Maximum batch size**: 500 messages per request
- **Maximum recipients per message**: 50 (To, Cc, Bcc combined)
- **Tag maximum length**: 1000 characters

## Related Documentation

- [Email API](../email-api/README.md)
- [Server API](../server-api/README.md)
