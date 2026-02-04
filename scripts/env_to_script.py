#!/usr/bin/env python3
"""
Convert a repo-root .env file into a copy-friendly .env.script file.

Default output uses shell export lines for easy pasting into
environment-variable settings pages. Use --format env for KEY=VALUE lines.
If the repo-root .env is missing (common in git worktrees), the script
tries to locate an existing .env from other git worktrees.
"""
from __future__ import annotations

import argparse
import os
import shlex
import subprocess
import sys
from pathlib import Path


def warn(message: str) -> None:
    print(f"Warning: {message}", file=sys.stderr)


def parse_quoted_value(raw: str, line_no: int) -> tuple[str, bool]:
    quote = raw[0]
    if quote == "'":
        end = raw.find("'", 1)
        if end == -1:
            warn(f"Line {line_no}: unmatched single quote; using remainder")
            return raw[1:], True
        return raw[1:end], True

    # Double-quoted value with basic escape handling.
    value_chars: list[str] = []
    escaped = False
    i = 1
    while i < len(raw):
        ch = raw[i]
        if escaped:
            if ch == "n":
                value_chars.append("\n")
            elif ch == "r":
                value_chars.append("\r")
            elif ch == "t":
                value_chars.append("\t")
            else:
                value_chars.append(ch)
            escaped = False
        else:
            if ch == "\\":
                escaped = True
            elif ch == '"':
                return "".join(value_chars), True
            else:
                value_chars.append(ch)
        i += 1

    warn(f"Line {line_no}: unmatched double quote; using remainder")
    return "".join(value_chars), True


def strip_inline_comment(raw: str) -> str:
    for i, ch in enumerate(raw):
        if ch == "#":
            if i == 0:
                return ""
            if raw[i - 1].isspace():
                return raw[:i - 1].rstrip()
    return raw.strip()


def parse_value(raw: str, line_no: int) -> tuple[str, bool]:
    value = raw.strip()
    if not value:
        return "", False
    if value[0] in ("'", '"'):
        return parse_quoted_value(value, line_no)
    return strip_inline_comment(value), False


def parse_line(line: str, line_no: int) -> tuple[str, str, bool] | None:
    stripped = line.lstrip()
    if not stripped or stripped.startswith("#"):
        return None
    if stripped.startswith("export "):
        stripped = stripped[len("export ") :].lstrip()
    if "=" not in stripped:
        warn(f"Line {line_no}: missing '='; skipping")
        return None
    key, rest = stripped.split("=", 1)
    key = key.strip()
    if not key:
        warn(f"Line {line_no}: empty key; skipping")
        return None
    value, was_quoted = parse_value(rest, line_no)
    return key, value, was_quoted


def needs_quotes(value: str) -> bool:
    if value == "":
        return True
    if value[0].isspace() or value[-1].isspace():
        return True
    for ch in value:
        if ch.isspace() or ch in {'#', '"', "'"}:
            return True
    return False


def format_env_value(value: str, prefer_quote: bool) -> str:
    if prefer_quote or needs_quotes(value):
        escaped = (
            value.replace("\\", "\\\\")
            .replace('"', '\\"')
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
        )
        return f'"{escaped}"'
    return value


def resolve_repo_root(script_path: Path) -> Path:
    if script_path.parent.name == "scripts":
        return script_path.parent.parent
    return script_path.parent


def find_env_from_git_worktrees(start_dir: Path) -> Path | None:
    try:
        result = subprocess.run(
            ["git", "worktree", "list", "--porcelain"],
            cwd=start_dir,
            check=False,
            capture_output=True,
            text=True,
        )
    except FileNotFoundError:
        return None

    if result.returncode != 0:
        warn("Unable to query git worktrees for .env fallback")
        return None

    candidates: list[Path] = []
    for line in result.stdout.splitlines():
        if line.startswith("worktree "):
            path = Path(line.split(" ", 1)[1].strip())
            candidates.append(path / ".env")

    for candidate in candidates:
        if candidate.exists():
            return candidate
    return None


def main() -> int:
    script_path = Path(__file__).resolve()
    repo_root = resolve_repo_root(script_path)

    parser = argparse.ArgumentParser(
        description="Convert .env into a copy-friendly .env.script file."
    )
    parser.add_argument(
        "-i",
        "--input",
        default=str(repo_root / ".env"),
        help="Path to the source .env file.",
    )
    parser.add_argument(
        "-o",
        "--output",
        default=str(repo_root / ".env.script"),
        help="Path to write the output script file.",
    )
    parser.add_argument(
        "--format",
        choices=("env", "shell"),
        default="shell",
        help="Output format: shell exports or dotenv-style lines.",
    )

    args = parser.parse_args()
    input_path = Path(args.input).expanduser()
    output_path = Path(args.output)

    if not input_path.exists():
        default_input = repo_root / ".env"
        fallback: Path | None = None
        if input_path == default_input:
            env_override = os.environ.get("ENV_TO_SCRIPT_INPUT")
            if env_override:
                candidate = Path(env_override).expanduser()
                if candidate.exists():
                    fallback = candidate
            if fallback is None:
                fallback = find_env_from_git_worktrees(repo_root)

        if fallback is None:
            warn(f"Input file not found: {input_path}")
            return 1

        warn(f"Input file not found in worktree; using {fallback}")
        input_path = fallback

    lines = input_path.read_text(encoding="utf-8").splitlines()
    output_lines: list[str] = []
    for line_no, line in enumerate(lines, start=1):
        parsed = parse_line(line, line_no)
        if not parsed:
            continue
        key, value, was_quoted = parsed
        if args.format == "shell":
            output_lines.append(f"export {key}={shlex.quote(value)}")
        else:
            output_lines.append(f"{key}={format_env_value(value, was_quoted)}")

    output_path.write_text("\n".join(output_lines) + "\n", encoding="utf-8")
    print(f"Wrote {len(output_lines)} entries to {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
