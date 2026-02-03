# Local Email Pipeline MVP

This MVP runs a local SMTP server that receives emails for `agent@dowhiz.com`, triggers the Codex CLI, writes `email_reply.md`, and sends a reply in the same thread. Outbound replies are captured by a local SMTP sink for easy inspection.

## Prereqs
- Python 3.12
- `codex` CLI on your PATH
- `.env` contains `AZURE_OPENAI_API_KEY_BACKUP` and `AZURE_OPENAI_ENDPOINT_BACKUP`

Install deps:
```
pip install -r mvp/email_pipeline/requirements.txt
```

## Run end-to-end (two terminals)

Terminal 1: start the pipeline server
```
python -m mvp.email_pipeline.server
```

Terminal 2: send a test email with PDF + DOCX attachments
```
python -m mvp.email_pipeline.send_test_email \
  --from agent@dowhiz.com \
  --to deep-tutor@deep-tutor.com
```

Check the reply captured in the outbox:
```
python -m mvp.email_pipeline.read_outbox
```

Artifacts are saved under:
- `mvp/email_pipeline/workspaces/<message_id>/email_reply.md`
- `mvp/email_pipeline/workspaces/<message_id>/email_reply_attachments/`
- `mvp/email_pipeline/outbox/`

## Environment knobs
- `INBOUND_SMTP_HOST` / `INBOUND_SMTP_PORT`
- `OUTBOUND_MODE` = `smtp` (default) or `postmark`
- `OUTBOUND_SMTP_HOST` / `OUTBOUND_SMTP_PORT`
- `START_OUTBOX_SERVER` = `1` (default)
- `WORKSPACE_ROOT`
- `CODEX_MODEL`
- `CODEX_DISABLED=1` to bypass Codex CLI
- `MONGODB_URI`, `MONGODB_DB`, `USE_MONGODB=1`
- `PROCESSED_IDS_PATH` to override webhook dedupe storage

## Postmark outbound (optional)
Set:
- `OUTBOUND_MODE=postmark`
- `POSTMARK_SERVER_TOKEN`

Then run the server; outbound replies will go through Postmark.

## Start real email service (manual send → reply)
Use this when you want to send real email from any inbox to `agent@dowhiz.com`.

Terminal 1: start the inbound webhook service
```
python -m mvp.email_pipeline.postmark_webhook_server --port 9000
```

Terminal 2: expose webhook with ngrok
```
ngrok http 9000
```

Terminal 3: set Postmark inbound hook to the ngrok URL
```
python -m mvp.email_pipeline.set_postmark_inbound_hook \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound
```

Send an email manually to:
```
agent@dowhiz.com
```

The pipeline will run and reply via Postmark to the sender.

If you need to reprocess an email that was deduped, clear the dedupe file:
```
rm -f mvp/email_pipeline/state/postmark_processed_ids.txt
```

## Real email end-to-end (Postmark inbound + outbound)
This test starts a Postmark inbound webhook receiver locally, exposes it with ngrok, sends a real email to your Postmark server’s inbound address (hash@inbound.postmarkapp.com), and verifies that a reply is sent back via Postmark.

```
python -m mvp.email_pipeline.real_email_test --from agent@dowhiz.com
```

Note: This uses the server token in `.env` to temporarily set `InboundHookUrl` on your Postmark server and resets it afterward.

## Set Postmark inbound hook (helper)
```
python -m mvp.email_pipeline.set_postmark_inbound_hook \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound
```
