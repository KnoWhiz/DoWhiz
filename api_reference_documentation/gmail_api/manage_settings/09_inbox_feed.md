# Gmail Inbox Feed

## Overview

The Gmail Inbox Feed is a read-only feature that allows developers to access Gmail inbox data through an Atom feed format. This capability is exclusively available for Gmail accounts hosted on Google Workspace domains.

## What is Atom?

Atom represents a standardized system enabling users to receive regular updates from various sources—including news websites, blogs, and Gmail—in a centralized location. Users can leverage Atom with feed aggregators (also called newsreaders or RSS/Atom readers) to get notifications about new messages.

## Implementation Details

### Authentication

The Gmail Inbox Feed uses OAuth 2.0 as the preferred authentication method. Developers must use the following scope:

`https://mail.google.com/mail/feed/atom`

### Accessing the Feed

To retrieve the inbox feed, make a GET request to:

`https://mail.google.com/mail/feed/atom`

### Output Format

The Gmail Inbox Feed generates output as an XML document that can be consumed in two ways:

1. Through an RSS aggregator for visual display
2. Directly by custom applications for programmatic processing

## Key Limitations

- This feed is restricted to Google Workspace domain accounts only
- The feed provides read-only access to inbox data
- Standard OAuth 2.0 authentication mechanisms apply

## Related Resources

For additional Gmail API functionality beyond the inbox feed, refer to the complete Gmail API documentation covering draft creation, message sending, mailbox management, and settings configuration.
