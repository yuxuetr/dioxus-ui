# Measurement Pointer Gesture Contract Implementation Plan

This document defines the M24.1 implementation plan for measurement, pointer,
and gesture adapter contracts. It turns the M21 measurement/pointer/gesture
plan into a narrow primitive-layer code surface before any renderer-specific
runtime is added.

Status: Planned in M24.

## Decision

Measurement, pointer, and gesture adapter contracts should extend
`dioxus-ui-primitives/src/runtime.rs` behind the existing `runtime` feature.

Rationale:

- focus, portal, timer, and live-region contracts already share the same
  primitive-layer runtime module
- placement, resizable, and carousel helper math already live in
  `dioxus-ui-primitives`
- these contracts describe runtime command boundaries, not styled component
  behavior
- keeping the contracts together makes unsupported fallback behavior consistent
- a separate crate or feature should wait until renderer implementations add
  dependency pressure

The first implementation should add contracts only. It should not measure DOM
nodes, capture pointers, implement carousel physics, mutate resizable panel
state, or arbitrate native scrolling.

## Module Location

Planned file:

```text
crates/dioxus-ui-primitives/src/runtime.rs
```

Initial public exports:

```rust
RuntimeRect
MeasurementRuntimeRequest
MeasurementRuntimeResult
MeasurementRuntime
MeasurementRuntimeUnsupported
PointerDelta
PointerPhase
PointerRuntimeRequest
PointerRuntimeResult
PointerRuntime
PointerRuntimeUnsupported
GestureAxis
GestureState
GestureOutcome
GestureRuntimeRequest
GestureRuntimeResult
GestureRuntime
GestureRuntimeUnsupported
```

The implementation may refine names, but it should preserve these concepts:

- explicit request structs for deterministic tests
- explicit unsupported results
- no DOM or WebView handles in public types
- no component state mutation from runtime contracts
- no physics or native-scroll policy inside primitives

## Feature Strategy

Use the existing feature:

```toml
runtime = []
```

Do not add separate `measurement-runtime`, `pointer-runtime`, or
`gesture-runtime` features in M24. The contracts remain inert without concrete
runtime implementations.

## Measurement Contract Shape

```rust
pub struct RuntimeRect {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

pub enum MeasurementRuntimeRequest<NodeId> {
  Node(NodeId),
  Viewport,
}

pub enum MeasurementRuntimeResult {
  Rect(RuntimeRect),
  Missing,
  Unsupported,
}

pub trait MeasurementRuntime {
  type NodeId;

  fn measure(&self, request: &MeasurementRuntimeRequest<Self::NodeId>) -> MeasurementRuntimeResult;
}
```

Measurement contracts report rectangles only. Pure placement helpers should
continue to own collision math.

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

pub struct PointerRuntimeRequest {
  pub phase: PointerPhase,
  pub delta: PointerDelta,
}

pub enum PointerRuntimeResult {
  Started,
  Moved(PointerDelta),
  Ended,
  Cancelled,
  Unsupported,
}
```

Pointer contracts normalize event state only. Resizable state changes remain
owned by pure helpers and consuming apps.

## Gesture Contract Shape

```rust
pub enum GestureAxis {
  Horizontal,
  Vertical,
}

pub struct GestureState {
  pub axis: GestureAxis,
  pub distance: f64,
  pub velocity: f64,
}

pub enum GestureOutcome {
  CommitNext,
  CommitPrevious,
  Cancel,
}
```

Gesture contracts describe normalized gesture state and outcome. Carousel index
transitions remain owned by existing pure carousel helpers.

## Component Mapping

| Component | Contract | Mapping |
| --- | --- | --- |
| Popover | measurement | measure trigger/content/viewport, then feed pure placement helpers |
| Tooltip | measurement | same as Popover, with hover/focus policy still app-owned |
| Dropdown | measurement | measure trigger/content/viewport for menu placement |
| Resizable | pointer | emit pointer deltas, then feed `resizable_resize_pair` |
| Carousel | gesture | map horizontal swipe outcome to `carousel_next` or `carousel_previous` |
| Chart | measurement | measure plot/container later; backend remains deferred |

## Source-Copy Policy

M24 should not change default generated component output.

Runtime contracts should not be copied when a user runs:

```bash
dxui add popover
dxui add resizable
dxui add carousel
```

Future opt-in commands can be planned separately:

```bash
dxui add runtime-measurement
dxui add runtime-pointer
dxui add runtime-gestures
```

Until then, source-copy components remain controlled styled parts with app-owned
measurement, pointer, and gesture behavior.

## Platform Fallbacks

| Target | Measurement Fallback | Pointer Fallback | Gesture Fallback |
| --- | --- | --- | --- |
| Web | return `Unsupported` until DOM measurement runtime is installed | return `Unsupported` until pointer events are wired | return `Unsupported` until touch/pointer gesture runtime is wired |
| Desktop | return `Unsupported` until WebView measurement is verified | return `Unsupported` until pointer capture behavior is verified | conservative unsupported fallback |
| Mobile | prefer app-owned layout targets and visual viewport handling | touch-first pointer stream later | preserve native scroll and allow app policy to disable gestures |

Fallback behavior must be testable without a renderer.

## M24 Implementation Order

1. Extend `runtime` contracts with rectangle and measurement types.
2. Extend `runtime` contracts with pointer phase/delta types.
3. Extend `runtime` contracts with gesture state/outcome types.
4. Add Popover, Resizable, and Carousel mapping examples in pure tests.
5. Update runtime planning docs after the contract surface is proven.

## Quality Gates

Before marking M24 implementation tasks done:

```bash
cargo test -p dioxus-ui-primitives --features runtime
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
```

Generated fixture smoke is only required when registry/templates change.
