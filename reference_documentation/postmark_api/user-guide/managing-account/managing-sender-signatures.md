# Managing Sender Signatures

## Overview

Sender signatures represent the email address appearing in the "From" field when sending messages through Postmark. The platform requires prior confirmation of all sender addresses before use.

## Why Use the Sender Signature API

The RESTful API provides programmatic control over sender signatures through the `/senders` endpoint. While most users manage one or two signatures via the Postmark website, the API becomes essential when:

- Sending from numerous addresses
- Dynamically creating sender signatures
- Requiring automated processes without manual intervention

## When NOT to Use the API

Postmark recommends verifying your domain as the preferred approach. While DKIM and Return-Path authentication aren't strictly mandatory for creating a sender signature, implementing them significantly improves email deliverability and security.

## Creating a Sender Signature

### Authentication Requirements

Sender signatures belong to the account level (unlike emails, which belong to individual servers). You'll need an **Account API token** to manage them—accessible only by the account owner via the API Tokens tab.

Use this token in the `X-Postmark-Account-Token` HTTP header for all `/senders` endpoint requests. Authentication failures return HTTP 401 (Unauthorized).

### Required Fields

Two mandatory pieces of information:

#### 1. FromEmail

- The email address displayed as the sender
- Must be a functional mailbox address
- Postmark sends a confirmation email requiring verification before use
- Must use a private domain you control (not Gmail, Yahoo, etc.)

#### 2. Name

- How email clients display the From field
- While required at creation, it's overridable per request
- Example: Create "Nick <support@yourdomain.com>" but send as "Joe <support@yourdomain.com>" without additional setup

### Domain Restrictions

Postmark restricts sender signatures to private domains because:

- Sending from public domains (Gmail, Yahoo) constitutes email spoofing
- "Yahoo has specifically published policies disallowing third parties to send email from a yahoo.com address"
- DNS modification access is required for SPF and DKIM implementation

### Example: Create a Sender Signature

```bash
curl "https://api.postmarkapp.com/senders" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "FromEmail": "john.doe@example.com",
    "Name": "John Doe",
    "ReplyToEmail": "reply@example.com",
    "ConfirmationPersonalNote": "Context about Postmark"
  }'
```

## Response Fields - Key Elements

After creation, important fields include:

| Field | Description |
|-------|-------------|
| `ID` | Used for editing or deleting the signature later |
| `ConfirmationPersonalNote` | Provides context to recipients about Postmark usage |
| `DKIMPendingHost` | DNS hostname for DKIM setup |
| `DKIMPendingTextValue` | DNS TXT record value for DKIM |
| `Confirmed` | Whether the signature has been confirmed |

## Full Lifecycle Management

Using the `/senders` API endpoint, you can:

### List Sender Signatures

```bash
curl "https://api.postmarkapp.com/senders?count=50&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

### Get Signature Details

```bash
curl "https://api.postmarkapp.com/senders/{signatureid}" \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

### Edit Signature

```bash
curl "https://api.postmarkapp.com/senders/{signatureid}" \
  -X PUT \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "Name": "Jane Doe",
    "ReplyToEmail": "jane.doe@example.com"
  }'
```

### Delete Signature

```bash
curl "https://api.postmarkapp.com/senders/{signatureid}" \
  -X DELETE \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

### Resend Confirmation

```bash
curl "https://api.postmarkapp.com/senders/{signatureid}/resend" \
  -X POST \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

## DKIM Configuration

After creating a signature, configure DKIM by adding DNS records:

1. Check `DKIMPendingHost` and `DKIMPendingTextValue` in the signature response
2. Add a TXT record to your DNS with these values
3. After DNS propagation, values migrate to `DKIMHost` and `DKIMTextValue`
4. `DKIMVerified` becomes `true` once verification completes

## Return-Path Configuration

Custom Return-Path domains must:
- Be a subdomain of the sender's email domain
- Have a CNAME record pointing to `pm.mtasv.net`

Example: For sender `user@example.com`, use `pm-bounces.example.com` as the Return-Path domain.

## Best Practices

1. **Use domain verification** - Better than individual signature verification
2. **Configure DKIM** - Improves deliverability and prevents spoofing
3. **Set up Return-Path** - Enables proper bounce handling
4. **Monitor confirmation status** - Unconfirmed signatures cannot send
5. **Store signature IDs** - Needed for updates and deletions

## Related Documentation

- [Sender Signatures API](../../api-reference/sender-signatures-api/README.md)
- [Domains API](../../api-reference/domains-api/README.md)
- [Managing Servers](managing-servers.md)
