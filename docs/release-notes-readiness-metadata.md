# Release Notes Readiness Metadata

M99 refines the publish readiness blocker language after M98. The project has
a project-owned `CHANGELOG.md` structure, and M133 records the first publish
release note scope in its Unreleased section, resolving release notes
readiness locally.
The decision preparation plan is tracked in
[Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md).
The maintainer evidence checklist is tracked in
[Release Notes Evidence Checklist](release-notes-evidence-checklist.md).
The local follow-up map is tracked in
[Release Notes Local Follow-up Map](release-notes-local-follow-up-map.md).
The consolidated first-publish release scope and rollback view is tracked in
[Release Notes Blocker Handoff](release-notes-blocker-handoff.md).

## Expected Shape

Publish readiness docs should distinguish these states:

- `CHANGELOG.md` is project-owned and structurally checked.
- The Unreleased section carries the first publish (`0.1.0`) release notes.
- First publish notes record included scope, excluded scope, and known
  warnings.

The first publish notes are written from the component catalog and maintainer
decisions. They are not derived from Git history, and recording them does not
create tags, GitHub releases, or package archives.

## Scope

In scope:

- publish blocker wording
- first publish release note scope in `CHANGELOG.md`
- release and quality gate summaries
- changelog metadata alignment
- docs-site history notes
- package script and release aggregate wiring for existing gates

Out of scope:

- generating release notes
- deriving changes from Git history
- running git-cliff
- creating Git tags
- publishing crates or GitHub releases

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- publish blockers return to saying the changelog is not project-owned
- `CHANGELOG.md` loses its excluded scope or known warnings sections
- release docs stop describing the recorded first publish release note scope
- changelog metadata docs stop saying the gate validates structure, not
  publish-ready note completeness

This gate should keep first publish release note readiness explicit after
resolution.
