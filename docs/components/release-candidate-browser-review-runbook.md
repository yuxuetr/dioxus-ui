# Release Candidate Browser Review Runbook

This runbook gives maintainers one local-first sequence for browser-backed
release-candidate review. It connects deterministic release checks, opt-in
browser smoke, optional screenshot capture, manual visual review, review notes,
and cleanup.

Use this when preparing a release candidate that needs browser-rendered
confidence beyond source-level metadata checks.

## Related Documents

- [Browser Smoke Aggregate](browser-smoke-aggregate.md)
- [Web Preview Screenshot Smoke](web-preview-screenshot-smoke.md)
- [Release Screenshot Review Checklist](release-screenshot-review-checklist.md)
- [Release Screenshot Review Notes Template](release-screenshot-review-notes-template.md)
- [Screenshot Artifact Retention](screenshot-artifact-retention.md)
- [CI Browser Smoke Guide](../ci-browser-smoke.md)
- [Release and Package Strategy](../release.md)
- [Release Candidate Handoff Checklist](../release-candidate-handoff-checklist.md)
- [Quality Gates](../quality-gates.md)

## Prerequisites

Confirm these before starting:

- working branch is ready for release-candidate review
- Node dependencies are installed
- Rust workspace dependencies are available
- `dx` is available for browser preview commands
- either Playwright-managed Chromium is installed or
  `DIOXUS_UI_BROWSER_EXECUTABLE` points to a local Chrome or Chromium binary
- no other `dx serve` process is using the browser smoke preview port

Record the starting repository state:

```bash
git status --short
```

## Deterministic Checks

Run deterministic gates before browser review:

```bash
npm run verify
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:browser-artifact-policy
npm run verify:repo-hygiene
```

For a full local release candidate, run the release aggregate separately:

```bash
npm run verify:release
```

The release aggregate is intentionally separate from browser smoke. It does not
install browsers, launch Playwright, capture screenshots, upload artifacts, or
claim native Mobile/Desktop runtime coverage.

## Browser Smoke

Run the serial browser aggregate after deterministic checks pass:

```bash
npm run verify:browser-local
```

For local Chrome or Chromium:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:browser-local
```

Do not run browser preview commands in parallel. The aggregate already runs
mobile browser smoke, rendered component DOM verification, Web screenshot smoke,
and runtime interaction verification in sequence so their `dx serve` processes
do not overlap.

## Optional Screenshot Capture

Capture screenshots only after the serial browser aggregate passes.

With Playwright-managed Chromium:

```bash
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

With local Chrome or Chromium:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

Expected ignored screenshot patterns:

```text
dioxus-ui-web-preview-*.png
dioxus-ui-mobile-browser-preview-*.png
```

The commands validate PNG signature, byte size, and viewport dimensions. They
do not compare pixels or certify visual parity.

## Manual Review

Use [Release Screenshot Review Checklist](release-screenshot-review-checklist.md)
for the actual visual review areas. The checklist covers desktop, mobile,
message, form, chart, overlay, runtime interaction, and mobile profile panels.

## Review Notes

Copy command output and PNG metadata into
[Release Screenshot Review Notes Template](release-screenshot-review-notes-template.md).

At minimum, record:

- release candidate identifier
- branch or commit
- reviewer and date
- browser mode and executable
- commands run
- screenshot paths and PNG metadata
- observed issues
- release decision
- retention outcome
- cleanup evidence
- follow-up tasks

## Retention And Cleanup

Apply [Screenshot Artifact Retention](screenshot-artifact-retention.md):

- treat screenshots as temporary local review aids by default
- do not commit screenshot PNG files
- do not upload screenshots during normal local verification
- copy command output and PNG metadata into review notes when evidence is needed
- delete screenshots before final handoff unless intentionally keeping ignored
  local files

End with:

```bash
npm run verify:repo-hygiene
git status --short
```

## Failure Triage

Use this order when a command fails:

- deterministic docs or metadata failure: fix the documented source of truth
  before browser review
- `dx serve` or localhost failure: stop duplicate preview servers and rerun the
  serial aggregate
- missing browser binary: install Playwright Chromium or set
  `DIOXUS_UI_BROWSER_EXECUTABLE`
- selector or DOM failure: inspect rendered preview state before changing
  selectors
- screenshot metadata failure: inspect viewport configuration and generated PNG
  metadata
- visual review issue: record it in the review notes and create a follow-up task
  before deciding release readiness

## Non-goals

- no browser smoke promotion into `npm run verify`
- no browser smoke promotion into `npm run verify:release`
- no screenshot capture by default
- no screenshot uploads during normal local verification
- no GitHub release attachment automation
- no CI workflow activation
- no pixel-level visual diffing
- no automated visual baselines
- no native Desktop WebView screenshot claim
- no native Mobile device or simulator claim
- no generated docs output
- no component API changes
- no source-copy template rewrites
