#!/usr/bin/env bash
set -euo pipefail

# shellcheck shell=bash

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This script is intended to be sourced, not executed directly." >&2
  echo "Example: source DoWhiz_service/scripts/load_env_target.sh" >&2
  exit 1
fi

load_dowhiz_env_for_target() {
  local script_dir service_root repo_root env_file target normalized_target deploy_target_override
  deploy_target_override="${DEPLOY_TARGET:-}"
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

  if [[ -n "${deploy_target_override}" ]]; then
    export DEPLOY_TARGET="${deploy_target_override}"
  fi

  target="${DEPLOY_TARGET:-production}"
  normalized_target="$(printf '%s' "${target}" | tr '[:upper:]' '[:lower:]')"
  case "${normalized_target}" in
    production|staging) ;;
    *)
      echo "Invalid DEPLOY_TARGET='${target}'. Expected 'production' or 'staging'." >&2
      return 1
      ;;
  esac
  export DEPLOY_TARGET="${normalized_target}"

  # In staging mode, STAGING_FOO overrides FOO.
  if [[ "${DEPLOY_TARGET}" == "staging" ]]; then
    local key base_key
    while IFS='=' read -r key _; do
      [[ "${key}" == STAGING_* ]] || continue
      base_key="${key#STAGING_}"
      export "${base_key}=${!key-}"
    done < <(env)

    # Keep SCALE_OLIVER_* aliases in sync with staging-resolved base keys.
    # This prevents legacy SCALE_OLIVER_* production values from shadowing
    # the staging values in modules that read SCALE_OLIVER_* first.
    local scale_aliases alias value
    scale_aliases=(
      INGESTION_QUEUE_BACKEND
      SERVICE_BUS_CONNECTION_STRING
      SERVICE_BUS_QUEUE_NAME
      SERVICE_BUS_TEST_QUEUE_NAME
      SERVICE_BUS_NAMESPACE
      SERVICE_BUS_POLICY_NAME
      SERVICE_BUS_POLICY_KEY
      RAW_PAYLOAD_STORAGE_BACKEND
      RAW_PAYLOAD_PATH_PREFIX
      AZURE_STORAGE_ACCOUNT
      AZURE_STORAGE_CONTAINER_INGEST
      AZURE_STORAGE_SAS_TOKEN
      AZURE_STORAGE_CONTAINER_SAS_URL
      AZURE_STORAGE_CONNECTION_STRING_INGEST
    )
    for alias in "${scale_aliases[@]}"; do
      value="${!alias-}"
      if [[ -n "${value}" ]]; then
        export "SCALE_OLIVER_${alias}=${value}"
      fi
    done
  fi
}

load_dowhiz_env_for_target "$@"
