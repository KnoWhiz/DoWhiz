from __future__ import annotations

import argparse
from pathlib import Path

from ..responder import generate_response


def main() -> None:
    parser = argparse.ArgumentParser(description="CLI tests for response generator")
    parser.add_argument("--real", action="store_true", help="Call the AI model")
    parser.add_argument("--workspace", required=True)
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    workspace = Path(args.workspace)
    inbox = workspace / "email_inbox.md"
    if not inbox.exists():
        raise SystemExit(f"Missing email_inbox.md in workspace: {workspace}")

    if args.dry_run or not args.real:
        print("DRY RUN: responder")
        print("  Workspace:", workspace)
        print("  Inbox bytes:", len(inbox.read_bytes()))
        if args.verbose:
            attachments = workspace / "email_inbox_attachments"
            if attachments.exists():
                print("  Attachments:", [p.name for p in attachments.iterdir() if p.is_file()])
        return

    result = generate_response(str(workspace))
    if not result["success"]:
        raise SystemExit(f"Responder failed: {result['error']}")

    print("Generated response:")
    print("  Reply path:", result["reply_path"])
    print("  Attachments dir:", result["attachments_dir"])


if __name__ == "__main__":
    main()
