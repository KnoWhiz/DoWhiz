# OAuth 2.0 Mechanism for Gmail

## Overview

This documentation explains the SASL XOAUTH2 mechanism, which enables OAuth 2.0 access tokens for authenticating to Gmail accounts via IMAP, POP, and SMTP protocols.

## Getting Started with OAuth 2.0

Before implementing XOAUTH2, developers should familiarize themselves with "Using OAuth 2.0 to Access Google APIs," which covers fundamental OAuth 2.0 concepts and client implementation steps.

### OAuth 2.0 Scopes

The standard scope for mail access is `https://mail.google.com/`. Applications requesting this scope must comply with Google's User Data Policy and demonstrate full utilization of the scope.

For applications not requiring full mail access, migrating to the Gmail API with more granular restricted scopes is recommended.

#### Domain-Wide Delegation for Google Workspace

Service accounts using domain-wide delegation can employ the scope `https://www.googleapis.com/auth/gmail.imap_admin`. This scope has special behaviors:

- All labels display via IMAP, including hidden ones
- All messages appear, ignoring user-set folder size limits

**Note:** This scope is exclusively for Google Workspace domain-wide delegation.

## SASL XOAUTH2 Mechanism

### Initial Client Response Format

The client response follows this structure:

```
base64("user=" {User} "^Aauth=Bearer " {Access Token} "^A^A")
```

Where `^A` represents Control+A (octal 001).

**Example (pre-encoding):**
```
user=someuser@example.com^Aauth=Bearer ya29.vF9dft4qmTc2Nvb3RlckBhdHRhdmlzdGEuY29tCg^A^A
```

**After base64 encoding:**
```
dXNlcj1zb21ldXNlckBleGFtcGxlLmNvbQFhdXRoPUJlYXJlciB5YTI5LnZGOWRmdDRxbVRjMk52
YjNSbGNrQmhkSFJoZG1semRHRXVZMjl0Q2cBAQ==
```

### Error Response Format

Server errors return base64-encoded JSON:

```json
{
  "status": "401",
  "schemes": "bearer",
  "scope": "https://mail.google.com/"
}
```

Clients must respond with an empty line to this challenge.

## Protocol-Specific Implementation

### IMAP Protocol Exchange

**Successful Authentication:**
```
C: C01 CAPABILITY
S: * CAPABILITY IMAP4rev1 UNSELECT IDLE NAMESPACE QUOTA XLIST
   CHILDREN XYZZY SASL-IR AUTH=XOAUTH2 AUTH=XOAUTH
S: C01 OK Completed
C: A01 AUTHENTICATE XOAUTH2 dXNlcj1zb21ldXNlckBleGFtcGxlLmNvbQ...
S: A01 OK Success
```

**Key Points:**
- Uses the `AUTHENTICATE` command per RFC 3501
- SASL-IR (RFC 4959) enables sending credentials in the first line
- AUTH=XOAUTH2 capability indicates server support
- Base64 argument should be a single continuous string with no whitespace

**Failed Authentication:**
The server sends the error JSON response, and the client replies with an empty line:
```
S: + eyJzdGF0dXMiOiI0MDEiLCJzY2hlbWVzIjoiYmVhcmVyIG1hYyIsInNjb...
C: [empty line]
S: A01 NO SASL authentication failed
```

### POP Protocol Exchange

**Successful Authentication:**
```
C: AUTH XOAUTH2 dXNlcj1zb21ldXNlckBleGFtcGxlLmNvbQFhdXRoPUJlYX...
S: +OK Welcome.
```

**Failed Authentication:**
```
C: AUTH XOAUTH2 dXNlcj1zb21ldXNlckBleGFtcGxlLmNvbQFhdXRoPUJl...
S: + eyJzdGF0dXMiOiI0MDAiLCJzY2hlbWVzIjoiQmVhcmVyIiwic2NvcGUi...
```

References: RFC 1734

### SMTP Protocol Exchange

**Successful Authentication:**
```
S: 220 mx.google.com ESMTP 12sm2095603fks.9
C: EHLO sender.example.com
S: 250-mx.google.com at your service, [172.31.135.47]
S: 250-SIZE 35651584
S: 250-8BITMIME
S: 250-AUTH LOGIN PLAIN XOAUTH XOAUTH2
S: 250-ENHANCEDSTATUSCODES
S: 250 PIPELINING
C: AUTH XOAUTH2 dXNlcj1zb21ldXNlckBleGFtcGxlLmNvbQFhdXRoPUJl...
S: 235 2.7.0 Accepted
```

**Failed Authentication:**
```
S: 334 eyJzdGF0dXMiOiI0MDEiLCJzY2hlbWVzIjoiYmVhcmVyIG1hYyIsInNj...
C: [empty line]
S: 535-5.7.1 Username and Password not accepted. Learn more at
S: 535 5.7.1 https://support.google.com/mail/?p=BadCredentials...
```

References: RFC 4954

## Related Standards

- OAuth 2.0: Google's OAuth 2.0 Documentation
- IMAP: RFC 3501
- POP: RFC 1081
- SMTP: RFC 2821
- SASL: RFC 4422
- JSON: RFC 4627
- Base64: RFC 4648
- SASL-IR: RFC 4959
- SMTP-AUTH: RFC 4954
