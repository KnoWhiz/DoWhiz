from __future__ import annotations

import argparse
import base64
import hashlib
import json
import logging
import threading
from email.message import EmailMessage
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Optional

from .config import Settings, load_settings
from .pipeline import process_email
from .storage import MongoStore, get_store


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")
logger = logging.getLogger("postmark_webhook")


class ProcessedMessageStore:
    def __init__(self, path: Path) -> None:
        self.path = path
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self._lock = threading.Lock()
        self._seen: set[str] = set()
        if self.path.exists():
            for raw in self.path.read_text(encoding="utf-8").splitlines():
                line = raw.strip()
                if line:
                    self._seen.add(line)

    def mark_if_new(self, ids: list[str]) -> bool:
        candidates = [item for item in ids if item]
        if not candidates:
            return True
        with self._lock:
            if any(item in self._seen for item in candidates):
                return False
            with self.path.open("a", encoding="utf-8") as handle:
                for item in candidates:
                    self._seen.add(item)
                    handle.write(item + "\n")
        return True


def _extract_message_ids(payload: dict, raw_payload: bytes) -> list[str]:
    header_message_id = ""
    for header in payload.get("Headers", []) or []:
        name = (header.get("Name") or "").lower()
        if name == "message-id":
            header_message_id = (header.get("Value") or "").strip()
            break
    postmark_message_id = (payload.get("MessageID") or payload.get("MessageId") or "").strip()
    fallback_id = hashlib.md5(raw_payload).hexdigest()
    return [header_message_id, postmark_message_id, fallback_id]


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

    # Prefer the original Message-ID header (best for threading) if present.
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


class PostmarkWebhookHandler(BaseHTTPRequestHandler):
    settings: Settings
    store: Optional[MongoStore]
    dedupe_store: ProcessedMessageStore

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

        message_ids = _extract_message_ids(payload, payload_bytes)
        if not self.dedupe_store.mark_if_new(message_ids):
            self._safe_respond(200, b"{\"status\":\"duplicate\"}")
            return

        def _run_pipeline() -> None:
            try:
                email_msg = _email_from_postmark(payload)
                raw_bytes = email_msg.as_bytes()
                workspace = process_email(raw_bytes, self.settings, self.store)
                logger.info("Processed inbound webhook into workspace %s", workspace)
            except Exception as exc:
                logger.exception("Failed to process inbound webhook: %s", exc)

        threading.Thread(target=_run_pipeline, daemon=True).start()
        self._safe_respond(200, b"{\"status\":\"accepted\"}")


def main() -> None:
    parser = argparse.ArgumentParser(description="Postmark inbound webhook receiver")
    parser.add_argument("--host", default="0.0.0.0")
    parser.add_argument("--port", type=int, default=9000)
    args = parser.parse_args()

    settings = load_settings()
    store = get_store(settings)

    PostmarkWebhookHandler.settings = settings
    PostmarkWebhookHandler.store = store
    PostmarkWebhookHandler.dedupe_store = ProcessedMessageStore(
        settings.processed_ids_path
    )

    server = ThreadingHTTPServer((args.host, args.port), PostmarkWebhookHandler)
    logger.info("Postmark inbound webhook listening on %s:%s", args.host, args.port)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        logger.info("Shutting down.")
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
