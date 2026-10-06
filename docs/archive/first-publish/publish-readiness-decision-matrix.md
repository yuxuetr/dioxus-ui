# Publish Readiness Decision Matrix

This matrix turns the current publish readiness blockers into explicit
maintainer decisions. It is a handoff aid, not a publish authorization.

Use it with:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)
- [Publish Readiness Decision Handoff Plan](publish-readiness-decision-handoff-plan.md)
- [First Publish Decision Packet](first-publish-decision-packet.md)
- [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
- [Repository Identity Decision Record Template](repository-identity-decision-record-template.md)
- [Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md)
- [License Decision Preparation Plan](license-decision-preparation-plan.md)
- [License Decision Record Template](license-decision-record-template.md)
- [License Local Follow-up Map](license-local-follow-up-map.md)
- [Public API Surface Inventory](../../public-api-surface-inventory.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)

## Matrix

| Blocker | Required Decision | Evidence To Collect | Local Files To Update After Approval | Validation Commands |
| --- | --- | --- | --- | --- |
| Placeholder repository URL | Choose the final repository owner and URL | Final repository URL, owner confirmation, remote availability confirmation | `Cargo.toml`, `docs/repository-identity-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/cargo-publish-metadata.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:repository-identity-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:cargo-publish-metadata`; `npm run verify:release-docs`; `npm run verify:docs` |
| Root license files not committed | Approve exact license file and copyright holder | Maintainer-approved `LICENSE` and copyright text | `LICENSE`, `docs/license-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/cargo-publish-metadata.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:license-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:cargo-publish-metadata`; `npm run verify:release-docs`; `npm run verify:docs` |
| Pre-1.0 API stability | Decide whether `0.1.x` APIs are acceptable for first publish | Stability decision, public API surface inventory, breaking-change policy, migration note expectation | `docs/api-stability-readiness-metadata.md`, `docs/public-api-surface-inventory.md`, `docs/publish-readiness-blockers.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `CHANGELOG.md`, `TODOs.md` | `npm run verify:api-stability-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:release-docs`; `npm run verify:changelog`; `npm run verify:docs` |
| Release notes not publish-ready | Approve first publish release notes scope | Maintainer-approved release notes, included changes, excluded changes, known warnings | `CHANGELOG.md`, `docs/release-notes-readiness-metadata.md`, `docs/changelog-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:release-notes-readiness`; `npm run verify:changelog`; `npm run verify:publish-readiness-blockers`; `npm run verify:release-docs`; `npm run verify:docs` |
| CLI template packaging strategy | Resolved with compile-time embedding | Embedded registry/template catalog, source-copy behavior, generated fixture smoke | `crates/dioxus-shadcn-cli/build.rs`, `crates/dioxus-shadcn-cli/src/main.rs`, `docs/cli-template-packaging-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:cli-template-packaging-readiness`; `cargo test -p dioxus-shadcn-cli`; `scripts/generated-fixture-smoke.sh`; `npm run verify:registry`; `npm run verify:docs` |
| Registry availability not checked | Confirm crates.io names, ownership, credentials, and publish order | crates.io name review, owner list, token/credential readiness, publish order confirmation | `docs/registry-availability-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/cargo-publish-metadata.md`, `docs/publish-order-metadata.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:registry-availability-readiness`; `npm run verify:publish-order`; `npm run verify:publish-readiness-blockers`; `npm run verify:cargo-publish-metadata`; `npm run verify:docs` |
| Workspace dependency publish readiness | Approve crates.io-resolvable internal dependency metadata | Internal crate version metadata policy, dependency publish order, package dependency review | `Cargo.toml`, `crates/*/Cargo.toml`, `docs/workspace-dependency-publish-readiness-metadata.md`, `docs/publish-order-metadata.md`, `docs/cargo-publish-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:workspace-dependency-publish-readiness`; `npm run verify:publish-order`; `npm run verify:cargo-workspace`; `npm run verify:cargo-publish-metadata`; `npm run verify:docs` |

## Decision States

Use these states when recording handoff notes:

| State | Meaning |
| --- | --- |
| `blocked` | Maintainer input is missing. Do not implement local follow-up yet. |
| `approved` | Maintainer input is recorded and local follow-up can start. |
| `implemented` | Local follow-up files are updated but focused gates have not all passed. |
| `verified` | Focused gates pass and the blocker inventory has been updated. |
| `deferred` | Maintainers intentionally postpone the blocker and publishing remains blocked. |

## Local Safety Rules

- Do not resolve a blocker based only on this matrix.
- Do not infer repository ownership, copyright holders, registry ownership, or
  release scope.
- Do not run publish or package commands as part of matrix upkeep.
- Keep blocker metadata unresolved until the approved local follow-up is
  implemented and verified.
- After any blocker is resolved, rerun `npm run verify:release` before a
  release-candidate handoff.
