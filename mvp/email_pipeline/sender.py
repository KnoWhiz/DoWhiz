from __future__ import annotations

import base64
import html
import json
import mimetypes
import os
import urllib.request
from email.utils import make_msgid
from pathlib import Path
from typing import Iterable

from .config import load_settings
from .email_utils import safe_filename

try:  # Optional dependency for nicer HTML rendering.
    import markdown as md  # type: ignore
except Exception:  # pragma: no cover
    md = None


POSTMARK_SEND_URL = "https://api.postmarkapp.com/email"


def send_email(
    from_address: str,
    to_addresses: list[str],
    subject: str,
    markdown_file_path: str,
    attachments_dir_path: str | None = None,
    reply_to_message_id: str | None = None,
    references: str | None = None,
) -> dict:
    """
    Send an email with markdown content and optional attachments.

    Args:
        from_address: Sender email address
        to_addresses: List of recipient email addresses
        subject: Email subject line
        markdown_file_path: Path to markdown file containing email body
        attachments_dir_path: Path to folder containing attachments (optional)
        reply_to_message_id: Message-ID for email threading (optional)
        references: References header for email threading (optional)

    Returns:
        dict with keys: success (bool), message_id (str), error (str|None)
        - message_id is the RFC 5322 Message-ID used in the outbound headers
    """
    settings = load_settings()
    token = settings.postmark_token or os.getenv("POSTMARK_SERVER_TOKEN", "")
    if not token:
        return {"success": False, "message_id": "", "error": "POSTMARK_SERVER_TOKEN not set"}

    markdown_path = Path(markdown_file_path)
    if not markdown_path.exists():
        return {"success": False, "message_id": "", "error": f"Markdown file not found: {markdown_path}"}

    try:
        payload, message_id = _prepare_postmark_payload(
            from_address=from_address,
            to_addresses=to_addresses,
            subject=subject,
            markdown_text=markdown_path.read_text(encoding="utf-8", errors="replace"),
            attachments_dir=Path(attachments_dir_path) if attachments_dir_path else None,
            reply_to_message_id=reply_to_message_id,
            references=references,
        )
    except Exception as exc:
        return {"success": False, "message_id": "", "error": str(exc)}

    try:
        _postmark_send(payload, token)
    except Exception as exc:
        return {"success": False, "message_id": message_id, "error": str(exc)}

    return {"success": True, "message_id": message_id, "error": None}


def _prepare_postmark_payload(
    *,
    from_address: str,
    to_addresses: list[str],
    subject: str,
    markdown_text: str,
    attachments_dir: Path | None,
    reply_to_message_id: str | None,
    references: str | None,
) -> tuple[dict, str]:
    message_id = make_msgid(domain=_domain_from_address(from_address))
    text_body = markdown_text.strip() or "(no content)"
    html_body = _markdown_to_html(markdown_text)

    headers = [
        {"Name": "Message-ID", "Value": message_id},
    ]
    if reply_to_message_id:
        headers.append({"Name": "In-Reply-To", "Value": reply_to_message_id})
    if references:
        headers.append({"Name": "References", "Value": references})
    elif reply_to_message_id:
        headers.append({"Name": "References", "Value": reply_to_message_id})

    payload = {
        "From": from_address,
        "To": ", ".join([addr for addr in to_addresses if addr]),
        "Subject": subject,
        "TextBody": text_body,
        "HtmlBody": html_body,
        "Headers": headers,
    }

    attachments = _collect_attachments(attachments_dir)
    if attachments:
        payload["Attachments"] = attachments

    return payload, message_id


def _postmark_send(payload: dict, token: str) -> None:
    req = urllib.request.Request(
        POSTMARK_SEND_URL,
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


def _collect_attachments(attachments_dir: Path | None) -> list[dict]:
    if not attachments_dir or not attachments_dir.exists():
        return []
    attachments: list[dict] = []
    for path in _iter_files(attachments_dir):
        payload = path.read_bytes()
        content_type, _ = mimetypes.guess_type(path.name)
        attachments.append(
            {
                "Name": safe_filename(path.name),
                "Content": base64.b64encode(payload).decode("ascii"),
                "ContentType": content_type or "application/octet-stream",
            }
        )
    return attachments


def _markdown_to_html(markdown_text: str) -> str:
    if md is not None:
        try:
            return md.markdown(markdown_text, output_format="html5")
        except Exception:
            pass

    escaped = html.escape(markdown_text)
    blocks = [block for block in escaped.split("\n\n") if block.strip()]
    if not blocks:
        return ""
    html_blocks = [f"<p>{block.replace('\n', '<br />')}</p>" for block in blocks]
    return "\n".join(html_blocks)


def _iter_files(path: Path) -> Iterable[Path]:
    return sorted([entry for entry in path.iterdir() if entry.is_file()])


def _domain_from_address(address: str) -> str:
    if "@" in address:
        return address.split("@", 1)[-1]
    return "localhost"
