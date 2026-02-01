# Choose Gmail API Scopes

## Overview

This document provides Gmail API-specific authorization and authentication guidance. Developers should first review the general Google Workspace authentication and authorization documentation before proceeding.

## OAuth 2.0 Configuration

Applications must "Configure the OAuth consent screen and choose scopes" to specify what user information is displayed and register applications for publication.

## Authorization Scopes Explained

An authorization scope is an OAuth 2.0 URI that specifies which Google Workspace app, data type, and access level an application requires. Developers should choose the most narrowly focused scope possible and avoid requesting scopes that your app doesn't require. Users are more likely to grant permissions for limited, clearly described scopes.

Public applications accessing certain user data must complete a verification process to remove "unverified app" warnings.

## Gmail API Scope Reference

| Scope Code | Description | Usage |
|---|---|---|
| `gmail.addons.current.action.compose` | Manage drafts and send emails via add-ons | Non-sensitive |
| `gmail.addons.current.message.action` | View emails when interacting with add-ons | Non-sensitive |
| `gmail.addons.current.message.metadata` | View email metadata when add-on is running | Sensitive |
| `gmail.addons.current.message.readonly` | View emails when add-on is running | Sensitive |
| `gmail.labels` | Create, read, update, delete labels only | Non-sensitive |
| `gmail.send` | Send messages only—no read/modify privileges | Sensitive |
| `gmail.readonly` | Read all resources and metadata—no write operations | Restricted |
| `gmail.compose` | Create, update, delete drafts; send messages | Restricted |
| `gmail.insert` | Insert and import messages only | Restricted |
| `gmail.modify` | All read/write operations except permanent deletion | Restricted |
| `gmail.metadata` | Read metadata including labels and headers | Restricted |
| `gmail.settings.basic` | Manage basic mail settings | Restricted |
| `gmail.settings.sharing` | Manage forwarding and aliases (admin use only) | Restricted |
| `mail.google.com/` | Full mailbox access including permanent deletion | Restricted |

## Scope Sensitivity Levels

**Non-sensitive** scopes provide minimal authorization and require basic app verification.

**Sensitive** scopes grant access to Google user data and require sensitive scope verification per the Google API Services: User Data Policy, but do not require security assessments.

**Restricted** scopes offer broad user data access and mandate restricted scope verification. Applications storing or transmitting restricted scope data require security assessments.

## Additional Compliance

The Gmail API Services User Data and Developer Policy governs use and access regarding user data requests. Applications using certain sensitive OAuth scopes may require Google's OAuth verification process.
