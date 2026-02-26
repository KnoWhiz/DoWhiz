# Servers API Documentation

## Overview

The Servers API enables management of servers for a Postmark account. All requests require account-level authentication using the `X-Postmark-Account-Token` header.

---

## Endpoints

### 1. Get a Server

**Endpoint:** `GET /servers/{serverid}`

Retrieves detailed information about a specific server.

#### Request Headers
- `Accept: application/json` (required)
- `X-Postmark-Account-Token` (required)

#### Example Request
```bash
curl "https://api.postmarkapp.com/servers/{serverid}" \
  -X GET \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| ID | integer | Server identifier |
| Name | string | Server name |
| ApiTokens | array | Associated API tokens |
| Color | string | Visual identifier (Purple, Blue, Turquoise, Green, Red, Yellow, Grey, Orange) |
| SmtpApiActivated | boolean | SMTP enablement status |
| RawEmailEnabled | boolean | Raw email inclusion in webhooks |
| DeliveryType | string | Environment type (Live or Sandbox) |
| ServerLink | string | Dashboard URL |
| InboundAddress | string | Email reception address |
| InboundHookUrl | string | Inbound event webhook URL |
| BounceHookUrl | string | Bounce notification webhook URL |
| OpenHookUrl | string | Open tracking webhook URL |
| DeliveryHookUrl | string | Delivery confirmation webhook URL |
| PostFirstOpenOnly | boolean | Track first open only |
| InboundDomain | string | MX record domain |
| InboundHash | string | Inbound address hash |
| InboundSpamThreshold | integer | Maximum spam score tolerance |
| TrackOpens | boolean | Enable open tracking for all emails |
| TrackLinks | string | Link tracking mode (None, HtmlAndText, HtmlOnly, TextOnly) |
| IncludeBounceContentInHook | boolean | Include bounce email content |
| ClickHookUrl | string | Click tracking webhook URL |
| EnableSmtpApiErrorHooks | boolean | Include SMTP API errors in bounce webhooks |

---

### 2. Create a Server

**Endpoint:** `POST /servers`

Establishes a new server for your account.

#### Request Headers
- `Content-Type: application/json` (required)
- `Accept: application/json` (required)
- `X-Postmark-Account-Token` (required)

#### Request Body Parameters

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| Name | string | Yes | Server identifier |
| Color | string | No | Visual categorization |
| SmtpApiActivated | boolean | No | Enable SMTP protocol |
| RawEmailEnabled | boolean | No | Include raw content in webhooks |
| DeliveryType | string | No | Live (default) or Sandbox |
| InboundHookUrl | string | No | Inbound event endpoint |
| BounceHookUrl | string | No | **Deprecated** - use Bounce Webhook API |
| OpenHookUrl | string | No | **Deprecated** - use Open Tracking API |
| DeliveryHookUrl | string | No | **Deprecated** - use Delivery Webhook API |
| PostFirstOpenOnly | boolean | No | Trigger webhook only on initial open |
| InboundDomain | string | No | MX record domain |
| InboundSpamThreshold | integer | No | Spam score limit |
| TrackOpens | boolean | No | Enable universal open tracking |
| TrackLinks | string | No | Link tracking configuration |
| IncludeBounceContentInHook | boolean | No | Add bounce email body to webhooks |
| ClickHookUrl | string | No | **Deprecated** - use Click Webhook API |
| EnableSmtpApiErrorHooks | boolean | No | Include SMTP errors in bounce webhooks |

#### Example Request
```bash
curl "https://api.postmarkapp.com/servers" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "Name": "Production",
    "Color": "Purple"
  }'
```

---

### 3. Edit a Server

**Endpoint:** `PUT /servers/{serverid}`

Modifies existing server configuration.

#### Example Request
```bash
curl "https://api.postmarkapp.com/servers/{serverid}" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "Name": "Production 2",
    "Color": "Blue"
  }'
```

---

### 4. List Servers

**Endpoint:** `GET /servers`

Retrieves all servers for your account with pagination support.

#### Query Parameters

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| count | integer | Yes | Number of servers per response |
| offset | integer | Yes | Number of servers to skip |
| name | string | No | Filter by server name (supports partial matches) |

#### Example Request
```bash
curl "https://api.postmarkapp.com/servers?count=50&offset=0" \
  -X GET \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

#### Response
```json
{
  "TotalCount": 2,
  "Servers": [
    {
      "ID": 1,
      "Name": "Production01",
      "ApiTokens": ["server token"],
      "Color": "red",
      "SmtpApiActivated": true,
      "DeliveryType": "Live"
    }
  ]
}
```

---

### 5. Delete a Server

**Endpoint:** `DELETE /servers/{serverid}`

Removes a server from your account.

**Note:** This feature requires special enablement—contact support to request access.

#### Example Request
```bash
curl "https://api.postmarkapp.com/servers/{serverid}" \
  -X DELETE \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

#### Response
```json
{
  "ErrorCode": 0,
  "Message": "Server Production Server 2 removed."
}
```

---

## Color Options

Servers support eight visual colors for quick identification:
- Purple
- Blue
- Turquoise
- Green
- Red
- Yellow
- Grey
- Orange

## Delivery Types

| Type | Description |
|------|-------------|
| Live | Production environment (default) |
| Sandbox | Testing environment |

The delivery type **cannot be modified** after server creation.

## Deprecation Notice

Several webhook URL parameters are deprecated in favor of dedicated webhook management APIs:
- BounceHookUrl → Use Bounce Webhook API
- OpenHookUrl → Use Open Tracking Webhook API
- DeliveryHookUrl → Use Delivery Webhook API
- ClickHookUrl → Use Click Webhook API

## Related Documentation

- [Server API](../server-api/README.md) - Single server configuration
- [Webhooks API](../webhooks-api/README.md) - Configure webhooks
- [Message Streams API](../message-streams-api/README.md) - Manage message streams
