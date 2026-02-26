# Sending Email with SMTP

## Overview

Postmark enables developers to send emails via SMTP without major code changes. This allows seamless migration from existing SMTP implementations by simply updating configuration settings to point to Postmark's servers.

## Why Choose SMTP?

The SMTP approach offers a practical advantage: "you can simply change a configuration setting in your application and instantly switch delivery to Postmark" rather than requiring extensive code refactoring.

## Connection Configuration

### Server Endpoints

| Type | Endpoint |
|------|----------|
| Transactional | `smtp.postmarkapp.com` |
| Broadcast | `smtp-broadcasts.postmarkapp.com` |

### Available Ports

Support for ports 25, 2525, and 587 provides flexibility for different network environments.

### Security

TLS encryption is available via STARTTLS extension, establishing encrypted connections for credentials and message content transmission.

## Authentication Methods

Two authentication approaches are available:

### Option 1: API Token + Header

- **Username/Password**: Server API Token (used for both fields)
- **Custom Header**: `X-PM-MESSAGE-STREAM: {stream-id}`
- Default stream: Uses `outbound` if header omitted

### Option 2: SMTP Token

- **Username**: Access Key
- **Password**: Secret Key
- Stream-specific credentials

## Supported Authentication Protocols

| Protocol | Security Level | Notes |
|----------|----------------|-------|
| CRAM-MD5 | Partial | Protects authentication only |
| DIGEST-MD5 | Partial | Protects authentication only |
| PLAIN | Low | Use only with TLS enabled |
| LOGIN | Low | Legacy client compatibility |

## Advanced Features

### Custom Tags

Include tags using SMTP header: `X-PM-Tag: welcome-email`

### Metadata Support

Add custom metadata fields with headers prefixed `X-PM-Metadata-`:
```
X-PM-Metadata-color: blue
X-PM-Metadata-client-id: 12345
```

### Message-ID Preservation

Prevent automatic Message-ID replacement: `X-PM-KeepID: true`

## Tracking Capabilities

- **Open Tracking**: Enable per-server or per-message basis
- **Link Tracking**: Multiple activation methods available
- **Metadata**: Supports custom metadata fields via headers

## Key Differences from REST API

The REST API offers advantages SMTP lacks:

| Feature | REST API | SMTP |
|---------|----------|------|
| Automatic retry logic | Yes | No |
| Batch sending | Yes (up to 500) | No |
| Immediate success/error responses | Yes | No |
| Better error handling for deactivated recipients | Yes | No |

SMTP accepts all messages and logs errors separately through bounce webhooks and APIs.

## Troubleshooting

### Common Issues

**Authentication Failures**: Verify you're using the transactional stream's API token and SMTP is enabled in server settings.

**Recipient Splitting**: Some clients (Gmail, Office 365) may submit recipients separately, causing message fragmentation.

**Port Congestion**: Test all three available ports; try alternate ports if issues persist.

**Character Set Problems**: Use Quoted-Printable encoding for maximum compatibility across mail systems.

### Error Monitoring

Since SMTP cannot return errors directly, monitor bounces via webhooks or the bounce API endpoint for `SMTPApiError` bounce type notifications.

## Global Infrastructure

Postmark maintains SMTP endpoints across multiple AWS regions, automatically routing to nearest servers for minimal latency.

## Example Configuration

### Ruby/Rails with ActionMailer

```ruby
# config/environments/production.rb
config.action_mailer.delivery_method = :smtp
config.action_mailer.smtp_settings = {
  address: "smtp.postmarkapp.com",
  port: 587,
  user_name: "YOUR_SERVER_TOKEN",
  password: "YOUR_SERVER_TOKEN",
  authentication: :plain,
  enable_starttls_auto: true
}
```

### Node.js with Nodemailer

```javascript
const transporter = nodemailer.createTransport({
  host: "smtp.postmarkapp.com",
  port: 587,
  secure: false,
  auth: {
    user: "YOUR_SERVER_TOKEN",
    pass: "YOUR_SERVER_TOKEN"
  }
});
```

### PHP with PHPMailer

```php
$mail = new PHPMailer();
$mail->isSMTP();
$mail->Host = 'smtp.postmarkapp.com';
$mail->SMTPAuth = true;
$mail->Username = 'YOUR_SERVER_TOKEN';
$mail->Password = 'YOUR_SERVER_TOKEN';
$mail->SMTPSecure = PHPMailer::ENCRYPTION_STARTTLS;
$mail->Port = 587;
```

## Related Documentation

- [Send Email with API](../sending-email-api/README.md)
- [Server API](../../api-reference/server-api/README.md)
- [Bounce Webhook](../../webhooks/bounce-webhook/README.md)
