# Browser Smoke Aggregate

This document defines the M113 plan for adding a serial local aggregate for
browser-backed Web preview verification commands.

Status: Planned in M113.1; package alias implemented in M113.2; documentation
and verifier wiring implemented in M113.3.

## Problem

The repository now has several opt-in browser-backed commands:

- `npm run verify:mobile-browser`
- `npm run verify:rendered-component-dom`
- `npm run verify:web-screenshot-smoke`
- `npm run verify:runtime-interactions`

Each command starts `dx serve` for the Web preview and owns its own cleanup.
They are stable when run one at a time, but running multiple Web preview
browser commands in parallel can make the served preview expose duplicate roots
or duplicate component targets. That produces false failures such as duplicate
`data-component-preview` targets or multiple `data-interaction-root` nodes.

The next step is a convenience aggregate that documents and enforces serial
execution for local browser smoke validation.

## Decision

M113 should add a focused local aggregate:

```bash
npm run verify:browser-local
```

The aggregate should run these commands sequentially:

```bash
npm run verify:mobile-browser
npm run verify:rendered-component-dom
npm run verify:web-screenshot-smoke
npm run verify:runtime-interactions
```

It should preserve each command's existing behavior:

- Playwright-managed Chromium by default
- `DIOXUS_UI_BROWSER_EXECUTABLE` for local Chrome or CI-provided Chromium
- screenshots disabled by default
- optional screenshot capture through existing command-specific environment
  variables
- local preview server cleanup after each command
- no release gate or CI workflow promotion

## Serial Execution

The aggregate must use shell `&&` chaining or an equivalent ordered execution
strategy. It must not run the browser commands in parallel.

The commands share the same Web preview package and each command starts a local
dev server. Running them in parallel can cause stale or duplicate rendered
roots to appear in the browser DOM. The aggregate exists to make the intended
local sequence explicit.

## Screenshot Behavior

The aggregate should not enable screenshots itself.

Users can opt into existing screenshot paths:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:browser-local
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:browser-local
```

Both variables may be combined when local screenshot artifacts are wanted for a
manual review pass. Generated PNG files stay ignored by Git and must not be
committed.

For manual release-candidate screenshot review after capture, use
[Release Screenshot Review Checklist](release-screenshot-review-checklist.md).

## Non-goals

- no CI workflow activation
- no browser smoke promotion into `npm run verify`
- no browser smoke promotion into `npm run verify:release`
- no screenshot capture by default
- no pixel-level visual diffing
- no shadcn/ui visual parity claims
- no Desktop WebView screenshot capture
- no native Mobile automation
- no component API changes
- no generated source-copy template rewrites
- no committed screenshots, traces, generated docs, or generated CSS output

## Documentation Alignment

M113 should keep these files aligned:

- `package.json`
- `README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/components/README.md`
- `docs/components/browser-smoke-aggregate.md`
- `docs/components/browser-dom-component-verification.md`
- `docs/components/runtime-interaction-verification.md`
- `docs/components/web-preview-screenshot-smoke.md`
- `docs/components/mobile-browser-smoke-metadata.md`
- `docs/browser-artifact-policy-metadata.md`

The documentation should say clearly that the aggregate is stronger than any
single browser command, but still local, opt-in, serial, and outside default
release gates.

## Follow-up Milestones

After M113, useful follow-up work is:

1. Add a manual screenshot review checklist for release candidates.
2. Decide whether a non-blocking CI browser workflow should run the aggregate.
3. Revisit Desktop WebView screenshot capture after window automation is
   repeatable.
