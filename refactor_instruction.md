# IceBrew Email Pipeline Refactoring

## Overview

The current implementation is too messy. Refactor the code into 4 well-defined, modular components with clear responsibilities and comprehensive CLI tests.

## Target File Structure

```
mvp/email_pipeline/
├── __init__.py
├── config.py                    # Keep existing, add new config as needed
├── sender.py                    # Module 1: Email sending
├── responder.py                 # Module 2: AI response generation
├── workspace.py                 # Module 3: Workspace preparation
├── monitor.py                   # Module 4: Email monitoring & orchestration
├── task_store.py                # Module 5: MongoDB-based task state management
├── storage.py                   # Keep existing MongoDB storage (for raw logging)
└── cli/
    ├── __init__.py
    ├── test_sender.py           # CLI tests for sender
    ├── test_responder.py        # CLI tests for responder
    ├── test_workspace.py        # CLI tests for workspace
    ├── test_monitor.py          # CLI tests for monitor
    └── task_status.py           # CLI for querying task status
```

---

## Module 1: Email Sender (`sender.py`)

### Function Signature

```python
def send_email(
    from_address: str,
    to_addresses: list[str],
    subject: str,
    markdown_file_path: str,
    attachments_dir_path: str | None = None,
    reply_to_message_id: str | None = None,
    references: str | None = None,
) -> dict:
    """
    Send an email with markdown content and optional attachments.

    Args:
        from_address: Sender email address
        to_addresses: List of recipient email addresses
        subject: Email subject line
        markdown_file_path: Path to markdown file containing email body
        attachments_dir_path: Path to folder containing attachments (optional)
        reply_to_message_id: Message-ID for email threading (optional)
        references: References header for email threading (optional)

    Returns:
        dict with keys: success (bool), message_id (str), error (str|None)
    """
```

### Requirements

1. Use Postmark API for sending (via `POSTMARK_SERVER_TOKEN` from env)
2. Convert markdown to HTML for email body, also include plain text version
3. Attach all files from `attachments_dir_path` if provided
4. Support proper email threading via `In-Reply-To` and `References` headers
5. Return the generated `Message-ID` on success

### CLI Tests (`cli/test_sender.py`)

```bash
# Test 1: Send simple email (real delivery)
python -m mvp.email_pipeline.cli.test_sender \
    --from "mini-mouse@deep-tutor.com" \
    --to "deep-tutor@deep-tutor.com" \
    --subject "Test Email" \
    --markdown-file "/path/to/test.md"

# Test 2: Send email with attachments
python -m mvp.email_pipeline.cli.test_sender \
    --from "mini-mouse@deep-tutor.com" \
    --to "deep-tutor@deep-tutor.com" \
    --subject "Test with Attachments" \
    --markdown-file "/path/to/test.md" \
    --attachments-dir "/path/to/attachments/"

# Test 3: Send reply (with threading)
python -m mvp.email_pipeline.cli.test_sender \
    --from "mini-mouse@deep-tutor.com" \
    --to "deep-tutor@deep-tutor.com" \
    --subject "Re: Original Subject" \
    --markdown-file "/path/to/reply.md" \
    --reply-to "<original-message-id@example.com>"

# Test 4: Send to multiple recipients
python -m mvp.email_pipeline.cli.test_sender \
    --from "mini-mouse@deep-tutor.com" \
    --to "deep-tutor@deep-tutor.com,another@example.com" \
    --subject "Multi-recipient Test" \
    --markdown-file "/path/to/test.md"
```

All tests should send REAL emails via Postmark. Use `deep-tutor@deep-tutor.com` as the test recipient.

---

## Module 2: Response Generator (`responder.py`)

### Function Signature

```python
def generate_response(
    workspace_dir: str,
    model: str = "codex",
) -> dict:
    """
    Generate an AI response for an email in the given workspace.

    Expects the workspace to contain:
        - email_inbox.md: The received email content
        - email_inbox_attachments/: Folder with received attachments

    Generates:
        - email_reply.md: The AI-generated response
        - email_reply_attachments/: Folder with any generated attachments

    Args:
        workspace_dir: Path to the workspace directory
        model: AI model to use ("codex" or future options)

    Returns:
        dict with keys:
            success (bool),
            reply_path (str),
            attachments_dir (str),
            error (str|None)
    """
```

### Requirements

1. Read `email_inbox.md` and `email_inbox_attachments/` from `workspace_dir`
2. Call Codex (or configured AI) to generate response
3. Save response to `email_reply.md` in `workspace_dir`
4. Save any generated attachments to `email_reply_attachments/` in `workspace_dir`
5. Use existing `codex_runner.py` logic but encapsulate it properly

### CLI Tests (`cli/test_responder.py`)

```bash
# Test 1: Generate response for a prepared workspace
python -m mvp.email_pipeline.cli.test_responder \
    --workspace "/path/to/workspace/"

# Test 2: Generate response with verbose output
python -m mvp.email_pipeline.cli.test_responder \
    --workspace "/path/to/workspace/" \
    --verbose

# Test 3: Dry-run (show what would be generated without calling AI)
python -m mvp.email_pipeline.cli.test_responder \
    --workspace "/path/to/workspace/" \
    --dry-run
```

The test should use a pre-prepared workspace with sample `email_inbox.md` and `email_inbox_attachments/`.

---

## Module 3: Workspace Manager (`workspace.py`)

### Function Signatures

```python
def prepare_workspace(
    raw_email: bytes | email.message.Message,
    workspace_root: str,
) -> dict:
    """
    Create a standardized workspace directory from a raw email.

    Creates:
        workspaces/{safe_message_id}/
        ├── raw_email.eml
        ├── email_inbox.md
        └── email_inbox_attachments/

    Args:
        raw_email: Raw email as bytes or parsed Message object
        workspace_root: Root directory for all workspaces

    Returns:
        dict with keys:
            workspace_path (str),
            message_id (str),
            from_address (str),
            to_addresses (list[str]),
            subject (str),
            success (bool),
            error (str|None)
    """

def create_workspace_from_files(
    workspace_root: str,
    inbox_md_path: str,
    inbox_attachments_path: str | None = None,
    metadata: dict | None = None,
) -> dict:
    """
    Create a workspace from existing markdown and attachments files.
    Useful for manual testing or re-processing.

    Args:
        workspace_root: Root directory for all workspaces
        inbox_md_path: Path to the email content markdown file
        inbox_attachments_path: Path to attachments folder (optional)
        metadata: Optional metadata dict (from, to, subject, message_id)

    Returns:
        dict with workspace_path and success status
    """
```

### Requirements

1. Parse email headers: From, To, Subject, Message-ID, In-Reply-To, References
2. Extract body as markdown (`email_inbox.md`)
3. Save all attachments to `email_inbox_attachments/`
4. Save raw email as `raw_email.eml`
5. Use `safe_message_id()` for directory naming
6. Handle both `bytes` and `email.message.Message` input types

### CLI Tests (`cli/test_workspace.py`)

```bash
# Test 1: Create workspace from .eml file
python -m mvp.email_pipeline.cli.test_workspace \
    --eml-file "/path/to/test.eml" \
    --workspace-root "/path/to/workspaces/"

# Test 2: Create workspace from markdown + attachments
python -m mvp.email_pipeline.cli.test_workspace \
    --inbox-md "/path/to/email.md" \
    --inbox-attachments "/path/to/attachments/" \
    --workspace-root "/path/to/workspaces/"

# Test 3: List existing workspaces
python -m mvp.email_pipeline.cli.test_workspace \
    --list \
    --workspace-root "/path/to/workspaces/"

# Test 4: Inspect a workspace
python -m mvp.email_pipeline.cli.test_workspace \
    --inspect "/path/to/workspaces/some_message_id/"
```

---

## Module 4: Email Monitor (`monitor.py`)

### Function Signatures

```python
def start_monitor(
    monitored_address: str = "mini-mouse@deep-tutor.com",
    webhook_port: int = 9000,
    max_retries: int = 2,
) -> None:
    """
    Start the email monitoring service.

    Listens for incoming emails via Postmark webhook and triggers
    the response pipeline for each new email.

    Args:
        monitored_address: Email address to monitor
        webhook_port: Port for webhook server
        max_retries: Maximum retry attempts for failed processing
    """

def process_incoming_email(
    raw_email: bytes,
    max_retries: int = 2,
) -> dict:
    """
    Process a single incoming email through the full pipeline.

    Pipeline steps:
        1. prepare_workspace() - Create workspace from email
        2. generate_response() - Generate AI response
        3. send_email() - Send the response

    Retry logic:
        - If any step fails, retry up to max_retries times
        - After all retries exhausted, log error and do not retry again
        - Record failure in storage for later inspection

    Idempotency:
        - Check storage for Message-ID before processing
        - Skip emails that have already been replied to
        - Only process new emails (not duplicates)

    Args:
        raw_email: Raw email bytes
        max_retries: Maximum retry attempts (default: 2, meaning 3 total attempts)

    Returns:
        dict with keys:
            success (bool),
            message_id (str),
            workspace_path (str),
            reply_sent (bool),
            attempts (int),
            error (str|None)
    """
```

### Requirements

1. **Webhook Server**: Use Postmark inbound webhook (not polling)
2. **Idempotency**: Use MongoDB to track processed Message-IDs (see Module 5: Task Store)
3. **Retry Logic**:
   - On failure, wait 5 seconds before retry
   - Maximum 2 retries (3 total attempts)
   - After exhausting retries, mark as `failed` and do not retry again
4. **Status Tracking**: See Module 5 for status definitions
5. **Duplicate Detection**: If same Message-ID arrives again, skip unless user sends a NEW email (different Message-ID)

### CLI Tests (`cli/test_monitor.py`)

```bash
# Test 1: Start monitor server
python -m mvp.email_pipeline.cli.test_monitor \
    --start \
    --port 9000

# Test 2: Simulate incoming email (for testing pipeline without real email)
python -m mvp.email_pipeline.cli.test_monitor \
    --simulate \
    --eml-file "/path/to/test.eml"

# Test 3: Check processing status of a message
python -m mvp.email_pipeline.cli.test_monitor \
    --status \
    --message-id "<some-message-id@example.com>"

# Test 4: List all processed emails
python -m mvp.email_pipeline.cli.test_monitor \
    --list-processed \
    --limit 20

# Test 5: Manually retry a failed email
python -m mvp.email_pipeline.cli.test_monitor \
    --retry \
    --message-id "<some-message-id@example.com>"

# Test 6: Test full pipeline with real email send
python -m mvp.email_pipeline.cli.test_monitor \
    --e2e-test \
    --from "deep-tutor@deep-tutor.com" \
    --to "mini-mouse@deep-tutor.com"
```

---

## Module 5: Task Store (`task_store.py`)

**Purpose**: MongoDB-based task state management for tracking email processing status, enabling deduplication, retry logic, and status queries. This replaces the simple `postmark_processed_ids.txt` file.

### Why MongoDB (Not Local SQLite)

- **Persistence**: Data survives server restarts, container rebuilds, and migrations
- **No Local State**: Server can be stateless, enabling horizontal scaling
- **Query Capabilities**: Rich queries for debugging and monitoring
- **Already Configured**: Project already uses MongoDB (`USE_MONGODB=True`)

### Data Model

```python
from dataclasses import dataclass
from datetime import datetime
from enum import Enum

class TaskStatus(Enum):
    PENDING = "pending"           # Received, waiting to be processed
    PROCESSING = "processing"     # Currently being processed
    COMPLETED = "completed"       # Successfully replied
    FAILED = "failed"             # All retries exhausted

@dataclass
class EmailTask:
    # Identifiers (for deduplication)
    message_id: str               # Primary key - original email Message-ID
    postmark_message_id: str      # Postmark's internal ID
    content_hash: str             # SHA256 of email content (fallback dedup)

    # Email metadata
    from_address: str
    to_addresses: list[str]
    subject: str

    # Processing state
    status: TaskStatus
    attempts: int                 # Number of attempts so far
    max_retries: int              # Maximum retries allowed (default: 2)

    # Results
    workspace_path: str | None    # Path to workspace directory
    reply_message_id: str | None  # Message-ID of sent reply

    # Error tracking
    last_error: str | None        # Last error message
    error_history: list[dict]     # [{timestamp, error, attempt}]

    # Timestamps
    created_at: datetime          # When first received
    updated_at: datetime          # Last state change
    completed_at: datetime | None # When successfully completed
```

### MongoDB Collection Schema

Collection name: `email_tasks`

```javascript
{
  "_id": "<message-id@mail.gmail.com>",  // Use message_id as _id
  "postmark_message_id": "uuid-from-postmark",
  "content_hash": "sha256-hash",

  "from_address": "sender@example.com",
  "to_addresses": ["mini-mouse@deep-tutor.com"],
  "subject": "Help me with...",

  "status": "completed",  // pending | processing | completed | failed
  "attempts": 1,
  "max_retries": 2,

  "workspace_path": "/path/to/workspaces/abc123/",
  "reply_message_id": "<reply-id@deep-tutor.com>",

  "last_error": null,
  "error_history": [],

  "created_at": ISODate("2024-01-15T10:30:00Z"),
  "updated_at": ISODate("2024-01-15T10:31:00Z"),
  "completed_at": ISODate("2024-01-15T10:31:00Z")
}
```

### Indexes

```javascript
// Create these indexes for performance
db.email_tasks.createIndex({ "status": 1 })
db.email_tasks.createIndex({ "content_hash": 1 }, { unique: true })
db.email_tasks.createIndex({ "from_address": 1 })
db.email_tasks.createIndex({ "created_at": -1 })
```

### Class Interface

```python
class TaskStore:
    def __init__(self, mongodb_uri: str, db_name: str = "icebrew_mvp"):
        """Initialize MongoDB connection."""

    # === Create / Check Duplicate ===

    def create_task(self, task: EmailTask) -> bool:
        """
        Create a new task record.
        Returns False if task already exists (duplicate).
        Uses message_id as primary key, content_hash as secondary dedup.
        """

    def is_duplicate(self, message_id: str = None, content_hash: str = None) -> bool:
        """
        Check if email has already been processed.
        Checks both message_id and content_hash.
        """

    def get_task(self, message_id: str) -> EmailTask | None:
        """Get task by message_id."""

    # === State Transitions ===

    def mark_processing(self, message_id: str) -> bool:
        """
        Mark task as processing, increment attempts.
        Returns False if task doesn't exist or already completed/failed.
        """

    def mark_completed(self, message_id: str, reply_message_id: str, workspace_path: str) -> bool:
        """
        Mark task as completed with reply details.
        Sets completed_at timestamp.
        """

    def mark_failed(self, message_id: str, error: str) -> bool:
        """
        Record failure. If attempts < max_retries, keep status as 'pending'.
        Otherwise, set status to 'failed'.
        Appends error to error_history.
        """

    def reset_for_retry(self, message_id: str) -> bool:
        """
        Manually reset a failed task to pending for retry.
        Resets attempts to 0.
        """

    # === Queries ===

    def get_pending_tasks(self, limit: int = 10) -> list[EmailTask]:
        """Get tasks waiting to be processed (for retry worker)."""

    def get_failed_tasks(self, limit: int = 100) -> list[EmailTask]:
        """Get all failed tasks for inspection."""

    def get_tasks_by_sender(self, email: str, limit: int = 50) -> list[EmailTask]:
        """Get all tasks from a specific sender."""

    def get_recent_tasks(self, limit: int = 20) -> list[EmailTask]:
        """Get most recent tasks, sorted by created_at desc."""

    def get_stats(self) -> dict:
        """
        Get statistics.
        Returns: {
            'total': int,
            'pending': int,
            'processing': int,
            'completed': int,
            'failed': int,
            'success_rate': float
        }
        """
```

### Usage in Monitor

```python
# In monitor.py

class EmailMonitor:
    def __init__(self, task_store: TaskStore, ...):
        self.task_store = task_store

    def handle_incoming_email(self, payload: dict, raw_bytes: bytes):
        # 1. Extract identifiers
        message_id = extract_message_id(payload)
        content_hash = hashlib.sha256(raw_bytes).hexdigest()

        # 2. Check for duplicate
        if self.task_store.is_duplicate(message_id, content_hash):
            logger.info(f"Duplicate email: {message_id}")
            return {"status": "duplicate"}

        # 3. Create task record
        task = EmailTask(
            message_id=message_id,
            content_hash=content_hash,
            from_address=payload.get("From", ""),
            to_addresses=[payload.get("To", "")],
            subject=payload.get("Subject", ""),
            status=TaskStatus.PENDING,
            attempts=0,
            max_retries=2,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
        )
        self.task_store.create_task(task)

        # 4. Process asynchronously
        threading.Thread(target=self._process_task, args=(message_id,)).start()
        return {"status": "accepted"}

    def _process_task(self, message_id: str):
        task = self.task_store.get_task(message_id)

        for attempt in range(task.max_retries + 1):
            self.task_store.mark_processing(message_id)

            try:
                # Run pipeline
                workspace = prepare_workspace(...)
                generate_response(workspace)
                reply_id = send_email(...)

                # Success!
                self.task_store.mark_completed(message_id, reply_id, str(workspace))
                return

            except Exception as e:
                self.task_store.mark_failed(message_id, str(e))

                if attempt < task.max_retries:
                    time.sleep(5)  # Wait before retry
                    continue
                else:
                    logger.error(f"Task {message_id} failed after {attempt + 1} attempts")
                    return
```

### CLI Tool (`cli/task_status.py`)

```bash
# List recent tasks
python -m mvp.email_pipeline.cli.task_status --list --limit 20

# Get specific task
python -m mvp.email_pipeline.cli.task_status --get "<message-id@mail.gmail.com>"

# List failed tasks
python -m mvp.email_pipeline.cli.task_status --failed

# List pending tasks (awaiting retry)
python -m mvp.email_pipeline.cli.task_status --pending

# Get statistics
python -m mvp.email_pipeline.cli.task_status --stats

# Manually retry a failed task
python -m mvp.email_pipeline.cli.task_status --retry "<message-id@mail.gmail.com>"

# Search by sender
python -m mvp.email_pipeline.cli.task_status --sender "user@gmail.com"
```

### Migration from `postmark_processed_ids.txt`

1. The old txt file only stored IDs with no status information
2. During migration, existing IDs can be imported as `completed` tasks with minimal metadata
3. After migration is verified, delete `state/postmark_processed_ids.txt`

```python
def migrate_from_txt(txt_path: Path, task_store: TaskStore):
    """One-time migration from old txt file to MongoDB."""
    if not txt_path.exists():
        return

    for line in txt_path.read_text().splitlines():
        line = line.strip()
        if not line:
            continue

        # Create minimal task record
        task = EmailTask(
            message_id=line,
            content_hash=line,  # Use ID as hash for old records
            status=TaskStatus.COMPLETED,
            attempts=1,
            # ... minimal fields
        )
        task_store.create_task(task)

    # Rename old file
    txt_path.rename(txt_path.with_suffix('.txt.migrated'))
```

---

## Configuration

All modules should read configuration from environment variables (via existing `config.py`):

| Variable | Purpose | Example |
|----------|---------|---------|
| `POSTMARK_SERVER_TOKEN` | Postmark API token | `2d1fdfa2-...` |
| `OUTBOUND_FROM` | Default sender address | `mini-mouse@deep-tutor.com` |
| `WORKSPACE_ROOT` | Root directory for workspaces | `./workspaces` |
| `MONGODB_URI` | MongoDB connection string (**required**) | `mongodb://localhost:27017` |
| `MONGODB_DB` | MongoDB database name | `icebrew_mvp` |
| `MONITOR_WEBHOOK_PORT` | Webhook server port | `9000` |
| `MAX_RETRIES` | Default retry count | `2` |

---

## Testing Guidelines

1. **Unit Tests**: Each module should have unit tests with mocked dependencies
2. **CLI Tests**: All CLI tests should work with real services (Postmark, AI)
3. **Test Email Address**: Use `deep-tutor@deep-tutor.com` for receiving test emails
4. **Test Sender Address**: Use `mini-mouse@deep-tutor.com` for sending
5. **Verbose Mode**: All CLI tools should support `--verbose` flag for debugging
6. **Dry-Run Mode**: Where applicable, support `--dry-run` to preview without side effects

---

## Migration Notes

1. Keep existing files (`pipeline.py`, `server.py`, `postmark_webhook_server.py`) during transition
2. New modules should import from existing utilities where possible (`email_utils.py`, `storage.py`)
3. Run migration script to import existing `postmark_processed_ids.txt` into MongoDB
4. Once new modules are tested and stable, deprecate old files
5. Delete `state/postmark_processed_ids.txt` after verifying migration
6. Update `README.md` with new CLI commands after completion

---

## Success Criteria

- [ ] All 5 modules implemented with the specified function signatures
- [ ] All CLI tests pass with real email delivery
- [ ] MongoDB task store working with proper indexes
- [ ] Retry logic works correctly (verified with simulated failures)
- [ ] Idempotency works (same email not processed twice, even after server restart)
- [ ] Email threading preserved in replies
- [ ] CLI tool can query task status from MongoDB
- [ ] Migration from old txt file completed
- [ ] Code is well-documented with docstrings
- [ ] No regressions in existing functionality
