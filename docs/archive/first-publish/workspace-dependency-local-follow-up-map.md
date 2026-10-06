# Workspace Dependency Local Follow-up Map

This map describes local follow-up after maintainers approve an internal
workspace dependency publish strategy. It is hypothetical until evidence is
recorded and a maintainer decision is committed.

Use this map with:

- [Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md)
- [Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md)
- [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md)
- [Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md)
- [Publish Order Metadata](../../publish-order-metadata.md)
- [Cargo Publish Metadata](../../cargo-publish-metadata.md)

## Outcome Map

| Decision | Local Files | Validation | Notes |
| --- | --- | --- | --- |
| `approved` with explicit internal versions | `Cargo.toml`, `crates/*/Cargo.toml`, `docs/workspace-dependency-publish-readiness-metadata.md`, `docs/publish-order-metadata.md`, `docs/cargo-publish-metadata.md`, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:workspace-dependency-publish-readiness`; `npm run verify:publish-order`; `npm run verify:cargo-workspace`; `npm run verify:cargo-publish-metadata`; `npm run verify:docs` | Keep local path support and add publish-ready version metadata only after maintainer approval |
| `approved` with alternate Cargo-supported strategy | Strategy documentation, affected manifests if required, readiness metadata, publish order metadata, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:workspace-dependency-publish-readiness`; `npm run verify:cargo-workspace`; `npm run verify:cargo-publish-metadata`; `npm run verify:docs` | Document why the strategy is publish-ready before removing the blocker |
| `blocked` | Evidence notes or TODO planning only | `npm run verify:workspace-dependency-publish-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:docs` | Keep blocker unresolved and preserve current path-only metadata |
| `deferred` | `TODOs.md`, dependency readiness planning docs, optional manifest audit docs | `npm run verify:workspace-dependency-publish-readiness`; `npm run verify:docs`; `git diff --check` | Create a focused dependency metadata milestone before resolving publish readiness |

## Manifest Boundary

Local development path dependencies and publishable dependency version metadata
must be reviewed separately:

- local development should continue to work inside the workspace
- publishable crates must be able to resolve internal dependencies after
  dependency crates are published
- planned publish order must stay dependency-first
- package and publish commands remain manual release-owner actions

## Common Validation

After approved local follow-up, run:

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

## Non-goals

This map does not:

- apply dependency changes
- rewrite manifests
- choose dependency versions
- contact crates.io
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- authorize publishing
