# Release Screenshot Review Checklist

This document defines the M114 plan for a manual release-candidate screenshot
review workflow built on the existing opt-in browser smoke commands.

Status: Planned in M114.1.

## Problem

The project now has browser-backed preview checks and optional screenshot
capture:

- `npm run verify:browser-local`
- `npm run verify:web-screenshot-smoke`
- `npm run verify:mobile-browser`
- `DIOXUS_UI_WEB_SCREENSHOT=1`
- `DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1`

Those commands prove selector readiness, viewport behavior, PNG metadata, and
representative interaction behavior. They do not tell a maintainer what to look
for when visually reviewing release-candidate screenshots.

The next step is a written manual checklist that makes screenshot review
repeatable without introducing pixel diffing, visual baselines, or CI
promotion.

## Decision

M114 should add a manual release screenshot review checklist that:

1. Starts from the existing serial browser smoke aggregate.
2. Captures Web desktop, Web mobile, and mobile browser screenshots only when
   explicitly requested.
3. Records screenshot paths and metadata from command output.
4. Guides maintainers through the component panels that need human visual
   review.
5. Requires generated screenshots to be deleted or left ignored after review.
6. Stays outside default and release gates.

## Review Inputs

The checklist should use these commands:

```bash
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

For local Chrome:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

The reviewer may run the serial aggregate before screenshot capture:

```bash
npm run verify:browser-local
```

## Review Areas

The checklist should cover:

- overview and action panels
- form controls, Input Group, Input OTP, labels, invalid and disabled states
- message composition, attachments, markers, and scroller status
- chart title, legend, SVG plot, fallback table, and empty/invalid fallbacks
- overlay-open panel and visible dialog structure
- runtime interaction fixtures
- mobile profile panel, touch targets, hover alternatives, safe-area ownership,
  reduced-motion note, and visible status text
- component inventory density, wrapping, and overflow
- typography, spacing, focus rings, borders, contrast, and responsive layout

## Artifact Policy

Generated screenshots are local review artifacts. They are ignored by Git and
must not be committed:

```text
dioxus-ui-web-preview-*.png
dioxus-ui-mobile-browser-preview-*.png
```

The checklist should end with:

```bash
npm run verify:repo-hygiene
git status --short
```

## Non-goals

- no pixel-level visual diffing
- no automated visual baselines
- no screenshot capture by default
- no CI workflow activation
- no promotion into `npm run verify`
- no promotion into `npm run verify:release`
- no Desktop WebView screenshot capture
- no native Mobile automation
- no full accessibility certification
- no component API changes
- no generated source-copy template rewrites
- no committed screenshots, traces, generated docs, or generated CSS output

## Documentation Alignment

M114 should keep these files aligned:

- `README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/components/README.md`
- `docs/components/release-screenshot-review-checklist.md`
- `docs/components/web-preview-screenshot-smoke.md`
- `docs/components/browser-smoke-aggregate.md`
- `docs/browser-artifact-policy-metadata.md`

The documentation should say clearly that this is a human release review aid,
not an automated pass/fail visual regression system.

## Follow-up Milestones

After M114, useful follow-up work is:

1. Decide whether release candidates should attach screenshot artifacts to
   GitHub releases or internal review notes.
2. Decide whether a non-blocking CI browser workflow should generate review
   screenshots.
3. Revisit Desktop WebView screenshot capture when native window capture is
   repeatable.
