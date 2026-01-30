from __future__ import annotations

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from mvp.email_pipeline.responder import generate_response


class ResponderTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.workspace = Path(self.tempdir.name)
        (self.workspace / "email_inbox.md").write_text("Hello", encoding="utf-8")
        (self.workspace / "email_inbox_attachments").mkdir()

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def test_generate_response_writes_reply(self) -> None:
        def _fake_run(prompt, workspace_dir, reply_path, model_name, codex_disabled=False):
            reply_path.write_text("Reply", encoding="utf-8")
            return "Reply"

        with mock.patch("mvp.email_pipeline.responder.run_codex_reply", side_effect=_fake_run):
            result = generate_response(str(self.workspace))

        self.assertTrue(result["success"])
        reply_path = Path(result["reply_path"])
        self.assertTrue(reply_path.exists())
        self.assertEqual(reply_path.read_text(encoding="utf-8"), "Reply")

    def test_generate_response_missing_inbox(self) -> None:
        (self.workspace / "email_inbox.md").unlink()
        result = generate_response(str(self.workspace))
        self.assertFalse(result["success"])
        self.assertIn("Missing inbox", result["error"])


if __name__ == "__main__":
    unittest.main()
