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

# Blocks (RFC 0073) bring their components and land in src/blocks.
mapfile -t blocks < <(cargo run -q -p dioxus-shadcn-cli -- list blocks)

if [[ "${#blocks[@]}" -eq 0 ]]; then
  echo "dxui list blocks returned no blocks" >&2
  exit 1
fi

for block in "${blocks[@]}"; do
  cargo run -q -p dioxus-shadcn-cli -- add "${block}" --root "${fixture_root}"
  module="${block//-/_}"
  if [[ ! -f "${fixture_root}/src/blocks/${module}.rs" ]]; then
    echo "missing generated block file: ${module}" >&2
    exit 1
  fi
  if ! grep -qx "pub mod ${module};" "${fixture_root}/src/blocks/mod.rs"; then
    echo "missing generated block module declaration: ${module}" >&2
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

if grep -R "dioxus_shadcn_core\|dioxus_shadcn_primitives" "${fixture_root}/src/components/ui" "${fixture_root}/src/blocks"; then
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
serde = "1"

[target.'cfg(target_arch = "wasm32")'.dependencies]
serde_json = "1"
wasm-bindgen = "0.2"
TOML


cat > "${fixture_root}/src/components/mod.rs" <<'RS'
pub mod ui;
RS

cat > "${fixture_root}/src/lib.rs" <<'RS'
#![deny(warnings)]
pub mod blocks;
pub mod components;
RS

# The fixture declares the crates the `script` helper uses (RFC 0080), so
# `dxui add` names none of them.
if cargo run -q -p dioxus-shadcn-cli -- add checkbox --root "${fixture_root}" | grep -q "need these crates"; then
  echo "dxui add named script crates the fixture declares" >&2
  exit 1
fi

# The library exports every component, so only real mistakes in the templates
# warn, such as an unused private helper or import. Drop the header that lets
# apps leave component API unused, so those fail here.
ui_mod="${fixture_root}/src/components/ui/mod.rs"
if ! grep -qx '#!\[allow(dead_code, unused_imports)\]' "${ui_mod}"; then
  echo "missing the dead code allowance in the generated ui module" >&2
  exit 1
fi
grep -vx '#!\[allow(dead_code, unused_imports)\]' "${ui_mod}" > "${ui_mod}.strict"
mv "${ui_mod}.strict" "${ui_mod}"

cargo check --manifest-path "${fixture_root}/Cargo.toml"
# The web build runs the page scripts as wasm-bindgen snippets.
cargo check --manifest-path "${fixture_root}/Cargo.toml" --target wasm32-unknown-unknown

# An app is a binary that uses a few components and leaves the rest of their
# API unused; with the generated header it still builds without warnings.
app_root="$(mktemp -d "${TMPDIR:-/tmp}/dxui-generated-app.XXXXXX")"
echo "app: ${app_root}"
cargo run -q -p dioxus-shadcn-cli -- init --root "${app_root}"
for component in button dialog popover; do
  cargo run -q -p dioxus-shadcn-cli -- add "${component}" --root "${app_root}" > /dev/null
done
# Without a manifest yet, `dxui add` names every crate the page scripts need.
if ! cargo run -q -p dioxus-shadcn-cli -- add checkbox --root "${app_root}" | grep -qx '  wasm-bindgen = "0.2"'; then
  echo "dxui add did not name the script crates the app lacks" >&2
  exit 1
fi

cat > "${app_root}/Cargo.toml" <<'TOML'
[package]
name = "dxui-generated-app"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
dioxus = { version = "0.7", features = ["web"] }
serde = "1"

[target.'cfg(target_arch = "wasm32")'.dependencies]
serde_json = "1"
wasm-bindgen = "0.2"
TOML

cat > "${app_root}/src/components/mod.rs" <<'RS'
pub mod ui;
RS

cat > "${app_root}/src/main.rs" <<'RS'
#![deny(warnings)]

use dioxus::prelude::*;

mod components;

use components::ui::button::Button;
use components::ui::dialog::{Dialog, DialogClose, DialogContent, DialogOverlay, DialogTitle};
use components::ui::popover::{Popover, PopoverContent, PopoverTitle, PopoverTrigger};

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  let mut rename = use_signal(|| false);
  rsx! {
    Button { onclick: move |_| rename.set(true), "Rename" }
    Dialog { open: rename(), on_open_change: move |next| rename.set(next),
      DialogOverlay {}
      DialogContent {
        DialogTitle { "Rename project" }
        DialogClose { "Cancel" }
      }
    }
    Popover {
      PopoverTrigger { "Share" }
      PopoverContent { PopoverTitle { "Share link" } }
    }
  }
}
RS

cargo check --manifest-path "${app_root}/Cargo.toml"

echo "generated fixture smoke passed"
