# Repository Identity Decision Record Template

Use this template when maintainers are ready to record the canonical repository
identity decision for first publish preparation. It is a decision record
template, not an approval by itself.

Related documents:

- [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
- [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md)
- [Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md)
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

## Repository Evidence

- Final repository owner:
- Canonical repository URL:
- Remote availability confirmed by:
- Remote availability evidence location:
- Workspace metadata update approved: yes / no
- Crate manifest inheritance accepted: yes / no
- Docs wording accepted:
- Rollback expectation if identity changes before publish:

## Decision Outcome

Choose exactly one:

- `approved`: the canonical repository owner and URL are accepted for first
  publish after local follow-up updates metadata and gates.
- `blocked`: required evidence is missing; keep the placeholder URL and
  unresolved blocker.
- `deferred`: first publish needs a focused repository ownership milestone
  before blocker resolution.

Rationale:

## Local Follow-up

If approved:

- Replace the placeholder repository URL in workspace metadata.
- Update repository identity readiness metadata.
- Update publish readiness blockers.
- Update Cargo publish metadata.
- Update release docs, quality gates, README, and docs-site notes.
- Update `TODOs.md` after implementation, validation, and commit.

If blocked or deferred:

- Keep the placeholder repository URL.
- Keep the repository identity blocker unresolved.
- Record missing evidence or required ownership work.
- Create a follow-up TODO milestone if repository ownership work is required.

## Validation Commands

```bash
npm run verify:repository-identity-readiness
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

- choose repository ownership without maintainer input
- replace repository metadata
- check remote repository existence
- check crates.io availability
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts
