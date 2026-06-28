# Measurement, Pointer, And Gesture Adapter Plan

This document defines the M21.4 measurement, pointer, and gesture adapter
contracts. It builds on the [runtime adapter plan](runtime-adapters.md), RFC
0008, and the M17 [layout and media plan](layout-media.md).

Status: Planned in M21.

## Goals

- Define anchor and viewport measurement boundaries for overlay positioning.
- Define pointer drag boundaries for Resizable and future draggable surfaces.
- Define gesture boundaries for Carousel and future mobile interactions.
- Keep placement math, sizing math, and index math pure and independently
  testable.

## Non-Goals

- Implement a positioning engine.
- Implement pointer capture.
- Implement carousel physics.
- Add virtualization or chart rendering backends.
- Replace native scrolling.

## Measurement Adapter

Required by:

- Popover
- Tooltip
- Dropdown
- Select
- Combobox
- Date Picker
- Context Menu
- Menubar
- Navigation Menu
- Hover Card
- Resizable
- Chart backend evaluation

Responsibilities:

- read anchor rectangle
- read floating content rectangle
- read viewport rectangle
- read resizable panel group rectangle
- subscribe to resize and scroll invalidation where available
- feed pure placement and sizing helpers

The adapter should report measurements; it should not decide product-specific
placement policy or own component state.

## Measurement Contract Shape

```rust
pub struct RuntimeRect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

pub enum MeasurementResult {
  Rect(RuntimeRect),
  Missing,
  Unsupported,
}

pub trait MeasurementRuntime {
  type NodeId;

  fn measure_node(&self, node: Self::NodeId) -> MeasurementResult;
  fn measure_viewport(&self) -> MeasurementResult;
}
```

Pure placement helpers should continue to accept plain rectangles so they can be
unit-tested without a renderer.

## Pointer Adapter

Required by:

- Resizable
- future splitter handles
- Drawer drag-to-dismiss, if added later

Responsibilities:

- normalize pointer down, move, up, and cancel events
- expose pointer delta in logical pixels
- expose active pointer identity where the platform supports it
- support cancellation when the pointer leaves or the component unmounts

Pointer adapters should not mutate panel state directly. They should emit
deltas that the app or controlled component can apply through pure helpers such
as `resizable_resize_pair`.

## Pointer Contract Shape

```rust
pub struct PointerDelta {
  pub delta_x: f64,
  pub delta_y: f64,
}

pub enum PointerPhase {
  Start,
  Move,
  End,
  Cancel,
}

pub struct PointerEventState {
  pub phase: PointerPhase,
  pub delta: PointerDelta,
}
```

The implementation may need renderer-specific event types later, but adapters
should reduce them to stable state before they reach primitives.

## Gesture Adapter

Required by:

- Carousel swipe gestures
- Drawer drag-to-dismiss, if added later
- mobile sheet interactions, if added later

Responsibilities:

- detect primary axis
- expose drag distance and velocity when available
- identify cancel versus commit
- allow app policy to disable gestures
- avoid blocking native scroll when the gesture is ambiguous

Non-goals:

- inertia physics
- autoplay
- route transitions
- global gesture arbitration

## Gesture Contract Shape

```rust
pub enum GestureAxis {
  Horizontal,
  Vertical,
}

pub enum GestureOutcome {
  CommitNext,
  CommitPrevious,
  Cancel,
}

pub struct GestureState {
  pub axis: GestureAxis,
  pub distance: f64,
  pub velocity: f64,
}
```

Carousel should still use pure index helpers for the final state transition.
The gesture adapter only decides whether a gesture should request next,
previous, or cancel.

## Component Mapping

| Component | Measurement | Pointer | Gesture | App-Owned Policy |
| --- | --- | --- | --- | --- |
| Popover | anchor, content, viewport | none | none | placement preference and open state |
| Tooltip | anchor, content, viewport | none | none | hover/focus timing and essential-content policy |
| Dropdown | trigger, content, viewport | none | none | menu open state and nested behavior |
| Select | trigger, content, viewport | none | mobile presentation later | selected value and filtering |
| Menubar | trigger/content chain | none | none | nested submenu policy |
| Context Menu | invocation point and content | none | none | invocation target and item actions |
| Navigation Menu | trigger/content/viewport | none | none | viewport animation policy |
| Resizable | panel group and handle | drag delta | none | persisted panel sizes |
| Carousel | viewport and slide size later | optional pointer stream | swipe | index state, snapping, autoplay |
| Chart | plot area and container | pointer hover later | pinch/zoom deferred | backend and data formatting |

## Platform Defaults

| Target | Measurement | Pointer | Gesture |
| --- | --- | --- | --- |
| Web | DOM rectangles and resize/scroll invalidation after browser verification | pointer events where available | touch/pointer gestures with native-scroll escape |
| Desktop | WebView measurement verification required | pointer capture may differ | gesture support conservative |
| Mobile | safe-area and visual viewport concerns | touch-first pointer stream | avoid hover and preserve scroll |

## Test Strategy

Before implementation:

- pure placement math remains unit-tested without adapters
- pure resizable math remains unit-tested without adapters
- Web example verifies anchor collision after scroll and resize
- Desktop example verifies WebView measurement and pointer deltas
- Mobile checks preserve native scroll when carousel gestures are ambiguous

## Source-Copy Strategy

Generated components should not include measurement or gesture runtimes by
default. Future optional commands can add runtime helpers explicitly:

```bash
dxui add runtime-measurement
dxui add runtime-pointer
dxui add runtime-gestures
```

This keeps source-copy components portable while still allowing richer behavior
when an app opts in.

## Implementation Order

1. Keep placement and sizing helpers pure.
2. Add measurement contracts and verify Popover as the first overlay slice.
3. Add pointer contracts and verify Resizable as the first pointer slice.
4. Add gesture contracts and verify Carousel as the first gesture slice.
5. Revisit Chart backend evaluation only after measurement contracts are proven.

M24 turns this plan into concrete primitive-layer contracts. See the
[measurement pointer gesture contract implementation plan](measurement-pointer-gesture-contracts.md).
