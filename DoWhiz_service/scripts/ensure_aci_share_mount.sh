#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=./load_env_target.sh
source "${script_dir}/load_env_target.sh"

backend_raw="${RUN_TASK_EXECUTION_BACKEND:-auto}"
backend="$(printf '%s' "${backend_raw}" | tr '[:upper:]' '[:lower:]')"

is_mounted() {
  local mount_path="$1"
  if command -v mountpoint >/dev/null 2>&1; then
    mountpoint -q "${mount_path}"
    return
  fi

  # Fallback for environments without mountpoint (e.g. macOS dev machines).
  mount | grep -F " on ${mount_path} " >/dev/null 2>&1
}

if [[ "${backend}" != "azure_aci" ]]; then
  echo "ensure_aci_share_mount: skip (RUN_TASK_EXECUTION_BACKEND=${backend_raw})"
  exit 0
fi

host_share_root="${RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT:-}"
if [[ -z "${host_share_root}" ]]; then
  echo "ensure_aci_share_mount: RUN_TASK_AZURE_ACI_HOST_SHARE_ROOT is required when RUN_TASK_EXECUTION_BACKEND=azure_aci" >&2
  exit 1
fi

mkdir -p "${host_share_root}"

if is_mounted "${host_share_root}"; then
  echo "ensure_aci_share_mount: already mounted at ${host_share_root}"
  exit 0
fi

if ! grep -qs "[[:space:]]${host_share_root}[[:space:]]" /etc/fstab; then
  cat >&2 <<EOF
ensure_aci_share_mount: ${host_share_root} is not mounted and has no /etc/fstab entry.
Add an Azure Files CIFS entry in /etc/fstab first, then rerun.
EOF
  exit 1
fi

if command -v sudo >/dev/null 2>&1; then
  sudo -n mount "${host_share_root}"
else
  mount "${host_share_root}"
fi

if ! is_mounted "${host_share_root}"; then
  echo "ensure_aci_share_mount: mount failed for ${host_share_root}" >&2
  exit 1
fi

echo "ensure_aci_share_mount: mounted ${host_share_root}"
