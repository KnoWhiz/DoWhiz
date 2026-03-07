# DoWhiz Legacy Local DB to MongoDB Refactor Plan

Last updated: 2026-03-02  
Status: Draft (codebase-audited)

## Recommendation

The migration is worth doing. Legacy Local DB-on-Azure-Files is a real operational risk in this codebase (lock handling and per-owner file proliferation), and MongoDB is a better fit for multi-node worker/gateway deployments.

The current draft was directionally correct, but it needed a few important scope corrections. This revised plan is parity-first, milestone-driven, and designed for safe cutover with measurable progress.

## Codebase-Verified Current State

### Storage inventory (actual code as of today)

| Domain | Current backend | Where in code | Migration target |
|---|---|---|---|
| Accounts/Auth/Billing (`accounts`, `account_identifiers`, `payments`, `email_verification_tokens`) | PostgreSQL (Supabase) | `scheduler_module/src/account_store.rs` | Keep in Postgres |
| Ingestion queue | PostgreSQL or Azure Service Bus | `scheduler_module/src/ingestion_queue.rs` | Keep as-is |
| User identity (`users.db`) | Legacy Local DB | `scheduler_module/src/user_store/mod.rs` | Move to MongoDB |
| Scheduler tasks (`tasks.db` per owner) | Legacy Local DB | `scheduler_module/src/scheduler/store/*` | Move to MongoDB |
| Task executions (`task_executions`) | Legacy Local DB | `scheduler_module/src/scheduler/store/schema.rs` | Move to MongoDB |
| Scheduler index (`task_index.db`) | Legacy Local DB | `scheduler_module/src/index_store/mod.rs` | Move to MongoDB (v1) |
| Slack OAuth installations (`slack.db`) | Legacy Local DB | `scheduler_module/src/slack_store.rs` | Move to MongoDB |
| Google Docs processed comments (`google_docs_processed.db`) | Legacy Local DB | `scheduler_module/src/google_docs_poller.rs` | Move to MongoDB |
| Google Workspace processed comments (`google_workspace_processed.db`) | Legacy Local DB | `scheduler_module/src/google_workspace_poller.rs` | Move to MongoDB |
| Collaboration session store | Legacy Local DB module exists | `scheduler_module/src/collaboration_store.rs` | Defer or migrate later |
| Memory (`memo.md`) | Filesystem | `memory_store`, workspace/user dirs | Keep as files |
| Secrets (`.env`) | Filesystem | `secrets_store` | Keep as files |
| Raw payload storage | Supabase Storage (default) or Azure Blob | `raw_payload_store.rs` | Keep as object storage |

### Important behavior details that affect schema design

1. `tasks.db` is not only "per-user". It exists for legacy user IDs, account-level shadow copies, and Discord guild scopes.
2. Task `task_id` is not globally unique across all Legacy Local DB files. The same `task_id` is intentionally duplicated into account-level shadow storage.
3. `IndexStore` is actively used by the scheduler thread for due-task selection (`due_task_refs`); dropping it immediately is high risk.
4. `UserStore` currently stores one `(identifier_type, identifier)` pair per user row, not an embedded identifier list.
5. `CollaborationStore` exists but is not broadly wired into runtime flows today; treat it as lower-priority migration scope.

## Target MongoDB Model (Parity First)

### 1. `users`

Replaces Legacy Local DB `users` table.

```javascript
{
  _id: ObjectId,
  user_id: String,                 // UUID string
  identifier_type: String,         // "email" | "phone" | "slack" | "discord" | ...
  identifier: String,              // normalized
  created_at: ISODate,
  last_seen_at: ISODate
}
```

Indexes:

```javascript
db.users.createIndex({ user_id: 1 }, { unique: true });
db.users.createIndex({ identifier_type: 1, identifier: 1 }, { unique: true });
```

### 2. `tasks` (canonical runnable tasks)

Replaces per-owner Legacy Local DB `tasks` + child tables (`send_*_tasks`, `run_task_tasks`, recipients).

```javascript
{
  _id: ObjectId,
  owner_scope: {
    kind: String,                  // "legacy_user" | "discord_guild"
    id: String
  },
  task_id: String,                 // UUID string (unique within owner_scope)
  kind: String,                    // "send_email" | "run_task" | "noop"
  channel: String,                 // "email" | "slack" | "discord" | "sms" | ...
  enabled: Boolean,
  created_at: ISODate,
  last_run: ISODate,               // nullable
  retry_count: Number,
  schedule: {
    type: String,                  // "cron" | "one_shot"
    cron_expression: String,       // for cron
    next_run: ISODate,             // for cron
    run_at: ISODate                // for one_shot
  },
  payload: {
    // Polymorphic union; covers current task subtables:
    // send_email, send_slack, send_discord, send_sms,
    // send_bluebubbles, send_telegram, send_whatsapp, run_task
  },
  linked_account_ids: [String]     // optional: account UUIDs for dashboard projection
}
```

Indexes:

```javascript
db.tasks.createIndex(
  { "owner_scope.kind": 1, "owner_scope.id": 1, task_id: 1 },
  { unique: true }
);
db.tasks.createIndex({ "owner_scope.kind": 1, "owner_scope.id": 1, enabled: 1 });
db.tasks.createIndex({ enabled: 1, "schedule.next_run": 1 });
```

### 3. `task_executions`

Replaces Legacy Local DB `task_executions`.

```javascript
{
  _id: ObjectId,
  owner_scope: { kind: String, id: String },
  task_id: String,
  started_at: ISODate,
  finished_at: ISODate,            // nullable
  status: String,                  // "running" | "success" | "failed"
  error_message: String            // nullable
}
```

Indexes:

```javascript
db.task_executions.createIndex({
  "owner_scope.kind": 1,
  "owner_scope.id": 1,
  task_id: 1,
  started_at: -1
});
db.task_executions.createIndex({ started_at: -1 });
```

Optional retention:

```javascript
db.task_executions.createIndex(
  { started_at: 1 },
  { expireAfterSeconds: 7776000 }  // 90 days
);
```

### 4. `task_index` (keep in v1 for scheduler parity)

Keep this materialized view in Mongo for low-risk scheduler migration.

```javascript
{
  _id: ObjectId,
  owner_scope: { kind: String, id: String },
  task_id: String,
  next_run: ISODate,
  enabled: Boolean
}
```

Indexes:

```javascript
db.task_index.createIndex(
  { "owner_scope.kind": 1, "owner_scope.id": 1, task_id: 1 },
  { unique: true }
);
db.task_index.createIndex({ enabled: 1, next_run: 1 });
```

### 5. `account_task_views` (replace account shadow `tasks.db`)

Current code writes duplicate task rows into account-level Legacy Local DB to power `/api/account/tasks`. Replace that with an explicit read model, not duplicated runnable tasks.

```javascript
{
  _id: ObjectId,
  account_id: String,              // UUID
  task_id: String,
  source_owner_scope: { kind: String, id: String },
  channel: String,
  created_at: ISODate,
  schedule_type: String,
  next_run: ISODate,               // nullable
  run_at: ISODate,                 // nullable
  latest_execution_status: String, // nullable
  latest_error_message: String,    // nullable
  latest_execution_started_at: ISODate
}
```

Indexes:

```javascript
db.account_task_views.createIndex({ account_id: 1, task_id: 1 }, { unique: true });
db.account_task_views.createIndex({ account_id: 1, created_at: -1 });
```

### 6. `slack_installations`

```javascript
{
  _id: ObjectId,
  team_id: String,
  team_name: String,
  bot_token: String,
  bot_user_id: String,
  installed_at: ISODate
}
```

Indexes:

```javascript
db.slack_installations.createIndex({ team_id: 1 }, { unique: true });
```

### 7. `processed_comments` (unified docs/sheets/slides)

```javascript
{
  _id: ObjectId,
  file_id: String,
  file_type: String,               // "docs" | "sheets" | "slides"
  tracking_id: String,             // comment or comment+reply tracking id
  processed_at: ISODate
}
```

Indexes:

```javascript
db.processed_comments.createIndex(
  { file_id: 1, tracking_id: 1 },
  { unique: true }
);
db.processed_comments.createIndex({ file_type: 1, processed_at: -1 });
```

### 8. `workspace_files` (optional metadata cache)

```javascript
{
  _id: ObjectId,
  file_id: String,
  file_type: String,
  file_name: String,
  owner_email: String,
  last_checked_at: ISODate,
  created_at: ISODate
}
```

### 9. Collaboration data

Defer from critical path unless you explicitly decide to activate these flows now. If migrated later, keep separate collections (`collaboration_sessions`, `collaboration_messages`, `collaboration_artifacts`) to avoid unbounded document growth.

## Isolation Model (RLS Equivalent)

Mongo has no built-in row-level security equivalent for this pattern. Enforce scope in repository layer:

1. No raw collection access in business logic.
2. All repository methods take `owner_scope` or `account_id` explicitly.
3. Repositories inject scope into every query/update.
4. Add static checks (`rg "collection::<"`) and code review rule: only storage module may call raw collections.

## Migration Milestones (Trackable)

### M0 - Baseline and guardrails

- [ ] M0.1 Freeze and document current storage invariants.
- [ ] M0.2 Add feature flags: `STORAGE_BACKEND=legacy_local_db|dual|mongo`.
- [ ] M0.3 Add parity tests for account task-view behavior (Slack/Discord/Google Workspace).
- [ ] M0.4 Add metrics counters for store reads/writes/errors by backend.

Exit criteria:

- All current AUTO tests still pass on Legacy Local DB.
- Feature flags compile and default to Legacy Local DB.

### M1 - Storage interfaces without behavior change

- [ ] M1.1 Define traits: `UserRepo`, `TaskRepo`, `TaskIndexRepo`, `ExecutionRepo`, `SlackRepo`, `ProcessedCommentRepo`, `AccountTaskViewRepo`.
- [ ] M1.2 Wrap existing Legacy Local DB implementations behind traits.
- [ ] M1.3 Replace direct `SqliteSchedulerStore` call sites in service flows with trait-backed stores.

Exit criteria:

- No behavior diff in tests.
- Direct Legacy Local DB usage limited to adapter implementations.

### M2 - Mongo infrastructure

- [ ] M2.1 Add `mongodb` crate, connection config, and client lifecycle management.
- [ ] M2.2 Implement index bootstrap at startup.
- [ ] M2.3 Add startup health check + fail-fast on bad Mongo config.
- [ ] M2.4 Add structured logs for query latency and errors.

Exit criteria:

- Service starts with Mongo config and creates required indexes.

### M3 - Migrate low-risk stores first

- [ ] M3.1 Implement Mongo `SlackRepo`.
- [ ] M3.2 Implement Mongo processed comment stores (Docs + Workspace).
- [ ] M3.3 Add compatibility tests and switch these reads/writes behind flag.

Exit criteria:

- Slack install + Google poller flows pass with `STORAGE_BACKEND=mongo`.

### M4 - Migrate user + scheduler storage core

- [ ] M4.1 Implement Mongo `UserRepo`.
- [ ] M4.2 Implement Mongo `TaskRepo` for all current task kinds/channels.
- [ ] M4.3 Implement Mongo `ExecutionRepo`.
- [ ] M4.4 Implement Mongo `TaskIndexRepo` and wire scheduler due-task loop.
- [ ] M4.5 Preserve retry and failure-notification logic.

Exit criteria:

- Scheduler integration tests pass with Mongo backend.
- Due-task throughput and latency are within acceptable range vs Legacy Local DB baseline.

### M5 - Replace account-level shadow tasks with projection

- [ ] M5.1 Implement `account_task_views` write/update paths.
- [ ] M5.2 Update inbound Slack/Discord/Google Workspace to write linked account view rows.
- [ ] M5.3 Update execution completion to update account task view status.
- [ ] M5.4 Simplify `/api/account/tasks` to query projection (remove Slack legacy merge fallback).

Exit criteria:

- `/api/account/tasks` parity confirmed against baseline fixtures.
- No duplicate runnable task documents required.

### M6 - Backfill and verification tooling

- [ ] M6.1 Build idempotent migration tool for Legacy Local DB files to Mongo.
- [ ] M6.2 Classify owner scope:
  - `legacy_user`: `USERS_ROOT/<id>/state/tasks.db` where `<id>` is not an account UUID
  - `account_shadow`: account UUID paths (for view projection import only)
  - `discord_guild`: `WORKSPACE_ROOT/discord/<guild_id>/state/tasks.db`
- [ ] M6.3 Add dry-run mode and consistency report output.
- [ ] M6.4 Validate counts, due tasks, and latest execution status on sampled owners.

Exit criteria:

- Dry-run and real import both complete with acceptable diff thresholds.

### M7 - Dual-run and cutover

- [ ] M7.1 Enable dual-write in staging.
- [ ] M7.2 Run consistency checker on interval (Legacy Local DB vs Mongo).
- [ ] M7.3 Switch read path to Mongo after stable window.
- [ ] M7.4 Disable Legacy Local DB writes after additional stable window.

Exit criteria:

- No critical mismatches during stability window.
- Operational metrics and error rates acceptable.

### M8 - Cleanup

- [ ] M8.1 Remove Legacy Local DB adapters and `legacy_local_db_driver` dependency where no longer needed.
- [ ] M8.2 Remove legacy account shadow task sync code.
- [ ] M8.3 Update docs/runbooks/test checklist for Mongo-only path.

Exit criteria:

- Build/test green without Legacy Local DB task/user/index stores in runtime path.

## Suggested PR Slices

1. PR-1: trait interfaces + Legacy Local DB adapters (no behavior change).
2. PR-2: Mongo infra + index bootstrap.
3. PR-3: Slack + processed comments migration.
4. PR-4: users migration.
5. PR-5: tasks + executions + task_index migration.
6. PR-6: account_task_views + `/api/account/tasks` migration.
7. PR-7: backfill tool + dual-write + cutover controls.
8. PR-8: cleanup/removal.

## Test Strategy and Release Gates

For each milestone touching `DoWhiz_service`, run relevant AUTO tests from `reference_documentation/test_plans/DoWhiz_service_tests.md`.

Core gates for this migration:

1. Unit and integration:
   - `cargo test -p scheduler_module`
   - `cargo test -p run_task_module`
   - `cargo test -p send_emails_module`
2. Scheduler behavior parity:
   - `cargo test -p scheduler_module --test scheduler_basic`
   - `cargo test -p scheduler_module --test scheduler_followups`
   - `cargo test -p scheduler_module --test scheduler_concurrency`
   - `cargo test -p scheduler_module --test thread_latest_epoch_e2e`
3. Inbound channel flows:
   - `cargo test -p scheduler_module --test github_env_e2e`
   - `cargo test -p scheduler_module --test send_reply_outbound_e2e`
   - `cargo test -p scheduler_module --test scheduler_retry_notifications_e2e`
   - `cargo test -p scheduler_module --test scheduler_retry_notifications_slack_e2e`

For LIVE/MANUAL/PLANNED checklist entries, mark `SKIP` with reason unless explicitly run.

## Rollback Plan

1. Keep Legacy Local DB data untouched during dual-write.
2. Keep runtime read toggle (`legacy_local_db` vs `mongo`) until post-cutover stability window is complete.
3. If Mongo errors spike, switch reads back to Legacy Local DB immediately and continue dual-write investigation.
4. Only remove Legacy Local DB write paths after stable production soak.

## Open Decisions

- [ ] Keep `task_index` permanently or remove after proving direct due-task query performance on `tasks`.
- [ ] Migrate collaboration storage now, or defer until collaboration runtime wiring is expanded.
- [ ] Execution retention policy (`task_executions` TTL duration).
- [ ] Provider choice finalization: Atlas vs Cosmos Mongo API.

## Final Notes

This plan intentionally prioritizes correctness and controlled rollout over a big-bang switch. The migration should be implemented as small, auditable PRs with explicit parity checks at each step.
