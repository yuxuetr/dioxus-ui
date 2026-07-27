# Publish Readiness Resolution Runbook

This document defines the manual resolution runbook for known publish readiness
blockers. It is a planning artifact, not a publish authorization.

Use [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
to collect maintainer decisions, required evidence, local follow-up files, and
safe validation commands before resolving any blocker.
Use [First Publish Readiness Plan](first-publish-readiness-plan.md) to keep the
resolution cycle ordered and repository-safe before blocker-specific follow-up
starts.
Use [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
to confirm maintainer evidence before changing blocker state.
Use [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
to identify local files, gates, and rollback considerations after approval.
Use
[First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)
when those decisions need to be copied into release-candidate notes.
Use [Registry Availability Blocker Handoff](registry-availability-blocker-handoff.md)
for the release-owner registry evidence needed before resolving crates.io
availability.

## Resolution Order

Resolve blockers in this order before any package archive or publish command:

1. Repository identity
2. License files
3. API stability policy
4. Release notes readiness
5. Registry availability and ownership
6. Workspace dependency publish readiness

## Blocker Runbook

| Blocker | Manual Evidence Required | Follow-up Updates |
| --- | --- | --- |
| Placeholder repository URL | Final repository owner and URL are chosen and committed in workspace metadata | Update repository identity metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and TODO planning |
| Root license files not committed | Maintainer-reviewed `LICENSE` file is committed | Update license readiness metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and TODO planning |
| Pre-1.0 API stability | Maintainers decide whether `0.1.x` APIs are acceptable for first publish or define a stabilization/migration policy | Update API stability metadata, release docs, changelog guidance, quality gates, README, and TODO planning |
| Release notes not publish-ready | Maintainers define complete release notes for the intended first publish using the release notes readiness preparation plan | Update release notes readiness metadata, changelog metadata, release docs, quality gates, README, and TODO planning |
| Registry availability not checked | Release owner confirms crates.io names, ownership, credentials, and publish order | Update registry availability metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and TODO planning |
| Workspace dependency publish readiness | Maintainers confirm internal crate dependencies have crates.io-resolvable version metadata using the workspace dependency preparation plan | Update workspace dependency metadata, publish blockers, Cargo publish metadata, publish order metadata, release docs, quality gates, README, and TODO planning |

Resolved items:

| Item | Resolution Evidence | Ongoing Verification |
| --- | --- | --- |
| CLI template packaging strategy | `dioxus-ui-cli` embeds registry and template assets at compile time | `npm run verify:cli-template-packaging-readiness` |

## Readiness Contract

`npm run verify:publish-readiness-runbook` should confirm that:

- every current publish blocker has manual evidence requirements
- every current publish blocker has required follow-up update targets
- package scripts expose the focused runbook check
- the release aggregate includes the focused runbook check
- README, release docs, quality gates, Cargo publish metadata, coverage
  metadata, and docs-site notes mention the runbook gate

The gate is intentionally read-only. It must not resolve blockers, replace
repository URLs, stabilize APIs, generate release notes, generate license text,
change embedded CLI template delivery, change dependency versions, contact
registries, inspect credentials, run `cargo package`, run `cargo publish`, or
create package archives.

## Exit Criteria

When a maintainer resolves a blocker, update the blocker-specific metadata
gate and this runbook in the same change. The publish readiness coverage gate
should continue to prove that every remaining blocker has a focused gate.
Keep the planned publish order aligned with
[Publish Order Metadata](publish-order-metadata.md).
For final release-candidate maintainer handoff before any publish decision, use
[Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md).
