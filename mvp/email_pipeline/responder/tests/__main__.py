from __future__ import annotations

from pathlib import Path

from ...testing.reporting import run_tests


def main() -> int:
    test_dir = Path(__file__).resolve().parent
    report_path = test_dir / "report.json"
    return run_tests("mvp.email_pipeline.responder.tests", test_dir, report_path)


if __name__ == "__main__":
    raise SystemExit(main())
