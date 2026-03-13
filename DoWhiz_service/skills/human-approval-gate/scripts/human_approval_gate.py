#!/usr/bin/env python3
"""Human approval gate for auth blockers that need a human.

This CLI sends an approval email and optionally blocks while polling Postmark inbound
messages for the first reply in the same challenge thread.
"""

from __future__ import annotations

import argparse
import base64
import json
import mimetypes
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from datetime import datetime, timedelta, timezone
from html import escape
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
from uuid import uuid4

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    import tomli as tomllib  # type: ignore

API_BASE_DEFAULT = "https://api.postmarkapp.com"
STATE_DIR_DEFAULT = ".human_approval_gate/challenges"
DEFAULT_TIMEOUT_MINUTES = 30
DEFAULT_POLL_SECONDS = 15
DEFAULT_ADMIN_RECIPIENT = "admin@dowhiz.com"
ADMIN_RECIPIENT_ENV_KEY = "HUMAN_APPROVAL_ADMIN_RECIPIENT"
SUBJECT_TOKEN_PREFIX = "HAG"
# Postmark limits metadata key names to at most 20 characters.
POSTMARK_METADATA_CHALLENGE_ID_KEY = "hag_challenge_id"
POSTMARK_METADATA_SCOPE_KEY = "hag_scope"
POSTMARK_METADATA_TYPE_KEY = "hag_type"
MAX_TOTAL_ATTACHMENT_BYTES = 10 * 1024 * 1024
DEFAULT_PASSWORD_ENV_KEY = "GOOGLE_PASSWORD"
CHALLENGE_TYPES = ("captcha", "password", "two_factor")
TWO_FACTOR_METHODS = ("sms", "email", "auth_app", "device_tap", "other")
EVENTS_LOG_FILENAME = "events.jsonl"
PAGE_STATES_BY_TYPE = {
    "captcha": {"captcha_blocked"},
    "password": {"waiting_for_password"},
    "two_factor": {"waiting_for_code_input", "waiting_for_device_approval"},
}

EMAIL_PATTERN = re.compile(r"([A-Za-z0-9._%+\-]+@[A-Za-z0-9.\-]+\.[A-Za-z]{2,})")


class CliError(RuntimeError):
    """Command error with user-facing message."""


@dataclass
class WaitResult:
    state: Dict[str, Any]
    replied: bool


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


def append_jsonl(path: Path, payload: Dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(payload, ensure_ascii=True, sort_keys=True))
        handle.write("\n")


def get_env_first(*keys: str) -> Optional[str]:
    for key in keys:
        value = os.environ.get(key, "").strip()
        if value:
            return value
    return None


def discover_do_whiz_service_root() -> Optional[Path]:
    script_path = Path(__file__).resolve()
    for parent in script_path.parents:
        if parent.name == "DoWhiz_service":
            return parent
    return None


def resolve_employee_config_candidates() -> List[Path]:
    explicit = get_env_first("EMPLOYEE_CONFIG_PATH")
    if explicit:
        raw = Path(explicit).expanduser()
        if raw.is_absolute():
            return [raw]
        cwd = Path.cwd()
        candidates: List[Path] = [cwd / raw, cwd / "DoWhiz_service" / raw]
        script_root = discover_do_whiz_service_root()
        if script_root is not None:
            candidates.append(script_root / raw)
        deduped: List[Path] = []
        seen: set[str] = set()
        for candidate in candidates:
            resolved = candidate.resolve()
            key = str(resolved)
            if key in seen:
                continue
            seen.add(key)
            deduped.append(resolved)
        return deduped

    deploy_target = (get_env_first("DEPLOY_TARGET") or "").strip().lower()
    if deploy_target == "staging":
        names = ("employee.staging.toml", "employee.toml")
    else:
        names = ("employee.toml", "employee.staging.toml")

    root_candidates: List[Path] = []
    cwd = Path.cwd()
    root_candidates.append(cwd)
    if cwd.name != "DoWhiz_service":
        root_candidates.append(cwd / "DoWhiz_service")

    script_root = discover_do_whiz_service_root()
    if script_root is not None:
        root_candidates.append(script_root)

    paths: List[Path] = []
    seen: set[str] = set()
    for root in root_candidates:
        for name in names:
            candidate = (root / name).resolve()
            key = str(candidate)
            if key in seen:
                continue
            seen.add(key)
            if candidate.exists():
                paths.append(candidate)
    return paths


def load_employee_mailbox_from_config(path: Path, employee_id: str) -> Optional[str]:
    try:
        with path.open("rb") as handle:
            payload = tomllib.load(handle)
    except Exception:
        return None

    employees = payload.get("employees")
    if not isinstance(employees, list):
        return None

    normalized_id = employee_id.strip().lower()
    for employee in employees:
        if not isinstance(employee, dict):
            continue
        config_id = str(employee.get("id", "")).strip().lower()
        if config_id != normalized_id:
            continue
        addresses = employee.get("addresses")
        if not isinstance(addresses, list):
            return None
        for address in addresses:
            if not isinstance(address, str) or not address.strip():
                continue
            email = extract_email(address) or address.strip().lower()
            if email:
                return email
        return None
    return None


def resolve_employee_mailbox_email() -> Optional[str]:
    employee_id = get_env_first("EMPLOYEE_ID")
    for config_path in resolve_employee_config_candidates():
        try:
            with config_path.open("rb") as handle:
                payload = tomllib.load(handle)
        except Exception:
            continue

        effective_employee_id = employee_id
        if not effective_employee_id:
            default_id = payload.get("default_employee_id")
            if isinstance(default_id, str) and default_id.strip():
                effective_employee_id = default_id.strip()
        if not effective_employee_id:
            continue

        employee_email = load_employee_mailbox_from_config(config_path, effective_employee_id)
        if employee_email:
            return employee_email
    return None


def resolve_sender(from_arg: Optional[str]) -> str:
    sender = (from_arg or "").strip() or get_env_first("HUMAN_APPROVAL_FROM")
    if not sender:
        sender = resolve_employee_mailbox_email() or ""
    if not sender:
        raise CliError("missing sender address: provide --from, set HUMAN_APPROVAL_FROM, or ensure employee config has a mailbox")
    return sender


def normalize_scope(scope: str) -> str:
    normalized = scope.strip().lower()
    if normalized not in ("admin", "user"):
        raise CliError("scope must be one of: admin, user")
    return normalized


def normalize_challenge_type(value: str) -> str:
    normalized = value.strip().lower()
    if normalized not in CHALLENGE_TYPES:
        raise CliError(f"challenge type must be one of: {', '.join(CHALLENGE_TYPES)}")
    return normalized


def normalize_two_factor_method(value: str) -> str:
    normalized = value.strip().lower()
    if normalized not in TWO_FACTOR_METHODS:
        raise CliError(f"two-factor method must be one of: {', '.join(TWO_FACTOR_METHODS)}")
    return normalized


def normalize_page_state(challenge_type: str, value: str) -> str:
    normalized = value.strip().lower()
    allowed_states = PAGE_STATES_BY_TYPE[challenge_type]
    if not normalized:
        if challenge_type == "captcha":
            return "captcha_blocked"
        if challenge_type == "password":
            return "waiting_for_password"
        raise CliError(
            "--page-state is required for two_factor challenges "
            f"({', '.join(sorted(allowed_states))})"
        )
    if normalized not in allowed_states:
        raise CliError(
            f"page state {normalized!r} is invalid for {challenge_type}; "
            f"expected one of: {', '.join(sorted(allowed_states))}"
        )
    return normalized


def canonical_email(value: str) -> str:
    email = extract_email(value)
    if email:
        return email
    return value.strip().lower()


def resolve_admin_recipient() -> str:
    return get_env_first(ADMIN_RECIPIENT_ENV_KEY) or DEFAULT_ADMIN_RECIPIENT


def resolve_recipient(scope: str, recipient_arg: Optional[str], user_email_arg: Optional[str]) -> str:
    if scope == "admin":
        admin_recipient = resolve_admin_recipient()
        if recipient_arg and recipient_arg.strip():
            recipient = recipient_arg.strip()
            if canonical_email(recipient) != canonical_email(admin_recipient):
                raise CliError(
                    "scope=admin cannot send to non-admin recipient; remove --recipient "
                    f"or set {ADMIN_RECIPIENT_ENV_KEY} if admin address changed"
                )
            return recipient
        return admin_recipient
    if recipient_arg and recipient_arg.strip():
        return recipient_arg.strip()
    if user_email_arg and user_email_arg.strip():
        return user_email_arg.strip()
    raise CliError("user scope requires --recipient or --user-email")


def get_state_path(state_dir: Path, challenge_id: str) -> Path:
    return state_dir / f"{challenge_id}.json"


def get_events_log_path(state_dir: Path) -> Path:
    return state_dir.parent / EVENTS_LOG_FILENAME


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


def ensure_file_paths(paths: List[str]) -> List[Path]:
    resolved: List[Path] = []
    for raw_path in paths:
        path_text = raw_path.strip()
        if not path_text:
            continue
        path = Path(path_text).expanduser()
        if not path.is_absolute():
            path = (Path.cwd() / path).resolve()
        else:
            path = path.resolve()
        if not path.exists():
            raise CliError(f"screenshot file not found: {path}")
        if not path.is_file():
            raise CliError(f"screenshot path is not a file: {path}")
        resolved.append(path)
    if not resolved:
        raise CliError("at least one --screenshot file is required")
    return resolved


def build_postmark_attachments(paths: List[Path]) -> Tuple[List[Dict[str, str]], List[Dict[str, Any]]]:
    total_bytes = 0
    postmark_attachments: List[Dict[str, str]] = []
    attachment_summaries: List[Dict[str, Any]] = []

    for path in paths:
        raw = path.read_bytes()
        total_bytes += len(raw)
        if total_bytes > MAX_TOTAL_ATTACHMENT_BYTES:
            raise CliError("total attachment size exceeds Postmark 10 MB limit")
        content_type = mimetypes.guess_type(path.name)[0] or "application/octet-stream"
        postmark_attachments.append(
            {
                "Name": path.name,
                "Content": base64.b64encode(raw).decode("ascii"),
                "ContentType": content_type,
            }
        )
        attachment_summaries.append(
            {
                "name": path.name,
                "path": str(path),
                "content_type": content_type,
                "size_bytes": len(raw),
            }
        )

    return postmark_attachments, attachment_summaries


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


def build_subject(challenge_id: str, account_label: str, challenge_type: str) -> str:
    token = f"[{SUBJECT_TOKEN_PREFIX}:{challenge_id}]"
    subject_prefix = {
        "captcha": "CAPTCHA help needed",
        "password": "Password needed",
        "two_factor": "2FA approval needed",
    }[challenge_type]
    if account_label.strip():
        return f"{token} {subject_prefix} for {account_label.strip()}"
    return f"{token} {subject_prefix}"


def humanize_challenge_type(challenge_type: str) -> str:
    return {
        "captcha": "CAPTCHA",
        "password": "Password",
        "two_factor": "2FA",
    }[challenge_type]


def humanize_page_state(page_state: str) -> str:
    return {
        "captcha_blocked": "Still blocked on CAPTCHA after one built-in solve attempt",
        "waiting_for_password": "Browser is waiting for the password field",
        "waiting_for_code_input": "Browser is waiting for a verification code to be typed",
        "waiting_for_device_approval": "Browser is waiting for an approval action on another device",
    }[page_state]


def describe_two_factor_method(method: str, destination: str) -> str:
    destination = destination.strip()
    if method == "sms":
        return f"SMS code to {destination}" if destination else "SMS code"
    if method == "email":
        return f"Email code sent to {destination}" if destination else "Email code"
    if method == "auth_app":
        return "Authenticator app code"
    if method == "device_tap":
        return f"Device approval / number tap on {destination}" if destination else "Device approval / number tap"
    return destination or "Other verification method"


def build_default_action_text(
    challenge_type: str,
    account_label: str,
    two_factor_method: str,
    verification_destination: str,
    page_state: str,
) -> str:
    label = account_label.strip() or "the target account"
    if challenge_type == "captcha":
        return (
            f"Please inspect the attached browser screenshot for {label} and reply "
            "with the CAPTCHA text or instructions I should enter."
        )
    if challenge_type == "password":
        return (
            f"Please reply with the password for {label}, or tell me if I should use another account."
        )
    method_description = describe_two_factor_method(two_factor_method, verification_destination)
    if page_state == "waiting_for_device_approval":
        return (
            f"Please complete the pending {method_description} and reply with the approval result "
            "or any number shown on the approval screen."
        )
    return f"Please reply with the {method_description} that is currently required on the attached screen."


def build_text_body(state: Dict[str, Any]) -> str:
    challenge_id = str(state["challenge_id"])
    account_label = str(state.get("account_label", ""))
    timeout_minutes = int(state.get("timeout_minutes", DEFAULT_TIMEOUT_MINUTES))
    action_text = str(state.get("action_text", ""))
    context = str(state.get("context", ""))
    scope = str(state.get("scope", ""))
    challenge_type = str(state.get("challenge_type", ""))
    page_state = str(state.get("page_state", ""))
    two_factor_method = str(state.get("two_factor_method", ""))
    verification_destination = str(state.get("verification_destination", ""))
    password_env_key = str(state.get("password_env_key", ""))
    password_lookup_status = str(state.get("password_lookup_status", ""))
    screenshots = state.get("request_attachments") or []

    lines = [
        "DoWhiz agent needs your help to continue a blocked authentication step.",
        "",
        f"Challenge ID: {challenge_id}",
        f"Challenge type: {humanize_challenge_type(challenge_type)}",
        f"Scope: {scope}",
    ]
    if account_label.strip():
        lines.append(f"Account context: {account_label.strip()}")
    if page_state:
        lines.append(f"Current browser state: {humanize_page_state(page_state)}")
    if challenge_type == "two_factor":
        lines.append(
            f"Verification method: {describe_two_factor_method(two_factor_method, verification_destination)}"
        )
    if challenge_type == "password":
        if password_env_key:
            lines.append(f"Password env key checked: {password_env_key}")
        if password_lookup_status:
            lines.append(f"Password lookup status: {password_lookup_status}")
    if challenge_type == "captcha":
        lines.append("Agent attempted one built-in visual solve before asking for help.")
    if screenshots:
        screenshot_names = ", ".join(
            str(item.get("name", "")).strip()
            for item in screenshots
            if str(item.get("name", "")).strip()
        )
        lines.append(
            f"Attached screenshot(s): {screenshot_names or f'{len(screenshots)} file(s)'}"
        )
    if action_text.strip():
        lines.append(f"Action needed: {action_text.strip()}")
    if context.strip():
        lines.extend(["", "Additional context:", context.strip()])
    lines.extend(
        [
            "",
            "Please reply in this same email thread with the exact information",
            "the current browser screen needs so the agent can continue.",
            "",
            f"The agent will wait for up to {timeout_minutes} minutes.",
            "It will not continue until a reply is received.",
        ]
    )
    return "\n".join(lines)


def build_html_body(text_body: str) -> str:
    escaped = escape(text_body).replace("\n", "<br>")
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
    attachments: List[Dict[str, str]],
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
        "Attachments": attachments,
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


def build_postmark_metadata(challenge_id: str, scope: str, challenge_type: str) -> Dict[str, str]:
    metadata = {
        POSTMARK_METADATA_CHALLENGE_ID_KEY: challenge_id,
        POSTMARK_METADATA_SCOPE_KEY: scope,
        POSTMARK_METADATA_TYPE_KEY: challenge_type,
    }
    for key in metadata:
        if len(key) > 20:
            raise CliError(f"metadata key exceeds Postmark 20-character limit: {key}")
    return metadata


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


def build_reply_payload(details: Dict[str, Any], message_id: str, received_at: Optional[datetime]) -> Dict[str, Any]:
    from_header = safe_get_str(details, "From")
    from_email = extract_from_email(details)
    subject = safe_get_str(details, "Subject")
    stripped_text_reply = safe_get_str(details, "StrippedTextReply")
    text_body = safe_get_str(details, "TextBody", "Text")
    html_body = safe_get_str(details, "HtmlBody", "Html")
    headers = details.get("Headers")
    attachments = details.get("Attachments")

    return {
        "inbound_message_id": message_id,
        "received_at": isoformat_utc(received_at) if received_at else isoformat_utc(utc_now()),
        "reply_from": from_email or from_header,
        "from_email": from_email,
        "reply_subject": subject,
        "stripped_text_reply": stripped_text_reply,
        "text_body": text_body,
        "html_body": html_body,
        "headers": headers if isinstance(headers, list) else None,
        "attachments": attachments if isinstance(attachments, list) else None,
        "message": details,
    }


def poll_for_reply_once(
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

        new_seen.append(message_id)
        return (build_reply_payload(details, message_id, received_at), new_seen)

    return (None, new_seen)


def mark_timeout(state: Dict[str, Any]) -> Dict[str, Any]:
    updated = dict(state)
    updated["status"] = "timeout"
    updated["reply"] = {
        "reason": "no reply received before timeout",
    }
    return updated


def mark_reply_received(state: Dict[str, Any], reply: Dict[str, Any]) -> Dict[str, Any]:
    updated = dict(state)
    updated["status"] = "replied"
    updated["reply"] = reply
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
    if state.get("status") in ("replied", "timeout"):
        return WaitResult(state=state, replied=state.get("status") == "replied")

    created_at = parse_iso8601(str(state["created_at"]))
    default_deadline = created_at + timedelta(minutes=int(state.get("timeout_minutes", DEFAULT_TIMEOUT_MINUTES)))
    if timeout_minutes is not None:
        deadline = min(default_deadline, utc_now() + timedelta(minutes=timeout_minutes))
    else:
        deadline = default_deadline

    while utc_now() <= deadline:
        state = load_state(state_dir, challenge_id)
        if state.get("status") in ("replied", "timeout"):
            return WaitResult(state=state, replied=state.get("status") == "replied")

        reply, new_seen = poll_for_reply_once(
            api_base=api_base,
            token=token,
            challenge=state,
        )

        if new_seen:
            seen = list(dict.fromkeys(list(state.get("seen_inbound_message_ids", [])) + new_seen))
            state["seen_inbound_message_ids"] = seen

        if reply:
            state = mark_reply_received(state, reply)
            save_state(state_dir, state)
            return WaitResult(state=state, replied=True)

        if new_seen:
            save_state(state_dir, state)

        time.sleep(max(1, poll_interval_seconds))

    timed_out = mark_timeout(load_state(state_dir, challenge_id))
    save_state(state_dir, timed_out)
    return WaitResult(state=timed_out, replied=False)


def emit_json(payload: Dict[str, Any]) -> None:
    print(json.dumps(payload, ensure_ascii=True, sort_keys=True))


def build_send_event_payload(state: Dict[str, Any]) -> Dict[str, Any]:
    return {
        "event": "hag_request_sent",
        "challenge_id": state.get("challenge_id"),
        "challenge_type": state.get("challenge_type"),
        "scope": state.get("scope"),
        "page_state": state.get("page_state"),
        "account_label": state.get("account_label"),
        "recipient": state.get("recipient"),
        "expected_reply_from": state.get("expected_reply_from"),
        "two_factor_method": state.get("two_factor_method"),
        "verification_destination": state.get("verification_destination"),
        "password_env_key": state.get("password_env_key"),
        "password_lookup_status": state.get("password_lookup_status"),
        "attachment_count": len(state.get("request_attachments") or []),
        "attachments": state.get("request_attachments") or [],
        "outbound_message_id": state.get("outbound_message_id"),
        "outbound_dry_run": state.get("outbound_dry_run"),
        "created_at": state.get("created_at"),
        "expires_at": state.get("expires_at"),
    }


def record_send_event(state_dir: Path, state: Dict[str, Any]) -> None:
    event_payload = build_send_event_payload(state)
    append_jsonl(get_events_log_path(state_dir), event_payload)
    print(
        f"HAG_EVENT {json.dumps(event_payload, ensure_ascii=True, sort_keys=True)}",
        file=sys.stderr,
        flush=True,
    )


def build_request_state(args: argparse.Namespace) -> Dict[str, Any]:
    if args.timeout_minutes <= 0:
        raise CliError("--timeout-minutes must be greater than zero")
    if args.wait_timeout_minutes is not None and args.wait_timeout_minutes <= 0:
        raise CliError("--wait-timeout-minutes must be greater than zero")
    if args.poll_interval_seconds <= 0:
        raise CliError("--poll-interval-seconds must be greater than zero")

    scope = normalize_scope(args.scope)
    challenge_type = normalize_challenge_type(args.challenge_type)
    page_state = normalize_page_state(challenge_type, args.page_state)
    recipient = resolve_recipient(scope, args.recipient, args.user_email)
    challenge_id = (args.challenge_id or "").strip() or str(uuid4())
    sender = resolve_sender(args.from_address)
    reply_to = (args.reply_to or "").strip() or get_env_first("HUMAN_APPROVAL_REPLY_TO") or sender
    expected_reply_from = (args.expected_reply_from or "").strip() or recipient
    timeout_minutes = args.timeout_minutes

    account_label = (args.account_label or "").strip()
    context = (args.context or "").strip()
    two_factor_method = ""
    verification_destination = (args.verification_destination or "").strip()
    if challenge_type == "two_factor":
        two_factor_method = normalize_two_factor_method(args.two_factor_method)
        if two_factor_method in {"sms", "email"} and not verification_destination:
            raise CliError(
                f"--verification-destination is required for {two_factor_method} two_factor challenges"
            )
    elif args.two_factor_method.strip():
        raise CliError("--two-factor-method is only valid for two_factor challenges")
    elif verification_destination:
        raise CliError("--verification-destination is only valid for two_factor challenges")

    password_env_key = (args.password_env_key or "").strip() or DEFAULT_PASSWORD_ENV_KEY
    password_lookup_status = (args.password_lookup_status or "").strip()
    if challenge_type != "password":
        password_env_key = ""
        password_lookup_status = ""

    screenshot_paths = ensure_file_paths(args.screenshot)
    postmark_attachments, attachment_summaries = build_postmark_attachments(screenshot_paths)

    action_text = (args.action_text or "").strip() or build_default_action_text(
        challenge_type=challenge_type,
        account_label=account_label,
        two_factor_method=two_factor_method,
        verification_destination=verification_destination,
        page_state=page_state,
    )

    created_at = utc_now()
    state: Dict[str, Any] = {
        "challenge_id": challenge_id,
        "status": "pending",
        "scope": scope,
        "challenge_type": challenge_type,
        "page_state": page_state,
        "recipient": recipient,
        "expected_reply_from": expected_reply_from,
        "from_address": sender,
        "reply_to": reply_to,
        "subject": build_subject(challenge_id, account_label, challenge_type),
        "subject_token": f"[{SUBJECT_TOKEN_PREFIX}:{challenge_id}]",
        "account_label": account_label,
        "action_text": action_text,
        "context": context,
        "two_factor_method": two_factor_method,
        "verification_destination": verification_destination,
        "password_env_key": password_env_key,
        "password_lookup_status": password_lookup_status,
        "request_attachments": attachment_summaries,
        "timeout_minutes": timeout_minutes,
        "created_at": isoformat_utc(created_at),
        "expires_at": isoformat_utc(created_at + timedelta(minutes=timeout_minutes)),
        "reply": None,
        "seen_inbound_message_ids": [],
        "outbound_message_id": None,
        "outbound_dry_run": bool(args.dry_run),
        "message_stream": "outbound",
    }
    text_body = build_text_body(state)
    state["_rendered_email"] = {
        "subject": state["subject"],
        "text_body": text_body,
        "html_body": build_html_body(text_body),
        "attachments": postmark_attachments,
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
        metadata = build_postmark_metadata(
            str(state["challenge_id"]),
            str(state["scope"]),
            str(state["challenge_type"]),
        )
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
            attachments=rendered["attachments"],
        )
        state["outbound_message_id"] = message_id

    save_state(state_dir, state)
    record_send_event(state_dir, state)

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
        return 0 if result.replied else 4

    emit_json(state)
    return 0


def cmd_status(args: argparse.Namespace) -> int:
    state_dir = Path(args.state_dir)
    state = load_state(state_dir, args.challenge_id)

    if args.refresh and state.get("status") == "pending":
        token = ensure_token(args.token, False)
        reply, new_seen = poll_for_reply_once(
            api_base=args.api_base,
            token=token,
            challenge=state,
        )
        if new_seen:
            seen = list(dict.fromkeys(list(state.get("seen_inbound_message_ids", [])) + new_seen))
            state["seen_inbound_message_ids"] = seen
        if reply:
            state = mark_reply_received(state, reply)
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
    return 0 if result.replied else 4


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="human_approval_gate",
        description="Send human approval request emails and wait for the first same-thread reply.",
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
    request_parser.add_argument(
        "--challenge-type",
        default="two_factor",
        choices=list(CHALLENGE_TYPES),
        help="auth blocker type being escalated",
    )
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
        "--page-state",
        default="",
        help="explicit browser state, required for two_factor challenges",
    )
    request_parser.add_argument(
        "--two-factor-method",
        default="",
        metavar="{sms,email,auth_app,device_tap,other}",
        help="specific 2FA method in use",
    )
    request_parser.add_argument(
        "--verification-destination",
        default="",
        help="where the code/approval was sent (masked phone/email/device label)",
    )
    request_parser.add_argument(
        "--password-env-key",
        default=DEFAULT_PASSWORD_ENV_KEY,
        help="workspace env key checked before asking for password help",
    )
    request_parser.add_argument(
        "--password-lookup-status",
        default="",
        help="honest note about what password sources were already checked",
    )
    request_parser.add_argument(
        "--screenshot",
        action="append",
        default=[],
        help="current browser screenshot to attach; repeatable and required",
    )
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

    wait_parser = subparsers.add_parser("wait", help="wait for the first reply in the challenge thread")
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
