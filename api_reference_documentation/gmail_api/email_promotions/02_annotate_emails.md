# Annotate Emails in the Promotions Tab

## Overview

Email annotations enhance promotional messages in Gmail's Promotions tab with interactive features including images, deals, and expiration dates. Users can engage with annotated content directly from their inbox without opening the email.

This documentation covers implementing annotations using JSON-LD and Microdata formats.

## Building Email Annotations

Four annotation types are available:

### 1. Product Carousel

Displays up to 10 unique image previews for promotions.

**Structure (JSON-LD):**

```json
{
  "@context": "http://schema.org/",
  "@type": "PromotionCard",
  "image": "IMAGE_URL",
  "url": "PROMO_URL",
  "headline": "HEADLINE",
  "price": PRICE,
  "priceCurrency": "PRICE_CURRENCY",
  "discountValue": DISCOUNT_VALUE,
  "position": POSITION
}
```

**Key Parameters:**
- `IMAGE_URL`: PNG/JPEG in supported aspect ratios (4:5, 1:1, 1.91:1)
- `PROMO_URL`: Destination when users click the image
- `HEADLINE`: 1-2 line promotional description
- `PRICE`: Promotion price
- `PRICE_CURRENCY`: 3-letter ISO 4217 code
- `DISCOUNT_VALUE`: Amount subtracted from original price
- `POSITION`: Card position in carousel

### 2. Single Image Preview

Shows one featured product image with optional pricing and discount details.

**Key Difference:** Aspect ratio requirement is 1.91:1; all recipients receive identical content.

### 3. Deal Annotation

Creates a promotional badge adjacent to the subject line highlighting key offer information.

**Required Fields (at least one):**
- `DESCRIPTION`: Concise offer summary
- `DISCOUNT_CODE`: Checkout code
- `END_DATE_TIME`: Promotion end in ISO 8601 format

**Recommended:**
- `START_DATE_TIME`: Promotion start in ISO 8601 format

### 4. Deal Card

Visually summarizes promotions, appearing in Promotions tab and within the email.

**Required Values:**
- `DESCRIPTION`: Short offer summary
- At least one URL: `OFFER_PAGE_URL` (preferred) or `MERCHANT_HOMEPAGE_URL`

**Recommended:**
- `DISCOUNT_CODE`
- `START_DATE_TIME` and `END_DATE_TIME` in ISO 8601 format

## Implementation Steps

1. Open email HTML editor in your marketing platform
2. Add schema markup (JSON-LD or Microdata) to the `<head>` section
3. Customize properties matching your promotion
4. Compose email message in `<body>` tag
5. Validate and preview annotations before sending

## Important Notes

- Annotations visibility depends on quality filters and frequency limits
- Not all recipients may see annotations
- Image URLs must be unique within carousels
- Contact p-Promo-Outreach@google.com for troubleshooting

## Next Steps

- Review reference documentation for available markup fields
- Consult best practices guide for optimization strategies
