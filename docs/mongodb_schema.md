# DoWhiz MongoDB Schema Design

## Overview

Migration from SQLite (Azure Files) to MongoDB to resolve:
- Multi-VM access conflicts (staging + prod accessing same files)
- SQLite lock issues on CIFS/network filesystems
- Per-user database file anti-pattern

## Current State

| Store | Current DB | Location | Issue |
|-------|-----------|----------|-------|
| AccountStore | PostgreSQL (Supabase) | Cloud | None - keep as is |
| UserStore | SQLite | Azure Files | Lock conflicts |
| TaskStore | SQLite (per-user) | Azure Files | Lock conflicts, file proliferation |
| SlackStore | SQLite | Azure Files | Lock conflicts |
| CollaborationStore | SQLite | Azure Files | Lock conflicts |
| GoogleDocsPoller | SQLite | Azure Files | Lock conflicts |
| GoogleWorkspacePoller | SQLite | Azure Files | Lock conflicts |
| IndexStore | SQLite | Azure Files | Lock conflicts |
| IngestionQueue | PostgreSQL | Supabase | None - keep as is |
| MemoryStore | Files (markdown) | Azure Files | None - keep as files |
| SecretsStore | Files (.env) | Azure Files | None - keep as files |

---

## Collections

### 1. `users`

Replaces: `UserStore` (SQLite `users` table)

```javascript
{
  _id: ObjectId,                    // MongoDB auto-generated
  user_id: String,                  // UUID, indexed, unique
  identifiers: [                    // Embedded array (was separate lookups)
    {
      type: String,                 // "email", "phone", "slack", "discord"
      value: String,                // Normalized identifier
      added_at: ISODate
    }
  ],
  created_at: ISODate,
  last_seen_at: ISODate
}
```

**Indexes:**
```javascript
db.users.createIndex({ "user_id": 1 }, { unique: true })
db.users.createIndex({ "identifiers.type": 1, "identifiers.value": 1 })
```

**Why this design:**
- Identifiers embedded (not separate collection) - a user has few identifiers, always fetched together
- Single document = atomic updates, no joins

---

### 2. `tasks`

Replaces: `TaskStore` (per-user SQLite databases with `tasks`, `send_email_tasks`, `send_slack_tasks`, etc.)

```javascript
{
  _id: ObjectId,
  task_id: String,                  // UUID, indexed, unique
  user_id: String,                  // Foreign key to users.user_id, indexed

  // Common fields (was `tasks` table)
  kind: String,                     // "send_email", "send_slack", "send_sms", "run_task", etc.
  channel: String,                  // "email", "slack", "sms", "telegram"
  enabled: Boolean,
  created_at: ISODate,
  last_run: ISODate,                // nullable
  retry_count: Number,

  // Schedule (polymorphic)
  schedule: {
    type: String,                   // "cron" or "one_shot"
    cron_expression: String,        // if type == "cron"
    next_run: ISODate,
    run_at: ISODate                 // if type == "one_shot"
  },

  // Task-specific payload (polymorphic, based on `kind`)
  payload: {
    // For send_email:
    subject: String,
    html_path: String,
    attachments_dir: String,
    from_address: String,
    recipients: [
      { type: String, address: String }  // type: "to", "cc", "bcc"
    ],
    in_reply_to: String,
    references_header: String,
    archive_root: String,
    thread_epoch: Number,
    thread_state_path: String,

    // For send_slack:
    slack_channel_id: String,
    thread_ts: String,
    text_path: String,
    workspace_dir: String,

    // For send_sms:
    from_number: String,
    to_number: String,
    text_path: String,
    thread_id: String,

    // For run_task:
    workspace_dir: String,
    input_email_dir: String,
    input_attachments_dir: String,
    memory_dir: String,
    reference_dir: String,
    model_name: String,
    runner: String,
    codex_disabled: Boolean,
    reply_to: [String],
    reply_from: String,
    employee_id: String

    // ... other task types as needed
  }
}
```

**Indexes:**
```javascript
db.tasks.createIndex({ "task_id": 1 }, { unique: true })
db.tasks.createIndex({ "user_id": 1 })
db.tasks.createIndex({ "user_id": 1, "enabled": 1 })
db.tasks.createIndex({ "schedule.next_run": 1, "enabled": 1 })  // For scheduler polling
```

**Why this design:**
- Single collection instead of 8+ tables (tasks, send_email_tasks, send_slack_tasks, etc.)
- Polymorphic `payload` field - MongoDB handles schema flexibility naturally
- `user_id` field enables multi-tenant queries with single index
- No per-user databases = no file proliferation

---

### 3. `task_executions`

Replaces: `task_executions` table (was per-user SQLite)

```javascript
{
  _id: ObjectId,
  task_id: String,                  // Foreign key to tasks.task_id
  user_id: String,                  // Denormalized for efficient queries
  started_at: ISODate,
  finished_at: ISODate,             // nullable
  status: String,                   // "running", "success", "failed"
  error_message: String             // nullable
}
```

**Indexes:**
```javascript
db.task_executions.createIndex({ "task_id": 1, "started_at": -1 })
db.task_executions.createIndex({ "user_id": 1, "started_at": -1 })
```

**TTL Index (auto-delete old executions):**
```javascript
db.task_executions.createIndex(
  { "started_at": 1 },
  { expireAfterSeconds: 2592000 }  // 30 days
)
```

---

### 4. `task_index` (Optional - for scheduler optimization)

Replaces: `IndexStore` SQLite

```javascript
{
  _id: ObjectId,
  task_id: String,
  user_id: String,
  next_run: ISODate,
  enabled: Boolean
}
```

**Note:** This could be eliminated - MongoDB can efficiently query `tasks` directly with compound index on `(schedule.next_run, enabled)`. Only keep if scheduler needs extreme throughput.

---

### 5. `slack_installations`

Replaces: `SlackStore` SQLite

```javascript
{
  _id: ObjectId,
  team_id: String,                  // Slack workspace ID, unique
  team_name: String,                // nullable
  bot_token: String,                // xoxb-... token (encrypt at rest)
  bot_user_id: String,
  installed_at: ISODate
}
```

**Indexes:**
```javascript
db.slack_installations.createIndex({ "team_id": 1 }, { unique: true })
```

---

### 6. `collaboration_sessions`

Replaces: `CollaborationStore` SQLite (3 tables → 1 collection with embedded docs)

```javascript
{
  _id: ObjectId,
  session_id: String,               // UUID, unique
  user_id: String,                  // indexed
  thread_id: String,
  primary_channel: String,          // "email", "slack", "google_docs"

  // Primary artifact (optional)
  artifact: {
    type: String,                   // "google_docs", "github_pr"
    id: String,
    title: String
  },

  original_request: String,
  status: String,                   // "active", "completed", "stale"
  workspace_path: String,

  created_at: ISODate,
  last_activity_at: ISODate,

  // Embedded messages (was separate table)
  messages: [
    {
      id: String,
      source_channel: String,
      external_message_id: String,
      sender_id: String,
      content_preview: String,      // First 500 chars
      has_attachments: Boolean,
      attachment_manifest: String,  // JSON
      timestamp: ISODate
    }
  ],

  // Embedded artifacts (was separate table)
  artifacts: [
    {
      id: String,
      type: String,                 // "google_docs", "github_pr"
      external_id: String,
      url: String,
      title: String,
      role: String,                 // "target" or "reference"
      created_at: ISODate
    }
  ]
}
```

**Indexes:**
```javascript
db.collaboration_sessions.createIndex({ "session_id": 1 }, { unique: true })
db.collaboration_sessions.createIndex({ "user_id": 1, "thread_id": 1 }, { unique: true })
db.collaboration_sessions.createIndex({ "user_id": 1, "status": 1 })
db.collaboration_sessions.createIndex({ "artifacts.type": 1, "artifacts.external_id": 1 })
```

**Why embedded:**
- Messages and artifacts are always fetched with the session
- A session has limited messages/artifacts (not unbounded)
- Atomic updates to the whole session

---

### 7. `processed_comments`

Replaces: `GoogleDocsPoller` + `GoogleWorkspacePoller` SQLite tables

```javascript
{
  _id: ObjectId,
  file_id: String,                  // Google Drive file ID
  file_type: String,                // "docs", "sheets", "slides"
  comment_id: String,
  processed_at: ISODate
}
```

**Indexes:**
```javascript
db.processed_comments.createIndex({ "file_id": 1, "comment_id": 1 }, { unique: true })
db.processed_comments.createIndex({ "file_type": 1 })
```

**TTL Index (auto-cleanup old records):**
```javascript
db.processed_comments.createIndex(
  { "processed_at": 1 },
  { expireAfterSeconds: 7776000 }  // 90 days
)
```

---

### 8. `google_workspace_files` (Optional cache)

Replaces: `google_docs_documents` + `google_workspace_files` SQLite tables

```javascript
{
  _id: ObjectId,
  file_id: String,                  // Google Drive file ID, unique
  file_type: String,                // "docs", "sheets", "slides"
  title: String,
  last_polled_at: ISODate
}
```

**Note:** This is a cache for file metadata. Could be eliminated if polling always fetches fresh from Google API.

---

## Migration Notes

### What stays in PostgreSQL (Supabase)
- Most Oauth functionalities
- `accounts` - billing, auth_user linkage
- `account_identifiers` - verified contact methods
- `payments` - Stripe payment records
- `email_verification_tokens`

### What moves to MongoDB
- `users` (UserStore)
- `tasks` + all task subtables (TaskStore)
- `task_executions`
- `task_index` (IndexStore) - optional, can query tasks directly
- `slack_installations` (SlackStore)
- `collaboration_sessions` (CollaborationStore - 3 tables merged)
- `processed_comments` (GoogleDocsPoller + GoogleWorkspacePoller)
- `google_workspace_files` (optional cache)

### What stays as files
- Memory (memo.md files) - these are user-editable markdown
- Secrets (.env files) - sensitive, should stay local or move to proper secrets manager

---

## Agent Isolation (RLS equivalent)

### What is RLS?

In PostgreSQL, Row-Level Security (RLS) lets the **database** enforce access rules:

```sql
-- PostgreSQL RLS example
CREATE POLICY user_isolation ON tasks
  USING (user_id = current_setting('app.current_user_id')::UUID);

-- Now even if code has a bug, database blocks cross-user access:
SET app.current_user_id = '123';
SELECT * FROM tasks WHERE user_id = '456';  -- Returns NOTHING (blocked by DB)
```
- RLS on `user_id`

### MongoDB: No native RLS

MongoDB doesn't have built-in RLS. Instead, isolation is enforced at **application level**:

```rust
// When agent connects, set user context
struct AgentContext {
    user_id: String,
}

// All queries MUST include user_id filter
async fn get_tasks(ctx: &AgentContext, db: &Database) -> Vec<Task> {
    db.collection("tasks")
        .find(doc! { "user_id": &ctx.user_id })  // <-- Developer must remember this
        .await
}
```
* Essentially we have to maintain RLS policy ourselves, and expose to the agent rows only by `user_id`

**Risk:** If a developer forgets the `user_id` filter, data leaks across users.

### Mitigation strategies

**1. Wrapper struct that enforces user_id (recommended)**

```rust
/// A "scoped" collection that automatically injects user_id into all queries
pub struct UserScopedCollection<T> {
    collection: Collection<T>,
    user_id: String,
}

impl<T> UserScopedCollection<T> {
    /// All queries automatically filtered by user_id
    pub async fn find(&self, mut filter: Document) -> Result<Vec<T>> {
        filter.insert("user_id", &self.user_id);  // Always injected
        self.collection.find(filter).await
    }

    pub async fn insert(&self, mut doc: Document) -> Result<()> {
        doc.insert("user_id", &self.user_id);  // Always injected
        self.collection.insert_one(doc).await
    }

    // ... delete, update also inject user_id
}
```

**2. MongoDB Atlas Field-Level Redaction (Atlas only)**

```javascript
// Atlas App Services rule (JSON)
{
  "roles": [{
    "name": "user",
    "apply_when": { "%%user.id": { "$exists": true } },
    "document_filters": {
      "read": { "user_id": "%%user.id" },
      "write": { "user_id": "%%user.id" }
    }
  }]
}
```

This is the closest to PostgreSQL RLS - database enforces the rule.

### Comparison

| Approach | Isolation Strength | Complexity | Cross-user queries |
|----------|-------------------|------------|-------------------|
| PostgreSQL RLS | Strong (DB enforced) | Low | Easy (admin bypasses RLS) |
| MongoDB wrapper struct | Medium (app enforced) | Low | Easy |
| MongoDB Atlas rules | Strong (DB enforced) | Medium | Requires admin role |
| Separate databases | Strongest | High | Impossible |

### Recommendation for DoWhiz

Use **wrapper struct** approach:
1. Simple to implement
2. Works with any MongoDB (Atlas, self-hosted, Cosmos)
3. Compile-time safety - can't accidentally use raw collection
4. Easy to audit - grep for raw collection usage

```rust
// Bad - raw access, could forget user_id
let tasks = db.collection::<Task>("tasks");

// Good - scoped access, user_id always enforced
let tasks = UserScopedCollection::new(db, &agent.user_id);
```

---

## Connection Example (Rust)

```rust
use mongodb::{Client, options::ClientOptions};

async fn connect() -> mongodb::error::Result<Client> {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

    let mut options = ClientOptions::parse(&uri).await?;
    options.app_name = Some("dowhiz-worker".to_string());

    Client::with_options(options)
}
```

---

## Comparison: SQLite vs MongoDB for this use case

| Aspect | SQLite (current) | MongoDB |
|--------|-----------------|---------|
| Multi-VM access | Broken (lock conflicts) | Native support |
| Per-user isolation | Separate files | `user_id` field |
| Schema flexibility | Rigid (migrations) | Flexible (polymorphic) |
| Horizontal scaling | Not possible | Replica sets, sharding |
| Operational complexity | Low (files) | Medium (managed service) |
| Cost | Free | ~$50/mo for Atlas M10 |

---

## Recommended MongoDB Provider

**MongoDB Atlas** (managed):
- M10 cluster (~$50/mo) handles DoWhiz scale easily
- Automatic backups, monitoring, scaling
- No CIFS/lock issues - proper database protocol

**Self-hosted** (not recommended):
- Same Azure VM issues if on Azure Files
- Use Azure Cosmos DB (MongoDB API) if staying in Azure
