#!/usr/bin/env bash
set -euo pipefail

# shellcheck shell=bash

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This script is intended to be sourced, not executed directly." >&2
  echo "Example: source DoWhiz_service/scripts/load_env_target.sh" >&2
  exit 1
fi

load_dowhiz_env_for_target() {
  local script_dir service_root repo_root env_file
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  service_root="$(cd "${script_dir}/.." && pwd)"
  repo_root="$(cd "${service_root}/.." && pwd)"

  env_file="${ENV_FILE:-}"
  if [[ -z "${env_file}" ]]; then
    if [[ -f "${service_root}/.env" ]]; then
      env_file="${service_root}/.env"
    elif [[ -f "${repo_root}/.env" ]]; then
      env_file="${repo_root}/.env"
    fi
  fi

  if [[ -n "${env_file}" ]] && [[ ! -f "${env_file}" ]]; then
    echo "Environment file not found: ${env_file}" >&2
    return 1
  fi

  if [[ -n "${env_file}" ]]; then
    set -a
    source "${env_file}"
    set +a
    export ENV_FILE="${env_file}"
  fi
}

load_dowhiz_env_for_target "$@"
