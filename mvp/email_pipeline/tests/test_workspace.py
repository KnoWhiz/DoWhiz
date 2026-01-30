from __future__ import annotations

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

    def test_prepare_workspace_from_email(self) -> None:
        msg = EmailMessage()
        msg["From"] = "sender@example.com"
        msg["To"] = "receiver@example.com"
        msg["Subject"] = "Hello"
        msg["Message-ID"] = "<test-message@example.com>"
        msg.set_content("Hello world")
        msg.add_attachment(b"data", maintype="application", subtype="octet-stream", filename="file.bin")

        result = prepare_workspace(msg, str(self.workspace_root))
        self.assertTrue(result["success"])
        workspace = Path(result["workspace_path"])
        self.assertTrue((workspace / "raw_email.eml").exists())
        self.assertTrue((workspace / "email_inbox.md").exists())
        attachments = list((workspace / "email_inbox_attachments").iterdir())
        self.assertEqual(len(attachments), 1)

    def test_create_workspace_from_files(self) -> None:
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
        self.assertTrue((workspace / "email_inbox.md").exists())
        self.assertTrue((workspace / "email_inbox_attachments" / "note.txt").exists())


if __name__ == "__main__":
    unittest.main()
