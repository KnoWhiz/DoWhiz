# Email Reaction Examples for Gmail

## Overview

This documentation covers valid and invalid formats for email reactions in Gmail, which allow users to respond to messages with emojis.

## Valid Formats

### Multipart Alternative Structure

Email reactions typically contain three parts:

- `text/plain`: Plain text version
- `text/vnd.google.email-reaction+json`: The reaction payload
- `text/html`: HTML version

The reaction JSON requires two fields: `"emoji"` containing a valid emoji character and `"version"` set to `1`.

### Single Body Part Format

Reactions can also use just the JSON part with content type `text/vnd.google.email-reaction+json`. The system supports standard encodings like quoted-printable and base64 for the JSON payload.

### Nested Multipart Structure

For messages with inline attachments, reactions may use `multipart/related` containing a `multipart/alternative` section plus attachment parts, following RFC2387 specifications.

## Invalid Examples

**JSON format errors:**
- Missing closing braces
- Trailing commas in JSON objects

**Version field issues:**
- Missing `version` field entirely
- `version` value other than `1`

**Emoji field problems:**
- Empty emoji strings
- Non-emoji characters (like plain letters)
- Incomplete emoji sequences missing required zero-width joiners

## Key Requirements

Valid reactions must include proper JSON structure, version field set to 1, and a syntactically correct emoji character.
