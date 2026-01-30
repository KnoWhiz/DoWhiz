# IceBrew Email Pipeline

Core modules are now structured as packages (one folder per module):

- `sender/`: Postmark outbound sending
- `responder/`: AI response generation
- `workspace/`: Workspace preparation
- `monitor/`: Webhook + orchestration
- `task_store/`: MongoDB task tracking

Supporting utilities remain at the package root (`config.py`, `email_utils.py`, `codex_runner.py`, `storage.py`).

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

## Module Tests (with per-folder reports)
Each module has 5 tests. Running tests via the module test package writes `report.json`
inside that module's `tests/` folder.

```
python -m mvp.email_pipeline.sender.tests
python -m mvp.email_pipeline.responder.tests
python -m mvp.email_pipeline.workspace.tests
python -m mvp.email_pipeline.monitor.tests
python -m mvp.email_pipeline.task_store.tests
```

Reports are written to:
- `mvp/email_pipeline/sender/tests/report.json`
- `mvp/email_pipeline/responder/tests/report.json`
- `mvp/email_pipeline/workspace/tests/report.json`
- `mvp/email_pipeline/monitor/tests/report.json`
- `mvp/email_pipeline/task_store/tests/report.json`

## Webhook Server
Start the Postmark inbound webhook listener:
```
python -m mvp.email_pipeline.monitor --port 9000
```

## Migration
A migration helper (`task_store.migrate_from_txt`) runs at monitor startup to import the old
`state/postmark_processed_ids.txt` into MongoDB (and rename it to `*.migrated` if found).
