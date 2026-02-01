# Declare Actions - Gmail Markup Documentation

## Overview

An Action in schema.org represents a verb or activity that can be performed on a piece of structured data. Gmail supports multiple action types, all defined using similar structured data formats.

## Go-To Actions

Go-To actions function as clickable links within email content. These actions are added to schema.org entities via the `potentialAction` property.

### Basic Example (JSON-LD)

```json
{
  "@context": "http://schema.org",
  "@type": "EmailMessage",
  "potentialAction": {
    "@type": "ViewAction",
    "target": "https://watch-movies.com/watch?movieId=abc123",
    "name": "Watch movie"
  },
  "description": "Watch the 'Avengers' movie online"
}
```

### Basic Example (Microdata)

```html
<div itemscope itemtype="http://schema.org/EmailMessage">
  <div itemprop="potentialAction" itemscope itemtype="http://schema.org/ViewAction">
    <link itemprop="target" href="https://watch-movies.com/watch?movieId=abc123"/>
    <meta itemprop="name" content="Watch movie"/>
  </div>
  <meta itemprop="description" content="Watch the 'Avengers' movie online"/>
</div>
```

**Note:** Email clients without schema support automatically ignore this markup.

## Mobile Deep Linking

Actions can direct users to native mobile applications using custom URL schemes:

### URL Scheme Format

```
"target": [
  "<web url>",
  "android-app://<android package name>/<scheme>/<host>/<path+query>",
  "ios-app://<App store ID>/<scheme>/<host><path+query>"
]
```

### Extended Example (JSON-LD)

```json
{
  "@context": "http://schema.org",
  "@type": "EmailMessage",
  "potentialAction": {
    "@type": "ViewAction",
    "target": [
      "https://watch-movies.com/watch?movieId=abc123",
      "android-app://com.watchmovies.app/http/watch-movies.com/watch?movieId=abc123",
      "ios-app://12345/movieapp/watch-movies.com/watch?movieId=abc123"
    ]
  }
}
```

If users lack the app, the action defaults to the web URL.

## In-App Actions

In-App Actions process requests within Gmail without redirecting users elsewhere. These require an `HttpActionHandler` instead of a direct `target`.

### ConfirmAction Example (JSON-LD)

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

## Expiring Actions

Actions can automatically expire based on associated dates or explicit time windows using `startTime` and `endTime` properties.

### Expiration Example (JSON-LD)

```json
{
  "@context": "http://schema.org",
  "@type": "EmailMessage",
  "potentialAction": {
    "@type": "ConfirmAction",
    "name": "Save coupon",
    "handler": {
      "@type": "HttpActionHandler",
      "url": "https://my-coupons.com/approve?couponId=abc123"
    },
    "startTime": "2015-06-01T12:00:00Z",
    "endTime": "2015-06-05T12:00:00Z"
  }
}
```

## Additional Resources

- Handling Action Requests
- Securing Actions
- Android Deep Linking documentation
- iOS Deep Linking documentation

**Note:** Schema implementations may evolve during standardization at schema.org.
