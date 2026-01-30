from __future__ import annotations

import argparse
import asyncio
import logging

from aiosmtpd.controller import Controller

from .config import Settings, load_settings
from .pipeline import process_email
from .storage import MongoStore, get_store


logging.basicConfig(level=logging.INFO, format="[%(asctime)s] %(levelname)s %(message)s")
logger = logging.getLogger("email_pipeline")


class InboundHandler:
    def __init__(self, settings: Settings, store: MongoStore | None) -> None:
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


def _start_controller(handler, hostname: str, port: int) -> Controller:
    controller = Controller(handler, hostname=hostname, port=port)
    controller.start()
    return controller


def main() -> None:
    parser = argparse.ArgumentParser(description="IceBrew SMTP ingress (legacy)")
    parser.add_argument("--inbound-host", default=None)
    parser.add_argument("--inbound-port", type=int, default=None)
    args = parser.parse_args()

    settings = load_settings()
    inbound_host = args.inbound_host or settings.inbound_host
    inbound_port = args.inbound_port or settings.inbound_port

    store = get_store(settings)

    inbound_handler = InboundHandler(settings, store)
    inbound_controller = _start_controller(inbound_handler, inbound_host, inbound_port)
    logger.info("Inbound SMTP listening on %s:%s", inbound_host, inbound_port)

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


if __name__ == "__main__":
    main()
