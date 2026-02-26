# Register with Google - Gmail Documentation

## Overview

This page provides guidance for developers who want to implement email markup and schemas in Gmail and need to register their implementation with Google before launching to production users.

## Getting Started with Testing

All schemas you send to yourself (from x@gmail.com to x@gmail.com) will be displayed in Google products. This allows developers to test their markup implementations immediately without registration. Developers should thoroughly test their schemas before proceeding with the registration process.

## Registration Process

When ready to launch marked-up emails to users, follow these steps:

1. **Verify Compliance**: Ensure your implementation meets all guidelines and requirements listed below.

2. **Submit Sample Email**: Send a production-quality email to `schema.whitelisting+sample@gmail.com` that:
   - Originates from production servers with proper DKIM/SPF/From:/Return-Path: headers
   - Contains your actual markup/schema
   - Passes the Email Markup Tester with no errors
   - Includes comprehensive data examples
   - Is sent directly (not forwarded, as Gmail removes markup during forwarding)

3. **Complete Registration Form**: Fill out the official registration form; Google will respond with approval status.

## Registration Guidelines

### Email Sender Quality Requirements

Senders must meet these criteria:

- Authentication via DKIM or SPF is mandatory
- Top-level domain matching between authentication method and sender address
- Static email addresses (e.g., `foo@bar.com`)
- Compliance with Google's bulk sender guidelines
- Established sending history (minimum hundreds of emails daily to Gmail users over several weeks)
- Minimal spam complaint rates from recipients

### Actions and Schema Requirements

- Use the highest-fidelity action available for your use case
- Reserve actions for transactional emails with expected high engagement
- Avoid using actions on promotional bulk mail
- Go-To Actions must deep-link to relevant pages with clear, concise button labels
- Prefer In-App actions when available (like verification or review scenarios)
- Maintain fast, reliable service for action request handling
