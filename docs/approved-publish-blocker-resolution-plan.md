# Approved Publish Blocker Resolution Plan

This plan records the maintainer decisions supplied for first publish
preparation and separates locally resolvable blockers from the crates.io
external readiness blocker. It is not a publish command or package command
authorization.

## Maintainer Decisions

| Area | Decision |
| --- | --- |
| Canonical repository | `https://github.com/yuxuetr/dioxus-ui` |
| Repository owner | `yuxuetr` |
| License | MIT |
| API stability | Current `0.1.x` API surface is acceptable for first publish |
| First publish | Local first-publish preparation may proceed |
| crates.io readiness | Deferred until registry evidence exists |

## Locally Resolvable Blockers

These blockers can be resolved by local metadata, documentation, and verifier
updates:

1. Placeholder repository URL
2. Root license files not committed
3. Pre-1.0 API stability
4. Release notes not publish-ready
5. Workspace dependency publish readiness

Each local resolution must update focused verifier expectations in the same
change as the corresponding metadata and documentation updates.

## Deferred External Blocker

The crates.io registry availability blocker remains unresolved until a release
owner supplies evidence for:

- crate name availability or existing ownership for `dioxus-shadcn-core`
- crate name availability or existing ownership for `dioxus-shadcn-primitives`
- crate name availability or existing ownership for `dioxus-shadcn`
- crate name availability or existing ownership for `dioxus-shadcn-cli`
- owner list or team for every crate
- credential readiness
- publish order confirmation
- release owner responsible for the actual publish

Deferring this blocker is acceptable for local release readiness and internal
trial. It blocks actual crates.io publication.

## Local Resolution Order

Resolve local blockers in this order:

1. Repository identity
2. MIT license metadata and root license text
3. API stability and release notes readiness
4. Workspace dependency publish readiness
5. crates.io registry availability deferral wording

## Safety Boundary

This plan does not:

- contact crates.io
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags
- create GitHub releases
- activate CI workflows

## Validation

For each blocker-specific change, run its focused readiness gate plus:

```bash
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
```
