# TECH_DESIGN

## Scope
This document describes the MVP tech design for Icebrew “Digital Employee”:
- Email-driven task intake for `main@icebrew.ai`
- Per-user workspace and thread memory with attachment versioning
- Task classification, billing quota, retries, and concurrency safety
- Modular, CLI-testable components

## Product goals
- Users work entirely from their email client.
- Tens of concurrent tasks without server instability.
- Clear, enforceable free-tier limits and upgrade messaging.
- Reliable, observable workflow with minimal manual intervention.
- MVP first; future-proof for tagging, monitoring, and research delivery.

## System overview
High-level flow:
1) Postmark inbound webhook receives email for `main@icebrew.ai`.
2) Webhook handler validates, normalizes, and enqueues a job quickly.
3) Celery workers process jobs asynchronously (Claude agent SDK runs here).
4) Workspace and thread memory are stored in Azure Blob.
5) PostgreSQL stores users, threads, messages, task status, and quotas.
6) Postmark outbound sends replies to the user.

Key design choice: keep webhook handler fast and stateless; all heavy work is async.

## Tech stack (MVP)
- **API**: Python + FastAPI (async inbound webhook, health, admin hooks)
- **Queue**: Celery + Redis (4 workers × concurrency 4 = 16 parallel tasks)
- **DB**: PostgreSQL (ACID for billing/quota and thread state)
- **Storage**: Azure Blob Storage (attachments + workspace)
- **Email provider**: Postmark (inbound + outbound, reliable deliverability)
- **LLM**: Claude Agent SDK (pipeline execution + LLM-as-judge)
- **Observability**: Sentry (errors), OpenTelemetry + logs (optional)

Rationale:
- Python ecosystem best fits Claude SDK and email parsing.
- Postgres ensures atomic quota enforcement.
- Celery provides stable retries and concurrency control.

## Key requirements mapped to modules

### 1) Email Pipeline
Inbound processing must be minimal and robust:
- Validate Postmark signature.
- Parse headers: `Message-ID`, `In-Reply-To`, `References`.
- Extract sender (email, name) and recipients (To/CC/BCC).
- Extract body (text + HTML) and attachments.
- Enqueue `process_email` task with normalized payload.

Outbound processing:
- Generate response via Claude agent SDK.
- Send response via Postmark outbound API.

### 2) Auto-Registration
On first email from a sender:
- Create a `users` row with `status=active`, `plan=free`.
- Initialize `tasks_used=0`.
- Create a default workspace root in Azure Blob.

### 3) Workspace Management (Azure Blob)
Workspace structure:
```
workspace/{user_id}/
  current/
    thread.md
    attachments/
  threads/
    {thread_id}/
      thread.md
      attachments/
```

Attachment versioning:
- If `report.pdf` exists, rename to `report_v1.pdf`, `report_v2.pdf`, ...
- Keep original filename as base for versioning.

Current workspace behavior:
- `current/` always mirrors the latest thread for that user.
- `threads/{thread_id}/` holds immutable snapshots for memory.

### 4) Thread Detection
Thread detection is based on headers:
- Primary: `In-Reply-To`
- Secondary: `References`
- Fallback: new thread per unique `Message-ID` if no headers

Implementation detail:
- Store `message_id`, `in_reply_to`, and `references` in DB.
- Build threads by walking `references` if needed.
- Do not rely solely on Postmark’s thread id.

### 5) LLM-as-Judge (Task Classification)
Goal: determine if email is a billable “task” vs. a clarification.
- Use Claude to classify `is_task` and `confidence`.
- If confidence < threshold (e.g. 0.6), default to non-task.
- Cache classification in DB per message to avoid rework.

### 6) Quota Management
Rules:
- 5 free tasks per user.
- After limit, refuse and direct to `icebrew.ai` upgrade flow.
- Paid plans:
  - $20/mo → 50 tasks/year
  - $200/mo → unlimited

Enforcement:
- Check quota at task start, inside a DB transaction.
- Increment `tasks_used` only once per confirmed task.
- Do not charge for non-task emails.
- Retries must not double-charge.

### 7) Retry Logic
Per task:
- 1 retry (2 total attempts).
- Capture exception and log reason.
- Mark status: `failed` after retry.

### 8) Concurrency Safety
Requirements:
- Handle “tens of tasks” in parallel.
- Avoid double-processing the same email.

Controls:
- Webhook uses idempotency: `message_id` unique constraint.
- Celery queue uses dedupe by `message_id`.
- Worker count: 4 workers × concurrency 4 (16 parallel).
- DB row locking when incrementing quota.

## Data model (PostgreSQL)
Core tables:

### users
- `id` (uuid)
- `email` (unique)
- `status` (active / blocked)
- `plan` (free / paid_20 / paid_200)
- `tasks_used` (int)
- `tasks_limit` (int or null for unlimited)
- `created_at`, `updated_at`

### threads
- `id` (uuid)
- `user_id`
- `root_message_id`
- `last_message_id`
- `created_at`, `updated_at`

### messages
- `id` (uuid)
- `thread_id`
- `message_id` (unique)
- `in_reply_to`
- `references` (jsonb array)
- `from_email`
- `subject`
- `body_text`
- `body_html`
- `received_at`
- `is_task` (bool nullable)
- `task_confidence` (float nullable)

### tasks
- `id` (uuid)
- `message_id` (unique)
- `user_id`
- `thread_id`
- `status` (queued / running / complete / failed / skipped)
- `attempts` (int)
- `last_error` (text)
- `charged` (bool)
- `created_at`, `updated_at`

### attachments
- `id` (uuid)
- `message_id`
- `filename`
- `blob_path`
- `version` (int)
- `content_type`
- `size_bytes`

## Processing pipeline detail

### Inbound webhook
1) Validate signature.
2) Normalize payload to internal schema.
3) Insert `message` if not exists (idempotent).
4) Enqueue `process_email(message_id)`.
5) Return 200 to Postmark quickly.

### Worker: process_email
1) Load message + user + thread context.
2) If new user: auto-register.
3) Resolve or create thread.
4) Save attachments to Azure Blob:
   - `threads/{thread_id}/attachments/`
   - `current/attachments/`
5) Update `thread.md` in both `threads/{thread_id}` and `current/`.
6) Run LLM-as-judge for `is_task`.
7) If not task: respond with clarification or acknowledgment.
8) If task:
   - Check quota in transaction.
   - Call Claude agent SDK for task execution.
   - Send reply.
   - Update status and quota.

## Workspace file content

### `thread.md`
Purpose:
- Store chronological email history for a given thread.
- Used as memory for LLM context.

Format:
```
# Thread: {subject}
## Message {n}
From: ...
Date: ...
Body:
...
```

## CLI-testable modules
Every module should expose a CLI entry for independent testing:

1) **email.parse**
   - Input: raw Postmark JSON
   - Output: normalized `message` object
2) **email.thread**
   - Input: message_id + headers
   - Output: thread_id
3) **storage.attachments**
   - Input: attachments list
   - Output: blob paths with versioning
4) **workspace.update**
   - Input: message + thread
   - Output: updated `thread.md`
5) **task.judge**
   - Input: message body
   - Output: is_task + confidence
6) **quota.check**
   - Input: user_id
   - Output: allow/deny + remaining
7) **task.run**
   - Input: message_id
   - Output: response email
8) **email.send**
   - Input: to, subject, body
   - Output: Postmark response

## Failure modes and mitigations
- **Duplicate emails**: `message_id` unique constraint + idempotent enqueue.
- **Worker crashes**: Celery retry with 1 retry, task status stored in DB.
- **Postmark delay**: all actions async, only webhook returns 200.
- **LLM failures**: mark `failed`, retry once, then respond with fallback.
- **Attachment conflicts**: deterministic versioning.

## Security considerations
- Validate inbound signatures.
- Encrypt secrets in env/secret manager.
- Restrict Azure Blob access to service principal.
- Rate-limit webhook endpoint.
- Store minimal PII (email only).

## Future expansions (non-MVP)
- BCC monitoring for phishing detection.
- Daily/weekly research delivery.
- Tagging and task routing by labels.
- Priority queues for paid users.
- User-defined automations (cron-like rules).

## Testing strategy
- Unit tests for each CLI module.
- Integration test pipeline using mocked Postmark payload.
- End-to-end test with Postmark sandbox + Azure Blob emulator.
- Load test: simulate 20-50 concurrent tasks via Celery.

## Operations
- Deployment: Docker + Azure App Service.
- Config via env vars (Postmark token, DB URL, Azure creds, Claude key).
- Metrics: task latency, failure rate, quota usage.

## References
- TECH_DESIGN_of_MOLTBOT_by_Codex.md
