# Release Notes Readiness Preparation Plan

This plan prepares the maintainer decision for first-publish release notes. It
does not generate release notes, derive changes from Git history, create tags,
or authorize publishing.

Use this plan with:

- [Release Notes Readiness Metadata](release-notes-readiness-metadata.md)
- [Release Notes Evidence Checklist](release-notes-evidence-checklist.md)
- [Release Notes Local Follow-up Map](release-notes-local-follow-up-map.md)
- [Changelog Metadata](changelog-metadata.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)

## Decision Question

Maintainers need to decide what the first publish release notes should include
and which known warnings should be carried into release-candidate handoff.

The decision must cover:

- release owner
- changelog owner
- included change scope
- excluded change scope
- known warning text
- whether `CHANGELOG.md` should become publish-ready or remain only
  structurally owned
- whether any migration note is required after API stability decisions

## Current Local Shape

`CHANGELOG.md` is project-owned and structurally checked. It has an
`Unreleased` section and does not carry stale template history.

That structure is not the same as complete first-publish release notes. The
release notes readiness blocker remains unresolved until maintainers approve the
first-publish release note scope and local follow-up.

## Preparation Pass

Prepare the decision in this order:

1. Confirm `CHANGELOG.md` still passes project-owned structure checks.
2. Confirm release notes readiness metadata still describes the blocker as
   unresolved.
3. Record included and excluded change categories for first publish.
4. Record known warnings that should appear in release-candidate handoff using
   the release notes evidence checklist.
5. Record whether API stability decisions require migration notes.
6. Only after approval, update changelog/release-note docs and blocker metadata
   together using the release notes local follow-up map.

## Decision Outcomes

| Outcome | Meaning | Local Follow-up |
| --- | --- | --- |
| `approved` | Maintainers approve first-publish release note scope | Update `CHANGELOG.md` if required, release notes readiness metadata, changelog metadata, publish blockers, release docs, quality gates, README, docs-site notes, and TODO planning |
| `blocked` | Required release scope or warning evidence is missing | Keep blocker unresolved and record missing evidence |
| `deferred` | Release notes need a focused drafting milestone before blocker resolution | Create a release notes drafting TODO milestone without creating tags or releases |

## Non-goals

M128 must not:

- generate final release notes
- derive changes from Git history
- run git-cliff
- rewrite commit history
- create Git tags
- create GitHub releases
- create package archives
- run `cargo package`
- run `cargo publish`
- authorize publishing

## Safe Validation

Use read-only checks while preparing decision artifacts:

```bash
npm run verify:release-notes-readiness
npm run verify:changelog
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
```

Run `npm run verify:release` only for final local release-candidate confidence.
It still must not package, publish, create tags, create GitHub releases, or
generate release artifacts.

## Exit Criteria

This preparation pass is complete when:

- first-publish release note decision inputs are explicit
- changelog structure ownership is separate from publish-ready note completeness
- release notes readiness remains unresolved until maintainer approval is
  recorded
- no generated release notes, Git history derivation, git-cliff output, tags,
  GitHub releases, package archives, publish commands, screenshots, traces, or
  CI workflow files are committed
