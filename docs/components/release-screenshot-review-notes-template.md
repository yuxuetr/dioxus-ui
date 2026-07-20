# Release Screenshot Review Notes Template

This template records release-candidate screenshot review evidence without
committing screenshot files or requiring an artifact upload system.

Use it with the manual checklist in
[`release-screenshot-review-checklist.md`](release-screenshot-review-checklist.md)
and the retention policy in
[`screenshot-artifact-retention.md`](screenshot-artifact-retention.md).

## Copyable Template

````markdown
# Release Screenshot Review Notes

## Release Candidate

- Release candidate:
- Branch or commit:
- Reviewer:
- Review date:
- Review location:

## Environment

- Operating system:
- Browser mode:
- Browser executable:
- Playwright-managed Chromium:
- Dioxus CLI version:
- Node.js version:
- Rust toolchain:

## Commands

```bash
npm run verify:browser-local
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
npm run verify:repo-hygiene
git status --short
````

## Screenshot Metadata

Web desktop:

- Path:
- Viewport:
- Dimensions:
- Bytes:
- PNG metadata line:

Web mobile:

- Path:
- Viewport:
- Dimensions:
- Bytes:
- PNG metadata line:

Mobile browser:

- Path:
- Viewport:
- Dimensions:
- Bytes:
- PNG metadata line:

## Review Areas

- Overview and action panels:
- Forms, labels, invalid states, and disabled states:
- Input Group and Input OTP:
- Message, Attachment, Bubble, Marker, and Message Scroller:
- Chart title, legend, SVG plot, fallback table, and fallback states:
- Overlay-open panel and visible dialog structure:
- Runtime interaction fixtures:
- Mobile profile markers:
- Component inventory wrapping and overflow:
- Typography, spacing, borders, focus rings, and contrast:

## Observed Issues

- Issue:
  - Area:
  - Viewport:
  - Severity:
  - Evidence:
  - Follow-up:

## Decision

- Release screenshot review decision:
- Blocking issues:
- Non-blocking follow-up tasks:

## Retention Outcome

- Screenshots deleted:
- Screenshots kept ignored locally:
- Screenshots attached outside Git:
- Attachment location:
- Retention owner:
- Retention period:

## Cleanup Evidence

- `npm run verify:repo-hygiene` result:
- `git status --short` result:
- Confirmed no staged screenshot PNG files:
- Confirmed no traces:
- Confirmed no generated docs:
- Confirmed no component API changes:
- Confirmed no template rewrites:
- Confirmed no release artifacts:
- Confirmed no CI workflow files:
```

## Guidance

Keep screenshot files local unless a separate release process explicitly owns
uploads and retention. The useful long-lived evidence is the command output,
PNG metadata, review decision, observed issue list, and cleanup result.

Do not paste binary screenshot data into this note. Do not commit screenshot
files that match:

```text
dioxus-ui-web-preview-*.png
dioxus-ui-mobile-browser-preview-*.png
```

## Non-goals

- no screenshot upload requirement
- no GitHub release attachment automation
- no internal artifact storage integration
- no CI workflow activation
- no screenshot capture by default
- no visual baseline or pixel diff claim
- no generated docs output
- no component API changes
- no source-copy template rewrites
