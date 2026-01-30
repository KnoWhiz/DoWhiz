from __future__ import annotations

import json
import sys
import time
import unittest
from pathlib import Path
from typing import Any


class ReportingResult(unittest.TextTestResult):
    def __init__(self, stream, descriptions, verbosity):
        super().__init__(stream, descriptions, verbosity)
        self.started_at = time.time()

    def stopTestRun(self) -> None:  # noqa: N802
        super().stopTestRun()
        self.finished_at = time.time()


def run_tests(package_name: str, test_dir: Path, report_path: Path) -> int:
    loader = unittest.defaultTestLoader
    top_level_dir = test_dir.parents[3]
    suite = loader.discover(start_dir=str(test_dir), pattern="test*.py", top_level_dir=str(top_level_dir))
    stream = _NullStream()
    runner = unittest.TextTestRunner(stream=stream, verbosity=2, resultclass=ReportingResult)
    result: ReportingResult = runner.run(suite)  # type: ignore[assignment]

    report = _build_report(result, stream.value, package_name)
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, indent=2), encoding="utf-8")

    print(f"Wrote test report to {report_path}")
    summary = report["summary"]
    print(
        "Tests run: {total}, failures: {failures}, errors: {errors}, skipped: {skipped}".format(
            total=summary["total"],
            failures=summary["failures"],
            errors=summary["errors"],
            skipped=summary["skipped"],
        )
    )

    return 0 if result.wasSuccessful() else 1


def _build_report(result: ReportingResult, output: str, package_name: str) -> dict[str, Any]:
    return {
        "package": package_name,
        "summary": {
            "total": result.testsRun,
            "failures": len(result.failures),
            "errors": len(result.errors),
            "skipped": len(result.skipped),
        },
        "failures": _format_failures(result.failures),
        "errors": _format_failures(result.errors),
        "skipped": _format_skipped(result.skipped),
        "output": output,
        "python": sys.version,
        "started_at": getattr(result, "started_at", None),
        "finished_at": getattr(result, "finished_at", None),
    }


def _format_failures(items):
    formatted = []
    for test, tb in items:
        formatted.append({"test": test.id(), "traceback": tb})
    return formatted


def _format_skipped(items):
    formatted = []
    for test, reason in items:
        formatted.append({"test": test.id(), "reason": reason})
    return formatted


class _NullStream:
    def __init__(self) -> None:
        self.value = ""

    def write(self, message: str) -> None:
        self.value += message

    def flush(self) -> None:
        return None
