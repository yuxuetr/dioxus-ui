# Workspace Dependency Blocker Handoff

This handoff consolidates the workspace dependency publish readiness blocker
for the first publish preparation cycle. It does not change dependency
versions, rewrite manifests, run package commands, or authorize publishing.

Use it with:

- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md)
- [Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md)
- [Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md)
- [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md)
- [Publish Order Metadata](publish-order-metadata.md)
- [Cargo Publish Metadata](cargo-publish-metadata.md)

## Current Blocker

| Field | Current Value |
| --- | --- |
| Blocker | Workspace dependency publish readiness |
| Current dependency shape | Path-only internal workspace dependencies |
| Publish state | Unresolved |
| Resolution owner | Maintainer or release owner |
| Focused gates | `npm run verify:workspace-dependency-publish-readiness`, `npm run verify:publish-order`, `npm run verify:cargo-workspace` |

## Current Internal Dependencies

The workspace currently uses local path dependency entries for project crates:

```toml
dioxus-ui-core = { path = "crates/dioxus-ui-core" }
dioxus-ui-primitives = { path = "crates/dioxus-ui-primitives" }
dioxus-ui = { path = "crates/dioxus-ui" }
```

This is valid for local development and workspace checks. It is not enough by
itself to prove that publishable crates can resolve internal dependencies from
crates.io after dependency crates are published.

## Required Evidence

Local follow-up can start only after a maintainer or release owner records:

- confirmed publishable crate set
- confirmed dependency-first publish order
- internal project-crate dependency graph that must resolve after publish
- approved version requirement policy for internal published crate dependencies
- confirmation that local workspace development remains supported
- package owner responsible for applying and validating manifest changes
- confirmation that `cargo package` and `cargo publish` remain manual
  release-owner actions
- rollback expectation if dependency version policy changes before first publish

Record the evidence with
[Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md).

## Local Follow-up After Approval

If workspace dependency publish readiness is approved, update these surfaces
together:

| Surface | Follow-up |
| --- | --- |
| `Cargo.toml` and `crates/*/Cargo.toml` | Apply only maintainer-approved dependency metadata changes |
| `docs/workspace-dependency-publish-readiness-metadata.md` | Move from path-only unresolved metadata to approved publish dependency strategy metadata |
| `docs/publish-order-metadata.md` | Keep dependency-first order aligned with the approved strategy |
| `docs/cargo-publish-metadata.md` | Reflect dependency metadata as approved for publish preparation |
| `docs/publish-readiness-blockers.md` | Remove workspace dependency readiness from current blockers only after focused gates pass |
| `docs/publish-readiness-coverage-metadata.md` | Update coverage from unresolved blocker to resolved readiness item if useful |
| `docs/publish-readiness-resolution-runbook.md` | Update manual dependency evidence and follow-up wording |
| `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md` | Replace unresolved-dependency wording with approved dependency strategy wording |
| `TODOs.md` | Mark the approved local follow-up task done only after validation and commit |

If the decision is `blocked` or `deferred`, keep the blocker unresolved and
record the missing dependency graph, version policy, local development, or
publish-order evidence in TODO planning.

## Manifest Boundary

Keep these states separate:

- local path dependencies can remain valid for workspace development
- publishable crates need a maintainer-approved strategy for resolving internal
  crate dependencies after publish
- dependency crates must remain before downstream crates in publish order
- package and publish commands remain manual release-owner actions

## Validation

Run these checks after any workspace dependency handoff or approved local
follow-up change:

```bash
npm run verify:workspace-dependency-publish-readiness
npm run verify:publish-order
npm run verify:cargo-workspace
npm run verify:cargo-publish-metadata
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
git status --short
```

Run `npm run verify:release` before release-candidate handoff.

## Rollback

If approved dependency metadata changes before first publish:

- revert manifests and dependency docs to the last approved strategy
- keep the blocker unresolved until replacement evidence is recorded
- rerun workspace dependency, publish order, Cargo workspace, Cargo publish
  metadata, and shared publish readiness checks
- update TODO status only after the rollback commit and validation

## Non-goals

This handoff does not:

- choose dependency versions
- change dependency versions
- rewrite manifests
- contact crates.io
- check registry ownership
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- authorize a release
