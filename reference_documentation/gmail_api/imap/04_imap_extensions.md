# IMAP Extensions for Gmail

## Overview

Gmail provides IMAP extensions enabling developers to create more Gmail-like experiences through standard IMAP protocol. These are alternatives to the RESTful Gmail API for integrating Gmail features into applications.

## Checking for Extension Support

Gmail advertises extension support via the `CAPABILITY` command. Support is indicated by `X-GM-EXT-1` in the capabilities list.

**Recommended Practice:** Clients should announce themselves using the IMAP ID command (RFC 2971) with a contact address for potential future updates.

**Example handshake:**
```
* OK Gimap ready for requests from 127.0.0.1 k2if6111336rvb.0
a001 LOGIN username@gmail.com password
a002 CAPABILITY
* CAPABILITY IMAP4rev1 UNSELECT LITERAL+ IDLE NAMESPACE QUOTA ID XLIST CHILDREN X-GM-EXT-1
a002 OK Success
```

## Special-Use Extension (LIST Command)

Gmail supports RFC 6154 for special-use mailbox attributes, identifying folders like Starred, Important, Sent Items, Drafts, Spam, All Mail, and Trash.

**Example response:**
```
a004 LIST "" "*"
* LIST (\HasNoChildren) "/" "INBOX"
* LIST (\HasNoChildren \All) "/" "[Gmail]/All Mail"
* LIST (\HasNoChildren \Drafts) "/" "[Gmail]/Drafts"
* LIST (\HasNoChildren \Important) "/" "[Gmail]/Important"
* LIST (\HasNoChildren \Sent) "/" "[Gmail]/Sent Mail"
* LIST (\HasNoChildren \Junk) "/" "[Gmail]/Spam"
* LIST (\HasNoChildren \Flagged) "/" "[Gmail]/Starred"
* LIST (\HasNoChildren \Trash) "/" "[Gmail]/Trash"
```

## XLIST Deprecation

The Gmail-specific `XLIST` command was deprecated in 2013 in favor of RFC 6154 standards. Clients should migrate to the Special-Use standard.

## X-GM-RAW: Enhanced Search

Provides access to full Gmail search syntax, interpreting arguments like the Gmail web interface.

**Example:**
```
a005 SEARCH X-GM-RAW "has:attachment in:unread"
* SEARCH 123 12344 5992
a005 OK SEARCH (Success)
```

## X-GM-MSGID: Gmail Message ID

Retrieves Gmail's unique 64-bit message identifier (decimal equivalent of web interface hex string).

**Fetch example:**
```
a006 FETCH 1 (X-GM-MSGID)
* 1 FETCH (X-GM-MSGID 1278455344230334865)
```

**Search example:**
```
a007 UID SEARCH X-GM-MSGID 1278455344230334865
* SEARCH 1
```

## X-GM-THRID: Gmail Thread ID

Retrieves Gmail's 64-bit thread identifier for message grouping.

**Fetch example:**
```
a008 FETCH 1:4 (X-GM-THRID)
* 1 FETCH (X-GM-THRID 1278455344230334865)
* 2 FETCH (X-GM-THRID 1266894439832287888)
```

**Search example:**
```
a009 UID SEARCH X-GM-THRID 1266894439832287888
* SEARCH 2 3 4
```

## X-GM-LABELS: Gmail Labels

Gmail treats labels as folders, modifiable using standard IMAP commands (`CREATE`, `RENAME`, `DELETE`). System labels are prefixed with `[Gmail]` or `[GoogleMail]`.

**Fetch example:**
```
a010 FETCH 1:4 (X-GM-LABELS)
* 1 FETCH (X-GM-LABELS (\Inbox \Sent Important "Muy Importante"))
* 2 FETCH (X-GM-LABELS (foo))
* 3 FETCH (X-GM-LABELS ())
* 4 FETCH (X-GM-LABELS (\Drafts))
```

**Adding labels:**
```
a011 STORE 1 +X-GM-LABELS (foo)
* 1 FETCH (X-GM-LABELS (\Inbox \Sent Important "Muy Importante" foo))
```

**Searching by label:**
```
a012 SEARCH X-GM-LABELS foo
* SEARCH 1 2
```

## References

- RFC 3501: Internet Message Access Protocol
- RFC 2971: IMAP4 ID Extension
- RFC 6154: IMAP LIST Extension for Special-Use Mailboxes
