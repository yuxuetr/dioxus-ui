# RFC 0073: Blocks

- Status: Accepted
- Created: 2026-10-06

## Summary

Add blocks: whole screens, such as a dashboard or a sign-in page, that
`dxui add` copies into an app together with the components they use.

## Current State

The CLI copies one component at a time
([RFC 0002](0002-cli-registry-and-code-generation.md)). An app shell built
from the library takes dozens of parts wired together: a sidebar that goes
off-canvas, a header, cards, a chart, a table. shadcn/ui ships blocks for
this, and building them also tests whether the components fit together.

## Decision

### Registry

A block lives in `crates/dioxus-shadcn-cli/blocks/`: `<name>.json` in the
registry format and `<name>.rs`, its source. Its `dependencies` are the
components it uses. Blocks have their own directory so the tools that read
the component registry (the catalog, the docs and feature checks, the
template parity test) see components only, and the registry type needs no
new field.

### CLI

- `dxui add <name>` takes a component or a block. For a block it adds the
  block's components as `dxui add` would, writes the block to
  `src/blocks/<module>.rs`, and declares it in `src/blocks/mod.rs`.
- `dxui list` still prints the components only, one per line, since scripts
  read it; `dxui list blocks` prints the blocks.
- No block shares a name with a component.

### Block sources

A block is one file with a `#[component]` named after it, such as
`DashboardBlock`. It imports components from `crate::components::ui`, the
layout `dxui init` sets up, holds its own sample data and state in signals,
and depends only on `dioxus` and the copied components, so it compiles as
soon as it is added and the app replaces the sample data.

## Alternatives

- **A `kind` field in the component registry.** Every reader of the registry
  would need to skip blocks, and `RegistryComponent` would gain a field.
- **Blocks as crate modules.** A screen is app code that the app changes;
  copying fits it, and the crate stays a component library.

## Verification

- CLI tests: adding a block copies it and its components and declares both
  modules; `dxui list blocks` lists it; an unknown name lists components and
  blocks.
- Registry tests: each block's source exists, targets `src/blocks/`, names
  only known components, imports no internal crate, and has a unique name.
- The generated fixture app adds every block and compiles it with the
  components.
- Each block has a docs page and a site preview.
