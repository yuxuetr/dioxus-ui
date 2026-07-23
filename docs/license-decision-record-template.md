# License Decision Record Template

Use this template when maintainers are ready to record the root license file
decision for first publish preparation. It is a decision record template, not
an approval by itself.

Related documents:

- [License Decision Preparation Plan](license-decision-preparation-plan.md)
- [License Readiness Metadata](license-readiness-metadata.md)
- [License Local Follow-up Map](license-local-follow-up-map.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)

## Decision Metadata

- Candidate commit:
- Review date:
- Review owner:
- Decision state: `approved` / `blocked` / `deferred`
- Local follow-up owner:
- Required follow-up milestone:

## License Evidence

- Workspace license expression accepted:
- `LICENSE-MIT` text approved by:
- `LICENSE-MIT` evidence location:
- `LICENSE-APACHE` text approved by:
- `LICENSE-APACHE` evidence location:
- Copyright holder text:
- Root license file commit approved: yes / no
- Crate manifest inheritance accepted: yes / no
- Docs wording accepted:
- Rollback expectation if license ownership changes before publish:

## Decision Outcome

Choose exactly one:

- `approved`: root license files and copyright holder text are accepted for
  first publish after local follow-up updates metadata and gates.
- `blocked`: required evidence is missing; keep root license files absent and
  the blocker unresolved.
- `deferred`: first publish needs a focused legal or ownership milestone before
  blocker resolution.

Rationale:

## Local Follow-up

If approved:

- Commit maintainer-approved `LICENSE-MIT`.
- Commit maintainer-approved `LICENSE-APACHE`.
- Update license readiness metadata.
- Update publish readiness blockers.
- Update Cargo publish metadata.
- Update release docs, quality gates, README, and docs-site notes.
- Update `TODOs.md` after implementation, validation, and commit.

If blocked or deferred:

- Keep root license files absent.
- Keep the license readiness blocker unresolved.
- Record missing evidence or required legal/ownership work.
- Create a follow-up TODO milestone if license ownership work is required.

## Validation Commands

```bash
npm run verify:license-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
git status --short
```

Run `npm run verify:release` before release-candidate handoff.

## Non-goals

This template does not:

- choose license terms without maintainer input
- generate license text
- commit license files
- change copyright holders
- change workspace license metadata
- run `cargo package`
- run `cargo publish`
- contact crates.io
- create package archives
- create tags or release artifacts
