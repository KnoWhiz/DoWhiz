# Securing Actions in Gmail

## Overview

This documentation explains how Gmail enforces security measures for email actions, ensuring safe delivery and execution of user interactions with email content.

## Security Measures Enforced by Google

For schemas embedded in email, Gmail requires:

1. **Registration**: Sender must Register with Google
2. **SPF or DKIM**: Emails with schema markup must arrive from SPF or DKIM authenticated domains

## Additional Measures for Inline Actions

Extra protections apply to actionable elements:

- **HTTPS Required**: All actions must be handled via HTTPS URLs. Hosts must have valid SSL server certificates installed.

- **Access Tokens**: Implementation of Limited-Use Access Tokens is encouraged to defend against replay attacks

- **Bearer Authorization**: Services should verify the HTTP "Authorization" header, which contains a Bearer Token string confirming the request originates from google.com. This verification uses Google-provided open source library tools.

**Note**: Bearer tokens require explicit request during registration with Google.

## Edge-Case Email Access Patterns

| Access Pattern | Security Response |
|---|---|
| **Manual Forwarding** | Actions rejected (DKIM signature breaks) |
| **Auto Forwarding to Gmail** | Actions accepted (user verification required) |
| **Gmail POP Fetching** | Actions accepted (DKIM preserved) |
| **Third-party Applications** | Actions accepted/rejected based on sender preference; bearer token mismatch provides opportunity for rejection |
