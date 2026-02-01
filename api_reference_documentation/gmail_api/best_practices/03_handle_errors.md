# Resolve Errors

## Overview

The Gmail API communicates errors at two levels: HTTP status codes in response headers and detailed JSON objects in response bodies containing troubleshooting information.

## Error Types and Solutions

### 400 Error: Bad Request

**Causes:**
- Missing required fields or parameters
- Invalid field values or field combinations
- Invalid attachments

**Sample Error Response:**
```json
{
  "error": {
    "code": 400,
    "errors": [
      {
        "domain": "global",
        "location": "orderBy",
        "locationType": "parameter",
        "message": "Sorting is not supported for queries with fullText terms. Results are always in descending relevance order.",
        "reason": "badRequest"
      }
    ],
    "message": "Sorting is not supported for queries with fullText terms. Results are always in descending relevance order."
  }
}
```

**Resolution:** Check the message field and adjust your code accordingly.

### 401 Error: Invalid Credentials

**Cause:** Expired or invalid access token, or missing authorization for required scopes.

**Sample Error Response:**
```json
{
  "error": {
    "errors": [
      {
        "domain": "global",
        "reason": "authError",
        "message": "Invalid Credentials",
        "locationType": "header",
        "location": "Authorization",
      }
    ],
    "code": 401,
    "message": "Invalid Credentials"
  }
}
```

**Resolution:** Refresh the access token using your long-lived refresh token. Client libraries handle this automatically. If refresh fails, complete the OAuth authorization flow again.

### 403 Error: Usage Limit Exceeded

This error occurs when quota limits are reached or user lacks proper privileges.

#### 403 - Daily Limit Exceeded

**Sample Error Response:**
```json
{
  "error": {
    "errors": [
      {
        "domain": "usageLimits",
        "reason": "dailyLimitExceeded",
        "message": "Daily Limit Exceeded"
      }
    ],
    "code": 403,
    "message": "Daily Limit Exceeded"
  }
}
```

Visit Google API Console → select your project → Quotas tab → request additional quota.

#### 403 - User Rate Limit Exceeded

**Sample Error Response:**
```json
{
 "error": {
  "errors": [
   {
    "domain": "usageLimits",
    "reason": "userRateLimitExceeded",
    "message": "User Rate Limit Exceeded"
   }
  ],
  "code": 403,
  "message": "User Rate Limit Exceeded"
 }
}
```

**Resolution:** Optimize code to make fewer requests or implement retry logic.

#### 403 - Rate Limit Exceeded

**Sample Error Response:**
```json
{
 "error": {
  "errors": [
   {
    "domain": "usageLimits",
    "message": "Rate Limit Exceeded",
    "reason": "rateLimitExceeded",
   }
  ],
  "code": 403,
  "message": "Rate Limit Exceeded"
 }
}
```

**Resolution:** Implement exponential backoff retry strategy.

#### 403 - Domain Policy

**Message:** "Domain administrators have disabled Gmail apps."

**Sample Error Response:**
```json
{
  "error": {
    "errors": [
      {
        "domain": "global",
        "reason": "domainPolicy",
        "message": "The domain administrators have disabled Gmail apps."
      }
    ],
    "code": 403,
    "message": "The domain administrators have disabled Gmail apps."
  }
}
```

**Resolution:** Inform the user and have them contact their domain administrator for access approval.

### 429 Error: Too Many Requests

This occurs due to:
- Daily per-user sending limits
- Bandwidth limits
- Concurrent request limits

**Mail Sending Limits:** Different for paying Google Workspace users versus trial Gmail accounts. These are per-user and shared across all clients.

**Bandwidth Limits:** Per-user upload/download limits shared across all Gmail API clients.

**Concurrent Requests:** API enforces per-user concurrent request limits to prevent overloading. Large batches or many parallel requests can trigger this.

**Resolution:** Retry using exponential backoff or distribute processing across multiple accounts.

### 500 Error: Backend Error

Unexpected errors during request processing.

**Related codes:** 502 Bad Gateway, 503 Service Unavailable, 504 Gateway Timeout

**Sample Error Response:**
```json
{
 "error": {
  "errors": [
   {
    "domain": "global",
    "reason": "backendError",
    "message": "Backend Error",
   }
  ],
  "code": 500,
  "message": "Backend Error"
 }
}
```

**Resolution:** Implement exponential backoff retry strategy.

## Retry Strategy

Start retry periods at least one second after the error. Implement exponential backoff by increasing wait times between attempts to improve bandwidth efficiency.

## Quota Management

To view or modify usage limits:

1. Ensure a billing account exists for your project
2. Visit Enabled APIs page in the API Console
3. Select the desired API
4. Use **Quotas** tab to adjust limits or request increases

## Batch Request Guidelines

Batching is recommended but larger batches trigger rate limiting more easily. Avoid batches exceeding 50 requests.
