# Google Service Account + Domain-Wide Delegation Setup

This guide enables DoWhiz digital employees to create, edit, and share Google Docs/Sheets/Slides using Service Account authentication instead of OAuth refresh tokens.

## Benefits

- **No token expiration**: Service Account JWT tokens are generated on-demand (no 7-day/6-month expiration)
- **No CASA certification required**: Avoids $3k-15k/year verification costs
- **No browser login needed**: API-only authentication, no CAPTCHA challenges
- **Impersonation support**: Operations appear as the impersonated user (e.g., `oliver@dowhiz.com`)

## Prerequisites

1. Google Cloud Project with APIs enabled:
   - Google Docs API
   - Google Drive API
   - Google Sheets API
   - Google Slides API

2. Google Workspace domain (e.g., `dowhiz.com`) with admin access

## 1) Create Service Account

1. Go to [Google Cloud Console](https://console.cloud.google.com/) > IAM & Admin > Service Accounts
2. Click "Create Service Account"
3. Name it (e.g., `dowhiz-workspace`)
4. Skip optional permissions
5. Click "Done"

## 2) Enable Domain-Wide Delegation

1. Click on the created Service Account
2. Go to "Details" tab
3. Click "Show Domain-Wide Delegation" section
4. Check "Enable Google Workspace Domain-Wide Delegation"
5. Save

## 3) Create JSON Key

1. Go to "Keys" tab
2. Click "Add Key" > "Create new key"
3. Select "JSON" format
4. Download the key file

Note: If key creation is blocked by organization policy (`iam.disableServiceAccountKeyCreation`), an org admin must temporarily disable this constraint.

## 4) Authorize in Google Workspace Admin

1. Login to [Google Workspace Admin Console](https://admin.google.com/) with domain admin
2. Navigate to: Security > API Controls > Domain-wide Delegation
3. Click "Add new"
4. Enter:
   - **Client ID**: (from Service Account details, e.g., `111347635953599935287`)
   - **OAuth Scopes** (comma-separated):
     ```
     https://www.googleapis.com/auth/documents,https://www.googleapis.com/auth/drive,https://www.googleapis.com/auth/spreadsheets,https://www.googleapis.com/auth/presentations
     ```
5. Click "Authorize"

Authorization may take a few minutes to propagate.

## 5) Configure Environment Variables

Add to `DoWhiz_service/.env`:

```bash
# Service Account JSON (minified, single line)
GOOGLE_SERVICE_ACCOUNT_JSON='{"type":"service_account","project_id":"...","private_key":"...","client_email":"...","client_id":"..."}'

# User to impersonate (must be in the Google Workspace domain)
GOOGLE_SERVICE_ACCOUNT_SUBJECT=oliver@dowhiz.com
```

### Per-Environment Configuration

| Environment | GOOGLE_SERVICE_ACCOUNT_SUBJECT |
|-------------|-------------------------------|
| Production  | `oliver@dowhiz.com`           |
| Staging     | `dowhiz@deep-tutor.com`       |
| Local/Test  | `proto@dowhiz.com`            |

The `GOOGLE_SERVICE_ACCOUNT_JSON` is the same across all environments.

## 6) Restart Services

```bash
pm2 restart all
```

## 7) Verification

Check gateway logs for successful token refresh:

```bash
pm2 logs dw_gateway --lines 20 | grep -i "service account"
```

Expected output:
```
INFO Service account token refreshed successfully (as oliver@dowhiz.com)
```

### Test Document Creation

Send email to the digital employee:
```
To: oliver@dowhiz.com
Subject: Test Service Account
Body: Please create a Google Doc titled "Service Account Test" and share it with me.
```

Expected behavior:
1. Agent uses `google-docs create-document` CLI (not browser)
2. No CAPTCHA or Human Approval Gate requests
3. Document created with owner = impersonated user
4. Sharing notification sent from impersonated user's email

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│ Worker (.env)                                                   │
│ ├── GOOGLE_SERVICE_ACCOUNT_JSON                                 │
│ └── GOOGLE_SERVICE_ACCOUNT_SUBJECT                              │
│                        │                                        │
│                        ▼                                        │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ACI Container                                               │ │
│ │ ├── Receives env vars via collect_google_workspace_cli_...  │ │
│ │ ├── google-docs CLI reads GOOGLE_SERVICE_ACCOUNT_JSON       │ │
│ │ └── JWT signed with RS256 → exchanged for access token      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                        │                                        │
│                        ▼                                        │
│              Google APIs (as impersonated user)                 │
└─────────────────────────────────────────────────────────────────┘
```

## Troubleshooting

### Error: `unauthorized_client`

```
Client is unauthorized to retrieve access tokens using this method,
or client not authorized for any of the scopes requested.
```

**Cause**: Domain-Wide Delegation not authorized in Google Workspace Admin.

**Fix**: Go to admin.google.com > Security > API Controls > Domain-wide Delegation, add the Service Account Client ID with required scopes.

### Error: `invalid_grant`

**Cause**: The impersonated user doesn't exist or isn't in the authorized domain.

**Fix**: Verify `GOOGLE_SERVICE_ACCOUNT_SUBJECT` is a real user in the Google Workspace domain.

### Agent still uses browser login

**Cause**: Service Account env vars not passed to ACI container.

**Fix**: Ensure PR #906 is merged (passes `GOOGLE_SERVICE_ACCOUNT_JSON` and `GOOGLE_SERVICE_ACCOUNT_SUBJECT` to ACI).

### Token refresh works in gateway but not in ACI

**Cause**: Gateway has env vars but ACI containers don't receive them.

**Fix**: Check `collect_google_workspace_cli_env_overrides()` in `codex.rs` includes Service Account vars.

## Related Files

| File | Purpose |
|------|---------|
| `scheduler_module/src/google_auth.rs` | Service Account token refresh logic |
| `run_task_module/src/run_task/codex.rs` | Env var forwarding to ACI |
| `.claude/skills/google-docs/SKILL.md` | Agent instructions for CLI usage |

## Security Notes

- Never commit `GOOGLE_SERVICE_ACCOUNT_JSON` to the repository
- The Service Account can only impersonate users in authorized domains
- Scopes are limited to Docs/Drive/Sheets/Slides (no Gmail access)
- Each operation is logged with the impersonated user's identity
