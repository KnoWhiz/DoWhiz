from __future__ import annotations

import json
import tempfile
import unittest
from email.message import EmailMessage
from pathlib import Path

from mvp.email_pipeline.workspace import create_workspace_from_files, prepare_workspace


class WorkspaceTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.workspace_root = Path(self.tempdir.name) / "workspaces"
        self.workspace_root.mkdir()

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def test_prepare_workspace_from_bytes(self) -> None:
        msg = EmailMessage()
        msg["From"] = "sender@example.com"
        msg["To"] = "receiver@example.com"
        msg["Subject"] = "Hello"
        msg["Message-ID"] = "<test-message@example.com>"
        msg.set_content("Hello world")
        raw_bytes = msg.as_bytes()

        result = prepare_workspace(raw_bytes, str(self.workspace_root))
        self.assertTrue(result["success"])
        workspace = Path(result["workspace_path"])
        self.assertTrue((workspace / "raw_email.eml").exists())
        self.assertTrue((workspace / "email_inbox.md").exists())

    def test_prepare_workspace_from_message(self) -> None:
        msg = EmailMessage()
        msg["From"] = "sender@example.com"
        msg["To"] = "receiver@example.com"
        msg["Subject"] = "Hello"
        msg["Message-ID"] = "<test-message@example.com>"
        msg["In-Reply-To"] = "<parent@example.com>"
        msg.set_content("Hello world")

        result = prepare_workspace(msg, str(self.workspace_root))
        self.assertTrue(result["success"])
        self.assertEqual(result["from_address"], "sender@example.com")
        self.assertEqual(result["subject"], "Hello")
        self.assertEqual(result["in_reply_to"], "<parent@example.com>")

    def test_prepare_workspace_invalid_input(self) -> None:
        result = prepare_workspace("not-bytes", str(self.workspace_root))  # type: ignore[arg-type]
        self.assertFalse(result["success"])
        self.assertIn("raw_email must be bytes", result["error"])

    def test_create_workspace_from_files_copies_attachments(self) -> None:
        inbox_path = Path(self.tempdir.name) / "inbox.md"
        inbox_path.write_text("Body", encoding="utf-8")
        attachments_dir = Path(self.tempdir.name) / "attachments"
        attachments_dir.mkdir()
        (attachments_dir / "note.txt").write_text("hi", encoding="utf-8")

        result = create_workspace_from_files(
            str(self.workspace_root),
            str(inbox_path),
            inbox_attachments_path=str(attachments_dir),
        )
        self.assertTrue(result["success"])
        workspace = Path(result["workspace_path"])
        self.assertTrue((workspace / "email_inbox_attachments" / "note.txt").exists())

    def test_create_workspace_writes_metadata(self) -> None:
        inbox_path = Path(self.tempdir.name) / "inbox.md"
        inbox_path.write_text("Body", encoding="utf-8")
        metadata = {"from": "sender@example.com", "subject": "Hi"}

        result = create_workspace_from_files(
            str(self.workspace_root),
            str(inbox_path),
            metadata=metadata,
        )
        self.assertTrue(result["success"])
        workspace = Path(result["workspace_path"])
        metadata_path = workspace / "metadata.json"
        self.assertTrue(metadata_path.exists())
        saved = json.loads(metadata_path.read_text(encoding="utf-8"))
        self.assertEqual(saved["from"], "sender@example.com")


if __name__ == "__main__":
    unittest.main()
