# Publish Readiness Resolution Runbook

This document defines the manual resolution runbook for known publish readiness
blockers. It is a planning artifact, not a publish authorization.

## Resolution Order

Resolve blockers in this order before any package archive or publish command:

1. Repository identity
2. License files
3. API stability policy
4. Release notes readiness
5. CLI template packaging strategy
6. Registry availability and ownership

## Blocker Runbook

| Blocker | Manual Evidence Required | Follow-up Updates |
| --- | --- | --- |
| Placeholder repository URL | Final repository owner and URL are chosen and committed in workspace metadata | Update repository identity metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and TODO planning |
| Root license files not committed | Maintainer-reviewed `LICENSE-MIT` and `LICENSE-APACHE` files are committed | Update license readiness metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and TODO planning |
| Pre-1.0 API stability | Maintainers decide whether `0.1.x` APIs are acceptable for first publish or define a stabilization/migration policy | Update API stability metadata, release docs, changelog guidance, quality gates, README, and TODO planning |
| Release notes not publish-ready | Maintainers define complete release notes for the intended first publish | Update release notes readiness metadata, changelog metadata, release docs, quality gates, README, and TODO planning |
| CLI template packaging strategy | CLI owner implements compile-time embedding or a stable install-location template package | Update CLI template packaging metadata, publish blockers, release docs, quality gates, README, and TODO planning |
| Registry availability not checked | Release owner confirms crates.io names, ownership, credentials, and publish order | Update registry availability metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and TODO planning |

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
embed or package CLI templates, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.

## Exit Criteria

When a maintainer resolves a blocker, update the blocker-specific metadata
gate and this runbook in the same change. The publish readiness coverage gate
should continue to prove that every remaining blocker has a focused gate.
Keep the planned publish order aligned with
[Publish Order Metadata](publish-order-metadata.md).
