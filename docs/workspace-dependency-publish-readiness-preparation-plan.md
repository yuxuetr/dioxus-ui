# Workspace Dependency Publish Readiness Preparation Plan

This plan prepares the maintainer decision for internal workspace dependency
metadata before first publish. It is a planning artifact, not a dependency
change or publish authorization.

Use this plan with:

- [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md)
- [Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md)
- [Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md)
- [Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md)
- [Publish Order Metadata](publish-order-metadata.md)
- [Cargo Publish Metadata](cargo-publish-metadata.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)

## Decision Question

Maintainers need to decide how publishable crates should express internal
project crate dependencies for first publish.

The decision must cover:

- whether local development keeps workspace path dependencies
- whether publishable crate dependencies need explicit version requirements
- how dependency versions align with the workspace package version
- whether the planned publish order still publishes dependency crates first
- how `cargo package` and `cargo publish` remain explicit release-owner actions
- which local checks prove metadata consistency before any package command

## Current Local Shape

The workspace currently uses path-only internal workspace dependencies for
local development:

```toml
dioxus-ui-core = { path = "crates/dioxus-ui-core" }
dioxus-ui-primitives = { path = "crates/dioxus-ui-primitives" }
dioxus-ui = { path = "crates/dioxus-ui" }
```

That shape is valid for local checks, examples, source-copy fixture smoke, and
feature checks. It remains unresolved for publish readiness until maintainers
approve a crates.io-resolvable internal dependency strategy.

## Preparation Pass

Prepare the decision in this order:

1. Confirm planned publishable crates and publish order still match Cargo
   publish metadata.
2. Confirm which publishable crates depend on other project crates.
3. Review whether each internal dependency needs explicit version metadata for
   first publish.
4. Confirm whether the selected strategy preserves local development behavior.
5. Record maintainer evidence in the workspace dependency evidence checklist
   before changing manifests.
6. Only after approval, update dependency metadata and blocker docs together
   using the workspace dependency local follow-up map.

## Decision Outcomes

| Outcome | Meaning | Local Follow-up |
| --- | --- | --- |
| `approved` | Maintainers approve a publish-ready internal dependency strategy | Update manifests if required, workspace dependency metadata, publish order metadata, Cargo publish metadata, publish blockers, release docs, quality gates, README, docs-site notes, and TODO planning |
| `blocked` | Required dependency/version evidence is missing | Keep blocker unresolved and record missing evidence |
| `deferred` | First publish needs a focused dependency metadata milestone first | Create a follow-up TODO milestone without changing manifests or publishing |

## Non-goals

M127 must not:

- change dependency versions
- rewrite crate manifests
- run `cargo package`
- run `cargo publish`
- contact crates.io
- inspect credentials
- check registry ownership
- create package archives
- create Git tags
- authorize publishing

## Safe Validation

Use read-only checks while preparing decision artifacts:

```bash
npm run verify:workspace-dependency-publish-readiness
npm run verify:publish-order
npm run verify:cargo-workspace
npm run verify:cargo-publish-metadata
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
```

Run `npm run verify:release` only for final local release-candidate confidence.
It still must not package, publish, contact registries, create tags, or create
release artifacts.

## Exit Criteria

This preparation pass is complete when:

- the internal dependency strategy decision question is explicit
- planned publish order and dependency metadata review are linked
- local development path dependencies are separated from publish readiness
  requirements
- workspace dependency publish readiness remains unresolved until maintainer
  approval is recorded
- no dependency version changes, manifest rewrites, package archives, publish
  commands, registry contact, generated release notes, tags, screenshots,
  traces, or CI workflow files are committed
