import datetime
import hashlib
import json
import os
import uuid
from email.utils import getaddresses, parseaddr
from typing import Any, Dict, List, Optional, Tuple

import azure.functions as func
import tomllib
from azure.servicebus import ServiceBusClient, ServiceBusMessage
from azure.storage.blob import BlobClient

_CONFIG_CACHE = None


def _load_configs() -> Dict[str, Any]:
    global _CONFIG_CACHE
    if _CONFIG_CACHE is not None:
        return _CONFIG_CACHE

    gateway_path = os.getenv("GATEWAY_CONFIG_PATH", "gateway.toml")
    employee_path = os.getenv("EMPLOYEE_CONFIG_PATH", "employee.toml")

    with open(gateway_path, "rb") as handle:
        gateway = tomllib.load(handle)
    with open(employee_path, "rb") as handle:
        employees = tomllib.load(handle)

    employee_entries = employees.get("employees", [])
    service_addresses = set()
    address_to_employee = {}
    for employee in employee_entries:
        employee_id = employee.get("id")
        for address in employee.get("addresses", []) or []:
            normalized = normalize_email(address)
            service_addresses.add(normalized)
            if normalized not in address_to_employee:
                address_to_employee[normalized] = employee_id

    routes = {}
    channel_defaults = {}
    for route in gateway.get("routes", []) or []:
        channel = normalize_channel(route.get("channel"))
        if not channel:
            continue
        key = normalize_route_key(channel, route.get("key", ""))
        target = {
            "employee_id": route.get("employee_id"),
            "tenant_id": route.get("tenant_id"),
        }
        if key == "*":
            channel_defaults[channel] = target
        else:
            routes[(channel, key)] = target

    defaults = gateway.get("defaults", {})
    _CONFIG_CACHE = {
        "defaults": defaults,
        "routes": routes,
        "channel_defaults": channel_defaults,
        "service_addresses": service_addresses,
        "address_to_employee": address_to_employee,
    }
    return _CONFIG_CACHE


def normalize_channel(value: Optional[str]) -> Optional[str]:
    if not value:
        return None
    value = value.strip().lower()
    if value == "google_docs":
        return "google_docs"
    if value == "sms":
        return "sms"
    if value == "bluebubbles" or value == "imessage":
        return "bluebubbles"
    if value in {"email", "slack", "discord", "telegram", "whatsapp"}:
        return value
    return value


def normalize_email(value: str) -> str:
    return value.strip().lower()


def normalize_phone(value: str) -> str:
    return "".join(ch for ch in value if ch.isdigit() or ch == "+")


def normalize_route_key(channel: str, key: str) -> str:
    key = key.strip()
    if key == "*":
        return "*"
    if channel == "email":
        return normalize_email(key)
    if channel == "sms":
        return normalize_phone(key)
    return key


def resolve_route(channel: str, route_key: str, config: Dict[str, Any]) -> Optional[Dict[str, str]]:
    normalized = normalize_route_key(channel, route_key)
    target = config["routes"].get((channel, normalized))
    if not target:
        target = config["channel_defaults"].get(channel)
    if not target and channel == "email":
        employee_id = config["address_to_employee"].get(normalized)
        if employee_id:
            target = {
                "employee_id": employee_id,
                "tenant_id": config["defaults"].get("tenant_id"),
            }
    if not target:
        default_employee = config["defaults"].get("employee_id")
        if default_employee:
            target = {
                "employee_id": default_employee,
                "tenant_id": config["defaults"].get("tenant_id"),
            }
    if not target or not target.get("employee_id"):
        return None
    tenant_id = target.get("tenant_id") or config["defaults"].get("tenant_id") or "default"
    return {"tenant_id": tenant_id, "employee_id": target["employee_id"]}


def collect_email_candidates(payload: Dict[str, Any]) -> List[str]:
    candidates: List[str] = []
    for field in ["To", "Cc", "Bcc"]:
        value = payload.get(field)
        if isinstance(value, str) and value.strip():
            candidates.append(value)
    for field in ["ToFull", "CcFull", "BccFull"]:
        entries = payload.get(field)
        if isinstance(entries, list):
            for entry in entries:
                email_value = entry.get("Email") if isinstance(entry, dict) else None
                if email_value:
                    candidates.append(email_value)
    headers = payload.get("Headers") or []
    header_names = {
        "X-Original-To",
        "Delivered-To",
        "Envelope-To",
        "X-Envelope-To",
        "X-Forwarded-To",
        "X-Original-Recipient",
        "Original-Recipient",
    }
    for header in headers:
        name = header.get("Name") if isinstance(header, dict) else None
        if name in header_names:
            value = header.get("Value")
            if isinstance(value, str) and value.strip():
                candidates.append(value)
    return candidates


def find_service_address(payload: Dict[str, Any], service_addresses: set) -> Optional[str]:
    candidates = collect_email_candidates(payload)
    for candidate in candidates:
        for _, addr in getaddresses([candidate]):
            normalized = normalize_email(addr)
            if normalized in service_addresses:
                return normalized
    return None


def resolve_container_sas_url() -> str:
    sas_url = os.getenv("AZURE_STORAGE_CONTAINER_SAS_URL")
    if sas_url:
        return sas_url.strip()
    account = os.getenv("AZURE_STORAGE_ACCOUNT")
    container = os.getenv("AZURE_STORAGE_CONTAINER")
    sas_token = os.getenv("AZURE_STORAGE_SAS_TOKEN")
    if not account or not container or not sas_token:
        raise RuntimeError("Missing Azure storage SAS configuration")
    sas_token = sas_token.lstrip("?")
    return f"https://{account}.blob.core.windows.net/{container}?{sas_token}"


def build_blob_url(container_sas_url: str, path: str) -> str:
    base, _, query = container_sas_url.partition("?")
    base = base.rstrip("/")
    if query:
        return f"{base}/{path}?{query}"
    return f"{base}/{path}"


def upload_raw_payload(raw_payload: bytes, envelope_id: str, received_at: datetime.datetime) -> str:
    container = os.getenv("AZURE_STORAGE_CONTAINER")
    if not container:
        raise RuntimeError("AZURE_STORAGE_CONTAINER is required")
    date_prefix = received_at.strftime("%Y/%m/%d")
    blob_path = f"ingestion_raw/{date_prefix}/{envelope_id}.bin"
    container_sas_url = resolve_container_sas_url()
    blob_url = build_blob_url(container_sas_url, blob_path)
    blob = BlobClient.from_blob_url(blob_url)
    blob.upload_blob(raw_payload, overwrite=True)
    return f"azure://{container}/{blob_path}"


def build_dedupe_key(tenant_id: str, employee_id: str, channel: str, external_id: Optional[str], raw_payload: bytes) -> str:
    if external_id:
        base = external_id
    elif raw_payload:
        base = hashlib.md5(raw_payload).hexdigest()
    else:
        base = str(uuid.uuid4())
    return f"{tenant_id}:{employee_id}:{channel}:{base}"


def resolve_queue_name(employee_id: str) -> str:
    base = os.getenv("SERVICE_BUS_QUEUE_NAME", "ingestion")
    per_employee = os.getenv("SERVICE_BUS_QUEUE_PER_EMPLOYEE", "true").lower() in {"1", "true", "yes", "on"}
    if per_employee:
        return f"{base}-{employee_id}"
    return base


def enqueue_message(envelope: Dict[str, Any]) -> None:
    connection_string = os.getenv("SERVICE_BUS_CONNECTION_STRING")
    if not connection_string:
        raise RuntimeError("SERVICE_BUS_CONNECTION_STRING is required")
    queue_name = resolve_queue_name(envelope["employee_id"])
    dedupe_key = envelope.get("dedupe_key")
    message = ServiceBusMessage(json.dumps(envelope), message_id=dedupe_key)
    with ServiceBusClient.from_connection_string(connection_string) as client:
        with client.get_queue_sender(queue_name) as sender:
            sender.send_messages(message)


def parse_postmark_payload(payload: Dict[str, Any], service_address: str) -> Dict[str, Any]:
    from_full = payload.get("FromFull") or {}
    sender = from_full.get("Email")
    sender_name = from_full.get("Name")
    if not sender:
        sender = parseaddr(payload.get("From") or "")[1]
    sender = sender or "unknown"
    subject = payload.get("Subject")
    text_body = payload.get("TextBody")
    html_body = payload.get("HtmlBody")
    message_id = payload.get("MessageID") or payload.get("MessageId")
    headers = payload.get("Headers") or []
    in_reply_to = None
    references = None
    for header in headers:
        if not isinstance(header, dict):
            continue
        name = header.get("Name")
        if name == "In-Reply-To":
            in_reply_to = header.get("Value")
        if name == "References":
            references = header.get("Value")
    reply_to_value = payload.get("ReplyTo")
    reply_to = []
    if reply_to_value:
        reply_to = [addr for _, addr in getaddresses([reply_to_value]) if addr]
    if not reply_to:
        reply_to = [sender]
    thread_id = message_id or str(uuid.uuid4())

    return {
        "sender": sender,
        "sender_name": sender_name,
        "recipient": service_address,
        "subject": subject,
        "text_body": text_body,
        "html_body": html_body,
        "thread_id": thread_id,
        "message_id": message_id,
        "attachments": [],
        "reply_to": reply_to,
        "metadata": {
            "in_reply_to": in_reply_to,
            "references": references,
        },
    }


def main(req: func.HttpRequest) -> func.HttpResponse:
    token = os.getenv("POSTMARK_INBOUND_TOKEN")
    if token:
        header = req.headers.get("x-postmark-token") or req.headers.get("X-Postmark-Token")
        if header != token:
            return func.HttpResponse(
                json.dumps({"status": "invalid_token"}), status_code=401, mimetype="application/json"
            )

    try:
        raw_payload = req.get_body()
        payload = json.loads(raw_payload.decode("utf-8"))
    except Exception:
        return func.HttpResponse(
            json.dumps({"status": "bad_json"}), status_code=400, mimetype="application/json"
        )

    config = _load_configs()
    service_address = find_service_address(payload, config["service_addresses"])
    if not service_address:
        return func.HttpResponse(
            json.dumps({"status": "no_route"}), status_code=200, mimetype="application/json"
        )

    route = resolve_route("email", service_address, config)
    if not route:
        return func.HttpResponse(
            json.dumps({"status": "no_route"}), status_code=200, mimetype="application/json"
        )

    received_at = datetime.datetime.now(tz=datetime.timezone.utc)
    envelope_id = str(uuid.uuid4())
    payload_data = parse_postmark_payload(payload, service_address)
    dedupe_key = build_dedupe_key(
        route["tenant_id"], route["employee_id"], "email", payload_data.get("message_id"), raw_payload
    )
    raw_payload_ref = upload_raw_payload(raw_payload, envelope_id, received_at)

    envelope = {
        "envelope_id": envelope_id,
        "received_at": received_at.isoformat(),
        "tenant_id": route["tenant_id"],
        "employee_id": route["employee_id"],
        "channel": "email",
        "external_message_id": payload_data.get("message_id"),
        "dedupe_key": dedupe_key,
        "payload": payload_data,
        "raw_payload_ref": raw_payload_ref,
    }

    enqueue_message(envelope)

    return func.HttpResponse(
        json.dumps({"status": "accepted"}), status_code=200, mimetype="application/json"
    )
