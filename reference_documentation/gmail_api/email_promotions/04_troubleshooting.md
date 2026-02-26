# Gmail Promotions Tab Troubleshooting Guide

## Overview

This documentation provides solutions for common issues when implementing email annotations in the Gmail Promotions tab. For additional assistance, contact `p-Promo-Outreach@google.com`.

## Account Settings Prerequisites

Before troubleshooting, verify these account-level settings are enabled:

- **Images:** Always show
- **Conversation view:** On
- **Enable Bundling of Top Email:** On

## Email Construction Issues

### Script Tags in Head Section

If your email infrastructure separates head and body handling, you can place the entire script tag or relevant portions in the email body instead of the head.

### Email Service Provider Stripping Scripts

If your ESP removes script tags, use Schema.org microdata format instead. Example structure:

```html
<div itemscope itemtype="http://schema.org/Organization">
  <meta itemprop="logo" content="https://example.com/logo.png" />
</div>
<div itemscope itemtype="http://schema.org/DiscountOffer">
  <meta itemprop="description" content="20% off" />
  <meta itemprop="discountCode" content="PROMO" />
</div>
```

### Split Annotation Implementation

You can distribute annotation components between head and body:

**Head section example:**
```json
{
  "@context": "http://schema.org/",
  "@type": "Organization",
  "logo": "https://example.com/logo.png"
}
```

**Body section example:**
```json
{
  "@context": "http://schema.org/",
  "@type": "DiscountOffer",
  "description": "20% Off",
  "discountCode": "PROMO"
}
```

**Important:** You can't distribute your annotation fields between the head and body for the same object type.

## Display and Testing Issues

### Email Bundles Not Appearing

- Update to the latest Gmail app version
- Refresh by pulling down in the Promotions tab
- Restart your device
- Test on different devices (tablets may display differently)
- Note: Only consumer accounts see bundles, not Google Workspace accounts
- If still unresolved, create a `promotabtesting@gmail.com` account for testing

### Email Not Reaching Promotions Tab

- Verify you're sending from recognized marketing subdomains
- Check for email filters routing messages to Primary tab
- Review Bulk Senders Guidelines for proper subdomain usage
- For initial testing: manually move email to Promotions tab, then resend from same address with different subject line

### Annotated Email Visible in Bundle

- Confirm email is in Promotions tab
- Ensure email hasn't been opened previously
- Note: Viewing on one device prevents bundling on other devices
- Create multiple test accounts for multi-device testing
- Verify annotation code correctness, especially expiration dates
- Use latest Gmail app version
- Refresh the Promotions tab
- Avoid sensitive categories (adult content, debt collection)

### Image Preview Not Displaying

- Validate image URL in script tag
- Refresh the Promotions tab
- Remove images from sensitive content categories

### Bundle Visibility Inconsistent Across Devices

- Confirm using the same email account
- Remember that opening mail on one device affects visibility on others
