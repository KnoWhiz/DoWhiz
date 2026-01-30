from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime
from enum import Enum
from pathlib import Path
from typing import Any, Dict

try:
    from pymongo import MongoClient
    from pymongo.errors import DuplicateKeyError
except Exception:  # pragma: no cover
    MongoClient = None
    DuplicateKeyError = Exception


class TaskStatus(Enum):
    PENDING = "pending"
    PROCESSING = "processing"
    COMPLETED = "completed"
    FAILED = "failed"


@dataclass
class EmailTask:
    # Identifiers (for deduplication)
    message_id: str
    postmark_message_id: str | None
    content_hash: str

    # Email metadata
    from_address: str
    to_addresses: list[str]
    subject: str

    # Processing state
    status: TaskStatus
    attempts: int
    max_retries: int

    # Results
    workspace_path: str | None = None
    reply_message_id: str | None = None

    # Error tracking
    last_error: str | None = None
    error_history: list[dict] = field(default_factory=list)

    # Timestamps
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)
    completed_at: datetime | None = None


class TaskStore:
    def __init__(self, mongodb_uri: str, db_name: str = "icebrew_mvp"):
        """Initialize MongoDB connection."""
        if MongoClient is None:
            raise RuntimeError("pymongo is not installed")
        self.client = MongoClient(mongodb_uri)
        self.db = self.client[db_name]
        self.collection = self.db["email_tasks"]
        self._ensure_indexes()

    def _ensure_indexes(self) -> None:
        self.collection.create_index([("status", 1)])
        self.collection.create_index([("content_hash", 1)])
        self.collection.create_index([("from_address", 1)])
        self.collection.create_index([("created_at", -1)])

    # === Create / Check Duplicate ===

    def create_task(self, task: EmailTask) -> bool:
        """
        Create a new task record.
        Returns False if task already exists (duplicate).
        Uses message_id as primary key; only fall back to content_hash when message_id is missing.
        If message_id is missing, set message_id to `hash:<content_hash>` and use it as `_id`.
        """
        if not task.message_id:
            task.message_id = f"hash:{task.content_hash}"
        doc = _task_to_doc(task)
        doc["_id"] = task.message_id
        try:
            self.collection.insert_one(doc)
        except DuplicateKeyError:
            return False
        return True

    def is_duplicate(self, message_id: str | None = None, content_hash: str | None = None) -> bool:
        """
        Check if email has already been processed.
        Prefer message_id; if missing, synthesize it from content_hash.
        """
        lookup_id = message_id or (f"hash:{content_hash}" if content_hash else None)
        if not lookup_id:
            return False
        return self.collection.find_one({"_id": lookup_id}) is not None

    def get_task(self, message_id: str) -> EmailTask | None:
        """Get task by message_id."""
        doc = self.collection.find_one({"_id": message_id})
        if not doc:
            return None
        return _doc_to_task(doc)

    # === State Transitions ===

    def mark_processing(self, message_id: str) -> bool:
        """
        Mark task as processing, increment attempts.
        Returns False if task doesn't exist or already completed/failed.
        """
        now = datetime.utcnow()
        result = self.collection.update_one(
            {"_id": message_id, "status": {"$nin": [TaskStatus.COMPLETED.value, TaskStatus.FAILED.value]}},
            {"$set": {"status": TaskStatus.PROCESSING.value, "updated_at": now}, "$inc": {"attempts": 1}},
        )
        return result.modified_count == 1

    def mark_completed(self, message_id: str, reply_message_id: str, workspace_path: str) -> bool:
        """
        Mark task as completed with reply details.
        Sets completed_at timestamp.
        """
        now = datetime.utcnow()
        result = self.collection.update_one(
            {"_id": message_id},
            {
                "$set": {
                    "status": TaskStatus.COMPLETED.value,
                    "reply_message_id": reply_message_id,
                    "workspace_path": workspace_path,
                    "updated_at": now,
                    "completed_at": now,
                }
            },
        )
        return result.modified_count == 1

    def mark_failed(self, message_id: str, error: str) -> bool:
        """
        Record failure. If attempts <= max_retries, keep status as 'pending'.
        Otherwise, set status to 'failed'.
        Appends error to error_history.
        """
        doc = self.collection.find_one({"_id": message_id})
        if not doc:
            return False
        attempts = int(doc.get("attempts", 0))
        max_retries = int(doc.get("max_retries", 2))
        now = datetime.utcnow()
        status = TaskStatus.PENDING.value if attempts <= max_retries else TaskStatus.FAILED.value
        update: Dict[str, Any] = {
            "$set": {
                "status": status,
                "last_error": error,
                "updated_at": now,
            },
            "$push": {"error_history": {"timestamp": now, "error": error, "attempt": attempts}},
        }
        result = self.collection.update_one({"_id": message_id}, update)
        return result.modified_count == 1

    def reset_for_retry(self, message_id: str) -> bool:
        """
        Manually reset a failed task to pending for retry.
        Resets attempts to 0.
        """
        now = datetime.utcnow()
        result = self.collection.update_one(
            {"_id": message_id},
            {"$set": {"status": TaskStatus.PENDING.value, "attempts": 0, "updated_at": now}},
        )
        return result.modified_count == 1

    # === Queries ===

    def get_pending_tasks(self, limit: int = 10) -> list[EmailTask]:
        """Get tasks waiting to be processed (for retry worker)."""
        cursor = self.collection.find({"status": TaskStatus.PENDING.value}).sort("created_at", 1).limit(limit)
        return [_doc_to_task(doc) for doc in cursor]

    def get_failed_tasks(self, limit: int = 100) -> list[EmailTask]:
        """Get all failed tasks for inspection."""
        cursor = self.collection.find({"status": TaskStatus.FAILED.value}).sort("created_at", -1).limit(limit)
        return [_doc_to_task(doc) for doc in cursor]

    def get_tasks_by_sender(self, email: str, limit: int = 50) -> list[EmailTask]:
        """Get all tasks from a specific sender."""
        cursor = self.collection.find({"from_address": email}).sort("created_at", -1).limit(limit)
        return [_doc_to_task(doc) for doc in cursor]

    def get_recent_tasks(self, limit: int = 20) -> list[EmailTask]:
        """Get most recent tasks, sorted by created_at desc."""
        cursor = self.collection.find({}).sort("created_at", -1).limit(limit)
        return [_doc_to_task(doc) for doc in cursor]

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
        total = self.collection.count_documents({})
        pending = self.collection.count_documents({"status": TaskStatus.PENDING.value})
        processing = self.collection.count_documents({"status": TaskStatus.PROCESSING.value})
        completed = self.collection.count_documents({"status": TaskStatus.COMPLETED.value})
        failed = self.collection.count_documents({"status": TaskStatus.FAILED.value})
        success_rate = (completed / total) if total else 0.0
        return {
            "total": total,
            "pending": pending,
            "processing": processing,
            "completed": completed,
            "failed": failed,
            "success_rate": success_rate,
        }


def migrate_from_txt(txt_path: Path, task_store: TaskStore) -> None:
    """One-time migration from old txt file to MongoDB."""
    if not txt_path.exists():
        return

    for raw in txt_path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        task = EmailTask(
            message_id=line,
            postmark_message_id=None,
            content_hash=line,
            from_address="",
            to_addresses=[],
            subject="",
            status=TaskStatus.COMPLETED,
            attempts=1,
            max_retries=2,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
            completed_at=datetime.utcnow(),
        )
        task_store.create_task(task)

    txt_path.rename(txt_path.with_suffix(".txt.migrated"))


def _task_to_doc(task: EmailTask) -> dict:
    return {
        "message_id": task.message_id,
        "postmark_message_id": task.postmark_message_id,
        "content_hash": task.content_hash,
        "from_address": task.from_address,
        "to_addresses": list(task.to_addresses),
        "subject": task.subject,
        "status": task.status.value,
        "attempts": task.attempts,
        "max_retries": task.max_retries,
        "workspace_path": task.workspace_path,
        "reply_message_id": task.reply_message_id,
        "last_error": task.last_error,
        "error_history": list(task.error_history),
        "created_at": task.created_at,
        "updated_at": task.updated_at,
        "completed_at": task.completed_at,
    }


def _doc_to_task(doc: Dict[str, Any]) -> EmailTask:
    return EmailTask(
        message_id=doc.get("message_id", doc.get("_id", "")),
        postmark_message_id=doc.get("postmark_message_id"),
        content_hash=doc.get("content_hash", ""),
        from_address=doc.get("from_address", ""),
        to_addresses=list(doc.get("to_addresses", [])),
        subject=doc.get("subject", ""),
        status=TaskStatus(doc.get("status", TaskStatus.PENDING.value)),
        attempts=int(doc.get("attempts", 0)),
        max_retries=int(doc.get("max_retries", 2)),
        workspace_path=doc.get("workspace_path"),
        reply_message_id=doc.get("reply_message_id"),
        last_error=doc.get("last_error"),
        error_history=list(doc.get("error_history", [])),
        created_at=doc.get("created_at", datetime.utcnow()),
        updated_at=doc.get("updated_at", datetime.utcnow()),
        completed_at=doc.get("completed_at"),
    )
