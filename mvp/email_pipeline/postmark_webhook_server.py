from __future__ import annotations

import argparse

from .config import load_settings
from .monitor import start_monitor


def main() -> None:
    parser = argparse.ArgumentParser(description="Postmark inbound webhook receiver")
    parser.add_argument("--port", type=int, default=None)
    args = parser.parse_args()

    settings = load_settings()
    start_monitor(
        monitored_address=settings.inbound_address,
        webhook_port=args.port or settings.monitor_webhook_port,
        max_retries=settings.max_retries,
    )


if __name__ == "__main__":
    main()
