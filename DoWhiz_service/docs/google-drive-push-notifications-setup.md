# Google Drive Push Notifications Setup

This guide enables near-real-time Google Docs/Sheets comment handling by using Drive push notifications.

Without push, gateway pollers run on interval (default 15s). With push, webhook events trigger immediate single-file poll.

## 1) Required Env

Set in runtime `DoWhiz_service/.env`:

```bash
GOOGLE_DRIVE_PUSH_ENABLED=true
GOOGLE_DRIVE_WEBHOOK_URL=https://<public-domain>/webhooks/google-drive-changes
```

Optional:

```bash
GOOGLE_DRIVE_CHANNEL_EXPIRATION_SECS=3600
```

## 2) Public Webhook Requirement

`GOOGLE_DRIVE_WEBHOOK_URL` must be publicly reachable over HTTPS.

Expected route in gateway:
- `POST /webhooks/google-drive-changes`

If you use reverse proxy (Caddy/Nginx), ensure this path reaches `inbound_gateway` (default local port 9100).

## 3) Domain Verification

Google Drive push notifications require verified domain ownership in Google Cloud domain verification.

Checklist:
1. Project with Drive API enabled
2. OAuth/client config already working for polling path
3. Domain used by `GOOGLE_DRIVE_WEBHOOK_URL` verified in Google Cloud

## 4) Restart and Verify

Restart gateway after env updates.

PM2 example:

```bash
cd /home/azureuser/server/.dowhiz/DoWhiz/DoWhiz_service
pm2 restart dw_gateway --update-env
```

Webhook health probe:

```bash
curl -X POST "https://<public-domain>/webhooks/google-drive-changes" \
  -H "X-Goog-Channel-ID: test-channel" \
  -H "X-Goog-Resource-ID: test-resource" \
  -H "X-Goog-Resource-State: sync"
```

Expected response body contains status `ok`.

## 5) Runtime Behavior Notes

- Docs and Sheets support Drive `files.watch` channels.
- Slides does not support `files.watch`; Slides remains polling-only.
- Gateway keeps channel renewal and maps `resource_id` back to file id for immediate poll.

## 6) Troubleshooting

### Notification received but no tasks enqueued

Check:
- route exists for `google_docs` / `google_sheets` in `gateway.toml`
- worker for target `employee_id` is running
- queue credentials and queue name are consistent between gateway and worker

### No webhook traffic observed

Check:
- public DNS + TLS reachability
- reverse proxy path forwarding
- Google domain verification status

### Channel expires frequently or not renewed

Check:
- gateway logs for renewal failures
- OAuth token refresh validity
- Drive API quotas
