# Security Requirements for AMP for Gmail

## Overview

Dynamic emails in Gmail must meet strict security standards to protect user safety and privacy. Non-compliance may result in improper rendering or the AMP content failing to display entirely.

## Sender Authentication

Three authentication mechanisms are required:

1. **DKIM (Domain Keys Identified Mail)**: The email must pass DKIM authentication checks.

2. **DKIM Alignment**: The DKIM-authenticated signing domain must be aligned with the domain of the email in the `From` field.

3. **SPF (Sender Policy Framework)**: The email must pass SPF authentication.

### DKIM Alignment Details

Alignment requires matching organizational domains between the DKIM signature and sender address. The "Organizational Domain" (eTLD+1) of the DKIM signing domain must match the organizational domain in the From header.

**Example**: If DKIM signature uses `d=foo.example.com`, aligned From addresses include:
- `bar@foo.example.com`
- `foo@example.com`
- `foo@bar.example.com`

Non-aligned example: `user@gmail.com` (different organizational domain)

### DMARC Recommendation

Senders should implement Domain-based Message Authentication, Reporting and Conformance (DMARC) policies with disposition set to `quarantine` or `reject`. This may become mandatory in the future.

## TLS Encryption

To ensure the contents of an AMP email are encrypted in transit, you must TLS Encrypt emails containing AMP.

## HTTP Proxy Requirements

All XMLHttpRequests from AMP emails are proxied to protect user privacy. This means XHR requests cannot include cookies.

### CORS Headers

All server endpoints used by `amp-list` and `amp-form` must implement CORS in AMP for Email and correctly set the `AMP-Email-Allow-Sender` HTTP header.

### URL Restrictions

**Redirects**: XHR URLs mustn't use HTTP redirection. Requests that return a status code from the redirection class (`3XX` range) fail, resulting in a browser console warning message.
