#!/usr/bin/env python3
import argparse
import base64
import datetime as dt
import hashlib
import hmac
import json
import os
import sys
import time
import uuid
import urllib.parse
import urllib.request


def env_with_scale_oliver(name: str, default: str = "") -> str:
    prefixed = os.getenv(f"SCALE_OLIVER_{name}")
    if prefixed and prefixed.strip():
        return prefixed.strip()
    value = os.getenv(name)
    if value and value.strip():
        return value.strip()
    return default


def parse_connection_string(conn_str: str) -> dict:
    parts = {}
    for item in conn_str.split(";"):
        if "=" in item:
            key, value = item.split("=", 1)
            parts[key] = value
    return parts


def build_sas_token(resource_uri: str, key_name: str, key: str, ttl_seconds: int) -> str:
    expiry = int(time.time()) + ttl_seconds
    encoded_uri = urllib.parse.quote_plus(resource_uri)
    string_to_sign = f"{encoded_uri}\n{expiry}".encode("utf-8")
    key_bytes = base64.b64decode(key)
    signature = base64.b64encode(hmac.new(key_bytes, string_to_sign, hashlib.sha256).digest())
    encoded_sig = urllib.parse.quote_plus(signature)
    return (
        f"SharedAccessSignature sr={encoded_uri}&sig={encoded_sig}&se={expiry}&skn={key_name}"
    )


def build_envelope(employee_id: str, index: int) -> dict:
    envelope_id = str(uuid.uuid4())
    now = dt.datetime.utcnow().replace(tzinfo=dt.timezone.utc)
    thread_id = f"load-test-thread-{index:04d}"
    message_id = f"load-test-msg-{index:04d}-{uuid.uuid4()}"
    payload = {
        "sender": f"loadtest+{index}@example.com",
        "sender_name": "Load Test",
        "recipient": "service@example.com",
        "subject": f"Load test task {index}",
        "text_body": f"Please summarize task {index} in one sentence and confirm completion.",
        "html_body": None,
        "thread_id": thread_id,
        "message_id": message_id,
        "attachments": [],
        "reply_to": ["loadtest@example.com"],
        "metadata": {},
    }
    return {
        "envelope_id": envelope_id,
        "received_at": now.isoformat().replace("+00:00", "Z"),
        "tenant_id": None,
        "employee_id": employee_id,
        "channel": "email",
        "external_message_id": message_id,
        "dedupe_key": f"load-test-{envelope_id}",
        "payload": payload,
        "raw_payload_ref": None,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Fanout load test messages to Service Bus.")
    parser.add_argument("--count", type=int, default=200, help="Number of messages to send.")
    parser.add_argument(
        "--queue",
        default=env_with_scale_oliver("SERVICE_BUS_QUEUE_NAME", "ingestion"),
        help="Service Bus queue name (default: SCALE_OLIVER_SERVICE_BUS_QUEUE_NAME, SERVICE_BUS_QUEUE_NAME, or 'ingestion').",
    )
    parser.add_argument(
        "--employee-id",
        default=os.getenv("EMPLOYEE_ID", "little_bear"),
        help="Employee id to target (default: env EMPLOYEE_ID or little_bear).",
    )
    parser.add_argument(
        "--ttl-seconds",
        type=int,
        default=3600,
        help="SAS token TTL in seconds (default: 3600).",
    )
    args = parser.parse_args()

    conn_str = env_with_scale_oliver("SERVICE_BUS_CONNECTION_STRING")
    if not conn_str:
        print(
            "Missing SCALE_OLIVER_SERVICE_BUS_CONNECTION_STRING/SERVICE_BUS_CONNECTION_STRING",
            file=sys.stderr,
        )
        return 1

    parts = parse_connection_string(conn_str)
    endpoint = parts.get("Endpoint")
    key_name = parts.get("SharedAccessKeyName")
    key = parts.get("SharedAccessKey")
    if not endpoint or not key_name or not key:
        print("Invalid SERVICE_BUS_CONNECTION_STRING", file=sys.stderr)
        return 1

    endpoint = endpoint.replace("sb://", "https://").rstrip("/")
    resource_uri = f"{endpoint}/{args.queue}"
    sas_token = build_sas_token(resource_uri, key_name, key, args.ttl_seconds)

    url = f"{resource_uri}/messages"
    headers = {
        "Authorization": sas_token,
        "Content-Type": "application/json",
    }

    sent = 0
    for idx in range(args.count):
        body = json.dumps(build_envelope(args.employee_id, idx)).encode("utf-8")
        req = urllib.request.Request(url, data=body, headers=headers, method="POST")
        try:
            with urllib.request.urlopen(req, timeout=10) as resp:
                if resp.status not in (200, 201, 202):
                    print(f"Unexpected status {resp.status} for message {idx}", file=sys.stderr)
                    return 1
        except Exception as exc:
            print(f"Failed to send message {idx}: {exc}", file=sys.stderr)
            return 1
        sent += 1
        if sent % 25 == 0:
            print(f"Sent {sent}/{args.count}")

    print(f"Successfully sent {sent} messages to {args.queue}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
