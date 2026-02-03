from __future__ import annotations

from dataclasses import asdict
from datetime import datetime
from typing import Any, Dict, Optional

from .config import Settings

try:
    from pymongo import MongoClient
except Exception:  # pragma: no cover
    MongoClient = None


class MongoStore:
    def __init__(self, settings: Settings) -> None:
        if MongoClient is None:
            raise RuntimeError("pymongo is not installed")
        self.client = MongoClient(settings.mongodb_uri)
        self.db = self.client[settings.mongodb_db]
        self.inbound = self.db["inbound_messages"]
        self.outbound = self.db["outbound_messages"]

    def record_inbound(self, payload: Dict[str, Any]) -> None:
        payload = dict(payload)
        payload["_recorded_at"] = datetime.utcnow()
        self.inbound.insert_one(payload)

    def record_outbound(self, payload: Dict[str, Any]) -> None:
        payload = dict(payload)
        payload["_recorded_at"] = datetime.utcnow()
        self.outbound.insert_one(payload)


def get_store(settings: Settings) -> Optional[MongoStore]:
    if not settings.use_mongodb:
        return None
    try:
        return MongoStore(settings)
    except Exception:
        return None
