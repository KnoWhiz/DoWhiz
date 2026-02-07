---
name: scheduler_maintain
description: Manage the current user's scheduler tasks using the workspace snapshot and scheduler action blocks.
allowed-tools: None
---

# Scheduler Management (scheduler_maintain)

## Context
- Scheduler snapshot (if available): `scheduler_snapshot.json` in workspace root.
- Snapshot includes enabled tasks scheduled between `window_start` and `window_end` (UTC, 7-day window), plus counts outside the window.

## Listing tasks
- Read and summarize `upcoming` tasks (id, kind, next_run/run_at, status, label).
- If the snapshot is missing, state that scheduler state is unavailable.

## Scheduling outputs
There are two scheduling outputs. Use the correct block for the desired action:

### A) Future email sending
Use the scheduled tasks block (this is the only way to schedule future send_email tasks):

```
SCHEDULED_TASKS_JSON_BEGIN
[
  {"type":"send_email","delay_seconds":0,"subject":"Reminder","html_path":"reminder_email_draft.html","attachments_dir":"reminder_email_attachments","to":["you@example.com"],"cc":[],"bcc":[]}
]
SCHEDULED_TASKS_JSON_END
```

### B) Scheduler management (cancel/reschedule/create run_task)
Use the scheduler actions block:

```
SCHEDULER_ACTIONS_JSON_BEGIN
[
  { "action": "cancel", "task_ids": ["..."] },
  { "action": "reschedule", "task_id": "...", "schedule": { "type": "one_shot", "run_at": "2026-02-07T12:00:00Z" } },
  { "action": "reschedule", "task_id": "...", "schedule": { "type": "cron", "expression": "0 0 9 * * *" } },
  { "action": "create_run_task", "schedule": { "type": "one_shot", "run_at": "2026-02-07T12:00:00Z" }, "model_name": "gpt-5.2-codex", "codex_disabled": false, "reply_to": ["user@example.com"] }
]
SCHEDULER_ACTIONS_JSON_END
```

## Rules
- Use RFC3339 UTC timestamps.
- Cron uses 6 fields: `sec min hour day month weekday`.
- Do not include workspace paths; `create_run_task` always targets the current workspace.
- Output only JSON inside blocks; no commentary inside blocks.
- If no changes are requested, omit the relevant block.
