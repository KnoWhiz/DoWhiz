# Postmark API Overview Documentation

## Endpoint URL

The Postmark API follows REST principles with TLS encryption. All requests must be directed to:

```
https://api.postmarkapp.com
```

## Authentication

All API requests require authentication via specific HTTP headers containing API tokens. Postmark offers two token types:

### Server Token

**Header:** `X-Postmark-Server-Token`

- Requires server-level privileges
- Located in the API Tokens tab of your Postmark server
- Accessible to Account Owners, Account Admins, and Server Admins
- Used for: email sending, message searching, bounce management, statistics

### Account Token

**Header:** `X-Postmark-Account-Token`

- Requires account-level privileges
- Found in the API Tokens section of your account dashboard
- Available only to Account Owners and Account Admins
- Used for: managing servers, domains, sender signatures

Headers are case-insensitive. Invalid or missing credentials result in HTTP 401 (Unauthorized) responses.

### Test Mode

For validation without delivery, use `POSTMARK_API_TEST` as the Server Token value to verify data integrity.

## HTTP Response Codes

| Code | Status | Description |
|------|--------|-------------|
| 200 | Success | Request processed successfully |
| 401 | Unauthorized | Missing or incorrect API token |
| 404 | Not Found | Requested resource doesn't exist |
| 413 | Payload Too Large | Exceeds 10 MB (Email API) or 50 MB (Batch API) limits |
| 415 | Unsupported Media Type | Missing required request headers |
| 422 | Unprocessable Entity | Malformed JSON or invalid fields |
| 429 | Rate Limit Exceeded | Request rate exceeds acceptable thresholds |
| 500 | Internal Server Error | Server-side processing failure |
| 503 | Service Unavailable | Scheduled maintenance or outage |

## API Error Codes

Errors return HTTP 422 with JSON structure:

```json
{
  "ErrorCode": 405,
  "Message": "details"
}
```

### Common Error Codes

#### Authentication & Signatures

| Code | Description |
|------|-------------|
| 10 | Bad or missing API token |
| 400 | Sender Signature not found |
| 401 | Sender signature not confirmed |
| 504 | Sender Signature already exists |

#### Validation

| Code | Description |
|------|-------------|
| 300 | Invalid email request |
| 402 | Invalid JSON format |
| 403 | Invalid request fields (specifies field name) |
| 409 | Missing JSON headers |

#### Account Status

| Code | Description |
|------|-------------|
| 405 | Account out of credits |
| 406 | Inactive recipient (bounced or spam complaint) |
| 412 | Account pending approval |
| 413 | Account not approved for sending |

#### Template Issues

| Code | Description |
|------|-------------|
| 1101 | Template not found |
| 1105 | Template limit exceeded (100 maximum) |
| 1120 | Required template field missing |
| 1130 | Layout template has dependent templates |

#### Message Streams

| Code | Description |
|------|-------------|
| 1221 | Invalid MessageStreamType for operation |
| 1226 | Message stream ID not found |
| 1228 | Server limited to one inbound stream |
| 1229 | Cannot archive default streams |

#### Data Management

| Code | Description |
|------|-------------|
| 1301 | Invalid data removal request ID |
| 1302 | Insufficient data removal permissions |

## Request Headers

### Required Headers

All API requests must include:

```
Accept: application/json
Content-Type: application/json
```

Plus the appropriate authentication header:

```
X-Postmark-Server-Token: your_server_token
```

or

```
X-Postmark-Account-Token: your_account_token
```

## Rate Limiting

Postmark implements rate limiting to ensure fair usage. If you exceed the rate limit, you'll receive a 429 response. Best practices:

- Implement exponential backoff
- Use batch endpoints when possible
- Cache responses where appropriate

## Key Features

- RESTful architecture with predictable endpoints
- Comprehensive error reporting with specific codes
- Rate limiting to prevent abuse
- Support for batch operations (up to 500 messages)
- Webhook integration for event notifications
- Multiple authentication levels for security

## API Sections

- [Email API](../email-api/README.md) - Send single and batch emails
- [Bounce API](../bounce-api/README.md) - Manage bounces
- [Templates API](../templates-api/README.md) - Create and manage templates
- [Server API](../server-api/README.md) - Configure server settings
- [Servers API](../servers-api/README.md) - Manage multiple servers
- [Message Streams API](../message-streams-api/README.md) - Manage message streams
- [Messages API](../messages-api/README.md) - Search and retrieve messages
- [Domains API](../domains-api/README.md) - Configure domains
- [Sender Signatures API](../sender-signatures-api/README.md) - Manage sender signatures
- [Stats API](../stats-api/README.md) - Retrieve statistics
- [Webhooks API](../webhooks-api/README.md) - Configure webhooks
- [Suppressions API](../suppressions-api/README.md) - Manage suppressions
