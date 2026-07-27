# First Publish Decision Packet Plan

This plan defines a repository-safe maintainer decision packet for the six
current first-publish blockers. It is a planning artifact, not a publish
authorization.

Use it with:

- [First Publish Decision Packet](first-publish-decision-packet.md)
- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)

## Packet Goal

The decision packet should give release owners one copyable place to record:

- blocker state
- required maintainer evidence
- evidence location
- local follow-up owner
- rollback owner
- focused validation gates
- final release-owner boundary

The packet should link back to the detailed blocker handoff documents rather
than duplicating every detail.

## Covered Blockers

The packet covers these unresolved blockers:

| Blocker | Detailed Handoff |
| --- | --- |
| Placeholder repository URL | [Repository Identity Blocker Handoff](repository-identity-blocker-handoff.md) |
| Root license files not committed | [License Blocker Handoff](license-blocker-handoff.md) |
| Pre-1.0 API stability | [API Stability Blocker Handoff](api-stability-blocker-handoff.md) |
| Release notes not publish-ready | [Release Notes Blocker Handoff](release-notes-blocker-handoff.md) |
| Registry availability not checked | [Registry Availability Blocker Handoff](registry-availability-blocker-handoff.md) |
| Workspace dependency publish readiness | [Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md) |

## Decision States

Use the same states as the decision matrix:

| State | Meaning |
| --- | --- |
| `blocked` | Required evidence is missing; no local follow-up starts. |
| `approved` | Maintainer evidence is recorded; local follow-up can start. |
| `implemented` | Local files changed, but focused gates are not complete. |
| `verified` | Focused gates passed and blocker metadata was updated. |
| `deferred` | Publishing remains blocked by an intentional follow-up milestone. |

## Required Links

The final packet should be discoverable from:

- publish blocker tracker
- decision matrix
- release candidate handoff checklist
- first publish maintainer handoff template
- README
- docs index

## Non-goals

This plan does not:

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
- create tags or release artifacts

## Validation

Use read-only checks:

```bash
npm run verify:docs
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
```
