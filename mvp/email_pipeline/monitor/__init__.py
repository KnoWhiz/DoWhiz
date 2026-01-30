from __future__ import annotations

import base64
import hashlib
import json
import logging
import threading
import time
from datetime import datetime
from email import policy
from email.message import EmailMessage
from email.parser import BytesParser
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Callable

from ..config import Settings, load_settings
from ..responder import generate_response
from ..sender import send_email
from ..storage import get_store
from ..task_store import EmailTask, TaskStatus, TaskStore, migrate_from_txt
from ..workspace import prepare_workspace


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")
logger = logging.getLogger("email_monitor")


class EmailMonitor:
    def __init__(self, task_store: TaskStore, settings: Settings, max_retries: int | None = None) -> None:
        self.task_store = task_store
        self.settings = settings
        self.max_retries = max_retries if max_retries is not None else settings.max_retries

    def handle_incoming_email(self, raw_email: bytes, postmark_message_id: str | None = None) -> dict:
        return _process_incoming_email(
            raw_email=raw_email,
            task_store=self.task_store,
            settings=self.settings,
            max_retries=self.max_retries,
            postmark_message_id=postmark_message_id,
        )


class _WebhookHandler(BaseHTTPRequestHandler):
    monitor: EmailMonitor

    def _safe_respond(self, status: int, body: bytes) -> None:
        try:
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(body)
        except BrokenPipeError:
            logger.warning("Client closed connection before response was sent.")

    def do_GET(self):  # noqa: N802
        if self.path in {"/", "/health"}:
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            self.end_headers()
            self.wfile.write(b"ok")
            return
        self.send_response(404)
        self.end_headers()

    def do_POST(self):  # noqa: N802
        if self.path not in {"/", "/postmark/inbound"}:
            self._safe_respond(404, b"{\"status\":\"not_found\"}")
            return

        length = int(self.headers.get("Content-Length", "0"))
        payload_bytes = self.rfile.read(length)

        try:
            payload = json.loads(payload_bytes.decode("utf-8"))
        except Exception:
            self._safe_respond(400, b"{\"status\":\"bad_json\"}")
            return

        def _run_pipeline() -> None:
            try:
                email_msg = _email_from_postmark(payload)
                raw_bytes = email_msg.as_bytes()
                postmark_message_id = payload.get("MessageID") or payload.get("MessageId")
                result = self.monitor.handle_incoming_email(raw_bytes, postmark_message_id=postmark_message_id)
                logger.info("Processed inbound webhook: %s", result)
            except Exception as exc:
                logger.exception("Failed to process inbound webhook: %s", exc)

        threading.Thread(target=_run_pipeline, daemon=True).start()
        self._safe_respond(200, b"{\"status\":\"accepted\"}")


def start_monitor(
    monitored_address: str = "mini-mouse@deep-tutor.com",
    webhook_port: int = 9000,
    max_retries: int = 2,
) -> None:
    """
    Start the email monitoring service.

    Listens for incoming emails via Postmark webhook and triggers
    the response pipeline for each new email.

    Args:
        monitored_address: Email address to monitor
        webhook_port: Port for webhook server
        max_retries: Maximum retry attempts for failed processing
    """
    settings = load_settings()
    task_store = TaskStore(settings.mongodb_uri, settings.mongodb_db)

    # Migrate legacy dedupe file if present.
    try:
        migrate_from_txt(settings.processed_ids_path, task_store)
    except Exception as exc:
        logger.warning("Migration from legacy dedupe file failed: %s", exc)

    monitor = EmailMonitor(task_store=task_store, settings=settings, max_retries=max_retries)

    _WebhookHandler.monitor = monitor
    server = ThreadingHTTPServer(("0.0.0.0", webhook_port), _WebhookHandler)
    logger.info("Postmark inbound webhook listening on %s", webhook_port)
    logger.info("Monitoring address: %s", monitored_address)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        logger.info("Shutting down.")
    finally:
        server.server_close()


def process_incoming_email(
    raw_email: bytes,
    max_retries: int = 2,
) -> dict:
    """
    Process a single incoming email through the full pipeline.

    Pipeline steps:
        1. prepare_workspace() - Create workspace from email
        2. generate_response() - Generate AI response
        3. send_email() - Send the response

    Retry logic:
        - If any step fails, retry up to max_retries times
        - After all retries exhausted, log error and do not retry again
        - Record failure in storage for later inspection

    Idempotency:
        - Check storage for Message-ID before processing
        - Skip emails that have already been replied to
        - Only process new emails (not duplicates)

    Args:
        raw_email: Raw email bytes
        max_retries: Maximum retry attempts (default: 2, meaning 3 total attempts)

    Returns:
        dict with keys:
            success (bool),
            message_id (str),
            workspace_path (str),
            reply_sent (bool),
            attempts (int),
            error (str|None)
    """
    settings = load_settings()
    task_store = TaskStore(settings.mongodb_uri, settings.mongodb_db)
    return _process_incoming_email(
        raw_email=raw_email,
        task_store=task_store,
        settings=settings,
        max_retries=max_retries,
        postmark_message_id=None,
    )


def _process_incoming_email(
    *,
    raw_email: bytes,
    task_store: TaskStore,
    settings: Settings,
    max_retries: int,
    postmark_message_id: str | None,
    responder_fn: Callable[[str, str], dict] = generate_response,
    sender_fn: Callable[..., dict] = send_email,
    sleep_fn: Callable[[float], None] = time.sleep,
) -> dict:
    msg = BytesParser(policy=policy.default).parsebytes(raw_email)
    header_message_id = msg.get("Message-ID") or ""
    content_hash = hashlib.sha256(raw_email).hexdigest()
    task_message_id = header_message_id or f"hash:{content_hash}"

    existing = task_store.get_task(task_message_id)
    if existing:
        if existing.status == TaskStatus.COMPLETED:
            return {
                "success": False,
                "message_id": task_message_id,
                "workspace_path": existing.workspace_path or "",
                "reply_sent": False,
                "attempts": existing.attempts,
                "error": "duplicate",
            }
        if existing.status == TaskStatus.PROCESSING:
            return {
                "success": False,
                "message_id": task_message_id,
                "workspace_path": existing.workspace_path or "",
                "reply_sent": False,
                "attempts": existing.attempts,
                "error": "already_processing",
            }
        if existing.status == TaskStatus.FAILED:
            return {
                "success": False,
                "message_id": task_message_id,
                "workspace_path": existing.workspace_path or "",
                "reply_sent": False,
                "attempts": existing.attempts,
                "error": "failed_previously",
            }

    workspace_info = prepare_workspace(raw_email, str(settings.workspace_root))
    if not workspace_info.get("success"):
        return {
            "success": False,
            "message_id": task_message_id,
            "workspace_path": "",
            "reply_sent": False,
            "attempts": 0,
            "error": workspace_info.get("error") or "workspace_error",
        }

    workspace_path = workspace_info["workspace_path"]
    workspace_message_id = workspace_info.get("message_id") or header_message_id or task_message_id

    if not existing:
        task = EmailTask(
            message_id=task_message_id,
            postmark_message_id=postmark_message_id,
            content_hash=content_hash,
            from_address=workspace_info.get("from_address", ""),
            to_addresses=workspace_info.get("to_addresses", []),
            subject=workspace_info.get("subject", ""),
            status=TaskStatus.PENDING,
            attempts=0,
            max_retries=max_retries,
            workspace_path=workspace_path,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
        )
        task_store.create_task(task)

    attempts = 0
    last_error: str | None = None
    for attempt in range(max_retries + 1):
        attempts = attempt + 1
        if not task_store.mark_processing(task_message_id):
            return {
                "success": False,
                "message_id": task_message_id,
                "workspace_path": workspace_path,
                "reply_sent": False,
                "attempts": attempts,
                "error": "task_not_pending",
            }

        try:
            response = responder_fn(workspace_path, "codex")
            if not response.get("success"):
                raise RuntimeError(response.get("error") or "responder_failed")

            reply_path = response["reply_path"]
            attachments_dir = response["attachments_dir"]

            reply_to_addresses = workspace_info.get("reply_to_addresses", [])
            from_address = workspace_info.get("from_address", "")
            reply_recipient = reply_to_addresses[0] if reply_to_addresses else from_address
            if not reply_recipient:
                raise RuntimeError("No reply recipient resolved.")

            subject = _normalize_subject(workspace_info.get("subject", ""))
            references = _build_references(
                workspace_info.get("references"),
                workspace_message_id,
            )

            send_result = sender_fn(
                from_address=settings.outbound_from,
                to_addresses=[reply_recipient],
                subject=subject,
                markdown_file_path=reply_path,
                attachments_dir_path=attachments_dir,
                reply_to_message_id=workspace_message_id,
                references=references,
            )
            if not send_result.get("success"):
                raise RuntimeError(send_result.get("error") or "send_failed")

            task_store.mark_completed(task_message_id, send_result.get("message_id", ""), workspace_path)

            _record_storage(settings, workspace_info, response)

            return {
                "success": True,
                "message_id": task_message_id,
                "workspace_path": workspace_path,
                "reply_sent": True,
                "attempts": attempts,
                "error": None,
            }
        except Exception as exc:
            last_error = str(exc)
            task_store.mark_failed(task_message_id, last_error)
            if attempt < max_retries:
                sleep_fn(5)
                continue
            break

    return {
        "success": False,
        "message_id": task_message_id,
        "workspace_path": workspace_path,
        "reply_sent": False,
        "attempts": attempts,
        "error": last_error,
    }


def _record_storage(settings: Settings, workspace_info: dict, response: dict) -> None:
    store = get_store(settings)
    if not store:
        return
    store.record_inbound(
        {
            "message_id": workspace_info.get("message_id"),
            "from": workspace_info.get("from_address"),
            "to": workspace_info.get("to_addresses"),
            "subject": workspace_info.get("subject"),
            "workspace": workspace_info.get("workspace_path"),
        }
    )
    store.record_outbound(
        {
            "in_reply_to": workspace_info.get("message_id"),
            "to": workspace_info.get("from_address"),
            "subject": _normalize_subject(workspace_info.get("subject", "")),
            "workspace": workspace_info.get("workspace_path"),
            "attachments": _list_attachments(response.get("attachments_dir")),
        }
    )


def _list_attachments(path: str | None) -> list[str]:
    if not path:
        return []
    attachments_dir = Path(path)
    if not attachments_dir.exists():
        return []
    return [p.name for p in attachments_dir.iterdir() if p.is_file()]


def _normalize_subject(subject: str) -> str:
    normalized = (subject or "").strip()
    if not normalized:
        return "Re:"
    if normalized.lower().startswith("re:"):
        return normalized
    return f"Re: {normalized}"


def _build_references(existing: str | None, message_id: str) -> str:
    if existing:
        return f"{existing} {message_id}".strip()
    return message_id


def _email_from_postmark(payload: dict) -> EmailMessage:
    msg = EmailMessage()

    from_value = payload.get("From") or ""
    to_value = payload.get("To") or ""
    cc_value = payload.get("Cc") or ""
    bcc_value = payload.get("Bcc") or ""

    if from_value:
        msg["From"] = from_value
    if to_value:
        msg["To"] = to_value
    if cc_value:
        msg["Cc"] = cc_value
    if bcc_value:
        msg["Bcc"] = bcc_value

    subject = payload.get("Subject") or ""
    if subject:
        msg["Subject"] = subject

    header_message_id = ""
    for header in payload.get("Headers", []) or []:
        name = (header.get("Name") or "").lower()
        if name == "message-id":
            header_message_id = (header.get("Value") or "").strip()
            break

    message_id = header_message_id or payload.get("MessageID") or payload.get("MessageId") or ""
    if message_id:
        msg_id = message_id.strip()
        if not msg_id.startswith("<"):
            msg_id = f"<{msg_id}>"
        msg["Message-ID"] = msg_id

    reply_to = payload.get("ReplyTo") or ""
    if reply_to:
        msg["Reply-To"] = reply_to

    existing = {h.lower() for h in msg.keys()}
    for header in payload.get("Headers", []) or []:
        name = header.get("Name")
        value = header.get("Value")
        if not name or value is None:
            continue
        lname = name.lower()
        if lname in existing:
            continue
        if lname == "message-id":
            continue
        msg[name] = value
        existing.add(lname)

    text_body = payload.get("TextBody") or payload.get("StrippedTextReply") or ""
    html_body = payload.get("HtmlBody") or ""

    if text_body and html_body:
        msg.set_content(text_body)
        msg.add_alternative(html_body, subtype="html")
    elif html_body:
        msg.add_alternative(html_body, subtype="html")
    else:
        msg.set_content(text_body or "")

    for attachment in payload.get("Attachments", []) or []:
        name = attachment.get("Name") or "attachment"
        content_type = attachment.get("ContentType") or "application/octet-stream"
        data_b64 = attachment.get("Content") or ""
        try:
            data = base64.b64decode(data_b64)
        except Exception:
            data = b""
        maintype, subtype = content_type.split("/", 1) if "/" in content_type else ("application", "octet-stream")
        msg.add_attachment(data, maintype=maintype, subtype=subtype, filename=name)

    return msg
