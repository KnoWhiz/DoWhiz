from __future__ import annotations

import argparse
from email import policy
from email.parser import BytesParser
from pathlib import Path

from .config import load_settings


def main() -> None:
    parser = argparse.ArgumentParser(description="Inspect captured outbound emails")
    parser.add_argument("--outbox", default=None)
    args = parser.parse_args()

    settings = load_settings()
    outbox_dir = Path(args.outbox) if args.outbox else settings.outbox_dir

    if not outbox_dir.exists():
        print("Outbox directory not found:", outbox_dir)
        return

    files = sorted([p for p in outbox_dir.iterdir() if p.is_file()])
    if not files:
        print("No outbound emails captured yet in", outbox_dir)
        return

    latest = files[-1]
    msg = BytesParser(policy=policy.default).parsebytes(latest.read_bytes())

    print("Latest outbound email:")
    print("  File:", latest)
    print("  To:", msg.get("To"))
    print("  Subject:", msg.get("Subject"))
    print("  In-Reply-To:", msg.get("In-Reply-To"))
    print("  References:", msg.get("References"))


if __name__ == "__main__":
    main()
