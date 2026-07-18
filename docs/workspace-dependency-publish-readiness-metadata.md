# Workspace Dependency Publish Readiness Metadata

This document defines the planned metadata gate for workspace dependency
publish readiness. It keeps internal crate dependency versioning explicit
before any crate packaging or publishing work starts.

## Current State

The workspace currently defines local crate dependencies with path-only
workspace dependency entries:

```toml
dioxus-ui-core = { path = "crates/dioxus-ui-core" }
dioxus-ui-primitives = { path = "crates/dioxus-ui-primitives" }
dioxus-ui = { path = "crates/dioxus-ui" }
```

That layout is correct for local development, workspace tests, feature checks,
and source-copy fixture smoke tests. It is not enough by itself to prove that
publishable crates have crates.io-resolvable dependency metadata.

Before publishing, internal publishable crate dependencies should be reviewed
and updated so each published crate can resolve its internal dependencies from
crates.io after dependency crates are published. The expected reviewed shape is
path dependencies with explicit version requirements aligned with the workspace
package version, or an equivalent Cargo-supported publish strategy chosen by
maintainers.

## Readiness Contract

The future `npm run verify:workspace-dependency-publish-readiness` gate should
confirm that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- Cargo workspace dependency entries for publishable internal crates remain
  documented
- publish blocker docs list workspace dependency publish readiness as
  unresolved while path-only internal dependencies remain
- publish readiness coverage and runbook docs include the new blocker
- release, quality gate, Cargo publish metadata, publish order metadata, and
  docs-site notes do not imply dependency versions have been made
  publish-ready

The gate should stay read-only. It must not change dependency versions, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, create package archives, or authorize a
release.

## Resolution Criteria

This blocker can be removed only after maintainers review and commit a
publish-ready internal dependency strategy. The review should confirm:

- every publishable crate dependency on another project crate has
  crates.io-resolvable version metadata
- the dependency versions match the intended release versioning policy
- the documented publish order still publishes dependency crates before
  downstream crates
- `cargo package` and `cargo publish` remain separate explicit release-owner
  actions, not side effects of metadata verification

After that review is complete, update this metadata gate, publish blocker docs,
publish readiness coverage, the resolution runbook, Cargo publish metadata,
publish order metadata, release docs, quality gates, README, docs-site notes,
and TODO planning together.
