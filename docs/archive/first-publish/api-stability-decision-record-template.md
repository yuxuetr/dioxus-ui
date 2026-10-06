# API Stability Decision Record Template

Use this template when maintainers are ready to record the pre-`1.0` API
stability decision for first publish preparation. It is a decision record
template, not an approval by itself.

Related documents:

- [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md)
- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- [Public API Surface Inventory](../../public-api-surface-inventory.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)
- [API Stability Local Follow-up Map](api-stability-local-follow-up-map.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Decision Metadata

- Candidate commit:
- Review date:
- Review owner:
- Decision state: `approved` / `blocked` / `deferred`
- Local follow-up owner:
- Required follow-up milestone:

## Surface Review

Component crate API:

- Component names accepted:
- Prop names accepted:
- Variant and size enums accepted:
- Public class helpers accepted:
- Required changes:

Feature and source-copy API:

- Feature names accepted:
- Registry slugs accepted:
- Source-copy filenames accepted:
- Generated target paths accepted:
- Template API differences accepted:
- Required changes:

Primitive and core API:

- Primitive helper names accepted:
- Primitive state/config structs accepted:
- Runtime adapter traits accepted:
- Core class and registry exports accepted:
- Required changes:

Documentation API:

- README examples accepted:
- Component docs accepted:
- Source preview routes accepted:
- Release docs wording accepted:
- Required changes:

## Policy Evidence

- First-publish API policy:
- Breaking-change policy before `1.0`:
- Migration note expectation:
- Changelog expectation:
- Known provisional APIs:
- Deferred stabilization work:

## Decision Outcome

Choose exactly one:

- `approved`: current `0.1.x` APIs are acceptable for first publish after local
  follow-up updates blocker metadata and gates.
- `blocked`: required evidence is missing; keep blocker unresolved.
- `deferred`: first publish needs a focused API stabilization milestone before
  blocker resolution.

Rationale:

## Local Follow-up

If approved:

- Update API stability readiness metadata.
- Update publish readiness blockers.
- Update publish readiness coverage and runbook docs.
- Update release docs, quality gates, README, and docs-site notes.
- Update changelog metadata or `CHANGELOG.md` only if release notes change.
- Update `TODOs.md` after implementation, validation, and commit.

If blocked or deferred:

- Keep the API stability blocker unresolved.
- Record missing evidence or required API changes.
- Create a follow-up TODO milestone if API changes are required.

## Validation Commands

```bash
npm run verify:api-stability-readiness
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

Run `npm run verify:release` before release-candidate handoff.

## Non-goals

This template does not:

- approve API stability without maintainer input
- rewrite public APIs
- change versions
- generate migration guides
- generate release notes
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts
