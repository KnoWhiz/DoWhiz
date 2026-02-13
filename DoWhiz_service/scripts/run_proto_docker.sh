#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
compose_file="${script_dir}/../docker-compose.fanout.yml"

if [[ ! -f "$compose_file" ]]; then
  echo "docker-compose.fanout.yml not found at ${compose_file}" >&2
  exit 1
fi

docker compose -f "$compose_file" up --build proto "$@"
