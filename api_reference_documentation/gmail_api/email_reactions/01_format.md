# Email Reactions in Gmail

## Overview

Email reactions enable users to respond to messages using emoji in a straightforward manner without friction.

## Message Format

### Structure Requirements

An email reaction is a standard MIME-formatted message containing a specialized component. The message must include a body part with `Content-Type: text/vnd.google.email-reaction+json`.

A body part can be either:
- The top-level message part (making the email's `Content-Type` header `text/vnd.google.email-reaction+json`)
- A sub-part within a multipart MIME section with `Content-Type: text/vnd.google.email-reaction+json` and non-attachment `Content-Disposition`

### Recommended Composition

Gmail suggests positioning the reaction part between standard `text/plain` and `text/html` sections. This accounts for clients that display either the final part regardless of MIME type understanding, or only the initial part.

Include an `In-Reply-To` header referencing the targeted message's ID as a single message identifier.

### JSON Part Specification

The `text/vnd.google.email-reaction+json` section contains JSON with two required fields:

| Field | Description |
|-------|-------------|
| `version` | Integer value; must equal `1` |
| `emoji` | Exactly one emoji per Unicode Technical Standard 51 |

UTF-8 encoding applies to binary `Content-Transfer-Encoding`; otherwise standard encodings work.

## Gmail Validation & Display

### Validation Steps

Gmail validates incoming reaction messages through sequential checks:

1. **JSON parsing**: Malformed JSON invalidates the part
2. **Version verification**: Must be integer `1` (not string)
3. **Emoji validation**: Must match Unicode Standard 51 v15+ specifications

### Display Behavior

Valid reactions display near the referenced message with potential emoji counts and sender identification. Missing or unfound `In-Reply-To` references result in standard email rendering.

Invalid reactions display the `text/html` part when available, otherwise `text/plain`.

### Recommended Limits

Implementations should consider restrictions including:

- Disallow reactions on mailing list messages
- Block reactions for messages with excessive recipients (Gmail's threshold: 20 distinct `To`/`CC` recipients)
- Prevent reactions when recipient isn't in `To` or `CC` fields
- Limit reactions per message (Gmail uses 20 reactions maximum per user)
