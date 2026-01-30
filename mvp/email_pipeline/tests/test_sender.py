from __future__ import annotations

import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from mvp.email_pipeline import sender


class SenderTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.markdown_path = Path(self.tempdir.name) / "email.md"
        self.markdown_path.write_text("Hello **world**", encoding="utf-8")
        self.attachments_dir = Path(self.tempdir.name) / "attachments"
        self.attachments_dir.mkdir()
        (self.attachments_dir / "note.txt").write_text("attachment", encoding="utf-8")

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def test_send_email_prepares_payload(self) -> None:
        with mock.patch.dict(os.environ, {"POSTMARK_SERVER_TOKEN": "token"}):
            with mock.patch.object(sender, "_postmark_send") as mocked_send:
                result = sender.send_email(
                    from_address="mini-mouse@deep-tutor.com",
                    to_addresses=["deep-tutor@deep-tutor.com"],
                    subject="Test",
                    markdown_file_path=str(self.markdown_path),
                    attachments_dir_path=str(self.attachments_dir),
                    reply_to_message_id="<orig@example.com>",
                    references="<orig@example.com>",
                )
        self.assertTrue(result["success"])
        self.assertTrue(result["message_id"].startswith("<"))
        mocked_send.assert_called_once()

    def test_send_email_requires_token(self) -> None:
        with mock.patch.object(sender, "load_settings") as mocked_settings:
            mocked_settings.return_value = type("Settings", (), {"postmark_token": ""})()
            with mock.patch.dict(os.environ, {"POSTMARK_SERVER_TOKEN": ""}):
                result = sender.send_email(
                    from_address="mini-mouse@deep-tutor.com",
                    to_addresses=["deep-tutor@deep-tutor.com"],
                    subject="Test",
                    markdown_file_path=str(self.markdown_path),
                )
        self.assertFalse(result["success"])
        self.assertIn("POSTMARK_SERVER_TOKEN", result["error"])


if __name__ == "__main__":
    unittest.main()
