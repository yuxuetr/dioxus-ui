# Release Candidate Handoff Metadata

M119 defines the read-only metadata gate for the release candidate handoff
checklist. The goal is to keep the final manual handoff record discoverable and
aligned with release gates, optional browser review, publish readiness blockers,
known warning inventory, and artifact hygiene policy.

## Gate Intent

The handoff metadata gate should verify committed source and documentation
only. It should answer whether the handoff checklist still records the evidence
a maintainer needs before making a release-candidate decision.

The gate should validate:

- handoff checklist sections and required evidence fields
- release gate command references
- optional browser review runbook references
- screenshot review notes and retention references
- publish readiness blocker references
- release warning inventory references
- repository hygiene and artifact boundary references
- README, docs index, release docs, quality gates, docs-site notes, browser
  review runbook, and publish readiness runbook discoverability

## Required Checklist Coverage

The checklist should keep these sections:

- Related Documents
- Candidate Identity
- Required Gate Evidence
- Optional Browser Review Evidence
- Publish Readiness Evidence
- Warning Inventory Evidence
- Artifact And Repository Hygiene
- Handoff Decision
- Non-goals

The checklist should keep these command references:

- `npm run verify:release`
- `npm run verify:release-docs`
- `npm run verify:package-scripts`
- `npm run verify:package-lock`
- `npm run verify:browser-artifact-policy`
- `npm run verify:repo-hygiene`
- `git diff --check`
- `git status --short`

The checklist should keep these artifact boundaries:

- no screenshot PNG files staged or committed
- no trace files staged or committed
- no generated docs output staged or committed
- no release artifacts staged or committed
- no CI workflow activation staged or committed
- no Git tag created by this checklist

## Alignment Targets

The verifier should keep these documents aligned:

- `README.md`
- `docs/README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/release-candidate-handoff-checklist.md`
- `docs/components/release-candidate-browser-review-runbook.md`
- `docs/publish-readiness-resolution-runbook.md`

## Out Of Scope

The gate must not:

- run `npm run verify:release`
- launch browser automation
- run `dx serve`
- capture screenshots
- create or upload artifacts
- create Git tags
- run `cargo package`
- run `cargo publish`
- contact crates.io
- activate CI workflows
- generate docs output
- change component APIs
- rewrite source-copy templates

## Failure Examples

The verifier should fail when:

- the handoff checklist loses required sections
- release gate evidence omits required command references
- browser review, screenshot notes, or retention links disappear
- publish readiness blocker links disappear
- release warning inventory links disappear
- repository hygiene boundaries stop mentioning screenshots, traces, generated
  docs, release artifacts, CI workflow activation, or Git tags
- README, docs index, release docs, quality gates, docs-site notes, browser
  review runbook, or publish readiness runbook no longer link to the checklist
