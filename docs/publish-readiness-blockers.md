# Publish Readiness Blockers

M97 defines the read-only contract for known publish blockers. The project now
has Cargo publish metadata, but metadata completeness is not the same as publish
readiness.

Use [First Publish Readiness Plan](first-publish-readiness-plan.md) for the
repository-safe planning cycle before any maintainer-approved blocker
resolution work starts.
Use [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
to record the minimum evidence required before local follow-up can begin.

Current blockers:

| Blocker | Evidence | Resolution Owner |
| --- | --- | --- |
| Placeholder repository URL | `https://github.com/your-org/dioxus-ui` in workspace package metadata | Maintainer updates release identity before publishing |
| Pre-1.0 API stability | Release docs allow breaking API changes before `1.0` | Maintainers decide crate-mode stability and versioning policy |
| Release notes not publish-ready | `CHANGELOG.md` has project-owned structure, but release notes have not been maintained as complete publish-ready history | Maintainers define and maintain release notes before publishing |
| Root license files not committed | `MIT OR Apache-2.0` is declared in workspace metadata, but `LICENSE-MIT` and `LICENSE-APACHE` are not committed | Maintainer commits reviewed root license files before publishing |
| Registry availability not checked | Cargo publish metadata gate intentionally avoids crates.io lookups | Release owner checks names and ownership during publish preparation |
| Workspace dependency publish readiness | Path-only internal workspace dependencies do not yet prove crates.io-resolvable dependency metadata | Maintainers add or approve publish-ready internal dependency version metadata |

Resolved publish readiness items:

| Item | Evidence | Verification |
| --- | --- | --- |
| CLI template packaging strategy | `dioxus-ui-cli` embeds registry and template assets at compile time | `npm run verify:cli-template-packaging-readiness` |

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
- generating license text
- changing embedded CLI template delivery
- changing dependency versions

## Expected Check

The verifier should fail when committed blocker metadata drifts. Examples
include:

- publish metadata docs stop saying the crates are not ready to publish
- release docs imply CLI template delivery still depends on repository layout
- workspace docs omit the placeholder repository URL blocker
- package scripts stop running the blocker inventory gate during release

This gate should keep publish blockers explicit until a maintainer intentionally
resolves them. It does not resolve the blockers.
