# Workspace Dependency Publish Readiness Metadata

This document defines the metadata gate for workspace dependency publish
readiness. It keeps internal crate dependency versioning explicit before any
crate packaging or publishing work starts.
The decision preparation plan is tracked in
[Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md).
The maintainer evidence checklist is tracked in
[Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md).
The local follow-up map is tracked in
[Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md).
The consolidated first-publish dependency evidence and rollback view is tracked
in [Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md).

## Current State

M133 resolves this item locally. The workspace declares internal crate
dependencies with both a local path and a version requirement:

```toml
dioxus-shadcn-core = { version = "0.2.0", path = "crates/dioxus-shadcn-core" }
dioxus-shadcn-primitives = { version = "0.2.0", path = "crates/dioxus-shadcn-primitives" }
dioxus-shadcn = { version = "0.2.0", path = "crates/dioxus-shadcn" }
```

Cargo uses the path for local development, workspace tests, feature checks,
and source-copy fixture smoke tests, and strips the path when packaging, so a
published crate resolves its internal dependencies from crates.io by version.
This gives publishable crates crates.io-resolvable version metadata once the
dependency crates are published in the documented publish order.

The version requirement must track `[workspace.package] version`. When the
workspace version is bumped, these three entries are bumped in the same change.

## Readiness Contract

`npm run verify:workspace-dependency-publish-readiness` confirms that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- every internal workspace dependency keeps its local path
- every internal workspace dependency declares a version equal to the
  workspace package version
- publish blocker docs list workspace dependency publish readiness as resolved
- publish readiness coverage and runbook docs include the item
- release, quality gate, Cargo publish metadata, publish order metadata, and
  docs-site notes describe the versioned internal dependency shape

The gate should stay read-only. It must not change dependency versions, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, create package archives, or authorize a
release.

## Documentation Alignment

Keep this metadata gate aligned with:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Publish Readiness Coverage Metadata](publish-readiness-coverage-metadata.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Cargo Publish Metadata](cargo-publish-metadata.md)
- [Publish Order Metadata](publish-order-metadata.md)
- [Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md)
- [Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md)
- [Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md)
- [Release and Package Strategy](release.md)
- [Quality Gates](quality-gates.md)
- [Documentation Site Plan](site.md)
- [Project README](../README.md)
- [TODO Plan](../TODOs.md)

These files should be updated in the same change whenever the blocker is
resolved, renamed, or replaced with a different publish dependency strategy.

## Resolution Criteria

This blocker was resolved after maintainers approved first publish and the
versioned internal dependency shape was committed. The review confirmed:

- every publishable crate dependency on another project crate has
  crates.io-resolvable version metadata
- the dependency versions match the intended release versioning policy
- the documented publish order still publishes dependency crates before
  downstream crates
- `cargo package` and `cargo publish` remain separate explicit release-owner
  actions, not side effects of metadata verification

If the dependency strategy changes, update this metadata gate, publish blocker
docs, publish readiness coverage, the resolution runbook, Cargo publish
metadata, publish order metadata, release docs, quality gates, README,
docs-site notes, and TODO planning together.

This gate should keep the versioned internal dependency shape explicit after
resolution.
