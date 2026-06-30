#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repo_root}"

web_output="$(cargo run -q -p dioxus-ui-web-demo)"
desktop_output="$(cargo run -q -p dioxus-ui-desktop-demo)"

require_fragment() {
  local label="$1"
  local output="$2"
  local fragment="$3"

  if [[ "${output}" != *"${fragment}"* ]]; then
    echo "missing ${label} demo fragment: ${fragment}" >&2
    exit 1
  fi
}

required_fragments=(
  "button group class"
  "input group class"
  "input otp helper"
  "attachment class"
  "bubble class"
  "message class"
  "message scroller helper"
  "marker class"
  "chart helper"
  "direction class/attr"
  "collapsible class"
)

for fragment in "${required_fragments[@]}"; do
  require_fragment "web" "${web_output}" "dioxus-ui web demo ${fragment}"
  require_fragment "desktop" "${desktop_output}" "dioxus-ui desktop demo ${fragment}"
done

echo "example smoke passed"
