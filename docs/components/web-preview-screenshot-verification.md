# Web Preview Screenshot Verification

This document records the M37.4 screenshot verification gate for the rendered
Web preview shell.

Status: Verified in M37.4.

## Command

Start the preview app:

```bash
dx serve --web --package dioxus-ui-web-demo --bin preview --port 45237 --addr 127.0.0.1 --open false --hot-reload false --watch false --interactive false
```

Run the structural prerequisite gate:

```bash
node scripts/web-preview-verify.mjs
```

Then use Playwright against:

```text
http://127.0.0.1:45237
```

## Required Viewports

- desktop: `1280x900`
- mobile: `390x844`

## Required Assertions

Both viewports must verify:

- page title is `dioxus-ui preview`
- `[data-preview-root="web"]` exists
- `[data-preview-panel="form"]` exists and contains an input
- `[data-preview-panel="message"]` exists and contains the message scroller
- `[data-preview-panel="chart"]` exists and contains an SVG chart
- chart fallback table rows are present
- `[data-preview-panel="overlay-open"]` exists and contains `[role="dialog"]`

The M37.4 run saved these local screenshots:

- `dioxus-ui-web-preview-desktop.png`
- `dioxus-ui-web-preview-mobile.png`

These files are local verification artifacts and are intentionally ignored by
Git.

## Current Scope

This gate verifies the rendered shell and screenshot targets for representative
states. It does not claim pixel-perfect visual parity with shadcn/ui.

Desktop WebView screenshots remain part of M37.5.
