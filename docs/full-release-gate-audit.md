# Full Release Gate Audit

M121 records the local full release gate audit. It is an execution record for
maintainers, not a publish authorization.

## Audit Order

1. Confirm the repository starts from a clean working tree.
2. Run `npm run verify:release`.
3. If the release aggregate fails, use
   [Release Gate Failure Triage Runbook](release-gate-failure-triage-runbook.md)
   to isolate the first failed command.
4. Rerun the focused command after any minimal fix.
5. Rerun `npm run verify:release` if a fix was needed.
6. Run optional browser-local verification when local Chrome is available.
7. Run final metadata and repository hygiene checks.
8. Update `TODOs.md` only after implementation, validation, and commits.

## Source Of Truth

- Release command order:
  [Release and Package Strategy](release.md)
- Failure isolation:
  [Release Gate Failure Triage Runbook](release-gate-failure-triage-runbook.md)
- Final handoff:
  [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)
- Browser review:
  [Release Candidate Browser Review Runbook](components/release-candidate-browser-review-runbook.md)
- Artifact policy:
  [Browser Artifact Policy Metadata](browser-artifact-policy-metadata.md)
- Known warning boundary:
  [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)

## Evidence To Record

- starting `git status --short`
- `npm run verify:release` result
- first failed command if any
- focused rerun result if any
- known warning observations
- optional browser-local verification result or skipped reason
- final repository hygiene result
- final `git status --short`

## Boundaries

This audit must not:

- publish packages
- run `cargo publish`
- run `cargo package`
- create Git tags
- create release artifacts
- upload artifacts
- commit screenshots
- capture screenshots by default
- activate CI workflows
- commit generated docs output
- change component APIs unless a focused failure requires it
- rewrite source-copy templates unless a focused failure requires it

## Current Result

Status: Planned. Results will be recorded as M121 tasks progress.
