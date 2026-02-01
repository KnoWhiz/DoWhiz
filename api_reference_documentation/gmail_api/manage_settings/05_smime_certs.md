# Managing S/MIME Certificates

## Overview

The Gmail S/MIME API enables programmatic management of S/MIME email certificates for users within Google Workspace domains. An administrator must first enable S/MIME for the domain.

The S/MIME standard provides specifications for public key encryption and signing of MIME data. When configured, Gmail:
- Signs outgoing mail using the user's certificate and private key
- Decrypts incoming mail using the user's private key
- Encrypts outgoing mail using the recipient's public key
- Verifies incoming mail using the sender's public key

Individual S/MIME certificates are uploaded via API for specific email aliases, including primary addresses and custom "Send As" addresses. One certificate per alias is designated as default.

## Authorization Methods

Two authorization approaches are available:

1. **Service Account with Domain-Wide Delegation**: Requires enabling domain-wide delegation of authority
2. **OAuth2 Flow**: Requires end-user consent; domain admin must enable "S/MIME API end user access"

## ACL Scopes

The API uses Gmail settings scopes:
- `gmail.settings.basic` - Required for primary SendAs S/MIME updates
- `gmail.settings.sharing` - Required for custom "from" S/MIME updates

## API Operations

The `users.settings.sendAs.smimeInfo` resource provides certificate management methods:

### Upload S/MIME Key

Use `smimeInfo.insert()` to upload a new S/MIME key. Parameters:
- `userId`: User's email address (use "me" for authenticated user)
- `sendAsEmail`: Target alias email address

The request must include `pkcs12` field containing both key and certificate chain. Passwords go in `encryptedKeyPassword`. The API validates:
- Subject matches specified email
- Expiration validity
- Issuing CA is trusted
- Technical compatibility

### List User's S/MIME Keys

`smimeInfo.list()` returns certificates for a given user and alias (PEM format only, excludes private keys).

### Retrieve Specific Keys

`smimeInfo.get()` returns specific S/MIME keys for an alias (PEM format, no private keys).

### Delete S/MIME Key

`smimeInfo.delete()` removes a key using:
- `userId`
- `sendAsEmail`
- `id`: SmimeInfo's immutable ID

**Note**: Deleted keys become unusable. Revoke with issuing authority before deletion.

### Set Default Key

`smimeInfo.setDefault()` marks a key as default for an alias. Only one default per alias exists; calling this unsets the previous default. Gmail associates the key with the latest `not_before` date as default when encrypting outgoing mail.

## Code Examples

### Java: Create SmimeInfo Resource

```java
import com.google.api.services.gmail.model.SmimeInfo;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Base64;

/* Class to demonstrate the use of Gmail Create SmimeInfo API */
public class CreateSmimeInfo {
  /**
   * Create an SmimeInfo resource for a certificate from file.
   *
   * @param filename Name of the file containing the S/MIME certificate.
   * @param password Password for the certificate file, or null if the file is not
   *                 password-protected.
   * @return An SmimeInfo object with the specified certificate.
   */
  public static SmimeInfo createSmimeInfo(String filename, String password) {
    SmimeInfo smimeInfo = null;
    InputStream in = null;

    try {
      File file = new File(filename);
      in = new FileInputStream(file);
      byte[] fileContent = new byte[(int) file.length()];
      in.read(fileContent);

      smimeInfo = new SmimeInfo();
      smimeInfo.setPkcs12(Base64.getUrlEncoder().encodeToString(fileContent));
      if (password != null && password.length() > 0) {
        smimeInfo.setEncryptedKeyPassword(password);
      }
    } catch (Exception e) {
      System.out.printf("An error occured while reading the certificate file: %s\n", e);
    } finally {
      try {
        if (in != null) {
          in.close();
        }
      } catch (IOException ioe) {
        System.out.printf("An error occured while closing the input stream: %s\n", ioe);
      }
    }
    return smimeInfo;
  }
}
```

### Python: Create SmimeInfo Resource

```python
import base64


def create_smime_info(cert_filename, cert_password):
  """Create an smimeInfo resource for a certificate from file.
  Args:
    cert_filename: Name of the file containing the S/MIME certificate.
    cert_password: Password for the certificate file, or None if the file is not
        password-protected.
  Returns : Smime object, including smime information
  """

  smime_info = None
  try:
    with open(cert_filename, "rb") as cert:
      smime_info = {}
      data = cert.read().encode("UTF-8")
      smime_info["pkcs12"] = base64.urlsafe_b64encode(data).decode()
      if cert_password and len(cert_password) > 0:
        smime_info["encryptedKeyPassword"] = cert_password

  except (OSError, IOError) as error:
    print(f"An error occurred while reading the certificate file: {error}")
    smime_info = None

  return smime_info


if __name__ == "__main__":
  print(create_smime_info(cert_filename="xyz", cert_password="xyz"))
```

### Java: Upload Certificate

```java
import com.google.api.client.http.HttpRequestInitializer;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.services.gmail.Gmail;
import com.google.api.services.gmail.GmailScopes;
import com.google.api.services.gmail.model.SmimeInfo;
import com.google.auth.http.HttpCredentialsAdapter;
import com.google.auth.oauth2.GoogleCredentials;
import java.io.IOException;

/* Class to demonstrate the use of Gmail Insert Smime Certificate API*/
public class InsertSmimeInfo {
  /**
   * Upload an S/MIME certificate for the user.
   *
   * @param userId      User's email address.
   * @param sendAsEmail The "send as" email address, or null if it should be the same as userId.
   * @param smimeInfo   The SmimeInfo object containing the user's S/MIME certificate.
   * @return An SmimeInfo object with details about the uploaded certificate, {@code null} otherwise.
   * @throws IOException - if service account credentials file not found.
   */
  public static SmimeInfo insertSmimeInfo(String userId,
                                          String sendAsEmail,
                                          SmimeInfo smimeInfo)
      throws IOException {
        /* Load pre-authorized user credentials from the environment.
           TODO(developer) - See https://developers.google.com/identity for
            guides on implementing OAuth2 for your application. */
    GoogleCredentials credentials = GoogleCredentials.getApplicationDefault()
        .createScoped(GmailScopes.GMAIL_SETTINGS_SHARING);
    HttpRequestInitializer requestInitializer = new HttpCredentialsAdapter(
        credentials);

    // Create the gmail API client
    Gmail service = new Gmail.Builder(new NetHttpTransport(),
        GsonFactory.getDefaultInstance(),
        requestInitializer)
        .setApplicationName("Gmail samples")
        .build();

    if (sendAsEmail == null) {
      sendAsEmail = userId;
    }

    try {
      SmimeInfo results = service.users().settings().sendAs().smimeInfo()
          .insert(userId, sendAsEmail, smimeInfo)
          .execute();
      System.out.printf("Inserted certificate, id: %s\n", results.getId());
      return results;
    } catch (IOException e) {
      System.err.printf("An error occured: %s", e);
    }
    return null;
  }
}
```

### Python: Upload Certificate

```python
import create_smime_info
import google.auth
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError


def insert_smime_info():
  """Upload an S/MIME certificate for the user.
  Print the inserted certificate's id
  Returns : Result object with inserted certificate id and other meta-data

  Load pre-authorized user credentials from the environment.
  TODO(developer) - See https://developers.google.com/identity
  for guides on implementing OAuth2 for the application.
  """
  creds, _ = google.auth.default()

  try:
    # create gmail api client
    service = build("gmail", "v1", credentials=creds)

    user_id = "gduser1@workspacesamples.dev"
    smime_info = create_smime_info.create_smime_info(
        cert_filename="xyz", cert_password="xyz"
    )
    send_as_email = None

    if not send_as_email:
      send_as_email = user_id

    # pylint: disable=maybe-no-member
    results = (
        service.users()
        .settings()
        .sendAs()
        .smimeInfo()
        .insert(userId=user_id, sendAsEmail=send_as_email, body=smime_info)
        .execute()
    )
    print(f'Inserted certificate; id: {results["id"]}')

  except HttpError as error:
    print(f"An error occurred: {error}")
    results = None

  return results


if __name__ == "__main__":
  insert_smime_info()
```

## Bulk Operations: CSV Import

For managing certificates across multiple users, use a CSV file structure:

```bash
$ cat certificates.csv
user1@example.com,/path/to/user1_cert.p12,cert_password_1
user2@example.com,/path/to/user2_cert.p12,cert_password_2
user3@example.com,/path/to/user3_cert.p12,cert_password_3
```

### Java: Upload Certificates from CSV

```java
import com.google.api.services.gmail.model.SmimeInfo;
import java.io.File;
import org.apache.commons.csv.CSVFormat;
import org.apache.commons.csv.CSVParser;
import org.apache.commons.csv.CSVRecord;

/* Class to demonstrate the use of Gmail Insert Certificate from CSV File */
public class InsertCertFromCsv {
  /**
   * Upload S/MIME certificates based on the contents of a CSV file.
   *
   * <p>Each row of the CSV file should contain a user ID, path to the certificate, and the
   * certificate password.
   *
   * @param csvFilename Name of the CSV file.
   */
  public static void insertCertFromCsv(String csvFilename) {
    try {
      File csvFile = new File(csvFilename);
      CSVParser parser =
          CSVParser.parse(csvFile, java.nio.charset.StandardCharsets.UTF_8, CSVFormat.DEFAULT);
      for (CSVRecord record : parser) {
        String userId = record.get(0);
        String certFilename = record.get(1);
        String certPassword = record.get(2);
        SmimeInfo smimeInfo = CreateSmimeInfo.createSmimeInfo(certFilename,
            certPassword);
        if (smimeInfo != null) {
          InsertSmimeInfo.insertSmimeInfo(userId,
              userId,
              smimeInfo);
        } else {
          System.err.printf("Unable to read certificate file for userId: %s\n", userId);
        }
      }
    } catch (Exception e) {
      System.err.printf("An error occured while reading the CSV file: %s", e);
    }
  }
}
```

### Python: Upload Certificates from CSV

```python
import csv

import create_smime_info
import insert_smime_info


def insert_cert_from_csv(csv_filename):
  """Upload S/MIME certificates based on the contents of a CSV file.
  Each row of the CSV file should contain a user ID, path to the certificate,
  and the certificate password.

  Args:
    csv_filename: Name of the CSV file.
  """

  try:
    with open(csv_filename, "rb") as cert:
      csv_reader = csv.reader(cert, delimiter=",")
      next(csv_reader, None)  # skip CSV file header
      for row in csv_reader:
        user_id = row[0]
        cert_filename = row[1]
        cert_password = row[2]
        smime_info = create_smime_info.create_smime_info(
            cert_filename=cert_filename, cert_password=cert_password
        )
        if smime_info:
          insert_smime_info.insert_smime_info()
        else:
          print(f"Unable to read certificate file for user_id: {user_id}")
        return smime_info

  except (OSError, IOError) as error:
    print(f"An error occured while reading the CSV file: {error}")


if __name__ == "__main__":
  insert_cert_from_csv(csv_filename="xyz")
```

## Update Default Certificate

Examples show how to list certificates, choose the best candidate, and set the default certificate.

### Java: Update Default Certificate

```java
import com.google.api.client.http.HttpRequestInitializer;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.services.gmail.Gmail;
import com.google.api.services.gmail.GmailScopes;
import com.google.api.services.gmail.model.ListSmimeInfoResponse;
import com.google.api.services.gmail.model.SmimeInfo;
import com.google.auth.http.HttpCredentialsAdapter;
import com.google.auth.oauth2.GoogleCredentials;
import java.io.IOException;
import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneId;

/* Class to demonstrate the use of Gmail Update Smime Certificate API*/
public class UpdateSmimeCerts {
  /**
   * Update S/MIME certificates for the user.
   *
   * <p>First performs a lookup of all certificates for a user. If there are no certificates, or
   * they all expire before the specified date/time, uploads the certificate in the specified file.
   * If the default certificate is expired or there was no default set, chooses the certificate with
   * the expiration furthest into the future and sets it as default.
   *
   * @param userId       User's email address.
   * @param sendAsEmail  The "send as" email address, or None if it should be the same as user_id.
   * @param certFilename Name of the file containing the S/MIME certificate.
   * @param certPassword Password for the certificate file, or None if the file is not
   *                     password-protected.
   * @param expireTime   DateTime object against which the certificate expiration is compared. If
   *                     None, uses the current time. @ returns: The ID of the default certificate.
   * @return The ID of the default certificate, {@code null} otherwise.
   * @throws IOException - if service account credentials file not found.
   */
  public static String updateSmimeCerts(String userId,
                                        String sendAsEmail,
                                        String certFilename,
                                        String certPassword,
                                        LocalDateTime expireTime)
      throws IOException {
        /* Load pre-authorized user credentials from the environment.
           TODO(developer) - See https://developers.google.com/identity for
            guides on implementing OAuth2 for your application. */
    GoogleCredentials credentials = GoogleCredentials.getApplicationDefault()
        .createScoped(GmailScopes.GMAIL_SETTINGS_SHARING);
    HttpRequestInitializer requestInitializer = new HttpCredentialsAdapter(
        credentials);

    // Create the gmail API client
    Gmail service = new Gmail.Builder(new NetHttpTransport(),
        GsonFactory.getDefaultInstance(),
        requestInitializer)
        .setApplicationName("Gmail samples")
        .build();

    if (sendAsEmail == null) {
      sendAsEmail = userId;
    }

    ListSmimeInfoResponse listResults;
    try {
      listResults =
          service.users().settings().sendAs().smimeInfo().list(userId, sendAsEmail).execute();
    } catch (IOException e) {
      System.err.printf("An error occurred during list: %s\n", e);
      return null;
    }

    String defaultCertId = null;
    String bestCertId = null;
    LocalDateTime bestCertExpire = LocalDateTime.MIN;

    if (expireTime == null) {
      expireTime = LocalDateTime.now();
    }
    if (listResults != null && listResults.getSmimeInfo() != null) {
      for (SmimeInfo smimeInfo : listResults.getSmimeInfo()) {
        String certId = smimeInfo.getId();
        boolean isDefaultCert = smimeInfo.getIsDefault();
        if (isDefaultCert) {
          defaultCertId = certId;
        }
        LocalDateTime exp =
            LocalDateTime.ofInstant(
                Instant.ofEpochMilli(smimeInfo.getExpiration()), ZoneId.systemDefault());
        if (exp.isAfter(expireTime)) {
          if (exp.isAfter(bestCertExpire)) {
            bestCertId = certId;
            bestCertExpire = exp;
          }
        } else {
          if (isDefaultCert) {
            defaultCertId = null;
          }
        }
      }
    }
    if (defaultCertId == null) {
      String defaultId = bestCertId;
      if (defaultId == null && certFilename != null) {
        SmimeInfo smimeInfo = CreateSmimeInfo.createSmimeInfo(certFilename,
            certPassword);
        SmimeInfo insertResults = InsertSmimeInfo.insertSmimeInfo(userId,
            sendAsEmail,
            smimeInfo);
        if (insertResults != null) {
          defaultId = insertResults.getId();
        }
      }

      if (defaultId != null) {
        try {
          service.users().settings().sendAs().smimeInfo().setDefault(userId, sendAsEmail, defaultId)
              .execute();
          return defaultId;
        } catch (IOException e) {
          System.err.printf("An error occured during setDefault: %s", e);
        }
      }
    } else {
      return defaultCertId;
    }

    return null;
  }
}
```

### Python: Update Default Certificate

```python
from datetime import datetime

import create_smime_info
import google.auth
import insert_smime_info
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError


def update_smime_cert(
    user_id, send_as_email, cert_filename, cert_password, expire_dt
):
  """Update S/MIME certificates for the user.

  First performs a lookup of all certificates for a user.  If there are no
  certificates, or they all expire before the specified date/time, uploads the
  certificate in the specified file.  If the default certificate is expired or
  there was no default set, chooses the certificate with the expiration furthest
  into the future and sets it as default.

  Args:
    user_id: User's email address.
    send_as_email: The "send as" email address, or None if it should be the same
        as user_id.
    cert_filename: Name of the file containing the S/MIME certificate.
    cert_password: Password for the certificate file, or None if the file is not
        password-protected.
    expire_dt: DateTime object against which the certificate expiration is
      compared.  If None, uses the current time.

  Returns:
    The ID of the default certificate.

  Load pre-authorized user credentials from the environment.
  TODO(developer) - See https://developers.google.com/identity
  for guides on implementing OAuth2 for the application.
  """
  if not send_as_email:
    send_as_email = user_id

  creds, _ = google.auth.default()

  try:
    # create gmail api client
    service = build("gmail", "v1", credentials=creds)

    # pylint: disable=maybe-no-member
    results = (
        service.users()
        .settings()
        .sendAs()
        .smimeInfo()
        .list(userId=user_id, sendAsEmail=send_as_email)
        .execute()
    )

  except HttpError as error:
    print(f"An error occurred during list: {error}")
    return None

  default_cert_id = None
  best_cert_id = (None, datetime.datetime.fromtimestamp(0))

  if not expire_dt:
    expire_dt = datetime.datetime.now()
  if results and "smimeInfo" in results:
    for smime_info in results["smimeInfo"]:
      cert_id = smime_info["id"]
      is_default_cert = smime_info["isDefault"]
      if is_default_cert:
        default_cert_id = cert_id
      exp = datetime.datetime.fromtimestamp(smime_info["expiration"] / 1000)
      if exp > expire_dt:
        if exp > best_cert_id[1]:
          best_cert_id = (cert_id, exp)
      else:
        if is_default_cert:
          default_cert_id = None

  if not default_cert_id:
    default_id = best_cert_id[0]
    if not default_id and cert_filename:
      create_smime_info.create_smime_info(
          cert_filename=cert_filename, cert_password=cert_password
      )
      results = insert_smime_info.insert_smime_info()
      if results:
        default_id = results["id"]

    if default_id:
      try:
        # pylint: disable=maybe-no-member
        service.users().settings().sendAs().smimeInfo().setDefault(
            userId=user_id, sendAsEmail=send_as_email, id=default_id
        ).execute()
        return default_id
      except HttpError as error:
        print(f"An error occurred during setDefault: {error}")
  else:
    return default_cert_id

  return None


if __name__ == "__main__":
  update_smime_cert(
      user_id="xyz",
      send_as_email=None,
      cert_filename="xyz",
      cert_password="xyz",
      expire_dt=None,
  )
```
