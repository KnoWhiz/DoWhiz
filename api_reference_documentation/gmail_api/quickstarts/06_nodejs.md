# Node.js Quickstart for Gmail API

## Overview

This guide demonstrates how to build a Node.js command-line application that interacts with the Gmail API. It emphasizes that this simplified authentication approach suits testing environments only; production deployments require proper credential selection.

## Objectives

- Configure your development environment
- Install required client libraries
- Prepare the sample application
- Execute the sample code

## Prerequisites

Before starting, ensure you have:

- **Node.js & npm** installed (see npm documentation for installation steps)
- **A Google Cloud project** already created
- **A Google account** with Gmail enabled

## Environment Setup

### Enable the Gmail API

Access the Google Cloud console and activate the Gmail API for your project using the provided enable link.

### Configure OAuth Consent Screen

If this is a new Cloud project:

1. Navigate to **Google Auth platform** > **Branding** in the console
2. If needed, click **Get Started** to initialize configuration
3. Enter app information:
   - **App name**: Choose a descriptive name
   - **User support email**: Provide a contact address
4. Set audience to **Internal** and proceed
5. Supply contact information for notifications
6. Review and accept the Google API Services User Data Policy
7. Complete the setup by clicking **Create**

For now, skip scope configuration. External deployments require changing user type and adding authorization scopes.

### Create Desktop Application Credentials

Generate OAuth 2.0 credentials for desktop applications:

1. Go to **Google Auth platform** > **Clients**
2. Click **Create Client**
3. Select **Desktop app** as the application type
4. Name your credential (visible only in console)
5. Click **Create**
6. Download the JSON file and rename it to `credentials.json`
7. Move this file to your project directory

**Note**: This quickstart requires local execution with browser access and won't function on remote terminals like Cloud Shell.

## Install Client Libraries

Execute this command in your project directory:

```bash
npm install googleapis@105 @google-cloud/local-auth@2.1.0 --save
```

## Sample Application Setup

### Create index.js

In your working directory, create a file named `index.js` with the following code:

```javascript
import path from 'node:path';
import process from 'node:process';
import {authenticate} from '@google-cloud/local-auth';
import {google} from 'googleapis';

// The scope for reading Gmail labels.
const SCOPES = ['https://www.googleapis.com/auth/gmail.readonly'];
// The path to the credentials file.
const CREDENTIALS_PATH = path.join(process.cwd(), 'credentials.json');

/**
 * Lists the labels in the user's account.
 */
async function listLabels() {
  // Authenticate with Google and get an authorized client.
  const auth = await authenticate({
    scopes: SCOPES,
    keyfilePath: CREDENTIALS_PATH,
  });

  // Create a new Gmail API client.
  const gmail = google.gmail({version: 'v1', auth});
  // Get the list of labels.
  const result = await gmail.users.labels.list({
    userId: 'me',
  });
  const labels = result.data.labels;
  if (!labels || labels.length === 0) {
    console.log('No labels found.');
    return;
  }
  console.log('Labels:');
  // Print the name of each label.
  labels.forEach((label) => {
    console.log(`- ${label.name}`);
  });
}

await listLabels();
```

## Run the Application

### Execute the Sample

In your project directory, run:

```bash
node .
```

### Authorization Flow

On first execution, the application prompts for authorization:

1. Sign in to your Google Account (select one if multiple accounts exist)
2. Click **Accept** on the consent screen
3. The application executes and calls the Gmail API
4. Authorization credentials are cached locally, eliminating future prompts

## Next Steps

Explore additional resources:

- Try APIs using the "APIs explorer" feature
- Review "Gmail API reference documentation" for detailed endpoint information
- Consult the google-api-nodejs-client GitHub section for library details
- Troubleshoot authentication issues using dedicated guides
