# Java Quickstart for Gmail API

## Overview

This guide demonstrates how to create a Java command-line application that interacts with the Gmail API. The approach uses simplified authentication suitable for testing environments.

## Objectives

- Configure your development environment
- Prepare the sample application
- Execute the sample code

## Prerequisites

Required software and accounts:
- Java 11 or later
- Gradle 7.0 or later
- Google Cloud project
- Gmail-enabled Google account

## Environment Setup

### Enable Gmail API

Access the Google Cloud console and activate the Gmail API through the provided link.

### Configure OAuth Consent Screen

1. Navigate to Google Cloud console > Google Auth platform > Branding
2. If not configured, click "Get Started"
3. Enter application name and support email
4. Set audience type to "Internal"
5. Provide contact email for notifications
6. Accept the Google API Services User Data Policy
7. Complete the setup

### Create Desktop Application Credentials

1. Go to Google Cloud console > Google Auth platform > Clients
2. Create a new client for "Desktop app" type
3. Assign a descriptive name
4. Download the JSON credentials file
5. Save as `credentials.json` in your working directory

## Project Structure Setup

Create project structure:
```bash
gradle init --type basic
mkdir -p src/main/java src/main/resources
```

Place `credentials.json` in `src/main/resources/`

## Build Configuration

Create `build.gradle`:

```gradle
apply plugin: 'java'
apply plugin: 'application'

mainClassName = 'GmailQuickstart'
sourceCompatibility = 11
targetCompatibility = 11
version = '1.0'

repositories {
    mavenCentral()
}

dependencies {
    implementation 'com.google.api-client:google-api-client:2.0.0'
    implementation 'com.google.oauth-client:google-oauth-client-jetty:1.34.1'
    implementation 'com.google.apis:google-api-services-gmail:v1-rev20220404-2.0.0'
}
```

## Sample Application Code

Create `src/main/java/GmailQuickstart.java`:

```java
import com.google.api.client.auth.oauth2.Credential;
import com.google.api.client.extensions.java6.auth.oauth2.AuthorizationCodeInstalledApp;
import com.google.api.client.extensions.jetty.auth.oauth2.LocalServerReceiver;
import com.google.api.client.googleapis.auth.oauth2.GoogleAuthorizationCodeFlow;
import com.google.api.client.googleapis.auth.oauth2.GoogleClientSecrets;
import com.google.api.client.googleapis.javanet.GoogleNetHttpTransport;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.JsonFactory;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.client.util.store.FileDataStoreFactory;
import com.google.api.services.gmail.Gmail;
import com.google.api.services.gmail.GmailScopes;
import com.google.api.services.gmail.model.Label;
import com.google.api.services.gmail.model.ListLabelsResponse;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.security.GeneralSecurityException;
import java.util.Collections;
import java.util.List;

public class GmailQuickstart {
  private static final String APPLICATION_NAME = "Gmail API Java Quickstart";
  private static final JsonFactory JSON_FACTORY = GsonFactory.getDefaultInstance();
  private static final String TOKENS_DIRECTORY_PATH = "tokens";
  private static final List<String> SCOPES = Collections.singletonList(GmailScopes.GMAIL_LABELS);
  private static final String CREDENTIALS_FILE_PATH = "/credentials.json";

  private static Credential getCredentials(final NetHttpTransport HTTP_TRANSPORT)
      throws IOException {
    InputStream in = GmailQuickstart.class.getResourceAsStream(CREDENTIALS_FILE_PATH);
    if (in == null) {
      throw new FileNotFoundException("Resource not found: " + CREDENTIALS_FILE_PATH);
    }
    GoogleClientSecrets clientSecrets =
        GoogleClientSecrets.load(JSON_FACTORY, new InputStreamReader(in));

    GoogleAuthorizationCodeFlow flow = new GoogleAuthorizationCodeFlow.Builder(
        HTTP_TRANSPORT, JSON_FACTORY, clientSecrets, SCOPES)
        .setDataStoreFactory(new FileDataStoreFactory(new java.io.File(TOKENS_DIRECTORY_PATH)))
        .setAccessType("offline")
        .build();
    LocalServerReceiver receiver = new LocalServerReceiver.Builder().setPort(8888).build();
    Credential credential = new AuthorizationCodeInstalledApp(flow, receiver).authorize("user");
    return credential;
  }

  public static void main(String... args) throws IOException, GeneralSecurityException {
    final NetHttpTransport HTTP_TRANSPORT = GoogleNetHttpTransport.newTrustedTransport();
    Gmail service = new Gmail.Builder(HTTP_TRANSPORT, JSON_FACTORY, getCredentials(HTTP_TRANSPORT))
        .setApplicationName(APPLICATION_NAME)
        .build();

    String user = "me";
    ListLabelsResponse listResponse = service.users().labels().list(user).execute();
    List<Label> labels = listResponse.getLabels();
    if (labels.isEmpty()) {
      System.out.println("No labels found.");
    } else {
      System.out.println("Labels:");
      for (Label label : labels) {
        System.out.printf("- %s\n", label.getName());
      }
    }
  }
}
```

## Running the Application

Execute with Gradle:
```bash
gradle run
```

On first execution:
1. A browser window opens for Google Account authorization
2. Select your account if using multiple accounts
3. Click "Accept" to grant permissions
4. The application runs and displays your Gmail labels

Subsequent executions use stored authorization tokens without prompting.

## Next Steps

- Explore APIs in the API Explorer
- Review troubleshooting guides for authentication issues
- Consult Gmail API reference documentation
- Study Google APIs Client for Java documentation
