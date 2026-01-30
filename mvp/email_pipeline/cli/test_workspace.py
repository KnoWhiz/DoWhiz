from __future__ import annotations

import argparse
from pathlib import Path

from ..workspace import create_workspace_from_files, prepare_workspace


def _list_workspaces(root: Path) -> None:
    if not root.exists():
        print("Workspace root does not exist:", root)
        return
    dirs = [p for p in root.iterdir() if p.is_dir()]
    print("Workspaces:")
    for entry in sorted(dirs):
        print(" -", entry)


def _inspect_workspace(path: Path) -> None:
    if not path.exists():
        print("Workspace not found:", path)
        return
    print("Workspace:", path)
    for entry in sorted(path.rglob("*")):
        if entry.is_file():
            print(" -", entry.relative_to(path))


def main() -> None:
    parser = argparse.ArgumentParser(description="CLI tests for workspace manager")
    parser.add_argument("--eml-file")
    parser.add_argument("--inbox-md")
    parser.add_argument("--inbox-attachments")
    parser.add_argument("--workspace-root", required=True)
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--inspect")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    workspace_root = Path(args.workspace_root)

    if args.list:
        _list_workspaces(workspace_root)
        return

    if args.inspect:
        _inspect_workspace(Path(args.inspect))
        return

    if args.eml_file:
        raw_bytes = Path(args.eml_file).read_bytes()
        result = prepare_workspace(raw_bytes, str(workspace_root))
        if not result["success"]:
            raise SystemExit(f"Workspace creation failed: {result['error']}")
        print("Workspace created:", result["workspace_path"])
        if args.verbose:
            print("  Message-ID:", result["message_id"])
            print("  From:", result["from_address"])
            print("  Subject:", result["subject"])
        return

    if args.inbox_md:
        result = create_workspace_from_files(
            str(workspace_root),
            args.inbox_md,
            inbox_attachments_path=args.inbox_attachments,
        )
        if not result["success"]:
            raise SystemExit(f"Workspace creation failed: {result['error']}")
        print("Workspace created:", result["workspace_path"])
        return

    raise SystemExit("No action specified. Use --eml-file, --inbox-md, --list, or --inspect.")


if __name__ == "__main__":
    main()
