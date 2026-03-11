#!/usr/bin/env python3
"""Human approval gate for 2FA flows.

This CLI sends an approval email and optionally blocks while polling Postmark inbound
messages for a reply in the same challenge thread.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
from uuid import uuid4

API_BASE_DEFAULT = "https://api.postmarkapp.com"
STATE_DIR_DEFAULT = ".human_approval_gate/challenges"
DEFAULT_TIMEOUT_MINUTES = 30
DEFAULT_POLL_SECONDS = 15
DEFAULT_ADMIN_RECIPIENT = "admin@dowhiz.com"
SUBJECT_TOKEN_PREFIX = "HAG"
MAX_REPLY_SNIPPET_CHARS = 2000
# Postmark limits metadata key names to at most 20 characters.
POSTMARK_METADATA_CHALLENGE_ID_KEY = "hag_challenge_id"
POSTMARK_METADATA_SCOPE_KEY = "hag_scope"

EMAIL_PATTERN = re.compile(r"([A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,})")
CODE_PATTERNS = [
    re.compile(
        r"(?:code|otp|passcode|verification(?:\s+code)?)\D{0,12}([A-Za-z0-9-]{4,12})",
        re.IGNORECASE,
    ),
    re.compile(r"\b(\d{4,10})\b"),
]
APPROVAL_KEYWORDS = (
    "approved",
    "approve",
    "yes",
    "done",
    "clicked",
    "confirmed",
    "confirm",
)
REJECTION_KEYWORDS = (
    "rejected",
    "reject",
    "denied",
    "deny",
    "cancel",
    "cannot",
    "can't",
)


class CliError(RuntimeError):
    """Command error with user-facing message."""


@dataclass
class WaitResult:
    state: Dict[str, Any]
    approved: bool


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def isoformat_utc(value: datetime) -> str:
    return value.astimezone(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def parse_iso8601(value: str) -> datetime:
    normalized = value.strip()
    if normalized.endswith("Z"):
        normalized = normalized[:-1] + "+00:00"
    return datetime.fromisoformat(normalized).astimezone(timezone.utc)


def extract_email(value: str) -> Optional[str]:
    if not value:
        return None
    match = EMAIL_PATTERN.search(value)
    if not match:
        return None
    return match.group(1).strip().lower()


def truncate(value: str, limit: int) -> str:
    if len(value) <= limit:
        return value
    return value[: limit - 1] + "..."


def read_json(path: Path) -> Dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp_path = path.with_suffix(".tmp")
    temp_path.write_text(json.dumps(payload, indent=2, sort_keys=True), encoding="utf-8")
    temp_path.replace(path)


def get_env_first(*keys: str) -> Optional[str]:
    for key in keys:
        value = os.environ.get(key, "").strip()
        if value:
            return value
    return None


def resolve_sender(from_arg: Optional[str]) -> str:
    sender = (from_arg or "").strip() or get_env_first(
        "HUMAN_APPROVAL_FROM",
        "POSTMARK_FROM_EMAIL",
        "POSTMARK_TEST_FROM",
    )
    if not sender:
        raise CliError(
            "missing sender address: provide --from or set HUMAN_APPROVAL_FROM / POSTMARK_FROM_EMAIL"
        )
    return sender


def normalize_scope(scope: str) -> str:
    normalized = scope.strip().lower()
    if normalized not in ("admin", "user"):
        raise CliError("scope must be one of: admin, user")
    return normalized


def resolve_recipient(scope: str, recipient_arg: Optional[str], user_email_arg: Optional[str]) -> str:
    if recipient_arg and recipient_arg.strip():
        return recipient_arg.strip()
    if scope == "admin":
        return DEFAULT_ADMIN_RECIPIENT
    if user_email_arg and user_email_arg.strip():
        return user_email_arg.strip()
    raise CliError("user scope requires --recipient or --user-email")


def get_state_path(state_dir: Path, challenge_id: str) -> Path:
    return state_dir / f"{challenge_id}.json"


def load_state(state_dir: Path, challenge_id: str) -> Dict[str, Any]:
    path = get_state_path(state_dir, challenge_id)
    if not path.exists():
        raise CliError(f"challenge not found: {challenge_id}")
    return read_json(path)


def save_state(state_dir: Path, state: Dict[str, Any]) -> Path:
    challenge_id = state.get("challenge_id", "")
    if not challenge_id:
        raise CliError("state is missing challenge_id")
    path = get_state_path(state_dir, challenge_id)
    state["updated_at"] = isoformat_utc(utc_now())
    write_json(path, state)
    return path


def ensure_token(token_arg: Optional[str], dry_run: bool) -> str:
    if dry_run:
        return "DRY_RUN"
    token = (token_arg or "").strip() or os.environ.get("POSTMARK_SERVER_TOKEN", "").strip()
    if not token:
        raise CliError("missing Postmark token: provide --token or set POSTMARK_SERVER_TOKEN")
    return token


def http_json_request(
    method: str,
    api_base: str,
    token: str,
    path: str,
    query: Optional[Dict[str, str]] = None,
    body: Optional[Dict[str, Any]] = None,
) -> Dict[str, Any]:
    base = api_base.rstrip("/")
    url = f"{base}{path}"
    if query:
        encoded = urllib.parse.urlencode({k: v for k, v in query.items() if v is not None and v != ""})
        if encoded:
            url = f"{url}?{encoded}"

    headers = {
        "Accept": "application/json",
        "X-Postmark-Server-Token": token,
    }
    data: Optional[bytes] = None
    if body is not None:
        headers["Content-Type"] = "application/json"
        data = json.dumps(body).encode("utf-8")

    request = urllib.request.Request(url=url, data=data, headers=headers, method=method.upper())

    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            payload = response.read().decode("utf-8")
    except urllib.error.HTTPError as exc:
        raw = exc.read().decode("utf-8", errors="replace")
        raise CliError(f"Postmark API error {exc.code} for {path}: {truncate(raw, 500)}") from exc
    except urllib.error.URLError as exc:
        raise CliError(f"failed to call Postmark API {path}: {exc.reason}") from exc

    try:
        parsed = json.loads(payload)
    except json.JSONDecodeError as exc:
        raise CliError(f"Postmark API returned invalid JSON for {path}: {truncate(payload, 200)}") from exc

    if not isinstance(parsed, dict):
        raise CliError(f"Postmark API returned non-object response for {path}")
    return parsed


def build_subject(challenge_id: str, account_label: str) -> str:
    token = f"[{SUBJECT_TOKEN_PREFIX}:{challenge_id}]"
    if account_label.strip():
        return f"{token} 2FA approval needed for {account_label.strip()}"
    return f"{token} 2FA approval needed"


def build_text_body(
    challenge_id: str,
    account_label: str,
    timeout_minutes: int,
    action_text: str,
    context: str,
    scope: str,
) -> str:
    lines = [
        "DoWhiz agent needs your help to continue a blocked authentication step.",
        "",
        f"Challenge ID: {challenge_id}",
        f"Scope: {scope}",
    ]
    if account_label.strip():
        lines.append(f"Account context: {account_label.strip()}")
    if action_text.strip():
        lines.append(f"Action needed: {action_text.strip()}")
    if context.strip():
        lines.extend(["", "Additional context:", context.strip()])
    lines.extend(
        [
            "",
            "Please reply in this same email thread with one of:",
            "- CODE: <verification-code>",
            "- APPROVED (if no code is needed and you approved on your device)",
            "- DENIED (if you reject this login)",
            "",
            f"The agent will wait for up to {timeout_minutes} minutes.",
            "It will not continue until a valid reply is received.",
        ]
    )
    return "\n".join(lines)


def build_html_body(text_body: str) -> str:
    escaped = (
        text_body.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\n", "<br>")
    )
    return f"<html><body><p>{escaped}</p></body></html>"


def send_approval_email(
    *,
    api_base: str,
    token: str,
    from_address: str,
    to_address: str,
    reply_to: Optional[str],
    subject: str,
    text_body: str,
    html_body: str,
    metadata: Dict[str, str],
) -> str:
    payload: Dict[str, Any] = {
        "From": from_address,
        "To": to_address,
        "Subject": subject,
        "TextBody": text_body,
        "HtmlBody": html_body,
        "MessageStream": "outbound",
        "Tag": "human-approval-gate",
        "Metadata": metadata,
    }
    if reply_to:
        payload["ReplyTo"] = reply_to

    response = http_json_request("POST", api_base, token, "/email", body=payload)
    error_code = response.get("ErrorCode", 0)
    if error_code != 0:
        message = str(response.get("Message", "unknown Postmark error"))
        raise CliError(f"Postmark send failed with ErrorCode={error_code}: {message}")

    message_id = str(response.get("MessageID", "")).strip()
    if not message_id:
        raise CliError("Postmark send response missing MessageID")
    return message_id


def build_postmark_metadata(challenge_id: str, scope: str) -> Dict[str, str]:
    metadata = {
        POSTMARK_METADATA_CHALLENGE_ID_KEY: challenge_id,
        POSTMARK_METADATA_SCOPE_KEY: scope,
    }
    for key in metadata:
        if len(key) > 20:
            raise CliError(f"metadata key exceeds Postmark 20-character limit: {key}")
    return metadata


def parse_reply_decision(text: str) -> Tuple[str, Optional[str], str]:
    body = (text or "").strip()
    if not body:
        return ("unknown", None, "empty body")

    for pattern in CODE_PATTERNS:
        match = pattern.search(body)
        if match:
            return ("approved", match.group(1), "verification code found")

    lowered = body.lower()
    if any(keyword in lowered for keyword in REJECTION_KEYWORDS):
        return ("rejected", None, "rejection keyword found")
    if any(keyword in lowered for keyword in APPROVAL_KEYWORDS):
        return ("approved", None, "approval keyword found")
    return ("unknown", None, "no decision keyword found")


def safe_get_str(payload: Dict[str, Any], *keys: str) -> str:
    for key in keys:
        value = payload.get(key)
        if isinstance(value, str):
            return value
    return ""


def extract_from_email(payload: Dict[str, Any]) -> Optional[str]:
    from_raw = safe_get_str(payload, "From")
    from_email = extract_email(from_raw)
    if from_email:
        return from_email

    from_full = payload.get("FromFull")
    if isinstance(from_full, dict):
        email = from_full.get("Email")
        if isinstance(email, str) and email.strip():
            return email.strip().lower()
    return None


def parse_message_date(payload: Dict[str, Any]) -> Optional[datetime]:
    for key in ("ReceivedAt", "Date", "DateTime", "MessageDate"):
        value = payload.get(key)
        if isinstance(value, str) and value.strip():
            try:
                return parse_iso8601(value)
            except Exception:
                continue
    return None


def poll_for_approval_once(
    *,
    api_base: str,
    token: str,
    challenge: Dict[str, Any],
) -> Tuple[Optional[Dict[str, Any]], List[str]]:
    challenge_id = str(challenge["challenge_id"])
    subject_token = f"[{SUBJECT_TOKEN_PREFIX}:{challenge_id}]"
    created_at = parse_iso8601(str(challenge["created_at"]))
    created_date = created_at.date().isoformat()
    expected_reply_from = extract_email(str(challenge.get("expected_reply_from", "")))
    recipient_filter = str(challenge.get("reply_to", "")).strip()

    query = {
        "count": "100",
        "offset": "0",
        "status": "processed",
        "subject": subject_token,
        "fromdate": created_date,
    }
    if expected_reply_from:
        query["fromemail"] = expected_reply_from
    if recipient_filter:
        query["recipient"] = recipient_filter

    search_response = http_json_request("GET", api_base, token, "/messages/inbound", query=query)
    messages = search_response.get("InboundMessages")
    if not isinstance(messages, list):
        messages = search_response.get("Messages")
    if not isinstance(messages, list):
        messages = []

    seen = set(challenge.get("seen_inbound_message_ids", []))
    new_seen: List[str] = []

    for summary in messages:
        if not isinstance(summary, dict):
            continue
        message_id = safe_get_str(summary, "MessageID", "MessageId", "ID", "Id")
        if not message_id:
            continue
        if message_id in seen:
            continue

        details_path = f"/messages/inbound/{urllib.parse.quote(message_id, safe='')}/details"
        details = http_json_request("GET", api_base, token, details_path)

        from_header = safe_get_str(details, "From")
        from_email = extract_from_email(details)
        if expected_reply_from and from_email != expected_reply_from:
            new_seen.append(message_id)
            continue

        subject = safe_get_str(details, "Subject")
        if subject_token.lower() not in subject.lower():
            new_seen.append(message_id)
            continue

        received_at = parse_message_date(details)
        if received_at and received_at < created_at:
            new_seen.append(message_id)
            continue

        reply_text = safe_get_str(details, "StrippedTextReply", "TextBody", "Text")
        decision, code, reason = parse_reply_decision(reply_text)
        new_seen.append(message_id)

        if decision == "unknown":
            continue

        resolution = {
            "decision": decision,
            "code": code,
            "reason": reason,
            "inbound_message_id": message_id,
            "received_at": isoformat_utc(received_at) if received_at else isoformat_utc(utc_now()),
            "reply_from": from_email or from_header,
            "reply_subject": subject,
            "reply_excerpt": truncate(reply_text.strip(), MAX_REPLY_SNIPPET_CHARS),
        }
        return (resolution, new_seen)

    return (None, new_seen)


def mark_timeout(state: Dict[str, Any]) -> Dict[str, Any]:
    updated = dict(state)
    updated["status"] = "timeout"
    updated["resolution"] = {
        "decision": "timeout",
        "reason": "no valid approval reply received before timeout",
    }
    return updated


def mark_resolution(state: Dict[str, Any], resolution: Dict[str, Any]) -> Dict[str, Any]:
    updated = dict(state)
    decision = str(resolution.get("decision", "")).lower()
    if decision == "approved":
        updated["status"] = "approved"
    elif decision == "rejected":
        updated["status"] = "rejected"
    else:
        updated["status"] = "pending"
    updated["resolution"] = resolution
    return updated


def wait_for_resolution(
    *,
    state_dir: Path,
    api_base: str,
    token: str,
    challenge_id: str,
    timeout_minutes: Optional[int],
    poll_interval_seconds: int,
) -> WaitResult:
    state = load_state(state_dir, challenge_id)
    if state.get("status") in ("approved", "rejected", "timeout"):
        return WaitResult(state=state, approved=state.get("status") == "approved")

    created_at = parse_iso8601(str(state["created_at"]))
    default_deadline = created_at + timedelta(minutes=int(state.get("timeout_minutes", DEFAULT_TIMEOUT_MINUTES)))
    if timeout_minutes is not None:
        deadline = min(default_deadline, utc_now() + timedelta(minutes=timeout_minutes))
    else:
        deadline = default_deadline

    while utc_now() <= deadline:
        state = load_state(state_dir, challenge_id)
        if state.get("status") in ("approved", "rejected", "timeout"):
            return WaitResult(state=state, approved=state.get("status") == "approved")

        resolution, new_seen = poll_for_approval_once(
            api_base=api_base,
            token=token,
            challenge=state,
        )

        if new_seen:
            seen = list(dict.fromkeys(list(state.get("seen_inbound_message_ids", [])) + new_seen))
            state["seen_inbound_message_ids"] = seen

        if resolution:
            state = mark_resolution(state, resolution)
            save_state(state_dir, state)
            return WaitResult(state=state, approved=state.get("status") == "approved")

        if new_seen:
            save_state(state_dir, state)

        time.sleep(max(1, poll_interval_seconds))

    timed_out = mark_timeout(load_state(state_dir, challenge_id))
    save_state(state_dir, timed_out)
    return WaitResult(state=timed_out, approved=False)


def emit_json(payload: Dict[str, Any]) -> None:
    print(json.dumps(payload, ensure_ascii=True, sort_keys=True))


def build_request_state(args: argparse.Namespace) -> Dict[str, Any]:
    if args.timeout_minutes <= 0:
        raise CliError("--timeout-minutes must be greater than zero")
    if args.wait_timeout_minutes is not None and args.wait_timeout_minutes <= 0:
        raise CliError("--wait-timeout-minutes must be greater than zero")
    if args.poll_interval_seconds <= 0:
        raise CliError("--poll-interval-seconds must be greater than zero")

    scope = normalize_scope(args.scope)
    recipient = resolve_recipient(scope, args.recipient, args.user_email)
    challenge_id = (args.challenge_id or "").strip() or str(uuid4())
    sender = resolve_sender(args.from_address)
    reply_to = (args.reply_to or "").strip() or get_env_first("HUMAN_APPROVAL_REPLY_TO") or sender
    expected_reply_from = (args.expected_reply_from or "").strip() or recipient
    timeout_minutes = args.timeout_minutes

    account_label = (args.account_label or "").strip()
    action_text = (args.action_text or "").strip()
    context = (args.context or "").strip()

    subject = build_subject(challenge_id, account_label)
    text_body = build_text_body(
        challenge_id=challenge_id,
        account_label=account_label,
        timeout_minutes=timeout_minutes,
        action_text=action_text,
        context=context,
        scope=scope,
    )
    html_body = build_html_body(text_body)

    created_at = utc_now()
    state: Dict[str, Any] = {
        "challenge_id": challenge_id,
        "status": "pending",
        "scope": scope,
        "recipient": recipient,
        "expected_reply_from": expected_reply_from,
        "from_address": sender,
        "reply_to": reply_to,
        "subject": subject,
        "subject_token": f"[{SUBJECT_TOKEN_PREFIX}:{challenge_id}]",
        "account_label": account_label,
        "action_text": action_text,
        "context": context,
        "timeout_minutes": timeout_minutes,
        "created_at": isoformat_utc(created_at),
        "expires_at": isoformat_utc(created_at + timedelta(minutes=timeout_minutes)),
        "resolution": None,
        "seen_inbound_message_ids": [],
        "outbound_message_id": None,
        "outbound_dry_run": bool(args.dry_run),
        "message_stream": "outbound",
    }
    state["_rendered_email"] = {
        "subject": subject,
        "text_body": text_body,
        "html_body": html_body,
    }
    return state


def cmd_request(args: argparse.Namespace) -> int:
    state_dir = Path(args.state_dir)
    api_base = args.api_base
    token = ensure_token(args.token, args.dry_run)

    state = build_request_state(args)
    rendered = state.pop("_rendered_email")

    if args.dry_run:
        state["outbound_message_id"] = "DRY_RUN"
    else:
        metadata = build_postmark_metadata(str(state["challenge_id"]), str(state["scope"]))
        message_id = send_approval_email(
            api_base=api_base,
            token=token,
            from_address=state["from_address"],
            to_address=state["recipient"],
            reply_to=state["reply_to"],
            subject=rendered["subject"],
            text_body=rendered["text_body"],
            html_body=rendered["html_body"],
            metadata=metadata,
        )
        state["outbound_message_id"] = message_id

    save_state(state_dir, state)

    if args.wait:
        result = wait_for_resolution(
            state_dir=state_dir,
            api_base=api_base,
            token=token,
            challenge_id=state["challenge_id"],
            timeout_minutes=args.wait_timeout_minutes,
            poll_interval_seconds=args.poll_interval_seconds,
        )
        emit_json(result.state)
        return 0 if result.approved else 4

    emit_json(state)
    return 0


def cmd_status(args: argparse.Namespace) -> int:
    state_dir = Path(args.state_dir)
    state = load_state(state_dir, args.challenge_id)

    if args.refresh and state.get("status") == "pending":
        token = ensure_token(args.token, False)
        resolution, new_seen = poll_for_approval_once(
            api_base=args.api_base,
            token=token,
            challenge=state,
        )
        if new_seen:
            seen = list(dict.fromkeys(list(state.get("seen_inbound_message_ids", [])) + new_seen))
            state["seen_inbound_message_ids"] = seen
        if resolution:
            state = mark_resolution(state, resolution)
        save_state(state_dir, state)

    emit_json(state)
    return 0


def cmd_wait(args: argparse.Namespace) -> int:
    state_dir = Path(args.state_dir)
    token = ensure_token(args.token, False)
    result = wait_for_resolution(
        state_dir=state_dir,
        api_base=args.api_base,
        token=token,
        challenge_id=args.challenge_id,
        timeout_minutes=args.timeout_minutes,
        poll_interval_seconds=args.poll_interval_seconds,
    )
    emit_json(result.state)
    return 0 if result.approved else 4


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="human_approval_gate",
        description="Send human approval request emails and wait for 2FA replies.",
    )
    parser.add_argument(
        "--api-base",
        default=get_env_first("POSTMARK_API_BASE_URL") or API_BASE_DEFAULT,
        help="Postmark API base URL (default: %(default)s)",
    )
    parser.add_argument(
        "--state-dir",
        default=STATE_DIR_DEFAULT,
        help="State directory for challenge files (default: %(default)s)",
    )
    parser.add_argument(
        "--token",
        default="",
        help="Postmark server token (default: POSTMARK_SERVER_TOKEN env)",
    )

    subparsers = parser.add_subparsers(dest="command", required=True)

    request_parser = subparsers.add_parser("request", help="create a challenge and send email")
    request_parser.add_argument("--scope", default="user", choices=["admin", "user"])
    request_parser.add_argument("--challenge-id", default="")
    request_parser.add_argument("--recipient", default="")
    request_parser.add_argument("--user-email", default="")
    request_parser.add_argument("--from", dest="from_address", default="")
    request_parser.add_argument("--reply-to", default="")
    request_parser.add_argument("--expected-reply-from", default="")
    request_parser.add_argument("--account-label", default="")
    request_parser.add_argument("--action-text", default="")
    request_parser.add_argument("--context", default="")
    request_parser.add_argument(
        "--timeout-minutes",
        type=int,
        default=DEFAULT_TIMEOUT_MINUTES,
        help="max wait window communicated in the email",
    )
    request_parser.add_argument("--wait", action="store_true", help="wait for reply after sending")
    request_parser.add_argument(
        "--wait-timeout-minutes",
        type=int,
        default=None,
        help="optional shorter local wait timeout",
    )
    request_parser.add_argument(
        "--poll-interval-seconds",
        type=int,
        default=DEFAULT_POLL_SECONDS,
    )
    request_parser.add_argument("--dry-run", action="store_true")
    request_parser.set_defaults(func=cmd_request)

    status_parser = subparsers.add_parser("status", help="show current challenge state")
    status_parser.add_argument("--challenge-id", required=True)
    status_parser.add_argument(
        "--refresh",
        action="store_true",
        help="poll Postmark once before returning state",
    )
    status_parser.set_defaults(func=cmd_status)

    wait_parser = subparsers.add_parser("wait", help="wait for challenge resolution")
    wait_parser.add_argument("--challenge-id", required=True)
    wait_parser.add_argument("--timeout-minutes", type=int, default=None)
    wait_parser.add_argument(
        "--poll-interval-seconds",
        type=int,
        default=DEFAULT_POLL_SECONDS,
    )
    wait_parser.set_defaults(func=cmd_wait)

    return parser


def main(argv: List[str]) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    try:
        result = args.func(args)
        return int(result)
    except CliError as exc:
        emit_json(
            {
                "status": "error",
                "error": str(exc),
            }
        )
        return 1
    except KeyboardInterrupt:
        emit_json(
            {
                "status": "error",
                "error": "interrupted",
            }
        )
        return 130


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
