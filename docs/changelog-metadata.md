# Changelog Metadata

Project-owned changelog structure is the release note baseline for this
repository.
First-publish release note readiness preparation is tracked in
[Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md).
Maintainer evidence for publish-ready notes is tracked in
[Release Notes Evidence Checklist](release-notes-evidence-checklist.md).

M98 defines the read-only contract for the project changelog. The repository
already has `CHANGELOG.md`, but it must be owned by `dioxus-ui` rather than
carrying template release history.

## Expected Shape

`CHANGELOG.md` should keep:

- `# Changelog`
- a short Keep a Changelog style introduction
- a link to Conventional Commits
- an `## [Unreleased]` section, also referred to as the Unreleased section
- `### Added`, `### Changed`, and `### Fixed` subsections under Unreleased
- a note that generated release notes are not produced by the metadata gate

The changelog must not keep stale template links such as:

```text
github.com/yuxuetr/rust-template
```

## Scope

In scope:

- changelog ownership and structure
- stale template-link detection, including stale template-link bans
- release docs and quality gate references
- publish blocker alignment
- package script and release aggregate wiring

Out of scope:

- generating release notes
- running git-cliff
- deriving changes from Git history
- rewriting commit history
- creating Git tags
- publishing crates or GitHub releases

## Expected Check

The verifier should fail when committed changelog metadata drifts. Examples
include:

- `CHANGELOG.md` loses its Unreleased section
- stale template repository links reappear
- release docs stop mentioning changelog requirements for breaking changes
- publish blocker docs stop tracking changelog ownership
- release verification stops running the changelog metadata gate

This gate should make changelog ownership explicit. It validates changelog
ownership and structure, not publish-ready release note completeness.
