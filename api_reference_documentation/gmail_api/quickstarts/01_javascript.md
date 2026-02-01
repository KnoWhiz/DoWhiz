# JavaScript Quickstart for Gmail API

## Overview

This guide demonstrates how to create a JavaScript web application that interfaces with the Gmail API. The quickstart uses simplified authentication suitable for testing environments.

## Objectives

- Configure your development environment
- Set up the sample application
- Execute and test the sample code

## Prerequisites

Required tools and accounts:
- Node.js and npm installed
- A Google Cloud project configured
- A Google account with Gmail enabled

## Environment Setup

### Enable Gmail API

Access the Google Cloud console and activate the Gmail API through the enablement flow.

### Configure OAuth Consent Screen

If this is your first time using the Cloud project, configure the consent screen:

1. Navigate to Google Auth platform settings in the Cloud console
2. Complete app information with a name for your application
3. Provide a support email for user inquiries
4. Set audience as "Internal" for testing
5. Enter contact information for notifications
6. Review and accept the Google API Services User Data Policy
7. Finalize the configuration

### Create OAuth 2.0 Credentials

Set up credentials for web application authentication:

1. Generate a new OAuth 2.0 Client ID
2. Select "Web application" as the type
3. Register authorized JavaScript origins for browser requests
4. Record the Client ID for later use

### Generate API Key

Obtain an API key from the credentials section and note it for implementation.

## Sample Implementation

Create an `index.html` file containing the complete JavaScript application. The code includes:

- Authorization buttons for user sign-in and sign-out
- Integration with Google's authentication libraries
- Gmail API client initialization
- Label listing functionality

Replace placeholder values (`YOUR_CLIENT_ID` and `YOUR_API_KEY`) with actual credentials.

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Gmail API Quickstart</title>
    <meta charset="utf-8" />
  </head>
  <body>
    <p>Gmail API Quickstart</p>

    <!--Add buttons to initiate auth sequence and sign out-->
    <button id="authorize_button" onclick="handleAuthClick()">Authorize</button>
    <button id="signout_button" onclick="handleSignoutClick()">Sign Out</button>

    <pre id="content" style="white-space: pre-wrap;"></pre>

    <script type="text/javascript">
      /* exported gapiLoaded */
      /* exported gisLoaded */
      /* exported handleAuthClick */
      /* exported handleSignoutClick */

      // TODO(developer): Set to client ID and API key from the Developer Console
      const CLIENT_ID = '<YOUR_CLIENT_ID>';
      const API_KEY = '<YOUR_API_KEY>';

      // Discovery doc URL for APIs used by the quickstart
      const DISCOVERY_DOC = 'https://www.googleapis.com/discovery/v1/apis/gmail/v1/rest';

      // Authorization scopes required by the API; multiple scopes can be
      // included, separated by spaces.
      const SCOPES = 'https://www.googleapis.com/auth/gmail.readonly';

      let tokenClient;
      let gapiInited = false;
      let gisInited = false;

      document.getElementById('authorize_button').style.visibility = 'hidden';
      document.getElementById('signout_button').style.visibility = 'hidden';

      /**
       * Callback after api.js is loaded.
       */
      function gapiLoaded() {
        gapi.load('client', initializeGapiClient);
      }

      /**
       * Callback after the API client is loaded. Loads the
       * discovery doc to initialize the API.
       */
      async function initializeGapiClient() {
        await gapi.client.init({
          apiKey: API_KEY,
          discoveryDocs: [DISCOVERY_DOC],
        });
        gapiInited = true;
        maybeEnableButtons();
      }

      /**
       * Callback after Google Identity Services are loaded.
       */
      function gisLoaded() {
        tokenClient = google.accounts.oauth2.initTokenClient({
          client_id: CLIENT_ID,
          scope: SCOPES,
          callback: '', // defined later
        });
        gisInited = true;
        maybeEnableButtons();
      }

      /**
       * Enables user interaction after all libraries are loaded.
       */
      function maybeEnableButtons() {
        if (gapiInited && gisInited) {
          document.getElementById('authorize_button').style.visibility = 'visible';
        }
      }

      /**
       *  Sign in the user upon button click.
       */
      function handleAuthClick() {
        tokenClient.callback = async (resp) => {
          if (resp.error !== undefined) {
            throw (resp);
          }
          document.getElementById('signout_button').style.visibility = 'visible';
          document.getElementById('authorize_button').innerText = 'Refresh';
          await listLabels();
        };

        if (gapi.client.getToken() === null) {
          // Prompt the user to select a Google Account and ask for consent to share their data
          // when establishing a new session.
          tokenClient.requestAccessToken({prompt: 'consent'});
        } else {
          // Skip display of account chooser and consent dialog for an existing session.
          tokenClient.requestAccessToken({prompt: ''});
        }
      }

      /**
       *  Sign out the user upon button click.
       */
      function handleSignoutClick() {
        const token = gapi.client.getToken();
        if (token !== null) {
          google.accounts.oauth2.revoke(token.access_token);
          gapi.client.setToken('');
          document.getElementById('content').innerText = '';
          document.getElementById('authorize_button').innerText = 'Authorize';
          document.getElementById('signout_button').style.visibility = 'hidden';
        }
      }

      /**
       * Print all Labels in the authorized user's inbox. If no labels
       * are found an appropriate message is printed.
       */
      async function listLabels() {
        let response;
        try {
          response = await gapi.client.gmail.users.labels.list({
            'userId': 'me',
          });
        } catch (err) {
          document.getElementById('content').innerText = err.message;
          return;
        }
        const labels = response.result.labels;
        if (!labels || labels.length == 0) {
          document.getElementById('content').innerText = 'No labels found.';
          return;
        }
        // Flatten to string to display
        const output = labels.reduce(
            (str, label) => `${str}${label.name}\n`,
            'Labels:\n');
        document.getElementById('content').innerText = output;
      }
    </script>
    <script async defer src="https://apis.google.com/js/api.js" onload="gapiLoaded()"></script>
    <script async defer src="https://accounts.google.com/gsi/client" onload="gisLoaded()"></script>
  </body>
</html>
```

## Running the Application

1. Install the http-server package via npm
2. Start a local web server on port 8000
3. Navigate to `http://localhost:8000` in your browser
4. Click the authorization button
5. Complete the Google account authentication flow
6. Accept permission requests

```bash
npm install http-server
```

```bash
npx http-server -p 8000
```

The application will then display Gmail labels from your account.

## Next Steps

- Explore additional Gmail API methods in the API reference
- Review authentication troubleshooting guides
- Examine sample implementations for expanded functionality
