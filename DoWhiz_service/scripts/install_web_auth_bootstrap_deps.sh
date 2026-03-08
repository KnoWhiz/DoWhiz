#!/usr/bin/env bash
set -euo pipefail

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for web auth bootstrap dependencies" >&2
  exit 1
fi

check_playwright_health() {
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
    sys.exit(3)

if not chromium_path or not os.path.exists(chromium_path):
    sys.exit(2)

try:
    with sync_playwright() as playwright:
        browser = playwright.chromium.launch(
            headless=True,
            args=["--disable-dev-shm-usage", "--no-sandbox"],
        )
        page = browser.new_page()
        page.goto("about:blank", wait_until="domcontentloaded", timeout=10000)
        browser.close()
except Exception as exc:
    message = str(exc).lower()
    if (
        "error while loading shared libraries" in message
        or "host system is missing dependencies" in message
        or "please install them with the following command" in message
        or "apt-get install" in message
    ):
        sys.exit(4)
    sys.exit(3)

sys.exit(0)
PY
  local status=$?
  set -e
  return "$status"
}

if check_playwright_health; then
  check_status=0
else
  check_status=$?
fi

install_pkg=0
install_browser=0
install_system_deps=0

case "$check_status" in
  0)
    echo "Playwright Python package, Chromium, and launch runtime are available."
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
  4)
    echo "Playwright runtime dependencies are missing; installing system packages."
    install_system_deps=1
    ;;
  *)
    echo "Unexpected Playwright check status (${check_status}); reinstalling and repairing dependencies."
    install_pkg=1
    install_browser=1
    install_system_deps=1
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

run_playwright_install_deps() {
  if python3 -m playwright install-deps chromium; then
    return 0
  fi

  if command -v sudo >/dev/null 2>&1 && sudo -n true >/dev/null 2>&1; then
    local user_site
    user_site="$(
      python3 - <<'PY'
import site
print(site.getusersitepackages())
PY
    )"
    sudo -n env "PYTHONPATH=${user_site}${PYTHONPATH:+:$PYTHONPATH}" \
      python3 -m playwright install-deps chromium
    return $?
  fi

  echo "Unable to install Playwright system dependencies automatically (requires sudo access)." >&2
  return 1
}

if [[ "$install_pkg" -eq 1 ]]; then
  ensure_pip
  run_with_pep668_retry python3 -m pip install --user --upgrade playwright
fi

if [[ "$install_system_deps" -eq 1 ]]; then
  run_playwright_install_deps
fi

if [[ "$install_browser" -eq 1 ]]; then
  python3 -m playwright install chromium
fi

if check_playwright_health; then
  final_status=0
else
  final_status=$?
fi
if [[ "$final_status" -ne 0 ]]; then
  echo "Playwright bootstrap dependencies are still not healthy after install (status=${final_status})." >&2
  exit 1
fi

echo "Playwright bootstrap dependencies validated successfully."
