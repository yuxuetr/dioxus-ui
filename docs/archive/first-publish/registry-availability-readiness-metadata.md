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
  [Publish Order Metadata](../../publish-order-metadata.md)
- named release owner responsible for the actual publish
- confirmation that registry checks happened outside the repository-safe local
  metadata gates

If any crate name is unavailable and not owned by the release owner, the
blocker stays unresolved and a rename milestone is required before publish.

## Resolution

M184 resolved this blocker on 2026-10-05. The first check found `dioxus-ui`
taken by another owner's `dioxus_ui`, so M184 renamed the crates (see
[RFC 0056](../../rfcs/0056-published-crate-names.md)), and the release owner
supplied this evidence:

| Crate | crates.io on 2026-10-05 | Owners | Publish Position |
| --- | --- | --- | --- |
| `dioxus-shadcn-core` | Free (the crates API answered 404) | the release owner's account after the first publish | 1 |
| `dioxus-shadcn-primitives` | Free (the crates API answered 404) | the release owner's account after the first publish | 2 |
| `dioxus-shadcn` | Free (the crates API answered 404) | the release owner's account after the first publish | 3 |
| `dioxus-shadcn-cli` | Free (the crates API answered 404) | the release owner's account after the first publish | 4 |

- Release owner: the repository owner, `yuxuetr`, who runs the publish.
- Credentials: the release owner ran `cargo login`; no token is committed.
- Publish order: `cargo publish --workspace --dry-run` packaged and verified
  all four crates in the order above.
- The check ran outside the local gates, which stay read-only.

A name can be taken between this check and the publish; a failed upload for
that reason reopens the blocker.

## Readiness Contract

`npm run verify:registry-availability-readiness` should confirm that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- planned publishable crate names remain documented
- publish blocker docs list registry availability as resolved
- the per-crate evidence table and the recorded resolution remain documented

The gate is intentionally read-only. It must not contact crates.io, check crate
name availability, check ownership, inspect credentials, run `cargo package`,
run `cargo publish`, or create package archives.

## Resolution Criteria

This blocker can be removed only after a release owner performs the external
publish preparation review. That review should confirm crate name availability
or ownership, publishing credentials, and the intended publish order.

After that review is complete, update this metadata gate, publish blocker docs,
release docs, quality gates, and TODO planning together.
