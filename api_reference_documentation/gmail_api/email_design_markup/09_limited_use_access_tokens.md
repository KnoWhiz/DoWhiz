# Limited Use Access Tokens

## Overview

Limited-Use Access Tokens are security mechanisms designed to protect Gmail actions from request spoofing and replay attacks. They ensure that only the intended recipient can perform a specific action.

## Key Concepts

### Purpose

Limited-Use Access Tokens provide protection from request spoofing and replay attacks, ensuring that the action is performed by the user the message was sent to.

### How They Work

The token system operates through these principles:

1. **Unique Token Generation**: Each token should be generated specifically for a particular action and individual user, making it impossible to derive the token from other parameters.

2. **Validation Before Action**: Before executing any requested action, the system must verify that the token is valid and matches the previously generated value for that user.

3. **One-Time Use**: Once a token is successfully used, it becomes invalid for all future requests, preventing replay attacks.

## Implementation Details

### URL Integration

Tokens should be included in the URL of the `HttpActionHandler`. For example:
- Original endpoint: `http://www.example.com/approve?requestId=123`
- Enhanced endpoint: `http://www.example.com/approve?requestId=123&accessToken=xyz`

### Security Requirements

- The combination of parameters (like `requestId` and `accessToken`) must be pre-generated
- Any request missing the token or containing an incorrect token should be rejected
- After successful use, subsequent requests with identical parameters must be rejected
