# Handle Action Requests

## Overview

When users interact with In-App Actions in Gmail emails, Google sends HTTP requests to URLs declared in those actions. This documentation explains how to properly receive and process these requests.

## Example: ConfirmAction Button

### JSON-LD Format

```json
{
  "@context": "http://schema.org",
  "@type": "EmailMessage",
  "potentialAction": {
    "@type": "ConfirmAction",
    "name": "Approve Expense",
    "handler": {
      "@type": "HttpActionHandler",
      "url": "https://myexpenses.com/approve?expenseId=abc123"
    }
  },
  "description": "Approval request for John's $10.13 expense for office supplies"
}
```

### Microdata Format

```html
<div itemscope itemtype="http://schema.org/EmailMessage">
  <div itemprop="potentialAction" itemscope itemtype="http://schema.org/ConfirmAction">
    <meta itemprop="name" content="Approve Expense"/>
    <div itemprop="handler" itemscope itemtype="http://schema.org/HttpActionHandler">
      <link itemprop="url" href="https://myexpenses.com/approve?expenseId=abc123"/>
    </div>
  </div>
  <meta itemprop="description" content="Approval request for John's $10.13 expense for office supplies"/>
</div>
```

## Incoming HTTP Request

When users click an action button, Google issues a POST request:

```
POST /approve?expenseId=abc123 HTTP/1.1
Host: your-domain.com
Authorization: Bearer AbCdEf123456
Content-Type: application/x-www-form-urlencoded
User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/1.0 (KHTML, like Gecko; Gmail Actions)

confirmed=Approved
```

## Three-Step Processing

### Step 1: Verify the Request

Services should validate:
- **Limited Use Access Token** — Protects against replay attacks
- **User Agent** — Confirms request originates from Google: `"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/1.0 (KHTML, like Gecko; Gmail Actions)"`
- **Bearer Token** — Ensures the request is authentic and intended for your service

### Step 2: Process the Action

The service should process the action using URL parameters and additional user-provided data. The request body contains x-www-form-urlencoded data with property names corresponding to the action type (e.g., `confirmed` for ConfirmAction).

### Step 3: Return Response Code

After successful processing:

| Response Code | Treatment |
|---|---|
| 200 (OK) | Action processed successfully |
| 400 (Bad Request) | Google fails the action |
| 401 (Unauthorized) | Google fails the action |
| 404 (Not Found) | Google fails the action |
| 408 (Request Timeout) | Google retries later |

On permanent failure, users receive notification to follow alternative email instructions.

## Related Resources

- Securing Actions documentation
- Bearer token verification guide
- Limited-use access tokens documentation
