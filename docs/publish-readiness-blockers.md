# Publish Readiness Blockers

M97 defines the read-only contract for known publish blockers. The project now
has Cargo publish metadata, but metadata completeness is not the same as publish
readiness.

Current blockers:

| Blocker | Evidence | Resolution Owner |
| --- | --- | --- |
| Placeholder repository URL | `https://github.com/your-org/dioxus-ui` in workspace package metadata | Maintainer updates release identity before publishing |
| Pre-1.0 API stability | Release docs allow breaking API changes before `1.0` | Maintainers decide crate-mode stability and versioning policy |
| Changelog not yet release-owned | Release docs require breaking changes to be documented in the changelog | Maintainers define and maintain release notes before publishing |
| CLI template packaging strategy | CLI release notes still say templates are read from the repository layout | CLI owner embeds templates or packages them in a stable install location |
| Registry availability not checked | Cargo publish metadata gate intentionally avoids crates.io lookups | Release owner checks names and ownership during publish preparation |

## Scope

In scope:

- keeping the blocker inventory discoverable
- checking the placeholder repository URL is still documented as a blocker
- checking release docs and publish metadata docs do not imply readiness
- checking package script and release aggregate wiring

Out of scope:

- replacing repository URLs
- checking crates.io name availability
- running `cargo package`
- running `cargo publish`
- stabilizing component APIs
- generating changelogs or release notes
- packaging CLI templates

## Expected Check

The verifier should fail when committed blocker metadata drifts. Examples
include:

- publish metadata docs stop saying the crates are not ready to publish
- release docs omit the CLI template packaging blocker
- workspace docs omit the placeholder repository URL blocker
- package scripts stop running the blocker inventory gate during release

This gate should keep publish blockers explicit until a maintainer intentionally
resolves them. It does not resolve the blockers.
