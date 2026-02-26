# Debug Dynamic Emails - Gmail Documentation

## Overview

Gmail offers debugging capabilities to identify why dynamic emails fail to render as AMP (Accelerated Mobile Pages) emails.

## Enabling the Fallback Debugging Banner

To activate debugging in Gmail Web:

1. Whitelist the sender's address in developer settings
2. Refresh the page

The banner displays reasons why AMP content isn't rendering. **Note:** The banner won't appear if dynamic email is disabled in settings.

## Common Fallback Reasons

| Reason | Description |
|--------|-------------|
| `ACCOUNT_TYPE` | Dynamic emails aren't supported by your account type |
| `AUTH_FAILED` | Email failed authentication requirements |
| `AUTO_FORWARDED` | The email was automatically forwarded from a different account |
| `BROWSER_ERROR` | Unexpected error; verify browser compatibility |
| `DKIM_FAILED` | Domain Keys Identified Mail authentication failed |
| `DKIM_NOT_MATCHING_FROM` | DKIM alignment issue detected |
| `DYNAMIC_EMAIL_DISABLED` | Dynamic emails are disabled in Gmail settings |
| `HIDING_IMAGES` | "Always display external images" setting disabled |
| `INVALID_AMP` | AMP markup contains errors |
| `MALFORMED` | Multiple `text/x-amp-html` parts or missing fallback content |
| `MESSAGE_CLIPPED` | AMP section exceeds size limits |
| `NOT_AVAILABLE_ON_PLATFORM` | Sender disabled dynamic content on current platform |
| `OLD_EMAIL` | The email was sent over 30 days ago |
| `PHISHY` | Email marked as phishing |
| `SPAM` | Email marked as spam |
| `TLS_ENCRYPTION` | Email lacks TLS encryption |
| `TRANSLATED` | Translation is enabled for the email |

## Resources

- Testing Dynamic Email
- Security Requirements
- AMP Email Specification
