#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture_root="$(mktemp -d "${TMPDIR:-/tmp}/dxui-generated-fixture.XXXXXX")"

cd "${repo_root}"

echo "fixture: ${fixture_root}"

cargo run -q -p dioxus-ui-cli -- init --root "${fixture_root}"

while IFS= read -r component; do
  cargo run -q -p dioxus-ui-cli -- add "${component}" --root "${fixture_root}"
done < <(cargo run -q -p dioxus-ui-cli -- list)

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
