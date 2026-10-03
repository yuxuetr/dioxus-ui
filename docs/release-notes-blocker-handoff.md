# Release Notes Blocker Handoff

This handoff consolidates the release notes readiness blocker for the first
publish preparation cycle. It does not approve release contents, generate
release notes, derive changes from Git history, create tags, or authorize
publishing.

Use it with:

- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md)
- [Release Notes Evidence Checklist](release-notes-evidence-checklist.md)
- [Release Notes Local Follow-up Map](release-notes-local-follow-up-map.md)
- [Release Notes Readiness Metadata](release-notes-readiness-metadata.md)
- [Changelog Metadata](changelog-metadata.md)

## Current Blocker

| Field | Current Value |
| --- | --- |
| Blocker | Release notes not publish-ready |
| Current changelog state | Project-owned structure exists |
| Publish state | Resolved locally: first publish scope recorded in `CHANGELOG.md` |
| Resolution owner | Maintainer or release owner |
| Focused gates | `npm run verify:release-notes-readiness`, `npm run verify:changelog` |

## Required Evidence

Local follow-up can start only after a maintainer or release owner records:

- release owner responsible for final approval
- changelog owner responsible for local `CHANGELOG.md` updates
- included first-publish scope
- excluded scope that should remain deferred
- known warnings for release-candidate handoff or release notes
- whether `CHANGELOG.md` should become publish-ready or remain structurally
  owned while another handoff surface carries release notes
- migration note requirement after API stability decisions, or explicit
  confirmation that none are required
- confirmation that this evidence does not create tags, GitHub releases,
  package archives, or publish crates

Record the evidence with
[Release Notes Evidence Checklist](release-notes-evidence-checklist.md).

## Local Follow-up After Approval

If release notes are approved, update one of these surfaces:

| Approved Path | Local Follow-up |
| --- | --- |
| `CHANGELOG.md` carries release notes | Update `CHANGELOG.md`, release notes metadata, changelog metadata, publish readiness docs, release docs, quality gates, README, docs-site notes, and `TODOs.md` |
| Separate handoff notes carry release notes | Update release candidate handoff docs, release notes metadata, publish readiness docs, release docs, quality gates, README, docs-site notes, and `TODOs.md` |

In both paths, remove release notes readiness from current blockers only after
approved content and focused gates agree.

If the decision is `blocked` or `deferred`, keep the blocker unresolved and
record the missing release scope, warning text, owner, or migration-note
evidence in TODO planning.

## Changelog Boundary

Keep these states separate:

- `CHANGELOG.md` can be structurally valid while release notes remain incomplete
- generated notes from Git history are out of scope for readiness metadata
- release notes scope must come from maintainer or release-owner approval
- tags, GitHub releases, package archives, and publish commands remain separate
  release-owner actions

## Validation

Run these checks after any release notes handoff or approved local follow-up
change:

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

Run `npm run verify:release-candidate-handoff` when release candidate handoff
wording changes.

## Rollback

If approved release-note scope changes before first publish:

- revert release notes and changelog metadata to the last approved decision
- keep the blocker unresolved until replacement evidence is recorded
- rerun release notes, changelog, and shared publish readiness checks
- update TODO status only after the rollback commit and validation

## Non-goals

This handoff does not:

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
