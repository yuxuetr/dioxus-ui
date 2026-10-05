# TODOs

## Progress

- Overall: 0.2.0 plan, 1 of 8 milestones complete
- Current milestone: M188 Theme Presets
- Current task: M188.1

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), and `docs/archive/TODOs.completed-20261005-m186.md` (M186)

## Goals

- The component site is public at <https://yuxuetr.github.io/dioxus-ui/>.
- 0.2.0 fills the gaps against shadcn/ui and daisyUI: theme presets an app can switch at runtime, status variants, the daisyUI components shadcn/ui lacks, and the 0.1.0 excluded scope users are most likely to hit.

## Evidence

- 0.1.0 ships one theme. Rebranding means hand-editing about 70 token values per color scheme; daisyUI ships 35 themes switched by `data-theme`, and shadcn/ui ships base color presets.
- The stylesheet already defines `--success`, `--warning`, and `--info`, but only Toast, Sonner, and Attachment read them; Alert has only Default and Destructive, and Badge has no status variant.
- daisyUI components with no counterpart here: Stat, Timeline, Steps, Indicator, Status, Radial Progress, Countdown, Diff, Rating, Swap, File Input, Dock, and FAB.
- The 0.1.0 release notes exclude multi-select, Navigation Menu submenus, typed date parsing, and chart families beyond line, bar, and area; each one blocks a common app screen (filters, mega menus, date forms, dashboards).

## Scope Rules

- Every new component lands complete: crate module and feature, source-copy template, registry entry, docs page, site example, SSR tests, and a runtime check when it is interactive. A component that cannot meet this is cut, not stubbed.
- Ported daisyUI palettes keep daisyUI's MIT copyright notice in the theme file.
- Adding enum variants is breaking for exhaustive matches; collect these in the CHANGELOG migration notes for 0.2.0.
- Publishing 0.2.0 needs the release owner's confirmation.

## M187 Component Site Deployment

- DONE M187.1 Deploy the site to GitHub Pages
  - Build with `--base-path dioxus-ui` on every push to `main`; copy `index.html` to each route and `404.html`.
  - Done: `.github/workflows/pages.yml` and `scripts/site-pages.mjs`; Pages enabled with the Actions source. The live site serves every route with status 200 and unknown paths with the router's not found page.

## M188 Theme Presets

- TODO M188.1 Design theme presets
  - RFC 0057: a preset is a token set scoped by `[data-theme="name"]`, with `color-scheme`; the daisyUI-to-token mapping; which presets ship; the CLI surface.
- TODO M188.2 Ship preset data and `dxui theme`
  - Preset files embedded in the CLI; `dxui theme list` and `dxui theme add <name>` append a preset to `assets/dioxus-shadcn.css`, idempotently. CLI tests.
- TODO M188.3 Gate preset contrast
  - Every preset's foreground and background pairs meet WCAG AA, computed from the OKLCH values; reverse-verified with a failing preset.
- TODO M188.4 Theme picker on the site
  - The site switches presets at runtime, the Theming page lists them with swatches, and `npm run verify:site` audits each preset.

## M189 Status Variants

- TODO M189.1 Success, Warning, and Info variants for Alert and Badge
  - Crate and templates, SSR tests, docs, site examples, contrast in every preset.

## M190 Display Components

- TODO M190.1 Stat
- TODO M190.2 Timeline
- TODO M190.3 Steps
- TODO M190.4 Indicator and Status
- TODO M190.5 Radial Progress
- TODO M190.6 Countdown
- TODO M190.7 Diff

## M191 Input Components

- TODO M191.1 Rating
- TODO M191.2 Number Input
- TODO M191.3 Tags Input
- TODO M191.4 File Input
- TODO M191.5 Swap

## M192 Mobile Navigation

- TODO M192.1 Dock
- TODO M192.2 FAB and Speed Dial

## M193 Completing Existing Components

- TODO M193.1 Multi-select for Select and Combobox
- TODO M193.2 Navigation Menu submenus
- TODO M193.3 Typed date input for Date Picker
- TODO M193.4 Pie and donut charts

## M194 0.2.0 Release

- TODO M194.1 Prepare 0.2.0
  - CHANGELOG with migration notes, workspace and internal dependency versions bumped, release gate and publish dry run pass.
- TODO M194.2 Publish 0.2.0
  - Only after the release owner confirms.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
