# Runtime Renderer Verification Matrix

This document defines the M25.1 verification matrix for renderer-backed runtime
adapters. It comes after the primitive-layer runtime contracts and before any
Web, Desktop, or Mobile adapter becomes default behavior.

Status: Designed in M25.1.

## Decision

Runtime adapters should not become default component behavior until each target
has an explicit verification path.

Rationale:

- primitive contracts already provide unsupported fallbacks
- Web, Desktop WebView, and Mobile differ in focus, portals, timers,
  measurement, pointer capture, and gestures
- generated source-copy components must remain controlled styled parts unless a
  user opts into runtime helpers
- a failing adapter should degrade to `Unsupported`, `Inline`, `Disabled`, or
  another explicit fallback rather than panic

## Verification Layers

| Layer | Owns | Suitable For |
| --- | --- | --- |
| Pure Rust tests | contract request/result mapping and unsupported fallback behavior | all runtime contracts |
| Browser automation | DOM focus, portal mounting, timers, live regions, measurement, pointer events, and gesture events | Web runtime adapters |
| Desktop smoke tests | WebView focus, stacking, measurement, pointer capture, timers, and live-region behavior | Desktop runtime adapters |
| Mobile verification | touch gestures, visual viewport behavior, safe areas, reduced motion, and native scroll arbitration | Mobile runtime adapters |
| Manual accessibility checks | screen reader announcement behavior, focus order, escape behavior, and modality | overlays and feedback |

Pure Rust tests remain the default gate for primitive contracts. Renderer tests
are required only when a concrete adapter is introduced.

## Contract Matrix

| Contract | Affected Components | Pure Rust Gate | Web Gate | Desktop Gate | Mobile Gate | Fallback Requirement |
| --- | --- | --- | --- | --- | --- | --- |
| Focus | Dialog, Alert Dialog, Sheet, Drawer, Popover, Select, Combobox, Date Picker, menus | request policy maps modal/non-modal behavior | initial focus, trap, restore, escape close, missing target | WebView focus return and trap behavior | full-screen or app-owned focus flows where trapping is unreliable | `Unsupported` or no-op with controlled state intact |
| Portal | Dialog, Alert Dialog, Sheet, Drawer, Popover, Toast, Sonner, menus | inline/body/selector requests are deterministic | body target, named target, inline fallback, ARIA relationships | stacking and target availability in WebView | app shell target or inline fallback | `Inline`, `MissingTarget`, or `Unsupported` |
| Timer | Toast, Sonner, Tooltip delay, Hover Card delay, Carousel autoplay | schedule/cancel/disabled results | timeout scheduling, cancellation, cleanup on unmount | WebView timer behavior while backgrounded or hidden | reduced-motion and user policy can disable timers | `Disabled` or `Unsupported` |
| Live region | Toast, Sonner, Carousel announcements, async status | priority and duplicate policy mapping | DOM live-region insertion, duplicate suppression, cleanup | assistive behavior verified before support claim | visible status text preferred for noisy updates | `EmptyMessage`, `SuppressedDuplicate`, or `Unsupported` |
| Measurement | Popover, Tooltip, Dropdown, Select, Menubar, Context Menu, Navigation Menu, Resizable, Chart | rectangle requests and missing target results | anchor/content/viewport rects, scroll and resize invalidation | WebView rect consistency and device scale behavior | visual viewport, safe areas, and orientation changes | `Missing` or `Unsupported` |
| Pointer | Resizable, future drag interactions | phase/delta request mapping | pointer down/move/up/cancel and capture release | pointer capture differences in WebView | touch-first stream and cancellation | `Unsupported` without mutating app state |
| Gesture | Carousel, future Drawer drag-to-dismiss | threshold resolution and outcome mapping | swipe next/previous/cancel without native scroll conflicts | conservative support after pointer behavior is proven | native scroll escape, velocity thresholds, safe areas | `Cancel` or `Unsupported` |

## Target-Specific Acceptance

### Web

Web adapters may graduate from experimental only after browser automation
proves:

- focus enters, traps, and restores for modal overlays
- portal targets mount and unmount without breaking labels or descriptions
- timers can be scheduled, cancelled, and disabled
- live-region announcements can be queued without duplicate noise
- measurement returns stable rectangles after layout, scroll, and resize
- pointer and gesture events can be cancelled without stealing native scroll

### Desktop

Desktop adapters should stay conservative until WebView smoke tests prove:

- focus behavior matches the intended overlay modality
- body and named portal targets stack correctly in the app shell
- measurement is stable across window resize and device scale changes
- pointer capture and cancellation are reliable
- timers behave predictably when windows are hidden or backgrounded

### Mobile

Mobile adapters should be opt-in until verification covers:

- touch target sizing and safe-area layout
- visual viewport changes from browser chrome or keyboard display
- gesture thresholds that preserve native scrolling
- reduced-motion and longer-duration notification policy
- explicit visible status text for important updates

## Source-Copy Policy

M25 does not change generated component output.

Runtime helpers can be planned as opt-in commands later:

```bash
dxui add runtime-web
dxui add runtime-desktop
dxui add runtime-mobile
```

Generated components should continue to compile without runtime helpers and
without imports from `dioxus-ui-primitives`.

## Implementation Order

1. Define this renderer verification matrix.
2. Define the Web verification harness and browser assertions.
3. Define Desktop and Mobile verification strategy.
4. Seed concrete runtime implementation milestones after verification plans are
   accepted.

For the Web harness shape, see the
[Web runtime verification harness](runtime-web-verification-harness.md).
For Desktop and Mobile target strategy, see the
[Desktop and Mobile runtime verification strategy](runtime-desktop-mobile-verification.md).
For the implementation order after verification planning, see the
[runtime implementation milestone seeds](runtime-implementation-milestones.md).

## Quality Gates

Before a renderer adapter is marked stable:

```bash
cargo test -p dioxus-ui-primitives --features runtime
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
```

Adapter-specific browser or desktop commands should be added only when the
corresponding harness exists.
