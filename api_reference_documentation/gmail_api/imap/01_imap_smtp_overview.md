# IMAP, POP, and SMTP Documentation

## Overview

Gmail facilitates access through standard email protocols for clients that don't use Gmail natively. The service extends these protocols with OAuth 2.0 support for modern authentication.

## Supported Protocols

Gmail supports three standard protocols:
- **IMAP** - for receiving messages
- **POP** - for receiving messages
- **SMTP** - for sending messages

## Authentication Mechanism

These protocols utilize the Simple Authentication and Security Layer (SASL) standard, which includes protocol-specific authentication commands:
- IMAP uses the `AUTHENTICATE` command
- POP uses the `AUTH` command
- SMTP uses the `AUTH` command

The SASL XOAUTH2 mechanism enables clients to authenticate using OAuth 2.0 credentials instead of traditional passwords.

## Server Endpoints and Security

| Service | Server | Port | Security |
|---------|--------|------|----------|
| IMAP | imap.gmail.com | 993 | SSL (required) |
| POP | pop.gmail.com | 995 | SSL (required) |
| SMTP | smtp.gmail.com | 465 or 587 | SSL (port 465) or TLS (port 587) |

## Session Duration Limits

Connection length restrictions vary by protocol:

- **POP sessions**: Limited to approximately 7 days
- **IMAP sessions**: Limited to approximately 24 hours
- **OAuth sessions**: Limited to the access token validity period (typically 1 hour)

When sessions expire, Gmail terminates the connection. Clients must reconnect and reauthenticate to continue operations. Using OAuth requires ensuring the access token remains current.

## Implementation Resources

The documentation references available code samples and libraries that implement SASL XOAUTH2 across multiple programming languages. Any IMAP or SMTP library supporting SASL should be compatible with Gmail's extended authentication mechanism.
