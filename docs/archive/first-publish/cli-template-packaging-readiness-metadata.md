# CLI Template Packaging Readiness Metadata

This document defines the metadata gate for CLI template packaging readiness.
The CLI now uses compile-time embedded registry and template assets, so
`dxui list` and `dxui add <component>` no longer require the repository
`registry/` and `templates/` directories at runtime. M134 moved both
directories into `crates/dioxus-shadcn-cli/` so the build script reads them from
`CARGO_MANIFEST_DIR` and `cargo package` ships them with the CLI crate.
`npm run verify:package-contents` checks the packaged file list, and the M134
`cargo publish --workspace --dry-run` verified the packaged CLI builds.

## Current State

The CLI supports source-copy workflows through:

```text
dxui init
dxui list
dxui add <component>
```

The implementation generates an embedded asset catalog in
`crates/dioxus-shadcn-cli/build.rs` and includes that catalog from
`crates/dioxus-shadcn-cli/src/main.rs`. The generated catalog embeds:

- all public registry JSON entries except `crates/dioxus-shadcn-cli/registry/schema.json`
- every source-copy file referenced by registry `files` mappings
- every asset referenced by registry `assets` mappings

The runtime CLI parses embedded registry JSON and writes embedded file content
to each registry target path.

## Readiness Contract

`npm run verify:cli-template-packaging-readiness` should confirm that:

- package scripts expose the focused readiness check
- the release aggregate includes the focused readiness check
- CLI build script generates embedded registry/template assets
- CLI runtime reads embedded registry/template content
- publish blocker docs no longer list CLI template packaging as unresolved
- release, quality gate, Cargo publish metadata, and docs-site notes describe
  embedded template delivery

The gate is intentionally read-only. It must not run `cargo package`, run
`cargo publish`, install the CLI, contact crates.io, create package archives,
or change embedded template contents.

## Resolution Criteria

This blocker is resolved when:

- CLI tests pass
- generated fixture smoke passes
- registry metadata checks pass
- this focused readiness gate passes
- publish blocker docs list only the remaining blockers

If the embedded catalog strategy changes later, update this metadata gate,
publish blocker docs, release docs, quality gates, and TODO planning together.
