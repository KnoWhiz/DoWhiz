#!/usr/bin/env bash
set -euo pipefail

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for web auth bootstrap dependencies" >&2
  exit 1
fi

set +e
python3 - <<'PY'
import importlib.util
import os
import sys

if importlib.util.find_spec("playwright") is None:
    sys.exit(1)

try:
    from playwright.sync_api import sync_playwright
except Exception:
    sys.exit(3)

try:
    with sync_playwright() as playwright:
        chromium_path = playwright.chromium.executable_path
except Exception:
    sys.exit(2)

if not chromium_path or not os.path.exists(chromium_path):
    sys.exit(2)

sys.exit(0)
PY
check_status=$?
set -e

install_pkg=0
install_browser=0

case "$check_status" in
  0)
    echo "Playwright Python package and Chromium are already available."
    ;;
  1)
    echo "Playwright Python package is missing; installing."
    install_pkg=1
    install_browser=1
    ;;
  2)
    echo "Playwright Chromium browser is missing; installing."
    install_browser=1
    ;;
  3)
    echo "Playwright Python package is present but unusable; reinstalling."
    install_pkg=1
    install_browser=1
    ;;
  *)
    echo "Unexpected Playwright check status (${check_status}); reinstalling."
    install_pkg=1
    install_browser=1
    ;;
esac

run_with_pep668_retry() {
  local log_file
  log_file="$(mktemp)"
  if "$@" >"$log_file" 2>&1; then
    cat "$log_file"
    rm -f "$log_file"
    return 0
  fi

  local status=$?
  if grep -qi "externally-managed-environment" "$log_file"; then
    echo "Detected externally-managed-environment; retrying with --break-system-packages."
    cat "$log_file" >&2
    rm -f "$log_file"
    "$@" --break-system-packages
    return $?
  fi

  cat "$log_file" >&2
  rm -f "$log_file"
  return $status
}

ensure_pip() {
  if python3 -m pip --version >/dev/null 2>&1; then
    return 0
  fi

  if ! command -v curl >/dev/null 2>&1; then
    echo "curl is required to bootstrap pip for Playwright installation." >&2
    return 1
  fi

  echo "pip is missing; bootstrapping with get-pip.py."
  tmp_dir="$(mktemp -d)"
  curl -fsSL https://bootstrap.pypa.io/get-pip.py -o "$tmp_dir/get-pip.py"
  run_with_pep668_retry python3 "$tmp_dir/get-pip.py" --user
  rm -rf "$tmp_dir"

  python3 -m pip --version >/dev/null 2>&1
}

if [[ "$install_pkg" -eq 1 ]]; then
  ensure_pip
  run_with_pep668_retry python3 -m pip install --user --upgrade playwright
fi

if [[ "$install_browser" -eq 1 ]]; then
  python3 -m playwright install chromium
fi
