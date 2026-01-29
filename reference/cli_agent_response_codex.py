"""codex_chatbot_refactored.py
================================

Minimal, self‑contained helpers to:

1.  **`pdfs_to_markdown_workspace`** – Convert one or more PDF files to plain
    markdown and store them in a deterministic *Codex* workspace directory that
    is named after the combined MD5 hash of the source files. The function
    returns the absolute path to the workspace.

2.  **`stream_codex_answer`** – Given a *workspace* directory (produced by the
    first helper) and a *question*, call the `codex` CLI and yield a streaming
    response formatted as

        ``<think>…</think><response>…</response>``.

These two top‑level functions are the only public API exposed by this module, as
requested. Any additional utilities are defined *inside* the functions to
avoid leaking extra symbols.

Notes
-----
* The implementation mirrors the behaviour of the original
  `codex_chatbot_test.py` experiment while dramatically reducing surface area.
* The Azure OpenAI credential is read from the backup environment variable
  `AZURE_OPENAI_API_KEY_BACKUP` on each invocation so that callers do not need
  to manipulate their primary key settings.
* The module relies on the external *PyPDF2* package for text extraction and
  on the *Codex* CLI being available on the `$PATH`.
"""

from __future__ import annotations

import asyncio
import hashlib
import json
import os
import logging
import subprocess
import sys
import re
from functools import lru_cache
from pathlib import Path
from typing import AsyncGenerator, AsyncIterator, List, Sequence, Set

from dotenv import load_dotenv

# Project‑local helper for reproducible file hashing
PROJECT_ROOT = Path(__file__).resolve().parents[4]
if str(PROJECT_ROOT) not in sys.path:
    sys.path.insert(0, str(PROJECT_ROOT))

try:
    # `generate_file_id` lives in the main tutor pipeline utils and performs a
    # robust MD5 hash on the raw bytes of a file.
    from pipeline.science.pipeline.utils import generate_file_id
except Exception:  # pragma: no cover – fallback if dependency graph changes
    def generate_file_id(file_path: str | Path) -> str:  # type: ignore
        """Light‑weight MD5 helper used as a graceful fallback."""

        path = Path(file_path)
        digest = hashlib.md5()
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(8192), b""):
                digest.update(chunk)
        return digest.hexdigest()


CODEX_CONFIG_BLOCK_TEMPLATE = """# IMPORTANT: Use your Azure *deployment name* here (e.g., "gpt-5.2-codex")
model = "gpt-5.2-codex"
model_provider = "azure"
model_reasoning_effort = "xhigh"

[model_providers.azure]
name = "Azure OpenAI"
# Use your resource endpoint and include /openai/v1
base_url = "{azure_endpoint}"
# This is the ENV VAR NAME, not the key:
env_key = "AZURE_OPENAI_API_KEY_BACKUP"
wire_api = "responses"
"""


@lru_cache(maxsize=1)
def _load_env_sources() -> None:
    """Load .env files once so we can reuse the resolved API keys."""
    load_dotenv()
    env_path = PROJECT_ROOT / ".env"
    if env_path.exists():
        load_dotenv(env_path)


def _ensure_codex_config(model_name: str) -> None:
    """Ensure DeepTutor Agent config.toml contains the desired Azure settings."""

    config_path = Path.home() / ".codex" / "config.toml"
    config_path.parent.mkdir(parents=True, exist_ok=True)

    # Get the Azure endpoint from environment variable
    azure_endpoint = os.getenv("AZURE_OPENAI_ENDPOINT_BACKUP", "")
    if not azure_endpoint:
        raise RuntimeError(
            "AZURE_OPENAI_ENDPOINT_BACKUP is not set. "
            "Add it to your .env file before running server-side agent mode."
        )
    # Ensure the endpoint includes /openai/v1 if not already present
    if not azure_endpoint.endswith("/openai/v1"):
        if azure_endpoint.endswith("/"):
            azure_endpoint = azure_endpoint.rstrip("/") + "/openai/v1"
        else:
            azure_endpoint = azure_endpoint + "/openai/v1"

    block = CODEX_CONFIG_BLOCK_TEMPLATE.format(model_name=model_name, azure_endpoint=azure_endpoint).strip()

    if config_path.exists():
        existing = config_path.read_text(encoding="utf-8")
    else:
        existing = ""

    pattern = re.compile(
        r"# IMPORTANT: Use your Azure \*deployment name\* here.*?wire_api = \"responses\"",
        re.DOTALL,
    )

    if pattern.search(existing):
        updated = pattern.sub(block, existing)
    else:
        updated = existing.rstrip()
        if updated:
            updated += "\n\n"
        updated += block

    updated = updated.rstrip() + "\n"
    config_path.write_text(updated, encoding="utf-8")


def _run_npm_install(env: dict[str, str]) -> None:
    """Run the DeepTutor Agent installation check before invoking the agent."""
    try:
        result = subprocess.run(
            ["npm", "i", "-g", "@openai/codex"],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            env=env,
        )
    except FileNotFoundError as exc:  # pragma: no cover - unlikely in prod env
        raise RuntimeError(
            "npm is required to install the DeepTutor Agent. "
            "Please install Node.js and npm before using server-side agent mode."
        ) from exc

    if result.returncode != 0:
        payload = (result.stdout or "").strip()
        logger.warning("npm install returned %s: %s", result.returncode, payload)


def _configure_tavily_mcp(env: dict[str, str]) -> None:
    """Ensure Tavily MCP is registered before the agent runs."""

    tavily_api_key = str(os.getenv("TAVILY_API_KEY") or "")
    if not tavily_api_key:
        raise RuntimeError(
            "TAVILY_API_KEY is not set. "
            "Add it to your environment or .env file before running server-side agent mode."
        )

    env["TAVILY_API_KEY"] = tavily_api_key

    result = subprocess.run(
        [
            "codex",
            "mcp",
            "add",
            "tavily",
            "--url",
            f"https://mcp.tavily.com/mcp/?tavilyApiKey={tavily_api_key}",
        ],
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        env=env,
    )
    if result.returncode != 0:
        payload = (result.stdout or "").strip()
        raise RuntimeError(
            f"Registering Tavily MCP failed with code {result.returncode}: {payload}"
        )


def _prepare_codex_runtime(workspace: Path, model_name: str) -> dict[str, str]:
    """Prepare environment variables and config prior to DeepTutor Agent invocation."""

    _load_env_sources()
    api_key = os.getenv("AZURE_OPENAI_API_KEY_BACKUP")
    if not api_key:
        raise RuntimeError(
            "AZURE_OPENAI_API_KEY_BACKUP is not set. "
            "Add it to your .env file before running server-side agent mode."
        )

    env = os.environ.copy()
    env["AZURE_OPENAI_API_KEY_BACKUP"] = api_key

    _run_npm_install(env)
    # _configure_tavily_mcp(env)
    _ensure_codex_config(model_name)

    export_script = f'export AZURE_OPENAI_API_KEY_BACKUP="{api_key}"'
    try:
        subprocess.run(
            ["bash", "-lc", export_script],
            check=False,
            cwd=str(workspace),
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
    except FileNotFoundError:
        logger.debug("Skipping export command because bash is not available on PATH.")

    return env


# ---------------------------------------------------------------------------
# Public API – 1) PDF → Markdown workspace
# ---------------------------------------------------------------------------


def pdfs_to_markdown_workspace(pdf_paths: Sequence[str | Path]) -> Path:
    """Convert *pdf_paths* to markdown inside a hashed Codex workspace.

    Parameters
    ----------
    pdf_paths:
        A sequence of absolute or relative paths pointing to PDF documents.

    Returns
    -------
    Path
        Absolute path of the workspace directory that now contains the newly
        generated markdown files.
    """

    if not pdf_paths:
        raise ValueError("pdf_paths must contain at least one path.")

    # 1. Normalise & validate paths ------------------------------------------------
    resolved_pdfs: List[Path] = []
    for raw in pdf_paths:
        path = Path(raw).expanduser().resolve()
        if not path.exists():
            raise FileNotFoundError(f"PDF not found: {path}")
        resolved_pdfs.append(path)

    # 2. Compute stable combined hash ---------------------------------------------
    file_ids: List[str] = [generate_file_id(p) for p in resolved_pdfs]

    digest = hashlib.md5()
    for fid in sorted(file_ids):
        digest.update(fid.encode())
    workspace_id = digest.hexdigest()

    # 3. Create workspace directory ------------------------------------------------
    workspace_root = PROJECT_ROOT / "tmp" / "deeptutor_agent_workspaces"
    workspace_dir = workspace_root / workspace_id
    workspace_dir.mkdir(parents=True, exist_ok=True)

    # 4. Convert each PDF ----------------------------------------------------------
    try:
        from PyPDF2 import PdfReader  # Lazy import; heavy dependency
    except ImportError as exc:  # pragma: no cover – explicit for clarity
        raise RuntimeError(
            "PyPDF2 is required for PDF -> markdown conversion. "
            "Install with `pip install PyPDF2`."
        ) from exc

    existing_names: set[str] = set()

    def _safe_name(pdf: Path, fid: str) -> str:
        base = pdf.stem or fid[:8]
        candidate = f"{base}.md"
        if candidate not in existing_names:
            return candidate
        # Collision handling
        suffix = fid[:8]
        counter = 0
        while True:
            tail = f"_{suffix}" if counter == 0 else f"_{suffix}_{counter}"
            candidate = f"{base}{tail}.md"
            if candidate not in existing_names:
                return candidate
            counter += 1

    for pdf_path, fid in zip(resolved_pdfs, file_ids):
        md_name = _safe_name(pdf_path, fid)
        existing_names.add(md_name)
        md_path = workspace_dir / md_name

        if md_path.exists():
            # File already converted – skip work to keep the function idempotent
            continue

        # Extract raw text per page in a minimal markdown structure
        reader = PdfReader(str(pdf_path))
        sections: List[str] = []
        for idx, page in enumerate(reader.pages, start=1):
            text = (page.extract_text() or "").replace("\x00", "").strip()
            if text:
                sections.append(f"## Page {idx}\n\n{text}")
            else:
                sections.append(f"## Page {idx}\n\n*(No extractable text found on this page.)*")

        md_content = "\n\n".join(sections).strip() or "*No extractable text found in the provided PDF.*"
        md_path.write_text(md_content + "\n", encoding="utf-8")

    return workspace_dir


# ---------------------------------------------------------------------------
# Public API – 2) Codex Q&A
# ---------------------------------------------------------------------------

logger = logging.getLogger("tutorpipeline.science.cli_agent_response_codex")


def _create_thinking_step(
    title: str,
    content: str,
    step_type: str,
    status: str = "completed"
) -> str:
    """Create a JSON thinking step for structured streaming.

    Parameters
    ----------
    title:
        Short description of what this step is doing.
    content:
        Detailed content/output of the step.
    step_type:
        One of "operation", "thinking", or "request".
    status:
        One of "in_progress" or "completed".

    Returns
    -------
    str
        JSON string representation of the thinking step.
    """
    step = {
        "title": title,
        "content": content,
        "type": step_type,
        "status": status
    }
    return json.dumps(step, ensure_ascii=False)


def _determine_subagents(user_input):
    """Extract requested subagents from user input
    The sub agents are defined at the beginning of the user input
    like \"/make_slides <the rest of user's input>\", or \"/summary_generator <the rest of user's input>\".
    Based on the extracted subagents, we return the directory of the subagents files in the codebase under "/DeepTutorPipeline/tutorService/pipeline/science/pipeline/sub_agents"
    """
    subagent_mapping = {
        "make_slides": "slide_maker_agent.md",
        "summary_generator": "summary_generator_agent.md",
        "flashcard_creator": "flashcard_creator_agent.md",
        "quiz_generator": "quiz_generator_agent.md",
    }

    requested_subagents = []
    for command, subagent_dir in subagent_mapping.items():
        if user_input.strip().startswith(f"/{command}"):
            requested_subagents.append(subagent_dir)

    return requested_subagents


def _determine_skills(user_input):
    """Extract requested skills from user input
    The skills are defined at the beginning of the user input
    like \"/make_slides <the rest of user's input>\", or \"/summary_generator <the rest of user's input>\".
    Based on the extracted skills, we return the directory of the skills files in the codebase under "/DeepTutorPipeline/tutorService/pipeline/science/pipeline/agent_skills"

    Also save the corresponding skills files into "~/.codex/skills/**/SKILL.md"
    """
    subagent_mapping = {
        "make_slides": "slide_maker_agent_skills.md",
        "summary_generator": "summary_generator_agent_skills.md",
        "flashcard_creator": "flashcard_creator_agent_skills.md",
        "quiz_generator": "quiz_generator_agent_skills.md",
    }

    requested_skills = []
    for command, subagent_dir in subagent_mapping.items():
        if user_input.strip().startswith(f"/{command}"):
            requested_skills.append(subagent_dir)

    return requested_skills


async def stream_codex_answer(
    workspace_dir: str | Path,
    question: str,
    *,
    model: str = "gpt-5.1-codex-max",
    # skills: Sequence[str] | None = None,
    # subagents: Sequence[str] | None = None,
) -> AsyncGenerator[str, None]:
    """Yield a streaming Codex answer for *question* over *workspace_dir*.

    The generator emits incremental chunks, first surrounded by a ``<think>``
    block (agent reasoning/tool calls) followed by a ``<response>`` block
    (answer). Example consumer::

        async for chunk in stream_codex_answer("/path/to/ws", "What is ...?"):
            print(chunk, end="")
    """
    # First determine which subagents to load based on user input
    requested_subagents_paths = _determine_subagents(question)

    # The main function
    workspace = Path(workspace_dir).expanduser().resolve()
    if not workspace.exists() or not workspace.is_dir():
        raise FileNotFoundError(f"Workspace directory not found: {workspace}")

    command_env = _prepare_codex_runtime(workspace, model)

    question_refined = question.replace("@", "file_path: ")

    system_instruction = (
        """SYSTEM: You identity is the DeepTutor Agent based on OpenAI GPT-5 models. Your job is answering questions based on a workspace with raw text files for one user's entire DeepTutor library. All files in the workspace are papers or documents (no code files). Your role is as a professional tutor to provide accurate answers grounded in the workspace context.

        Make sure in the response every the context files are cited with @filename.md / @filedirectory as references whenever needed. If same file shows up multiple times, attach the shorted file path to that file.

        Format the response in Markdown format and make the content structured and easy to understand. Use bold, italics, bullet points, underlines, and numbered lists where appropriate (all in markdown syntax).

        For math formulas do NOT use \( \) or \[ \]; only $...$ or $$...$$.
        """
    )
    # Later we change the prompt to include subagents info if any subagents are requested
    prompt = f"""{system_instruction}\n\n{question.lstrip()}"""
    subagents_info = "" # (Load from the requested_subagents_paths)
    prompt_with_subagents = prompt + "\n\n" + subagents_info if requested_subagents_paths else prompt

    # ---------------------------------------------------------------------
    # Helper: build CLI command
    # ---------------------------------------------------------------------
    def build_cmd(prompt: str) -> List[str]:
        return [
            "codex",
            "exec",
            "--json",
            "--skip-git-repo-check",
            "-m", model,
            "-c", 'model_providers.azure.env_key="AZURE_OPENAI_API_KEY_BACKUP"',
            # "-C", str(workspace),
            "--cd", str(workspace),

            # Match Agent (full access): no approvals, no sandbox, network allowed
            "--dangerously-bypass-approvals-and-sandbox",

            # "--sandbox", "workspace-write",

            prompt,
        ]

    # ------------------------------------------------------------------
    # Spawn DeepTutor Agent as an async subprocess and stream JSON events
    # ------------------------------------------------------------------
    raw_logs: List[str] = []
    seen_event = False
    max_log_lines = 20

    async def iter_events() -> AsyncIterator[dict]:
        nonlocal seen_event, raw_logs
        proc = await asyncio.create_subprocess_exec(
            *build_cmd(prompt),
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.STDOUT,
            env=command_env,
            cwd=str(workspace),
        )

        assert proc.stdout is not None  # mypy: ignore[assert]

        # Read raw chunks to avoid asyncio's 64 KiB per-line StreamReader limit.
        buffer = bytearray()

        def handle_line(line: str) -> dict | None:
            """Process a full stdout line from Codex."""
            nonlocal seen_event
            try:
                payload = json.loads(line)
            except json.JSONDecodeError:
                stripped = line.strip()
                if stripped:
                    raw_logs.append(stripped)
                    if len(raw_logs) > max_log_lines:
                        del raw_logs[:-max_log_lines]
                    logger.debug("DeepTutor Agent raw stdout: %s", stripped)
                return None

            seen_event = True
            return payload

        while True:
            chunk = await proc.stdout.read(65536)
            if not chunk:
                if buffer:
                    line_bytes = bytes(buffer)
                    buffer.clear()
                    line = line_bytes.decode(errors="ignore")
                    payload = handle_line(line)
                    if payload is not None:
                        yield payload
                break

            buffer.extend(chunk)

            while True:
                newline_index = buffer.find(b"\n")
                if newline_index == -1:
                    break

                line_bytes = buffer[:newline_index]
                del buffer[:newline_index + 1]

                line = line_bytes.decode(errors="ignore")
                payload = handle_line(line)
                if payload is None:
                    continue

                yield payload

        returncode = await proc.wait()
        if returncode != 0:
            tail = "\n".join(raw_logs[-5:])
            raise RuntimeError(
                f"DeepTutor Agent exited with code {returncode}. "
                f"Last output:\n{tail or 'No additional output captured.'}"
            )
        if not seen_event:
            tail = "\n".join(raw_logs[-5:])
            raise RuntimeError(
                "DeepTutor Agent produced no JSON events. "
                f"Last output:\n{tail or 'No additional output captured.'}"
            )

    # ------------------------------------------------------------------
    # Transform events -> contiguous text stream with tags
    # ------------------------------------------------------------------
    async def format_events() -> AsyncGenerator[str, None]:
        think_open = True
        response_open = False
        produced_text = False
        event_types: Set[str] = set()

        last_chunk_category = "init"
        last_chunk_ended_newline = True

        # Buffer for accumulating reasoning text to emit as JSON steps
        reasoning_buffer: List[str] = []

        def prepare_chunk(chunk: str, *, category: str = "generic") -> str:
            nonlocal last_chunk_category, last_chunk_ended_newline
            if category == "command_execution":
                if not chunk.endswith("\n"):
                    chunk += "\n"
                if not chunk.endswith("\n\n"):
                    chunk += "\n"
            elif category == "reasoning":
                if (
                    chunk
                    and not chunk.startswith("\n")
                    and last_chunk_ended_newline
                    and last_chunk_category not in {"reasoning", "think_open"}
                ):
                    chunk = "\n" + chunk
                if not chunk.endswith("\n"):
                    chunk += "\n"
            elif category in ("json_step", "json_meta"):
                # JSON steps should be on their own line
                if not chunk.endswith("\n"):
                    chunk += "\n"
            last_chunk_category = category
            last_chunk_ended_newline = chunk.endswith("\n")
            return chunk

        yield prepare_chunk("<think>\n", category="think_open")
        # Emit JSON thinking mode marker so the client knows to parse JSON steps
        yield prepare_chunk('{"_meta":"json_thinking_start"}\n', category="json_meta")

        async for event in iter_events():
            etype = event.get("type")
            if not etype:
                continue
            event_types.add(etype)

            if etype.startswith("item."):
                item = event.get("item", {})
                itype = item.get("type")

                # Command execution start – emit as JSON thinking step
                if etype == "item.started" and itype == "command_execution":
                    cmd = item.get("command")
                    if cmd:
                        step_json = _create_thinking_step(
                            title="Executing Command",
                            content=cmd,
                            step_type="operation",
                            status="completed"
                        )
                        yield prepare_chunk(step_json, category="json_step")
                    continue

                # Incremental delta updates for agent_message / reasoning / etc.
                if etype == "item.delta":
                    if itype == "command_execution":
                        continue

                    delta = event.get("delta", {})
                    fragment = (
                        delta.get("text")
                        or delta.get("aggregated_output")
                        or delta.get("output")
                        or next((v for v in delta.values() if isinstance(v, str)), "")
                    )
                    if not fragment:
                        continue
                    category = "generic"

                    if itype == "agent_message":
                        if think_open:
                            yield prepare_chunk("</think>\n", category="think_close")
                            think_open = False
                        if not response_open:
                            yield prepare_chunk("<response>\n", category="response_open")
                            response_open = True
                        category = "agent_message"
                        if not fragment.endswith("\n"):
                            fragment += "\n"
                        if fragment.strip():
                            produced_text = True
                        yield prepare_chunk(fragment, category=category)
                    elif itype == "reasoning":
                        # Buffer reasoning deltas for JSON step emission
                        reasoning_buffer.append(fragment)
                    else:
                        # Generic delta handling
                        if not fragment.endswith("\n"):
                            fragment += "\n"
                        if fragment.strip():
                            produced_text = True
                        yield prepare_chunk(fragment, category="generic")
                    continue

                # Completed items – flush any remaining buffered output
                if etype == "item.completed":
                    # Skip command_execution completion - already emitted on start
                    if itype == "command_execution":
                        continue

                    if itype == "reasoning":
                        # Use buffered content if available, otherwise fall back to item text
                        text = "".join(reasoning_buffer) if reasoning_buffer else (item.get("text") or "")
                        reasoning_buffer.clear()
                        if text:
                            # Extract title from first line or use default
                            lines = text.strip().split('\n', 1)
                            title = lines[0][:60] if lines else "Analyzing"
                            # Clean up title (remove markdown bold markers if present)
                            title = title.replace("**", "").strip()
                            if not title:
                                title = "Analyzing"
                            content = lines[1].strip() if len(lines) > 1 else ""
                            step_json = _create_thinking_step(
                                title=title,
                                content=content,
                                step_type="thinking",
                                status="completed"
                            )
                            yield prepare_chunk(step_json, category="json_step")
                        continue

                    if itype == "agent_message":
                        if think_open:
                            yield prepare_chunk("</think>\n", category="think_close")
                            think_open = False
                        if not response_open:
                            yield prepare_chunk("<response>\n", category="response_open")
                            response_open = True
                        text = item.get("text") or ""
                        if text:
                            produced_text = produced_text or bool(text.strip())
                            if not text.endswith("\n"):
                                text += "\n"
                            yield prepare_chunk(text, category="agent_message")
                        continue

                    # For unhandled item types, attempt to forward any text content
                    fallback_text = item.get("text")
                    if fallback_text:
                        if think_open:
                            yield prepare_chunk("</think>\n", category="think_close")
                            think_open = False
                        if not response_open:
                            yield prepare_chunk("<response>\n", category="response_open")
                            response_open = True
                        produced_text = produced_text or bool(fallback_text.strip())
                        if not fallback_text.endswith("\n"):
                            fallback_text += "\n"
                        yield prepare_chunk(fallback_text, category="agent_message")
                    continue

            # Handle non-item events that still carry text payloads
            if etype.endswith(".delta"):
                delta = event.get("delta", {})
                fragment = (
                    delta.get("text")
                    or delta.get("output")
                    or next((v for v in delta.values() if isinstance(v, str)), "")
                )
                if not fragment:
                    continue
                if think_open:
                    yield prepare_chunk("</think>\n", category="think_close")
                    think_open = False
                if not response_open:
                    yield prepare_chunk("<response>\n", category="response_open")
                    response_open = True
                if not fragment.endswith("\n"):
                    fragment += "\n"
                if fragment.strip():
                    produced_text = True
                yield prepare_chunk(fragment, category="agent_message")
                continue

            if etype.endswith(".completed"):
                text = event.get("text") or ""
                if not text:
                    continue
                if think_open:
                    yield prepare_chunk("</think>\n", category="think_close")
                    think_open = False
                if not response_open:
                    yield prepare_chunk("<response>\n", category="response_open")
                    response_open = True
                produced_text = produced_text or bool(text.strip())
                if not text.endswith("\n"):
                    text += "\n"
                yield prepare_chunk(text, category="agent_message")
                continue

        if think_open:
            yield prepare_chunk("</think>\n", category="think_close")

        if response_open:
            if not produced_text:
                tail = "\n".join(raw_logs[-5:])
                raise RuntimeError(
                    "DeepTutor Agent completed without emitting assistant content. "
                    f"Event types observed: {sorted(event_types)}. "
                    f"Last output:\n{tail or 'No additional output captured.'}"
                )
            yield prepare_chunk("</response>", category="response_close")
        else:
            if produced_text:
                yield prepare_chunk("<response>\n</response>", category="response_empty")
            else:
                tail = "\n".join(raw_logs[-5:])
                raise RuntimeError(
                    "DeepTutor Agent did not emit any response text. "
                    f"Event types observed: {sorted(event_types)}. "
                    f"Last output:\n{tail or 'No additional output captured.'}"
                )

    # Finally, stream the formatted output to the caller
    async for chunk in format_events():
        yield chunk