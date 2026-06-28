# Runtime Adapter Plan

This document defines the M21 runtime adapter boundaries before implementation.
The goal is to move from controlled styled parts and pure primitives toward
optional runtime integration without hiding platform-specific behavior inside
component styling APIs.

Status: Planned in M21.

## Scope

M21 covers planning for:

- focus commands
- portal mounting
- timer scheduling
- live-region announcements
- pointer gestures
- DOM or WebView measurement

The first adapter pass should stay optional. Existing crate-mode and
source-copy components must continue to work as controlled composition parts
when no adapter is installed.

M22 implemented the first primitive-layer contract slice for focus and portal
requests behind the `dioxus-ui-primitives/runtime` feature.
M23 extends the same feature with timer and live-region contracts for feedback
runtime integration.

## Ownership Model

| Layer | Owns | Does Not Own |
| --- | --- | --- |
| Primitives | pure state, deterministic helpers, typed contracts | DOM handles, timers, portals, browser APIs |
| Styled components | semantic elements, Tailwind classes, data attributes | scheduling, measurement, focus movement |
| Runtime adapters | platform commands and event wiring behind narrow traits | visual styling, business rules, application state |
| Consuming apps | routing, async workflows, persistence, labels, policy decisions | internal helper math duplicated per component |

Adapters should be small enough that Web, Desktop, and Mobile can diverge where
the renderer requires it.

## Adapter Families

### Focus Adapter

Required by:

- Dialog
- Alert Dialog
- Sheet
- Drawer
- Popover
- Select
- Combobox
- Context Menu
- Menubar
- Navigation Menu
- Date Picker

Responsibilities:

- find initial focus target
- move focus into an overlay when configured
- trap focus for modal overlays
- restore focus when an overlay closes
- ignore focus movement for Tooltip and hover-only previews

Non-goals:

- deciding whether an overlay should open
- deciding form validation state
- owning keyboard shortcut policy for the whole app

### Portal Adapter

Required by:

- modal overlays
- collision-sensitive popovers
- Toast and Sonner viewports when rendered outside local layout

Responsibilities:

- mount content inline, in body, or in a named target
- preserve ARIA relationships across the chosen portal target
- expose target availability per platform

Non-goals:

- hard-coding `body` as the default for every target
- assuming Desktop WebView portal behavior matches browser behavior
- managing app shell z-index policy

### Timer Adapter

Required by:

- Toast
- Sonner
- Carousel autoplay, if added later
- Tooltip and Hover Card delays, if added later

Responsibilities:

- schedule timeout callbacks
- cancel scheduled callbacks
- expose elapsed time for deterministic dismissal checks
- allow reduced-motion or user timing policy to disable timers

Non-goals:

- owning notification queue state
- owning promise lifecycle or async request state
- forcing auto-dismiss defaults

### Live Region Adapter

Required by:

- Toast
- Sonner
- Carousel slide announcements, if added later
- async loading or data table status, if added later

Responsibilities:

- enqueue announcement text
- choose polite or assertive channel
- suppress duplicate announcements when configured
- expose testable announcement history in demos

Non-goals:

- writing product-specific copy
- translating messages
- deciding whether errors are recoverable or destructive

### Measurement Adapter

Required by:

- Popover
- Tooltip
- Dropdown
- Select
- Menubar
- Context Menu
- Navigation Menu
- Resizable
- Chart backend evaluation

Responsibilities:

- read anchor, overlay, viewport, and panel rectangles
- subscribe to resize or scroll invalidation where available
- feed pure placement and sizing helpers

Non-goals:

- choosing a chart rendering backend
- owning virtualization
- coupling placement math to component rendering

### Pointer And Gesture Adapter

Required by:

- Resizable
- Carousel
- Drawer drag-to-dismiss, if added later

Responsibilities:

- normalize pointer down, move, up, and cancel events
- expose deltas to pure state helpers
- handle capture or cancellation differences per platform

Non-goals:

- full physics engine
- gesture libraries hidden inside styled components
- replacing native scroll behavior

## Platform Constraints

| Target | Constraints |
| --- | --- |
| Web | DOM focus, portals, measurement, timers, and live regions are available, but need browser-level verification. |
| Desktop | WebView behavior can differ for focus, stacking, pointer capture, and portals; defaults should remain conservative. |
| Mobile | Hover-only behavior is unavailable; gestures, safe areas, reduced motion, and large tap targets matter. |

## Source-Copy Strategy

Runtime adapters should not become a hard dependency for every generated
component.

Preferred source-copy model:

1. Components remain generated as controlled styled parts.
2. Optional adapter helpers can be added through separate commands later.
3. Generated adapter code should live under `src/components/ui/runtime` or a
   similarly explicit module.
4. Templates must keep Tailwind class tokens static and avoid importing
   `dioxus-ui-core` or `dioxus-ui-primitives`.

## Initial API Direction

M21 should design trait-sized contracts before implementing renderer-specific
code. Example shape:

```rust
pub trait FocusAdapter {
  type NodeId;

  fn focus_first(&self, scope: Self::NodeId) -> bool;
  fn focus_node(&self, node: Self::NodeId) -> bool;
  fn restore_focus(&self, node: Self::NodeId) -> bool;
}
```

The exact API may change after Web and Desktop verification. The important
boundary is that adapters perform commands while primitives hold state and
policies.

For the focus and portal slice, see the
[focus and portal adapter plan](focus-portal-adapters.md).
For the first implementation slice, see the
[focus and portal contract implementation plan](focus-portal-contracts.md).
For transient feedback behavior, see the
[timer and live-region adapter plan](timer-live-region-adapters.md).
For the feedback runtime contract slice, see the
[timer and live-region contract implementation plan](timer-live-region-contracts.md).
For layout and interaction measurement, see the
[measurement, pointer, and gesture adapter plan](measurement-pointer-gesture-adapters.md).
For the measurement, pointer, and gesture contract slice, see the
[measurement pointer gesture contract implementation plan](measurement-pointer-gesture-contracts.md).

## Implementation Order

1. Runtime adapter boundary plan
2. Focus and portal adapter plan
3. Timer and live-region adapter plan
4. Measurement, pointer, and gesture adapter plan
5. Documentation updates and next implementation milestone

## Quality Gates

Before implementation starts:

- every adapter family has clear ownership boundaries
- Web/Desktop/Mobile differences are documented
- accessibility checklist links planned adapter work to affected components
- the next milestone identifies one narrow adapter slice to implement first
