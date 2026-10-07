# Roadmap

The stages up to 0.4.2 are complete; their full text is archived in
[Roadmap: Stages 0 to 10](archive/roadmap-20261006-stages-0-10.md). This
roadmap plans the way from 0.4.2 to 1.0.

## Completed Stages

| Stage | Release | Result |
| --- | --- | --- |
| 0 to 5 | 0.1.0 | Workspace, CLI, registry, styled components, primitives, crate mode |
| 6 | 0.1.0 | Component site with live examples, theming guide, and API notes |
| 7 | 0.1.0 | Keyboard, focus, placement, dismissal, and live regions, verified in a browser |
| 8 | 0.2.0 | 33 theme presets, status variants, daisyUI components |
| 9 | 0.3.0 | Menus with submenus, scroll lock, Theme Controller, blocks, template parity |
| 10 | 0.4.0 to 0.4.2 | Helper templates, warning-free copies, `dxui diff`, render-scoped element ids |

## What 1.0 Means

1.0 promises that the public API of every crate and the source-copy templates
stay compatible within the 1.x series. Before making that promise, the API
must be the one the project wants to keep, and someone outside the project
must have used it. Measured on 2026-10-06 at `v0.4.2`:

- **Ergonomics.** Stateful components are fully controlled and their parts
  do not share state. A Select needs `open` passed twice, an `anchor_id`
  matched to the trigger id by hand, and `selected` computed per item; Tabs
  need `active` computed per trigger and per panel. 25 crate modules take
  `open: bool` and 25 take per-part `active`, `selected`, `checked`, or
  `pressed`.
- **Overrides.** A user class does not win by position (RFC 0076).
- **Density.** `UiDensity` is public and the design names it the platform
  strategy, but only Button uses it, 1 of 82 components.
- **Surface.** About 1,484 public items in `dioxus-shadcn` and 294 in
  `dioxus-shadcn-primitives`, some of them re-exported, so a primitives
  change can break the styled crate.
- **Dependency.** Props use Dioxus types such as `Element` and
  `EventHandler`, so moving from Dioxus 0.7 to 0.8 is a breaking change here.
  Dioxus 0.8 is at `0.8.0-alpha.1`.
- **Users.** 18 to 30 downloads per crate and no issues: no outside
  evidence yet that the API works for anyone else.

## Stage 11: 0.4.x Crate-Mode Setup and Theme Marks

Goal: fix the non-breaking defects found in the 0.4.2 review.

Deliverables:

- `dxui` writes and refreshes the crate's `@source` line from
  `cargo metadata`, so a crate upgrade cannot leave Tailwind scanning the
  previous version's source
- known limitations that match the release: the Checkbox mark limitation
  was stale, since the marks have followed `--primary-foreground` since 0.2.0

Exit criteria:

- a crate-mode app that bumps `dioxus-shadcn` gets the new path from one
  `dxui` command, which replaces the stale path
- the browser check and the preset contrast gate cover the Checkbox marks

## Stage 12: 0.5.0 Overrides, Owned State, and Density

Goal: make the component API the one 1.0 keeps. Every change here is
breaking and lands in one minor release with migration notes.

Deliverables:

- user classes that win by position, never removing a style they do not
  replace ([RFC 0076](rfcs/0076-user-class-overrides.md))
- compound components whose root owns state and shares it with its parts
  through context: uncontrolled with a default value, controlled with a
  value and a change callback, and element ids and anchors linked
  automatically; designed in an RFC on Select and Tabs before it spreads
- a measured decision on density: implemented through context for the
  interactive components, or removed from the public API

Exit criteria:

- the RFC 0076 ground-truth gate and browser check pass in the release gate
- the Select and Tabs doc examples need no hand-matched ids and no per-item
  state, and the site and every block use the new API
- the density decision records the touch target sizes it was based on
- 0.5.0 release notes list each breaking change with a migration note

## Stage 13: 0.6.0 Public Surface

Goal: commit to only the API users need.

Deliverables:

- an audit of every public item, kept or made `pub(crate)`
- primitives that the styled crate does not re-export kept out of its
  semver promise, or the re-exports removed

Exit criteria:

- every remaining public item has a docs page or doc comment that says what
  it is for
- `cargo-semver-checks` runs against the audited surface in the release gate

## Stage 14: 0.7.0 Dioxus 0.8

Goal: move to Dioxus 0.8 before 1.0, so 1.x does not start on a Dioxus line
that is about to be replaced.

Starts when `cargo search dioxus --limit 1 --color never | grep -qE '^dioxus = "0\.8\.[0-9]+"'`
exits 0.

Measured on 2026-10-07 at `v0.6.0` against `0.8.0-alpha.1`: the crates,
templates, blocks, site, and demos build and test without a source change,
and the browser interactions pass once `dx` 0.8's default hot-patching is
turned off. Before the release, the browser checks learn both `dx` lines, the
fullstack hydration check becomes a script in the release gate, and
`npm run verify:dioxus-next` reruns this measurement on each 0.8
pre-release (M215 in the [TODO Plan](../TODOs.md)).

Exit criteria:

- release gate, browser checks, Desktop and Mobile self-tests, and a
  fullstack hydration check pass on Dioxus 0.8

## Stage 15: 1.0

Goal: freeze the API after outside use.

Deliverables:

- `1.0.0-rc` releases that change only bugs and docs
- a call for feedback in the Dioxus community, with the issues it raises
  triaged

Exit criteria:

- Stages 11 to 14 are complete
- at least one app outside this repository has built on an `rc` and its
  issues are closed or deferred with a reason
- no breaking change was needed during the `rc` period

Measured on 2026-10-07 at `086e7de`: `verify:semver` treats `1.0.0-rc.1` to
`rc.2` as a major change and lets it break the API, no crate declares a Rust
floor, and there is no issue form for outside feedback. These are fixed before
Stage 14 ends; the `rc` releases start after `v0.7.0` (M217 to M219 in the
[TODO Plan](../TODOs.md)).

## Not Planned Before 1.0

Each item is re-evaluated by the command in the
[TODO Plan](../TODOs.md#deferred-re-evaluate-when):

- form state and validation, chart tooltips, swipe gestures, a DOM portal,
  Command fuzzy ranking, editing an Input OTP slot in the middle, and
  right-to-left Slider, Resizable, and Calendar keys
- generating templates from the crate, while `CRATE_ONLY` in the parity test
  stays at 10 entries or fewer
- new components or blocks without an issue that asks for them
