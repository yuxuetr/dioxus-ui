# Publish Blocker Resolution Tracker

This tracker keeps the six first-publish blockers in one place while they move
from planning to maintainer handoff and then to local follow-up. It is not a
publish authorization.

Use it with:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Decision Packet](first-publish-decision-packet.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)

## Status

| Blocker | Current Status | Existing Preparation | Required Evidence | Local Follow-up After Approval | Focused Gate |
| --- | --- | --- | --- | --- | --- |
| Placeholder repository URL | Resolved locally | [Repository Identity Blocker Handoff](repository-identity-blocker-handoff.md), [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md), [Repository Identity Decision Record Template](repository-identity-decision-record-template.md), [Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md) | Canonical repository URL, repository owner, remote availability evidence, metadata update approval | Update workspace repository metadata and readiness docs | `npm run verify:repository-identity-readiness` |
| Root license files not committed | Resolved locally | [License Blocker Handoff](license-blocker-handoff.md), [License Decision Preparation Plan](license-decision-preparation-plan.md), [License Decision Record Template](license-decision-record-template.md), [License Local Follow-up Map](license-local-follow-up-map.md) | Approved `LICENSE`, copyright holder text, license expression confirmation, file commit approval | Commit approved root license file and update license readiness docs | `npm run verify:license-readiness` |
| Pre-1.0 API stability | Resolved locally | [API Stability Blocker Handoff](api-stability-blocker-handoff.md), [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md), [API Stability Decision Record Template](api-stability-decision-record-template.md), [API Stability Local Follow-up Map](api-stability-local-follow-up-map.md) | Accept current `0.1.x` API policy or provide a stabilization worklist, breaking-change policy, migration note expectation | Update API stability metadata, public surface docs, release docs, and publish blockers | `npm run verify:api-stability-readiness` |
| Release notes not publish-ready | Resolved locally | [Release Notes Blocker Handoff](release-notes-blocker-handoff.md), [Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md), [Release Notes Evidence Checklist](release-notes-evidence-checklist.md), [Release Notes Local Follow-up Map](release-notes-local-follow-up-map.md) | First-publish release note scope, included changes, excluded changes, known warnings, changelog owner | Update `CHANGELOG.md`, release notes metadata, and release docs | `npm run verify:release-notes-readiness` |
| Registry availability not checked | Deferred | [Registry Availability Blocker Handoff](registry-availability-blocker-handoff.md), [Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md), [Publish Order Metadata](publish-order-metadata.md), [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md) | crates.io names, owner list, credential readiness, publish order confirmation, release owner | Update registry availability metadata, publish order metadata, Cargo publish metadata, and blocker docs | `npm run verify:registry-availability-readiness` |
| Workspace dependency publish readiness | Resolved locally | [Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md), [Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md), [Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md), [Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md) | Internal crate version policy, crates.io-resolvable dependency plan, publish order dependency review, package owner | Update crate manifests, workspace dependency metadata, publish order metadata, and blocker docs | `npm run verify:workspace-dependency-publish-readiness` |

## Execution Order

The blockers can be prepared in parallel, but local resolution should stay
sequenced because several updates touch shared release docs and publish
metadata.

1. Repository identity: decide the canonical URL before publishing metadata is
   treated as final.
2. License files: commit reviewed root license files before package metadata is
   considered complete.
3. API stability: decide whether current `0.1.x` crate-mode APIs are acceptable
   or need stabilization work.
4. Release notes: finalize publish-ready notes after API and scope decisions are
   known.
5. Registry availability: check crate names, ownership, and credentials during
   release-owner preparation.
6. Workspace dependencies: apply approved internal dependency version metadata
   before package or publish dry runs.

## Shared Validation

After any approved blocker follow-up is implemented, run the focused gate and
the shared release-readiness checks:

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

Use `npm run verify:release` for a full local release-candidate audit after the
blockers have maintainer decisions. This tracker does not require that command
for planning-only changes.

## Non-goals

This tracker does not:

- replace repository URLs
- create root license files
- approve API stability
- generate release notes
- contact crates.io
- inspect credentials
- change dependency versions
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags
- activate CI workflows
