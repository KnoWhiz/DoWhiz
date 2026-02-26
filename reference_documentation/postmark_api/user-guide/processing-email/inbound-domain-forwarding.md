# Inbound Domain Forwarding Guide

## Overview

Postmark enables email reception through DNS configuration, allowing you to process incoming messages to your domain or subdomain.

## Setup Instructions

### Step 1: Configure MX Records

Create an MX record pointing to **inbound.postmarkapp.com** with a priority value of **10**.

**Three configuration options:**

| Option | Name Value | Description |
|--------|------------|-------------|
| Root domain | `@` | Listen on your primary domain |
| Subdomain (recommended) | `example` | Configure a separate subdomain like `example.yourdomain.com` |
| Wildcard domain | `*` | Capture messages across all subdomains (e.g., `user@client1.yourdomain.com`) |

### Step 2: Set the Inbound Domain

Configure your inbound domain through the Stream settings interface, or use the Server API:

```bash
curl "https://api.postmarkapp.com/servers/:serverid" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d "{InboundDomain: 'inbound.yourdomain.com'}"
```

**Key point:** "Inbound domains are unique across Postmark and are Stream-specific."

### Step 3: Enable SMTP

Activate SMTP functionality on your server's settings page.

### Step 4: Receive Messages

Your system is now ready to process incoming email sent to addresses on your configured inbound domain.

## Alternative Option

If you lack DNS access, Postmark supports third-party mail service forwarding (such as Gmail-based solutions).

## DNS Record Example

```
Type: MX
Name: inbound (or @ for root domain)
Value: inbound.postmarkapp.com
Priority: 10
TTL: 3600
```

## Best Practices

1. **Use a subdomain** - Keep inbound email separate from your main domain MX records
2. **Test thoroughly** - Send test emails to verify configuration before production use
3. **Monitor webhook responses** - Ensure your application handles the incoming JSON payloads correctly
4. **Configure spam filtering** - Set appropriate spam thresholds in your server settings

## Related Documentation

- [Parse an Email](parse-an-email.md)
- [Sample Inbound Workflow](sample-inbound-workflow.md)
- [Server API](../../api-reference/server-api/README.md)
