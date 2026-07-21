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

Status: Release gate passed.

M121.2 result:

- starting `git status --short`: clean
- command: `npm run verify:release`
- result: passed
- first failed command: none
- focused rerun: not needed
- known warning observed: `block v0.1.6` Rust future-incompatibility warning
- warning status: expected and covered by
  [Release Warning Inventory Metadata](release-warning-inventory-metadata.md)
- generated fixture smoke: passed
- release candidate handoff metadata: passed
- repository hygiene inside release gate: passed

No screenshots, traces, generated docs output, component API changes,
source-copy template rewrites, release artifacts, tags, or CI workflow files
were produced by the release gate.

M121.3 result:

- command:
  `DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:browser-local`
- result: passed
- mobile browser smoke: passed
- rendered component DOM verification: passed for 64 components
- web screenshot smoke: passed for 2 viewports
- runtime interaction verification: passed for 5 fixtures
- post-run `git status --short`: clean
- tracked artifact scan: no screenshots, traces, Playwright reports,
  test-results directories, release artifacts, tags, or CI workflow files
  were introduced

M121.4 result:

- `npm run verify:docs`: passed
- `npm run verify:release-docs`: passed
- `npm run verify:package-scripts`: passed
- `npm run verify:release-candidate-handoff`: passed
- `npm run verify:repo-hygiene`: passed for 486 tracked files
- `npm run verify:browser-artifact-policy`: passed
- `npm run verify:package-lock`: passed
- `git diff --check`: passed
- tracked artifact scan: no matching tracked artifacts
- `git tag --points-at HEAD`: no tags
- final `git status --short`: clean
