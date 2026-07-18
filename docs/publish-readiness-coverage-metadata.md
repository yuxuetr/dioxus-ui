# Publish Readiness Coverage Metadata

This document defines the coverage contract between known publish readiness
blockers and focused readiness metadata gates.

## Current Coverage

Each current blocker in [Publish Readiness Blockers](publish-readiness-blockers.md)
must have a focused metadata gate:

| Blocker | Focused Gate | Metadata Doc |
| --- | --- | --- |
| Placeholder repository URL | `npm run verify:repository-identity-readiness` | [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md) |
| Pre-1.0 API stability | `npm run verify:api-stability-readiness` | [API Stability Readiness Metadata](api-stability-readiness-metadata.md) |
| Release notes not publish-ready | `npm run verify:release-notes-readiness` | [Release Notes Readiness Metadata](release-notes-readiness-metadata.md) |
| Root license files not committed | `npm run verify:license-readiness` | [License Readiness Metadata](license-readiness-metadata.md) |
| CLI template packaging strategy | `npm run verify:cli-template-packaging-readiness` | [CLI Template Packaging Readiness Metadata](cli-template-packaging-readiness-metadata.md) |
| Registry availability not checked | `npm run verify:registry-availability-readiness` | [Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md) |
| Workspace dependency publish readiness | `npm run verify:workspace-dependency-publish-readiness` | [Workspace Dependency Publish Readiness Metadata](workspace-dependency-publish-readiness-metadata.md) |

## Readiness Contract

`npm run verify:publish-readiness-coverage` should confirm that:

- every current blocker has a focused readiness gate
- every focused readiness gate has a metadata document
- package scripts expose the focused coverage check
- the release aggregate includes the focused coverage check
- README, release docs, quality gates, and docs-site notes mention the coverage
  gate

The gate is intentionally read-only. It must not resolve blockers, replace
repository URLs, stabilize APIs, generate release notes, generate license text,
embed or package CLI templates, change dependency versions, contact registries,
inspect credentials, run `cargo package`, run `cargo publish`, or create
package archives.

## Resolution Criteria

When a blocker is intentionally resolved or a new blocker is added, update the
blocker inventory, focused readiness gate mapping, package scripts, release
docs, quality gates, runbook, and TODO planning together.
