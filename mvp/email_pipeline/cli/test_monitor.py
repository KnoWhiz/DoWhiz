from __future__ import annotations

import argparse
from dataclasses import replace
from email.message import EmailMessage
from pathlib import Path
from typing import Dict

from ..config import load_settings
from ..email_utils import safe_message_id
from ..monitor import _process_incoming_email, process_incoming_email, start_monitor
from ..task_store import EmailTask, TaskStatus, TaskStore


class _MemoryTaskStore:
    def __init__(self) -> None:
        self.tasks: Dict[str, EmailTask] = {}

    def create_task(self, task: EmailTask) -> bool:
        if task.message_id in self.tasks:
            return False
        self.tasks[task.message_id] = task
        return True

    def get_task(self, message_id: str) -> EmailTask | None:
        return self.tasks.get(message_id)

    def mark_processing(self, message_id: str) -> bool:
        task = self.tasks.get(message_id)
        if not task or task.status in {TaskStatus.COMPLETED, TaskStatus.FAILED}:
            return False
        updated = replace(task, status=TaskStatus.PROCESSING, attempts=task.attempts + 1)
        self.tasks[message_id] = updated
        return True

    def mark_completed(self, message_id: str, reply_message_id: str, workspace_path: str) -> bool:
        task = self.tasks.get(message_id)
        if not task:
            return False
        updated = replace(task, status=TaskStatus.COMPLETED, reply_message_id=reply_message_id, workspace_path=workspace_path)
        self.tasks[message_id] = updated
        return True

    def mark_failed(self, message_id: str, error: str) -> bool:
        task = self.tasks.get(message_id)
        if not task:
            return False
        status = TaskStatus.PENDING if task.attempts <= task.max_retries else TaskStatus.FAILED
        updated = replace(task, status=status, last_error=error)
        self.tasks[message_id] = updated
        return True

    def reset_for_retry(self, message_id: str) -> bool:
        task = self.tasks.get(message_id)
        if not task:
            return False
        updated = replace(task, status=TaskStatus.PENDING, attempts=0)
        self.tasks[message_id] = updated
        return True


def _dry_run_responder(workspace_dir: str, model: str = "codex") -> dict:
    reply_path = Path(workspace_dir) / "email_reply.md"
    reply_path.write_text("DRY RUN reply.", encoding="utf-8")
    attachments_dir = Path(workspace_dir) / "email_reply_attachments"
    attachments_dir.mkdir(parents=True, exist_ok=True)
    return {
        "success": True,
        "reply_path": str(reply_path),
        "attachments_dir": str(attachments_dir),
        "error": None,
    }


def _dry_run_sender(**kwargs) -> dict:
    return {"success": True, "message_id": "<dry-run@local>", "error": None}


def _dry_run_process(raw_bytes: bytes) -> dict:
    settings = load_settings()
    store = _MemoryTaskStore()
    return _process_incoming_email(
        raw_email=raw_bytes,
        task_store=store,  # type: ignore[arg-type]
        settings=settings,
        max_retries=settings.max_retries,
        postmark_message_id=None,
        responder_fn=_dry_run_responder,
        sender_fn=_dry_run_sender,
        sleep_fn=lambda _: None,
    )


def _load_raw_email(eml_file: str) -> bytes:
    return Path(eml_file).read_bytes()


def _build_test_email(from_addr: str, to_addr: str, subject: str) -> bytes:
    msg = EmailMessage()
    msg["From"] = from_addr
    msg["To"] = to_addr
    msg["Subject"] = subject
    msg.set_content("Test email for monitor CLI.")
    return msg.as_bytes()


def _print_task(task: EmailTask) -> None:
    print("Message-ID:", task.message_id)
    print("Status:", task.status.value)
    print("Attempts:", task.attempts)
    print("Subject:", task.subject)
    print("From:", task.from_address)
    print("Workspace:", task.workspace_path)


def main() -> None:
    parser = argparse.ArgumentParser(description="CLI tests for email monitor")
    parser.add_argument("--start", action="store_true")
    parser.add_argument("--port", type=int, default=None)
    parser.add_argument("--simulate", action="store_true")
    parser.add_argument("--eml-file")
    parser.add_argument("--real", action="store_true")
    parser.add_argument("--status", action="store_true")
    parser.add_argument("--message-id")
    parser.add_argument("--list-processed", action="store_true")
    parser.add_argument("--limit", type=int, default=20)
    parser.add_argument("--retry", action="store_true")
    parser.add_argument("--e2e-test", action="store_true")
    parser.add_argument("--from", dest="from_addr")
    parser.add_argument("--to", dest="to_addr")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    settings = load_settings()

    if args.start:
        start_monitor(webhook_port=args.port or settings.monitor_webhook_port, max_retries=settings.max_retries)
        return

    if args.simulate:
        if not args.eml_file:
            raise SystemExit("--eml-file is required with --simulate")
        raw_bytes = _load_raw_email(args.eml_file)
        if args.real:
            result = process_incoming_email(raw_bytes, max_retries=settings.max_retries)
        else:
            result = _dry_run_process(raw_bytes)
        if args.verbose:
            print("Simulated raw bytes:", len(raw_bytes))
        print("Simulation result:", result)
        return

    if args.e2e_test:
        if not args.from_addr or not args.to_addr:
            raise SystemExit("--from and --to are required for --e2e-test")
        raw_bytes = _build_test_email(args.from_addr, args.to_addr, "IceBrew monitor E2E test")
        if args.real:
            result = process_incoming_email(raw_bytes, max_retries=settings.max_retries)
        else:
            result = _dry_run_process(raw_bytes)
        if args.verbose:
            print("E2E raw bytes:", len(raw_bytes))
        print("E2E result:", result)
        return

    if args.status:
        if not args.message_id:
            raise SystemExit("--message-id is required with --status")
        store = TaskStore(settings.mongodb_uri, settings.mongodb_db)
        task = store.get_task(args.message_id)
        if not task:
            print("Task not found.")
            return
        _print_task(task)
        return

    if args.list_processed:
        store = TaskStore(settings.mongodb_uri, settings.mongodb_db)
        tasks = store.get_recent_tasks(limit=args.limit)
        for task in tasks:
            print(f"{task.status.value}\t{task.message_id}\t{task.subject}")
        return

    if args.retry:
        if not args.message_id:
            raise SystemExit("--message-id is required with --retry")
        store = TaskStore(settings.mongodb_uri, settings.mongodb_db)
        task = store.get_task(args.message_id)
        if not task:
            raise SystemExit("Task not found.")

        if not store.reset_for_retry(args.message_id):
            raise SystemExit("Failed to reset task for retry.")

        workspace_path = task.workspace_path
        if not workspace_path:
            safe_id = safe_message_id(args.message_id, args.message_id)
            workspace_path = str(Path(settings.workspace_root) / safe_id)
        raw_path = Path(workspace_path) / "raw_email.eml"
        if not raw_path.exists():
            raise SystemExit(f"raw_email.eml not found at {raw_path}")

        raw_bytes = raw_path.read_bytes()
        if args.real:
            result = process_incoming_email(raw_bytes, max_retries=settings.max_retries)
        else:
            result = _dry_run_process(raw_bytes)
        print("Retry result:", result)
        return

    raise SystemExit("No action specified. Use --start, --simulate, --status, --list-processed, --retry, or --e2e-test.")


if __name__ == "__main__":
    main()
