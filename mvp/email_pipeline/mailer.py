from __future__ import annotations

import base64
import json
import mimetypes
import smtplib
import urllib.request
from email.message import EmailMessage
from email.utils import make_msgid
from pathlib import Path
from typing import Iterable

from .config import Settings


def build_reply_message(
    *,
    settings: Settings,
    to_address: str,
    subject: str,
    body_text: str,
    in_reply_to: str | None,
    references: str | None,
    attachments_dir: Path,
) -> EmailMessage:
    msg = EmailMessage()
    msg["From"] = settings.outbound_from
    msg["To"] = to_address

    normalized_subject = subject.strip() if subject else ""
    if normalized_subject.lower().startswith("re:"):
        msg["Subject"] = normalized_subject
    else:
        msg["Subject"] = f"Re: {normalized_subject}" if normalized_subject else "Re:"

    msg["Message-ID"] = make_msgid(domain=settings.outbound_from.split("@")[-1])
    if in_reply_to:
        msg["In-Reply-To"] = in_reply_to
    if references:
        msg["References"] = references
    elif in_reply_to:
        msg["References"] = in_reply_to

    msg.set_content(body_text or "(no content)")

    for attachment_path in _iter_attachments(attachments_dir):
        data = attachment_path.read_bytes()
        content_type, _ = mimetypes.guess_type(attachment_path.name)
        maintype, subtype = (content_type or "application/octet-stream").split("/", 1)
        msg.add_attachment(data, maintype=maintype, subtype=subtype, filename=attachment_path.name)

    return msg


def send_via_smtp(message: EmailMessage, host: str, port: int) -> None:
    with smtplib.SMTP(host, port) as smtp:
        smtp.send_message(message)


def send_via_postmark(message: EmailMessage, attachments_dir: Path, settings: Settings) -> None:
    token = settings.postmark_token
    if not token:
        raise RuntimeError("POSTMARK_SERVER_TOKEN not set")

    attachments = []
    for attachment_path in _iter_attachments(attachments_dir):
        payload = attachment_path.read_bytes()
        content_type, _ = mimetypes.guess_type(attachment_path.name)
        attachments.append(
            {
                "Name": attachment_path.name,
                "Content": base64.b64encode(payload).decode("ascii"),
                "ContentType": content_type or "application/octet-stream",
            }
        )

    body_part = message.get_body(preferencelist=("plain",))
    text_body = body_part.get_content() if body_part else message.get_content()

    payload = {
        "From": message["From"],
        "To": message["To"],
        "Subject": message["Subject"],
        "TextBody": text_body,
        "Headers": [],
    }

    if message.get("In-Reply-To"):
        payload["Headers"].append({"Name": "In-Reply-To", "Value": message["In-Reply-To"]})
    if message.get("References"):
        payload["Headers"].append({"Name": "References", "Value": message["References"]})
    if attachments:
        payload["Attachments"] = attachments

    req = urllib.request.Request(
        "https://api.postmarkapp.com/email",
        data=json.dumps(payload).encode("utf-8"),
        headers={
            "Accept": "application/json",
            "Content-Type": "application/json",
            "X-Postmark-Server-Token": token,
        },
        method="POST",
    )

    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            if resp.status >= 400:
                raise RuntimeError(f"Postmark error: {resp.status} {resp.read().decode('utf-8')}")
    except urllib.error.HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"Postmark error {exc.code}: {detail}") from exc


def _iter_attachments(attachments_dir: Path) -> Iterable[Path]:
    if not attachments_dir.exists():
        return []
    return sorted([p for p in attachments_dir.iterdir() if p.is_file()])
