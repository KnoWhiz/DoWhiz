# Working with Drafts

## Overview

Drafts represent unsent messages with the `DRAFT` system label applied. The underlying message within a draft cannot be modified once created, though it may be replaced entirely. The draft resource serves as a container offering a stable ID since message IDs change with each replacement.

Messages in drafts behave similarly to standard messages with key distinctions:

- Draft messages cannot carry labels beyond the `DRAFT` system label
- Upon sending, the draft is automatically removed and a new message with updated ID receives the `SENT` system label, returned in the `drafts.send` response

## Creating Draft Messages

Applications can create drafts using the `drafts.create` method through this process:

1. Create a MIME message compliant with RFC 2822
2. Convert the message to a base64url encoded string
3. Call `drafts.create`, setting `drafts.message.raw` to the encoded string

### Java Example

```java
import com.google.api.client.googleapis.json.GoogleJsonError;
import com.google.api.client.googleapis.json.GoogleJsonResponseException;
import com.google.api.client.http.HttpRequestInitializer;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.services.gmail.Gmail;
import com.google.api.services.gmail.GmailScopes;
import com.google.api.services.gmail.model.Draft;
import com.google.api.services.gmail.model.Message;
import com.google.auth.http.HttpCredentialsAdapter;
import com.google.auth.oauth2.GoogleCredentials;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Properties;
import javax.mail.MessagingException;
import javax.mail.Session;
import javax.mail.internet.InternetAddress;
import javax.mail.internet.MimeMessage;
import org.apache.commons.codec.binary.Base64;

public class CreateDraft {
  public static Draft createDraftMessage(String fromEmailAddress,
                                         String toEmailAddress)
      throws MessagingException, IOException {
    GoogleCredentials credentials = GoogleCredentials.getApplicationDefault()
        .createScoped(GmailScopes.GMAIL_COMPOSE);
    HttpRequestInitializer requestInitializer = new HttpCredentialsAdapter(credentials);

    Gmail service = new Gmail.Builder(new NetHttpTransport(),
        GsonFactory.getDefaultInstance(),
        requestInitializer)
        .setApplicationName("Gmail samples")
        .build();

    String messageSubject = "Test message";
    String bodyText = "lorem ipsum.";

    Properties props = new Properties();
    Session session = Session.getDefaultInstance(props, null);
    MimeMessage email = new MimeMessage(session);
    email.setFrom(new InternetAddress(fromEmailAddress));
    email.addRecipient(javax.mail.Message.RecipientType.TO,
        new InternetAddress(toEmailAddress));
    email.setSubject(messageSubject);
    email.setText(bodyText);

    ByteArrayOutputStream buffer = new ByteArrayOutputStream();
    email.writeTo(buffer);
    byte[] rawMessageBytes = buffer.toByteArray();
    String encodedEmail = Base64.encodeBase64URLSafeString(rawMessageBytes);
    Message message = new Message();
    message.setRaw(encodedEmail);

    try {
      Draft draft = new Draft();
      draft.setMessage(message);
      draft = service.users().drafts().create("me", draft).execute();
      System.out.println("Draft id: " + draft.getId());
      System.out.println(draft.toPrettyString());
      return draft;
    } catch (GoogleJsonResponseException e) {
      GoogleJsonError error = e.getDetails();
      if (error.getCode() == 403) {
        System.err.println("Unable to create draft: " + e.getMessage());
      } else {
        throw e;
      }
    }
    return null;
  }
}
```

### Python Example

```python
import base64
from email.message import EmailMessage

import google.auth
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

def gmail_create_draft():
  """Create and insert a draft email.
   Print the returned draft's message and id.
   Returns: Draft object, including draft id and message meta data.
  """
  creds, _ = google.auth.default()

  try:
    service = build("gmail", "v1", credentials=creds)

    message = EmailMessage()

    message.set_content("This is automated draft mail")

    message["To"] = "gduser1@workspacesamples.dev"
    message["From"] = "gduser2@workspacesamples.dev"
    message["Subject"] = "Automated draft"

    encoded_message = base64.urlsafe_b64encode(message.as_bytes()).decode()

    create_message = {"message": {"raw": encoded_message}}
    draft = (
        service.users()
        .drafts()
        .create(userId="me", body=create_message)
        .execute()
    )

    print(f'Draft id: {draft["id"]}\nDraft message: {draft["message"]}')

  except HttpError as error:
    print(f"An error occurred: {error}")
    draft = None

  return draft

if __name__ == "__main__":
  gmail_create_draft()
```

## Updating Drafts

To update a draft, provide a `Draft` resource with `draft.message.raw` set to a base64url-encoded MIME message. Because messages cannot be updated, the message contained in the draft is destroyed and replaced by the new MIME message supplied in the update request.

Retrieve current MIME content using `drafts.get` with parameter `format=raw`. See `drafts.update` API reference for details.

## Sending Drafts

When sending, you can transmit the message as-is or with updated content. For updates, supply a `Draft` resource in the `drafts.send` request body with:

- `draft.id` identifying the draft
- `draft.message.raw` containing the new base64url-encoded MIME message

Consult the `drafts.send` API reference for complete information.
