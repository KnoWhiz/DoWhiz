from __future__ import annotations

from pathlib import Path
from typing import Optional

from .config import Settings
from .responder import generate_response
from .sender import send_email
from .storage import MongoStore
from .workspace import prepare_workspace


def process_email(raw_bytes: bytes, settings: Settings, store: Optional[MongoStore]) -> Path:
    workspace_info = prepare_workspace(raw_bytes, str(settings.workspace_root))
    if not workspace_info.get("success"):
        raise RuntimeError(workspace_info.get("error") or "Failed to prepare workspace")

    workspace_path = Path(workspace_info["workspace_path"])

    response = generate_response(str(workspace_path))
    if not response.get("success"):
        raise RuntimeError(response.get("error") or "Failed to generate response")

    reply_to_addresses = workspace_info.get("reply_to_addresses", [])
    from_address = workspace_info.get("from_address", "")
    reply_recipient = reply_to_addresses[0] if reply_to_addresses else from_address
    if not reply_recipient:
        raise RuntimeError("No reply recipient resolved.")

    message_id = workspace_info.get("message_id")
    references = _build_references(workspace_info.get("references"), message_id)

    send_result = send_email(
        from_address=settings.outbound_from,
        to_addresses=[reply_recipient],
        subject=_normalize_subject(workspace_info.get("subject", "")),
        markdown_file_path=response["reply_path"],
        attachments_dir_path=response["attachments_dir"],
        reply_to_message_id=message_id,
        references=references,
    )

    if not send_result.get("success"):
        error_text = send_result.get("error") or "Failed to send"
        (workspace_path / "outbound_error.txt").write_text(error_text, encoding="utf-8")
        raise RuntimeError(error_text)

    if store:
        store.record_inbound(
            {
                "message_id": workspace_info.get("message_id"),
                "from": workspace_info.get("from_address"),
                "to": workspace_info.get("to_addresses"),
                "subject": workspace_info.get("subject"),
                "workspace": str(workspace_path),
            }
        )
        store.record_outbound(
            {
                "in_reply_to": workspace_info.get("message_id"),
                "to": reply_recipient,
                "subject": _normalize_subject(workspace_info.get("subject", "")),
                "workspace": str(workspace_path),
                "attachments": _list_attachments(response.get("attachments_dir")),
            }
        )

    return workspace_path


def _normalize_subject(subject: str) -> str:
    normalized = (subject or "").strip()
    if not normalized:
        return "Re:"
    if normalized.lower().startswith("re:"):
        return normalized
    return f"Re: {normalized}"


def _build_references(existing: str | None, message_id: str | None) -> str | None:
    if not message_id:
        return existing
    if existing:
        return f"{existing} {message_id}".strip()
    return message_id


def _list_attachments(path: str | None) -> list[str]:
    if not path:
        return []
    attachments_dir = Path(path)
    if not attachments_dir.exists():
        return []
    return [p.name for p in attachments_dir.iterdir() if p.is_file()]
