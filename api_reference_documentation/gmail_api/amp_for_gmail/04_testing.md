# Test Your AMP Emails in Gmail

## Overview

Before deploying AMP-powered dynamic emails, you can validate their functionality and appearance within Gmail using two dedicated testing methods.

## Testing Options

### 1. AMP for Email Playground

Google provides the AMP for Email Playground (https://amp.gmail.dev/playground/) which enables developers to:
- Draft AMP email content
- Preview rendered output in real-time
- Send test messages directly to personal Gmail accounts

**Important:** To receive playground emails, whitelist `amp@gmail.dev` in your Gmail developer settings.

### 2. Gmail Developer Settings

Access testing capabilities through your Gmail account:

1. Navigate to **Settings > General > Dynamic email**
2. Click **Developer settings**
3. Whitelist specific email addresses that can send dynamic content for testing

This approach allows you to test AMP rendering from unlisted senders before formal registration with Google.

## Delivery Requirements

Your AMP email must satisfy these conditions for successful delivery:

- **Security compliance:** Follow all AMP for Email security requirements
- **Fallback versions:** Include either `text/html` or `text/plain` MIME parts alongside the AMP version (`text/x-amp-html`)
- **Valid AMP document:** The AMP content must pass validation checks
- **MIME ordering:** Position the AMP part before the HTML part
- **Distinct addresses:** `From` and `To` headers must contain different email addresses

## Production Testing Recommendation

Make sure to test delivering your email from your production system using option 2 above. This helps verify that the email you send in production meets all the delivery requirements and identify any modifications introduced by your email service provider.
