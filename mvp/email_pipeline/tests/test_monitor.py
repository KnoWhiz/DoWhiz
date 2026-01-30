from __future__ import annotations

import tempfile
import unittest
from dataclasses import replace
from email.message import EmailMessage
from pathlib import Path

from mvp.email_pipeline.config import load_settings
from mvp.email_pipeline.monitor import _process_incoming_email
from mvp.email_pipeline.task_store import EmailTask, TaskStatus


class MemoryTaskStore:
    def __init__(self) -> None:
        self.tasks = {}

    def create_task(self, task: EmailTask) -> bool:
        if task.message_id in self.tasks:
            return False
        self.tasks[task.message_id] = task
        return True

    def get_task(self, message_id: str):
        return self.tasks.get(message_id)

    def mark_processing(self, message_id: str) -> bool:
        task = self.tasks.get(message_id)
        if not task or task.status in {TaskStatus.COMPLETED, TaskStatus.FAILED}:
            return False
        task.attempts += 1
        task.status = TaskStatus.PROCESSING
        return True

    def mark_completed(self, message_id: str, reply_message_id: str, workspace_path: str) -> bool:
        task = self.tasks.get(message_id)
        if not task:
            return False
        task.status = TaskStatus.COMPLETED
        task.reply_message_id = reply_message_id
        task.workspace_path = workspace_path
        return True

    def mark_failed(self, message_id: str, error: str) -> bool:
        task = self.tasks.get(message_id)
        if not task:
            return False
        task.last_error = error
        task.status = TaskStatus.PENDING if task.attempts <= task.max_retries else TaskStatus.FAILED
        return True


class MonitorTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        settings = load_settings()
        self.settings = replace(
            settings,
            workspace_root=Path(self.tempdir.name) / "workspaces",
            use_mongodb=False,
        )
        self.settings.workspace_root.mkdir(parents=True, exist_ok=True)
        self.store = MemoryTaskStore()

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def _build_email(self) -> bytes:
        msg = EmailMessage()
        msg["From"] = "sender@example.com"
        msg["To"] = "receiver@example.com"
        msg["Subject"] = "Hello"
        msg["Message-ID"] = "<monitor-test@example.com>"
        msg.set_content("Hello")
        return msg.as_bytes()

    def test_process_success(self) -> None:
        def responder(workspace_dir: str, model: str = "codex") -> dict:
            reply_path = Path(workspace_dir) / "email_reply.md"
            reply_path.write_text("Reply", encoding="utf-8")
            attachments_dir = Path(workspace_dir) / "email_reply_attachments"
            attachments_dir.mkdir(parents=True, exist_ok=True)
            return {"success": True, "reply_path": str(reply_path), "attachments_dir": str(attachments_dir), "error": None}

        def sender_fn(**_kwargs):
            return {"success": True, "message_id": "<reply@example.com>", "error": None}

        result = _process_incoming_email(
            raw_email=self._build_email(),
            task_store=self.store,  # type: ignore[arg-type]
            settings=self.settings,
            max_retries=1,
            postmark_message_id=None,
            responder_fn=responder,
            sender_fn=sender_fn,
            sleep_fn=lambda _: None,
        )

        self.assertTrue(result["success"])
        task = self.store.get_task("<monitor-test@example.com>")
        self.assertIsNotNone(task)
        self.assertEqual(task.status, TaskStatus.COMPLETED)

    def test_process_failure(self) -> None:
        def responder(_workspace_dir: str, _model: str = "codex") -> dict:
            return {"success": False, "reply_path": "", "attachments_dir": "", "error": "boom"}

        result = _process_incoming_email(
            raw_email=self._build_email(),
            task_store=self.store,  # type: ignore[arg-type]
            settings=self.settings,
            max_retries=1,
            postmark_message_id=None,
            responder_fn=responder,
            sender_fn=lambda **_kwargs: {"success": True, "message_id": "<reply@example.com>", "error": None},
            sleep_fn=lambda _: None,
        )

        self.assertFalse(result["success"])
        task = self.store.get_task("<monitor-test@example.com>")
        self.assertEqual(task.status, TaskStatus.FAILED)

    def test_duplicate_completed(self) -> None:
        task = EmailTask(
            message_id="<monitor-test@example.com>",
            postmark_message_id=None,
            content_hash="hash",
            from_address="sender@example.com",
            to_addresses=["receiver@example.com"],
            subject="Hello",
            status=TaskStatus.COMPLETED,
            attempts=1,
            max_retries=1,
        )
        self.store.create_task(task)

        result = _process_incoming_email(
            raw_email=self._build_email(),
            task_store=self.store,  # type: ignore[arg-type]
            settings=self.settings,
            max_retries=1,
            postmark_message_id=None,
            responder_fn=lambda *_args, **_kwargs: {"success": True, "reply_path": "", "attachments_dir": "", "error": None},
            sender_fn=lambda **_kwargs: {"success": True, "message_id": "<reply@example.com>", "error": None},
            sleep_fn=lambda _: None,
        )
        self.assertEqual(result["error"], "duplicate")


if __name__ == "__main__":
    unittest.main()
