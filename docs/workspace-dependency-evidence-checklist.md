# Workspace Dependency Evidence Checklist

This checklist records the minimum maintainer evidence required before local
workspace dependency metadata can change for first publish. It does not approve
or apply dependency changes by itself.

Use this checklist with:

- [Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md)
- [Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md)
- [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md)
- [Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md)
- [Publish Order Metadata](publish-order-metadata.md)
- [Cargo Publish Metadata](cargo-publish-metadata.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Evidence Rules

- Evidence must come from maintainers or the release owner.
- Local manifest follow-up starts only after the selected strategy is recorded.
- Path-only workspace dependencies remain valid for local development until a
  publish strategy is approved.
- `cargo package` and `cargo publish` remain explicit release-owner actions,
  not metadata verification side effects.

## Required Evidence

| Evidence Area | Required Evidence | Focused Gate |
| --- | --- | --- |
| Publishable crate set | Confirmed publishable crates remain `dioxus-shadcn-core`, `dioxus-shadcn-primitives`, `dioxus-shadcn`, and `dioxus-shadcn-cli` | `npm run verify:cargo-publish-metadata` |
| Publish order | Confirmed dependency-first order remains core, primitives, styled crate, then CLI | `npm run verify:publish-order` |
| Internal dependency graph | Confirmed list of project-crate dependencies that must resolve after publish | `npm run verify:cargo-workspace` |
| Version policy | Approved version requirement policy for internal published crate dependencies | `npm run verify:workspace-dependency-publish-readiness` |
| Local development behavior | Confirmation that local workspace development remains supported after metadata changes | `npm run verify:cargo-workspace` |
| Release-owner boundary | Confirmation that packaging and publishing remain manual release-owner actions | `npm run verify:release-docs` |

## Validation Commands

After evidence is recorded and any approved local follow-up is implemented, run:

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

This checklist does not:

- change dependency versions
- rewrite manifests
- check crates.io ownership
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- authorize publishing
