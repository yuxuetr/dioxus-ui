# RFC 0056: Published Crate Names

- Status: Accepted
- Created: 2026-10-05

## Summary

Publish the four crates as `dioxus-shadcn`, `dioxus-shadcn-core`,
`dioxus-shadcn-primitives`, and `dioxus-shadcn-cli`, since crates.io already
has a `dioxus_ui` crate that takes the `dioxus-ui` name.

## Current State

crates.io treats `-` and `_` in crate names as the same name. On 2026-10-05:

| Name | crates.io |
| --- | --- |
| `dioxus-ui` | taken by `dioxus_ui` 0.1.1, owner `max-wells`, last updated 2025-03-15, no repository |
| `dioxus-ui-core`, `dioxus-ui-primitives`, `dioxus-ui-cli` | free |
| `dioxus-shadcn`, `dioxus-shadcn-core`, `dioxus-shadcn-primitives`, `dioxus-shadcn-cli` | free |
| `dxui`, `dxui-core`, `dxui-primitives`, `dxui-cli` | free |

The styled crate cannot publish under its current name, and the project cannot
claim it from an active owner.

## Decision

The release owner chose the `dioxus-shadcn` names, which say what the project
follows and keep one prefix for all four crates:

| Before | After | Rust path |
| --- | --- | --- |
| `dioxus-ui` | `dioxus-shadcn` | `dioxus_shadcn` |
| `dioxus-ui-core` | `dioxus-shadcn-core` | `dioxus_shadcn_core` |
| `dioxus-ui-primitives` | `dioxus-shadcn-primitives` | `dioxus_shadcn_primitives` |
| `dioxus-ui-cli` | `dioxus-shadcn-cli` | none (binary only) |

The crate directories under `crates/` follow the package names, and the theme
stylesheet that `dxui init` writes and the docs reference becomes
`assets/dioxus-shadcn.css`.

These keep their names:

- the GitHub repository, `yuxuetr/dioxus-ui`, which the crates name as their
  `repository`;
- the `dxui` binary and its commands;
- the unpublished example and site packages (`dioxus-ui-web-demo`,
  `dioxus-ui-site`, and so on), the npm workspace name, and browser artifact
  prefixes, which no consumer sees;
- the `dxui`-prefixed ids, data attributes, and CSS variables the components
  render, such as `data-dxui-anchored` and `--dxui-anchor-width`;
- archived plans under `docs/archive`, which record history.

Nothing has been published, so the rename breaks no consumer.

## Scope

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Renaming the GitHub repository | A renamed repository redirects, but links in existing clones and the project's own history would still point at the old name | The release owner renames it |
| Renaming the `dxui` binary or the `dxui` DOM prefix | Neither collides with anything on crates.io, and both are short | A conflict with another tool or library is reported |

## Verification

- No tracked file outside `docs/archive` and this RFC names a published crate
  by its old name, checked with `git grep`.
- `CARGO_NET_OFFLINE=true npm run verify:release`, the runtime and site
  browser checks, and the Desktop self-test pass after the rename.
- A publish dry run passes for each crate (M184.4).
