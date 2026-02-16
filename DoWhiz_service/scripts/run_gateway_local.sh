#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
service_root="$(cd "${script_dir}/.." && pwd)"

export GATEWAY_CONFIG_PATH="${GATEWAY_CONFIG_PATH:-${service_root}/gateway.toml}"
export GATEWAY_HOST="${GATEWAY_HOST:-0.0.0.0}"
export GATEWAY_PORT="${GATEWAY_PORT:-9100}"

cd "$service_root"

cargo run -p scheduler_module --bin inbound_gateway
