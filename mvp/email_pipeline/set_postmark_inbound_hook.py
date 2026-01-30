from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.request
from pathlib import Path


def load_env(path: Path) -> None:
    if not path.exists():
        return
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        os.environ.setdefault(key, value)


def main() -> int:
    parser = argparse.ArgumentParser(description="Set Postmark InboundHookUrl for the server")
    parser.add_argument("--hook-url", required=True, help="Public webhook URL (e.g. https://xxxx.ngrok-free.dev/postmark/inbound)")
    parser.add_argument("--env", default=None, help="Path to .env (defaults to repo .env)")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    env_path = Path(args.env) if args.env else (repo_root / ".env")
    load_env(env_path)

    token = os.getenv("POSTMARK_SERVER_TOKEN")
    if not token:
        print("POSTMARK_SERVER_TOKEN not set in .env", file=sys.stderr)
        return 1

    payload = json.dumps({"InboundHookUrl": args.hook_url}).encode("utf-8")
    req = urllib.request.Request(
        "https://api.postmarkapp.com/server",
        data=payload,
        method="PUT",
        headers={
            "X-Postmark-Server-Token": token,
            "Accept": "application/json",
            "Content-Type": "application/json",
        },
    )

    with urllib.request.urlopen(req, timeout=30) as resp:
        print("Set InboundHookUrl:", resp.status)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
