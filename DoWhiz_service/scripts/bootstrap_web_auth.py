#!/usr/bin/env python3
"""Best-effort browser auth bootstrap for Notion and Google.

The script reads credentials from process env and workspace `.env`, attempts to
sign in with Playwright, and writes storage-state files under:

  <workspace>/.auth/notion_state.json
  <workspace>/.auth/google_state.json

It never prints credentials and always emits a status file:

  <workspace>/.auth/bootstrap_status.json
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import time
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Sequence, Tuple

try:
    from playwright.sync_api import TimeoutError as PlaywrightTimeoutError
    from playwright.sync_api import sync_playwright
except Exception as exc:  # pragma: no cover - environment-dependent
    sync_playwright = None  # type: ignore[assignment]
    PlaywrightTimeoutError = Exception  # type: ignore[assignment]
    PLAYWRIGHT_IMPORT_ERROR: Optional[str] = str(exc)
else:
    PLAYWRIGHT_IMPORT_ERROR = None

EMPLOYEE_PREFIX_MAP = {
    "little_bear": "OLIVER",
    "mini_mouse": "MAGGIE",
    "sticky_octopus": "DEVIN",
    "boiled_egg": "PROTO",
}

NOTION_EMAIL_KEYS = ("NOTION_ACCOUNT_EMAIL", "NOTION_EMAIL")
NOTION_PASSWORD_KEYS = ("NOTION_PASSWORD",)

GOOGLE_EMAIL_KEYS = ("GOOGLE_ACCOUNT_EMAIL", "GOOGLE_EMAIL", "GOOGLE_EMPLOYEE_EMAIL")
GOOGLE_PASSWORD_KEYS = (
    "GOOGLE_PASSWORD",
    "GOOGLE_ACCOUNT_PASSWORD",
    "GOOGLE_EMPLOYEE_PASSWORD",
)


def now_iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Bootstrap web auth states")
    parser.add_argument("--workspace", required=True, help="Workspace directory")
    parser.add_argument(
        "--timeout-secs",
        type=int,
        default=90,
        help="Total timeout budget in seconds (default: 90)",
    )
    return parser.parse_args()


def load_env_file(path: Path) -> Dict[str, str]:
    if not path.exists():
        return {}
    result: Dict[str, str] = {}
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError:
        return {}
    for raw in lines:
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip()
        if not key:
            continue
        if len(value) >= 2 and (
            (value[0] == '"' and value[-1] == '"')
            or (value[0] == "'" and value[-1] == "'")
        ):
            value = value[1:-1]
        if key not in result:
            result[key] = value
    return result


def read_env_value(key: str, dotenv: Dict[str, str]) -> Optional[str]:
    value = os.getenv(key)
    if value is not None and value.strip():
        return value.strip()
    value = dotenv.get(key)
    if value is not None and value.strip():
        return value.strip()
    return None


def normalize_prefix(value: str) -> str:
    normalized = []
    for ch in value.strip():
        if ch.isalnum():
            normalized.append(ch.upper())
        else:
            normalized.append("_")
    return "".join(normalized)


def dedupe(values: Iterable[str]) -> List[str]:
    out: List[str] = []
    seen = set()
    for value in values:
        if not value:
            continue
        if value in seen:
            continue
        out.append(value)
        seen.add(value)
    return out


def resolve_prefixes(dotenv: Dict[str, str]) -> List[str]:
    candidates: List[str] = []
    for key in (
        "WEB_AUTH_ENV_PREFIX",
        "EMPLOYEE_WEB_AUTH_ENV_PREFIX",
        "EMPLOYEE_GITHUB_ENV_PREFIX",
        "GITHUB_ENV_PREFIX",
    ):
        value = read_env_value(key, dotenv)
        if value:
            candidates.append(normalize_prefix(value))
    employee_id = read_env_value("EMPLOYEE_ID", dotenv)
    if employee_id:
        lower = employee_id.strip().lower()
        mapped = EMPLOYEE_PREFIX_MAP.get(lower)
        if mapped:
            candidates.append(mapped)
        candidates.append(normalize_prefix(employee_id))
    return dedupe(candidates)


def resolve_credential(
    keys: Sequence[str], prefixes: Sequence[str], dotenv: Dict[str, str]
) -> Optional[str]:
    for key in keys:
        value = read_env_value(key, dotenv)
        if value:
            return value
    for prefix in prefixes:
        for key in keys:
            value = read_env_value(f"{prefix}_{key}", dotenv)
            if value:
                return value
    return None


def fingerprint(*parts: str) -> str:
    payload = "\0".join(parts).encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def load_json(path: Path) -> Dict[str, object]:
    if not path.exists():
        return {}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}
    return data if isinstance(data, dict) else {}


def write_json(path: Path, data: Dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=True, indent=2), encoding="utf-8")


def has_cookie(context, url: str, names: Sequence[str]) -> bool:
    try:
        cookies = context.cookies([url])
    except Exception:
        return False
    wanted = set(names)
    for cookie in cookies:
        name = cookie.get("name")
        value = cookie.get("value")
        if name in wanted and value:
            return True
    return False


def first_text_snippet(text: str, max_len: int = 200) -> str:
    cleaned = re.sub(r"\s+", " ", text or "").strip()
    return cleaned[:max_len]


def save_debug_screenshot(page, auth_dir: Path, provider: str, step: str) -> Optional[str]:
    """Save a debug screenshot and return the path if successful."""
    try:
        filename = f"{provider}_debug_{step}_{int(time.time())}.png"
        screenshot_path = auth_dir / filename
        page.screenshot(path=str(screenshot_path))
        return str(screenshot_path)
    except Exception:
        return None


def safe_page_wait(page, timeout_ms: int) -> bool:
    try:
        page.wait_for_timeout(timeout_ms)
        return True
    except Exception:
        return False


def fill_first(page, selectors: Sequence[str], value: str, timeout_ms: int) -> bool:
    deadline = time.time() + (timeout_ms / 1000.0)
    while time.time() < deadline:
        for selector in selectors:
            try:
                handle = page.query_selector(selector)
                if handle:
                    handle.fill(value, timeout=min(3000, timeout_ms))
                    return True
            except Exception:
                continue
        if not safe_page_wait(page, 150):
            return False
    return False


def click_first(page, selectors: Sequence[str], timeout_ms: int) -> bool:
    deadline = time.time() + (timeout_ms / 1000.0)
    while time.time() < deadline:
        for selector in selectors:
            try:
                handle = page.query_selector(selector)
                if handle:
                    handle.click(timeout=min(3000, timeout_ms))
                    return True
            except Exception:
                continue
        if not safe_page_wait(page, 150):
            return False
    return False


def wait_for_any_selector(page, selectors: Sequence[str], timeout_ms: int) -> bool:
    end_time = time.time() + (timeout_ms / 1000.0)
    while time.time() < end_time:
        for selector in selectors:
            try:
                if page.query_selector(selector):
                    return True
            except Exception:
                continue
        if not safe_page_wait(page, 150):
            return False
    return False


def notion_session_ready(page, context) -> bool:
    return has_cookie(
        context,
        "https://www.notion.so",
        ("token_v2", "notion_user_id", "notion_users"),
    )


def notion_password_login(
    page, context, email: str, password: str, timeout_ms: int, auth_dir: Optional[Path] = None
) -> Tuple[bool, str]:
    page.goto(
        "https://www.notion.so/login",
        wait_until="domcontentloaded",
        timeout=timeout_ms,
    )
    page.wait_for_timeout(400)
    if notion_session_ready(page, context):
        return True, "already authenticated"

    email_selectors = (
        "#notion-email-input-1",
        "input[id^='notion-email-input']",
        "input[type='email']",
        "input[name='email']",
        "input[placeholder*='mail']",
    )

    if not fill_first(page, email_selectors, email, 8000):
        click_first(
            page,
            (
                "button:has-text('Continue with email')",
                "button:has-text('Email')",
                "button:has-text('Sign in')",
                "a:has-text('Sign in')",
            ),
            4000,
        )
        if not fill_first(page, email_selectors, email, 8000):
            if auth_dir:
                save_debug_screenshot(page, auth_dir, "notion", "email_not_found")
            return False, "email input not found"

    click_first(
        page,
        (
            "button:has-text('Continue with email')",
            "button:has-text('Continue')",
            "button[type='submit']",
        ),
        3000,
    )
    page.wait_for_timeout(800)

    if not wait_for_any_selector(page, ("input[type='password']",), 6000):
        click_first(
            page,
            (
                "button:has-text('Continue with password')",
                "button:has-text('Use password')",
            ),
            3000,
        )
        if not wait_for_any_selector(page, ("input[type='password']",), 6000):
            if notion_session_ready(page, context):
                return True, "authenticated without password step"
            if auth_dir:
                save_debug_screenshot(page, auth_dir, "notion", "password_step_missing")
            return False, "password step not available"

    if not fill_first(page, ("input[type='password']",), password, 3000):
        if auth_dir:
            save_debug_screenshot(page, auth_dir, "notion", "password_input_not_found")
        return False, "password input not found"

    click_first(
        page,
        (
            "button:has-text('Sign in')",
            "button:has-text('Log in')",
            "button:has-text('Continue')",
            "button[type='submit']",
        ),
        3000,
    )

    deadline = time.time() + (timeout_ms / 1000.0)
    while time.time() < deadline:
        url = (page.url or "").lower()
        if notion_session_ready(page, context):
            return True, "signed in"
        if any(token in url for token in ("verify", "challenge", "mfa", "otp")):
            return False, "additional verification required"
        page.wait_for_timeout(350)

    body = first_text_snippet(page.inner_text("body"))
    if body:
        return False, f"timeout waiting for authenticated session ({body})"
    return False, "timeout waiting for authenticated session"


def notion_google_login(
    page, context, google_email: str, google_password: str, timeout_ms: int, auth_dir: Optional[Path] = None
) -> Tuple[bool, str]:
    started = time.time()
    preauth_timeout_ms = max(10000, int(timeout_ms * 0.55))
    preauth_ok, preauth_message = google_login(
        page,
        context,
        google_email,
        google_password,
        preauth_timeout_ms,
        auth_dir=auth_dir,
    )
    if not preauth_ok:
        return False, f"google pre-auth failed ({preauth_message})"

    elapsed_ms = int((time.time() - started) * 1000)
    handshake_timeout_ms = max(10000, timeout_ms - elapsed_ms)

    page.goto(
        "https://www.notion.so/login",
        wait_until="domcontentloaded",
        timeout=handshake_timeout_ms,
    )
    page.wait_for_timeout(400)
    if notion_session_ready(page, context):
        return True, "already authenticated"

    if not click_google_button(page, 10000):
        return False, "google sign-in button not found"

    deadline = time.time() + (handshake_timeout_ms / 1000.0)
    account_selectors = (
        f"[data-identifier='{google_email}']",
        f"div[data-email='{google_email}']",
        f"button:has-text('{google_email}')",
        f"div:has-text('{google_email}')",
    )
    while time.time() < deadline:
        if notion_session_ready(page, context):
            return True, "signed in via google"

        for candidate in context.pages:
            try:
                if candidate.is_closed():
                    continue
                url = (candidate.url or "").lower()
            except Exception:
                continue
            if "accounts.google.com" in url:
                click_first(candidate, account_selectors, 1200)
                click_first(
                    candidate,
                    (
                        "button:has-text('Continue')",
                        "button:has-text('Next')",
                        "button:has-text('Allow')",
                    ),
                    1200,
                )
        if not safe_page_wait(page, 200):
            break

    if notion_session_ready(page, context):
        return True, "signed in via google"

    body = ""
    try:
        body = first_text_snippet(page.inner_text("body"))
    except Exception:
        body = ""
    if body:
        return False, f"timeout waiting for notion session after google login ({body})"
    return False, "timeout waiting for notion session after google login"


def click_google_button(page, timeout_ms: int) -> bool:
    if click_first(
        page,
        (
            "button:has-text('Continue with Google')",
            "button:has-text('Google')",
            "a:has-text('Continue with Google')",
            "a:has-text('Google')",
            "button[aria-label='Google']",
            "[role='button']:has-text('Google')",
        ),
        min(timeout_ms, 5000),
    ):
        return True

    deadline = time.time() + (timeout_ms / 1000.0)
    while time.time() < deadline:
        for label in ("Continue with Google", "Google"):
            try:
                button = page.get_by_role("button", name=re.compile(label, re.IGNORECASE)).first
                button.click(timeout=min(3000, timeout_ms))
                return True
            except Exception:
                continue
        if not safe_page_wait(page, 150):
            return False
    return False


def notion_login(
    page,
    context,
    email: str,
    password: str,
    timeout_ms: int,
    google_email: Optional[str] = None,
    google_password: Optional[str] = None,
    auth_dir: Optional[Path] = None,
) -> Tuple[bool, str]:
    started = time.time()
    password_timeout_ms = max(10000, int(timeout_ms * 0.6))
    ok, message = notion_password_login(page, context, email, password, password_timeout_ms, auth_dir=auth_dir)
    if ok:
        return ok, message

    if not google_email or not google_password:
        return False, message

    elapsed_ms = int((time.time() - started) * 1000)
    fallback_timeout_ms = max(10000, timeout_ms - elapsed_ms)
    fallback_ok, fallback_message = notion_google_login(
        page,
        context,
        google_email,
        google_password,
        fallback_timeout_ms,
        auth_dir=auth_dir,
    )
    if fallback_ok:
        return True, f"password login failed ({message}); google fallback succeeded ({fallback_message})"
    return False, f"password login failed ({message}); google fallback failed ({fallback_message})"


def google_login(page, context, email: str, password: str, timeout_ms: int, auth_dir: Optional[Path] = None) -> Tuple[bool, str]:
    page.goto(
        "https://accounts.google.com/signin/v2/identifier",
        wait_until="domcontentloaded",
        timeout=timeout_ms,
    )
    page.wait_for_timeout(400)
    if has_cookie(
        context,
        "https://accounts.google.com",
        ("SID", "HSID", "SSID", "SAPISID", "__Secure-1PSID"),
    ):
        return True, "already authenticated"

    if not fill_first(page, ("input[type='email']", "#identifierId"), email, 3000):
        if auth_dir:
            save_debug_screenshot(page, auth_dir, "google", "email_not_found")
        return False, "email input not found"
    click_first(page, ("#identifierNext button", "button:has-text('Next')"), 3000)

    if not wait_for_any_selector(page, ("input[type='password']",), 8000):
        if auth_dir:
            save_debug_screenshot(page, auth_dir, "google", "password_step_missing")
        body = first_text_snippet(page.inner_text("body"))
        if body:
            return False, f"password step not available ({body})"
        return False, "password step not available"

    if not fill_first(page, ("input[type='password']",), password, 3000):
        if auth_dir:
            save_debug_screenshot(page, auth_dir, "google", "password_input_not_found")
        return False, "password input not found"
    click_first(page, ("#passwordNext button", "button:has-text('Next')"), 3000)

    deadline = time.time() + (timeout_ms / 1000.0)
    while time.time() < deadline:
        url = (page.url or "").lower()
        if has_cookie(
            context,
            "https://accounts.google.com",
            ("SID", "HSID", "SSID", "SAPISID", "__Secure-1PSID"),
        ):
            return True, "signed in"
        if "challenge" in url or "interstitial" in url:
            if auth_dir:
                save_debug_screenshot(page, auth_dir, "google", "challenge_required")
            return False, "additional verification required"
        if "myaccount.google.com" in url:
            return True, "signed in (myaccount)"
        page.wait_for_timeout(350)

    if auth_dir:
        save_debug_screenshot(page, auth_dir, "google", "timeout")
    body = first_text_snippet(page.inner_text("body"))
    if body:
        return False, f"timeout waiting for authenticated session ({body})"
    return False, "timeout waiting for authenticated session"


def attempt_login(
    provider: str,
    email: str,
    password: str,
    state_path: Path,
    timeout_secs: int,
    fallback_email: Optional[str] = None,
    fallback_password: Optional[str] = None,
    auth_dir: Optional[Path] = None,
) -> Tuple[bool, str]:
    if sync_playwright is None:
        message = PLAYWRIGHT_IMPORT_ERROR or "playwright is unavailable"
        return False, message

    timeout_ms = max(10000, timeout_secs * 1000)
    try:
        with sync_playwright() as playwright:
            browser = playwright.chromium.launch(
                headless=True,
                args=["--disable-dev-shm-usage", "--no-sandbox"],
            )
            context = browser.new_context()
            page = context.new_page()
            if provider == "notion":
                ok, message = notion_login(
                    page,
                    context,
                    email,
                    password,
                    timeout_ms,
                    google_email=fallback_email,
                    google_password=fallback_password,
                    auth_dir=auth_dir,
                )
            elif provider == "google":
                ok, message = google_login(page, context, email, password, timeout_ms, auth_dir=auth_dir)
            else:
                browser.close()
                return False, f"unsupported provider: {provider}"

            if ok:
                state_path.parent.mkdir(parents=True, exist_ok=True)
                context.storage_state(path=str(state_path))
            browser.close()
            return ok, message
    except PlaywrightTimeoutError:
        return False, "playwright timeout"
    except Exception as exc:  # pragma: no cover - external runtime failures
        detail = first_text_snippet(str(exc), max_len=500)
        if detail:
            return False, f"playwright error: {type(exc).__name__}: {detail}"
        return False, f"playwright error: {type(exc).__name__}"


def bootstrap_provider(
    provider: str,
    email: Optional[str],
    password: Optional[str],
    auth_dir: Path,
    timeout_secs: int,
    fallback_email: Optional[str] = None,
    fallback_password: Optional[str] = None,
    extra_fingerprint_parts: Optional[Sequence[str]] = None,
) -> Dict[str, object]:
    result: Dict[str, object] = {
        "provider": provider,
        "attempted": False,
        "success": False,
        "cached": False,
        "state_file": str(auth_dir.joinpath(f"{provider}_state.json")),
        "message": "",
        "updated_at": now_iso(),
    }
    if not email or not password:
        result["message"] = "missing credentials"
        return result

    state_path = auth_dir.joinpath(f"{provider}_state.json")
    meta_path = auth_dir.joinpath(f"{provider}_state.meta.json")
    fp_parts = [email, password]
    if extra_fingerprint_parts:
        fp_parts.extend(extra_fingerprint_parts)
    current_fp = fingerprint(*fp_parts)
    previous_meta = load_json(meta_path)
    previous_fp = previous_meta.get("fingerprint")

    if previous_fp == current_fp and state_path.exists():
        result["success"] = True
        result["cached"] = True
        result["message"] = "reused cached storage state"
        return result

    if state_path.exists():
        try:
            state_path.unlink()
        except OSError:
            pass

    result["attempted"] = True
    ok, message = attempt_login(
        provider,
        email,
        password,
        state_path,
        timeout_secs,
        fallback_email=fallback_email,
        fallback_password=fallback_password,
        auth_dir=auth_dir,
    )
    result["success"] = ok
    result["message"] = message
    if ok:
        write_json(
            meta_path,
            {
                "provider": provider,
                "fingerprint": current_fp,
                "updated_at": now_iso(),
            },
        )
    return result


def main() -> int:
    args = parse_args()
    workspace = Path(args.workspace).resolve()
    auth_dir = workspace.joinpath(".auth")
    auth_dir.mkdir(parents=True, exist_ok=True)
    dotenv = load_env_file(workspace.joinpath(".env"))
    prefixes = resolve_prefixes(dotenv)

    notion_email = resolve_credential(NOTION_EMAIL_KEYS, prefixes, dotenv)
    notion_password = resolve_credential(NOTION_PASSWORD_KEYS, prefixes, dotenv)
    google_email = resolve_credential(GOOGLE_EMAIL_KEYS, prefixes, dotenv)
    google_password = resolve_credential(GOOGLE_PASSWORD_KEYS, prefixes, dotenv)

    requested = []
    if notion_email and notion_password:
        requested.append("notion")
    if google_email and google_password:
        requested.append("google")
    per_provider_timeout = max(
        20,
        args.timeout_secs // max(1, len(requested) if requested else 1),
    )

    started_at = time.time()
    providers: Dict[str, Dict[str, object]] = {}
    providers["notion"] = bootstrap_provider(
        "notion",
        notion_email,
        notion_password,
        auth_dir,
        per_provider_timeout,
        fallback_email=google_email,
        fallback_password=google_password,
        extra_fingerprint_parts=(
            [google_email, google_password] if google_email and google_password else []
        ),
    )
    providers["google"] = bootstrap_provider(
        "google",
        google_email,
        google_password,
        auth_dir,
        per_provider_timeout,
    )

    summary = {
        "generated_at": now_iso(),
        "workspace": str(workspace),
        "duration_ms": int((time.time() - started_at) * 1000),
        "playwright_available": PLAYWRIGHT_IMPORT_ERROR is None,
        "prefixes_checked": prefixes,
        "providers": providers,
    }
    status_path = auth_dir.joinpath("bootstrap_status.json")
    write_json(status_path, summary)

    success_count = sum(
        1
        for item in providers.values()
        if item.get("success") is True and item.get("message") != "missing credentials"
    )
    attempted_count = sum(1 for item in providers.values() if item.get("attempted") is True)
    print(
        json.dumps(
            {
                "status_file": str(status_path),
                "attempted": attempted_count,
                "successful": success_count,
            },
            ensure_ascii=True,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
