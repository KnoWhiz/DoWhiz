from __future__ import annotations

import re
from email.message import EmailMessage
from email.utils import getaddresses
from pathlib import Path
from typing import List, Tuple


_SANITIZE_RE = re.compile(r"[^a-zA-Z0-9._-]+")


def safe_filename(name: str, fallback: str = "attachment") -> str:
    if not name:
        return fallback
    cleaned = _SANITIZE_RE.sub("_", name).strip("._")
    return cleaned or fallback


def safe_message_id(message_id: str | None, fallback: str) -> str:
    if not message_id:
        return fallback
    cleaned = message_id.strip().lstrip("<").rstrip(">")
    cleaned = _SANITIZE_RE.sub("_", cleaned)
    return cleaned or fallback


def extract_addresses(value: str | None) -> List[str]:
    if not value:
        return []
    return [addr for _, addr in getaddresses([value]) if addr]


def extract_body_text(message: EmailMessage) -> Tuple[str, str]:
    text_body = ""
    html_body = ""

    if message.is_multipart():
        for part in message.walk():
            if part.get_content_disposition() == "attachment":
                continue
            if part.get_content_type() == "text/plain":
                text_body = part.get_content()
                break
        if not text_body:
            for part in message.walk():
                if part.get_content_type() == "text/html":
                    html_body = part.get_content()
                    text_body = strip_html(html_body)
                    break
    else:
        if message.get_content_type() == "text/plain":
            text_body = message.get_content()
        elif message.get_content_type() == "text/html":
            html_body = message.get_content()
            text_body = strip_html(html_body)

    return text_body.strip(), html_body.strip()


def strip_html(html: str) -> str:
    text = re.sub(r"<[^>]+>", " ", html)
    return re.sub(r"\s+", " ", text).strip()


def save_attachments(message: EmailMessage, dest_dir: Path) -> List[Path]:
    dest_dir.mkdir(parents=True, exist_ok=True)
    saved: List[Path] = []
    for part in message.iter_attachments():
        filename = safe_filename(part.get_filename() or "attachment")
        data = part.get_content()
        if isinstance(data, str):
            data = data.encode("utf-8")
        target = dest_dir / filename
        target.write_bytes(data)
        saved.append(target)
    return saved
