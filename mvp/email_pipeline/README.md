# IceBrew Email Pipeline

This refactor organizes the pipeline into five clear modules with CLI tooling and tests:

- `sender.py`: Postmark outbound email sending
- `responder.py`: AI response generation
- `workspace.py`: Workspace preparation
- `monitor.py`: Postmark inbound webhook + orchestration
- `task_store.py`: MongoDB task state tracking

## Prereqs
- Python 3.12
- `pymongo` installed (`pip install -r mvp/email_pipeline/requirements.txt`)
- `POSTMARK_SERVER_TOKEN` for real email sends
- `MONGODB_URI` pointing to a running MongoDB instance

## Environment Variables
- `POSTMARK_SERVER_TOKEN`: Postmark API token
- `OUTBOUND_FROM`: Default sender address
- `WORKSPACE_ROOT`: Root directory for workspaces
- `MONGODB_URI`: MongoDB connection string
- `MONGODB_DB`: MongoDB database name
- `MONITOR_WEBHOOK_PORT`: Webhook server port
- `MAX_RETRIES`: Default retry count

## CLI Tests
All CLI tests run in dry-run mode unless `--real` is passed.

### Sender
```
python -m mvp.email_pipeline.cli.test_sender \
  --real \
  --from "mini-mouse@deep-tutor.com" \
  --to "deep-tutor@deep-tutor.com" \
  --subject "Test Email" \
  --markdown-file "/path/to/test.md"

python -m mvp.email_pipeline.cli.test_sender \
  --real \
  --from "mini-mouse@deep-tutor.com" \
  --to "deep-tutor@deep-tutor.com,another@example.com" \
  --subject "Multi-recipient Test" \
  --markdown-file "/path/to/test.md" \
  --attachments-dir "/path/to/attachments"
```

### Responder
```
python -m mvp.email_pipeline.cli.test_responder \
  --workspace "/path/to/workspace" \
  --dry-run

python -m mvp.email_pipeline.cli.test_responder \
  --real \
  --workspace "/path/to/workspace" \
  --verbose
```

### Workspace
```
python -m mvp.email_pipeline.cli.test_workspace \
  --eml-file "/path/to/test.eml" \
  --workspace-root "/path/to/workspaces"

python -m mvp.email_pipeline.cli.test_workspace \
  --inbox-md "/path/to/email.md" \
  --inbox-attachments "/path/to/attachments" \
  --workspace-root "/path/to/workspaces"

python -m mvp.email_pipeline.cli.test_workspace \
  --list \
  --workspace-root "/path/to/workspaces"

python -m mvp.email_pipeline.cli.test_workspace \
  --inspect "/path/to/workspaces/some_message_id"
```

### Monitor
```
python -m mvp.email_pipeline.cli.test_monitor \
  --start \
  --port 9000

python -m mvp.email_pipeline.cli.test_monitor \
  --simulate \
  --eml-file "/path/to/test.eml"

python -m mvp.email_pipeline.cli.test_monitor \
  --status \
  --message-id "<some-message-id@example.com>"
```

### Task Status
```
python -m mvp.email_pipeline.cli.task_status --list --limit 20
python -m mvp.email_pipeline.cli.task_status --get "<message-id@example.com>"
python -m mvp.email_pipeline.cli.task_status --failed
python -m mvp.email_pipeline.cli.task_status --pending
python -m mvp.email_pipeline.cli.task_status --stats
python -m mvp.email_pipeline.cli.task_status --retry "<message-id@example.com>"
python -m mvp.email_pipeline.cli.task_status --sender "user@gmail.com"
```

## Webhook Server
Start the Postmark inbound webhook listener:
```
python -m mvp.email_pipeline.postmark_webhook_server --port 9000
```

## Legacy SMTP Ingress (Deprecated)
A legacy SMTP ingress still exists for local testing:
```
python -m mvp.email_pipeline.server --inbound-port 8025
```

## Migration
A migration helper (`task_store.migrate_from_txt`) is used by the monitor on startup to import the old
dedupe file into MongoDB and rename it to `*.migrated`.
