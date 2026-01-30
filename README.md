# IceBrew
IceBrew: Your digital employee. Email any task.

## Email Pipeline (MVP)
See `mvp/email_pipeline/README.md` for the refactored pipeline modules and CLI tests.

Quick start:
```
python -m mvp.email_pipeline.postmark_webhook_server --port 9000
python -m mvp.email_pipeline.cli.task_status --list --limit 20
```
