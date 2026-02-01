# Troubleshoot Authentication & Authorization Issues - Postmaster Tools

## Overview

This documentation page provides solutions for common authentication and authorization problems when working with Gmail Postmaster Tools API in Google Workspace.

## Common Issues and Solutions

### "This app isn't verified" Warning

**Problem:** The OAuth consent screen displays a verification warning when requesting access to sensitive user data.

**Solution:** Applications using sensitive scopes must undergo Google's verification process. During development, you can bypass this by selecting "Advanced > Go to {Project Name} (unsafe)."

### "File not found error for credentials.json"

**Problem:** Runtime error indicating missing credentials.json file when executing code samples.

**Solution:** Create desktop application credentials through Google Cloud Console, download the JSON file, rename it to `credentials.json`, and place it in your working directory.

### "Token has been expired or revoked"

**Problem:** Access tokens from Google Authorization Server become invalid during execution.

**Solution:** Implement token refresh logic. Reference Google's OAuth2 documentation on refresh token expiration for implementation details and potential causes.

## JavaScript-Specific Errors

### Origin Mismatch

Occurs when the web server's host/port differs from authorized JavaScript origins in Cloud Console. Verify that your browser URL matches the registered origin.

### localStorage Access Failure

Happens when browsers block third-party cookies and data storage. Enable these features or add `accounts.google.com` as an exception.

### Invalid Origin for Client

Triggered when registered domain doesn't match the hosting domain. Ensure your registered origin precisely matches the browser URL.
