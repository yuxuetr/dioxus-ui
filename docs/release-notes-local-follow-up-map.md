# Release Notes Local Follow-up Map

This map describes local follow-up after maintainers approve first-publish
release note evidence. It is hypothetical until evidence is recorded and a
maintainer decision is committed.

Use this map with:

- [Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md)
- [Release Notes Evidence Checklist](release-notes-evidence-checklist.md)
- [Release Notes Readiness Metadata](release-notes-readiness-metadata.md)
- [Changelog Metadata](changelog-metadata.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Outcome Map

| Decision | Local Files | Validation | Notes |
| --- | --- | --- | --- |
| `approved` with changelog updates | `CHANGELOG.md`, `docs/release-notes-readiness-metadata.md`, `docs/changelog-metadata.md`, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:release-notes-readiness`; `npm run verify:changelog`; `npm run verify:publish-readiness-blockers`; `npm run verify:release-docs`; `npm run verify:docs` | Move release notes out of current blockers only after approved content and gates agree |
| `approved` with separate handoff notes | Release candidate handoff docs, release notes metadata, publish readiness docs, release docs, quality gates, README, docs-site notes, `TODOs.md` | `npm run verify:release-notes-readiness`; `npm run verify:release-candidate-handoff`; `npm run verify:publish-readiness-blockers`; `npm run verify:docs` | Keep `CHANGELOG.md` structural if maintainers choose another approved release note surface |
| `blocked` | Evidence notes or TODO planning only | `npm run verify:release-notes-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:docs` | Keep blocker unresolved and preserve current changelog structure |
| `deferred` | `TODOs.md`, release notes drafting plan, optional changelog planning docs | `npm run verify:release-notes-readiness`; `npm run verify:changelog`; `npm run verify:docs`; `git diff --check` | Create a focused release notes drafting milestone before resolving publish readiness |

## Changelog Boundary

Changelog ownership and publish-ready release note completeness must remain
separate:

- `CHANGELOG.md` structure can be valid while release notes remain incomplete
- generated notes from Git history are out of scope for metadata gates
- release notes scope must come from maintainer approval
- tags, GitHub releases, package archives, and publish commands remain separate
  release-owner actions

## Common Validation

After approved local follow-up, run:

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
git status --short
```

## Non-goals

This map does not:

- write final release notes
- approve release contents
- derive changes from Git history
- run git-cliff
- rewrite commit history
- create tags
- create GitHub releases
- create package archives
- run `cargo package`
- run `cargo publish`
