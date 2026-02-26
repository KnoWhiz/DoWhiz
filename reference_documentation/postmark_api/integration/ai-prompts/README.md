# Postmark AI Prompts

## Overview

Postmark provides a collection of pre-built prompts designed to help developers integrate the email service into applications using AI-powered development tools like Cursor, GitHub Copilot, Windsurf, or Claude.

## Purpose

These prompts equip AI assistants with sufficient context to implement common email workflows—from straightforward API integration to sophisticated authentication systems—while adhering to Postmark's best practices for dependability and email deliverability.

## How to Use These Prompts

### Three Simple Steps

1. **Copy** - Each guide includes a complete, ready-to-use prompt
2. **Paste** - Insert into your preferred AI tool:
   - Cursor or GitHub Copilot via chat or #reference syntax
   - Claude or ChatGPT via direct conversation
   - Windsurf or Zed as project context
3. **Generate** - Receive production-ready code with error handling and security considerations

## What You'll Get

Generated code typically includes:

- Complete integration setup with environment configuration
- Comprehensive error handling and retry mechanisms
- Security best practices (token protection, input validation)
- Appropriate message stream usage
- Metadata for tracking and analytics

**Important Note:** "AI can make mistakes! Always verify the output before using AI generated code."

## Why Use AI Prompts with Postmark?

- **Accelerated Development** - Begin sending emails within minutes rather than extended documentation review
- **Built-in Best Practices** - Prompts encode Postmark's recommendations for security and reliability
- **Avoid Common Pitfalls** - Generated code handles edge cases like bounce management and rate limiting
- **Learn While Building** - AI explanations clarify integration mechanics beyond simple copy-paste

## Available Prompts

### 1. Node.js/Express Integration

**Builds:** Transactional email from Node.js applications with webhook handling for bounces and delivery notifications

**Prerequisites:** Node.js 14+, verified Postmark sender, Server API token

**Key Components:**
- Postmark client configuration
- Welcome email endpoint
- Webhook parsing for delivery events
- Retry logic for failed sends

### 2. Rails Application Integration

**Builds:** ActionMailer configuration for transactional emails with background job processing

**Prerequisites:** Rails 6.0+, verified sender, Server API token

**Key Components:**
- postmark-rails gem installation
- UserMailer with multiple email types
- Background job queue integration
- Email preview functionality

### 3. Laravel Implementation

**Builds:** Laravel Mail driver configuration with Mailable classes and queue support

**Prerequisites:** Laravel 9.0+, verified sender, Server API token

**Key Components:**
- Postmark driver installation
- Custom Mailable classes
- Markdown email templates
- Queue worker configuration

### 4. Password Reset Flow

**Builds:** Complete secure password reset system with token generation and validation

**Key Components:**
- Cryptographically secure token generation
- One-hour token expiration
- Token hashing before storage
- Confirmation emails post-reset
- Rate limiting for abuse prevention

**Security Features:**
- Tokens expire after one hour
- Single active token per user
- Email existence concealment
- HTTPS requirement for reset URLs
- Failed attempt logging

### 5. User Action Email Notifications

**Builds:** Event-driven system sending emails for application events (signup, purchase, profile updates)

**Key Components:**
- Event emitter or message queue integration
- Multiple notification email types
- User preference checking
- Notification history logging
- Retry queuing for failures

**Notification Types:**
- Welcome emails on signup
- Purchase confirmations
- Profile update confirmations
- Password change alerts
- Login activity alerts

### 6. Inbound Email Handling

**Builds:** Webhook receiver for processing incoming emails and attachments

**Key Components:**
- Webhook endpoint creation
- Email content parsing
- Base64 attachment decoding
- Disk or S3 storage options
- Recipient-based routing
- Support ticket creation
- Email reply threading

**Use Cases:**
- Support ticket generation
- Form submission via email
- Email reply handling
- Attachment processing

### 7. Better Auth Integration

**Builds:** Email verification and password reset for Better Auth authentication system

**Key Components:**
- Better Auth configuration with Postmark
- Email verification flow
- Password reset implementation
- Client-side signup/reset components
- Email template design
- Environment variable setup

**Features:**
- Auto sign-in after email verification
- Secure password reset tokens
- Mobile-responsive email templates
- Rate limiting support

## Common Features Across All Prompts

### Production Readiness

- Comprehensive error handling
- Logging for debugging
- Security best practices
- Environment variable usage

### Email Best Practices

- Clear, actionable call-to-action buttons
- Mobile-responsive HTML
- Plain text alternatives
- Security-related messaging
- Personalization with user data

### Integration Features

- Metadata for tracking
- Message stream configuration
- Webhook validation
- Async/await patterns
- Rate limiting considerations

## Key Recommendations

1. **Environment Configuration** - Always use environment variables for sensitive tokens
2. **Testing** - Use sandbox mode or test email addresses during development
3. **Template Usage** - Consider Postmark's template system for easier future updates
4. **Webhook Security** - Validate webhook authenticity where applicable
5. **Verification** - Ensure sender addresses are verified before production use
6. **Monitoring** - Track delivery via Postmark's activity feed

## Related Resources

- [Postmark Node.js library documentation](https://github.com/wildbit/postmark.js)
- [API reference](../../api-reference/overview/README.md)
- [Webhook documentation](../../webhooks/overview/README.md)
- [Message streams guide](../../api-reference/message-streams-api/README.md)
- [Email templates](https://postmarkapp.com/email-templates)
