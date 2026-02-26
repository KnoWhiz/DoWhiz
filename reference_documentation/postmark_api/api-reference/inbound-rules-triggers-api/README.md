# Inbound Rules Triggers API Documentation

## Overview

The Inbound Rules Triggers API enables blocking of inbound messages from specific senders. Filtering can be applied based on individual email addresses or entire domains.

## How Inbound Rules Work

You can manage your Inbound rules through the Triggers API. Email messages that are blocked by any of the inbound rules are not processed by Postmark's servers.

Inbound Rules offer control over inbound processing. You can add either an email address or a domain to a list of rules that will be consulted for each inbound message. If the sender of an inbound message matches one of the rules, the message will be blocked.

**Bypassing:** You can process a message that was once blocked by using the bypass endpoint.

**Retention:** Blocked inbound messages have a retention of 10 days before being deleted.

---

## Endpoints

### 1. List Inbound Rule Triggers

**Method:** `GET`
**Path:** `/triggers/inboundrules`

#### Required Headers
- `Accept: application/json`
- `X-Postmark-Server-Token: [server token]`

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| count | integer | Number of records to return per request |
| offset | integer | Number of records to skip |

#### Example Request
```bash
curl "https://api.postmarkapp.com/triggers/inboundrules?count=50&offset=0" \
  -X GET \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| TotalCount | integer | Matching trigger count |
| InboundRules | array | List of rule objects |

#### Example Response
```json
{
  "TotalCount": 3,
  "InboundRules": [
    {"ID": 3, "Rule": "someone@example.com"},
    {"ID": 5, "Rule": "badsender@example.com"},
    {"ID": 7, "Rule": "baddomain.com"}
  ]
}
```

---

### 2. Create Inbound Rule Trigger

**Method:** `POST`
**Path:** `/triggers/inboundrules`

#### Required Headers
- `Content-Type: application/json`
- `Accept: application/json`
- `X-Postmark-Server-Token: [server token]`

#### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| Rule | string | Yes | Email address or domain to block |

#### Example Request
```bash
curl "https://api.postmarkapp.com/triggers/inboundrules" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{"Rule": "someone@example.com"}'
```

#### Example Response
```json
{
  "ID": 15,
  "Rule": "someone@example.com"
}
```

---

### 3. Delete Inbound Rule Trigger

**Method:** `DELETE`
**Path:** `/triggers/inboundrules/{triggerid}`

#### Required Headers
- `Accept: application/json`
- `X-Postmark-Server-Token: [server token]`

#### Example Request
```bash
curl "https://api.postmarkapp.com/triggers/inboundrules/{triggerid}" \
  -X DELETE \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

#### Example Response
```json
{
  "ErrorCode": 0,
  "Message": "Rule someone@example.com removed."
}
```

---

## Key Features

- **Flexible Blocking:** Block by specific email address or entire domain
- **Server-Level Access:** Requires server authentication tokens
- **Pagination Support:** Use count and offset parameters for large rule sets
- **Unique IDs:** Each rule receives a distinct identifier for management

## Error Codes

| Code | Description |
|------|-------------|
| 800 | Trigger query exception - invalid querystring parameters |

## Use Cases

1. **Block spam senders** - Add known spam email addresses
2. **Block entire domains** - Block all emails from specific domains
3. **Temporary blocking** - Add rules that can be removed later
4. **Automated management** - Programmatically manage block lists

## Related Documentation

- [Processing Email](../../user-guide/processing-email/README.md)
- [Inbound Webhook](../../webhooks/inbound-webhook/README.md)
- [Messages API](../messages-api/README.md)
