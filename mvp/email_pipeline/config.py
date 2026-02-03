from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import os


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_WORKSPACE_ROOT = REPO_ROOT / "mvp" / "email_pipeline" / "workspaces"
DEFAULT_OUTBOX_DIR = REPO_ROOT / "mvp" / "email_pipeline" / "outbox"
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
    inbound_host: str = _env("INBOUND_SMTP_HOST", "127.0.0.1")
    inbound_port: int = _env_int("INBOUND_SMTP_PORT", 8025)
    inbound_address: str = _env("INBOUND_ADDRESS", "mini-mouse@deep-tutor.com")

    outbound_mode: str = _env("OUTBOUND_MODE", "smtp")
    outbound_host: str = _env("OUTBOUND_SMTP_HOST", "127.0.0.1")
    outbound_port: int = _env_int("OUTBOUND_SMTP_PORT", 8026)
    outbound_from: str = _env("OUTBOUND_FROM", "mini-mouse@deep-tutor.com")

    start_outbox_server: bool = _env_bool("START_OUTBOX_SERVER", True)
    outbox_dir: Path = Path(_env("OUTBOX_DIR", str(DEFAULT_OUTBOX_DIR)))

    workspace_root: Path = Path(_env("WORKSPACE_ROOT", str(DEFAULT_WORKSPACE_ROOT)))
    code_model: str = _env("CODEX_MODEL", "gpt-5.1-codex-max")

    processed_ids_path: Path = Path(
        _env("PROCESSED_IDS_PATH", str(DEFAULT_STATE_DIR / "postmark_processed_ids.txt"))
    )

    dt_base_url: str = _env("DT_BASE_URL", "http://localhost:8081")

    mongodb_uri: str = _env("MONGODB_URI", "mongodb://localhost:27017")
    mongodb_db: str = _env("MONGODB_DB", "icebrew_mvp")
    use_mongodb: bool = _env_bool("USE_MONGODB", True)

    echo_attachments: bool = _env_bool("ECHO_ATTACHMENTS", True)

    postmark_token: str = _env("POSTMARK_SERVER_TOKEN", "")
    azure_storage_connection_string: str = _env("AZURE_STORAGE_CONNECTION_STRING", "")

    codex_disabled: bool = _env_bool("CODEX_DISABLED", False)


def load_settings() -> Settings:
    return Settings()
