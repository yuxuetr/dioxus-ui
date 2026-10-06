# Workspace Specification

## Goal

The repository is a Cargo workspace that supports two distribution modes:

- source-copy mode through `dxui add`
- crate mode through feature-gated `dioxus-shadcn` exports

Both modes share one component API, so the module boundaries below serve both.

## Layout

```text
dioxus-ui/
├─ Cargo.toml
├─ crates/
│  ├─ dioxus-shadcn-core/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ dioxus-shadcn-primitives/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ dioxus-shadcn/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  └─ dioxus-shadcn-cli/
│     ├─ Cargo.toml
│     ├─ registry/
│     ├─ templates/
│     └─ src/main.rs
├─ examples/                # demos, preview fixtures, runtime verification
├─ site/                    # component site
└─ docs/
```

## Workspace Manifest

The root manifest is virtual:

```toml
[workspace]
resolver = "2"
members = [
  "crates/dioxus-shadcn-core",
  "crates/dioxus-shadcn-primitives",
  "crates/dioxus-shadcn",
  "crates/dioxus-shadcn-cli",
  "examples/web-demo",
  "examples/desktop-demo",
  "examples/mobile-demo",
  "examples/preview-states",
  "examples/runtime-web-verification",
  "examples/runtime-desktop-verification",
  "site",
]

[workspace.package]
version = "0.3.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/yuxuetr/dioxus-ui"
readme = "crates/README.md"
keywords = ["dioxus", "ui", "tailwind", "components"]
categories = ["gui", "web-programming"]

[workspace.dependencies]
dioxus = "0.7"
dioxus-shadcn-core = { version = "0.3.0", path = "crates/dioxus-shadcn-core" }
dioxus-shadcn-primitives = { version = "0.3.0", path = "crates/dioxus-shadcn-primitives" }
dioxus-shadcn = { version = "0.3.0", path = "crates/dioxus-shadcn" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

The repository URL has been approved for first publish preparation.
The shared README, keywords, and categories are publish metadata. 0.1.0 and 0.2.0
were published on 2026-10-05. APIs are still pre-1.0; breaking changes bump
the minor version, and the current `0.3.x` surface is accepted.

Publishable crate manifests under `crates/` should keep crate-specific
descriptions and inherit shared workspace publish metadata. Example and
verification crates should keep `publish = false`.

The approved repository URL is tracked by
[Repository Identity Readiness Metadata](archive/first-publish/repository-identity-readiness-metadata.md).
Do not change it as a side effect of metadata verification; update it only as
part of an explicit publish-readiness review.

## Crate Responsibilities

### dioxus-shadcn-core

Platform-neutral shared code.

Owns:

- class composition utilities
- `UiDensity`
- later `UiPlatform` if a concrete behavior needs it
- shared component conventions
- registry data types if CLI and tests both need them

Must not depend on:

- `dioxus-shadcn`
- `dioxus-shadcn-cli`
- styled component modules

### dioxus-shadcn-primitives

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
- `dioxus-shadcn-core`

Must not depend on:

- Tailwind classes
- `dioxus-shadcn`
- CLI internals

### dioxus-shadcn

Styled public component crate.

Owns:

- component modules
- Tailwind class mappings
- root re-exports
- feature-gated styled wrappers

May depend on:

- `dioxus`
- `dioxus-shadcn-core`
- `dioxus-shadcn-primitives`

### dioxus-shadcn-cli

Code generation and project initialization.

Owns:

- `dxui init`
- `dxui list`
- `dxui add <component>`
- registry loading and validation
- template copying

May depend on:

- `dioxus-shadcn-core` for shared registry types
- `serde_json`; arguments are parsed by hand

Must not depend on:

- `dioxus-shadcn`

The CLI embeds the registry and templates at compile time (`build.rs`).

## Dependency Direction

```text
dioxus-shadcn-core
       ▲
       │
dioxus-shadcn-primitives
       ▲
       │
   dioxus-shadcn

dioxus-shadcn-core
       ▲
       │
 dioxus-shadcn-cli
```

Invalid dependencies:

- `dioxus-shadcn-core` -> any project crate
- `dioxus-shadcn-primitives` -> `dioxus-shadcn`
- `dioxus-shadcn-cli` -> `dioxus-shadcn`

## Component Modules

Each component should have a module, feature flag, registry entry, template, and
documentation page.

Example for Button:

```text
crates/dioxus-shadcn/src/button.rs
crates/dioxus-shadcn-cli/registry/button.json
crates/dioxus-shadcn-cli/templates/button.rs
docs/components/button.md
```

Registry and templates live inside the CLI crate so `cargo package` ships them
with `dioxus-shadcn-cli`. Registry `source` paths such as `templates/button.rs` are
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
dialog = ["dioxus-shadcn-primitives/dialog"]
popover = ["dioxus-shadcn-primitives/popover"]
tooltip = ["dioxus-shadcn-primitives/tooltip"]
select = ["dioxus-shadcn-primitives/select"]
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
