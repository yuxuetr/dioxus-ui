# Web Preview Screenshot Smoke

This document defines the M112 plan for turning the existing Web preview
screenshot procedure into a focused opt-in browser smoke command.

Status: Planned in M112.1; script implemented in M112.2; package alias and
documentation wiring implemented in M112.3.

## Problem

The repository already has:

- a rendered Web preview shell
- structural Web preview checks
- rendered component DOM verification
- runtime interaction verification
- optional mobile browser screenshot capture
- ignored screenshot artifact patterns

The Web preview screenshot procedure is still mostly documented as manual
steps. That makes it harder to repeat the same desktop and mobile viewport
checks before visual review, and it leaves screenshot artifact behavior split
between docs and browser smoke scripts.

## Decision

M112 should add a small opt-in Web screenshot smoke layer:

1. Start the existing Web preview with `dx serve`.
2. Open the preview in Playwright Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`.
3. Verify stable preview panels and representative rendered targets at desktop
   and mobile viewports.
4. Capture screenshots only when explicitly requested through an environment
   variable.
5. Validate screenshot PNG metadata when screenshots are captured.
6. Keep the command outside default and release gates until browser and
   artifact behavior are proven reliable.

The goal is repeatable preview screenshot readiness, not visual regression
testing.

## Command Shape

The focused command should be:

```bash
npm run verify:web-screenshot-smoke
```

By default, the command should run browser assertions without writing
screenshots. Screenshot capture should require an explicit opt-in:

```bash
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

For local Chrome:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

## Screenshot Artifacts

When screenshot capture is enabled, the script should write ignored PNG files
under the repository root:

```text
dioxus-ui-web-preview-desktop-*.png
dioxus-ui-web-preview-mobile-*.png
```

Each captured screenshot should be validated for:

- nonzero byte size
- PNG signature
- IHDR metadata
- width and height at least as large as the requested viewport

The script should print saved paths and metadata after successful validation.

## Required Viewports

- desktop: `1280x900`
- mobile: `390x844`

## Required Assertions

Both viewports should verify:

- page title is `dioxus-ui preview`
- `[data-preview-root="web"]` exists once
- `[data-preview-panel="form"]` exists and contains an input
- `[data-preview-panel="message"]` exists
- `[data-preview-panel="chart"]` exists and contains an SVG chart
- chart fallback table rows are present
- `[data-preview-panel="overlay-open"]` exists and contains `[role="dialog"]`
- `[data-preview-panel="interactions"]` exists and exposes
  `[data-interaction-root="runtime"]`

The smoke may reuse the stable selectors from rendered DOM and runtime
interaction verification, but it should not replace those commands.

For a serial local run of all browser-backed preview checks, use
`npm run verify:browser-local`. Do not parallelize browser preview commands;
each command starts its own `dx serve` process.

For manual release-candidate screenshot review after capture, use
[Release Screenshot Review Checklist](release-screenshot-review-checklist.md).

## Non-goals

- no screenshots by default
- no pixel-level visual diffing
- no shadcn/ui visual parity claims
- no CI workflow activation
- no Desktop WebView screenshot capture
- no native Mobile screenshot capture
- no full accessibility certification
- no component API changes
- no generated source-copy template rewrites
- no committed screenshots, traces, generated docs, or generated CSS output

## Documentation Alignment

M112 should keep these files aligned:

- `package.json`
- `README.md`
- `docs/README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/components/README.md`
- `docs/components/web-preview-screenshot-verification.md`
- `docs/components/web-preview-screenshot-smoke.md`
- `docs/components/runtime-interaction-verification.md`
- `docs/browser-artifact-policy-metadata.md`
- `.gitignore`
- `scripts/repo-hygiene-verify.mjs`

The documentation should say clearly that screenshot smoke is stronger than
selector-only browser checks, but still weaker than visual regression testing,
pixel comparison, or visual parity certification.

## Follow-up Milestones

After M112, useful follow-up work is:

1. Decide whether screenshot smoke should run in a non-blocking CI workflow.
2. Add a small screenshot review checklist for releases.
3. Revisit Desktop WebView screenshot capture after local window automation is
   repeatable.
