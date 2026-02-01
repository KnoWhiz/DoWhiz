# Implement Server-Side Authorization for Gmail API

## Overview

Requests to the Gmail API must be authorized using OAuth 2.0 credentials. Server-side flow is appropriate when applications need access to Google APIs while users are offline, requiring an authorization code exchange process between client and server.

## Creating OAuth Credentials

Developers begin by using the setup tool in Google Cloud Console to create a project and enable the Gmail API. The process involves:

1. Navigating to the Credentials page
2. Creating either OAuth client ID or service account credentials
3. Selecting appropriate application type
4. Recording the Client ID for later implementation

## Authorization Flow

### Initial User Authentication

When users first access the application, they receive a permission dialog requesting access to their Gmail account with specified scopes. After this initial authorization, the user is only presented with the permission dialog if your app's client ID changes or the requested scopes have changed.

### Code Exchange Process

The authorization code is a one-time code that your server can exchange for an access token. This token grants temporary access to user data. For offline access, the exchange returns a refresh token for obtaining new access tokens after expiration.

**Critical requirement:** Always store user refresh tokens. Applications needing new refresh tokens must send requests with `approval_prompt` set to `force`.

## Python Implementation Example

The documentation provides sample code demonstrating:

- `exchange_code()`: Converts authorization codes to credentials
- `get_user_info()`: Retrieves user information via UserInfo API
- `get_authorization_url()`: Generates authorization URLs with offline access parameters
- `get_credentials()`: Orchestrates the complete flow with error handling

Key exception types include `CodeExchangeException`, `NoRefreshTokenException`, and `NoUserIdException`.

## Using Stored Credentials

After initial authorization, applications retrieve stored refresh tokens from databases to authorize subsequent requests without prompting users again.

## Building Service Objects

The `build_service()` function creates authorized Gmail service instances:

```python
def build_service(credentials):
  http = httplib2.Http()
  http = credentials.authorize(http)
  return build('gmail', 'v1', http=http)
```

## Making Authorized Requests

API calls use authorized service instances. The example `ListMessages()` function demonstrates message retrieval with pagination support.

**Error handling:** Applications should check for HTTP 401 status codes indicating revoked credentials, then redirect users to re-authorization.

## Next Steps

Developers should consult the API Reference for additional methods and explore guides covering messages, threads, and labels management.
