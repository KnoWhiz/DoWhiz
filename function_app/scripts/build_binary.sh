#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP_DIR="$ROOT_DIR/function_app"
CRATE="scheduler_module"
BIN_NAME="rust_service"
TARGET="${TARGET:-x86_64-unknown-linux-gnu}"
PROFILE="${PROFILE:-release}"

if command -v cross >/dev/null 2>&1; then
  BUILDER="cross"
else
  BUILDER="cargo"
fi

if [[ "$BUILDER" == "cargo" ]] && command -v rustup >/dev/null 2>&1; then
  rustup target add "$TARGET" >/dev/null 2>&1 || true
fi

PROFILE_FLAGS=()
if [[ "$PROFILE" == "release" ]]; then
  PROFILE_FLAGS+=("--release")
else
  PROFILE_FLAGS+=("--profile" "$PROFILE")
fi

"$BUILDER" build -p "$CRATE" --bin "$BIN_NAME" "${PROFILE_FLAGS[@]}" --target "$TARGET"

BIN_PATH="$ROOT_DIR/DoWhiz_service/target/$TARGET/$PROFILE/$BIN_NAME"
if [[ ! -f "$BIN_PATH" ]]; then
  echo "Binary not found: $BIN_PATH" >&2
  exit 1
fi

cp "$BIN_PATH" "$APP_DIR/$BIN_NAME"
chmod +x "$APP_DIR/$BIN_NAME"

echo "Built $BIN_NAME for $TARGET ($PROFILE) -> $APP_DIR/$BIN_NAME"
