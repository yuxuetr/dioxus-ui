# First Publish Maintainer Handoff Template

Use this template when maintainers are ready to decide whether the repository
can move from local release-candidate readiness to first publish preparation.
It is a copyable checklist, not a publish authorization.

Related source-of-truth documents:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)

## Release Candidate

- Candidate identifier:
- Git commit:
- Release owner:
- Review date:
- Decision state: `blocked` / `approved` / `implemented` / `verified` / `deferred`

## Required Maintainer Decisions

Repository identity:

- Final repository URL:
- Repository owner confirmed:
- Remote URL checked:
- Local follow-up owner:
- Decision state:

License files:

- `LICENSE-MIT` approved:
- `LICENSE-APACHE` approved:
- Copyright holder approved:
- Local follow-up owner:
- Decision state:

API stability:

- `0.1.x` first-publish API policy:
- Breaking-change policy:
- Migration note requirement:
- Local follow-up owner:
- Decision state:

Release notes:

- First publish release notes approved:
- Included change scope:
- Excluded change scope:
- Known warning notes:
- Local follow-up owner:
- Decision state:

CLI template packaging:

- Selected strategy: compile-time embedding
- Source-copy behavior reviewed:
- Install behavior reviewed:
- Local follow-up owner:
- Verification state:

Registry availability and ownership:

- crates.io crate names reviewed:
- Owners confirmed:
- Credentials ready:
- Publish order confirmed:
- Local follow-up owner:
- Decision state:

Workspace dependency publish readiness:

- Internal crate version metadata policy:
- Dependency publish order:
- Package dependency review owner:
- Local follow-up owner:
- Decision state:

## Local Follow-up Checklist

- Repository identity metadata updated if approved.
- Root license files committed only if approved.
- API stability metadata updated if approved.
- `CHANGELOG.md` and release notes metadata updated if approved.
- CLI template packaging readiness verified.
- Registry availability metadata updated if approved.
- Workspace dependency metadata updated if approved.
- Publish blocker inventory reflects only unresolved blockers.
- Decision matrix state updated if needed.
- `TODOs.md` updated only after validation and commits.

## Final Verification Commands

Run focused checks after local follow-up:

```bash
npm run verify:repository-identity-readiness
npm run verify:license-readiness
npm run verify:api-stability-readiness
npm run verify:release-notes-readiness
npm run verify:cli-template-packaging-readiness
npm run verify:registry-availability-readiness
npm run verify:workspace-dependency-publish-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:cargo-publish-metadata
npm run verify:publish-order
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
git diff --check
git status --short
```

Run the full local release gate before any publish decision:

```bash
npm run verify:release
```

Optional browser-local review remains separate:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:browser-local
```

## Non-goals

This template does not:

- run `cargo package`
- run `cargo publish`
- contact crates.io
- inspect credentials
- create package archives
- create Git tags
- publish GitHub releases
- authorize publishing
- replace maintainer approval
