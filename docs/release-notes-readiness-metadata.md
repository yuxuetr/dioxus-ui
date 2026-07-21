# Release Notes Readiness Metadata

M99 refines the publish readiness blocker language after M98. The project now
has a project-owned `CHANGELOG.md` structure, so the remaining blocker is not
changelog ownership. The remaining blocker is release notes readiness.
The decision preparation plan is tracked in
[Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md).
The maintainer evidence checklist is tracked in
[Release Notes Evidence Checklist](release-notes-evidence-checklist.md).

## Expected Shape

Publish readiness docs should distinguish these states:

- `CHANGELOG.md` is project-owned and structurally checked.
- Release notes are not yet maintained as complete publish-ready history.
- The release notes blocker remains unresolved until maintainers prepare
  release notes for a real crate release.

The wording should avoid claiming that the changelog is not project-owned after
M98.

## Scope

In scope:

- publish blocker wording
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
- deciding release contents

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- publish blockers return to saying the changelog is not project-owned
- release docs omit the release notes readiness blocker
- quality gates stop describing the blocker as release notes readiness
- changelog metadata docs stop saying the gate validates structure, not
  publish-ready note completeness

This gate should keep release notes readiness explicit without resolving it.
