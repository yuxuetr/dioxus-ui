# TODOs

## Progress

- Overall: 80%
- Current milestone: M182 Automated Accessibility Audit
- Current task: M182.5

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), and `docs/archive/TODOs.completed-20261005-m181.md` (M181)

## Goals

- The site and the runtime preview pass an axe-core audit (WCAG 2.1 A and AA plus best practices) in the light and dark themes, and a regression fails a browser check.

## Evidence

- A one-off axe-core 4 run over the 64 component pages and the runtime preview found:
  - `ButtonGroup` and `ToggleGroup` put `aria-orientation` on `role="group"`, which does not support it (critical).
  - The `ScrollArea` and `MessageScroller` viewports scroll but cannot be reached from the keyboard (serious).
  - `AlertTitle` renders an `h5`, which skips heading levels on any page; shadcn/ui v4 renders a `div`.
  - The site and preview documents have no `lang`.
  - Example and fixture gaps: the Skeleton example puts `aria-label` on a plain `div`, a preview listbox and a Select trigger have no accessible name, and two preview Paginations share one landmark name.

## M182 Automated Accessibility Audit

- DONE M182.1 Design the accessibility audit
  - Choose the axe-core rule tags, where the audit runs (each site component page, and the runtime preview at the points where the contrast check runs, including open overlays), how it handles both themes, and which rules, if any, are disabled and why.
  - Decide each component fix from the evidence and record the decisions and reevaluation conditions in an RFC.
  - Done: RFC 0054. The preview audit disables `landmark-unique` only, since it shows several instances of the same landmark component.

- DONE M182.2 Fix the component findings
  - Drop `aria-orientation` from the `ButtonGroup` and `ToggleGroup` roots, keeping `data-orientation`; make the `ScrollArea` and `MessageScroller` viewports keyboard reachable; render `AlertTitle` as a `div`; in the crate and the templates, with tests.
  - Done: a passed `tabindex` replaces the viewport default through `default_attribute`, the generalized `default_aria_label`, so SSR writes one value.

- DONE M182.3 Fix the site and preview findings
  - Set the document language on the site and the previews, and fix the Skeleton example and the unnamed or duplicate-named preview fixtures.
  - Done: dx's page template has no `lang`, so the site and `PreviewSurface` set it at startup. Duplicate preview landmarks stay, covered by the preview's `landmark-unique` exception.

- DONE M182.4 Add the audit to the browser checks
  - Add `axe-core` as a dev dependency, run it from `npm run verify:site` and `npm run verify:runtime-interactions`, and reverse-verify that a reintroduced finding fails each check.
  - Done: the audit also caught the preview's hand-built scrolling message list, which now has a Tab stop and a region name. Reverse checks: `aria-orientation` on `ToggleGroup` fails the runtime check; a viewport without `tabindex` and an `h5` `AlertTitle` fail the site check.

- TODO M182.5 Complete the accessibility audit milestone
  - Update CHANGELOG, the affected component docs, quality gates, and the docs index.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, `npm run verify:runtime-interactions`, `npm run verify:site`, and the Desktop self-test.
  - Push local commits to `origin/main`.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
