# Java Quickstart for Gmail Postmaster Tools API

## Overview

This guide explains how to create a Java command-line application that makes requests to the Postmaster Tools API. The quickstart uses a simplified authentication approach suitable for testing environments.

## Prerequisites

- Java 11 or greater
- Gradle 7.0 or greater
- A Google Cloud project
- A Google account with Gmail enabled

## Setup Instructions

### 1. Enable the API

Navigate to the Google Cloud console and enable the Postmaster Tools API through the provided link.

### 2. Configure OAuth Consent Screen

If this is your first time setting up the project:

1. Go to **Google Auth platform > Branding** in Google Cloud console
2. Click **Get Started** if prompted
3. Enter an app name under **App Information**
4. Provide a user support email address
5. Under **Audience**, select **Internal**
6. Add contact information for notifications
7. Review and accept the "Google API Services User Data Policy"
8. Complete the setup by clicking **Create**

### 3. Create Desktop Application Credentials

1. Navigate to **Google Auth platform > Clients**
2. Select **Create Client > Desktop app**
3. Name your credential
4. Download the JSON file as `credentials.json`
5. Move it to your working directory

## Workspace Preparation

### Initialize Project Structure

```bash
gradle init --type basic
mkdir -p src/main/java src/main/resources
```

### Copy Credentials

Place `credentials.json` in `src/main/resources/`

### Update build.gradle

Replace the default `build.gradle` with:

```gradle
apply plugin: 'java'
apply plugin: 'application'

mainClassName = 'GmailPostmasterToolsQuickstart'
sourceCompatibility = 1.8
targetCompatibility = 1.8
version = '1.0'

repositories {
    mavenCentral()
}

dependencies {
    implementation 'com.google.api-client:google-api-client:1.23.0'
    implementation 'com.google.oauth-client:google-oauth-client-jetty:1.23.0'
    implementation 'com.google.apis:google-api-services-gmailpostmastertools:v1-rev20210528-1.31.0'
}
```

## Sample Implementation

Create `src/main/java/PostmasterToolsApiQuickStart.java`:

```java
import com.google.api.client.auth.oauth2.Credential;
import com.google.api.client.extensions.java6.auth.oauth2.AuthorizationCodeInstalledApp;
import com.google.api.client.extensions.jetty.auth.oauth2.LocalServerReceiver;
import com.google.api.client.googleapis.auth.oauth2.GoogleAuthorizationCodeFlow;
import com.google.api.client.googleapis.auth.oauth2.GoogleClientSecrets;
import com.google.api.client.googleapis.javanet.GoogleNetHttpTransport;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.JsonFactory;
import com.google.api.client.json.jackson2.JacksonFactory;
import com.google.api.client.util.store.FileDataStoreFactory;
import com.google.api.services.gmailpostmastertools.v1beta1.PostmasterTools;
import com.google.api.services.gmailpostmastertools.v1beta1.model.*;

import java.io.*;
import java.security.GeneralSecurityException;
import java.util.Collections;
import java.util.List;

public class PostmasterToolsApiQuickStart {
    private static final String APPLICATION_NAME = "PostmasterTools API Java Quickstart";
    private static final JsonFactory JSON_FACTORY = JacksonFactory.getDefaultInstance();
    private static final String TOKENS_DIRECTORY_PATH = "tokens";
    private static final List<String> SCOPES = Collections.singletonList(
        "https://www.googleapis.com/auth/postmaster.readonly");
    private static final String CREDENTIALS_FILE_PATH = "/credentials.json";

    private static Credential getCredentials(final NetHttpTransport HTTP_TRANSPORT)
            throws IOException {
        InputStream in = PostmasterToolsApiQuickStart.class
            .getResourceAsStream(CREDENTIALS_FILE_PATH);
        if (in == null) {
            throw new FileNotFoundException("Resource not found: " + CREDENTIALS_FILE_PATH);
        }
        GoogleClientSecrets clientSecrets = GoogleClientSecrets.load(JSON_FACTORY,
            new InputStreamReader(in));

        GoogleAuthorizationCodeFlow flow =
            new GoogleAuthorizationCodeFlow.Builder(
                HTTP_TRANSPORT, JSON_FACTORY, clientSecrets, SCOPES)
            .setDataStoreFactory(new FileDataStoreFactory(
                new java.io.File(TOKENS_DIRECTORY_PATH)))
            .setAccessType("offline")
            .build();
        LocalServerReceiver receiver = new LocalServerReceiver.Builder()
            .setPort(8891).build();
        return new AuthorizationCodeInstalledApp(flow, receiver).authorize("user");
    }

    public static void main(String... args) throws IOException, GeneralSecurityException {
        final NetHttpTransport HTTP_TRANSPORT = GoogleNetHttpTransport.newTrustedTransport();
        PostmasterTools postmasterTools = new PostmasterTools.Builder(
            HTTP_TRANSPORT, JSON_FACTORY, getCredentials(HTTP_TRANSPORT))
            .setApplicationName(APPLICATION_NAME)
            .build();

        List<Domain> domains = postmasterTools.domains().list()
            .execute().getDomains();
        if (domains == null || domains.isEmpty()) {
            System.out.println("No domains found!");
        } else {
            for (Domain domain : domains) {
                System.out.println(domain.toPrettyString());
            }
        }
    }
}
```

## Running the Application

Execute the following command:

```bash
gradle run
```

On first run, the application prompts for authorization. Sign in with your Google account and grant access permissions. Authorization credentials are stored locally, eliminating the need for repeated authorization.

## Next Steps

- Explore APIs using the APIs Explorer tool
- Consult troubleshooting guides for authentication issues
- Review the REST reference documentation
- Access Java client library documentation
