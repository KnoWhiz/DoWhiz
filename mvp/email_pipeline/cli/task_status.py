from __future__ import annotations

import argparse

from ..config import load_settings
from ..task_store import TaskStore


def _print_task(task) -> None:
    print("Message-ID:", task.message_id)
    print("Status:", task.status.value)
    print("Attempts:", task.attempts)
    print("From:", task.from_address)
    print("To:", ", ".join(task.to_addresses))
    print("Subject:", task.subject)
    print("Workspace:", task.workspace_path)
    print("Reply Message-ID:", task.reply_message_id)
    print("Last error:", task.last_error)


def main() -> None:
    parser = argparse.ArgumentParser(description="Query email task status in MongoDB")
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--limit", type=int, default=20)
    parser.add_argument("--get")
    parser.add_argument("--failed", action="store_true")
    parser.add_argument("--pending", action="store_true")
    parser.add_argument("--stats", action="store_true")
    parser.add_argument("--retry")
    parser.add_argument("--sender")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    settings = load_settings()
    store = TaskStore(settings.mongodb_uri, settings.mongodb_db)

    if args.stats:
        stats = store.get_stats()
        print("Stats:")
        for key, value in stats.items():
            print(f"  {key}: {value}")
        return

    if args.get:
        task = store.get_task(args.get)
        if not task:
            print("Task not found.")
            return
        _print_task(task)
        return

    if args.failed:
        tasks = store.get_failed_tasks(limit=args.limit)
        for task in tasks:
            if args.verbose:
                _print_task(task)
            else:
                print(f\"{task.status.value}\\t{task.message_id}\\t{task.subject}\")
        return

    if args.pending:
        tasks = store.get_pending_tasks(limit=args.limit)
        for task in tasks:
            if args.verbose:
                _print_task(task)
            else:
                print(f\"{task.status.value}\\t{task.message_id}\\t{task.subject}\")
        return

    if args.sender:
        tasks = store.get_tasks_by_sender(args.sender, limit=args.limit)
        for task in tasks:
            if args.verbose:
                _print_task(task)
            else:
                print(f\"{task.status.value}\\t{task.message_id}\\t{task.subject}\")
        return

    if args.retry:
        if store.reset_for_retry(args.retry):
            print("Task reset to pending:", args.retry)
        else:
            print("Failed to reset task:", args.retry)
        return

    if args.list:
        tasks = store.get_recent_tasks(limit=args.limit)
        for task in tasks:
            if args.verbose:
                _print_task(task)
            else:
                print(f\"{task.status.value}\\t{task.message_id}\\t{task.subject}\")
        return

    raise SystemExit("No action specified. Use --list, --get, --failed, --pending, --stats, --retry, or --sender.")


if __name__ == "__main__":
    main()
