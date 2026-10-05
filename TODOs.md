# TODOs

## Progress

- Overall: 50%
- Current milestone: M185 Release Notes Finalization
- Current task: M185.2

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), and `docs/archive/TODOs.completed-20261005-m184.md` (M184)

## Goals

- The crates.io pages show a short user guide, and the changelog names the `0.1.0` release, so the release owner can run `cargo publish --workspace`.

## Evidence

- All four packages ship the root README, 1128 lines that mostly document development checks; crates.io shows it on every crate page, fixed per version.
- Crate-mode Tailwind generates no component classes unless the stylesheet has an `@source` line for the crate's source, and no doc says so (checked by compiling the generated stylesheet with and without it).
- `CHANGELOG.md` keeps the first release under `[Unreleased]`; the publish steps in `docs/release.md` rename it to `[0.1.0]` before publishing.

## M185 Release Notes Finalization

- DONE M185.1 Write the published crate README
  - Add a short user guide as the packages' README: the crates, source-copy and crate-mode setup (including the crate `@source` line), the theme, the components, and status; compile its examples.
  - Done: `crates/README.md` is the workspace `readme`. Both setups compile from a scratch app, and Tailwind generates the component classes only with the crate `@source` line. The packaged README dropped the core crate archive from 16 KiB to 5 KiB.

- TODO M185.2 Name the 0.1.0 release in the changelog
  - Rename `[Unreleased]` to `[0.1.0]` with the release date, keep the gates passing, rerun the publish dry run, and push.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
