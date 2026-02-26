# Managing Servers in Postmark

## Overview

In Postmark, servers are a way to organize the emails that you are sending or parsing. Each server maintains unique identifiers and API tokens for segregating different applications or deployment stages.

## Purpose of the Servers API

The primary benefits include:

- Separate production, staging, and development environments
- Distinguish between different clients or applications
- Aggregate message activity and statistics per server
- Manage inbound and outbound email processing independently

## Creating a Server

### Authentication Requirements

Server management requires account-level access rather than server-level credentials. You must use an **Account API token** available to the account owner through the API Tokens section of your Postmark dashboard.

Include this token in the `X-Postmark-Account-Token` HTTP header for all `/servers` endpoint requests. Authentication failures return a **401 (Unauthorized)** HTTP status code.

### Required Configuration

The `Name` field is the only mandatory parameter when creating a server. Postmark needs minimal information to establish a new server instance.

### Example: Create a Server

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

### Important Response Fields

After creation, the API response includes:

| Field | Description |
|-------|-------------|
| `ID` | Save this value for subsequent server modifications, edits, or deletions |
| `ApiTokens` | Array containing credentials for sending emails; initially contains one token with expansion capability |
| `InboundHash` | Required for inbound processing; forms your unique server address at `:InboundHash@inbound.postmarkapp.com` |

## Server Management Operations

### List All Servers

```bash
curl "https://api.postmarkapp.com/servers?count=50&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

### Get Server Details

```bash
curl "https://api.postmarkapp.com/servers/{serverid}" \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

### Edit Server Configuration

```bash
curl "https://api.postmarkapp.com/servers/{serverid}" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "Name": "Production 2",
    "Color": "Blue",
    "TrackOpens": true
  }'
```

### Delete a Server

```bash
curl "https://api.postmarkapp.com/servers/{serverid}" \
  -X DELETE \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

**Note:** Server deletion requires special enablement—contact support to request access.

## Server Configuration Options

### Basic Settings

| Field | Type | Description |
|-------|------|-------------|
| `Name` | string | Server identifier |
| `Color` | string | Visual categorization (Purple, Blue, Turquoise, Green, Red, Yellow, Grey, Orange) |
| `SmtpApiActivated` | boolean | Enable SMTP protocol |
| `RawEmailEnabled` | boolean | Include raw email content in webhooks |
| `DeliveryType` | string | Live (production) or Sandbox (testing) - cannot be changed after creation |

### Tracking Settings

| Field | Type | Description |
|-------|------|-------------|
| `TrackOpens` | boolean | Enable open tracking for all emails |
| `TrackLinks` | string | Link tracking mode (None, HtmlAndText, HtmlOnly, TextOnly) |
| `PostFirstOpenOnly` | boolean | Trigger webhook only on initial open |

### Webhook URLs

| Field | Description |
|-------|-------------|
| `BounceHookUrl` | Bounce notification endpoint |
| `OpenHookUrl` | Open tracking endpoint |
| `ClickHookUrl` | Click tracking endpoint |
| `DeliveryHookUrl` | Delivery confirmation endpoint |
| `InboundHookUrl` | Inbound email endpoint |

**Note:** Using dedicated Webhook API is recommended over these deprecated server-level settings.

### Inbound Settings

| Field | Type | Description |
|-------|------|-------------|
| `InboundDomain` | string | MX record domain configuration |
| `InboundSpamThreshold` | integer | Maximum spam score before blocking |
| `IncludeBounceContentInHook` | boolean | Include bounce email content in webhooks |

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

Two environment types are available:

| Type | Description |
|------|-------------|
| **Live** | Production environment (default) |
| **Sandbox** | Testing environment |

The delivery type **cannot be modified** after server creation.

## Best Practices

1. **Separate environments** - Create distinct servers for development, staging, and production
2. **Use meaningful names** - Clear naming helps identify servers quickly
3. **Store server IDs** - Save IDs for programmatic management
4. **Configure webhooks** - Set up event notifications for monitoring
5. **Use appropriate colors** - Visual differentiation aids quick identification

## Related Documentation

- [Servers API](../../api-reference/servers-api/README.md)
- [Server API](../../api-reference/server-api/README.md)
- [Managing Sender Signatures](managing-sender-signatures.md)
- [Message Streams API](../../api-reference/message-streams-api/README.md)
