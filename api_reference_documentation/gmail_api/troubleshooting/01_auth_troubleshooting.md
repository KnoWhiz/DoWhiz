# Troubleshoot Authentication & Authorization Issues

This guide addresses common problems developers encounter when implementing authentication and authorization for Gmail APIs.

## "This app isn't verified" Warning

When your OAuth consent screen displays this warning, it indicates the application is requesting access to sensitive user data. According to Google's requirements, apps using sensitive scopes must complete a verification process to remove this limitation.

During development, you can proceed by selecting **Advanced > Go to {Project Name} (unsafe)** to bypass the warning temporarily.

## Credentials File Not Found

A "file not found" error regarding `credentials.json` typically occurs when desktop application credentials haven't been properly authorized. To resolve this:

1. Create credentials for a desktop application through the credentials creation process
2. Save the downloaded JSON file as `credentials.json`
3. Move the file to your project's working directory

## Token Expiration or Revocation

When you encounter "Token has been expired" or "Token has been revoked" messages, the access token from Google's Authorization Server has either expired naturally or been manually revoked. Review Google's documentation on refresh token expiration for potential causes and solutions.

## JavaScript-Specific Errors

### Origin Mismatch

This error occurs when the host and port serving your webpage doesn't match an authorized JavaScript origin in your Google Cloud project. Verify that your authorized origin matches the URL in your browser.

### localStorage Access Failure

The error message "idpiframe_initialization_failed: Failed to read the 'localStorage' property" indicates third-party cookies and data storage are disabled in your browser. The Google Sign-in library requires these features to function properly. Consider prompting users to enable these settings or add an exception for `accounts.google.com`.

### Invalid Origin for Client

When the registered domain doesn't match your hosting domain, you'll see this error. Ensure your registered origin precisely matches the URL displayed in your browser.
