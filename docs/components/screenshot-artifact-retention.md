# Screenshot Artifact Retention

This document defines the M115 decision boundary for release-candidate
screenshot artifacts after manual screenshot review.

Status: Planned in M115.1; initial retention policy added in M115.2; linked
from release, quality, site, CI, and artifact policy docs in M115.3.

## Problem

The release screenshot review checklist can produce ignored local screenshot
artifacts:

```text
dioxus-ui-web-preview-*.png
dioxus-ui-mobile-browser-preview-*.png
```

Those screenshots are useful for human review, but the project needs a clear
policy for what happens after review. Without one, maintainers may accidentally
commit screenshots, keep stale local evidence, or assume screenshot capture is a
release artifact requirement.

## Options

| Option | Benefit | Cost | Initial Fit |
| --- | --- | --- | --- |
| Delete screenshots after review | Keeps repository and local handoff clean. | Review evidence is limited to notes. | Best default. |
| Keep ignored local screenshots | Useful while iterating on a release candidate. | Easy to confuse stale screenshots with current results. | Acceptable short-term local state. |
| Copy metadata into review notes | Preserves command evidence without binary files. | Does not preserve pixels. | Best lightweight evidence. |
| Attach screenshots to internal review notes | Gives reviewers visual evidence outside Git. | Requires a review system and retention owner. | Deferred until process exists. |
| Attach screenshots to GitHub releases | Gives public release evidence. | Bloats releases and creates support expectations. | Deferred until release policy is stable. |

## Initial Decision

M115 adopts an initial local-first policy:

1. Screenshots are temporary local review artifacts by default.
2. Review notes should record command output and PNG metadata.
3. Screenshots may remain as ignored local files during the review session.
4. Screenshots should be deleted before final handoff unless the reviewer
   intentionally keeps them locally.
5. Screenshots must not be committed to Git.
6. GitHub release attachments and internal review artifact storage remain
   deferred until there is an explicit release owner and retention policy.

## Retention Policy

Default handling:

- Treat screenshots as temporary release-candidate review aids.
- Do not commit screenshot PNG files.
- Do not upload screenshots as part of normal local verification.
- Copy command output and PNG metadata into review notes when evidence is
  needed.
- Delete screenshots before final handoff unless the reviewer intentionally
  keeps them as ignored local files.

Allowed local-only cases:

- Keep ignored screenshots while comparing two local release-candidate runs.
- Keep ignored screenshots while investigating a visual issue.
- Keep ignored screenshots long enough to file a follow-up task with metadata.

Deferred attachment cases:

- Internal review note attachments are deferred until the project has a review
  location, owner, and retention period.
- GitHub release attachments are deferred until public release evidence is part
  of the release policy.
- CI-generated screenshot artifacts are deferred until a non-blocking browser
  workflow is activated and retention is reviewed.

Required cleanup evidence:

- `npm run verify:repo-hygiene` passes.
- `git status --short` shows no staged or unstaged screenshot artifacts.
- Review notes record whether screenshots were deleted or intentionally kept
  ignored locally.

## Review Note Metadata

When screenshot capture is used, copy these details into review notes:

- release candidate identifier
- reviewer
- date
- browser executable or Playwright-managed Chromium
- commands run
- Web desktop screenshot path and PNG metadata
- Web mobile screenshot path and PNG metadata
- mobile browser screenshot path and PNG metadata
- observed visual issues
- retention decision: deleted, kept ignored locally, or attached outside Git
- follow-up tasks

The metadata is enough to prove what was reviewed without making PNG files part
of the source tree.

## Review Notes Template Plan

M116 should add a copyable Markdown review notes template for release-candidate
screenshot review. The template should be small enough to paste into a pull
request, issue, local handoff note, or release checklist without requiring a
separate artifact store.

Required sections:

- release candidate metadata
- reviewer and date
- browser/runtime environment
- commands run
- Web desktop screenshot path and PNG metadata
- Web mobile screenshot path and PNG metadata
- mobile browser screenshot path and PNG metadata
- observed visual issues
- release decision
- retention outcome
- cleanup evidence
- follow-up tasks

Repository-safe boundaries:

- no committed screenshot PNG files
- no screenshot uploads during normal local verification
- no CI workflow activation
- no GitHub release attachment automation
- no internal artifact storage integration
- no generated docs output
- no visual baseline or pixel diff claim

M116 should link the template from:

- `docs/components/release-screenshot-review-checklist.md`
- `docs/components/screenshot-artifact-retention.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/ci-browser-smoke.md`
- `docs/README.md`

## Cleanup Contract

A screenshot review should end with:

```bash
npm run verify:repo-hygiene
git status --short
```

If screenshots were captured, either delete them or leave them ignored locally.
Do not stage or commit PNG files matching:

```text
dioxus-ui-web-preview-*.png
dioxus-ui-mobile-browser-preview-*.png
```

## Non-goals

- no artifact uploads
- no GitHub release automation
- no internal review system integration
- no CI workflow activation
- no screenshot capture by default
- no pixel-level visual diffing
- no automated visual baselines
- no release gate promotion
- no component API changes
- no generated source-copy template rewrites
- no committed screenshots, traces, generated docs, or generated CSS output

## Documentation Alignment

M115 should keep these files aligned:

- `docs/components/screenshot-artifact-retention.md`
- `docs/components/release-screenshot-review-checklist.md`
- `docs/browser-artifact-policy-metadata.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/ci-browser-smoke.md`
- `docs/components/README.md`

The documentation should say clearly that screenshots are review aids, not
source artifacts or required release attachments.

## Follow-up Milestones

After M115, useful follow-up work is:

1. Decide whether a non-blocking CI browser workflow should generate review
   screenshots.
2. Define a release notes evidence format if public screenshot attachments are
   ever needed.
3. Revisit Desktop WebView screenshot capture when native window capture is
   repeatable.
