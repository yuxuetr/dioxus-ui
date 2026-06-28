# Mobile Runtime Verification Checklist

This document defines the M28.2 Mobile runtime verification checklist. It keeps
Mobile runtime behavior opt-in until this project has repeatable device or
emulator tooling.

Status: Defined in M28.2.

## Decision

Mobile verification should begin as a documentation-first checklist, not a
runtime fixture. The current project has repeatable Rust, Web fixture, and
Desktop smoke commands, but it does not yet have stable Mobile device or
emulator automation.

Mobile runtime adapters must remain opt-in until the checks below can run
repeatably and expose explicit fallback states.

## Verification Modes

| Mode | Meaning | Current Use |
| --- | --- | --- |
| Manual | Human verifies behavior on a target device or browser profile. | Required for assistive behavior, safe areas, and visual polish. |
| Emulator-backed | Repeatable command drives a simulator, emulator, or device farm. | Planned for touch, viewport, reduced motion, and native scroll checks. |
| Deferred | Do not claim support yet; keep adapter or behavior opt-in. | Required for unstable tooling or platform-specific assistive behavior. |

## Checklist

| Area | Required Checks | Mode | Fallback Requirement |
| --- | --- | --- | --- |
| Touch targets | Interactive controls meet target-size expectations and dense variants remain tappable. | Manual first, emulator-backed later | Keep compact density app-owned on Mobile. |
| Safe areas | Dialog, Sheet, Drawer, Toast, Sonner, and command overlays avoid unsafe screen edges. | Manual first | Allow app shell padding or inline fallback. |
| Visual viewport | Measurement updates when browser chrome, orientation, or software keyboard changes viewport size. | Emulator-backed later | Return `Missing` or `Unsupported` until repeatable. |
| Native scroll | Carousel, Drawer, and future drag gestures preserve vertical page scroll and cancel below threshold. | Emulator-backed later | Prefer `Cancel` over stealing scroll. |
| Reduced motion | Timers, autoplay, transitions, and gesture-driven animations can be disabled or softened. | Manual first | Keep policy app-owned and default conservative. |
| Visible status text | Critical feedback has visible text, not only live-region side effects. | Manual | Do not claim screen-reader support from visual checks alone. |
| Hover absence | Tooltip, Hover Card, menus, and disclosure controls do not require hover-only access. | Manual first | Provide tap/click alternatives or document unsupported profile. |
| Focus behavior | Full-screen or app-owned focus flows are preferred where trapping is unreliable. | Manual first | Return `Unsupported` or keep controlled state intact. |
| Pointer/touch stream | Touch start, move, cancel, and lost-capture paths are deterministic. | Emulator-backed later | `Unsupported` must not mutate app state. |
| Density and keyboard | Inputs, Select, Combobox, Command, and Date Picker remain usable with software keyboard visible. | Emulator-backed later | Keep fallback layouts app-owned. |

## Candidate Harness Shape

A future Mobile harness can be either:

```text
examples/runtime-mobile-verification/
```

or a script-backed device profile attached to the Web fixture:

```bash
node scripts/runtime-mobile-verify.mjs
```

The first implementation should expose visible status lines equivalent to the
Web and Desktop fixtures:

```text
mobile-runtime-touch-target
mobile-runtime-safe-area
mobile-runtime-visual-viewport
mobile-runtime-native-scroll
mobile-runtime-reduced-motion
mobile-runtime-visible-status
```

Do not add a workspace package until the target command and runtime surface are
chosen. A documentation checklist is enough for M28.2.

## Manual Pass Template

Use this template when testing on a physical device or emulator manually:

```text
Target:
OS and version:
Browser/WebView:
Viewport/orientation:
Reduced motion setting:
Keyboard visible:

Touch targets: PASS/FAIL/DEFERRED
Safe areas: PASS/FAIL/DEFERRED
Visual viewport: PASS/FAIL/DEFERRED
Native scroll: PASS/FAIL/DEFERRED
Reduced motion: PASS/FAIL/DEFERRED
Visible status text: PASS/FAIL/DEFERRED
Hover alternatives: PASS/FAIL/DEFERRED
Focus behavior: PASS/FAIL/DEFERRED
Pointer/touch stream: PASS/FAIL/DEFERRED
Density and keyboard: PASS/FAIL/DEFERRED

Notes:
```

## Graduation Criteria

Mobile runtime support can move beyond documentation-only when:

- a repeatable command exists for at least one target profile
- touch, native scroll, visual viewport, and reduced motion checks emit visible
  status output
- unsupported or cancel fallback states are asserted
- source-copy components remain runtime-free by default
- hover-only interaction alternatives are documented for each affected component

Until then, Mobile behavior remains a target profile and app integration policy,
not a separate default component tree.

## Relationship To Other Plans

- [Desktop And Mobile Runtime Verification Strategy](runtime-desktop-mobile-verification.md)
- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Measurement, Pointer, And Gesture Adapter Plan](measurement-pointer-gesture-adapters.md)
