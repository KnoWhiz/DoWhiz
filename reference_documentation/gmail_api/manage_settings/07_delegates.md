# Managing Delegates

## Overview

Gmail users within a Google Workspace organization can authorize another user to access their mailbox. The person granting permission is the **delegator**, while the person receiving access is the **delegate**.

## Delegate Permissions

Delegates have the following capabilities for the delegator's account:

- Read messages
- Send messages
- Delete messages
- View contacts
- Add contacts

## API Management

Google Workspace organizations can programmatically manage delegates using the `Delegates` API resource. This functionality requires:

- A **service account** with domain-wide delegation authority
- Appropriate permissions to manage delegate settings

## Available Operations

The Delegates API reference provides methods to:

- **Create** new delegates for user accounts
- **List** existing delegates
- **Get** specific delegate information
- **Delete** delegate access

## Requirements

Implementation requires leveraging OAuth2 service account authentication with delegated domain-wide authority enabled on the service account.
