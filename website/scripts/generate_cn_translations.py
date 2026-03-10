#!/usr/bin/env python3

from __future__ import annotations

import json
import re
import time
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

from bs4 import BeautifulSoup


ROOT = Path(__file__).resolve().parents[1]
OUTPUT_PATH = ROOT / "public" / "cn-translations.json"
APP_PATH = ROOT / "src" / "App.jsx"
HTML_PATHS = [ROOT / "index.html", *sorted((ROOT / "public").rglob("*.html"))]
SKIP_PARENT_TAGS = {"script", "style", "svg", "path"}
TEXT_ATTRS = ("content", "placeholder", "title", "aria-label", "alt", "value")
SHORT_ALLOWLIST = {
    "Account",
    "Active",
    "Back to DoWhiz",
    "Back to blog",
    "Back to home",
    "Best for",
    "Blog",
    "Channels",
    "Coming Soon",
    "Coming soon",
    "Contact",
    "Continue with Google",
    "Dashboard",
    "Deliver",
    "Deployment",
    "Email",
    "Execute",
    "FAQ",
    "Features",
    "Grant",
    "Help Center",
    "How it works",
    "Integrations",
    "Join waitlist",
    "Memory",
    "New password",
    "Nickname",
    "Operate",
    "Output",
    "Preferences",
    "Privacy",
    "Reset form",
    "Revoke",
    "Role",
    "Safety",
    "Scope",
    "Sign In",
    "Sign out",
    "Submitting...",
    "Tasks",
    "Team",
    "Terms",
    "Trigger",
    "Trust & Safety",
    "User Guide",
    "Watch demo videos",
    "Workflows",
}
MANUAL_OVERRIDES = {
    "Active": "已上线",
    "Blog": "博客",
    "Buy More Hours": "购买更多工时",
    "Buy Now": "立即购买",
    "CEO": "首席执行官",
    "Coder": "编码工程师",
    "Coming Soon": "即将推出",
    "Coming soon": "即将推出",
    "Contact": "联系",
    "Continue with Google": "使用 Google 继续",
    "DeepTutor": "深度导师",
    "Deployment": "部署",
    "DoWhiz": "DoWhiz",
    "DoWhiz | Multi-Channel Tool-Native Digital Employees": "DoWhiz | 多渠道工具原生数字员工",
    "DoWhiz Blog | SEO and Product Workflow Guides": "DoWhiz 博客 | SEO 与产品工作流指南",
    "DoWhiz Help Center | Top 20 User Questions": "DoWhiz 帮助中心 | 前 20 个用户问题",
    "FAQ": "常见问题",
    "Features": "功能",
    "GTM Specialist": "市场增长专家",
    "Generalist": "通才助理",
    "How it works": "工作方式",
    "OpenClaw": "OpenClaw",
    "Role Design": "角色设计",
    "Safety": "安全",
    "Sign In": "登录",
    "TBD": "待定",
    "TPM": "TPM",
    "Team": "团队",
    "Workflow Specialist": "工作流专家",
    "Workflows": "工作流",
}
TRANSLATE_URL = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=zh-CN&dt=t&q="


def normalize_text(value: str) -> str:
    return re.sub(r"\s+", " ", value).strip()


def sanitize_translation(value: str) -> str:
    return (
        value.replace("Do Whiz", "DoWhiz")
        .replace("Open Claw", "OpenClaw")
        .replace("多威兹", "DoWhiz")
        .replace("多惠兹", "DoWhiz")
        .replace("多维兹", "DoWhiz")
        .replace("多奇才", "DoWhiz")
    )


def should_keep(value: str) -> bool:
    if not value or len(value) < 2 or len(value) > 220:
        return False
    if not any(char.isalpha() for char in value):
        return False
    if any(
        token in value
        for token in (
            "http://",
            "https://",
            "mailto:",
            "/assets/",
            "/icons/",
            ".svg",
            ".png",
            ".jpg",
            ".jpeg",
            ".mov",
            "@context",
            "@supabase",
            "@type",
            "className",
            "document.",
            "querySelector",
            "requestAnimationFrame",
            "schema.org",
            "window.",
        )
    ):
        return False
    if any(char in value for char in "{};[]"):
        return False
    if value.startswith(("/", ".", "#")) or value.startswith("App: "):
        return False
    if value.startswith("(") and value.endswith(")") and len(value) < 40:
        return False
    if re.fullmatch(r"[A-Za-z0-9_.#%\-/?:&=+]+", value):
        return False
    if "@dowhiz.com" in value:
        return False
    if " " not in value and len(value) < 15 and value not in SHORT_ALLOWLIST:
        return False
    if " = " in value or " && " in value:
        return False
    return True


def collect_html_strings() -> set[str]:
    values: set[str] = set()
    for path in HTML_PATHS:
        soup = BeautifulSoup(path.read_text(), "html.parser")
        for node in soup.find_all(string=True):
            if node.parent and node.parent.name in SKIP_PARENT_TAGS:
                continue
            text = normalize_text(str(node))
            if should_keep(text):
                values.add(text)

        for tag in soup.find_all(True):
            for attr in TEXT_ATTRS:
                raw = tag.get(attr)
                if not raw:
                    continue
                text = normalize_text(raw)
                if should_keep(text):
                    values.add(text)
    return values


def collect_app_strings() -> set[str]:
    source = APP_PATH.read_text()
    values: set[str] = set()

    for match in re.finditer(r">\s*([^<>{][^<>]*?[^<>{\s])\s*<", source, re.S):
        text = normalize_text(match.group(1))
        if should_keep(text):
            values.add(text)

    for match in re.finditer(r"'((?:[^'\\]|\\.)*)'|\"((?:[^\"\\]|\\.)*)\"", source, re.S):
        raw = match.group(1) or match.group(2) or ""
        text = normalize_text(bytes(raw, "utf-8").decode("unicode_escape"))
        if should_keep(text):
            values.add(text)

    return values


def translate_text(text: str) -> tuple[str, str]:
    if text in MANUAL_OVERRIDES:
        return text, sanitize_translation(MANUAL_OVERRIDES[text])

    request = urllib.request.Request(
        TRANSLATE_URL + urllib.parse.quote(text),
        headers={
            "User-Agent": "Mozilla/5.0",
            "Accept-Language": "en-US,en;q=0.9",
        },
    )

    for attempt in range(3):
        try:
            with urllib.request.urlopen(request, timeout=20) as response:
                payload = json.loads(response.read().decode("utf-8"))
            translated = "".join(part[0] for part in payload[0] if part and part[0])
            return text, sanitize_translation(translated or text)
        except Exception:
            if attempt == 2:
                return text, text
            time.sleep(0.5 * (attempt + 1))

    return text, text


def main() -> int:
    strings = sorted(collect_html_strings() | collect_app_strings())
    translations: dict[str, str] = {}

    with ThreadPoolExecutor(max_workers=16) as pool:
        futures = [pool.submit(translate_text, text) for text in strings]
        for index, future in enumerate(as_completed(futures), start=1):
            source, translated = future.result()
            translations[source] = translated
            if index % 100 == 0:
                print(f"{index}/{len(strings)}", flush=True)

    OUTPUT_PATH.write_text(
        json.dumps(
            {
                "_generated_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                "translations": dict(sorted(translations.items())),
            },
            ensure_ascii=False,
            indent=2,
        )
        + "\n"
    )
    print(f"Wrote {len(translations)} translations to {OUTPUT_PATH}", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
