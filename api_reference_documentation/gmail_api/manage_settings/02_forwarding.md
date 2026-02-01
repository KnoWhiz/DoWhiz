# Managing Forwarding in Gmail API

## Overview

The Gmail Settings API enables programmatic configuration of email forwarding. Before implementing forwarding, addresses must meet specific verification criteria.

## Forwarding Address Requirements

To use an address as a forwarding destination, it must satisfy one of these conditions:

- The address has been verified (see creation and verification section below)
- It belongs to the sender's domain
- It's on a subdomain within the sender's domain
- It's configured as a domain alias within the same Google Workspace account

If an address doesn't meet these requirements, the API will reject the forwarding setup.

## Creating and Verifying Forwarding Addresses

### Creation Process

Forwarding addresses must be created before use. Some require owner verification.

### Verification Status

- **"pending"**: Gmail requires verification; a message is sent to the target address for confirmation
- **"accepted"**: No verification needed; ready for use immediately

## Enabling Auto-Forwarding

### Method Requirements

Call `updateAutoForwarding()` with:
- A registered and verified forwarding address
- A disposition action for forwarded messages

### Java Implementation

```java
import com.google.api.client.googleapis.json.GoogleJsonError;
import com.google.api.client.googleapis.json.GoogleJsonResponseException;
import com.google.api.client.http.HttpRequestInitializer;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.services.gmail.Gmail;
import com.google.api.services.gmail.GmailScopes;
import com.google.api.services.gmail.model.AutoForwarding;
import com.google.api.services.gmail.model.ForwardingAddress;
import com.google.auth.http.HttpCredentialsAdapter;
import com.google.auth.oauth2.GoogleCredentials;
import java.io.IOException;

public class EnableForwarding {
  public static AutoForwarding enableAutoForwarding(String forwardingEmail)
      throws IOException {
    GoogleCredentials credentials = GoogleCredentials.getApplicationDefault()
        .createScoped(GmailScopes.GMAIL_SETTINGS_SHARING);
    HttpRequestInitializer requestInitializer =
        new HttpCredentialsAdapter(credentials);

    Gmail service = new Gmail.Builder(new NetHttpTransport(),
        GsonFactory.getDefaultInstance(),
        requestInitializer)
        .setApplicationName("Gmail samples")
        .build();

    try {
      ForwardingAddress address = new ForwardingAddress()
          .setForwardingEmail(forwardingEmail);
      ForwardingAddress createAddressResult =
          service.users().settings().forwardingAddresses()
          .create("me", address).execute();

      if (createAddressResult.getVerificationStatus().equals("accepted")) {
        AutoForwarding autoForwarding = new AutoForwarding()
            .setEnabled(true)
            .setEmailAddress(address.getForwardingEmail())
            .setDisposition("trash");
        autoForwarding =
            service.users().settings().updateAutoForwarding("me",
            autoForwarding).execute();
        System.out.println(autoForwarding.toPrettyString());
        return autoForwarding;
      }
    } catch (GoogleJsonResponseException e) {
      GoogleJsonError error = e.getDetails();
      if (error.getCode() == 403) {
        System.err.println("Unable to enable forwarding: " +
            e.getDetails());
      } else {
        throw e;
      }
    }
    return null;
  }
}
```

### Python Implementation

```python
import google.auth
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

def enable_forwarding():
  """Enable email forwarding."""
  creds, _ = google.auth.default()

  try:
    service = build("gmail", "v1", credentials=creds)
    address = {"forwardingEmail": "gduser1@workspacesamples.dev"}

    result = (
        service.users()
        .settings()
        .forwardingAddresses()
        .create(userId="me", body=address)
        .execute()
    )

    if result.get("verificationStatus") == "accepted":
      body = {
          "emailAddress": result.get("forwardingEmail"),
          "enabled": True,
          "disposition": "trash",
      }
      result = (
          service.users()
          .settings()
          .updateAutoForwarding(userId="me", body=body)
          .execute()
      )
      print(f"Forwarding is enabled : {result}")

  except HttpError as error:
    print(f"An error occurred: {error}")
    result = None

  return result

if __name__ == "__main__":
  enable_forwarding()
```

## Disabling Auto-Forwarding

Call `updateAutoForwarding()` and set the `enabled` property to `false`.

## Selective Message Forwarding

To forward only specific messages instead of all incoming mail, use the filters API to establish rules based on message attributes or content.

## API References

- **ForwardingAddresses**: create, list, get, delete
- **AutoForwarding Settings**: get, update
