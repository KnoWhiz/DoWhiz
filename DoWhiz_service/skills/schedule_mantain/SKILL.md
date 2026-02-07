---
name: schedule_mantain
description: Manage the current user's scheduler tasks using the workspace snapshot and scheduler action blocks.
allowed-tools: None
---

# Scheduler Management (schedule_mantain)

## Context
- A scheduler snapshot may be available at `scheduler_snapshot.json` in the workspace root.
- It lists enabled tasks scheduled between `window_start` and `window_end` (UTC, 7-day window), plus counts outside the window.

## Listing tasks
- Read and summarize `upcoming` tasks (id, kind, next_run/run_at, status, label).
- If the snapshot is missing, state that scheduler state is unavailable.

## Applying changes
- If the user wants to cancel, modify, or create tasks, output a scheduler actions block at the end of your response:

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

### Rules
- Use RFC3339 UTC timestamps.
- Cron uses 6 fields: `sec min hour day month weekday`.
- Do not include workspace paths; `create_run_task` always targets the current workspace.
- Output only JSON inside the block; do not add commentary inside the block.
- If no changes are requested, omit the block.
