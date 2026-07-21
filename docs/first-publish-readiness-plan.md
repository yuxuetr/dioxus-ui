# First Publish Readiness Plan

This document defines the repository-safe planning cycle for resolving the
remaining first publish blockers. It is a planning artifact, not a publish
authorization.

Use this plan with:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)

## Current Blockers

The remaining blockers are:

1. Placeholder repository URL
2. Root license files not committed
3. Pre-1.0 API stability
4. Release notes not publish-ready
5. Registry availability not checked
6. Workspace dependency publish readiness

The CLI template packaging strategy is already resolved through compile-time
embedded registry and template assets.

## Resolution Cycle

Resolve blockers in maintainer-approved order:

| Step | Blocker | Maintainer Decision | Local Follow-up After Approval |
| --- | --- | --- | --- |
| 1 | Placeholder repository URL | Final repository owner and canonical URL | Update workspace metadata and readiness docs, then rerun repository identity and publish readiness gates |
| 2 | Root license files not committed | Exact root license files and copyright holder | Commit approved license files and update license readiness metadata |
| 3 | Pre-1.0 API stability | Accept `0.1.x` first-publish API policy or require stabilization work using [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md) | Update API stability metadata, public surface inventory, changelog guidance, and release docs |
| 4 | Release notes not publish-ready | First publish release-note scope and known warning text using [Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md) | Update `CHANGELOG.md`, release notes metadata, and release docs |
| 5 | Registry availability not checked | crates.io names, owners, credentials, and publish order | Update registry availability metadata, publish order metadata, and Cargo publish metadata |
| 6 | Workspace dependency publish readiness | Internal crate dependency version policy using [Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md) | Update crate manifests, workspace dependency metadata, and publish order metadata |

## Decision Boundaries

M125 may document the order, required decisions, evidence, local files, and safe
validation commands. It must not:

- replace repository URLs
- create or modify root license text
- decide API stability policy
- generate final release notes
- contact crates.io
- inspect credentials
- change dependency versions
- run `cargo package`
- run `cargo publish`
- create package archives
- create Git tags
- activate CI workflows

## Local Validation

Before and after first-publish planning changes, use read-only checks:

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

Use `npm run verify:release` only for a full local release-candidate audit. It
still must not package, publish, contact crates.io, create tags, or generate
release artifacts.

## Exit Criteria

This planning cycle is complete when:

- every remaining blocker has a maintainer decision boundary
- every blocker separates required evidence from local implementation follow-up
- every blocker lists read-only validation commands
- approved local follow-up files and rollback considerations are mapped
- resolved CLI template packaging remains separate from current blockers
- publish readiness docs do not imply publishing is authorized
- repository hygiene checks prove no package, publish, tag, screenshot, trace,
  license, repository URL, version, or CI workflow artifacts were committed
