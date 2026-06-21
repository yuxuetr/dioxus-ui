# dioxus-ui

`dioxus-ui` aims to be a shadcn/ui-style component system for Dioxus:

- headless primitives for behavior, accessibility, state, and composition
- Tailwind CSS styled components as the default visual layer
- a CLI that copies component source into user projects
- an optional packaged crate for users who prefer dependency-based usage

The project is intentionally documentation-first. The initial goal is to define
the architecture and development sequence before implementing components.

## Product Direction

The library should not be a pure Tailwind component package. The target shape is:

```text
Dioxus headless/primitive logic layer
+ Tailwind default style layer
+ CLI component source generator
```

This gives users two workflows:

1. Add source code into an app and customize it freely.
2. Depend on a crate and enable only the component features they need.

The source-copy workflow is the priority for early versions because Dioxus and
the component APIs are expected to evolve quickly.

## Planned Repository Layout

```text
dioxus-ui/
├─ crates/
│  ├─ dioxus-ui-core/         # shared types, class merging, theme tokens
│  ├─ dioxus-ui-primitives/   # unstyled logic components
│  ├─ dioxus-ui/              # styled public components
│  └─ dioxus-ui-cli/          # dxui init / dxui add
├─ registry/                  # component metadata used by the CLI
├─ templates/                 # source templates copied by the CLI
├─ examples/
│  ├─ web-demo/
│  └─ desktop-demo/
└─ docs/
   └─ rfcs/
```

Example run commands are documented in [examples/README.md](examples/README.md).

## Planned Usage

### Source-Copy Mode

```bash
cargo install dioxus-ui-cli

dxui init
dxui add button
dxui add dialog
dxui add input
```

Expected output in a Dioxus app:

```text
src/components/ui/button.rs
src/components/ui/dialog.rs
src/components/ui/input.rs
assets/dioxus-ui.css
```

For Tailwind CSS v4, `assets/dioxus-ui.css` should be an input stylesheet, not
a precompiled full Tailwind output:

```css
@import "tailwindcss";

@theme {
  --color-background: var(--dxui-background);
  --color-foreground: var(--dxui-foreground);
}
```

The user's Dioxus app build should produce the final CSS after scanning the app
source and generated component files.

### Crate Mode

```toml
[dependencies]
dioxus-ui = { version = "0.1", default-features = false, features = ["button", "input", "dialog"] }
```

```rust
use dioxus_ui::{Button, Dialog, Input};
```

Crate mode comes after the copied source API stabilizes.

## Component Scope

Early components:

- Button
- Input
- Textarea
- Label
- Checkbox
- Switch
- Badge
- Card
- Alert
- Avatar
- Separator
- Tabs
- Accordion

Later primitive-first components:

- Dialog
- Dropdown
- Popover
- Tooltip
- Toast
- Select
- Command

The later group needs stronger accessibility and interaction design, including
focus management, keyboard navigation, ARIA attributes, portal behavior,
outside-click handling, and positioning.

## Tailwind Rule

Tailwind class names must appear as complete source tokens. Runtime selection is
allowed, but runtime class construction is not.

Use this:

```rust
match variant {
  ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
  ButtonVariant::Secondary => "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
}
```

Avoid this:

```rust
format!("bg-{}-500", color)
```

## Documentation

- [Design Overview](docs/design.md)
- [Roadmap](docs/roadmap.md)
- [Workspace Specification](docs/workspace.md)
- [Component API Specification](docs/component-api.md)
- [Release and Package Strategy](docs/release.md)
- [Component Catalog](docs/components/README.md)
- [Documentation Site Plan](docs/site.md)
- [TODO Plan](TODOs.md)
- [RFC 0001: Project Architecture](docs/rfcs/0001-project-architecture.md)
- [RFC 0002: CLI Registry and Code Generation](docs/rfcs/0002-cli-registry-and-code-generation.md)
- [RFC 0003: Tailwind Styling Contract](docs/rfcs/0003-tailwind-styling-contract.md)

## References

- Dioxus RSX and UI documentation: <https://dioxuslabs.com/learn/0.7/essentials/ui/rsx/>
- Dioxus components direction: <https://github.com/DioxusLabs/dioxus-components>
- Tailwind class detection: <https://tailwindcss.com/docs/detecting-classes-in-source-files>
- Tailwind v4 installation: <https://tailwindcss.com/docs/installation>
- Dioxus component macro docs: <https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html>
