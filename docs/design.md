# Design Overview

## Problem

Dioxus users need reusable UI components that feel easy to adopt but remain
adaptable inside real Rust applications. A conventional black-box component
crate is convenient, but it can be difficult to customize. A pure copied-source
approach is flexible, but it needs strong conventions and tooling to stay
consistent.

`dioxus-shadcn` should support both workflows:

- copy component source into an app with `dxui add`
- install a crate and enable components by feature flag

The copied-source workflow is the first-class early path.

## Architecture

```text
templates + registry
        │
        ▼
   dioxus-shadcn-cli ───────────────► user app source tree
        │
        ▼
crates/dioxus-shadcn
        │
        ├── dioxus-shadcn-core
        └── dioxus-shadcn-primitives
```

The detailed Cargo workspace contract is documented in
[Workspace Specification](workspace.md).

## Crate Responsibilities

### dioxus-shadcn-core

Shared utilities and types that do not depend on a specific component.

Contents:

- class composition helpers
- design token names
- shared props conventions
- component metadata types used by tests or generation

### dioxus-shadcn-primitives

Unstyled behavior components for difficult interactions.

Contents:

- Dialog behavior
- Popover behavior
- Select behavior
- Tooltip behavior
- focus management utilities
- keyboard navigation utilities
- portal-related abstractions when Dioxus support is ready

This crate should avoid Tailwind classes.

### dioxus-shadcn

Styled public components for direct dependency use.

Contents:

- the 82 components listed in [the catalog](components/catalog.md)
- one Cargo feature per component
- re-exports from primitives when useful

### dioxus-shadcn-cli

Command-line tool for project setup and component source generation.

Commands:

```bash
dxui init [--root <path>]
dxui add <component|block>... [--root <path>] [--overwrite]
dxui diff [<component|block>...] [--root <path>]
dxui list [blocks]
dxui theme list
dxui theme add <theme>... [--root <path>]
dxui --version
```

## Registry and Templates

The registry describes each component and its file dependencies. Templates are
the source files copied into user applications.

Registry entry for Button (`crates/dioxus-shadcn-cli/registry/button.json`):

```json
{
  "name": "button",
  "description": "Button component with variants, sizes, and density-aware spacing.",
  "files": [
    {
      "source": "templates/button.rs",
      "target": "src/components/ui/button.rs"
    }
  ],
  "dependencies": [
    "utils"
  ],
  "assets": []
}
```

## Component Categories

### Static Components

Examples: Button, Badge, Card, Alert, Avatar, Separator.

Implementation approach:

- direct RSX
- enum-based variants and sizes
- complete Tailwind class strings
- minimal state

### Form Components

Examples: Input, Textarea, Label, Checkbox, Switch.

Implementation approach:

- direct RSX for simple controls
- Dioxus state and event props for interactive controls
- documented disabled, invalid, and focus-visible states

### Stateful Layout Components

Examples: Tabs, Accordion.

Implementation approach:

- Dioxus signals for selected/open state
- controlled and uncontrolled APIs where useful
- keyboard behavior documented before implementation

### Overlay Components

Examples: Dialog, Dropdown, Popover, Tooltip, Select.

Implementation approach:

- primitive-first
- accessibility behavior designed before styled wrappers
- tests for focus, keyboard navigation, and dismissal behavior

## Public API Direction

Component APIs should feel natural in Dioxus:

```rust
rsx! {
  Button {
    variant: ButtonVariant::Primary,
    size: ButtonSize::Md,
    class: "w-full",
    "Save changes"
  }
}
```

Common conventions:

- `variant` controls visual intent.
- `size` controls spacing and height.
- `class` appends user classes.
- `children` is supported for composable content.
- event props should follow Dioxus conventions.

Detailed API rules are documented in
[Component API Specification](component-api.md).

## Component Modules

Components should be modular from the beginning. Users should be able to add,
import, and feature-enable components independently.

Source-copy mode:

```bash
dxui add button
dxui add dialog
dxui add tabs
```

Crate mode:

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["button", "dialog"] }
```

Module layout:

```text
dioxus_shadcn::button
dioxus_shadcn::input
dioxus_shadcn::tabs
dioxus_shadcn::dialog
dioxus_shadcn::popover
dioxus_shadcn::select
```

The crate may re-export common components at the root for ergonomic imports,
but internal implementation and feature flags should stay component-scoped.

## Platform and Density Strategy

Tailwind's responsive utilities solve many layout differences across screen
sizes, but they do not fully solve Dioxus platform differences. Web, desktop,
and mobile can share most component source, while still needing different
defaults for pointer targets, overlay behavior, safe areas, and keyboard-heavy
workflows.

The project should avoid separate component families such as `ButtonWeb`,
`ButtonDesktop`, and `ButtonMobile`. Instead, components should support a small
set of environment-oriented knobs:

```rust
pub enum UiDensity {
  Compact,
  Comfortable,
  Touch,
}

pub enum UiPlatform {
  Web,
  Desktop,
  Mobile,
}
```

Early implementation should start with `density`, because it maps directly to
spacing, height, and hit target size. Platform-specific behavior should be added
only when a component needs it.

Recommended defaults:

- Web: `Comfortable`
- Desktop: `Compact` or `Comfortable`, depending on app type
- Mobile: `Touch`

Components take an explicit `density` prop. A `dxui init` option that writes a
default target profile is an idea, not implemented.

## Styling Strategy

Tailwind classes are the default styled layer. Runtime selection between known
class strings is allowed. Runtime construction of class tokens is not allowed.

The style layer should be replaceable by users who copy source files.

For Tailwind CSS v4, the generated CSS entry should use CSS-first imports:

```css
@import "tailwindcss";
```

`dxui init` may also add project-level theme variables or base selectors to that
entry file. In a crate-mode app it keeps one `@source` line per resolved
`dioxus-shadcn` package, from `cargo metadata`, and replaces a line naming
another version of the crate; a manifest Cargo cannot read is an error, not a
skipped line. It should not treat `assets/dioxus-shadcn.css` as a precompiled complete
Tailwind output. The final CSS output belongs to the user's app build, because
Tailwind must scan the user's application and generated component files.

## Accessibility Strategy

Accessibility must be designed per component before implementation.

Baseline requirements:

- semantic HTML first
- keyboard navigation for interactive components
- ARIA attributes only where native semantics are insufficient
- disabled states that affect behavior and visual state
- focus-visible styles
- overlay dismissal behavior that is predictable and documented

Styled crate components build their behavior on `dioxus-shadcn-primitives`.
Copied templates inline that behavior and depend only on `dioxus` and the
helper templates they use, one per crate helper module
([RFC 0074](rfcs/0074-helper-templates.md)).

Focus, dismissal, and portal behavior are designed in
[RFC 0006: Focus and Portal Primitives](rfcs/0006-focus-and-portal-primitives.md).
