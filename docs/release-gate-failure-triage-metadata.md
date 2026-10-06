# Release Gate Failure Triage Metadata

M120 defines the manual triage contract for failures inside
`npm run verify:release`. The goal is to make release aggregate failures easier
to isolate without adding automatic repair behavior.

## Triage Intent

The triage runbook should help a maintainer answer three questions:

1. Which release command segment failed?
2. Which focused command should be rerun to isolate the failure?
3. What evidence should be recorded before changing code or documentation?

The runbook should stay manual and repository-safe. It should not turn failures
into automatic cleanup, automatic regeneration, or publish automation.

## Failure Groups

The runbook should group failures by:

- Rust workspace checks
- CLI registry and list smoke
- Cargo and publish readiness metadata gates
- documentation metadata gates
- source-copy generated fixture smoke
- component feature checks
- browser artifact policy metadata
- repository hygiene

## Required Guidance

Each failure group should include:

- first focused command to rerun
- source files or docs to inspect
- evidence to copy into handoff notes
- likely owner or follow-up area
- non-goals and unsafe shortcuts to avoid

## Alignment Targets

The triage runbook should be linked from:

- `README.md`
- `docs/README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/release-candidate-handoff-checklist.md`
- `docs/release-candidate-handoff-metadata.md`

## Repository-safe Boundaries

The runbook should explicitly avoid:

- destructive cleanup by default
- generated artifact commits
- screenshot commits
- trace commits
- browser workflow activation
- package publishing
- Git tag creation
- release artifact creation
- component API changes unless the failure proves they are required
- source-copy template rewrites unless the failure proves they are required

## Out Of Scope

- no automatic repair command
- no generated docs output
- no browser launch
- no screenshot capture
- no CI workflow activation
- no `cargo package`
- no `cargo publish`
- no crates.io access
- no release tagging
