from __future__ import annotations

from pathlib import Path

from .codex_runner import run_codex_reply
from .config import load_settings


def generate_response(
    workspace_dir: str,
    model: str = "codex",
) -> dict:
    """
    Generate an AI response for an email in the given workspace.

    Expects the workspace to contain:
        - email_inbox.md: The received email content
        - email_inbox_attachments/: Folder with received attachments

    Generates:
        - email_reply.md: The AI-generated response
        - email_reply_attachments/: Folder with any generated attachments

    Args:
        workspace_dir: Path to the workspace directory
        model: AI model to use ("codex" or future options)

    Returns:
        dict with keys:
            success (bool),
            reply_path (str),
            attachments_dir (str),
            error (str|None)
    """
    workspace = Path(workspace_dir)
    inbox_path = workspace / "email_inbox.md"
    attachments_dir = workspace / "email_inbox_attachments"
    reply_path = workspace / "email_reply.md"
    reply_attachments_dir = workspace / "email_reply_attachments"
    reply_attachments_dir.mkdir(parents=True, exist_ok=True)

    if not inbox_path.exists():
        return {
            "success": False,
            "reply_path": str(reply_path),
            "attachments_dir": str(reply_attachments_dir),
            "error": f"Missing inbox markdown: {inbox_path}",
        }

    inbox_text = inbox_path.read_text(encoding="utf-8", errors="replace")
    attachment_names = [p.name for p in attachments_dir.iterdir()] if attachments_dir.exists() else []

    prompt = _build_codex_prompt(inbox_text, attachment_names)

    if model != "codex":
        return {
            "success": False,
            "reply_path": str(reply_path),
            "attachments_dir": str(reply_attachments_dir),
            "error": f"Unsupported model: {model}",
        }

    settings = load_settings()
    try:
        run_codex_reply(
            prompt,
            workspace_dir=workspace,
            reply_path=reply_path,
            model_name=settings.code_model,
            codex_disabled=settings.codex_disabled,
        )
    except Exception as exc:
        return {
            "success": False,
            "reply_path": str(reply_path),
            "attachments_dir": str(reply_attachments_dir),
            "error": str(exc),
        }

    if not reply_path.exists():
        return {
            "success": False,
            "reply_path": str(reply_path),
            "attachments_dir": str(reply_attachments_dir),
            "error": "Reply file was not generated.",
        }

    return {
        "success": True,
        "reply_path": str(reply_path),
        "attachments_dir": str(reply_attachments_dir),
        "error": None,
    }


def _build_codex_prompt(body: str, attachment_names: list[str]) -> str:
    attachments_line = ", ".join(attachment_names) if attachment_names else "(none)"
    return (
        "You are the IceBrew email agent.\n"
        "Write a helpful reply to the incoming email.\n"
        "You must write the reply to a file named email_reply.md in the current directory.\n"
        "If you create any files to send back, place them in email_reply_attachments/.\n"
        "Do not include anything else outside the reply text in email_reply.md.\n\n"
        f"Attachments: {attachments_line}\n\n"
        "Body:\n"
        f"{body.strip()}\n"
    )
