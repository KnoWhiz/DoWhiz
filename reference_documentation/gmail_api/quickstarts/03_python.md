# Python Quickstart for Gmail API

## Overview

This guide demonstrates how to create a Python command-line application that interfaces with the Gmail API. It uses simplified authentication suitable for testing environments, though production deployments require more robust security measures.

## Objectives

- Configure your development environment
- Install necessary client libraries
- Set up the sample application
- Execute the sample code

## Prerequisites

Before starting, ensure you have:

- Python 3.10.7 or later installed
- The pip package management tool
- A Google Cloud project
- A Gmail-enabled Google account

## Environment Setup

### Enable the Gmail API

Navigate to the Google Cloud Console and activate the Gmail API for your project using the provided enablement link.

### Configure OAuth Consent Screen

For new projects, configure the OAuth consent screen:

1. Access the Google Cloud Console's Auth platform Branding section
2. Enter application name and support email
3. Select "Internal" for audience type
4. Provide contact information for notifications
5. Accept the Google API Services User Data Policy
6. Complete the setup process

For external applications, change the user type accordingly and add required authorization scopes.

### Create OAuth 2.0 Credentials

1. Navigate to the Clients section in Google Cloud Console
2. Create a new Desktop app credential
3. Save the downloaded JSON file as `credentials.json` in your working directory

## Install Google Client Library

Execute the following command:

```bash
pip install --upgrade google-api-python-client google-auth-httplib2 google-auth-oauthlib
```

## Sample Configuration

### Create quickstart.py

Create a file named `quickstart.py` with the following code:

```python
import os.path

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

# If modifying these scopes, delete the file token.json.
SCOPES = ["https://www.googleapis.com/auth/gmail.readonly"]

def main():
  """Shows basic usage of the Gmail API.
  Lists the user's Gmail labels.
  """
  creds = None
  # The file token.json stores the user's access and refresh tokens, and is
  # created automatically when the authorization flow completes for the first
  # time.
  if os.path.exists("token.json"):
    creds = Credentials.from_authorized_user_file("token.json", SCOPES)
  # If there are no (valid) credentials available, let the user log in.
  if not creds or not creds.valid:
    if creds and creds.expired and creds.refresh_token:
      creds.refresh(Request())
    else:
      flow = InstalledAppFlow.from_client_secrets_file(
          "credentials.json", SCOPES
      )
      creds = flow.run_local_server(port=0)
    # Save the credentials for the next run
    with open("token.json", "w") as token:
      token.write(creds.to_json())

  try:
    # Call the Gmail API
    service = build("gmail", "v1", credentials=creds)
    results = service.users().labels().list(userId="me").execute()
    labels = results.get("labels", [])

    if not labels:
      print("No labels found.")
      return
    print("Labels:")
    for label in labels:
      print(label["name"])

  except HttpError as error:
    # TODO(developer) - Handle errors from gmail API.
    print(f"An error occurred: {error}")

if __name__ == "__main__":
  main()
```

## Running the Application

Execute the sample:

```bash
python3 quickstart.py
```

On first run, the application prompts for authorization. Sign in with your Google account and grant access permissions. The authorization details are cached for subsequent executions.

## Next Steps

- Explore APIs using the APIs Explorer tool
- Review Gmail API reference documentation
- Consult Python client library documentation
- Troubleshoot authentication issues with provided guides
