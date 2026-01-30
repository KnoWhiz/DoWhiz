from __future__ import annotations

import argparse
from pathlib import Path

from ..sender import _prepare_postmark_payload, send_email


def main() -> None:
    parser = argparse.ArgumentParser(description="CLI tests for email sender")
    parser.add_argument("--real", action="store_true", help="Send a real email via Postmark")
    parser.add_argument("--from", dest="from_addr", required=True)
    parser.add_argument("--to", dest="to_addrs", required=True)
    parser.add_argument("--subject", required=True)
    parser.add_argument("--markdown-file", required=True)
    parser.add_argument("--attachments-dir")
    parser.add_argument("--reply-to", dest="reply_to")
    parser.add_argument("--references")
    parser.add_argument("--verbose", action="store_true")
    args = parser.parse_args()

    to_addresses = [addr.strip() for addr in args.to_addrs.split(",") if addr.strip()]

    if not args.real:
        markdown_text = Path(args.markdown_file).read_text(encoding="utf-8", errors="replace")
        payload, message_id = _prepare_postmark_payload(
            from_address=args.from_addr,
            to_addresses=to_addresses,
            subject=args.subject,
            markdown_text=markdown_text,
            attachments_dir=Path(args.attachments_dir) if args.attachments_dir else None,
            reply_to_message_id=args.reply_to,
            references=args.references,
        )
        print("DRY RUN: prepared Postmark payload")
        print("  Message-ID:", message_id)
        print("  To:", payload.get("To"))
        print("  Subject:", payload.get("Subject"))
        if args.verbose:
            print("  Payload keys:", sorted(payload.keys()))
        return

    result = send_email(
        from_address=args.from_addr,
        to_addresses=to_addresses,
        subject=args.subject,
        markdown_file_path=args.markdown_file,
        attachments_dir_path=args.attachments_dir,
        reply_to_message_id=args.reply_to,
        references=args.references,
    )
    if result["success"]:
        print("Sent email.")
        print("  Message-ID:", result["message_id"])
    else:
        raise SystemExit(f"Send failed: {result['error']}")


if __name__ == "__main__":
    main()
