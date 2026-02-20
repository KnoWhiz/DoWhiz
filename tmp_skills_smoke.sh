#!/usr/bin/env bash
set -u

if [ -f /app/.env ]; then
  set -a
  # shellcheck disable=SC1091
  source /app/.env
  set +a
fi

export PATH="/app/bin:$PATH"

results=()
failures=0

run_test() {
  local name="$1"
  shift
  echo "=== ${name} ==="
  ( set -e; "$@" )
  local rc=$?
  if [ $rc -eq 0 ]; then
    echo "RESULT: PASS"
    results+=("${name}:PASS")
  else
    echo "RESULT: FAIL (exit ${rc})"
    results+=("${name}:FAIL")
    failures=1
  fi
  echo
  return 0
}

# Tests

test_browser_use() {
  command -v browser-use >/dev/null
  browser-use doctor >/dev/null
}

test_remote_browser() {
  command -v browser-use >/dev/null
  command -v cloudflared >/dev/null
}

test_playwright_cli() {
  command -v playwright-cli >/dev/null
  playwright-cli --help >/dev/null
}

test_docx() {
  pandoc --version >/dev/null
  soffice --headless --version >/dev/null
  pdftoppm -h >/dev/null 2>&1
  NODE_PATH=$(npm root -g) node -e "require('docx'); console.log('ok')" >/dev/null
}

test_pptx() {
  NODE_PATH=$(npm root -g) node -e "require('pptxgenjs'); console.log('ok')" >/dev/null
  python3 - << 'PY'
import importlib
importlib.import_module('markitdown')
importlib.import_module('pptx')
importlib.import_module('PIL')
print('ok')
PY
}

test_pdf() {
  pdftotext -v >/dev/null 2>&1
  pdftoppm -h >/dev/null 2>&1
  qpdf --version >/dev/null
  tesseract --version >/dev/null
  python3 - << 'PY'
import importlib
for mod in ['pdfplumber','pypdf','reportlab','pytesseract','pdf2image']:
    importlib.import_module(mod)
print('ok')
PY
}

test_xlsx() {
  soffice --headless --version >/dev/null
  python3 - << 'PY'
import pandas, openpyxl
print('ok')
PY
}

test_web_artifacts_builder() {
  local tmpdir
  tmpdir=$(mktemp -d)
  cd "$tmpdir"
  cat > package.json << 'PKG'
{
  "name": "skill-web-artifacts-test",
  "version": "1.0.0",
  "private": true
}
PKG
  cat > index.html << 'HTML'
<!doctype html>
<html>
  <head><meta charset="utf-8"><title>artifact test</title></head>
  <body><div id="app">ok</div></body>
</html>
HTML
  bash /app/DoWhiz_service/skills/web-artifacts-builder/scripts/bundle-artifact.sh >/dev/null
}

test_webapp_testing() {
  local tmpdir
  tmpdir=$(mktemp -d)
  cat > "$tmpdir/index.html" << 'HTML'
<!doctype html>
<html><head><title>pwtest</title></head><body>ok</body></html>
HTML
  export PW_TEST_HTML="$tmpdir/index.html"
  python3 - << 'PY'
from pathlib import Path
from os import environ
from playwright.sync_api import sync_playwright
url = Path(environ['PW_TEST_HTML']).resolve().as_uri()
with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    page = browser.new_page()
    page.goto(url)
    assert page.title() == 'pwtest'
    browser.close()
print('ok')
PY
}

test_google_docs() {
  if [ -n "${GOOGLE_DOCS_TEST_DOC_ID:-}" ]; then
    google-docs read-document "$GOOGLE_DOCS_TEST_DOC_ID" >/dev/null
  else
    google-docs list-documents >/dev/null
  fi
}

test_mcp_builder() {
  npx -y @modelcontextprotocol/inspector --help >/dev/null
}

test_doc_only() {
  local dir="$1"
  test -f "/app/DoWhiz_service/skills/${dir}/SKILL.md"
}

# Run tests
run_test "browser-use" test_browser_use
run_test "remote-browser" test_remote_browser
run_test "playwright-cli" test_playwright_cli
run_test "docx" test_docx
run_test "pptx" test_pptx
run_test "pdf" test_pdf
run_test "xlsx" test_xlsx
run_test "web-artifacts-builder" test_web_artifacts_builder
run_test "webapp-testing" test_webapp_testing
run_test "google-docs" test_google_docs
run_test "mcp-builder" test_mcp_builder

# Doc-only skills
run_test "canvas-design" test_doc_only "canvas-design"
run_test "doc-coauthoring" test_doc_only "doc-coauthoring"
run_test "frontend-design" test_doc_only "frontend-design"
run_test "scheduler_maintain" test_doc_only "scheduler_maintain"
run_test "skill-creator" test_doc_only "skill-creator"
run_test "theme-factory" test_doc_only "theme-factory"

printf "SUMMARY\n"
for r in "${results[@]}"; do
  echo "$r"
done

exit $failures
