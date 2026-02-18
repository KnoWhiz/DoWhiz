pub(super) const SCHEDULER_SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    kind TEXT NOT NULL,
    channel TEXT NOT NULL DEFAULT 'email',
    enabled INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    last_run TEXT,
    schedule_type TEXT NOT NULL,
    cron_expression TEXT,
    next_run TEXT,
    run_at TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS send_email_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    subject TEXT NOT NULL,
    html_path TEXT NOT NULL,
    attachments_dir TEXT NOT NULL,
    from_address TEXT,
    in_reply_to TEXT,
    references_header TEXT,
    archive_root TEXT,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS send_email_recipients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    recipient_type TEXT NOT NULL,
    address TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS send_slack_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    slack_channel_id TEXT NOT NULL,
    thread_ts TEXT,
    text_path TEXT NOT NULL,
    workspace_dir TEXT
);

CREATE TABLE IF NOT EXISTS send_sms_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    from_number TEXT,
    to_number TEXT NOT NULL,
    text_path TEXT NOT NULL,
    thread_id TEXT,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS send_bluebubbles_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    chat_guid TEXT NOT NULL,
    text_path TEXT NOT NULL,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS send_telegram_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    chat_id TEXT NOT NULL,
    text_path TEXT NOT NULL,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS run_task_tasks (
    task_id TEXT PRIMARY KEY REFERENCES tasks(id) ON DELETE CASCADE,
    workspace_dir TEXT NOT NULL,
    input_email_dir TEXT NOT NULL,
    input_attachments_dir TEXT NOT NULL,
    memory_dir TEXT NOT NULL,
    reference_dir TEXT NOT NULL,
    model_name TEXT NOT NULL,
    runner TEXT NOT NULL,
    codex_disabled INTEGER NOT NULL,
    reply_to TEXT NOT NULL,
    reply_from TEXT,
    archive_root TEXT,
    thread_id TEXT,
    thread_epoch INTEGER,
    thread_state_path TEXT
);

CREATE TABLE IF NOT EXISTS task_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    started_at TEXT NOT NULL,
    finished_at TEXT,
    status TEXT NOT NULL,
    error_message TEXT
);
"#;
