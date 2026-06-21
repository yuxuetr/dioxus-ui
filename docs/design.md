# Design Overview

## Problem

Dioxus users need reusable UI components that feel easy to adopt but remain
adaptable inside real Rust applications. A conventional black-box component
crate is convenient, but it can be difficult to customize. A pure copied-source
approach is flexible, but it needs strong conventions and tooling to stay
consistent.

`dioxus-ui` should support both workflows:

- copy component source into an app with `dxui add`
- install a crate and enable components by feature flag

The copied-source workflow is the first-class early path.

## Architecture

```text
templates + registry
        │
        ▼
   dioxus-ui-cli ───────────────► user app source tree
        │
        ▼
crates/dioxus-ui
        │
        ├── dioxus-ui-core
        └── dioxus-ui-primitives
```

The detailed Cargo workspace contract is documented in
[Workspace Specification](workspace.md).

## Crate Responsibilities

### dioxus-ui-core

Shared utilities and types that do not depend on a specific component.

Planned contents:

- class composition helpers
- design token names
- shared props conventions
- component metadata types used by tests or generation

### dioxus-ui-primitives

Unstyled behavior components for difficult interactions.

Planned contents:

- Dialog behavior
- Popover behavior
- Select behavior
- Tooltip behavior
- focus management utilities
- keyboard navigation utilities
- portal-related abstractions when Dioxus support is ready

This crate should avoid Tailwind classes.

### dioxus-ui

Styled public components for direct dependency use.

Planned contents:

- Button
- Input
- Tabs
- Dialog
- Select
- component feature flags
- re-exports from primitives when useful

### dioxus-ui-cli

Command-line tool for project setup and component source generation.

Planned commands:

```bash
dxui init
dxui add button
dxui add dialog
dxui list
```

## Registry and Templates

The registry describes each component and its file dependencies. Templates are
the source files copied into user applications.

Example registry shape:

```json
{
  "name": "button",
  "files": ["templates/button.rs"],
  "dependencies": [],
  "assets": []
}
```

The exact schema should be finalized before CLI implementation.

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
dioxus-ui = { version = "0.1", default-features = false, features = ["button", "dialog"] }
```

Planned module layout:

```text
dioxus_ui::button
dioxus_ui::input
dioxus_ui::tabs
dioxus_ui::dialog
dioxus_ui::popover
dioxus_ui::select
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

`dxui init` can later ask for a default target profile and write it to generated
configuration. Individual components should still allow explicit overrides.

## Styling Strategy

Tailwind classes are the default styled layer. Runtime selection between known
class strings is allowed. Runtime construction of class tokens is not allowed.

The style layer should be replaceable by users who copy source files.

For Tailwind CSS v4, the generated CSS entry should use CSS-first imports:

```css
@import "tailwindcss";
```

`dxui init` may also add project-level theme variables or base selectors to that
entry file. It should not treat `assets/dioxus-ui.css` as a precompiled complete
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

Complex components should use primitives so the behavior can be reused by both
styled crate components and copied templates.

## Initial Implementation Sequence

1. Finish documentation and RFCs.
2. Convert the single crate into a workspace.
3. Add core class utilities.
4. Add registry schema and validation.
5. Implement Button in source-copy mode.
6. Implement Input, Textarea, and Label.
7. Add Tabs and Accordion.
8. Design Dialog primitive before implementing overlays.
