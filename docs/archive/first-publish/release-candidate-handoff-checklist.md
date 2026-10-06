# Release Candidate Handoff Checklist

This checklist is the final maintainer handoff record for a release candidate.
Use it after deterministic gates and any optional browser review have completed.

It is a manual evidence checklist. It does not authorize publishing by itself.

## Related Documents

- [Release and Package Strategy](../../release.md)
- [Quality Gates](../../quality-gates.md)
- [Release Gate Failure Triage Runbook](../../release-gate-failure-triage-runbook.md)
- [Release Candidate Browser Review Runbook](../../components/release-candidate-browser-review-runbook.md)
- [Release Screenshot Review Notes Template](../../components/release-screenshot-review-notes-template.md)
- [Screenshot Artifact Retention](../../components/screenshot-artifact-retention.md)
- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)

## Candidate Identity

- Release candidate:
- Branch:
- Commit:
- Reviewer:
- Review date:
- Intended next action:

## Required Gate Evidence

Record command results:

```bash
npm run verify:release
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:package-lock
npm run verify:browser-artifact-policy
npm run verify:repo-hygiene
```

Checklist:

- `npm run verify:release` passed or focused failures are recorded below
- release documentation checks passed
- package script wiring checks passed
- package lock metadata checks passed
- browser artifact policy checks passed
- repository hygiene checks passed
- `git diff --check` passed

Focused failures:

- Command:
  - Result:
  - Owner:
  - Follow-up:

Use [Release Gate Failure Triage Runbook](../../release-gate-failure-triage-runbook.md)
to isolate the first failing release command before changing code or
documentation.

## Optional Browser Review Evidence

Browser review is optional and remains outside default and release gates.

- Browser review runbook used:
- Browser executable:
- `npm run verify:browser-local` result:
- Screenshot capture enabled:
- Screenshot review notes location:
- Screenshot retention outcome:
- Visual blocking issues:
- Visual non-blocking follow-ups:

Use
[Release Candidate Browser Review Runbook](../../components/release-candidate-browser-review-runbook.md)
for command order and
[Release Screenshot Review Notes Template](../../components/release-screenshot-review-notes-template.md)
for screenshot metadata.

## Publish Readiness Evidence

Record current blocker status before any publish decision:

- Placeholder repository URL:
- Root license files:
- Pre-1.0 API stability:
- Release notes readiness:
- CLI template packaging strategy:
- Registry availability and ownership:
- Workspace dependency publish readiness:

Use [Publish Readiness Blockers](publish-readiness-blockers.md) and
[Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
for the current source of truth. Use
[Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
when maintainer decisions need explicit evidence before local follow-up. This
checklist does not resolve blockers or authorize publishing. For copyable
decision notes, use
[First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md).
For a single review surface covering all six current blockers, use
[First Publish Decision Packet](first-publish-decision-packet.md).

## Warning Inventory Evidence

Known warning status:

- `block` `0.1.6` Rust future-incompatibility warning:
- New warnings observed:
- Warning follow-up owner:

Use [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)
to distinguish known upstream dependency warnings from new project warnings.

## Artifact And Repository Hygiene

Confirm the release candidate did not commit generated artifacts:

- no screenshot PNG files staged or committed
- no trace files staged or committed
- no generated docs output staged or committed
- no release artifacts staged or committed
- no CI workflow activation staged or committed
- no component API changes staged unexpectedly
- no source-copy template rewrites staged unexpectedly
- no Git tag created by this checklist

Final cleanup commands:

```bash
npm run verify:repo-hygiene
git status --short
```

## Handoff Decision

- Candidate ready for maintainer decision:
- Candidate blocked:
- Blocking reason:
- Required follow-up tasks:
- Non-blocking follow-up tasks:
- Reviewer notes:

## Non-goals

- no package publishing
- no `cargo publish`
- no `cargo package`
- no Git tag creation
- no release artifact creation
- no screenshot upload requirement
- no GitHub release attachment automation
- no CI workflow activation
- no browser smoke promotion into default or release gates
- no generated docs output
- no component API changes
- no source-copy template rewrites
