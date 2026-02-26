# Python Quickstart for Gmail Postmaster Tools API

## Overview

This guide demonstrates how to build a Python command-line application that interfaces with the Postmaster Tools API.

## Objectives

- Set up your development environment
- Install the required client library
- Configure the sample application
- Execute the sample code

## Prerequisites

Before beginning, ensure you have:
- Python 3.10.7 or higher installed
- The `pip` package manager
- A Google Cloud project
- A Google account with Gmail enabled

## Environment Setup

### Enable the API

Navigate to the Google Cloud console and enable the Postmaster Tools API through the provided link.

### Configure OAuth Consent Screen

1. Access the Google Cloud console's Branding section
2. If not previously configured, click "Get Started"
3. Enter an app name under App Information
4. Specify a user support email address
5. Select "Internal" for the Audience setting
6. Provide contact information for project notifications
7. Accept the Google API Services User Data Policy
8. Complete the configuration by clicking "Create"

### Create Desktop Application Credentials

1. Navigate to the Clients section in Google Cloud console
2. Select "Create Client" and choose "Desktop app"
3. Name the credential appropriately
4. Save the downloaded JSON file as `credentials.json`, and move the file to your working directory

## Installation

Execute the following command:
```
pip install --upgrade google-api-python-client google-auth-httplib2 google-auth-oauthlib
```

## Sample Configuration

Create a file named `quickstart.py` with this code:

```python
from __future__ import print_function
import pickle
import os.path
from googleapiclient.discovery import build
from google_auth_oauthlib.flow import InstalledAppFlow
from google.auth.transport.requests import Request
from googleapiclient import errors

# If modifying these scopes, delete the file token.pickle.
SCOPES = ['https://www.googleapis.com/auth/postmaster.readonly']

def main():
    """Shows basic usage of the PostmasterTools v1beta1 API.
    Prints the visible domains on user's domain dashboard in https://postmaster.google.com/managedomains.
    """
    creds = None
    # The file token.pickle stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists('token.pickle'):
        with open('token.pickle', 'rb') as token:
            creds = pickle.load(token)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                'credentials.json', SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open('token.pickle', 'wb') as token:
            pickle.dump(creds, token)

    service = build('gmailpostmastertools', 'v1beta1', credentials=creds)

    domains = service.domains().list().execute()
    if not domains:
        print('No domains found.')
    else:
        print('Domains:')
        for domain in domains['domains']:
            print(domain)

if __name__ == '__main__':
  main()
```

## Running the Application

Execute: `python3 quickstart.py`

Upon first execution, you'll be prompted to authorize access. Sign in with your Google Account and grant permission. Subsequent runs will use stored authorization tokens.

## Next Steps

- Explore the Google Workspace APIs in the APIs explorer
- Review the Postmaster Tools API REST reference documentation
- Consult the Google APIs Client for Python documentation
