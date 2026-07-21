# Publish Order Metadata

This document defines the planned crate publish order. It is a metadata
contract, not a publish authorization.

## Planned Order

Publish crates in this order after all publish readiness blockers are resolved:

1. `dioxus-ui-core`
2. `dioxus-ui-primitives`
3. `dioxus-ui`
4. `dioxus-ui-cli`

The dependency crates come first so downstream crates can resolve their
workspace dependencies from crates.io during publication.
Workspace dependency publish readiness remains unresolved until internal crate
dependencies have crates.io-resolvable version metadata aligned with this
publish order.
The workspace dependency publish readiness gate should stay aligned with this
publish order whenever internal project crate dependencies change.
Use
[Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md)
before changing dependency metadata.
Use [Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md)
to record the approved dependency evidence before local follow-up.
Use [Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md)
to keep approved manifest changes aligned with publish order.

## Readiness Contract

`npm run verify:publish-order` should confirm that:

- release docs list the same planned publish order
- Cargo publish metadata lists the same planned publishable crate set
- registry availability metadata lists the same planned publishable crate set
- the publish readiness runbook includes publish order in the registry
  availability evidence
- package scripts expose the focused publish order check
- the release aggregate includes the focused publish order check

The gate is intentionally read-only. It must not create package archives, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, change dependency versions, or authorize a
release.

## Update Rules

When workspace package dependencies or crate publication scope changes, update
this document, release docs, Cargo publish metadata, registry availability
metadata, the publish readiness runbook, quality gates, README, and TODO
planning together.
