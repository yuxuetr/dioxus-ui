# Release Screenshot Review Checklist

This document defines the M114 manual release-candidate screenshot review
workflow built on the existing opt-in browser smoke commands.

Status: Planned in M114.1; checklist added in M114.2.

For the screenshot retention decision that governs this checklist, see
[`screenshot-artifact-retention.md`](screenshot-artifact-retention.md).
For copyable review notes, use
[`release-screenshot-review-notes-template.md`](release-screenshot-review-notes-template.md).

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

M114 adds a manual release screenshot review checklist that:

1. Starts from the existing serial browser smoke aggregate.
2. Captures Web desktop, Web mobile, and mobile browser screenshots only when
   explicitly requested.
3. Records screenshot paths and metadata from command output.
4. Guides maintainers through the component panels that need human visual
   review.
5. Requires generated screenshots to be deleted or left ignored after review.
6. Stays outside default and release gates.

## Review Inputs

Use these commands to create review inputs:

```bash
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

For local Chrome:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

Run the serial aggregate before screenshot capture:

```bash
npm run verify:browser-local
```

## Checklist

Before capture:

- Confirm the release candidate branch is clean enough for review with
  `git status --short`.
- Run `npm run verify:browser-local` serially. Do not run browser preview
  commands in parallel.
- Confirm the aggregate reports mobile browser smoke, rendered component DOM,
  Web screenshot smoke, and runtime interaction verification as passed.
- Decide whether to use Playwright-managed Chromium or
  `DIOXUS_UI_BROWSER_EXECUTABLE`.
- Keep screenshot capture disabled until the selector and interaction checks
  pass.

Capture review screenshots:

- Run `DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke`.
- Run `DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser`.
- When using local Chrome, include the same `DIOXUS_UI_BROWSER_EXECUTABLE`
  value for both commands.
- Record the printed screenshot paths.
- Record the printed PNG metadata, including viewport name, width, height, and
  byte size.

Review the Web desktop screenshot:

- Confirm the overview and action panels are visible without layout overlap.
- Confirm form controls, Input Group, Input OTP, labels, invalid state, and
  disabled state are readable.
- Confirm message composition, attachments, markers, and scroller status are
  visually distinct.
- Confirm the chart title, legend, SVG plot, fallback table, and fallback states
  are visible.
- Confirm the overlay-open panel shows the dialog structure without covering
  unrelated review content.
- Confirm runtime interaction fixtures are present and their labels fit.
- Confirm component inventory entries wrap cleanly without clipping.
- Confirm typography, spacing, borders, focus rings, and contrast look
  consistent with the current design direction.

Review the Web mobile screenshot:

- Confirm the preview fits the `390x844` viewport without horizontal scrolling.
- Confirm touch targets have enough visual separation.
- Confirm long labels wrap instead of clipping.
- Confirm forms, messages, charts, overlays, and interaction fixtures stay
  readable in the narrow viewport.
- Confirm chart fallback rows remain accessible below the chart.
- Confirm dense inventory content remains scannable.
- Confirm no sticky, overlay, or dialog content hides unrelated panels.

Review the mobile browser screenshot:

- Confirm the mobile profile panel is visible.
- Confirm touch-target, hover-alternative, safe-area-owned, reduced-motion, and
  visible-status markers are readable.
- Confirm the mobile browser viewport does not introduce unexpected clipping.
- Confirm mobile screenshot metadata is at least the documented `390x844`
  viewport lower bound.

Record review notes:

Use the copyable template in
[`release-screenshot-review-notes-template.md`](release-screenshot-review-notes-template.md)
and fill in release candidate metadata, browser environment, commands, PNG
metadata, observed issues, decision, retention outcome, cleanup evidence, and
follow-up tasks.

Clean up:

- Delete local screenshot files after review unless they are intentionally kept
  as ignored local artifacts.
- Do not commit screenshot PNG files.
- Run `npm run verify:repo-hygiene`.
- Run `git status --short` and confirm no screenshots, traces, generated docs,
  component API changes, template rewrites, or CI workflow files are staged.

## Review Areas

The review should cover:

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

End each review with:

```bash
npm run verify:repo-hygiene
git status --short
```

The retention policy remains local-first: screenshots are temporary review aids
by default, review notes should keep command output and PNG metadata, and
uploads or GitHub release attachments stay deferred until a release owner
defines that process.

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
- `docs/components/screenshot-artifact-retention.md`
- `docs/components/release-screenshot-review-notes-template.md`
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
