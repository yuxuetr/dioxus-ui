# Workspace Specification

## Goal

The repository should become a Cargo workspace that supports two distribution
modes:

- source-copy mode through `dxui add`
- crate mode through feature-gated `dioxus-ui` exports

Source-copy mode is the first implementation target. Crate mode must still be
designed early so the module boundaries do not need to be rewritten later.

## Planned Layout

```text
dioxus-ui/
├─ Cargo.toml
├─ crates/
│  ├─ dioxus-ui-core/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ dioxus-ui-primitives/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ dioxus-ui/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  └─ dioxus-ui-cli/
│     ├─ Cargo.toml
│     ├─ registry/
│     ├─ templates/
│     └─ src/main.rs
├─ examples/
└─ docs/
```

The root `Cargo.toml` should become a virtual workspace manifest after M1.1.
The current root package should move to `crates/dioxus-ui`.

## Workspace Manifest

Initial root manifest shape:

```toml
[workspace]
resolver = "2"
members = [
  "crates/dioxus-ui-core",
  "crates/dioxus-ui-primitives",
  "crates/dioxus-ui",
  "crates/dioxus-ui-cli",
  "examples/web-demo",
  "examples/desktop-demo",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/yuxuetr/dioxus-ui"
readme = "README.md"
keywords = ["dioxus", "ui", "tailwind", "components"]
categories = ["gui", "web-programming"]

[workspace.dependencies]
dioxus = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

The repository URL has been approved for first publish preparation.
The shared README, keywords, and categories are publish metadata only; they do
not imply the crates are ready to publish while other publish blockers remain
unresolved. APIs are still pre-1.0; the current `0.1.x` surface is accepted
for first publish.

Publishable crate manifests under `crates/` should keep crate-specific
descriptions and inherit shared workspace publish metadata. Example and
verification crates should keep `publish = false`.

The approved repository URL is tracked by
[Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md).
Do not change it as a side effect of metadata verification; update it only as
part of an explicit publish-readiness review.

## Crate Responsibilities

### dioxus-ui-core

Platform-neutral shared code.

Owns:

- class composition utilities
- `UiDensity`
- later `UiPlatform` if a concrete behavior needs it
- shared component conventions
- registry data types if CLI and tests both need them

Must not depend on:

- `dioxus-ui`
- `dioxus-ui-cli`
- styled component modules

### dioxus-ui-primitives

Unstyled behavior and accessibility primitives.

Owns:

- Dialog primitive
- Popover primitive
- Select primitive
- Tooltip primitive
- focus helpers
- keyboard navigation helpers

May depend on:

- `dioxus`
- `dioxus-ui-core`

Must not depend on:

- Tailwind classes
- `dioxus-ui`
- CLI internals

### dioxus-ui

Styled public component crate.

Owns:

- component modules
- Tailwind class mappings
- root re-exports
- feature-gated styled wrappers

May depend on:

- `dioxus`
- `dioxus-ui-core`
- `dioxus-ui-primitives`

### dioxus-ui-cli

Code generation and project initialization.

Owns:

- `dxui init`
- `dxui list`
- `dxui add <component>`
- registry loading and validation
- template copying

May depend on:

- `dioxus-ui-core` for shared registry types if useful
- CLI-focused crates such as `clap`, `serde`, and `serde_json`

Must not depend on:

- `dioxus-ui`

The CLI should read templates from the repository during development and embed
or package templates for release.

## Dependency Direction

```text
dioxus-ui-core
       ▲
       │
dioxus-ui-primitives
       ▲
       │
   dioxus-ui

dioxus-ui-core
       ▲
       │
 dioxus-ui-cli
```

Invalid dependencies:

- `dioxus-ui-core` -> any project crate
- `dioxus-ui-primitives` -> `dioxus-ui`
- `dioxus-ui-cli` -> `dioxus-ui`

## Component Modules

Each component should have a module, feature flag, registry entry, template, and
documentation page.

Example for Button:

```text
crates/dioxus-ui/src/button.rs
crates/dioxus-ui-cli/registry/button.json
crates/dioxus-ui-cli/templates/button.rs
docs/components/button.md
```

Registry and templates live inside the CLI crate so `cargo package` ships them
with `dioxus-ui-cli`. Registry `source` paths such as `templates/button.rs` are
relative to the CLI crate root.

The public crate can re-export ergonomic names:

```rust
pub use button::{Button, ButtonSize, ButtonVariant};
```

But the implementation should stay module-scoped.

## Feature Flags

The styled crate should start with no default components:

```toml
[features]
default = []
button = []
input = []
textarea = []
label = []
tabs = []
accordion = []
dialog = ["dioxus-ui-primitives/dialog"]
popover = ["dioxus-ui-primitives/popover"]
tooltip = ["dioxus-ui-primitives/tooltip"]
select = ["dioxus-ui-primitives/select"]
```

Primitive features should be similarly scoped:

```toml
[features]
default = []
dialog = []
popover = []
tooltip = []
select = []
```

Feature names should match registry component names where possible.

## Platform Strategy

The first platform-related shared type should be `UiDensity`:

```rust
pub enum UiDensity {
  Compact,
  Comfortable,
  Touch,
}
```

Do not add `ButtonWeb`, `ButtonDesktop`, or `ButtonMobile`. Components should
share one API and adapt through density, variants, and later profile defaults.

## Validation Rules

Before marking M1.1 complete:

- `cargo check --workspace` succeeds
- all workspace members are included in the root manifest
- crate dependencies follow the allowed direction
- existing docs match the actual directory layout

Before marking component tasks complete:

- component feature compiles alone when practical
- source-copy template compiles in an example
- registry entry points to existing template files
- public registry names match crate feature names
- component docs and catalog entries match public registry entries
- every template is registered exactly once, with `utils` kept as a support
  template outside public component parity
