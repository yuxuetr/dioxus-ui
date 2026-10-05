#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture_root="$(mktemp -d "${TMPDIR:-/tmp}/dxui-generated-fixture.XXXXXX")"

cd "${repo_root}"

echo "fixture: ${fixture_root}"

templates_dir="crates/dioxus-shadcn-cli/templates"

# grep exits 2 on a missing directory, which `if` would read as "no match".
if [[ ! -d "${templates_dir}" ]]; then
  echo "missing templates directory: ${templates_dir}" >&2
  exit 1
fi

if grep -R "dioxus_shadcn_core\|dioxus_shadcn_primitives" "${templates_dir}"; then
  echo "templates must not import dioxus-shadcn internal crates" >&2
  exit 1
fi

cargo run -q -p dioxus-shadcn-cli -- init --root "${fixture_root}"

mapfile -t components < <(cargo run -q -p dioxus-shadcn-cli -- list)

if [[ "${#components[@]}" -eq 0 ]]; then
  echo "dxui list returned no public components" >&2
  exit 1
fi

for component in "${components[@]}"; do
  cargo run -q -p dioxus-shadcn-cli -- add "${component}" --root "${fixture_root}"
done

for component in "${components[@]}"; do
  module="${component//-/_}"
  component_file="${fixture_root}/src/components/ui/${module}.rs"

  if [[ ! -f "${component_file}" ]]; then
    echo "missing generated component file: ${component_file}" >&2
    exit 1
  fi

  if ! grep -qx "pub mod ${module};" "${fixture_root}/src/components/ui/mod.rs"; then
    echo "missing generated module declaration: ${module}" >&2
    exit 1
  fi
done

if [[ ! -f "${fixture_root}/src/components/ui/utils.rs" ]]; then
  echo "missing generated utils dependency" >&2
  exit 1
fi

if ! grep -qx "pub mod utils;" "${fixture_root}/src/components/ui/mod.rs"; then
  echo "missing generated utils module declaration" >&2
  exit 1
fi

if grep -R "dioxus_shadcn_core\|dioxus_shadcn_primitives" "${fixture_root}/src/components/ui"; then
  echo "generated components must not import dioxus-shadcn internal crates" >&2
  exit 1
fi

cat > "${fixture_root}/Cargo.toml" <<'TOML'
[package]
name = "dxui-generated-fixture"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
dioxus = "0.7"
TOML

cat > "${fixture_root}/src/components/mod.rs" <<'RS'
pub mod ui;
RS

cat > "${fixture_root}/src/lib.rs" <<'RS'
pub mod components;
RS

cargo check --manifest-path "${fixture_root}/Cargo.toml"

echo "generated fixture smoke passed"
