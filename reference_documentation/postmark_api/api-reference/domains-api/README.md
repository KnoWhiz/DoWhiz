# Postmark Domains API Documentation

## Overview

The Domains API enables account-level management of domain configurations, including authentication verification and DNS record management for email delivery.

**Authentication**: Requires `X-Postmark-Account-Token` header with account owner credentials.

---

## Endpoints

### List Domains

**Endpoint**: `GET /domains`

Retrieves all domains associated with the account with authentication status overview.

**Request Headers**:
- `Accept: application/json` (required)
- `X-Postmark-Account-Token` (required)

**Query Parameters**:

| Parameter | Required | Description |
|-----------|----------|-------------|
| count | Yes | Records per request, max 500 |
| offset | Yes | Number of records to skip |

**Response Fields**:
- `TotalCount`: Integer matching search criteria
- `Domains`: Array of domain objects

**Example Request**:
```bash
curl "https://api.postmarkapp.com/domains?count=50&offset=0" \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

**Example Response**:
```json
{
  "TotalCount": 2,
  "Domains": [
    {
      "Name": "example.com",
      "DKIMVerified": true,
      "WeakDKIM": false,
      "ReturnPathDomainVerified": true,
      "ID": 12345
    }
  ]
}
```

---

### Get Domain Details

**Endpoint**: `GET /domains/{domainid}`

Retrieves comprehensive configuration details for a specific domain.

**Response Fields**:

| Field | Type | Description |
|-------|------|-------------|
| Name | string | Domain name |
| ID | integer | Domain identifier |
| DKIMVerified | boolean | DKIM verification status |
| WeakDKIM | boolean | Sub-1024-bit DKIM strength |
| DKIMHost | string | Current DKIM DNS hostname |
| DKIMTextValue | string | Current DKIM DNS TXT value |
| DKIMPendingHost | string | Pending DKIM hostname (awaiting verification) |
| DKIMPendingTextValue | string | Pending DKIM TXT value |
| DKIMRevokedHost | string | Deprecated DKIM hostname |
| DKIMRevokedTextValue | string | Deprecated DKIM TXT value |
| SafeToRemoveRevokedKeyFromDNS | boolean | Safe to delete old DNS records |
| DKIMUpdateStatus | string | "Pending" or "Verified" |
| ReturnPathDomain | string | Custom Return-Path subdomain |
| ReturnPathDomainVerified | boolean | Return-Path verification status |
| ReturnPathDomainCNAMEValue | string | Expected CNAME record value |

---

### Create Domain

**Endpoint**: `POST /domains`

Registers a new domain for email sending.

**Request Headers**:
- `Content-Type: application/json` (required)
- `Accept: application/json` (required)
- `X-Postmark-Account-Token` (required)

**Body Parameters**:

| Parameter | Required | Description |
|-----------|----------|-------------|
| Name | Yes | Domain name string |
| ReturnPathDomain | No | Custom subdomain for bounce handling; must be a subdomain with CNAME pointing to `pm.mtasv.net` |

**Example Request**:
```bash
curl "https://api.postmarkapp.com/domains" \
  -X POST \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Account-Token: account token" \
  -d '{
    "Name": "example.com",
    "ReturnPathDomain": "pm-bounces.example.com"
  }'
```

---

### Edit Domain

**Endpoint**: `PUT /domains/{domainid}`

Updates domain configuration settings.

**Body Parameters**:
- `ReturnPathDomain` (optional): Modify bounce handling domain

---

### Delete Domain

**Endpoint**: `DELETE /domains/{domainid}`

Removes a domain from the account.

**Response**:
```json
{
  "ErrorCode": 0,
  "Message": "Domain example.com removed."
}
```

---

## Authentication Operations

### Verify DKIM Status

**Endpoint**: `PUT /domains/{domainid}/verifyDkim`

Checks and confirms DKIM DNS record configuration.

**Example Request**:
```bash
curl "https://api.postmarkapp.com/domains/12345/verifyDkim" \
  -X PUT \
  -H "Accept: application/json" \
  -H "X-Postmark-Account-Token: account token"
```

---

### Verify Return-Path Status

**Endpoint**: `PUT /domains/{domainid}/verifyReturnPath`

Validates Return-Path CNAME DNS record setup.

---

### Verify SPF Record

**Endpoint**: `POST /domains/{domainid}/verifyspf`

> **Deprecated**: This method has been deprecated. SPF configuration is no longer required.

---

### Rotate DKIM Keys

**Endpoint**: `POST /domains/{domainid}/rotatedkim`

Generates replacement DKIM credentials for enhanced security.

**Process Flow**:

1. New key created; pending values populate `DKIMPendingHost`/`DKIMPendingTextValue`
2. Add new DNS records based on pending values
3. After DNS verification, pending values migrate to active `DKIMTextValue`/`DKIMHost`
4. Previous key moves to `DKIMRevokedHost`/`DKIMRevokedTextValue` for deprecation
5. `DKIMUpdateStatus` transitions from "Pending" to "Verified"
6. Remove old DNS records once `SafeToRemoveRevokedKeyFromDNS` is `true`

---

## DNS Configuration Guide

### DKIM Setup

Add a TXT record to your DNS:

| Field | Value |
|-------|-------|
| Type | TXT |
| Name | Value from `DKIMPendingHost` (e.g., `20230101._domainkey`) |
| Value | Value from `DKIMPendingTextValue` |
| TTL | 3600 (or your preference) |

### Return-Path Setup

Add a CNAME record:

| Field | Value |
|-------|-------|
| Type | CNAME |
| Name | Your ReturnPathDomain subdomain (e.g., `pm-bounces`) |
| Value | `pm.mtasv.net` |
| TTL | 3600 |

---

## Key Implementation Notes

- DKIM verification remains true historically even after DNS record removal
- Return-Path domain requires functional CNAME record pointing to Postmark infrastructure
- DKIM rotation involves temporary coexistence of multiple DNS records during verification
- SPF configuration is no longer required

## Related Documentation

- [Sender Signatures API](../sender-signatures-api/README.md)
- [Server API](../server-api/README.md)
