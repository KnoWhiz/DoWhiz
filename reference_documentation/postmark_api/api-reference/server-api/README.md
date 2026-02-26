# Server API Documentation

## Overview

The Server API enables retrieval and modification of specific server settings using `X-Postmark-Server-Token` authentication.

## Endpoints

### Get Server Configuration

**Endpoint:** `GET /server`

**Authentication:** Required `X-Postmark-Server-Token` header

**Headers:**
- `Accept: application/json` (required)
- `X-Postmark-Server-Token` (required)

**Example Request:**
```bash
curl "https://api.postmarkapp.com/server" \
  -X GET \
  -H "Accept: application/json" \
  -H "X-Postmark-Server-Token: server token"
```

**Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| ID | integer | Server identifier |
| Name | string | Server name |
| ApiTokens | array | Associated API tokens |
| Color | string | Server color (Purple, Blue, Turquoise, Green, Red, Yellow, Grey, Orange) |
| SmtpApiActivated | boolean | SMTP enablement status |
| RawEmailEnabled | boolean | Raw email inclusion in inbound webhooks |
| DeliveryType | string | Environment type (Live or Sandbox) |
| ServerLink | string | Server overview page URL |
| InboundAddress | string | Inbound email address |
| InboundHookUrl | string | Inbound event webhook URL |
| BounceHookUrl | string | Bounce event webhook URL |
| OpenHookUrl | string | Open tracking webhook URL |
| DeliveryHookUrl | string | Delivery event webhook URL |
| PostFirstOpenOnly | boolean | Track first open only |
| InboundDomain | string | MX setup domain |
| InboundHash | string | Inbound email address hash |
| InboundSpamThreshold | integer | Maximum spam score before blocking |
| TrackOpens | boolean | Open tracking enablement |
| TrackLinks | string | Link tracking mode (None, HtmlAndText, HtmlOnly, TextOnly) |
| IncludeBounceContentInHook | boolean | Include bounce content in webhooks |
| ClickHookUrl | string | Click event webhook URL |
| EnableSmtpApiErrorHooks | boolean | SMTP API error inclusion in bounces |

**Example Response:**
```json
{
  "ID": 1,
  "Name": "Production Server",
  "ApiTokens": ["server-token-here"],
  "Color": "blue",
  "SmtpApiActivated": true,
  "RawEmailEnabled": false,
  "DeliveryType": "Live",
  "ServerLink": "https://postmarkapp.com/servers/1/streams",
  "InboundAddress": "abc123@inbound.postmarkapp.com",
  "InboundHookUrl": "https://example.com/inbound",
  "BounceHookUrl": "https://example.com/bounce",
  "OpenHookUrl": "https://example.com/open",
  "DeliveryHookUrl": "https://example.com/delivery",
  "PostFirstOpenOnly": false,
  "InboundDomain": "inbound.example.com",
  "InboundHash": "abc123",
  "InboundSpamThreshold": 5,
  "TrackOpens": true,
  "TrackLinks": "HtmlAndText",
  "IncludeBounceContentInHook": false,
  "ClickHookUrl": "https://example.com/click",
  "EnableSmtpApiErrorHooks": false
}
```

---

### Edit Server Configuration

**Endpoint:** `PUT /server`

**Authentication:** Required `X-Postmark-Server-Token` header

**Headers:**
- `Content-Type: application/json` (required)
- `Accept: application/json` (required)
- `X-Postmark-Server-Token` (required)

**Example Request:**
```bash
curl "https://api.postmarkapp.com/server" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: server token" \
  -d '{
    "Name": "Production 2",
    "Color": "Blue",
    "TrackOpens": true,
    "TrackLinks": "HtmlAndText"
  }'
```

**Editable Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| Name | string | Server name |
| Color | string | Display color (Purple, Blue, Turquoise, Green, Red, Yellow, Grey, Orange) |
| RawEmailEnabled | boolean | Enable raw email in webhooks |
| DeliveryHookUrl | string | Delivery webhook endpoint |
| SmtpApiActivated | boolean | Enable SMTP |
| InboundHookUrl | string | Inbound webhook endpoint |
| BounceHookUrl | string | Bounce webhook endpoint |
| OpenHookUrl | string | Open tracking webhook endpoint |
| PostFirstOpenOnly | boolean | Limit open tracking to first open |
| TrackOpens | boolean | Enable open tracking |
| TrackLinks | string | Link tracking scope |
| ClickHookUrl | string | Click webhook endpoint |
| InboundDomain | string | MX domain configuration |
| InboundSpamThreshold | integer | Spam blocking threshold |
| IncludeBounceContentInHook | boolean | Include content in bounce notifications |
| EnableSmtpApiErrorHooks | boolean | Include SMTP errors in bounces |

**Response:** Returns updated server configuration with all fields listed above.

---

## Key Notes

- `DeliveryType` (Live or Sandbox) is immutable after creation
- Server tokens provide server-level API access
- Webhook URLs enable event notifications for various email activities
- Tracking settings apply to all emails sent through the server by default

## Related Documentation

- [Servers API](../servers-api/README.md) - Manage multiple servers
- [Webhooks API](../webhooks-api/README.md) - Configure webhooks
- [Message Streams API](../message-streams-api/README.md) - Manage message streams
