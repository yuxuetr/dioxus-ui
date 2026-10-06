# Release Notes Evidence Checklist

This checklist records the minimum maintainer evidence required before first
publish release notes can be marked ready. It does not approve release contents
or generate release notes by itself.

Use this checklist with:

- [Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md)
- [Release Notes Local Follow-up Map](release-notes-local-follow-up-map.md)
- [Release Notes Readiness Metadata](release-notes-readiness-metadata.md)
- [Release Notes Blocker Handoff](release-notes-blocker-handoff.md)
- [Changelog Metadata](../../changelog-metadata.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Evidence Rules

- Evidence must come from maintainers or the release owner.
- `CHANGELOG.md` structure remains separate from publish-ready note
  completeness.
- Local changelog follow-up starts only after the included/excluded scope and
  known warnings are recorded.
- Tags, GitHub releases, package archives, and publish commands remain separate
  release-owner actions.

## Required Evidence

| Evidence Area | Required Evidence | Focused Gate |
| --- | --- | --- |
| Release owner | Owner responsible for final release-note approval | `npm run verify:release-notes-readiness` |
| Changelog owner | Owner responsible for `CHANGELOG.md` updates | `npm run verify:changelog` |
| Included scope | Components, CLI, docs, examples, and release-hardening changes included in first publish notes | `npm run verify:release-notes-readiness` |
| Excluded scope | Deferred platform, visual, browser, publish, or API work excluded from first publish notes | `npm run verify:release-notes-readiness` |
| Known warnings | Warnings that should appear in release-candidate handoff or release notes | `npm run verify:release-docs` |
| Migration notes | Required migration notes after API stability decisions, or explicit confirmation that none are required | `npm run verify:changelog` |
| Release boundary | Confirmation that evidence does not create tags, GitHub releases, package archives, or publish crates | `npm run verify:repo-hygiene` |

## Validation Commands

After evidence is recorded and approved local follow-up is implemented, run:

```bash
npm run verify:release-notes-readiness
npm run verify:changelog
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
git status --short
```

## Non-goals

This checklist does not:

- approve release contents
- generate release notes
- derive changes from Git history
- run git-cliff
- rewrite commit history
- create tags
- create GitHub releases
- create package archives
- run `cargo package`
- run `cargo publish`
