#!/usr/bin/env python3
"""Tests for bootstrap_web_auth.py screenshot functionality."""

import sys
import tempfile
import time
from pathlib import Path
from unittest.mock import MagicMock, patch, PropertyMock

# Add the scripts directory to path
sys.path.insert(0, str(Path(__file__).parent))

from bootstrap_web_auth import save_debug_screenshot

# All expected screenshot step names for each provider
NOTION_SCREENSHOT_STEPS = [
    "email_not_found",
    "password_step_missing",
    "password_input_not_found",
    "verification_required",
    "timeout_waiting_session",
    "google_button_not_found",
    "timeout_after_google_login",
]

GOOGLE_SCREENSHOT_STEPS = [
    "email_not_found",
    "password_step_missing",
    "password_input_not_found",
    "challenge_required",
    "timeout",
]


class TestSaveDebugScreenshot:
    """Unit tests for save_debug_screenshot function."""

    def test_screenshot_saves_to_correct_path(self):
        """Test that screenshot is saved with correct filename pattern."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            result = save_debug_screenshot(mock_page, auth_dir, "google", "password_step_missing")

            assert result is not None
            assert "google_debug_password_step_missing_" in result
            assert result.endswith(".png")
            mock_page.screenshot.assert_called_once()

            # Verify the path passed to screenshot
            call_args = mock_page.screenshot.call_args
            assert "path" in call_args.kwargs
            assert auth_dir.as_posix() in call_args.kwargs["path"]

    def test_screenshot_returns_none_on_error(self):
        """Test that screenshot returns None if page.screenshot fails."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()
            mock_page.screenshot.side_effect = Exception("Browser crashed")

            result = save_debug_screenshot(mock_page, auth_dir, "notion", "email_not_found")

            assert result is None

    def test_screenshot_filename_includes_timestamp(self):
        """Test that screenshot filename includes a timestamp."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            before = int(time.time())
            result = save_debug_screenshot(mock_page, auth_dir, "google", "timeout")
            after = int(time.time())

            assert result is not None
            # Extract timestamp from filename
            filename = Path(result).name
            # Format: google_debug_timeout_<timestamp>.png
            parts = filename.replace(".png", "").split("_")
            timestamp = int(parts[-1])
            assert before <= timestamp <= after

    def test_screenshot_different_providers(self):
        """Test screenshot works for different providers."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            google_result = save_debug_screenshot(mock_page, auth_dir, "google", "test_step")
            notion_result = save_debug_screenshot(mock_page, auth_dir, "notion", "test_step")

            assert "google_debug_" in google_result
            assert "notion_debug_" in notion_result


class TestNotionScreenshotCoverage:
    """Tests to verify all Notion error paths have screenshot coverage."""

    def test_all_notion_steps_produce_valid_filenames(self):
        """Test that all expected Notion screenshot steps produce valid filenames."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            for step in NOTION_SCREENSHOT_STEPS:
                result = save_debug_screenshot(mock_page, auth_dir, "notion", step)
                assert result is not None, f"Step '{step}' returned None"
                assert f"notion_debug_{step}_" in result, f"Step '{step}' not in filename"
                assert result.endswith(".png"), f"Step '{step}' doesn't end with .png"

    def test_timeout_waiting_session_screenshot(self):
        """Test the timeout_waiting_session screenshot is captured correctly."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            result = save_debug_screenshot(mock_page, auth_dir, "notion", "timeout_waiting_session")

            assert result is not None
            assert "notion_debug_timeout_waiting_session_" in result
            mock_page.screenshot.assert_called_once()

    def test_verification_required_screenshot(self):
        """Test the verification_required screenshot is captured correctly."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            result = save_debug_screenshot(mock_page, auth_dir, "notion", "verification_required")

            assert result is not None
            assert "notion_debug_verification_required_" in result

    def test_google_button_not_found_screenshot(self):
        """Test the google_button_not_found screenshot is captured correctly."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            result = save_debug_screenshot(mock_page, auth_dir, "notion", "google_button_not_found")

            assert result is not None
            assert "notion_debug_google_button_not_found_" in result

    def test_timeout_after_google_login_screenshot(self):
        """Test the timeout_after_google_login screenshot is captured correctly."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            result = save_debug_screenshot(mock_page, auth_dir, "notion", "timeout_after_google_login")

            assert result is not None
            assert "notion_debug_timeout_after_google_login_" in result


class TestGoogleScreenshotCoverage:
    """Tests to verify all Google error paths have screenshot coverage."""

    def test_all_google_steps_produce_valid_filenames(self):
        """Test that all expected Google screenshot steps produce valid filenames."""
        with tempfile.TemporaryDirectory() as tmpdir:
            auth_dir = Path(tmpdir)
            mock_page = MagicMock()

            for step in GOOGLE_SCREENSHOT_STEPS:
                result = save_debug_screenshot(mock_page, auth_dir, "google", step)
                assert result is not None, f"Step '{step}' returned None"
                assert f"google_debug_{step}_" in result, f"Step '{step}' not in filename"
                assert result.endswith(".png"), f"Step '{step}' doesn't end with .png"


def test_integration_with_playwright():
    """Integration test that actually creates a screenshot with Playwright.

    This test requires Playwright to be installed. Run with:
        python test_bootstrap_screenshots.py --integration
    """
    try:
        from playwright.sync_api import sync_playwright
    except ImportError:
        print("SKIP: Playwright not installed")
        return False

    with tempfile.TemporaryDirectory() as tmpdir:
        auth_dir = Path(tmpdir)

        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True)
            context = browser.new_context()
            page = context.new_page()

            # Navigate to a simple page
            page.goto("https://example.com")
            page.wait_for_timeout(500)

            # Take a screenshot using our function
            result = save_debug_screenshot(page, auth_dir, "test", "example_page")

            browser.close()

        if result is None:
            print("FAIL: Screenshot returned None")
            return False

        screenshot_path = Path(result)
        if not screenshot_path.exists():
            print(f"FAIL: Screenshot file does not exist: {result}")
            return False

        file_size = screenshot_path.stat().st_size
        if file_size < 1000:
            print(f"FAIL: Screenshot file too small ({file_size} bytes)")
            return False

        print(f"PASS: Screenshot saved to {result} ({file_size} bytes)")
        return True


def run_unit_tests():
    """Run unit tests without pytest."""
    # Basic screenshot tests
    basic_tests = TestSaveDebugScreenshot()
    # Notion coverage tests
    notion_tests = TestNotionScreenshotCoverage()
    # Google coverage tests
    google_tests = TestGoogleScreenshotCoverage()

    tests = [
        # Basic tests
        ("test_screenshot_saves_to_correct_path", basic_tests.test_screenshot_saves_to_correct_path),
        ("test_screenshot_returns_none_on_error", basic_tests.test_screenshot_returns_none_on_error),
        ("test_screenshot_filename_includes_timestamp", basic_tests.test_screenshot_filename_includes_timestamp),
        ("test_screenshot_different_providers", basic_tests.test_screenshot_different_providers),
        # Notion coverage tests
        ("test_all_notion_steps_produce_valid_filenames", notion_tests.test_all_notion_steps_produce_valid_filenames),
        ("test_timeout_waiting_session_screenshot", notion_tests.test_timeout_waiting_session_screenshot),
        ("test_verification_required_screenshot", notion_tests.test_verification_required_screenshot),
        ("test_google_button_not_found_screenshot", notion_tests.test_google_button_not_found_screenshot),
        ("test_timeout_after_google_login_screenshot", notion_tests.test_timeout_after_google_login_screenshot),
        # Google coverage tests
        ("test_all_google_steps_produce_valid_filenames", google_tests.test_all_google_steps_produce_valid_filenames),
    ]

    passed = 0
    failed = 0

    for name, test_fn in tests:
        try:
            test_fn()
            print(f"PASS: {name}")
            passed += 1
        except AssertionError as e:
            print(f"FAIL: {name} - {e}")
            failed += 1
        except Exception as e:
            print(f"ERROR: {name} - {type(e).__name__}: {e}")
            failed += 1

    print(f"\nResults: {passed} passed, {failed} failed")
    return failed == 0


if __name__ == "__main__":
    print("=" * 60)
    print("Running unit tests for save_debug_screenshot")
    print("=" * 60)

    unit_ok = run_unit_tests()

    if "--integration" in sys.argv:
        print("\n" + "=" * 60)
        print("Running integration test with Playwright")
        print("=" * 60)
        integration_ok = test_integration_with_playwright()
    else:
        print("\nSkipping integration test (run with --integration to include)")
        integration_ok = True

    sys.exit(0 if (unit_ok and integration_ok) else 1)
