# Managing Aliases and Signatures

## Overview

Send-as aliases represent email addresses from which an account can send mail. Every account has at least one alias for its primary email address. These aliases correspond to the "Send mail as" feature in Gmail's web interface.

## Key Concepts

**Purpose of Aliases:**
- Enable users to send emails from multiple addresses
- Manage email signatures across different sending identities
- Correspond to Gmail's web interface "Send mail as" feature

## Creating and Verifying Aliases

Before using aliases, you must create them. Some require user verification:

- **Pending Status**: When Gmail requires verification, the alias returns with status `pending`
- **Accepted Status**: Aliases that don't require verification show as `accepted`
- **Verification Process**: A verification message is automatically sent to the target email address
- **Resend Verification**: Use the verify method to resend requests if needed

### SMTP Configuration

External address aliases should send mail through a remote SMTP mail sending agent (MSA). Configure connection details using the `smtpMsa` field.

## Managing Signatures

Configure email signatures for each alias. The process involves:

1. Listing all send-as aliases for the user
2. Identifying the primary alias
3. Updating the signature field via the API

### Java Example

```java
// Load credentials and create Gmail service
GoogleCredentials credentials = GoogleCredentials.getApplicationDefault()
    .createScoped(GmailScopes.GMAIL_SETTINGS_BASIC);

// Find primary alias
SendAs primaryAlias = null;
ListSendAsResponse aliases = service.users().settings().sendAs().list("me").execute();
for (SendAs alias : aliases.getSendAs()) {
  if (alias.getIsPrimary()) {
    primaryAlias = alias;
    break;
  }
}

// Update signature
SendAs aliasSettings = new SendAs().setSignature("Automated Signature");
SendAs result = service.users().settings().sendAs().patch(
    "me",
    primaryAlias.getSendAsEmail(),
    aliasSettings).execute();
```

### Python Example

```python
# Get credentials and build service
creds, _ = google.auth.default()
service = build("gmail", "v1", credentials=creds)

# Find primary alias
aliases = service.users().settings().sendAs().list(userId="me").execute()
for alias in aliases.get("sendAs"):
  if alias.get("isPrimary"):
    primary_alias = alias
    break

# Update signature
send_as_configuration = {
    "displayName": primary_alias.get("sendAsEmail"),
    "signature": "Automated Signature",
}

result = service.users().settings().sendAs().patch(
    userId="me",
    sendAsEmail=primary_alias.get("sendAsEmail"),
    body=send_as_configuration).execute()
```

## API Reference Methods

Available operations on send-as aliases:
- **Create**: Add new aliases
- **List**: Retrieve all aliases
- **Get**: Fetch specific alias details
- **Update**: Modify alias settings
- **Delete**: Remove aliases
- **Verify**: Resend verification requests
