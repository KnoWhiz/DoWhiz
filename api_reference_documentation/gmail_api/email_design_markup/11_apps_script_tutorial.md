# Apps Script Quickstart for Gmail Email Markup

## Overview

This guide demonstrates using Google Apps Script to send test emails with schemas for email markup validation without requiring registration with Google.

## Creating the Project

1. Navigate to [script.google.com](https://script.google.com)
2. Click **Start Scripting** if redirected to the information page
3. Create a **Blank Project**

### Code Implementation

Replace the content of `Code.gs` with:

```javascript
/**
 * Send an email with schemas in order to test email markup.
 */
function testSchemas() {
  try {
    const htmlBody =
      HtmlService.createHtmlOutputFromFile("mail_template").getContent();

    MailApp.sendEmail({
      to: Session.getActiveUser().getEmail(),
      subject: `Test Email markup - ${new Date()}`,
      htmlBody: htmlBody,
    });
  } catch (err) {
    console.log(err.message);
  }
}
```

### HTML Template

Create a new HTML file named `mail_template` with this content:

```html
<html>
  <head>
    <script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "EmailMessage",
      "description": "Check this out",
      "potentialAction": {
        "@type": "ViewAction",
        "target": "https://www.youtube.com/watch?v=eH8KwfdkSqU"
      }
    }
    </script>
  </head>
  <body>
    <p>
      This a test for a Go-To action in Gmail.
    </p>
  </body>
</html>
```

## Testing

1. Save the project
2. Select the `Code.gs` tab
3. Ensure `testSchemas` is selected in the function dropdown
4. Click **Run**
5. Grant authorization when prompted
6. Check your inbox for the test email with a Go-To Action button

## How It Works

The script retrieves HTML content from `mail_template.html` and sends it as an email to the authenticated user's inbox, allowing schema testing without formal registration requirements.
