# Desktop And Mobile Runtime Verification Strategy

This document defines the M25.3 strategy for Desktop and Mobile runtime
adapter verification. It complements the Web harness plan and keeps
renderer-specific behavior out of default components until each target is
verified.

Status: Desktop strategy planned in M25.3. M28.1 adds the Desktop runtime smoke
fixture scaffold. M28.2 defines the Mobile runtime verification checklist.
M142 adds the Desktop interaction self-test. M143 adds the iOS Simulator
interaction self-test. M145 adds the Android emulator interaction self-test.

## Decision

Desktop should get a focused WebView smoke fixture before runtime adapters are
enabled by default. Mobile should remain documentation-first until stable
project tooling exists for touch, viewport, and assistive verification.

Rationale:

- Desktop WebView behavior can differ from browser behavior for focus, portal
  stacking, pointer capture, timers, and device scale
- Mobile gesture behavior must preserve native scrolling and account for safe
  areas, visual viewport changes, reduced motion, and hover absence
- the current workspace has Web and Desktop examples, but no dedicated runtime
  verification fixtures
- unsupported fallbacks are safer than claiming runtime support without target
  evidence

## Desktop Verification

Planned location:

```text
examples/runtime-desktop-verification/
```

M28.1 adds the fixture as a workspace package:

```bash
cargo test -p dioxus-ui-runtime-desktop-verification
cargo run -p dioxus-ui-runtime-desktop-verification
```

The first fixture is a compile-checked smoke scaffold. It exposes stable
Desktop-oriented status output for focus, portal stacking, timers, visible live
status, measurement, pointer capture, and conservative gesture checks. It does
not start a real WebView window or claim renderer support.

The Desktop fixture should reuse the same runtime panels as the Web harness but
verify WebView-specific behavior:

| Runtime Family | Desktop Smoke Checks |
| --- | --- |
| Focus | Initial focus, modal trap, focus return after close, escape behavior, and missing target fallback. |
| Portal | Inline, body, and named target stacking inside the app shell; missing target fallback. |
| Timer | Schedule, cancel, disabled timer, cleanup on close, and behavior while the window is hidden if tooling supports it. |
| Live region | Visible status mirror and manual assistive check notes; do not claim screen-reader support from WebView smoke alone. |
| Measurement | Anchor, overlay, viewport, scroll, resize, and device scale consistency. |
| Pointer | Pointer down/move/up/cancel, capture release, and lost-capture behavior where WebView exposes it. |
| Gesture | Conservative carousel swipe checks only after pointer behavior is proven. |

Desktop support should graduate one runtime family at a time. Focus and portal
can be verified before gestures; gestures should wait until pointer capture is
stable.

### Desktop Interaction Self-Test

M142 adds an automated Desktop interaction check
([RFC 0017](../rfcs/0017-desktop-interaction-verification.md)):

```bash
npm run verify:desktop-interactions
```

The command builds the Desktop preview (`dioxus-ui-desktop-demo`, binary
`preview`) and runs it with `DIOXUS_UI_DESKTOP_SELF_TEST=1`. The app runs
`examples/desktop-demo/self-test/interactions.js` in its own WebView, prints
`desktop interaction verification passed (9 scenarios: ...)` or the failing
scenario, and exits with that status. WKWebView has no WebDriver endpoint, so
the scenarios drive the shared interaction fixtures from inside the page:

| Scenario | Path exercised |
| --- | --- |
| `stylesheet` | the compiled preview stylesheet linked by `PreviewSurface` applies (M174, [RFC 0049](../rfcs/0049-compiled-preview-stylesheet.md)) |
| `dialog` | modal focus scope: focus entry, Escape, focus return |
| `popover` | anchored overlay: fixed placement next to the trigger, outside press |
| `select` | Rust ArrowDown handler, listbox highlight, Enter selection |
| `dropdown` | menu mode: focus entry, wrapping, disabled skip, activation, focus return |
| `toast` | dismiss timer countdown and timeout reason |
| `date-picker` | calendar focus following through `MountedData::set_focus`, across a keyed month change |
| `menubar` | trigger roving, menu switching past a disabled trigger, focus return |
| `navigation-menu` | click toggle, ArrowDown content entry, Escape focus return |

Dispatched events are untrusted, so Tab movement and Enter clicking a button
are not exercised on Desktop; the Web browser smoke covers those browser
defaults. The command opens a window and needs a GUI session, so it is not
part of `npm run verify` or `npm run verify:release`. Do not move the mouse
over the window while it runs: real pointer movement reaches the hover-driven
scripts.

Reverse checks on macOS each made the command exit with status 1. Each one
removed one behavior and was then restored:

- dialog focus return
- listbox selection reporting
- menubar switching inside a menu
- the calendar `set_focus` call
- the `PreviewSurface` stylesheet link, which fails the `stylesheet` scenario

A 300 ms overall timeout also made it report `timed out` instead of hanging.

## Mobile Verification

### Mobile Interaction Self-Test

M143 runs the same interaction scenarios in the iOS Simulator
([RFC 0018](../rfcs/0018-mobile-interaction-verification.md)):

```bash
npm run verify:mobile-interactions
```

The command builds `examples/mobile-demo` (`dioxus-ui-mobile-demo`) with
`dx build --ios`. It uses a booted iPhone simulator, or boots the newest
iPhone on iOS 26 or older and shuts it down afterwards. It then installs the
app and launches it with `SIMCTL_CHILD_DIOXUS_UI_MOBILE_SELF_TEST=1`.

The app runs the shared scenario script from `examples/preview-states` (the
same nine scenarios as Desktop). The command passes only when the console
prints `mobile interaction verification passed`, because `simctl` does not
report the app's exit status.

The command uses `DEVELOPER_DIR` when set. When `xcode-select` points at the
Command Line Tools, it uses `/Applications/Xcode.app` without changing any
system setting. Set `DIOXUS_UI_IOS_SIMULATOR` to a device name or UDID to
choose the device.

Dioxus 0.7 apps do not adopt the UIScene lifecycle, and iOS 27 stops them at
launch (`UIApplicationEvaluateRuntimeIssueForNoSceneLifecycleAdoption`). This
affects every Dioxus 0.7 iOS app, not only dioxus-ui. To recheck after a
Dioxus upgrade:

```bash
DIOXUS_UI_IOS_SIMULATOR="iPhone 18 Pro" npm run verify:mobile-interactions
```

The command currently fails on iOS 27 with no result line. macOS also shows
a crash report for `dioxus-ui-mobile-demo` (`EXC_BREAKPOINT` in
`UIApplicationEvaluateRuntimeIssueForNoSceneLifecycleAdoption`), which is
expected until Dioxus adopts the UIScene lifecycle.

Reverse checks on an iOS 26.3 simulator each made the command exit with
status 1:

- removing dialog focus return
- removing menubar switching inside a menu
- running on iOS 27, where the app reports no result

### Android Interaction Self-Test

M145 runs the same scenarios in an Android emulator
([RFC 0020](../rfcs/0020-android-interaction-verification.md)):

```bash
npm run verify:android-interactions
```

The command builds `examples/mobile-demo` with
`dx build --android --target aarch64-linux-android`. It uses a running
emulator, or boots the first AVD headless and stops it afterwards.
`DIOXUS_UI_ANDROID_SERIAL` and `DIOXUS_UI_ANDROID_AVD` choose the device.

`am start` passes no environment, so the command sets the
`debug.dioxus_ui.self_test` system property, which the app reads with
`getprop`, and clears it after the run. The result line comes from logcat
(`RustStdoutStderr`).

The SDK comes from `ANDROID_HOME`, the NDK from `ANDROID_NDK_HOME` or the
newest one in the SDK, and `JAVA_HOME` from the Android Studio JDK when unset.
Before building, the command reports an NDK whose symbolic links were stored
as small text files, which makes linking fail with `clang-17: command not
found`; reinstall the NDK with the Android SDK Manager.

The command passed on an API 36.1 arm64 emulator in about 80 seconds with an
existing build. Reverse checks each made it exit with status 1:

- removing dialog focus return
- removing menubar switching inside a menu
- an app that never reads the request, which reports no result
- the local NDK install with flattened symbolic links

After the result line, logcat can show `FORTIFY: pthread_mutex_lock called on
a destroyed mutex` while WebView threads stop during `process::exit`; it does
not affect the result.

The scenarios use dispatched events, so touch gestures, the software
keyboard, safe areas, and the visual viewport stay on the manual checklist
below.

Mobile should be treated as a target profile with stricter interaction rules,
not as a separate component tree.

Mobile verification must cover:

| Area | Required Checks |
| --- | --- |
| Touch targets | Controls meet touch density expectations and do not rely on hover-only affordances. |
| Safe areas | Overlays, sheets, drawers, and notification viewports avoid unsafe screen edges. |
| Visual viewport | Measurement adapts when browser chrome or software keyboard changes viewport size. |
| Native scroll | Carousel and drawer gestures preserve vertical scrolling and cancel below threshold. |
| Reduced motion | Timer, autoplay, and gesture defaults can be disabled or softened by app policy. |
| Live status | Critical announcements have visible text, not only live-region side effects. |

Until stable mobile automation exists in this project, Mobile runtime adapters
should remain opt-in or documentation-only. Source-copy components should
continue to expose controlled parts that apps can wire to platform-specific
behavior.

M28.2 defines the detailed Mobile checklist in
[Mobile Runtime Verification Checklist](runtime-mobile-verification-checklist.md).
The chosen M28.2 path is documentation-first: no Mobile workspace fixture is
added until an emulator or device command is selected.

## Documentation-Only Targets

These areas should stay documentation-only until tooling or target behavior is
proven:

- mobile screen-reader live-region reliability
- mobile visual viewport measurement across browser chrome and keyboard states
- Desktop background timer behavior across operating systems
- Desktop WebView pointer capture quirks
- gesture physics beyond deterministic next, previous, and cancel outcomes
- chart rendering backend behavior on high-density Desktop and Mobile displays

## Candidate Commands

Future Desktop implementation can add commands similar to:

```bash
cargo run -p dioxus-ui-runtime-desktop-verification
```

Mobile verification may begin as a manual checklist attached to the Web fixture
or as a dedicated target fixture after Dioxus Mobile tooling is selected.

## Graduation Criteria

Desktop runtime support can move from planning to implementation when:

- a Desktop smoke fixture exists
- focus, portal, timer, measurement, and pointer checks can be run locally
- unsupported fallback states are visible
- live-region support is documented as manual until assistive behavior is
  verified

Mobile runtime support can move from documentation-only to implementation when:

- target device or emulator commands are defined
- touch, safe-area, visual viewport, and native scroll checks are repeatable
- unsupported or cancel fallbacks are visible
- hover-only components have explicit mobile alternatives

## Relationship To Other Plans

- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Web Runtime Verification Harness](runtime-web-verification-harness.md)
- [Mobile Runtime Verification Checklist](runtime-mobile-verification-checklist.md)
- [Runtime Adapter Plan](runtime-adapters.md)
