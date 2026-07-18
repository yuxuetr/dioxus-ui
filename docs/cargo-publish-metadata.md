# Cargo Publish Metadata

M96 defines the read-only contract for Cargo publish metadata. The goal is to
keep publishable crate manifests complete enough for review without running
`cargo publish`, creating package archives, or contacting crates.io.

Planned publishable crates:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

Example and verification crates under `examples/` remain application fixtures
and must keep `publish = false`.

## Scope

In scope:

- crate-specific package descriptions for the four planned published crates
- shared workspace `readme`, `keywords`, and `categories` metadata
- inheritance of shared publish metadata by crates under `crates/`
- `publish = false` boundaries for example workspace members
- release documentation and package script wiring

Out of scope:

- running `cargo publish`
- running `cargo package`
- creating package archives
- checking crates.io name availability
- replacing the placeholder repository URL
- updating dependencies or dependency freshness policy
- generating changelogs or release notes

## Expected Check

The verifier should fail when committed manifest metadata drifts. Examples
include:

- a planned published crate loses its description
- a crate stops inheriting shared `readme`, `keywords`, or `categories`
- an example package loses `publish = false`
- release verification stops running the focused metadata gate

This gate should make publish metadata reviewable. It does not claim the crates
are ready to publish while the repository URL is still a placeholder, APIs
remain pre-1.0, root license files are not yet committed, and CLI template
packaging remains unresolved. The crates.io name and ownership review remains
unresolved.

Known blockers are tracked separately in
[Publish Readiness Blockers](publish-readiness-blockers.md). Keep that
inventory aligned when a maintainer intentionally resolves a blocker.
Publish readiness coverage is tracked separately in
[Publish Readiness Coverage Metadata](publish-readiness-coverage-metadata.md).
Publish readiness resolution is tracked separately in
[Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md).
