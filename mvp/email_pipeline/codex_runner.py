from __future__ import annotations

import json
import os
import subprocess
from pathlib import Path
from typing import List

from .config import REPO_ROOT


CODEX_CONFIG_BLOCK_TEMPLATE = """# IMPORTANT: Use your Azure *deployment name* here (e.g., \"gpt5codex-prod\")
model = \"gpt-5.2-codex\"
model_provider = \"azure\"
model_reasoning_effort = \"xhigh\"

[model_providers.azure]
name = \"Azure OpenAI\"
# Use your resource endpoint and include /openai/v1
base_url = \"{azure_endpoint}\"
# This is the ENV VAR NAME, not the key:
env_key = \"AZURE_OPENAI_API_KEY_BACKUP\"
wire_api = \"responses\"
"""


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


def _ensure_codex_config(model_name: str) -> None:
    config_path = Path.home() / ".codex" / "config.toml"
    config_path.parent.mkdir(parents=True, exist_ok=True)

    azure_endpoint = os.getenv("AZURE_OPENAI_ENDPOINT_BACKUP", "")
    if not azure_endpoint:
        raise RuntimeError("AZURE_OPENAI_ENDPOINT_BACKUP is not set in .env or environment.")
    if not azure_endpoint.endswith("/openai/v1"):
        azure_endpoint = azure_endpoint.rstrip("/") + "/openai/v1"

    block = CODEX_CONFIG_BLOCK_TEMPLATE.format(model_name=model_name, azure_endpoint=azure_endpoint).strip()

    existing = config_path.read_text(encoding="utf-8") if config_path.exists() else ""
    marker = "# IMPORTANT: Use your Azure *deployment name* here"
    if marker in existing:
        before, _ = existing.split(marker, 1)
        updated = before.rstrip() + "\n\n" + block + "\n"
    else:
        updated = existing.rstrip()
        if updated:
            updated += "\n\n"
        updated += block + "\n"

    config_path.write_text(updated, encoding="utf-8")


def _parse_codex_response(output: str) -> str:
    response_parts: List[str] = []
    for line in output.splitlines():
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        etype = event.get("type") or ""
        if etype == "item.delta":
            item = event.get("item", {})
            if item.get("type") == "agent_message":
                delta = event.get("delta", {})
                fragment = (
                    delta.get("text")
                    or delta.get("output")
                    or delta.get("aggregated_output")
                )
                if fragment:
                    response_parts.append(fragment)
            continue
        if etype == "item.completed":
            item = event.get("item", {})
            if item.get("type") == "agent_message":
                text = item.get("text")
                if text:
                    response_parts.append(text)
            continue
        if etype.endswith(".delta"):
            delta = event.get("delta", {})
            fragment = delta.get("text") or delta.get("output")
            if fragment:
                response_parts.append(fragment)
            continue
        if etype.endswith(".completed"):
            text = event.get("text")
            if text:
                response_parts.append(text)
            continue

    return "".join(response_parts).strip()


def run_codex_reply(
    prompt: str,
    workspace_dir: Path,
    reply_path: Path,
    model_name: str,
    *,
    codex_disabled: bool = False,
) -> str:
    _load_env_file(REPO_ROOT / ".env")

    if codex_disabled:
        reply_path.write_text("Codex disabled. Received your email and saved attachments.", encoding="utf-8")
        return reply_path.read_text(encoding="utf-8")

    api_key = os.getenv("AZURE_OPENAI_API_KEY_BACKUP", "")
    if not api_key:
        reply_path.write_text("Missing AZURE_OPENAI_API_KEY_BACKUP. Cannot run Codex.", encoding="utf-8")
        return reply_path.read_text(encoding="utf-8")

    _ensure_codex_config(model_name)

    env = os.environ.copy()
    env["AZURE_OPENAI_API_KEY_BACKUP"] = api_key

    cmd = [
        "codex",
        "exec",
        "--json",
        "--skip-git-repo-check",
        "-m",
        model_name,
        "-c",
        'web_search="disabled"',
        "-c",
        'model_providers.azure.env_key="AZURE_OPENAI_API_KEY_BACKUP"',
        "--cd",
        str(workspace_dir),
        "--dangerously-bypass-approvals-and-sandbox",
        prompt,
    ]

    try:
        result = subprocess.run(
            cmd,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            env=env,
            cwd=str(workspace_dir),
        )
    except FileNotFoundError:
        reply_path.write_text("Codex CLI not found on PATH.", encoding="utf-8")
        return reply_path.read_text(encoding="utf-8")

    response_text = _parse_codex_response(result.stdout or "")

    if not reply_path.exists():
        if not response_text:
            response_text = "No response returned by Codex."
        reply_path.write_text(response_text, encoding="utf-8")

    if reply_path.exists():
        return reply_path.read_text(encoding="utf-8")

    return response_text
