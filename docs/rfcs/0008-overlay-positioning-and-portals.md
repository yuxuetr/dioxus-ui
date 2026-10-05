# RFC 0008: Overlay Positioning and Portals

- Status: Draft
- Created: 2026-06-22

## Summary

Define the positioning and portal plan for overlays before implementing the next
wave of complex components.

This RFC builds on RFC 0006 and focuses on anchor measurement, placement,
collision handling, and platform limitations.

## Goals

- Define reusable placement inputs for Popover, Dropdown, Tooltip, Select,
  Menubar, Context Menu, Hover Card, Sheet, and Navigation Menu.
- Keep positioning independent from Tailwind styling.
- Allow copied templates to remain self-contained when possible.
- Avoid committing to a browser-only implementation before Desktop and Mobile
  constraints are understood.

## Non-Goals

- Implement a concrete positioning engine.
- Implement collision-aware behavior in templates immediately.
- Replace native platform conventions for mobile overlays.

## Positioning Inputs

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverlaySide {
  Top,
  Right,
  Bottom,
  Left,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverlayAlign {
  Start,
  Center,
  End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OverlayOffset {
  pub main_axis: i16,
  pub cross_axis: i16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollisionPadding {
  pub top: u16,
  pub right: u16,
  pub bottom: u16,
  pub left: u16,
}
```

## Placement Model

Each overlay should be computed from:

- anchor rectangle
- overlay size
- viewport rectangle
- side
- align
- offset
- collision padding
- collision strategy

The output should be:

- x/y position
- resolved side
- transform origin token
- whether fallback placement was used

## Collision Strategy

Initial strategies:

| Strategy | Behavior |
| --- | --- |
| `None` | Use requested placement even if clipped. |
| `Flip` | Try opposite side when the preferred side has insufficient space. |
| `Shift` | Keep side but move along the cross axis to remain visible. |
| `FlipShift` | Flip first, then shift if needed. |

The first implementation can expose state and class hooks before fully
implementing runtime measurement.

## Portal Defaults

| Platform | Default | Notes |
| --- | --- | --- |
| Web | `Body` for overlays, `Inline` for simple disclosure | Body portals avoid clipping in scroll containers. |
| Desktop | `Inline` until WebView focus and z-index behavior is verified | Body portals may behave differently across renderers. |
| Mobile | `Inline` for simple overlays, full-screen/sheet pattern for modal selection | Avoid hover-only tooltip behavior. |

## Primitive Boundary

Belongs in primitives:

- typed placement state
- collision strategy types
- platform default selection
- resolved placement state
- focus and dismissal interaction with portal target

Belongs in styled components:

- Tailwind classes for side and animation
- default dimensions
- spacing tokens
- visual transform origin usage

Belongs in examples/tests:

- Web measurement verification
- Desktop WebView behavior checks
- Mobile-safe presentation examples

## Implementation Sequence

1. Add placement and collision types to `dioxus-shadcn-primitives`.
2. Add pure tests for placement fallback math.
3. Add side/align data attributes to Popover, Tooltip, Dropdown, and Select.
4. Verify Web overlay positioning in a real example.
5. Verify Desktop behavior before enabling portal body defaults there.
6. Use the same placement API for Context Menu, Menubar, Hover Card, and
   Navigation Menu.

## Risks

- Dioxus renderer differences may make direct DOM measurement unavailable in
  some targets.
- A browser-only positioning implementation can break Desktop or Mobile.
- Collision behavior can become hard to test if it is coupled to component
  rendering instead of pure placement math.

## Open Questions

- Should the first real implementation wrap an existing positioning crate or use
  a small first-party placement module?
- Should source-copy templates include placement math or depend on a generated
  local `utils.rs` module?
- Should mobile Select use a sheet-style presentation by default?
