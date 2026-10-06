# Cargo Publish Metadata

M96 defines the read-only contract for Cargo publish metadata. The goal is to
keep publishable crate manifests complete enough for review without running
`cargo publish`, creating package archives, or contacting crates.io.

Planned publishable crates:

```text
dioxus-shadcn-core
dioxus-shadcn-primitives
dioxus-shadcn
dioxus-shadcn-cli
```

The planned publish order is tracked in
[Publish Order Metadata](publish-order-metadata.md).
Registry name, ownership, credential, and publish-order evidence is tracked in
[Registry Availability Blocker Handoff](archive/first-publish/registry-availability-blocker-handoff.md)
before any publish readiness blocker is resolved.

Example and verification crates under `examples/` remain application fixtures
and must keep `publish = false`.

## Scope

In scope:

- crate-specific package descriptions for the four planned published crates
- shared workspace `readme`, `keywords`, and `categories` metadata; the
  `readme` is `crates/README.md`, a short user guide, since the root README
  documents development
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
are ready to publish and does not check crates.io itself; the release owner
recorded registry availability in
[Registry Availability Readiness Metadata](archive/first-publish/registry-availability-readiness-metadata.md#resolution).
First publish release notes are recorded in `CHANGELOG.md`. APIs remain
pre-1.0, and the current `0.2.x` API surface is accepted
under the pre-`1.0` breaking-change policy. The repository URL is approved as
`https://github.com/yuxuetr/dioxus-ui`, and root MIT license text is committed
in `LICENSE`. CLI template delivery now uses embedded registry/template assets
and is tracked by
[CLI Template Packaging Readiness Metadata](archive/first-publish/cli-template-packaging-readiness-metadata.md).
The crates.io name and ownership review is resolved for the renamed
`dioxus-shadcn` crates. Workspace
dependency publish readiness is resolved: internal workspace dependencies
declare crates.io-resolvable versions alongside local paths.

The first publish's blocker inventory and its readiness gates are in the
[First Publish Archive](archive/first-publish/README.md). This gate keeps the
internal dependency versions equal to the workspace version, and
`npm run verify:package-contents` keeps the embedded CLI assets and each
crate's `LICENSE` in the packages.
