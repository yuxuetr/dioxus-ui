# Mobile Browser Smoke Metadata Gate

This document defines the M92 plan for keeping the opt-in mobile browser smoke
contract aligned across script, package metadata, release docs, and CI browser
guidance.

Status: Planned in M92.1.

## Problem

The project has an opt-in Playwright-backed browser smoke:

```bash
npm run verify:mobile-browser
```

The command starts the rendered Web preview, opens it through a mobile browser
viewport, checks representative selectors, optionally writes an ignored PNG
screenshot, and validates screenshot metadata. This is intentionally outside
default release behavior because it depends on installed browser binaries and
local serving behavior.

The script contract is now documented in several places. M92 should add a
read-only metadata gate so future edits keep the script, package alias, release
notes, quality gates, CI browser docs, and workflow template aligned without
launching browser automation.

## Contract

The mobile browser smoke metadata gate should verify:

- `package.json` keeps `verify:mobile-browser` wired to
  `node scripts/mobile-browser-smoke.mjs`.
- `scripts/mobile-browser-smoke.mjs` keeps the localhost target
  `127.0.0.1:45237`.
- The script keeps the mobile viewport at `390x844`.
- The script keeps the explicit Playwright install hint:
  `npx playwright install chromium`.
- The script keeps the external executable environment variable:
  `DIOXUS_UI_BROWSER_EXECUTABLE`.
- The script keeps the screenshot opt-in environment variable:
  `DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT`.
- The script and docs keep the screenshot artifact pattern:
  `dioxus-ui-mobile-browser-preview-*.png`.
- The script keeps Mobile Web profile selector assertions:
  `touch-targets`, `hover-alternative`, `safe-area-owned`, `reduced-motion`,
  and `visible-status`.
- The script keeps representative preview panel assertions for `form`,
  `message`, `chart`, and `overlay-open`.
- CI browser docs keep both Playwright-managed Chromium and external Chrome
  paths documented.

## Non-goals

- no browser launch
- no `dx serve`
- no Playwright browser installation
- no screenshot writing
- no screenshot pixel comparison
- no native Mobile or Desktop WebView runtime claim
- no activation of `.github/workflows/`

## Planned Command

M92 should add:

```bash
npm run verify:mobile-browser-metadata
```

The command should be deterministic and read-only. It should report missing
fragments with enough context to repair docs or script drift.

After it is stable, it should be included in:

```bash
npm run verify:release
```

The opt-in browser smoke itself remains separate:

```bash
npm run verify:mobile-browser
```

For a serial local run of all browser-backed preview checks, use
`npm run verify:browser-local`. The aggregate calls this mobile browser smoke
first, then runs the other browser preview checks in order. Do not parallelize
browser preview commands; each command starts its own `dx serve` process.
