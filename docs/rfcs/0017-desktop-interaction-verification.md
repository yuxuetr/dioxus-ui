# RFC 0017: Desktop Interaction Verification

- Status: Accepted
- Created: 2026-10-04

## Summary

Verify interaction behavior in the Desktop WebView with an in-app self-test.
When an environment variable is set, the Desktop preview:

1. runs a scenario script in its own WebView through `document::eval`;
2. receives the results through the same channel;
3. prints a summary;
4. exits with a status code.

`npm run verify:desktop-interactions` builds and runs the self-test.

## Current State

As of M141:

- Every interaction behavior (RFCs 0010 through 0016) runs as a page script
  through `document::eval` or as a Rust event handler. Web, Desktop, and
  Mobile share that code.
- `npm run verify:runtime-interactions` drives the Web preview in Chromium
  with Playwright.
- The Desktop preview (`examples/desktop-demo`, binary `preview`) renders the
  same `PreviewSurface` interaction fixtures, but nothing exercises them.
- Desktop on macOS uses WKWebView. WKWebView has no WebDriver endpoint, so
  Playwright cannot attach to it.
- A probe in M142.1 ran a Dropdown scenario inside the Desktop preview. Focus
  entry, ArrowDown, Escape, and focus return all worked through
  `document::eval`, and placement applied `position: fixed`.

## Decision

### Self-Test Runner

- The Desktop `preview` binary checks `DIOXUS_UI_DESKTOP_SELF_TEST`. When it
  is set, the app mounts a `SelfTest` component next to the preview surface.
- `SelfTest` runs `examples/desktop-demo/self-test/interactions.js` once with
  `document::eval` and waits for a single JSON result:
  `{ ok, passed, error }`.
- It prints `desktop interaction verification passed (N scenarios)`, or the
  failing scenario and error. It then exits with status 0 or 1.
- The scenario script has its own overall timeout. It reports a failure
  instead of hanging.
- The production preview without the variable is unchanged.

### Scenarios

The script drives the shared fixtures with `element.focus()`, `click()`, and
dispatched `keydown` and `pointerdown` or `pointermove` events. It waits for
DOM state with polling. There is one scenario per interaction path:

| Scenario | Path exercised |
| --- | --- |
| Dialog | modal focus scope: focus entry, Escape, focus return |
| Popover | anchored overlay: fixed placement, outside press dismissal |
| Select | listbox: ArrowDown on the trigger (Rust handler), arrow highlight, Enter selection |
| Dropdown | menu mode: focus entry, wrapping arrows, activation, focus return |
| Toast | dismiss timer: countdown dismissal reason |
| Date Picker | calendar focus following through `MountedData::set_focus` after a key move |
| Menubar | menubar script: trigger roving, Left and Right menu switching, focus return |
| Navigation Menu | navigation script: click toggle, ArrowDown content entry, Escape focus return |

Dispatched events are untrusted, so browsers do not run their default
actions. Tab does not move focus, and Enter on a button does not click it.
The scenarios therefore cover script listeners and Rust handlers, which are
the code paths that differ by renderer. Default actions are browser behavior,
and the Web browser smoke already covers them.

### Command

`npm run verify:desktop-interactions` runs
`cargo run -p dioxus-ui-desktop-demo --bin preview` with the variable set. It
applies a timeout and passes the exit status through. The command opens a
window and needs a GUI session. Like the browser smoke, it stays out of
`npm run verify` and `npm run verify:release`.

## Scope

In scope:

- the Desktop self-test component, scenario script, and npm command
- reverse checks that broken interaction paths fail the self-test

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Mobile | No simulator or device automation exists in the workspace | iOS Simulator or Android emulator tooling is added to the project |
| Native default actions on Desktop (Tab movement, Enter activating buttons) | Synthetic events cannot trigger them; they are browser behavior covered on Web | A Desktop-only default-action difference is reported |
| CI activation | Needs a macOS runner with a GUI session; browser smoke is also local-only | The browser smoke moves into CI (RFC 0009) |
| Linux and Windows WebViews | Only macOS WKWebView is available locally | A contributor or CI runner can run the self-test on WebKitGTK or WebView2 |
| Pixel or screenshot checks | Screenshot policy keeps artifacts out of the repository | Screenshot artifacts are retained (M115) |

## Verification

- `npm run verify:desktop-interactions` passes on macOS.
- Reverse checks break one interaction path at a time and confirm the
  self-test exits with status 1:
  - focus return
  - listbox selection
  - menubar switching
  - calendar focus following
- A reverse check that blocks the scenario confirms the overall timeout
  reports a failure.
