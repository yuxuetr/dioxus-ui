# Mobile Browser Smoke Feasibility

This document defines the M41 feasibility plan for checking the rendered Web
preview through a mobile-sized browser profile.

Status: Planned in M41.1.

## Goal

Probe whether this repository can add a repeatable browser smoke for the
existing Web preview at a mobile viewport. The smoke should sit between the M40
source-level Mobile Web profile gate and future native device or emulator
automation.

The target surface remains:

```bash
dx serve --web --package dioxus-ui-web-demo --bin preview --port 45237 --addr 127.0.0.1 --open false --hot-reload false --watch false --interactive false
```

The target URL remains:

```text
http://127.0.0.1:45237
```

## Boundary

This milestone verifies mobile browser rendering, not native Mobile support.

In scope:

- a mobile browser viewport, initially `390x844`
- visible rendered selectors from the shared Web preview
- the M40 `mobile-profile` panel
- representative form, message, chart, and overlay-open panels
- browser-level smoke assertions that the preview is nonblank and queryable

Out of scope:

- native iOS or Android simulator automation
- software keyboard behavior
- safe-area inset measurement
- mobile screen-reader or assistive technology support
- native WebView gesture arbitration
- claiming runtime Mobile adapters as stable

## Dependency Strategy

The repository currently does not include a `package.json` or committed
Playwright dependency. M41 should avoid adding npm dependencies unless the probe
shows a stable, low-churn path.

Preferred order:

1. use the locally configured browser automation to prove feasibility
2. document required server lifecycle and selector assertions
3. add a repository script only if it can be deterministic without unplanned
   dependency churn

If the only available path requires a local Codex or MCP browser tool, document
that as a manual/local gate rather than pretending it is a portable release
gate.

## Candidate Assertions

The first mobile browser smoke should verify:

- page title is `dioxus-ui preview`
- `[data-preview-root="web"]` exists
- `[data-preview-panel="mobile-profile"]` exists
- `[data-mobile-profile="touch-targets"]` exists
- `[data-mobile-profile="hover-alternative"]` exists
- `[data-mobile-profile="safe-area-owned"]` exists
- `[data-mobile-profile="reduced-motion"]` exists
- `[data-mobile-profile="visible-status"]` exists
- `[data-preview-panel="form"]` contains an input
- `[data-preview-panel="message"]` contains rendered message content
- `[data-preview-panel="chart"]` contains an SVG chart and fallback table rows
- `[data-preview-panel="overlay-open"]` contains `[role="dialog"]`

The smoke should also reject an empty page by checking that body text is present
and the chart SVG has a non-empty bounding box when browser tooling exposes
layout measurements.

## Server Lifecycle

A repeatable script should own the preview server if possible:

1. start `dx serve` on a fixed localhost port
2. wait for the preview URL to respond
3. run mobile browser assertions
4. close the browser context
5. stop the `dx serve` process

If the chosen automation tool cannot be invoked from a repository script, the
documented local procedure should require the user to start `dx serve` manually
and then run assertions through the configured browser tool.

## Artifact Policy

Screenshots are optional. If local screenshots are produced, they should be
ignored by Git with this pattern:

```text
dioxus-ui-mobile-browser-preview-*.png
```

No screenshot should be committed as a source artifact.

## Graduation Criteria

M41 can add a repeatable smoke command only when:

- the Web preview server can be started and cleaned up deterministically
- browser automation can set the mobile viewport and query selectors
- failure messages identify missing selectors or server startup failures
- the command does not require undocumented local credentials or private tools

If those criteria are not met, M41 should finish as a documented feasibility
result and keep the M40 structural gate as the supported Mobile Web profile
verification path.

## M41.2 Local Probe Result

The local probe confirmed that the Web preview server can be started and served
at the planned URL:

```bash
dx serve --web --package dioxus-ui-web-demo --bin preview --port 45237 --addr 127.0.0.1 --open false --hot-reload false --watch false --interactive false
curl -I http://127.0.0.1:45237
```

The preview returned `HTTP/1.1 200 OK`, and the server was stopped cleanly after
the probe.

Browser automation availability is mixed:

- the configured in-app browser list was empty, so no `iab` browser instance was
  available for plugin-controlled assertions
- the Node REPL tool environment could import `playwright`
- repository shell commands could not import `playwright` because this project
  has no `package.json` or local Playwright dependency
- Playwright's bundled Chromium executable was not installed in the local cache
- launching system Google Chrome through Playwright in headless mode exited with
  `SIGABRT` under the current environment

Because the only successful Playwright import is tool-environment-specific, M41
should not add a repository browser smoke script yet. The current supported
portable gate remains:

```bash
node scripts/mobile-web-profile-verify.mjs
```

M41.3 should add a script only if a deterministic dependency and browser binary
strategy is selected, such as adding explicit npm tooling or documenting a
supported external browser installation contract.

## M41.3 Script Decision

No repository browser smoke script is added in M41.3. The current environment
can prove that the Web preview server responds, but it cannot provide a
portable browser automation path:

- committed project files do not include Node package metadata or a Playwright
  dependency
- the only successful Playwright import came from the tool runtime, not the
  repository shell environment
- the available browser binary paths were not stable enough for a local script

Future experiments may save screenshots with this ignored pattern:

```text
dioxus-ui-mobile-browser-preview-*.png
```

A future script should be introduced only together with an explicit dependency
and browser installation strategy, for example a committed package manifest with
Playwright setup instructions or a documented external Chrome contract.

## M47 Screenshot Artifact Follow-through

M47 added optional screenshot capture to the repository mobile browser smoke.
The default command remains assertion-only:

```bash
npm run verify:mobile-browser
```

To save a local screenshot after assertions pass:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

To use the local external Chrome path and save a screenshot:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

The script prints the saved screenshot path. Generated screenshots match the
ignored `dioxus-ui-mobile-browser-preview-*.png` pattern and should not be
committed.

This artifact captures rendered Web preview output at a mobile browser
viewport. It does not verify native Dioxus Mobile rendering, software keyboard
behavior, or device safe-area behavior.

## M41.4 Feasibility Result

M41 completed without adding a repository browser smoke command. The milestone
validated the available portable gates:

```bash
node scripts/mobile-web-profile-verify.mjs
node scripts/web-preview-verify.mjs
scripts/example-smoke.sh
```

All commands passed. The supported release claim remains source-level Mobile Web
profile verification plus Web preview structural verification. Browser-rendered
mobile smoke is feasible only after the project selects a deterministic
Playwright dependency and browser binary strategy.

## Next Milestone Seed

The next practical milestone is dependency strategy, not more selector work.
Choose one of:

- add a committed Node package manifest and Playwright install procedure
- require an external Chrome executable and document supported versions
- keep browser smoke as a local/manual MCP procedure outside release gates

M42 starts with this decision. See
[Browser Automation Dependency Strategy](browser-automation-dependency-strategy.md).

After that decision, a future script can own server startup, mobile viewport
assertions, screenshots, and cleanup.
