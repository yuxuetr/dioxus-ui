# RFC 0010: Overlay Interaction Behavior

- Status: Accepted
- Created: 2026-10-04

## Summary

Give the styled overlay components real interaction behavior: dismissal,
modal focus management, and anchored positioning. The behavior lives in the
components themselves, uses `document::eval` for the few DOM operations Dioxus
does not expose, and is mirrored into source-copy templates through the shared
`utils.rs` template.

## Current State

As of `0.1.0` the styled overlay parts render `open` state only:

- `DialogContent`, `AlertDialogContent`, `SheetContent`, and `DrawerContent`
  set `role`, `aria-modal`, and `hidden`, but do not react to Escape, do not
  move focus on open, do not trap Tab, and do not restore focus on close.
- Overlay backdrops and close buttons have no event handlers; the app has to
  wire every close path by hand.
- `PopoverContent`, `DropdownContent`, `HoverCardContent`, and
  `TooltipContent` have no anchor and no placement; the app positions them.
- `compute_overlay_placement` in `dioxus-ui-primitives` implements side,
  alignment, offset, flip, and shift, but nothing measures real elements and
  feeds it.
- The `FocusRuntime`, `PortalRuntime`, and `MeasurementRuntime` traits only
  have record-keeping adapters in `examples/runtime-web-verification`; no
  adapter touches the DOM.

## Decision

### Dismissal

Modal content, overlay, and close parts gain two additive props:

- `on_open_change: Option<EventHandler<bool>>`: called with `false` when a
  dismissal path fires. `open` stays controlled by the app.
- `dismiss: DismissBehavior` on content and overlay, defaulting to the
  existing primitive defaults (`dialog_default()` for Dialog, Sheet, Drawer,
  and Alert Dialog).

Escape is handled with a Rust `onkeydown` on the content. Overlay pointer
dismissal is handled with a Rust `onclick` on the overlay and checks
`DismissBehavior::should_dismiss(DismissalEvent::PointerOutside)`. Close parts
call `on_open_change(false)` on click. Existing `open`-only usage keeps working
because every new prop is optional.

### Modal Focus Scope

When modal content is open, a focus-scope script runs through
`document::eval`:

1. Remember `document.activeElement`.
2. Focus the first focusable descendant, or the content itself
   (`tabindex="-1"`) when there is none.
3. Install a `keydown` listener on the content that wraps Tab and Shift+Tab
   between the first and last focusable descendants.
4. Wait for a close message from Rust, remove the listener, and restore focus
   to the remembered element when it is still connected.

The content is located through a `data-dxui-focus-scope` attribute holding a
per-instance id, so no element ids are required from the app. `document::eval`
is available in the Web, Desktop, and Mobile renderers, which keeps one code
path for all three.

### Anchored Positioning

Floating content gains `anchor_id: Option<String>`, `side`, `align`, and
`side_offset` props. When open and anchored, a page script measures the anchor,
content, and viewport on open, resize, and scroll, and writes `position: fixed`
coordinates onto the content. It applies the rules of
`compute_overlay_placement` with `FlipShift` collision handling and an 8 pixel
collision padding: flip to the opposite side when the preferred side lacks room
and the opposite side has at least as much, then clamp the cross axis inside
the padded viewport.

Placement runs in the page rather than in Rust because a Rust round trip per
layout change adds an IPC hop on Desktop and Mobile, and because source-copy
templates would otherwise need a copy of the whole placement engine. The same
script reports Escape, outside pointer, and outside focus events, which Rust
filters through `DismissBehavior`.

Without `anchor_id` the content renders exactly as in `0.1.0`.

### Source-Copy Templates

Templates cannot import internal crates. The focus-scope script, the anchored
overlay script, and their hooks live in the shared `utils.rs` template, which every generated component already depends on. The registry
dependency graph does not change.

### Runtime Traits

The runtime traits are not the integration point for this behavior. Their
only implementations are record-keeping example adapters, and templates could
not use a trait-based adapter from `dioxus-ui-primitives` anyway. The traits
stay published unchanged.

Reevaluate when a renderer without JavaScript evaluation (for example a native
Blitz renderer) needs overlay behavior; the check is whether
`document::eval` returns an error in that renderer's smoke test.

## Scope

In scope:

- Dialog, Alert Dialog, Sheet, Drawer: Escape, overlay pointer, close part,
  initial focus, Tab wrap, focus restore.
- Popover, Dropdown, Hover Card, Tooltip: anchored placement with flip and
  shift; Escape and outside dismissal according to each component's
  `DismissBehavior` default.
- Browser verification of Dialog, Alert Dialog, and Popover through
  `npm run verify:runtime-interactions`.

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Select, Combobox | Need listbox keyboard and typeahead wiring with placement; larger than this milestone | `npm run verify:runtime-interactions` gains a listbox placement fixture (done in M137, see RFC 0012) |
| Date Picker | Composes Popover and Calendar; follows Popover once Popover is verified | Popover anchored placement passes the browser smoke |
| Navigation Menu | Uses viewport-relative CSS layout, not anchored content | A consumer reports Navigation Menu content clipping |
| Context Menu | Anchors to a pointer position, not an element | `anchor_id` needs a point anchor variant |
| Menubar | Needs cross-menu roving focus | Menubar keyboard navigation fixture exists |
| DOM portal | `position: fixed` placement escapes overflow clipping without moving nodes | Fixed content is clipped by a transformed ancestor in a reported case |
| Scroll lock | Not required for focus containment | A consumer reports background scroll behind an open modal |

## Verification

- Unit tests cover dismissal decisions for script messages, and a CLI test
  keeps the template scripts identical to the crate scripts.
- The Web preview renders real Dialog, Alert Dialog, and Popover components,
  and `npm run verify:runtime-interactions` asserts Escape, overlay click, Tab
  wrap, focus restore, and in-viewport placement with flip.
- Desktop and Mobile rely on the same `document::eval` path but are not
  covered by an automated interaction gate.
