# DoWhiz
DoWhiz is an email-first "digital employee" prototype. Send a task to an agent
inbox, run a Codex-backed workflow, and return a reply in the same thread.

## Vision (long-term)
- A team of role-based digital employees (researcher, TPM, docs collaborator).
- Minimal interruptions: ask for clarification only when needed.
- Tool-neutral execution with auditable, per-user isolated memory and data.
- Multi-tenant architecture: ingress, control plane, orchestrator, runtime,
  storage, and observability.

See `vision.md` for the full narrative (currently in Chinese).

## MVPs in this repo
- `mvp_python/email_pipeline/`: Local SMTP pipeline with a captured outbox.
- `DoWhiz_service/`: Postmark inbound webhook service with a scheduler for follow-ups.

## Quick start

### Python local pipeline (SMTP + local outbox)
Install deps:
```
pip install -r mvp_python/email_pipeline/requirements.txt
```

Terminal 1:
```
python -m mvp_python.email_pipeline.server
```

Terminal 2:
```
python -m mvp_python.email_pipeline.send_test_email \
  --from oliver@dowhiz.com \
  --to deep-tutor@deep-tutor.com
```

Read captured replies:
```
python -m mvp_python.email_pipeline.read_outbox
```

See `mvp_python/email_pipeline/README.md` for Postmark and real-email steps.

### Rust Postmark service (webhook + scheduler)
Terminal 1:
```
cargo run -p scheduler_module --bin rust_service -- --host 0.0.0.0 --port 9001
```

Terminal 2 (optional, for public inbound hook):
```
ngrok http 9001
```

Terminal 3 (optional, set Postmark inbound hook):
```
cargo run -p scheduler_module --bin set_postmark_inbound_hook -- \
  --hook-url https://YOUR-NGROK-URL.ngrok-free.dev/postmark/inbound
```

See `DoWhiz_service/README.md` for full setup and tests.

## Repo layout
- `mvp_python/email_pipeline/`: Python MVP for inbound email and replies.
- `DoWhiz_service/`: Rust workspace (scheduler, run_task, send_emails).
- `external/openclaw/`: Vendored upstream project; see its `AGENTS.md`.
- `api_reference_documentation/postmark_api/`: Postmark API notes.
- `example_files/`: Sample attachments for local testing.

## Environment and secrets
- Store secrets in `.env`; never commit it.
- Codex required unless `CODEX_DISABLED=1`.
- API keys used by Codex are read from `.env`.

## Generated artifacts (gitignored)
- `mvp_python/email_pipeline/workspaces/` and `mvp_python/email_pipeline/outbox/`
- `DoWhiz_service/.workspace`

## Testing
- Rust: `cargo test -p scheduler_module`
- Python: use the provided scripts (`send_test_email`, `real_email_test.py`)
