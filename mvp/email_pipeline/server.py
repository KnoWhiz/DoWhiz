from __future__ import annotations

import argparse
import asyncio
import logging
from datetime import datetime
from email.parser import BytesParser
from email import policy
from pathlib import Path
from typing import Optional

from aiosmtpd.controller import Controller

from .config import Settings, load_settings
from .pipeline import process_email
from .storage import MongoStore, get_store


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")
logger = logging.getLogger("email_pipeline")


class InboundHandler:
    def __init__(self, settings: Settings, store: Optional[MongoStore]) -> None:
        self.settings = settings
        self.store = store

    async def handle_DATA(self, server, session, envelope):  # type: ignore[override]
        raw_bytes = envelope.original_content or envelope.content
        logger.info("Inbound email from %s to %s", envelope.mail_from, envelope.rcpt_tos)
        try:
            loop = asyncio.get_running_loop()
            workspace = await loop.run_in_executor(
                None, process_email, raw_bytes, self.settings, self.store
            )
            logger.info("Processed email into workspace %s", workspace)
        except Exception as exc:
            logger.exception("Failed to process inbound email: %s", exc)
            return "451 Processing failed"
        return "250 OK"


class OutboxHandler:
    def __init__(self, outbox_dir: Path) -> None:
        self.outbox_dir = outbox_dir
        self.outbox_dir.mkdir(parents=True, exist_ok=True)

    async def handle_DATA(self, server, session, envelope):  # type: ignore[override]
        raw_bytes = envelope.original_content or envelope.content
        timestamp = datetime.utcnow().strftime("%Y%m%dT%H%M%SZ")
        filename = f"outbound_{timestamp}.eml"
        path = self.outbox_dir / filename
        path.write_bytes(raw_bytes)

        msg = BytesParser(policy=policy.default).parsebytes(raw_bytes)
        logger.info(
            "Captured outbound email to %s subject=%s file=%s",
            msg.get("To"),
            msg.get("Subject"),
            path,
        )
        return "250 OK"


def _start_controller(handler, hostname: str, port: int) -> Controller:
    controller = Controller(handler, hostname=hostname, port=port)
    controller.start()
    return controller


def main() -> None:
    parser = argparse.ArgumentParser(description="IceBrew local email pipeline server")
    parser.add_argument("--inbound-host", default=None)
    parser.add_argument("--inbound-port", type=int, default=None)
    parser.add_argument("--outbound-host", default=None)
    parser.add_argument("--outbound-port", type=int, default=None)
    args = parser.parse_args()

    settings = load_settings()
    inbound_host = args.inbound_host or settings.inbound_host
    inbound_port = args.inbound_port or settings.inbound_port
    outbound_host = args.outbound_host or settings.outbound_host
    outbound_port = args.outbound_port or settings.outbound_port

    store = get_store(settings)

    inbound_handler = InboundHandler(settings, store)
    inbound_controller = _start_controller(inbound_handler, inbound_host, inbound_port)
    logger.info("Inbound SMTP listening on %s:%s", inbound_host, inbound_port)

    outbox_controller = None
    if settings.outbound_mode == "smtp" and settings.start_outbox_server:
        outbox_handler = OutboxHandler(settings.outbox_dir)
        outbox_controller = _start_controller(outbox_handler, outbound_host, outbound_port)
        logger.info("Outbox SMTP sink listening on %s:%s", outbound_host, outbound_port)

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    try:
        loop.run_forever()
    except KeyboardInterrupt:
        logger.info("Shutting down.")
    finally:
        loop.stop()
        loop.close()
        inbound_controller.stop()
        if outbox_controller:
            outbox_controller.stop()


if __name__ == "__main__":
    main()
