# Google Apps Script Quickstart for Gmail API

## Overview

This guide teaches how to build a Google Apps Script application that communicates with the Gmail API. Quickstarts explain how to set up and run an app that calls a Google Workspace API.

## Objectives

- Configure the environment
- Create and configure the script
- Run the script

## Prerequisites

- A Google account with Gmail enabled
- Access to Google Drive

## Creating the Script

### Step 1: Create a New Script

Navigate to [script.google.com/create](https://script.google.com/create) to open the Apps Script editor.

### Step 2: Add the Code

Replace the editor contents with the following implementation:

```javascript
/**
 * Lists all labels in the user's mailbox
 * @see https://developers.google.com/gmail/api/reference/rest/v1/users.labels/list
 */
function listLabels() {
  try {
    const response = Gmail.Users.Labels.list("me");
    if (!response || response.labels.length === 0) {
      console.log("No labels found.");
      return;
    }
    console.log("Labels:");
    for (const label of response.labels) {
      console.log("- %s", label.name);
    }
  } catch (err) {
    console.log("Labels.list() API failed with error %s", err.toString());
  }
}
```

### Step 3: Save and Rename

Save the project, then rename it from "Untitled project" to "Quickstart."

## Configuration

### Enable Gmail API

1. Click the **Editor** code section
2. Next to **Services**, select the add option
3. Choose **Gmail API** and confirm addition

## Running the Application

1. Click **Run** in the Apps Script editor
2. On first execution, authorize access by clicking **Review permissions**
3. Select your account and grant permission
4. Monitor the execution log at the window's bottom

## Next Steps

- Explore Google Apps Script Advanced Services documentation for deeper integration
- Test additional endpoints via the APIs explorer
- Consult troubleshooting resources for authentication and authorization issues
- Review the Gmail API reference documentation
