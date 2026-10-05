# Registry Availability Readiness Metadata

This document defines the metadata gate for registry availability readiness.
It keeps the crates.io name and ownership review blocker visible without
performing external registry checks.
Use [Registry Availability Blocker Handoff](registry-availability-blocker-handoff.md)
for the consolidated release-owner evidence, rollback, and validation view.

## Current State

The planned publishable crates are:

```text
dioxus-shadcn-core
dioxus-shadcn-primitives
dioxus-shadcn
dioxus-shadcn-cli
```

The planned publish order is:

```text
dioxus-shadcn-core
dioxus-shadcn-primitives
dioxus-shadcn
dioxus-shadcn-cli
```

Local Cargo metadata can confirm that these crate names are present in the
workspace and that publish metadata is internally consistent. That does not
prove the names are available on crates.io, that ownership is configured, or
that release credentials are ready.

## Deferral

M133 keeps this blocker deferred until a release owner supplies crates.io
evidence. Deferral blocks crates.io publishing; it does not block local release
readiness or internal trial.

Required crates.io evidence, recorded by the release owner per crate:

| Crate | Name Evidence | Owners | Publish Position |
| --- | --- | --- | --- |
| `dioxus-shadcn-core` | Available, or already owned by the release owner | crates.io owner list or team | 1 |
| `dioxus-shadcn-primitives` | Available, or already owned by the release owner | crates.io owner list or team | 2 |
| `dioxus-shadcn` | Available, or already owned by the release owner | crates.io owner list or team | 3 |
| `dioxus-shadcn-cli` | Available, or already owned by the release owner | crates.io owner list or team | 4 |

Shared evidence for the whole publish:

- credential readiness confirmed by the release owner, without committing or
  pasting tokens into the repository
- publish order confirmation matching
  [Publish Order Metadata](publish-order-metadata.md)
- named release owner responsible for the actual publish
- confirmation that registry checks happened outside the repository-safe local
  metadata gates

If any crate name is unavailable and not owned by the release owner, the
blocker stays unresolved and a rename milestone is required before publish.

## Readiness Contract

`npm run verify:registry-availability-readiness` should confirm that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- planned publishable crate names remain documented
- publish blocker docs still list registry availability as unresolved
- the deferral statement and the per-crate evidence table remain documented
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
