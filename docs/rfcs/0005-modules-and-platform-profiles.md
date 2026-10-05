# RFC 0005: Modules and Platform Profiles

- Status: Draft
- Created: 2026-06-21

## Summary

Organize `dioxus-shadcn` by component modules and support platform-sensitive
defaults through density/profile settings instead of separate component
families.

## Motivation

Tailwind CSS already provides responsive utilities, which are useful for web,
desktop, and mobile layouts. But cross-platform Dioxus UI also has differences
that are not just CSS breakpoints:

- desktop apps often need denser controls and stronger keyboard workflows
- mobile apps need larger touch targets and safe-area-aware overlays
- web apps need browser-friendly semantics, forms, and SSR awareness
- overlays can need different placement or presentation on small screens

The library should support those differences without multiplying the public API
into separate `Web`, `Desktop`, and `Mobile` components.

## Decision

Use one component API per component:

```rust
Button
Dialog
Select
Tabs
```

Do not create component families like:

```rust
ButtonWeb
ButtonDesktop
ButtonMobile
```

Instead, support reusable configuration concepts.

## Density

Density should be the first cross-platform knob because it directly affects
spacing and hit target size.

```rust
pub enum UiDensity {
  Compact,
  Comfortable,
  Touch,
}
```

Recommended interpretation:

- `Compact`: dense desktop tools, admin panels, data-heavy views
- `Comfortable`: default web apps and mixed input environments
- `Touch`: mobile and touch-first interfaces

Components can map density to concrete class strings:

```rust
match density {
  UiDensity::Compact => "h-8 px-3 text-sm",
  UiDensity::Comfortable => "h-10 px-4 text-sm",
  UiDensity::Touch => "h-12 px-5 text-base",
}
```

## Platform Profile

Platform should be treated as a profile, not as a different component tree.

```rust
pub enum UiPlatform {
  Web,
  Desktop,
  Mobile,
}
```

Possible defaults:

```text
Web      -> Comfortable
Desktop  -> Compact or Comfortable
Mobile   -> Touch
```

The first implementation may omit `UiPlatform` and expose only `UiDensity`.
Platform profiles should be added when there is a concrete behavior difference,
for example Dialog presentation or Select interaction.

## Component Modules

Every component should be independently addable and feature-gated.

Source-copy mode:

```bash
dxui add button
dxui add dialog
```

Crate mode:

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["button", "dialog"] }
```

Planned module structure:

```text
dioxus_shadcn::button
dioxus_shadcn::dialog
dioxus_shadcn::input
dioxus_shadcn::tabs
dioxus_shadcn::select
```

Root re-exports are allowed for ergonomics:

```rust
use dioxus_shadcn::{Button, Dialog};
```

But implementation, tests, registry entries, and feature flags should remain
component-scoped.

## Overlay Components

Overlay components are where platform profiles matter most.

Examples:

- Dialog may be centered on desktop but full-screen or bottom-sheet-like on
  mobile.
- Select may use a popover on desktop but a larger touch-friendly panel on
  mobile.
- Tooltip may be unavailable or converted to a press-triggered disclosure on
  touch devices.

These differences should be expressed through variants and profile defaults,
not through separate component names.

## CLI Behavior

`dxui init` may eventually ask for a default profile:

```bash
dxui init --profile web
dxui init --profile desktop
dxui init --profile mobile
```

Generated configuration can set a default density or profile. Individual
components should still support explicit overrides.

## Consequences

Benefits:

- one public API works across Dioxus targets
- users can install only the components they need
- mobile and desktop needs are considered without early over-splitting

Costs:

- component classes need density variants from the beginning
- complex overlays need more design work before implementation
- profile defaults need documentation and examples

## Open Questions

- Should `UiDensity` live in `dioxus-shadcn-core` from M1?
- Should `dxui init` write a config file, or should defaults stay in Rust code?
- Should profile defaults be compile-time features, runtime context, or both?
