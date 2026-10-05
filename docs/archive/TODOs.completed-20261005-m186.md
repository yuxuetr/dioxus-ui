# TODOs

## Progress

- Overall: 100%
- Current milestone: none (M186 complete)
- Current task: none

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), and `docs/archive/TODOs.completed-20261005-m185.md` (M185)

## Goals

- `dioxus-shadcn` 0.1.0 is on crates.io, with `CardTitle` matching shadcn/ui v4 before the API is frozen in a published version.

## Evidence

- `CardTitle` renders a fixed `h3`, so an app cannot fit it to its page outline; shadcn/ui v4 renders a `div`, and M182 made the same change to `AlertTitle` for the same reason.
- On 2026-10-05 the release owner asked for the publish to run now, with the current login.

## M186 First Publish

- DONE M186.1 Render CardTitle as a div
  - Change `CardTitle` in the crate and the template, with an SSR test, and update the Card docs and CHANGELOG.
  - Done: release gate, site check, and runtime check pass.

- DONE M186.2 Publish 0.1.0
  - With a clean tree matching `origin/main`, rerun the publish dry run, run `cargo publish --workspace`, tag `v0.1.0`, and record the release in the docs.
  - Done: published on 2026-10-05 after the release owner verified the crates.io email (the first attempt was refused before any upload). `cargo install dioxus-shadcn-cli` and a fresh app against the published crates both work.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
