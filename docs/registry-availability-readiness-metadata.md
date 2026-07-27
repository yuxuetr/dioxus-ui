# Registry Availability Readiness Metadata

This document defines the metadata gate for registry availability readiness.
It keeps the crates.io name and ownership review blocker visible without
performing external registry checks.
Use [Registry Availability Blocker Handoff](registry-availability-blocker-handoff.md)
for the consolidated release-owner evidence, rollback, and validation view.

## Current State

The planned publishable crates are:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

The planned publish order is:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

Local Cargo metadata can confirm that these crate names are present in the
workspace and that publish metadata is internally consistent. That does not
prove the names are available on crates.io, that ownership is configured, or
that release credentials are ready.

## Readiness Contract

`npm run verify:registry-availability-readiness` should confirm that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- planned publishable crate names remain documented
- publish blocker docs still list registry availability as unresolved
- release, quality gate, Cargo publish metadata, and docs-site notes do not
  imply crates.io availability has been checked

The gate is intentionally read-only. It must not contact crates.io, check crate
name availability, check ownership, inspect credentials, run `cargo package`,
run `cargo publish`, or create package archives.

## Resolution Criteria

This blocker can be removed only after a release owner performs the external
publish preparation review. That review should confirm crate name availability
or ownership, publishing credentials, and the intended publish order.

After that review is complete, update this metadata gate, publish blocker docs,
release docs, quality gates, and TODO planning together.
