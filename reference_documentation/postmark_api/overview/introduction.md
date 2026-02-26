# Postmark Developer Documentation - Introduction

## Overview

Postmark is an email delivery and tracking service that replaces traditional SMTP infrastructure. The platform provides reliable, scalable email management with comprehensive analytics capabilities.

## Core Service Description

Postmark "helps deliver and track application email" and replaces "SMTP (or Sendmail) with a far more reliable, scalable and care-free environment." The service enables tracking of "statistics such as number of emails sent or processed, opens, bounces and spam complaints."

## Key Requirements & Constraints

### Email Stream Organization

Postmark categorizes emails into two types:

- **Transactional Streams**: One-to-one messages triggered by user actions (welcome emails, password resets, order confirmations)
- **Broadcast Streams**: Bulk communications like newsletters and announcements

### Sender Verification

"A confirmed email address is required to start sending." Users must verify their sender address in the Signatures section before initiating any email delivery.

### Size & Recipient Limits

- Maximum email size: 10 MB (including content, headers, and attachments)
- Maximum recipients per email: 50 (combined To, CC, and BCC recipients)

### File Type Restrictions

Postmark restricts attachment types and maintains a "list of accepted file types." The platform supports inline image embedding as an alternative.

## Getting Started Pathways

### Integration Options

- Official Postmark libraries (Rails, Ruby, .NET, Java, PHP, Node.js, WordPress, etc.)
- Community-maintained libraries
- Third-party tools and integrations
- AI-powered prompts

### Learning Resources

- User guides for API and SMTP email sending
- Inbound email processing documentation
- Open and link tracking guides
- Account management instructions
- Sandbox mode for testing

## API & Webhook Architecture

The platform provides comprehensive REST APIs for:

- Email delivery (individual and bulk)
- Bounce management
- Template management
- Server configuration
- Message stream handling
- Domain verification
- Statistics and analytics
- Webhook management

Webhooks support event notifications including bounces, inbound messages, opens, deliveries, clicks, spam complaints, and subscription changes.

## AI Tooling Integration

Postmark offers developer-friendly AI features:

- **llms.txt file**: Structured API documentation for AI assistants
- **MCP Server**: Direct integration with compatible AI tools
- **CLI**: Command-line interface for account management and testing

## Next Steps

1. [Set up official libraries](../integration/official-libraries/README.md)
2. [Send your first email with API](../user-guide/sending-email-api/README.md)
3. [Configure webhooks](../webhooks/overview/README.md)
4. [Review API reference](../api-reference/overview/README.md)
