import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT_PATH = Path(__file__).with_name("human_approval_gate.py")
SPEC = importlib.util.spec_from_file_location("human_approval_gate", SCRIPT_PATH)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = MODULE
SPEC.loader.exec_module(MODULE)


class HumanApprovalGateTests(unittest.TestCase):
    def create_screenshot(self, directory: str, name: str = "screen.png") -> str:
        path = Path(directory) / name
        path.write_bytes(
            b"\x89PNG\r\n\x1a\n"
            b"\x00\x00\x00\rIHDR"
            b"\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00"
            b"\x90wS\xde"
            b"\x00\x00\x00\x0cIDATx\x9cc```\x00\x00\x00\x04\x00\x01"
            b"\x0b\xe7\x02\x9d"
            b"\x00\x00\x00\x00IEND\xaeB`\x82"
        )
        return str(path)

    def parse_request(self, *extra_args: str):
        parser = MODULE.build_parser()
        return parser.parse_args(["request", *extra_args])

    def test_request_requires_screenshot(self):
        args = self.parse_request("--challenge-type", "captcha")
        with self.assertRaises(MODULE.CliError):
            MODULE.build_request_state(args)

    def test_two_factor_requires_explicit_page_state(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            screenshot = self.create_screenshot(temp_dir)
            args = self.parse_request(
                "--challenge-type",
                "two_factor",
                "--scope",
                "admin",
                "--two-factor-method",
                "sms",
                "--verification-destination",
                "phone ending in 9315",
                "--screenshot",
                screenshot,
            )
            with self.assertRaises(MODULE.CliError):
                MODULE.build_request_state(args)

    def test_two_factor_body_reports_method_and_destination(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            screenshot = self.create_screenshot(temp_dir)
            args = self.parse_request(
                "--challenge-type",
                "two_factor",
                "--scope",
                "admin",
                "--page-state",
                "waiting_for_code_input",
                "--two-factor-method",
                "sms",
                "--verification-destination",
                "phone ending in 9315",
                "--account-label",
                "Oliver Google account",
                "--screenshot",
                screenshot,
            )
            state = MODULE.build_request_state(args)
            rendered = state["_rendered_email"]

            self.assertEqual(state["challenge_type"], "two_factor")
            self.assertEqual(state["page_state"], "waiting_for_code_input")
            self.assertIn("Verification method: SMS code to phone ending in 9315", rendered["text_body"])
            self.assertIn("waiting for a verification code to be typed", rendered["text_body"])
            self.assertIn("screen.png", rendered["text_body"])
            self.assertEqual(rendered["attachments"][0]["Name"], "screen.png")
            self.assertEqual(state["request_attachments"][0]["content_type"], "image/png")

    def test_password_body_reports_env_lookup(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            screenshot = self.create_screenshot(temp_dir)
            args = self.parse_request(
                "--challenge-type",
                "password",
                "--scope",
                "admin",
                "--password-env-key",
                "GOOGLE_PASSWORD",
                "--password-lookup-status",
                "Checked workspace .env for GOOGLE_PASSWORD; no value was present.",
                "--account-label",
                "Oliver Google account",
                "--screenshot",
                screenshot,
            )
            state = MODULE.build_request_state(args)
            rendered = state["_rendered_email"]

            self.assertIn("Password env key checked: GOOGLE_PASSWORD", rendered["text_body"])
            self.assertIn("Checked workspace .env for GOOGLE_PASSWORD; no value was present.", rendered["text_body"])
            self.assertIn("Password needed", state["subject"])

    def test_record_send_event_writes_attachment_details(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            screenshot = self.create_screenshot(temp_dir)
            args = self.parse_request(
                "--challenge-type",
                "captcha",
                "--scope",
                "admin",
                "--account-label",
                "Oliver Google account",
                "--screenshot",
                screenshot,
            )
            state = MODULE.build_request_state(args)
            state["outbound_message_id"] = "msg-123"
            state_dir = Path(temp_dir) / ".human_approval_gate" / "challenges"

            MODULE.record_send_event(state_dir, state)

            events_path = state_dir.parent / "events.jsonl"
            lines = events_path.read_text(encoding="utf-8").strip().splitlines()
            payload = json.loads(lines[-1])
            self.assertEqual(payload["event"], "hag_request_sent")
            self.assertEqual(payload["challenge_type"], "captcha")
            self.assertEqual(payload["attachment_count"], 1)
            self.assertEqual(payload["attachments"][0]["name"], "screen.png")
            self.assertGreater(payload["attachments"][0]["size_bytes"], 0)


if __name__ == "__main__":
    unittest.main()
