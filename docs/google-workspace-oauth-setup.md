# Google Workspace Authentication Configuration

This document explains how to configure authentication for digital employees (Oliver, Proto, etc.) to use Google Workspace features.

> **推荐**: 使用 [google-workspace-service-account-setup.md](google-workspace-service-account-setup.md) 中的 Service Account + DWD 方案，有完整的 copy-paste 步骤。

## Recommended: Service Account + Domain-Wide Delegation (DWD)

**For production use, always use Service Account + DWD.** This approach:
- ✅ No token expiration - service accounts auto-refresh
- ✅ No 2FA prompts for users
- ✅ No per-user OAuth flow needed
- ✅ Central management by domain admin
- ✅ Works seamlessly across all domain users

### Step 1: Create Service Account

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Select your project (or create one)
3. Navigate to **IAM & Admin** > **Service Accounts**
4. Click **Create Service Account**
5. Name it (e.g., `dowhiz-digital-employee`)
6. Click **Create and Continue** > **Done**

### Step 2: Enable Domain-Wide Delegation

1. Click on the service account you just created
2. Go to **Details** tab
3. Under **Advanced settings**, check **Enable Google Workspace Domain-wide Delegation**
4. Note the **Client ID** (numeric, e.g., `123456789012345678901`)
5. Go to **Keys** tab > **Add Key** > **Create new key** > **JSON**
6. Download and securely store the JSON key file

### Step 3: Authorize in Workspace Admin Console

1. Go to [Google Workspace Admin Console](https://admin.google.com/)
2. Navigate to **Security** > **Access and data control** > **API controls**
3. Click **Manage Domain Wide Delegation**
4. Click **Add new**
5. Enter the **Client ID** from Step 2
6. Add these scopes (comma-separated):
   ```
   https://www.googleapis.com/auth/calendar,https://www.googleapis.com/auth/calendar.events,https://www.googleapis.com/auth/tasks,https://www.googleapis.com/auth/contacts.readonly,https://www.googleapis.com/auth/documents,https://www.googleapis.com/auth/drive,https://www.googleapis.com/auth/spreadsheets,https://www.googleapis.com/auth/presentations
   ```
7. Click **Authorize**

### Step 4: Enable Required APIs

In Google Cloud Console, enable these APIs:
- Google Calendar API
- Google Tasks API
- People API (for contacts)
- Google Drive API
- Google Docs API
- Google Sheets API
- Google Slides API

Navigate to **APIs & Services** > **Library** and search for each.

### Step 5: Configure Environment Variables

```bash
# Service Account JSON key (base64 encoded or file path)
GOOGLE_SERVICE_ACCOUNT_JSON="/path/to/service-account-key.json"

# Or as base64 encoded string (for CI/CD):
# GOOGLE_SERVICE_ACCOUNT_JSON_BASE64="eyJ0eXBlIjoi..."

# User to impersonate (the digital employee's Google account)
GOOGLE_SERVICE_ACCOUNT_SUBJECT_LITTLE_BEAR="oliver@dowhiz.com"
GOOGLE_SERVICE_ACCOUNT_SUBJECT_BOILED_EGG="proto@dowhiz.com"
GOOGLE_SERVICE_ACCOUNT_SUBJECT_MINI_MOUSE="maggie@dowhiz.com"
```

### Step 6: Token Generation (Automatic)

The task runner automatically generates access tokens using the service account:

```rust
// In run_task setup - generates token via service account impersonation
let token = google_auth::get_access_token_for_employee(employee_id).await?;
env::set_var("GOOGLE_WORKSPACE_CLI_TOKEN", token);
```

No user intervention, no 2FA, no token expiration issues.

### Verify Setup

```bash
# Generate token and test
gws calendar events list --params '{"calendarId":"primary","maxResults":1}' --format json
```

---

## Alternative: OAuth Refresh Tokens (Testing Only)

⚠️ **Use only for testing or non-Workspace domains.** OAuth tokens have these drawbacks:
- Tokens can expire or be revoked
- Users may need to re-authorize periodically
- 2FA may be triggered during re-authorization
- Requires per-user OAuth flow

### Required Scopes

```
https://www.googleapis.com/auth/documents
https://www.googleapis.com/auth/drive
https://www.googleapis.com/auth/spreadsheets
https://www.googleapis.com/auth/presentations
https://www.googleapis.com/auth/calendar
https://www.googleapis.com/auth/calendar.events
https://www.googleapis.com/auth/tasks
https://www.googleapis.com/auth/contacts.readonly
```

### Get Refresh Token via OAuth Playground

1. Go to [OAuth 2.0 Playground](https://developers.google.com/oauthplayground/)
2. Click gear icon > Check "Use your own OAuth credentials"
3. Enter your Client ID and Client Secret
4. Select all required scopes
5. Click **Authorize APIs** > Sign in
6. Click **Exchange authorization code for tokens**
7. Copy the **Refresh Token**

### Environment Variables (OAuth)

```bash
GOOGLE_CLIENT_ID="123456.apps.googleusercontent.com"
GOOGLE_CLIENT_SECRET="GOCSPX-xxxxx"
GOOGLE_REFRESH_TOKEN_LITTLE_BEAR="1//0xxxxx"
GOOGLE_REFRESH_TOKEN_BOILED_EGG="1//0xxxxx"
```

---

## Troubleshooting

### "Insufficient Permission" Error

**Service Account:** Check that the scope is authorized in Workspace Admin Console.

**OAuth:** The refresh token doesn't have the required scope. Re-authorize.

### "Token Expired" / "Invalid Grant" Error

**Service Account:** Shouldn't happen. Check the JSON key is valid.

**OAuth:** The refresh token was revoked. Re-run OAuth flow.

### "User not found" / "Delegation denied"

**Service Account:** The impersonated user must exist in the Workspace domain. Verify `GOOGLE_SERVICE_ACCOUNT_SUBJECT_*` is correct.

### Calendar/Tasks API Not Working

Ensure the API is enabled in Google Cloud Console under **APIs & Services** > **Library**.
