# CLI Template Packaging Readiness Metadata

This document defines the metadata gate for CLI template packaging readiness.
It keeps the current source-tree template loading model explicit until the CLI
has a publish-ready packaging strategy.

## Current State

The CLI supports source-copy workflows through:

```text
dxui init
dxui list
dxui add <component>
```

The implementation currently finds the repository root from
`CARGO_MANIFEST_DIR`, then reads registry entries from `registry/` and component
templates from the paths listed in those registry files.

That layout is useful for development and local source-copy verification, but
it is not enough for a published CLI binary. A publish-ready CLI should either
embed templates at compile time or package templates in a stable install
location that works after `cargo install`.

## Readiness Contract

`npm run verify:cli-template-packaging-readiness` should confirm that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- CLI source still uses repository-layout registry/template reads
- publish blocker docs still list CLI template packaging as unresolved
- release, quality gate, Cargo publish metadata, and docs-site notes do not
  imply the blocker is resolved

The gate is intentionally read-only. It must not embed templates, package
templates, change CLI runtime path lookup, run `cargo package`, run
`cargo publish`, install the CLI, or create package archives.

## Resolution Criteria

This blocker can be removed only after a maintainer chooses and implements a
publish-ready template delivery strategy. Acceptable outcomes include:

- compile-time embedding of all registry/template assets
- packaging templates in a stable install location with documented lookup rules

After that implementation lands, update this metadata gate, publish blocker
docs, release docs, quality gates, and TODO planning together.
