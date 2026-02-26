# Register with Google to Send Dynamic Emails

## Overview

When launching dynamic emails to users, senders must register with Google for each sender email address. Registration only needs to occur once per address, regardless of future content changes or use cases.

## Registration Process

Follow these three steps to register:

### 1. Meet Guidelines and Requirements

Review all guidelines listed in the registration guidelines section below.

### 2. Send a Production Email

Submit a real, production-ready email from your production servers to `ampforemail.whitelisting@gmail.com`. The email must include:

- Dynamic email MIME part
- Similar authentication headers (SPF/DKIM/DMARC/From/Return-Path) as your production environment
- AMP version tested in Gmail prior to sending
- Direct send (not forwarded, as Gmail strips AMP MIME parts during forwarding)

**Important cautions:**
- Google will reject your registration application if you don't send an email for review, if you send a test (blank) email, or if you send an email that doesn't contain an AMP MIME part.
- Do not add the whitelisting address to mailing lists
- Google reviews only the most recent email from each sender address

### 3. Complete Registration Form

Submit the registration form and await response (typically 5 business days).

## Registration Guidelines

Senders must meet these requirements:

- **Production Quality**: Submit a real, production-quality example email, not a demo or 'Hello World' example
- **No Placeholder Text**: Avoid strings like `{name}` or `[Test]` in subject/body
- **Fallback MIME**: Include similar HTML or text MIME parts for users with dynamic email disabled
- **Quality Standards**: Email must contain no bugs
- **Authentication**: Implement SPF, DKIM, and DMARC as specified in security requirements
- **AMP Compliance**: Follow AMP for Email best practices
- **Per-Address Registration**: Each sender email address requires separate registration
- **No Third-Party Authoring**: Prevent direct email composition/sending by external parties (user-generated content within emails is permitted)
- **Policy Compliance**: Follow Gmail's bulk sender guidelines
- **Low Spam Rates**: Maintain low user spam complaint rates
- **Valid Domain**: The eTLD+1 of sender address must serve or redirect to a legitimate, accessible website

## Compliance Notes

Participation is conditional on compliance with Google's bulk sender guidelines and subject to change. Google may revoke participation if abuse or violations are discovered and may request additional verification information during or after registration.
