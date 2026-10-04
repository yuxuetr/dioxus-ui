# RFC 0018: Mobile Interaction Verification

- Status: Accepted
- Created: 2026-10-04

## Summary

Run the RFC 0017 interaction self-test in an iOS Simulator build of a new
Mobile preview app. Desktop and Mobile share one scenario script and one
self-test component. `npm run verify:mobile-interactions` does the following:

1. builds the app for the simulator;
2. boots an iPhone simulator when none is running;
3. installs and launches the app with the self-test variable;
4. reads the result from the app's console.

## Current State

As of M142:

- Interaction behavior runs through `document::eval` and Rust handlers shared
  by Web, Desktop, and Mobile.
- Web has the Playwright browser smoke. Desktop has the in-app self-test
  (RFC 0017).
- The workspace has no Mobile app. Mobile verification is a documentation
  checklist (M28.2) plus a Web mobile-viewport profile.
- A probe in M143.1 built a minimal Mobile app with `dx build --ios`. The app
  rendered `PreviewSurface` and ran the Desktop scenario script. All eight
  scenarios passed in an iPhone 17 simulator on iOS 26.3. `simctl launch
  --console-pty` captured the printed result.
- The Android SDK, NDK, and an emulator image exist locally. The Rust Android
  targets are not installed, and installing them changes the toolchain and
  needs network access.

## Decision

### Shared Self-Test

- `dioxus-ui-preview-states` owns the scenario script, renamed
  `self-test/interactions.js` and exported as `INTERACTION_SELF_TEST_SCRIPT`,
  and the `InteractionSelfTest` component.
- The component takes a renderer label. It prints
  `<label> interaction verification passed (N scenarios: ...)`, or the
  failing scenario and error, and exits with status 0 or 1.
- The Desktop preview uses the shared component with label `desktop` and
  keeps `DIOXUS_UI_DESKTOP_SELF_TEST`.
- `PreviewTarget` gains `Mobile`. The preview root is marked `mobile`.

### Mobile Preview App

`examples/mobile-demo` is a workspace package with the Dioxus `mobile`
feature. It renders `PreviewSurface` with `PreviewTarget::Mobile`. When
`DIOXUS_UI_MOBILE_SELF_TEST` is set, it mounts `InteractionSelfTest` with
label `mobile`. It is a fixture app, not a published crate.

### Command

`npm run verify:mobile-interactions` runs `scripts/mobile-interactions-verify.mjs`:

1. It uses `DEVELOPER_DIR` when set. Otherwise it uses
   `/Applications/Xcode.app/Contents/Developer` when `xcode-select` points at
   the Command Line Tools, because `simctl` needs full Xcode. It changes no
   system setting.
2. It runs `dx build --ios --package dioxus-ui-mobile-demo`, which targets the
   simulator.
3. It picks a booted iPhone simulator, or the newest available iPhone, and
   boots it. If the command booted it, it shuts it down at the end.
4. It runs `simctl install`, then `simctl launch --console-pty` with
   `SIMCTL_CHILD_DIOXUS_UI_MOBILE_SELF_TEST=1`.
5. It passes only when the console prints the `mobile interaction
   verification passed` line. `simctl` does not report the app's exit status,
   so a missing line, a failure line, or the launch timeout fails the command.

The command needs Xcode and a simulator runtime, so it stays out of
`npm run verify` and `npm run verify:release`.

The scenarios drive the page with dispatched events (RFC 0017). They exercise
interaction scripts and Rust handlers in the iOS WKWebView. They do not
exercise touch gestures or native default actions.

## Scope

In scope:

- the shared self-test, the Mobile preview app, and the npm command
- reverse checks that broken interaction paths and a missing result fail the
  command

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Android | Rust Android targets are not installed; adding them changes the toolchain | `aarch64-linux-android` is installed and `dx build --android` works locally (done in M145, see RFC 0020) |
| Physical devices | Needs signing and a provisioning profile | A release owner provides device signing |
| Touch gestures, software keyboard, safe areas | Dispatched events do not model touch input or viewport changes; the M28.2 checklist stays manual | A touch automation tool is added |
| CI activation | Needs a macOS runner with Xcode simulators | Browser and Desktop checks move into CI (RFC 0009) |
| Screen reader announcements | Not observable from page scripts | VoiceOver automation is available |

## Verification

- `npm run verify:mobile-interactions` passes on macOS with Xcode.
- Reverse checks confirm the command fails when:
  - an interaction path is broken (focus return, menubar switching);
  - the app reports no result line.
