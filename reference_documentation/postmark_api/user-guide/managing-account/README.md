# Managing Your Account - Postmark Developer Documentation

## Overview

Postmark provides API endpoints specifically designed for account-level management. These tools enable users to control sender signatures and servers through dedicated account management endpoints.

## Key Capabilities

The account management API allows you to:

- Create, read, edit, and delete sender signatures
- Create, read, edit, and delete servers
- Manage DKIM configuration
- Verify SPF entries

## Authentication

### Two-Token System

Postmark uses distinct authentication methods for different API operations:

| Token Type | Header | Use Case |
|------------|--------|----------|
| Server Token | `X-Postmark-Server-Token` | Email sending, message searching, server-level operations |
| Account Token | `X-Postmark-Account-Token` | Account-level operations (signatures, servers, domains) |

### Obtaining Your Account Token

The Account API token is exclusively available to the account owner. You can retrieve it from the "API Tokens" section of your Postmark account dashboard.

### Authentication Errors

Failed authentication attempts return an HTTP 401 (Unauthorized) status code.

## Management Topics

### Managing Sender Signatures

Configure and maintain email sender identities. Sender signatures represent the email address appearing in the "From" field.

**Key operations:**
- Create new sender signatures
- Edit existing signatures
- Delete signatures
- Resend confirmation emails
- Manage DKIM and Return-Path settings

See [Managing Sender Signatures](managing-sender-signatures.md) for details.

### Managing Servers

Create and configure server instances for email operations. Servers organize emails by application or environment.

**Key operations:**
- Create new servers
- Edit server configuration
- Delete servers
- Configure webhooks
- Set tracking options

See [Managing Servers](managing-servers.md) for details.

## API Endpoints Summary

### Sender Signatures

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/senders` | List all sender signatures |
| GET | `/senders/{id}` | Get signature details |
| POST | `/senders` | Create new signature |
| PUT | `/senders/{id}` | Edit signature |
| DELETE | `/senders/{id}` | Delete signature |
| POST | `/senders/{id}/resend` | Resend confirmation |

### Servers

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/servers` | List all servers |
| GET | `/servers/{id}` | Get server details |
| POST | `/servers` | Create new server |
| PUT | `/servers/{id}` | Edit server |
| DELETE | `/servers/{id}` | Delete server |

### Domains

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/domains` | List all domains |
| GET | `/domains/{id}` | Get domain details |
| POST | `/domains` | Create new domain |
| PUT | `/domains/{id}` | Edit domain |
| DELETE | `/domains/{id}` | Delete domain |
| PUT | `/domains/{id}/verifyDkim` | Verify DKIM |
| PUT | `/domains/{id}/verifyReturnPath` | Verify Return-Path |

## Related Documentation

- [Managing Sender Signatures](managing-sender-signatures.md)
- [Managing Servers](managing-servers.md)
- [Sender Signatures API](../../api-reference/sender-signatures-api/README.md)
- [Servers API](../../api-reference/servers-api/README.md)
- [Domains API](../../api-reference/domains-api/README.md)
