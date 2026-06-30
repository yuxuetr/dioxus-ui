# Release and Package Strategy

## Goal

`dioxus-ui` supports two distribution modes:

- source-copy mode through `dxui add`
- crate mode through `dioxus-ui` feature flags

Source-copy mode remains the preferred early release path. Crate mode should be
published only after component APIs are stable enough that users can depend on
them without needing to edit internals.

## Package Set

Planned published crates:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

Publishing order:

1. `dioxus-ui-core`
2. `dioxus-ui-primitives`
3. `dioxus-ui`
4. `dioxus-ui-cli`

## Feature Policy

The styled crate should keep `default = []`.

Users opt into components:

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["button", "dialog"] }
```

Feature names should match registry names where possible:

```text
button
input
textarea
label
checkbox
switch
tabs
accordion
dialog
popover
tooltip
select
dropdown
```

Overlay components may enable primitive features:

```toml
dialog = ["dioxus-ui-primitives/dialog"]
popover = ["dioxus-ui-primitives/popover"]
tooltip = ["dioxus-ui-primitives/tooltip"]
select = ["dioxus-ui-primitives/select"]
dropdown = ["dioxus-ui-primitives/dropdown"]
```

## Versioning Policy

Before `1.0`, API changes are allowed but should still be documented in the
changelog.

Breaking changes include:

- renaming components or props
- changing feature names
- changing generated file paths
- changing registry schema fields
- changing class helper semantics
- removing public primitive config fields

Patch releases should be limited to:

- bug fixes
- documentation corrections
- non-breaking class additions
- registry metadata fixes

## Release Gates

Before publishing any crate:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
cargo run -p dioxus-ui-cli -- list
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```

Smoke commands:

```bash
cargo run -p dioxus-ui-cli -- init --root /tmp/dxui-release-smoke
cargo run -p dioxus-ui-cli -- add button --root /tmp/dxui-release-smoke
cargo run -p dioxus-ui-cli -- add dialog --root /tmp/dxui-release-smoke
```

Manual review:

- generated `assets/dioxus-ui.css` uses Tailwind CSS v4 syntax
- generated `src/components/ui/mod.rs` is deterministic
- registry entries point to existing templates
- component features compile individually and in representative combinations
- command-line Web and Desktop examples expose representative parity states

`scripts/feature-check.sh` is more expensive than a normal workspace check
because it invokes Cargo once per public component feature. Run it before
release and after feature-gating changes.

See [Quality Gates](quality-gates.md) for local, CI, source-copy, feature, and
release verification tiers.

## CLI Release Notes

The first CLI release can support:

```text
dxui init
dxui list
dxui add <component>
```

The CLI currently reads registry and templates from the repository layout. A
publish-ready CLI should either embed templates at compile time or package them
in a stable install location.

## Known Pre-1.0 Limitations

- Overlay primitives define state/config contracts but do not implement full
  focus trap, DOM portal, or positioning engines yet.
- Generated templates include a local `utils.rs` helper module and should not
  require `dioxus-ui-core` or `dioxus-ui-primitives` in source-copy mode.
- Examples are command-line smoke examples, not full Dioxus Web/Desktop visual
  previews yet.
