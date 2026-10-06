# CLI Template Embedding Plan

M124 resolves the CLI template packaging blocker by moving `dioxus-shadcn-cli` from
repository-layout registry/template reads to compile-time embedded assets. It
does not publish, package, install, or contact registries.

## Current Implementation

The CLI currently:

- loads registry JSON from `workspace_root().join("registry")`
- skips `registry/schema.json`
- reads each component file from `workspace.join(&file.source)`
- reads each component asset from `workspace.join(&asset.source)`
- derives `workspace_root()` from `env!("CARGO_MANIFEST_DIR")`

That works for local development and generated fixture smoke tests, but a
binary installed through `cargo install` cannot rely on the repository layout
being present.

## Chosen Strategy

Use compile-time embedding:

- embed every component registry JSON file except `registry/schema.json`
- embed every source-copy file referenced by those registry entries
- keep registry `source` and `target` metadata unchanged
- resolve file and asset content by matching the registry `source` path against
  the embedded asset catalog
- preserve existing CLI commands and flags

This keeps `dxui init`, `dxui list`, and `dxui add <component>` independent of
the repository directory at runtime.

## Implementation Shape

Add an internal asset catalog in `crates/dioxus-shadcn-cli/src/main.rs`:

```text
struct EmbeddedAsset {
  source: &'static str,
  content: &'static str,
}
```

Expected helpers:

- `embedded_registry_json() -> &'static [&'static str]`
- `embedded_assets() -> &'static [EmbeddedAsset]`
- `load_registry()` parses embedded JSON instead of reading `registry/`
- `embedded_asset_content(source: &str) -> Result<&'static str, Box<dyn Error>>`
- `add_component_recursive()` writes embedded content to the registry target

If manual `include_str!` entries become too large to maintain, a follow-up can
move the catalog generation to `build.rs`. M124 should start with the smallest
deterministic implementation that keeps the CLI binary self-contained.

## Tests

Update CLI tests to prove:

- `load_registry()` does not require repository path reads
- `dxui list` ordering is unchanged
- `dxui add button` still copies `button.rs` and `utils.rs`
- overwrite and no-overwrite behavior is unchanged
- unknown component errors still omit `utils`
- every registry `files[].source` and `assets[].source` has embedded content

Existing generated fixture smoke should continue to compile the copied source.

## Metadata Gate Changes

After implementation, update
[CLI Template Packaging Readiness Metadata](archive/first-publish/cli-template-packaging-readiness-metadata.md)
and `scripts/cli-template-packaging-readiness-verify.mjs` so the focused gate
checks the resolved state:

- CLI source uses compile-time embedding
- registry and template assets are available without repository path lookup
- publish blocker docs no longer list CLI template packaging as unresolved
- release, quality gate, README, Cargo publish metadata, decision matrix, and
  docs-site notes describe embedded template delivery

Other publish blockers must remain unresolved unless separately approved.

## Non-goals

M124 must not:

- run `cargo package`
- run `cargo publish`
- run a global `cargo install`
- contact crates.io
- change crate versions
- replace repository URLs
- commit license files
- rewrite component APIs
- rewrite source-copy templates unless required by embedding
- create package archives
- create Git tags
- publish artifacts
- add CI workflow activation

## Validation

Focused validation:

```bash
cargo test -p dioxus-shadcn-cli
cargo test -p dioxus-shadcn-cli --test registry
cargo run -p dioxus-shadcn-cli -- list
scripts/generated-fixture-smoke.sh
npm run verify:package-contents
npm run verify:registry
npm run verify:docs
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:repo-hygiene
git diff --check
```

Full local release confidence:

```bash
npm run verify:release
```

The full release gate remains local and does not package, publish, tag, or
contact registries.
