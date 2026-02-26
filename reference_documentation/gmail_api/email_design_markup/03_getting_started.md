# Gmail Email Markup: Getting Started

## Overview

Gmail email markup leverages structured data to enhance email functionality. The platform supports two formats for annotation:

- **JSON-LD**: Script-based structured data
- **Microdata**: HTML attribute-based markup

These formats enable Gmail to understand email content and deliver relevant search results, interactive features, and informational cards.

## Basic Implementation

### Use Case Example

Consider a ticket confirmation email. Rather than plain text, markup provides semantic meaning to reservation details like event names, dates, venues, and confirmation numbers.

### JSON-LD Approach

Structured data appears within a `<script type="application/ld+json">` tag in the email body:

```json
{
  "@context": "http://schema.org",
  "@type": "EventReservation",
  "reservationNumber": "IO12345",
  "underName": {
    "@type": "Person",
    "name": "John Smith"
  },
  "reservationFor": {
    "@type": "Event",
    "name": "Google I/O 2013",
    "startDate": "2013-05-15T08:30:00-08:00",
    "location": {
      "@type": "Place",
      "name": "Moscone Center",
      "address": {
        "@type": "PostalAddress",
        "streetAddress": "800 Howard St.",
        "addressLocality": "San Francisco",
        "addressRegion": "CA",
        "postalCode": "94103",
        "addressCountry": "US"
      }
    }
  }
}
```

### Microdata Approach

HTML attributes embed structured information directly into visible content:

```html
<div itemscope itemtype="http://schema.org/EventReservation">
  <meta itemprop="reservationNumber" content="IO12345"/>
  <div itemprop="underName" itemscope itemtype="http://schema.org/Person">
    <meta itemprop="name" content="John Smith"/>
  </div>
</div>
```

### Inline Microdata Variant

Markup integrates seamlessly with readable HTML:

```html
<p itemscope itemtype="http://schema.org/EventReservation">
  Reservation number: <span itemprop="reservationNumber">IO12345</span>
</p>
```

## Key Considerations

Some schemas used by Google are still going through the standardization process and may evolve. Developers should consult the reference guide for comprehensive property documentation and review schema.org proposals for emerging standards.
