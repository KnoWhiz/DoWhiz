#!/usr/bin/env python3
"""Small Bright Data helper for DoWhiz shared skills."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any


DATASETS = {
    "linkedin-person": "gd_l1viktl72bvl7bjuj0",
    "linkedin-company": "gd_l1vikfnt1wgvvqz95w",
    "linkedin-job": "gd_lpfll7v5hcqtkxl6l",
    "linkedin-post": "gd_lyy3tktm25m4avu764",
    "x-profile": "gd_lwxmeb2u1cniijd7t4",
    "x-post": "gd_lwxkxvnf1cynvib9co",
}

STATUS_URL = "https://api.brightdata.com/status"
DATASETS_URL = "https://api.brightdata.com/datasets/list"
DATASET_SCRAPE_URL = "https://api.brightdata.com/datasets/v3/scrape"
CUSTOM_SCRAPER_IMMEDIATE_URL = "https://api.brightdata.com/dca/trigger_immediate"


class BrightDataError(RuntimeError):
    """Raised for Bright Data request failures."""


def load_dotenv_fallback() -> None:
    """Load nearby .env files when the shell has not exported variables."""
    candidates: list[Path] = []
    for base in [Path.cwd(), *Path.cwd().parents]:
        candidates.append(base / ".env")
        candidates.append(base / "DoWhiz_service" / ".env")

    seen: set[Path] = set()
    for path in candidates:
        if path in seen or not path.is_file():
            continue
        seen.add(path)
        for raw_line in path.read_text(encoding="utf-8").splitlines():
            line = raw_line.strip()
            if not line or line.startswith("#"):
                continue
            if line.startswith("export "):
                line = line[len("export ") :]
            if "=" not in line:
                continue
            key, value = line.split("=", 1)
            key = key.strip()
            value = value.strip()
            if value and value[0] == value[-1] and value[0] in {"'", '"'}:
                value = value[1:-1]
            os.environ.setdefault(key, value)


def resolve_api_key() -> str:
    load_dotenv_fallback()
    key = os.environ.get("BRIGHT_DATA_API_KEY") or os.environ.get("BRIGHTDATA_API_KEY")
    if not key:
        raise BrightDataError(
            "Missing Bright Data credential. Set BRIGHT_DATA_API_KEY in the runtime env."
        )
    os.environ.setdefault("BRIGHTDATA_API_KEY", key)
    return key


def json_dumps(data: Any) -> bytes:
    return json.dumps(data, ensure_ascii=True).encode("utf-8")


def request_json(
    url: str,
    *,
    method: str = "GET",
    payload: Any | None = None,
    timeout: int = 300,
) -> Any:
    key = resolve_api_key()
    headers = {
        "Authorization": f"Bearer {key}",
        "Accept": "application/json",
    }
    body = None
    if payload is not None:
        headers["Content-Type"] = "application/json"
        body = json_dumps(payload)

    request = urllib.request.Request(url, data=body, headers=headers, method=method)
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            raw = response.read().decode("utf-8")
    except urllib.error.HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")
        raise BrightDataError(f"HTTP {exc.code} for {url}: {detail}") from exc
    except urllib.error.URLError as exc:
        raise BrightDataError(f"Request failed for {url}: {exc}") from exc

    try:
        return json.loads(raw)
    except json.JSONDecodeError:
        return {"raw_text": raw}


def ensure_output_parent(path: Path) -> None:
    if path.parent:
        path.parent.mkdir(parents=True, exist_ok=True)


def emit_output(result: Any, output_path: str | None) -> None:
    rendered = json.dumps(result, indent=2, ensure_ascii=True, sort_keys=False)
    if output_path:
        path = Path(output_path)
        ensure_output_parent(path)
        path.write_text(rendered + "\n", encoding="utf-8")
    else:
        sys.stdout.write(rendered)
        sys.stdout.write("\n")


def add_output_argument(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--output",
        help="Write formatted JSON output to a file instead of stdout.",
        default=argparse.SUPPRESS,
    )


def cmd_status(_: argparse.Namespace) -> Any:
    status = request_json(STATUS_URL)
    datasets = request_json(DATASETS_URL)
    notes: list[str] = []
    if not status.get("can_make_requests") and datasets:
        notes.append(
            "Status API reports proxy-zone limitations, but dataset APIs are reachable with this key."
        )
    return {
        "api_key_present": True,
        "status": status,
        "datasets_accessible": True,
        "dataset_count": len(datasets),
        "notes": notes,
    }


def cmd_datasets(args: argparse.Namespace) -> Any:
    datasets = request_json(DATASETS_URL)
    if not args.filter:
        return datasets

    pattern = re.compile(args.filter, re.IGNORECASE)
    return [
        item
        for item in datasets
        if pattern.search(item.get("name", "")) or pattern.search(item.get("id", ""))
    ]


def run_dataset(dataset_key: str, url: str) -> Any:
    dataset_id = DATASETS[dataset_key]
    query = urllib.parse.urlencode({"dataset_id": dataset_id, "format": "json"})
    endpoint = f"{DATASET_SCRAPE_URL}?{query}"
    return request_json(endpoint, method="POST", payload=[{"url": url}])


def cmd_dataset_url(args: argparse.Namespace) -> Any:
    return run_dataset(args.dataset_key, args.url)


def parse_payload(payload_json: str | None) -> dict[str, Any]:
    if not payload_json:
        return {}
    try:
        value = json.loads(payload_json)
    except json.JSONDecodeError as exc:
        raise BrightDataError(f"Invalid JSON payload: {exc}") from exc
    if not isinstance(value, dict):
        raise BrightDataError("Custom scraper payload must decode to a JSON object.")
    return value


def run_custom_scraper(
    *,
    collector_id: str | None,
    trigger_url: str | None,
    payload: dict[str, Any],
) -> Any:
    if trigger_url:
        url = trigger_url
    else:
        if not collector_id:
            raise BrightDataError(
                "Missing Xiaohongshu Bright Data config. Set BRIGHT_DATA_XIAOHONGSHU_COLLECTOR "
                "or BRIGHT_DATA_XIAOHONGSHU_TRIGGER_URL before calling this command."
            )
        query = urllib.parse.urlencode({"collector": collector_id})
        url = f"{CUSTOM_SCRAPER_IMMEDIATE_URL}?{query}"

    method = "POST" if payload else "GET"
    return request_json(url, method=method, payload=payload or None)


def cmd_custom_collector(args: argparse.Namespace) -> Any:
    payload = parse_payload(args.payload_json)
    if args.url:
        payload.setdefault("url", args.url)
    return run_custom_scraper(
        collector_id=args.collector_id,
        trigger_url=args.trigger_url,
        payload=payload,
    )


def cmd_xiaohongshu(args: argparse.Namespace) -> Any:
    payload = parse_payload(args.payload_json)
    if args.url:
        payload.setdefault("url", args.url)
    collector_id = args.collector_id or os.environ.get("BRIGHT_DATA_XIAOHONGSHU_COLLECTOR")
    trigger_url = args.trigger_url or os.environ.get("BRIGHT_DATA_XIAOHONGSHU_TRIGGER_URL")
    return run_custom_scraper(
        collector_id=collector_id,
        trigger_url=trigger_url,
        payload=payload,
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Bright Data social helper for DoWhiz shared skills."
    )
    add_output_argument(parser)
    subparsers = parser.add_subparsers(dest="command", required=True)

    status = subparsers.add_parser("status", help="Validate the Bright Data credential.")
    add_output_argument(status)
    status.set_defaults(handler=cmd_status)

    datasets = subparsers.add_parser("datasets", help="List available Bright Data datasets.")
    add_output_argument(datasets)
    datasets.add_argument(
        "--filter",
        help="Optional regex filter applied to dataset name and id.",
    )
    datasets.set_defaults(handler=cmd_datasets)

    for command_name in (
        "linkedin-person",
        "linkedin-company",
        "linkedin-job",
        "linkedin-post",
        "x-profile",
        "x-post",
    ):
        command = subparsers.add_parser(
            command_name,
            help=f"Run the Bright Data {command_name} dataset wrapper.",
        )
        add_output_argument(command)
        command.add_argument("--url", required=True, help="Public post/profile/company URL.")
        command.set_defaults(handler=cmd_dataset_url, dataset_key=command_name)

    custom = subparsers.add_parser(
        "custom-collector",
        help="Call a Bright Data custom scraper collector or deployed trigger URL.",
    )
    add_output_argument(custom)
    custom.add_argument("--collector-id", help="Bright Data custom scraper collector id.")
    custom.add_argument("--trigger-url", help="Full trigger URL for a deployed scraper.")
    custom.add_argument("--url", help="Convenience field added to the JSON payload.")
    custom.add_argument(
        "--payload-json",
        help="Optional JSON object payload passed to the custom scraper.",
    )
    custom.set_defaults(handler=cmd_custom_collector)

    xiaohongshu = subparsers.add_parser(
        "xiaohongshu",
        help="Run a Bright Data Xiaohongshu/RedNote custom scraper if configured.",
    )
    add_output_argument(xiaohongshu)
    xiaohongshu.add_argument("--collector-id", help="Override collector id for this call.")
    xiaohongshu.add_argument("--trigger-url", help="Override trigger URL for this call.")
    xiaohongshu.add_argument("--url", help="Convenience field added to the JSON payload.")
    xiaohongshu.add_argument(
        "--payload-json",
        help="Optional JSON object payload passed to the custom scraper.",
    )
    xiaohongshu.set_defaults(handler=cmd_xiaohongshu)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    try:
        result = args.handler(args)
    except BrightDataError as exc:
        print(f"bright-data-social: {exc}", file=sys.stderr)
        return 2
    emit_output(result, getattr(args, "output", None))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
