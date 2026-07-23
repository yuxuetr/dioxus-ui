# First Publish Local Implementation Map

This map describes the local file changes expected after maintainers approve a
first-publish blocker decision. It is hypothetical until the required evidence
is recorded in the blocker evidence checklist.

Use this map with:

- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)

## Implementation Map

| Approved Decision | Expected Local Files | Required Validation | Rollback Consideration |
| --- | --- | --- | --- |
| Repository identity | `Cargo.toml`, crate manifests if they stop inheriting workspace metadata, `docs/repository-identity-readiness-metadata.md`, `docs/repository-identity-decision-preparation-plan.md`, `docs/repository-identity-decision-record-template.md`, `docs/repository-identity-local-follow-up-map.md`, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:repository-identity-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:cargo-publish-metadata`; `npm run verify:release-docs`; `npm run verify:docs` | Restore the placeholder URL and blocker docs if maintainer approval is withdrawn before publish |
| Root license files | `LICENSE-MIT`, `LICENSE-APACHE`, `docs/license-readiness-metadata.md`, publish readiness docs, Cargo publish metadata, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:license-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:cargo-publish-metadata`; `npm run verify:release-docs`; `npm run verify:docs` | Remove unapproved license files and restore unresolved license blocker metadata |
| API stability policy | `docs/api-stability-readiness-metadata.md`, `docs/public-api-surface-inventory.md`, `docs/api-stability-review-checklist.md`, `docs/api-stability-decision-record-template.md`, `docs/api-stability-local-follow-up-map.md`, `CHANGELOG.md` if migration notes are approved, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:api-stability-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:release-docs`; `npm run verify:changelog`; `npm run verify:docs` | Revert stability claims and keep APIs documented as pre-1.0 if policy approval is withdrawn |
| Release notes readiness | `CHANGELOG.md`, `docs/release-notes-readiness-metadata.md`, `docs/release-notes-readiness-preparation-plan.md`, `docs/release-notes-evidence-checklist.md`, `docs/release-notes-local-follow-up-map.md`, `docs/changelog-metadata.md`, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:release-notes-readiness`; `npm run verify:changelog`; `npm run verify:publish-readiness-blockers`; `npm run verify:release-docs`; `npm run verify:docs` | Restore draft/unreleased notes and unresolved release-notes blocker metadata |
| Registry availability and ownership | `docs/registry-availability-readiness-metadata.md`, `docs/publish-order-metadata.md`, `docs/cargo-publish-metadata.md`, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:registry-availability-readiness`; `npm run verify:publish-order`; `npm run verify:publish-readiness-blockers`; `npm run verify:cargo-publish-metadata`; `npm run verify:docs` | Restore registry review as unresolved if names, owners, credentials, or order are not confirmed |
| Workspace dependency publish readiness | `Cargo.toml`, `crates/*/Cargo.toml`, `docs/workspace-dependency-publish-readiness-metadata.md`, `docs/workspace-dependency-publish-readiness-preparation-plan.md`, `docs/workspace-dependency-evidence-checklist.md`, `docs/workspace-dependency-local-follow-up-map.md`, `docs/publish-order-metadata.md`, `docs/cargo-publish-metadata.md`, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:workspace-dependency-publish-readiness`; `npm run verify:publish-order`; `npm run verify:cargo-workspace`; `npm run verify:cargo-publish-metadata`; `npm run verify:docs` | Restore path-only dependency blocker metadata if version policy or publish order is not approved |

## Common Follow-up

Every approved blocker resolution should also update:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Publish Readiness Coverage Metadata](publish-readiness-coverage-metadata.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md), if the
  change affects release-candidate evidence

Run common checks after the focused blocker gates:

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
git status --short
```

## Non-goals

This map does not:

- apply any listed file changes
- approve maintainer decisions
- contact registries
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- create Git tags
- activate CI workflows
