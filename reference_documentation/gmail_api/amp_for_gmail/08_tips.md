# Tips and Known Limitations for AMP for Gmail

This documentation provides guidance for implementing AMP (Accelerated Mobile Pages) emails in Gmail, complementing the broader AMP for Email best practices available on amp.dev.

## Key Tips and Limitations

### Preheader and Search Indexing

The preview text shown next to the subject line derives from the `text/html` or `text/plain` sections, not the `text/x-amp-html` portion. Similarly, Gmail's search function doesn't index dynamic content, so developers should place important keywords in the fallback HTML or plain text sections to improve discoverability.

### Email Markup and Annotations

Email Markup and Promotions Annotations are parsed from the `text/html` part of the email and ignored in the `text/x-amp-html` part. This means these features won't function within the dynamic AMP section.

### Form Data Encoding

Form data submitted through `amp-form` elements is consistently encoded as `application/x-www-form-urlencoded`, regardless of the `enctype` attribute specified on the form element itself.

### Duplicate Email Handling

Dynamic emails may generate duplicates within a thread. To address this, developers can implement the `email.contentIds` meta tag:

```html
<head>
  <meta name="email.contentIds" content="id1,id2,id3">
</head>
```

Gmail collapses duplicate emails when the final message's content IDs form a superset of all previous messages in the thread.

### Form Interaction Warnings

Users may receive confirmation prompts when closing emails with modified form inputs that haven't been submitted, though this doesn't occur in all scenarios like form clearing or message deletion.

### Template Restrictions

amp-mustache templates can't contain set delimiter tags, limiting customization options within dynamic template sections.
