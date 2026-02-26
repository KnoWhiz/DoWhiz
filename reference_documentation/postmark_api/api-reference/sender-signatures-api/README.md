# Sender Signatures API Documentation

## Overview

The Sender Signatures API enables management of sender signature details at the account level. All requests require the `X-Postmark-Account-Token` header and account owner privileges.

---

## Endpoints

### 1. List Sender Signatures

**Endpoint:** `GET /senders`

**Description:** Retrieves a paginated list of sender signatures with brief details.

**Required Headers:**
- `Accept: application/json`
- `X-Postmark-Account-Token: [account token]`

**Query Parameters:**

| Parameter | Required | Description |
|-----------|----------|-------------|
| count | Yes | Records per request, max 500 |
| offset | Yes | Records to skip |

**Response Fields:**
- `TotalCount`: Total matching signatures
- `SenderSignatures`: Array of signature objects

**Example Request:**
```bash
curl "https://api.postmarkapp.com/senders?count=50&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

**Example Response:**
```json
{
  "TotalCount": 5,
  "SenderSignatures": [
    {
      "Domain": "example.com",
      "EmailAddress": "sender@example.com",
      "ReplyToEmailAddress": "reply@example.com",
      "Name": "Sender Name",
      "Confirmed": true,
      "ID": 12345
    }
  ]
}
```

---

### 2. Get Sender Signature Details

**Endpoint:** `GET /senders/{signatureid}`

**Description:** Retrieves complete details for a specific sender signature, including DKIM and Return-Path information.

**Key Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| Domain | string | Email domain |
| EmailAddress | string | Sender email address |
| ReplyToEmailAddress | string | Reply-to address |
| Name | string | Display name |
| Confirmed | boolean | Confirmation status |
| ID | integer | Signature identifier |
| DKIMVerified | boolean | DKIM verification status |
| WeakDKIM | boolean | Sub-1024-bit DKIM |
| DKIMUpdateStatus | string | "Pending" or "Verified" |
| DKIMPendingHost | string | Pending DKIM hostname |
| DKIMPendingTextValue | string | Pending DKIM TXT value |
| ReturnPathDomain | string | Custom Return-Path subdomain |
| ReturnPathDomainVerified | boolean | Return-Path verification status |
| ReturnPathDomainCNAMEValue | string | Expected CNAME value |
| SafeToRemoveRevokedKeyFromDNS | boolean | Safe to delete old DKIM records |

---

### 3. Create Signature

**Endpoint:** `POST /senders`

**Description:** Creates a new sender signature requiring confirmation.

**Request Headers:**
- `Content-Type: application/json`
- `Accept: application/json`
- `X-Postmark-Account-Token: [account token]`

**Body Parameters:**

| Parameter | Required | Description |
|-----------|----------|-------------|
| FromEmail | Yes | Email address for signature |
| Name | Yes | Display name |
| ReplyToEmail | No | Override for reply-to address |
| ReturnPathDomain | No | Custom Return-Path subdomain (must point to `pm.mtasv.net`) |
| ConfirmationPersonalNote | No | Note in confirmation email (max 400 characters) |

**Example Request:**
```bash
curl "https://api.postmarkapp.com/senders" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "FromEmail": "john.doe@example.com",
    "Name": "John Doe",
    "ReplyToEmail": "reply@example.com",
    "ConfirmationPersonalNote": "Context about Postmark usage"
  }'
```

---

### 4. Edit Signature

**Endpoint:** `PUT /senders/{signatureid}`

**Description:** Modifies an existing sender signature.

**Editable Fields:**

| Parameter | Required | Description |
|-----------|----------|-------------|
| Name | Yes | Updated display name |
| ReplyToEmail | No | Updated reply-to address |
| ReturnPathDomain | No | Updated Return-Path domain |
| ConfirmationPersonalNote | No | Updated confirmation note |

---

### 5. Delete Signature

**Endpoint:** `DELETE /senders/{signatureid}`

**Description:** Removes a sender signature from the account.

**Response:**
```json
{
  "ErrorCode": 0,
  "Message": "Signature someone@example.com removed."
}
```

---

### 6. Resend Confirmation

**Endpoint:** `POST /senders/{signatureid}/resend`

**Description:** Resends the confirmation email for an unconfirmed sender signature.

**Example Response:**
```json
{
  "ErrorCode": 0,
  "Message": "Confirmation email for Sender Signature john.doe@example.com was re-sent."
}
```

---

### 7. Verify SPF Record

**Endpoint:** `POST /senders/{signatureid}/verifyspf`

**Status:** Deprecated

> SPF configuration is no longer required.

---

### 8. Request New DKIM

**Endpoint:** `POST /senders/{signatureid}/requestnewdkim`

**Status:** Deprecated

> Please use the Domains API for updated DKIM methods.

---

## Key Concepts

### Authentication

All requests require account-level token access available from the API Tokens tab. Only account owners can access this token.

### Confirmation

New signatures require email confirmation before they can be used for sending. The confirmation email is sent automatically upon creation.

### DKIM Management

The system tracks DKIM status through multiple states:
- Pending: Awaiting DNS verification
- Verified: DNS records confirmed

Fields track current, pending, and revoked DKIM keys for smooth rotation.

### Return-Path Domains

Custom Return-Path domains must be:
- Subdomains of the sender's email domain
- Have CNAME records pointing to `pm.mtasv.net`

### Domain Restrictions

Sender signatures are restricted to private domains. Public domains (Gmail, Yahoo, etc.) are not allowed because:
- Sending from public domains constitutes email spoofing
- Yahoo and other providers prohibit third-party sending
- DNS modification access is required for DKIM/SPF

## Related Documentation

- [Domains API](../domains-api/README.md)
- [Managing Sender Signatures](../../user-guide/managing-account/managing-sender-signatures.md)
