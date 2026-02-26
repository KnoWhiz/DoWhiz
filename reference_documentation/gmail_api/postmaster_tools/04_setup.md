# Set up the API - Gmail Postmaster Tools

## Overview

To use the Postmaster Tools API, applications must include authorization tokens with every request. These tokens identify your application to Google and are required for all operations.

## Authorization Protocol Requirements

The Postmaster Tools API exclusively supports **OAuth 2.0** for authorization. No alternative protocols are accepted. Applications using Sign In With Google may have some authorization aspects handled automatically.

## OAuth 2.0 Authorization Flow

All requests to the API must be authorized by an authenticated user. The general process involves:

1. **Registration**: Register your application in the Google API Console to receive a client ID and client secret
2. **Activation**: Enable the Postmaster Tools API in the Console (skip if not listed)
3. **Scope Request**: Your application requests specific access scopes from Google
4. **User Consent**: Google displays a consent screen to the user
5. **Token Grant**: Upon user approval, Google provides a short-lived access token
6. **API Request**: Your application attaches the token to API requests
7. **Validation & Response**: Google validates the request and token, returning the requested data

Some flows may include additional steps, such as using refresh tokens to obtain new access tokens. Detailed information about specific application flows is available in Google's OAuth 2.0 documentation.

## Postmaster Tools API Scope

| Scope | Meaning |
|-------|---------|
| `https://www.googleapis.com/auth/postmaster.readonly` | Provides read-only access to Postmaster Tools data |

To request access via OAuth 2.0, your application requires the scope information plus registration details from Google (client ID and secret).
