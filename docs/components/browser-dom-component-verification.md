# Browser DOM Component Verification

This document defines the M110 plan for moving rendered component coverage from
static metadata to a browser-backed DOM existence and visibility check.

Status: Script and package alias implemented in M110.2-M110.3.

## Problem

M109 proves that every public component has:

- a docs catalog entry
- a rendered coverage record
- a stable `data-component-preview` target id
- a matching target in the shared Web/Desktop preview source

That is still source-level evidence. It does not prove that the Dioxus Web
preview actually serves, hydrates, and exposes those targets in a browser DOM.

## Decision

M110 should add a focused, opt-in Playwright command that:

1. Starts the existing Web preview with `dx serve`.
2. Opens the preview in a headless browser.
3. Reads `docs/components/rendered-coverage.json`.
4. Checks every `data-component-preview` target from the manifest.
5. Fails on missing, detached, hidden, empty, or zero-sized targets.
6. Cleans up the preview server before exiting.

The command should reuse the operational shape of
[`scripts/mobile-browser-smoke.mjs`](../../scripts/mobile-browser-smoke.mjs):

- Playwright-managed Chromium by default.
- `DIOXUS_UI_BROWSER_EXECUTABLE` for local Chrome or CI-provided Chromium.
- actionable error when Playwright Chromium is missing.
- fixed local host and port.
- explicit server cleanup on success or failure.

## Verification Contract

For each rendered coverage record, the browser DOM verifier should validate:

- exactly one element matching `[data-component-preview="{test_id}"]`
- the element is connected to `document`
- the element is visible according to computed style and layout boxes
- `innerText` or accessible text fallback is non-empty
- `getBoundingClientRect()` reports positive width and height
- component slug, panel, and coverage level attributes match the manifest

The initial viewport should be desktop-sized, such as `1280x900`, because this
milestone is about catalog-wide target existence rather than Mobile Web
ergonomics. Mobile behavior remains covered by the existing opt-in mobile
browser smoke and future mobile-specific work.

## Non-goals

- no screenshot capture by default
- no visual diffing or pixel assertions
- no shadcn/ui visual parity claim
- no runtime interaction assertions such as click, keyboard, focus trap, or
  outside-click behavior
- no Desktop native WebView browser automation
- no component API changes
- no template rewrites
- no generated docs or route artifacts

## Command Shape

The focused command should be:

```bash
npm run verify:rendered-component-dom
```

It stays separate from:

- `npm run verify`
- `npm run verify:release`
- `npm run verify:mobile-browser`

It is documented as opt-in until browser availability is proven stable in local
and CI environments. A later milestone can decide whether it belongs in release
or CI gates.

If Playwright-managed Chromium is unavailable, run it with a local browser:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:rendered-component-dom
```

## Documentation Alignment

M110 should keep these files aligned:

- `package.json`
- `README.md`
- `docs/README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/components/README.md`
- `docs/components/rendered-component-verification.md`
- `docs/components/browser-dom-component-verification.md`
- `docs/browser-artifact-policy-metadata.md`

The documentation should be explicit that DOM verification is stronger than
M109 metadata coverage, but still weaker than visual parity or interaction
verification.

## Follow-up Milestones

After M110, useful follow-up work is:

1. Add interaction checks for runtime-sensitive components.
2. Add optional screenshot smoke for a small set of panels.
3. Add CI browser workflow integration only after local browser behavior is
   stable.

M111 plans the first interaction layer in
[Runtime Interaction Verification](runtime-interaction-verification.md). Its
focused command is `npm run verify:runtime-interactions`, and it stays opt-in
outside default and release gates.
