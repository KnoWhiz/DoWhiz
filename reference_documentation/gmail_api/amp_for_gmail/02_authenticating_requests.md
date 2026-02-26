# Authenticating Requests in AMP for Email

## Overview

Dynamic personalized email content often requires user authentication. However, HTTP requests made from AMP emails within Gmail are proxied and stripped of cookies to protect user data.

The solution is to use **access tokens** for authentication in AMP emails.

## Access Tokens

Access tokens authenticate users and are supplied by the email sender. These tokens:

- Are cryptographically secure
- Have time and scope limitations
- Are included within request URLs
- Ensure only those with access to the AMP email can make contained requests

### Implementation with amp-list

```html
<amp-list src="https://example.com/endpoint?token=REPLACE_WITH_YOUR_ACCESS_TOKEN"
      height="300">
      <template type="amp-mustache">
        ...
      </template>
    </amp-list>
```

### Implementation with amp-form

```html
<form action-xhr="https://example.com/endpoint?token=REPLACE_WITH_YOUR_ACCESS_TOKEN" method="post">
      <input type="text" name="data">
      <input type="submit" value="Send">
    </form>
```

## Important Considerations

**Token Lifetime:** Gmail only considers an AMP part to be useable for 30 days after it is received so the lifetime of your access token should be set to 31 days.

**Email Forwarding:** Email clients typically strip the AMP mime part on reply / forward.

## Example Use Case

A note-taking service sends an email to a user containing their previous notes. The service generates a cryptographically secure token (`A3a4roX9x`) and includes it in the URL query parameter:

```html
<amp-list src="https://example.com/personal-notes?exampletoken=A3a4roX9x" height="300">
      <template type="amp-mustache">
        <p>{{note}}</p>
      </template>
    </amp-list>
```

The endpoint validates the token parameter and identifies the associated user, since traditional cookie-based authentication doesn't function in Gmail AMP emails.
