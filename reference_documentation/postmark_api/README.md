# Postmark API Documentation

This directory contains comprehensive documentation for the Postmark email delivery and tracking API.

## Overview

Postmark is an email delivery and tracking service that replaces traditional SMTP infrastructure. The platform provides reliable, scalable email management with comprehensive analytics capabilities.

## Documentation Structure

```
postmark_api/
├── overview/
│   └── introduction.md
├── integration/
│   ├── official-libraries/
│   │   └── README.md
│   ├── community-libraries/
│   │   └── README.md
│   ├── tools-integrations/
│   │   └── README.md
│   └── ai-prompts/
│       └── README.md
├── user-guide/
│   ├── sending-email-api/
│   │   ├── README.md
│   │   ├── send-single-email.md
│   │   └── batch-emails.md
│   ├── sending-email-smtp/
│   │   └── README.md
│   ├── processing-email/
│   │   ├── README.md
│   │   ├── inbound-domain-forwarding.md
│   │   ├── parse-an-email.md
│   │   └── sample-inbound-workflow.md
│   ├── tracking-opens/
│   │   └── README.md
│   ├── tracking-links/
│   │   └── README.md
│   ├── managing-account/
│   │   ├── README.md
│   │   ├── managing-sender-signatures.md
│   │   └── managing-servers.md
│   └── sandbox-mode/
│       └── README.md
├── api-reference/
│   ├── overview/
│   │   └── README.md
│   ├── email-api/
│   │   └── README.md
│   ├── bounce-api/
│   │   └── README.md
│   ├── templates-api/
│   │   └── README.md
│   ├── server-api/
│   │   └── README.md
│   ├── servers-api/
│   │   └── README.md
│   ├── message-streams-api/
│   │   └── README.md
│   ├── messages-api/
│   │   └── README.md
│   ├── domains-api/
│   │   └── README.md
│   ├── sender-signatures-api/
│   │   └── README.md
│   ├── stats-api/
│   │   └── README.md
│   ├── webhooks-api/
│   │   └── README.md
│   └── suppressions-api/
│       └── README.md
└── webhooks/
    ├── overview/
    │   └── README.md
    ├── bounce-webhook/
    │   └── README.md
    ├── inbound-webhook/
    │   └── README.md
    ├── open-tracking-webhook/
    │   └── README.md
    ├── delivery-webhook/
    │   └── README.md
    ├── click-webhook/
    │   └── README.md
    ├── spam-complaint-webhook/
    │   └── README.md
    ├── subscription-change-webhook/
    │   └── README.md
    └── smtp-api-error/
        └── README.md
```

## Key Features

### Email Delivery
- **REST API**: Send single or batch emails via HTTP POST
- **SMTP**: Traditional SMTP integration for easy migration
- **Templates**: Server-side email templating with variable substitution

### Email Processing
- **Inbound Processing**: Receive and parse incoming emails as JSON
- **Webhooks**: Real-time event notifications for bounces, opens, clicks, etc.

### Tracking & Analytics
- **Open Tracking**: Monitor email opens via invisible pixel
- **Link Tracking**: Track link clicks with geographic and device data
- **Statistics API**: Comprehensive sending and engagement statistics

### Account Management
- **Servers**: Organize emails by application or environment
- **Message Streams**: Separate transactional and broadcast messages
- **Sender Signatures**: Manage verified sender addresses
- **Domains**: Configure DKIM and Return-Path authentication

## Core Concepts

### Email Stream Organization
- **Transactional Streams**: One-to-one messages triggered by user actions (welcome emails, password resets, order confirmations)
- **Broadcast Streams**: Bulk communications like newsletters and announcements

### Authentication
- **Server Token**: `X-Postmark-Server-Token` header for server-level operations
- **Account Token**: `X-Postmark-Account-Token` header for account-level management

### Limits
- Maximum email size: 10 MB (including content, headers, and attachments)
- Maximum recipients per email: 50 (combined To, CC, and BCC)
- Maximum batch size: 500 messages per API call

## Quick Links

- [API Overview](api-reference/overview/README.md)
- [Send Email with API](user-guide/sending-email-api/README.md)
- [Webhooks Overview](webhooks/overview/README.md)
- [Official Libraries](integration/official-libraries/README.md)

## Base API URL

```
https://api.postmarkapp.com
```

All API requests require TLS encryption.
