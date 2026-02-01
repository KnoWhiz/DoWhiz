# Sending Email with Gmail API

## Overview

The Gmail API provides two primary methods for sending emails:

1. **Direct sending** via the `messages.send` method
2. **Draft-based sending** via the `drafts.send` method

## Basic Workflow

The general process involves three steps:

1. Create email content and encode it as a base64url string
2. Create a message resource with the encoded string in its `raw` property
3. Call the appropriate send method

## Creating Messages

The Gmail API requires MIME messages compliant with RFC 2822. Most programming languages offer libraries to simplify this process.

### Java Implementation

Using the `MimeMessage` class from `javax.mail.internet`:

```java
import javax.mail.*;
import javax.mail.internet.*;
import java.util.Properties;

public class CreateEmail {
  public static MimeMessage createEmail(String toEmailAddress,
                                        String fromEmailAddress,
                                        String subject,
                                        String bodyText)
      throws MessagingException {
    Properties props = new Properties();
    Session session = Session.getDefaultInstance(props, null);
    MimeMessage email = new MimeMessage(session);

    email.setFrom(new InternetAddress(fromEmailAddress));
    email.addRecipient(Message.RecipientType.TO,
        new InternetAddress(toEmailAddress));
    email.setSubject(subject);
    email.setText(bodyText);
    return email;
  }
}
```

Encoding the message:

```java
import com.google.api.services.gmail.model.Message;
import org.apache.commons.codec.binary.Base64;
import java.io.ByteArrayOutputStream;

public class CreateMessage {
  public static Message createMessageWithEmail(MimeMessage emailContent)
      throws MessagingException, IOException {
    ByteArrayOutputStream buffer = new ByteArrayOutputStream();
    emailContent.writeTo(buffer);
    byte[] bytes = buffer.toByteArray();
    String encodedEmail = Base64.encodeBase64URLSafeString(bytes);
    Message message = new Message();
    message.setRaw(encodedEmail);
    return message;
  }
}
```

### Python Implementation

```python
import base64
from email.message import EmailMessage
from googleapiclient.discovery import build
import google.auth

def gmail_create_draft():
  creds, _ = google.auth.default()
  service = build("gmail", "v1", credentials=creds)

  message = EmailMessage()
  message.set_content("This is automated draft mail")
  message["To"] = "gduser1@workspacesamples.dev"
  message["From"] = "gduser2@workspacesamples.dev"
  message["Subject"] = "Automated draft"

  encoded_message = base64.urlsafe_b64encode(
      message.as_bytes()).decode()

  create_message = {"message": {"raw": encoded_message}}
  draft = (service.users().drafts().create(
      userId="me", body=create_message).execute())

  print(f'Draft id: {draft["id"]}')
  return draft
```

## Messages with Attachments

Creating multipart MIME messages allows file attachments. The encoding and sending process remains identical.

### Java with Attachments

```java
import javax.activation.*;
import javax.mail.Multipart;
import javax.mail.internet.MimeBodyPart;
import javax.mail.internet.MimeMultipart;

// Create message
MimeMessage email = new MimeMessage(session);
email.setFrom(new InternetAddress(fromEmailAddress));
email.addRecipient(Message.RecipientType.TO,
    new InternetAddress(toEmailAddress));
email.setSubject("Test message");

// Create multipart content
MimeBodyPart mimeBodyPart = new MimeBodyPart();
mimeBodyPart.setContent("lorem ipsum.", "text/plain");
Multipart multipart = new MimeMultipart();
multipart.addBodyPart(mimeBodyPart);

// Add attachment
mimeBodyPart = new MimeBodyPart();
DataSource source = new FileDataSource(file);
mimeBodyPart.setDataHandler(new DataHandler(source));
mimeBodyPart.setFileName(file.getName());
multipart.addBodyPart(mimeBodyPart);
email.setContent(multipart);
```

### Python with Attachments

```python
import mimetypes
from email.mime.audio import MIMEAudio
from email.mime.image import MIMEImage
from email.mime.base import MIMEBase

mime_message = EmailMessage()
mime_message["To"] = "gduser1@workspacesamples.dev"
mime_message["From"] = "gduser2@workspacesamples.dev"
mime_message["Subject"] = "sample with attachment"
mime_message.set_content(
    "Hi, this is automated mail with attachment.")

attachment_filename = "photo.jpg"
type_subtype, _ = mimetypes.guess_type(attachment_filename)
maintype, subtype = type_subtype.split("/")

with open(attachment_filename, "rb") as fp:
  attachment_data = fp.read()
mime_message.add_attachment(
    attachment_data, maintype, subtype)
```

## Sending Messages

### Java Implementation

```java
import com.google.api.services.gmail.Gmail;
import com.google.auth.oauth2.GoogleCredentials;

public class SendMessage {
  public static Message sendEmail(String fromEmailAddress,
                                  String toEmailAddress)
      throws MessagingException, IOException {
    GoogleCredentials credentials =
        GoogleCredentials.getApplicationDefault()
        .createScoped(GmailScopes.GMAIL_SEND);
    Gmail service = new Gmail.Builder(
        new NetHttpTransport(),
        GsonFactory.getDefaultInstance(),
        new HttpCredentialsAdapter(credentials))
        .setApplicationName("Gmail samples")
        .build();

    // Create and encode message (as shown above)
    Message message = new Message();
    message.setRaw(encodedEmail);

    message = service.users().messages()
        .send("me", message).execute();
    System.out.println("Message id: " + message.getId());
    return message;
  }
}
```

### Python Implementation

```python
def gmail_send_message():
  creds, _ = google.auth.default()
  service = build("gmail", "v1", credentials=creds)

  message = EmailMessage()
  message.set_content("This is automated draft mail")
  message["To"] = "gduser1@workspacesamples.dev"
  message["From"] = "gduser2@workspacesamples.dev"
  message["Subject"] = "Automated draft"

  encoded_message = base64.urlsafe_b64encode(
      message.as_bytes()).decode()

  create_message = {"raw": encoded_message}
  send_message = (service.users().messages().send(
      userId="me", body=create_message).execute())

  print(f'Message Id: {send_message["id"]}')
  return send_message
```

## Threading Emails

To ensure replies thread correctly:

- Match the `Subject` headers between messages
- Follow RFC 2822 standards for `References` and `In-Reply-To` headers

## Additional Resources

- Creating Drafts guide for draft-based sending
- RFC 2822 specification for email standards
- API Reference for detailed method documentation
