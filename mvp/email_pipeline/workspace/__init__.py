from __future__ import annotations

import json
import time
from email import policy
from email.message import Message
from email.parser import BytesParser
from email.utils import make_msgid
from pathlib import Path

from ..email_utils import extract_addresses, extract_body_text, safe_message_id, save_attachments


def prepare_workspace(
    raw_email: bytes | Message,
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
            reply_to_addresses (list[str]),
            to_addresses (list[str]),
            subject (str),
            in_reply_to (str|None),
            references (str|None),
            success (bool),
            error (str|None)
    """
    workspace_root_path = Path(workspace_root)

    try:
        if isinstance(raw_email, (bytes, bytearray)):
            msg = BytesParser(policy=policy.default).parsebytes(bytes(raw_email))
            raw_bytes = bytes(raw_email)
        elif isinstance(raw_email, Message):
            msg = raw_email
            raw_bytes = msg.as_bytes()
        else:
            raise TypeError("raw_email must be bytes or email.message.Message")
    except Exception as exc:
        return {
            "workspace_path": "",
            "message_id": "",
            "from_address": "",
            "reply_to_addresses": [],
            "to_addresses": [],
            "subject": "",
            "in_reply_to": None,
            "references": None,
            "success": False,
            "error": str(exc),
        }

    message_id = msg.get("Message-ID") or make_msgid()
    safe_id = safe_message_id(message_id, f"email_{int(time.time())}")
    workspace_path = workspace_root_path / safe_id
    workspace_path.mkdir(parents=True, exist_ok=True)

    raw_path = workspace_path / "raw_email.eml"
    raw_path.write_bytes(raw_bytes)

    text_body, html_body = extract_body_text(msg)
    body = text_body or html_body
    inbox_path = workspace_path / "email_inbox.md"
    inbox_path.write_text(body, encoding="utf-8")

    attachments_dir = workspace_path / "email_inbox_attachments"
    save_attachments(msg, attachments_dir)

    from_addresses = extract_addresses(msg.get("From"))
    reply_to_addresses = extract_addresses(msg.get("Reply-To"))
    to_addresses = extract_addresses(msg.get("To"))

    return {
        "workspace_path": str(workspace_path),
        "message_id": message_id,
        "from_address": from_addresses[0] if from_addresses else "",
        "reply_to_addresses": reply_to_addresses,
        "to_addresses": to_addresses,
        "subject": msg.get("Subject", ""),
        "in_reply_to": msg.get("In-Reply-To"),
        "references": msg.get("References"),
        "success": True,
        "error": None,
    }


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
    workspace_root_path = Path(workspace_root)
    inbox_path = Path(inbox_md_path)
    if not inbox_path.exists():
        return {"workspace_path": "", "success": False, "error": f"Missing inbox file: {inbox_path}"}

    message_id = (metadata or {}).get("message_id") or make_msgid()
    safe_id = safe_message_id(str(message_id), f"manual_{int(time.time())}")
    workspace_path = workspace_root_path / safe_id
    workspace_path.mkdir(parents=True, exist_ok=True)

    target_inbox = workspace_path / "email_inbox.md"
    target_inbox.write_text(inbox_path.read_text(encoding="utf-8", errors="replace"), encoding="utf-8")

    attachments_dir = workspace_path / "email_inbox_attachments"
    attachments_dir.mkdir(parents=True, exist_ok=True)
    if inbox_attachments_path:
        source_dir = Path(inbox_attachments_path)
        if source_dir.exists():
            for item in source_dir.iterdir():
                if item.is_file():
                    (attachments_dir / item.name).write_bytes(item.read_bytes())

    if metadata:
        metadata_path = workspace_path / "metadata.json"
        metadata_path.write_text(json.dumps(metadata, indent=2), encoding="utf-8")

    return {"workspace_path": str(workspace_path), "success": True, "error": None}
