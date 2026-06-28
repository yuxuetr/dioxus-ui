# Web Runtime Adapter Module Boundaries

This document defines the M27.1 plan for experimental Web runtime adapter
module boundaries. It follows the Web verification fixture and comes before any
concrete Web adapter implementation.

Status: Planned in M27.1.

## Decision

Experimental Web runtime adapters should start inside the dedicated verification
fixture, then graduate into an opt-in runtime package only after browser
assertions prove success and fallback behavior.

Initial location:

```text
examples/runtime-web-verification/src/
```

Future opt-in crate location, if implementation pressure justifies it:

```text
crates/dioxus-ui-runtime-web/
```

Do not add Web runtime adapters to `dioxus-ui` default features, generated
templates, or styled component modules in M27.

## Rationale

- the fixture already depends on `dioxus-ui-primitives/runtime`
- browser behavior needs verification before it becomes a reusable package
- generated source-copy components must stay runtime-free by default
- app-owned policies such as copy, z-index, lifecycle, and gesture thresholds
  should remain outside styled components
- a separate crate should wait until adapter code introduces Web-only
  dependencies or public API pressure

## Enabling Model

M27 adapters should be explicit and opt-in.

Recommended fixture-only import shape:

```rust
mod web_runtime;

use web_runtime::{
  WebFocusRuntime,
  WebLiveRegionRuntime,
  WebPortalRuntime,
  WebTimerRuntime,
};
```

Future crate-mode opt-in can be planned as:

```toml
dioxus-ui-runtime-web = { version = "0.1", features = ["focus", "portal"] }
```

Generated source-copy opt-in can be planned separately:

```bash
dxui add runtime-web
```

None of these should be wired into `dxui add dialog`, `dxui add toast`, or other
component commands by default.

## Adapter Families

| Family | First Boundary | Owns | Does Not Own |
| --- | --- | --- | --- |
| Timer | `WebTimerRuntime` | browser timeout scheduling and cancellation | Toast/Sonner queue mutation |
| Live region | `WebLiveRegionRuntime` | DOM live-region node updates and cleanup | announcement wording or localization |
| Portal | `WebPortalRuntime` | resolving body, inline, and named targets | app shell z-index policy |
| Focus | `WebFocusRuntime` | initial focus, focus return, modal trap after browser proof | overlay open state or validation policy |
| Measurement | `WebMeasurementRuntime` | DOM rect and viewport measurement | placement collision math |
| Pointer | `WebPointerRuntime` | pointer event normalization and capture cleanup | Resizable panel mutation |
| Gesture | `WebGestureRuntime` | browser gesture stream normalization | carousel physics or native-scroll policy |

## Implementation Order

1. Timer and live-region adapters
2. Portal adapter
3. Non-modal focus return
4. Modal focus trap
5. Measurement adapter
6. Pointer adapter
7. Gesture adapter

This order intentionally delays gesture-heavy work until lower-level pointer
behavior is verified.

## Source-Copy Policy

Source-copy templates must remain self-contained. Runtime adapter code may be
added only through an explicit command in a later milestone.

Generated component templates must not import:

- `dioxus-ui-primitives`
- `dioxus-ui-runtime-web`
- browser-only adapter modules

The verification fixture may import `dioxus-ui-primitives/runtime` because it
is a workspace example, not generated user code.

## Quality Gates

Before any M27 adapter implementation is marked done:

```bash
cargo test -p dioxus-ui-runtime-web-verification
cargo run -p dioxus-ui-runtime-web-verification
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
```

Browser automation remains required before stable runtime behavior is claimed.

## Relationship To Other Plans

- [Web Runtime Verification Harness](runtime-web-verification-harness.md)
- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Runtime Implementation Milestone Seeds](runtime-implementation-milestones.md)
