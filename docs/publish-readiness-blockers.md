# Publish Readiness Blockers

M97 defines the read-only contract for known publish blockers. The project now
has Cargo publish metadata, but metadata completeness is not the same as publish
readiness.

Use [First Publish Readiness Plan](first-publish-readiness-plan.md) for the
repository-safe planning cycle before any maintainer-approved blocker
resolution work starts.
Use [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
to record the minimum evidence required before local follow-up can begin.
Use [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
to track current blockers together while evidence and local follow-up are
prepared.

Current blockers:

| Blocker | Evidence | Resolution Owner |
| --- | --- | --- |
| Registry availability not checked | Cargo publish metadata gate intentionally avoids crates.io lookups | Release owner checks names and ownership during publish preparation |

Registry availability is deferred until a release owner supplies the crates.io
evidence listed in
[Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md#deferral).
Deferral blocks crates.io publishing; it does not block local release
readiness or internal trial.

Resolved publish readiness items:

| Item | Evidence | Verification |
| --- | --- | --- |
| Placeholder repository URL | Workspace metadata uses `https://github.com/yuxuetr/dioxus-ui` | `npm run verify:repository-identity-readiness` |
| Root license files not committed | `LICENSE` contains reviewed MIT license text | `npm run verify:license-readiness` |
| Pre-1.0 API stability | Current `0.1.x` API surface is accepted for first publish; breaking changes before `1.0` require a minor bump and a changelog migration note | `npm run verify:api-stability-readiness` |
| Release notes not publish-ready | `CHANGELOG.md` has project-owned structure and its Unreleased section records first publish included scope, excluded scope, and known warnings | `npm run verify:release-notes-readiness` |
| Workspace dependency publish readiness | Internal workspace dependencies declare `version = "0.1.0"` alongside local paths | `npm run verify:workspace-dependency-publish-readiness` |
| CLI template packaging strategy | `dioxus-shadcn-cli` embeds registry and template assets at compile time | `npm run verify:cli-template-packaging-readiness` |

## Scope

In scope:

- keeping the blocker inventory discoverable
- checking approved repository identity is documented as resolved
- checking release docs and publish metadata docs do not imply readiness
- checking package script and release aggregate wiring

Out of scope:

- changing repository URLs
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
- workspace docs omit the approved repository URL
- package scripts stop running the blocker inventory gate during release

This gate should keep publish blockers explicit until a maintainer intentionally
resolves them. It does not resolve the blockers.
