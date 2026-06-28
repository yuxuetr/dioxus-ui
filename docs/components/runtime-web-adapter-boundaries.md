# Web Runtime Adapter Module Boundaries

This document defines the M27 plan for experimental Web runtime adapter module
boundaries. It follows the Web verification fixture and keeps concrete adapter
work isolated until browser assertions prove the behavior.

Status: M27.1 boundaries documented. M27.2 adds fixture-local timer and
live-region feedback adapters. M27.3 adds fixture-local overlay adapters.

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

## Implemented Fixture Slices

M27.2 adds `WebTimerRuntime` and `WebLiveRegionRuntime` inside
`examples/runtime-web-verification/src/web_runtime.rs`.

The current slice verifies:

- timer scheduling returns a stable adapter-owned timer id
- timer cancellation marks the scheduled record as cancelled
- disabled timer requests still report `Disabled`
- live-region requests queue trimmed messages
- consecutive duplicate announcements can be suppressed
- empty live-region messages still report `EmptyMessage`
- unsupported fallback results remain visible beside the success paths

This implementation intentionally records adapter state in memory. It does not
call browser timeout APIs or mutate DOM live-region nodes yet. Browser-backed
behavior remains behind the future browser assertion gate.

M27.3 adds `WebFocusRuntime` and `WebPortalRuntime` in the same fixture module.
The overlay slice verifies:

- initial focus, focus trap, and focus return requests can report `Applied`
- missing focus nodes report `MissingTarget`
- inline portal requests remain inline
- body and known selector portal requests can return adapter-owned mount ids
- missing selector portal requests report `MissingTarget`
- unsupported focus and portal runtimes remain visible beside success paths

The overlay slice still records commands in memory. It does not call browser
focus APIs, create DOM portal roots, own app shell z-index policy, or prove
keyboard trap behavior until browser automation is added.

## Implementation Order

1. Timer and live-region adapters, started in M27.2
2. Portal adapter, started in M27.3
3. Non-modal focus return, started in M27.3
4. Modal focus trap, started in M27.3
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
