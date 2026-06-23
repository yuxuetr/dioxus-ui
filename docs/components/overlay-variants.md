# Overlay Variant API Plan

This document defines the M12 overlay variant APIs before implementation. The
goal is to reuse the existing Dialog, Popover, Tooltip, dismissal, and placement
primitives instead of creating independent behavior models for every overlay.

Status: Implemented in M12.

## Scope

M12 covers:

- Alert Dialog: implemented
- Sheet: implemented
- Drawer: implemented
- Hover Card: implemented

These components should ship in both crate mode and source-copy mode. Crate mode
can reuse `dioxus-ui-core` and `dioxus-ui-primitives`; generated templates must
remain self-contained and must not import internal crates.

## Shared Rules

All overlay variants use controlled state first:

- `open: bool`
- typed variant props for side, size, and intent where needed
- `class: String` on every styled part
- `children: Element` for content slots

All Tailwind classes must be complete static tokens in source. Runtime selection
between predefined strings is allowed; runtime construction of class names is
not allowed.

Overlay primitives remain responsible for:

- focus strategy and focus return configuration
- dismissal decisions for Escape, outside pointer, and outside focus
- portal target configuration
- preferred side and alignment data
- pure placement math

Styled components remain responsible for:

- semantic HTML elements and ARIA attributes
- Tailwind class tokens
- variant and size class selection
- source-copy-compatible composition APIs

## Alert Dialog

Alert Dialog is a modal confirmation layer built on Dialog behavior. It is for
destructive, irreversible, or high-impact confirmation flows.

Planned crate API:

```rust
AlertDialogOverlay { open, class }
AlertDialogContent { open, class, children }
AlertDialogHeader { class, children }
AlertDialogFooter { class, children }
AlertDialogTitle { class, children }
AlertDialogDescription { class, children }
AlertDialogAction { variant, class, disabled, children }
AlertDialogCancel { class, disabled, children }
```

Behavior defaults:

- `role="alertdialog"` on content
- `aria-modal="true"` when open
- `DialogPrimitiveConfig::controlled(open)` as the base primitive config
- Escape dismissal enabled
- outside pointer dismissal disabled
- focus return to trigger
- initial focus strategy stays `FirstFocusable`

The first implementation will expose styled parts only. Full DOM focus trapping
and automatic trigger wiring remain deferred until the portal/focus runtime
adapters are implemented.

## Sheet

Sheet is a modal panel that enters from one viewport side. It should be the
general side-panel primitive for settings, secondary forms, and contextual
workflows.

Planned crate API:

```rust
SheetSide::{Top, Right, Bottom, Left}
SheetOverlay { open, class }
SheetContent { open, side, class, children }
SheetHeader { class, children }
SheetFooter { class, children }
SheetTitle { class, children }
SheetDescription { class, children }
SheetClose { class, disabled, children }
```

Behavior defaults:

- Dialog-backed modal semantics
- Escape dismissal enabled
- outside pointer dismissal disabled by default
- focus return to trigger
- `SheetSide::Right` default for desktop-style layouts

Side classes are predefined static Tailwind strings. The first implementation
does not require a runtime animation engine; it only exposes `data-state` and
`data-side` hooks for future motion classes.

## Drawer

Drawer is a mobile-oriented overlay for bottom-first task flows. It should not
duplicate Sheet behavior unless the API needs diverge.

Planned approach:

- implement after Sheet
- expose a separate public component, not a pure alias
- reuse Sheet semantics while keeping bottom-first mobile defaults
- default side is bottom
- default content height is smaller than full-screen but easy to override
- keep touch gesture support out of M12

Planned crate API:

```rust
DrawerOverlay { open, class }
DrawerContent { open, class, children }
DrawerHeader { class, children }
DrawerFooter { class, children }
DrawerTitle { class, children }
DrawerDescription { class, children }
DrawerClose { class, disabled, children }
```

Decision: Drawer ships as its own public component because mobile-oriented
bottom sheets need different default sizing and documentation than desktop side
panels. It remains dialog-backed and intentionally does not introduce a new
primitive state model.

## Hover Card

Hover Card is non-modal preview content associated with a trigger. It is closer
to Popover than Tooltip because it can contain rich content.

Planned crate API:

```rust
HoverCardContent { open, side, align, class, children }
HoverCardHeader { class, children }
HoverCardTitle { class, children }
HoverCardDescription { class, children }
```

Behavior defaults:

- popover-backed placement fields
- non-modal content
- Escape dismissal enabled
- outside pointer and outside focus dismissal enabled
- default side `Bottom`
- default align `Center`

The first implementation is controlled-only. Hover timing, open delay, close
delay, and pointer intent tracking are runtime adapter work and should not block
the styled API.

Mobile behavior should not depend on hover. Docs and examples should present
mobile usage as controlled tap/click disclosure or recommend Popover/Sheet for
critical actions.

## Platform Defaults

| Target | Modal overlays | Non-modal overlays |
| --- | --- | --- |
| Web | `PortalTarget::Body` once runtime adapters are verified; otherwise inline | body when anchored positioning is available; otherwise inline |
| Desktop | inline until WebView focus and stacking behavior are verified | inline |
| Mobile | Sheet/Drawer-style presentation preferred | tap-controlled disclosure or inline fallback |

The primitive default remains `PortalTarget::Inline` until runtime portal
behavior is implemented and tested across targets.

## Implementation Order

Completed order:

1. Alert Dialog
2. Sheet
3. Drawer
4. Hover Card
5. documentation, examples, and parity updates

This order exercises the existing Dialog foundation before adding side-panel
styling and non-modal rich preview behavior.

## Quality Gates

Each M12 component should include:

- crate-mode class composition tests
- primitive default tests when a new config type is introduced
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking each task done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
