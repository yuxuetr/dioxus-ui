# First Publish Decision Packet

Use this copyable packet when maintainers are ready to review the six current
first-publish blockers in one pass. It is not a publish authorization and does
not resolve blockers by itself.

Related documents:

- [First Publish Decision Packet Plan](first-publish-decision-packet-plan.md)
- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)

## Review Metadata

- Candidate identifier:
- Git commit:
- Review date:
- Release owner:
- Maintainer reviewers:
- Decision packet owner:
- Overall state: `blocked` / `approved` / `implemented` / `verified` / `deferred`
- Follow-up milestone:

## Decision Summary

| Blocker | State | Evidence Location | Local Follow-up Owner | Rollback Owner | Focused Gate |
| --- | --- | --- | --- | --- | --- |
| Placeholder repository URL |  |  |  |  | `npm run verify:repository-identity-readiness` |
| Root license files not committed |  |  |  |  | `npm run verify:license-readiness` |
| Pre-1.0 API stability |  |  |  |  | `npm run verify:api-stability-readiness` |
| Release notes not publish-ready |  |  |  |  | `npm run verify:release-notes-readiness` |
| Registry availability not checked |  |  |  |  | `npm run verify:registry-availability-readiness` |
| Workspace dependency publish readiness |  |  |  |  | `npm run verify:workspace-dependency-publish-readiness` |

## Blocker Decisions

### Placeholder Repository URL

Detailed handoff:
[Repository Identity Blocker Handoff](repository-identity-blocker-handoff.md)

- Decision state:
- Final repository owner:
- Canonical repository URL:
- Remote availability evidence:
- Metadata update approved: yes / no
- Local follow-up owner:
- Rollback owner:
- Required follow-up milestone:

### Root License Files

Detailed handoff:
[License Blocker Handoff](license-blocker-handoff.md)

- Decision state:
- `LICENSE` evidence:
- Copyright holder text:
- Root license file commit approved: yes / no
- Local follow-up owner:
- Rollback owner:
- Required follow-up milestone:

### Pre-1.0 API Stability

Detailed handoff:
[API Stability Blocker Handoff](api-stability-blocker-handoff.md)

- Decision state:
- First-publish API policy:
- Breaking-change policy before `1.0`:
- Migration note expectation:
- Known provisional APIs:
- Stabilization worklist:
- Local follow-up owner:
- Rollback owner:
- Required follow-up milestone:

### Release Notes Readiness

Detailed handoff:
[Release Notes Blocker Handoff](release-notes-blocker-handoff.md)

- Decision state:
- Release owner:
- Changelog owner:
- Included first-publish scope:
- Excluded scope:
- Known warning text:
- Migration notes required: yes / no
- Local follow-up owner:
- Rollback owner:
- Required follow-up milestone:

### Registry Availability

Detailed handoff:
[Registry Availability Blocker Handoff](registry-availability-blocker-handoff.md)

- Decision state:
- crates.io names reviewed:
- Owners confirmed:
- Credentials ready:
- Publish order confirmed:
- Release-owner evidence location:
- Local follow-up owner:
- Rollback owner:
- Required follow-up milestone:

### Workspace Dependency Publish Readiness

Detailed handoff:
[Workspace Dependency Blocker Handoff](workspace-dependency-blocker-handoff.md)

- Decision state:
- Publishable crate set confirmed:
- Dependency-first publish order confirmed:
- Internal dependency graph evidence:
- Internal dependency version policy:
- Local development behavior accepted:
- Local follow-up owner:
- Rollback owner:
- Required follow-up milestone:

## Local Follow-up Boundary

Start local follow-up only for blockers with recorded maintainer evidence and
an `approved` state. Keep every other blocker unresolved.

Local follow-up must not:

- infer repository ownership
- generate license text
- approve API stability without maintainer input
- generate release notes from Git history
- contact crates.io
- inspect credentials
- change dependency versions without approval
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts

## Final Verification

After any approved local follow-up, run the focused gate for the changed
blocker plus the shared publish readiness checks:

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

Run the full local release gate before any publish decision:

```bash
npm run verify:release
```
