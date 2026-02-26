# Verify Bearer Tokens

## Overview

Bearer tokens are cryptographic tokens produced by Google and included in the `Authorization` header of every In-App Action HTTP Request. These tokens help verify that requests originate from Google and are intended for specific sender domains.

## Bearer Token Structure

A Bearer token appears in the Authorization header like this:

```
POST /approve?expenseId=abc123 HTTP/1.1
Host: your-domain.com
Authorization: Bearer AbCdEf123456
Content-Type: application/x-www-form-urlencoded
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/1.0 (KHTML, like Gecko; Gmail Actions)

confirmed=Approved
```

## Token Characteristics

All bearer tokens sent with actions contain:
- **`azp` field**: Always set to `gmail@system.gserviceaccount.com`
- **`audience` field**: Specifies the sender domain as a URL (e.g., `https://example.com`)

## Security Requirements

Services should verify that requests originate from Google and target the correct sender domain. Unverified tokens warrant an HTTP `401 (Unauthorized)` response.

## Verification Libraries

Google provides open-source libraries for token verification:

- **Java**: https://github.com/google/google-api-java-client
- **Python**: https://github.com/google/google-api-python-client
- **.NET**: https://github.com/google/google-api-dotnet-client

## Code Examples

### Java Implementation

The Java example uses `GoogleIdTokenVerifier` to authenticate tokens, checking both the authorized party and audience fields.

### Python Implementation

The Python example uses `oauth2client` to verify tokens, confirming they're signed by Google and match the expected authorized party.

## Important Note

Bearer tokens in authorization headers are not sent by default. If you require a bearer token to be sent, request it when registering with Google.
