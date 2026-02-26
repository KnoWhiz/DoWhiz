# Postmark Inbound Email Processing Guide

## Overview

Inbound processing allows you to send inbound emails (hash@inbound.postmarkapp.com) to Postmark, which processes and delivers them to you via a webhook in nicely formatted JSON.

This functionality is particularly useful for applications that need to handle user responses, such as:
- Processing comments on outbound emails
- Managing ticket replies
- Capturing responses from sent messages

## Getting Started

The platform provides a structured approach to implementing inbound email handling through several key guides:

### Setup Steps

1. **Configure an inbound server** - Establish your inbound email receiving infrastructure
2. **Inbound domain forwarding** - Route emails from your domain to Postmark's processing service
3. **Parse an email** - Extract and structure incoming email data
4. **Configure inbound blocking** - Set up filtering rules for unwanted messages
5. **Sample inbound workflow** - Review example implementations

## Key Features

- **Webhook delivery** - Processed emails arrive as structured JSON payloads
- **Automatic parsing** - Emails are formatted for easy application integration
- **Rule-based blocking** - Configure which emails to accept or reject
- **Message retry capability** - Failed inbound messages can be reprocessed

## Webhook Response Handling

When Postmark POSTs to your webhook:

- **Success**: Return a 200 status code
- **Retry Logic**: Non-200 responses trigger retries (up to 10 attempts) with intervals from 1 minute to 6 hours
- **Stop Retries**: A 403 response halts all retry attempts
- **Timeout**: Postmark waits 2 minutes maximum for your response

## Related Documentation

- [Inbound Domain Forwarding](inbound-domain-forwarding.md)
- [Parse an Email](parse-an-email.md)
- [Sample Inbound Workflow](sample-inbound-workflow.md)
- [Inbound Webhook](../../webhooks/inbound-webhook/README.md)
