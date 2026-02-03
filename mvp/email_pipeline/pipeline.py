from __future__ import annotations

import shutil
import time
from email import policy
from email.parser import BytesParser
from email.utils import make_msgid
from pathlib import Path
from typing import Optional

from .codex_runner import run_codex_reply
from .config import Settings
from .email_utils import (
    extract_addresses,
    extract_body_text,
    safe_message_id,
    save_attachments,
)
from .mailer import build_reply_message, send_via_postmark, send_via_smtp
from .storage import MongoStore


def process_email(raw_bytes: bytes, settings: Settings, store: Optional[MongoStore]) -> Path:
    msg = BytesParser(policy=policy.default).parsebytes(raw_bytes)

    message_id = msg.get("Message-ID") or make_msgid()
    subject = msg.get("Subject", "")
    from_addr = extract_addresses(msg.get("From"))
    reply_to_addr = extract_addresses(msg.get("Reply-To"))
    to_addr = extract_addresses(msg.get("To"))
    references = msg.get("References")
    in_reply_to = msg.get("In-Reply-To")

    text_body, html_body = extract_body_text(msg)

    workspace_name = safe_message_id(message_id, f"email_{int(time.time())}")
    workspace = settings.workspace_root / workspace_name
    workspace.mkdir(parents=True, exist_ok=True)

    raw_path = workspace / "raw_email.eml"
    raw_path.write_bytes(raw_bytes)

    incoming_dir = workspace / "incoming_attachments"
    incoming_paths = save_attachments(msg, incoming_dir)

    reply_path = workspace / "email_reply.md"
    reply_attachments_dir = workspace / "email_reply_attachments"
    reply_attachments_dir.mkdir(parents=True, exist_ok=True)

    prompt = _build_codex_prompt(
        from_addr=from_addr[0] if from_addr else "",
        to_addr=to_addr,
        subject=subject,
        body=text_body or html_body,
        attachment_names=[p.name for p in incoming_paths],
    )

    reply_text = run_codex_reply(
        prompt,
        workspace_dir=workspace,
        reply_path=reply_path,
        model_name=settings.code_model,
        codex_disabled=settings.codex_disabled,
    )

    if settings.echo_attachments and not list(reply_attachments_dir.iterdir()):
        for path in incoming_paths:
            shutil.copy2(path, reply_attachments_dir / path.name)

    if references:
        reply_references = f"{references} {message_id}".strip()
    else:
        reply_references = message_id

    reply_recipient = reply_to_addr[0] if reply_to_addr else (from_addr[0] if from_addr else "")

    reply_message = build_reply_message(
        settings=settings,
        to_address=reply_recipient,
        subject=subject,
        body_text=reply_text,
        in_reply_to=message_id,
        references=reply_references,
        attachments_dir=reply_attachments_dir,
    )

    outbound_error: str | None = None
    try:
        if settings.outbound_mode == "postmark":
            send_via_postmark(reply_message, reply_attachments_dir, settings)
        else:
            send_via_smtp(reply_message, settings.outbound_host, settings.outbound_port)
    except Exception as exc:
        outbound_error = str(exc)
        (workspace / "outbound_error.txt").write_text(outbound_error, encoding="utf-8")
        # If Postmark blocks a recipient (4xx), don't retry the webhook endlessly.
        if settings.outbound_mode == "postmark" and outbound_error.startswith("Postmark error 4"):
            pass
        else:
            raise

    if store:
        store.record_inbound(
            {
                "message_id": message_id,
                "from": from_addr,
                "to": to_addr,
                "subject": subject,
                "text_body": text_body,
                "html_body": html_body,
                "workspace": str(workspace),
                "attachments": [p.name for p in incoming_paths],
            }
        )
        store.record_outbound(
            {
                "in_reply_to": message_id,
                "to": from_addr,
                "subject": reply_message["Subject"],
                "workspace": str(workspace),
                "attachments": [p.name for p in reply_attachments_dir.iterdir()],
            }
        )

    return workspace


def _build_codex_prompt(
    *,
    from_addr: str,
    to_addr: list[str],
    subject: str,
    body: str,
    attachment_names: list[str],
) -> str:
    attachments_line = ", ".join(attachment_names) if attachment_names else "(none)"
    to_line = ", ".join(to_addr) if to_addr else ""

    return (
        "You are the IceBrew email agent.\n"
        "Write a helpful reply to the incoming email.\n"
        "You must write the reply to a file named email_reply.md in the current directory.\n"
        "If you create any files to send back, place them in email_reply_attachments/.\n"
        "Do not include anything else outside the reply text in email_reply.md.\n\n"
        f"From: {from_addr}\n"
        f"To: {to_line}\n"
        f"Subject: {subject}\n"
        f"Attachments: {attachments_line}\n\n"
        f"Body:\n{body}\n"
    )
