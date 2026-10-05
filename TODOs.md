# TODOs

## Progress

- Overall: 20%
- Current milestone: M184 First Publish Preparation
- Current task: M184.2

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), and `docs/archive/TODOs.completed-20261005-m183.md` (M183)

## Goals

- The four published crates have names that are free on crates.io, CI runs the release gate on every push, and every crate passes a publish dry run, so the release owner can publish.

## Evidence

- crates.io treats `-` and `_` as the same, and `dioxus_shadcn` 0.1.1 (owner `max-wells`, last updated 2025-03-15) takes `dioxus-shadcn`. `dioxus-shadcn-core`, `dioxus-shadcn-primitives`, and `dioxus-shadcn-cli` are free, and so are `dioxus-shadcn`, `dioxus-shadcn-core`, `dioxus-shadcn-primitives`, and `dioxus-shadcn-cli`. The release owner chose the `dioxus-shadcn` names on 2026-10-05.
- The public GitHub repository has no `.github/workflows`, so no check runs on push; the browser workflow is only a template in the docs.

## M184 First Publish Preparation

- DONE M184.1 Record the crate name decision
  - Record the conflict, the chosen names, what keeps its name (the repository, the `dxui` binary, unpublished examples), and reevaluation conditions in an RFC.
  - Done: RFC 0056. The theme stylesheet becomes `assets/dioxus-shadcn.css`; the `dxui` DOM prefix and archived plans keep their names.

- TODO M184.2 Rename the published crates
  - Rename `dioxus-shadcn`, `dioxus-shadcn-core`, `dioxus-shadcn-primitives`, and `dioxus-shadcn-cli` to the `dioxus-shadcn` names: package names, directories, Rust paths, the theme stylesheet, docs, registry, templates, and scripts.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser checks.

- TODO M184.3 Run the release gate in CI
  - Add a GitHub Actions workflow that runs the release gate on push and pull request, push it, and confirm the first run passes.

- TODO M184.4 Dry-run the publish
  - Run a publish dry run for each crate in publish order, fix what it finds, and record the publish commands for the release owner.

- TODO M184.5 Complete the first publish preparation milestone
  - Update CHANGELOG and the release docs, and push local commits to `origin/main`.
  - The real `cargo publish` stays with the release owner.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
