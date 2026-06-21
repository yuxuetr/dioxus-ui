# RFC 0006: Focus and Portal Primitives

- Status: Draft
- Created: 2026-06-21

## Summary

Design overlay-related primitives before implementing Dialog, Popover, Tooltip,
Dropdown, and Select.

The first primitive layer should describe behavior boundaries and typed state.
It should avoid browser-specific DOM control until a concrete Dioxus API and
test strategy are selected.

## Motivation

Overlay components are where UI libraries usually accumulate hidden complexity:

- focus entry
- focus return
- Escape key dismissal
- outside pointer dismissal
- scroll locking
- portal placement
- ARIA relationships
- mobile presentation differences

Static Tailwind components do not solve these behaviors. They need primitive
logic that can be reused by styled crate components and copied templates.

## Design Principles

- Prefer semantic HTML and ARIA contracts before custom behavior.
- Keep primitive state unstyled.
- Keep primitive APIs reusable by both crate mode and source-copy mode.
- Add DOM-specific behavior only behind small functions or adapters.
- Design for Web first, but do not make mobile and desktop behavior impossible.

## Focus Primitives

Initial focus concepts:

```rust
pub enum FocusStrategy {
  FirstFocusable,
  Container,
  None,
}

pub enum FocusReturn {
  Trigger,
  None,
}
```

Planned usage:

- Dialog defaults to `FirstFocusable` on open.
- Dialog returns focus to trigger on close.
- Popover may focus content or leave focus on trigger depending on use case.
- Tooltip should not move focus.

## Dismissal Primitives

Initial dismissal concepts:

```rust
pub struct DismissBehavior {
  pub escape_key: bool,
  pub outside_pointer: bool,
  pub focus_outside: bool,
}
```

Recommended defaults:

```text
Dialog   -> Escape yes, outside pointer configurable, focus outside no
Popover  -> Escape yes, outside pointer yes, focus outside yes
Tooltip  -> Escape yes, outside pointer no, focus outside no
Select   -> Escape yes, outside pointer yes, focus outside yes
```

## Portal Primitives

Portal behavior should be designed as a target abstraction, not hard-coded into
every component.

Initial concept:

```rust
pub enum PortalTarget {
  Body,
  Selector(String),
  Inline,
}
```

Recommended defaults:

```text
Web      -> Body when supported
Desktop  -> Body or Inline depending on WebView behavior
Mobile   -> Body for modal overlays, Inline for simple disclosure
```

The first implementation may use `Inline` until Dioxus portal behavior is
verified in examples.

## Overlay State

Overlay primitives should support controlled state first:

```rust
pub struct OpenState {
  pub open: bool,
}
```

Future API can add uncontrolled helpers, but the initial implementation should
not hide state transitions inside components before keyboard and focus behavior
is tested.

## Platform Notes

Web:

- ARIA and focus behavior are the main baseline.
- Portal target should default to document body when available.

Desktop:

- keyboard workflows are more important.
- WebView focus behavior must be verified in the desktop example.

Mobile:

- touch targets and safe areas matter.
- Dialog and Select may need full-screen or bottom-sheet presentation variants.
- Tooltip should not rely on hover.

## Implementation Sequence

1. Add primitive state types in `dioxus-ui-primitives`.
2. Add tests for default behavior values.
3. Implement Dialog primitive before styled Dialog.
4. Verify focus and portal behavior in web example.
5. Verify desktop WebView behavior before calling Dialog stable.

## Non-Goals

- Do not implement a full focus trap in this RFC.
- Do not pick a positioning engine yet.
- Do not implement Popover/Select before Dialog behavior is understood.
- Do not require JavaScript code in user applications for the first primitive
  design.

## Open Questions

- Which Dioxus portal API should be used for Web and Desktop?
- Should focus helpers live in `dioxus-ui-primitives` or a lower-level crate?
- Should copied templates depend on `dioxus-ui-primitives` for overlay behavior,
  or copy primitive code into the user project?
