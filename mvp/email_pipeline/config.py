from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import os


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_WORKSPACE_ROOT = REPO_ROOT / "mvp" / "email_pipeline" / "workspaces"
DEFAULT_STATE_DIR = REPO_ROOT / "mvp" / "email_pipeline" / "state"


def _load_env_file(path: Path) -> None:
    if not path.exists():
        return
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        os.environ.setdefault(key, value)


_load_env_file(REPO_ROOT / ".env")


def _env(key: str, default: str) -> str:
    return os.getenv(key, default)


def _env_int(key: str, default: int) -> int:
    try:
        return int(os.getenv(key, str(default)))
    except ValueError:
        return default


def _env_bool(key: str, default: bool) -> bool:
    raw = os.getenv(key)
    if raw is None:
        return default
    return raw.lower() in {"1", "true", "yes", "y"}


@dataclass(frozen=True)
class Settings:
    inbound_address: str = _env("INBOUND_ADDRESS", "mini-mouse@deep-tutor.com")

    outbound_from: str = _env("OUTBOUND_FROM", "mini-mouse@deep-tutor.com")

    workspace_root: Path = Path(_env("WORKSPACE_ROOT", str(DEFAULT_WORKSPACE_ROOT)))
    code_model: str = _env("CODEX_MODEL", "gpt-5.1-codex-max")
    monitor_webhook_port: int = _env_int("MONITOR_WEBHOOK_PORT", 9000)
    max_retries: int = _env_int("MAX_RETRIES", 2)

    processed_ids_path: Path = Path(
        _env("PROCESSED_IDS_PATH", str(DEFAULT_STATE_DIR / "postmark_processed_ids.txt"))
    )

    mongodb_uri: str = _env("MONGODB_URI", "mongodb://localhost:27017")
    mongodb_db: str = _env("MONGODB_DB", "icebrew_mvp")
    use_mongodb: bool = _env_bool("USE_MONGODB", True)

    postmark_token: str = _env("POSTMARK_SERVER_TOKEN", "")

    codex_disabled: bool = _env_bool("CODEX_DISABLED", False)


def load_settings() -> Settings:
    return Settings()
