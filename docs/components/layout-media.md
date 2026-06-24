# Layout Shells and Media

This document defines the M17 Sidebar, Scroll Area, Resizable, and Carousel
APIs and completion status. The goal is to provide controlled layout and media
composition parts backed by pure state primitives, without taking ownership of
DOM measurement, pointer dragging, gesture recognition, autoplay, or
persistence too early.

Status: Implemented in M17.

## Scope

M17 covers:

- Scroll Area
- Resizable
- Sidebar
- Carousel

These components should ship in crate mode and source-copy mode. Crate mode can
reuse `dioxus-ui-core` and `dioxus-ui-primitives`; generated templates must
remain self-contained and must not import internal crates.

## Shared Primitive Strategy

M17 added pure layout/media state helpers:

```rust
SidebarState { collapsed }
ResizablePanelState { size, min_size, max_size, collapsed }
ScrollAreaOrientation::{Vertical, Horizontal, Both}
CarouselState { index, item_count, looping }
```

Shipped helpers:

```rust
sidebar_toggle(collapsed) -> bool
resizable_clamp(size, min_size, max_size) -> f64
resizable_resize_pair(first, second, delta) -> (f64, f64)
scroll_area_orientation_attribute(orientation) -> &'static str
carousel_next(index, item_count, looping) -> usize
carousel_previous(index, item_count, looping) -> usize
carousel_can_go_next(index, item_count, looping) -> bool
carousel_can_go_previous(index, item_count, looping) -> bool
```

Rules:

- helpers are deterministic and pure
- no DOM measurement in primitives
- no pointer event ownership in primitives
- no storage or persistence in primitives
- IDs, labels, and persisted state are app-owned

## Scroll Area

Scroll Area wraps native scrolling with styled parts. It should not replace
native scroll behavior in the first implementation.

Shipped crate API:

```rust
ScrollArea { orientation, class, children }
ScrollAreaViewport { class, children }
ScrollAreaContent { class, children }
ScrollAreaScrollbar { orientation, class, children }
ScrollAreaThumb { class }
ScrollAreaCorner { class }
```

Behavior defaults:

- viewport uses native overflow behavior
- orientation maps to data attributes and class helpers
- scrollbar and thumb are styled hooks, not custom runtime scrollbars yet
- mobile behavior should rely on native scrolling

## Resizable

Resizable provides controlled panel layout parts. It does not own dragging or
measurement in the first implementation.

Shipped crate API:

```rust
ResizablePanelGroup { orientation, class, children }
ResizablePanel { size, min_size, max_size, collapsed, class, children }
ResizableHandle { disabled, class }
```

Behavior defaults:

- orientation is horizontal or vertical
- panel size is controlled and represented with inline flex-basis style helpers
- handle is a semantic separator with orientation data
- pointer dragging and layout measurement are app-owned

## Sidebar

Sidebar provides a controlled app shell and navigation composition surface. It
does not own routing, persistence, or keyboard shortcuts.

Shipped crate API:

```rust
Sidebar { collapsed, side, class, children }
SidebarRail { collapsed, class }
SidebarHeader { class, children }
SidebarContent { class, children }
SidebarFooter { class, children }
SidebarGroup { class, children }
SidebarGroupLabel { class, children }
SidebarItem { active, disabled, class, children }
SidebarTrigger { collapsed, disabled, class, children }
```

Behavior defaults:

- collapsed state is controlled
- side can be left or right
- active and disabled item states map to data attributes
- persistence and keyboard shortcuts are app-owned

## Carousel

Carousel provides controlled slide composition parts. Gesture recognition,
momentum, and autoplay are deferred.

Shipped crate API:

```rust
Carousel { orientation, class, children }
CarouselViewport { class, children }
CarouselContent { orientation, class, children }
CarouselItem { selected, class, children }
CarouselPrevious { disabled, class, children }
CarouselNext { disabled, class, children }
CarouselIndicator { selected, class }
CarouselOrientation::{Horizontal, Vertical}
```

Behavior defaults:

- index state is controlled by the app
- previous/next availability comes from carousel helpers
- orientation maps to static class and data attributes
- gestures, snapping physics, and autoplay are deferred

## Platform Defaults

| Target | Layout and media defaults |
| --- | --- |
| Web | Use native scrolling, controlled layout state, and app-owned pointer handlers. |
| Desktop | Avoid assuming DOM measurement or browser-specific scrollbar styling. |
| Mobile | Prefer native scrolling and tap controls; do not rely on hover or precision dragging. |

## Implementation Order

1. Layout and media API plan
2. Layout state primitives
3. Scroll Area
4. Resizable
5. Sidebar
6. Carousel
7. documentation, examples, and parity updates

This order starts with the lowest-risk native scroll wrapper, then adds
controlled layout and media composition.

## Quality Gates

Each M17 component includes:

- primitive unit tests for state helpers
- crate-mode class composition tests
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking implementation tasks done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
