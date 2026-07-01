# Desktop WebView Screenshot Plan

This document defines the M39 plan for moving from Desktop preview structural
verification to Desktop WebView screenshot capture.

Status: Local probe completed in M39.2. Desktop window capture is not
repeatable in the current environment.

## Goal

Verify the rendered Desktop preview inside a real Desktop WebView and capture
repeatable screenshots without replacing the command-line Desktop smoke gate.

## Constraints

Desktop WebView screenshots are not the same as Web screenshots:

- Playwright can verify the Web preview, but it does not control the native
  Desktop WebView window by default.
- `dx serve --platform desktop` may open an app window whose title, process
  name, and screen position are controlled by the operating system.
- macOS screenshot commands can capture windows, but they need a stable window
  selector and GUI permission.
- CI may not have a visible Desktop session.

The first milestone should prove local repeatability before adding this to
release gates.

## Candidate Local Command

The candidate command remains:

```bash
dx serve --package dioxus-ui-desktop-demo --bin preview --platform desktop
```

The screenshot script should not run this command indefinitely without cleanup.
It should:

1. start the Desktop preview
2. wait for a window with the preview title
3. capture a compact and comfortable screenshot if the platform supports it
4. close the preview process
5. report unsupported behavior without claiming a pass

## Screenshot Targets

The Desktop preview already shares the Web selectors:

```text
[data-preview-root="desktop"]
[data-preview-panel="form"]
[data-preview-panel="message"]
[data-preview-panel="chart"]
[data-preview-panel="overlay-open"]
[data-preview-panel="inventory"]
```

Native screenshot capture cannot query these selectors directly. The structural
gate must remain responsible for selector checks, while the native screenshot
gate proves the window renders a visible preview.

## Proposed Gate Shape

Use a local-only script first:

```bash
scripts/desktop-webview-screenshot-smoke.sh
```

The script should be conservative:

- require macOS window capture tooling before running capture steps
- keep screenshots ignored by Git
- fail clearly when the Desktop window cannot be found
- avoid claiming Desktop screenshot parity on unsupported platforms

## Local Probe Result

The M39.2 local probe found the expected macOS capture tools:

```text
/usr/sbin/screencapture
/usr/bin/osascript
/usr/bin/sips
```

The probe did not find a repeatable Desktop WebView window to capture.
Running the Desktop preview through `dx serve` built successfully and attempted
to launch the app, but the macOS app exited with `SIGBUS` immediately after
launch:

```text
Build completed successfully ..., launching app!
Application [macos] exited with error: signal: 10 (SIGBUS)
```

System Events did not expose a visible preview app window before the process
exited. Directly running the preview binary with Cargo also exited after
starting `/Users/hal/.target/debug/preview`, without producing a stable native
window.

Because the preview window cannot currently be selected reliably, M39 should
not add a Desktop WebView screenshot gate. The existing Desktop structural gate
remains the supported Desktop verification path until the WebView launch issue
is resolved on a local GUI session.

Documented failure modes:

- macOS capture tooling is present, but it has no stable target window.
- `dx serve --platform desktop` can complete the build while the launched app
  still fails during native WebView startup.
- sandboxed process enumeration may be unavailable, so cleanup should rely on
  the known preview process started by a future script rather than broad process
  scans.
- CI and headless sessions should continue to report Desktop screenshot capture
  as unsupported instead of failing release checks.

## Non-goals

- do not add Desktop screenshot capture to default release gates yet
- do not remove `node scripts/desktop-preview-verify.mjs`
- do not add Mobile screenshot automation in this milestone
- do not depend on visual pixel matching before window capture is reliable
