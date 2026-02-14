#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
service_root="$(cd "${script_dir}/.." && pwd)"

if [[ -z "${FANOUT_TARGETS:-}" ]]; then
  export FANOUT_TARGETS="http://127.0.0.1:9001,http://127.0.0.1:9002,http://127.0.0.1:9003,http://127.0.0.1:9004"
fi
export FANOUT_HOST="${FANOUT_HOST:-0.0.0.0}"
export FANOUT_PORT="${FANOUT_PORT:-9100}"

cd "$service_root"

cargo run -p scheduler_module --bin inbound_fanout
