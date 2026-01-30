from __future__ import annotations

import base64
import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from mvp.email_pipeline.sender import (
    _collect_attachments,
    _markdown_to_html,
    _prepare_postmark_payload,
    send_email,
)


class SenderTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.markdown_path = Path(self.tempdir.name) / "email.md"
        self.markdown_path.write_text("Hello **world**", encoding="utf-8")

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def test_missing_token_returns_error(self) -> None:
        with mock.patch("mvp.email_pipeline.sender.load_settings") as mocked_settings:
            mocked_settings.return_value = type("Settings", (), {"postmark_token": ""})()
            with mock.patch.dict(os.environ, {"POSTMARK_SERVER_TOKEN": ""}):
                result = send_email(
                    from_address="mini-mouse@deep-tutor.com",
                    to_addresses=["deep-tutor@deep-tutor.com"],
                    subject="Test",
                    markdown_file_path=str(self.markdown_path),
                )
        self.assertFalse(result["success"])
        self.assertIn("POSTMARK_SERVER_TOKEN", result["error"])

    def test_missing_markdown_returns_error(self) -> None:
        missing_path = Path(self.tempdir.name) / "missing.md"
        with mock.patch("mvp.email_pipeline.sender.load_settings") as mocked_settings:
            mocked_settings.return_value = type("Settings", (), {"postmark_token": "token"})()
            result = send_email(
                from_address="mini-mouse@deep-tutor.com",
                to_addresses=["deep-tutor@deep-tutor.com"],
                subject="Test",
                markdown_file_path=str(missing_path),
            )
        self.assertFalse(result["success"])
        self.assertIn("Markdown file not found", result["error"])

    def test_prepare_payload_includes_thread_headers(self) -> None:
        payload, message_id = _prepare_postmark_payload(
            from_address="mini-mouse@deep-tutor.com",
            to_addresses=["deep-tutor@deep-tutor.com"],
            subject="Subject",
            markdown_text="Body",
            attachments_dir=None,
            reply_to_message_id="<orig@example.com>",
            references="<orig@example.com>",
        )
        header_map = {h["Name"]: h["Value"] for h in payload["Headers"]}
        self.assertIn("Message-ID", header_map)
        self.assertEqual(header_map["Message-ID"], message_id)
        self.assertEqual(header_map["In-Reply-To"], "<orig@example.com>")
        self.assertEqual(header_map["References"], "<orig@example.com>")

    def test_markdown_to_html_fallback(self) -> None:
        with mock.patch("mvp.email_pipeline.sender.md", None):
            html = _markdown_to_html("Line 1\nLine 2\n\nLine 3")
        self.assertIn("<p>", html)
        self.assertIn("<br />", html)

    def test_collect_attachments_builds_payload(self) -> None:
        attachments_dir = Path(self.tempdir.name) / "attachments"
        attachments_dir.mkdir()
        file_path = attachments_dir / "note.txt"
        file_path.write_text("attachment", encoding="utf-8")

        attachments = _collect_attachments(attachments_dir)
        self.assertEqual(len(attachments), 1)
        payload = attachments[0]
        self.assertEqual(payload["Name"], "note.txt")
        self.assertEqual(
            base64.b64decode(payload["Content"]).decode("utf-8"),
            "attachment",
        )


if __name__ == "__main__":
    unittest.main()
