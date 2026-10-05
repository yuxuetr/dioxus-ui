# TODOs

## Progress

- Overall: 0.2.0 plan, 7 of 8 milestones complete
- Current milestone: M194 0.2.0 Release
- Current task: M194.1

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

- DONE M188.1 Design theme presets
  - RFC 0057: a preset is a token set scoped by `[data-theme="name"]`, with `color-scheme`; the daisyUI-to-token mapping; which presets ship; the CLI surface.
  - Done: RFC 0057. Measured 18 of 35 daisyUI themes below AA, so foregrounds are adjusted; 33 presets ship (not `light` and `dark`); Checkbox moves its check mark to a mask.
- DONE M188.2 Ship preset data and `dxui theme`
  - Preset files embedded in the CLI; `dxui theme list` and `dxui theme add <name>` append a preset to `assets/dioxus-shadcn.css`, idempotently. CLI tests.
  - Done: 33 presets from `scripts/theme-presets.mjs`, embedded by `build.rs`; four CLI tests (repeat, unknown name without writing, missing stylesheet, schemes). The RFC's two component changes also landed: Link buttons use `text-foreground` (breaking for apps matching the class), and Checkbox marks are a masked `::before` filled with `--primary-foreground`, reverse-verified in the runtime check.
- DONE M188.3 Gate preset contrast
  - Every preset's foreground and background pairs meet WCAG AA, computed from the OKLCH values; reverse-verified with a failing preset.
  - Done: `npm run verify:theme-presets` in the release gate, red for a lowered foreground and a missing token. It also covers the default theme and found focused destructive menu items at 3.99:1; the light `--destructive` lightness went from 0.577 to 0.532.
- DONE M188.4 Theme picker on the site
  - The site switches presets at runtime, the Theming page lists them with swatches, and `npm run verify:site` audits each preset.
  - Done: header theme menu and a Theming page gallery; `verify:site` audits 33 presets on four pages, reverse-verified with a weakened preset. The site's canvas measured nine presets at 4.48 to 4.49:1, so the contrast math now quantizes to 8-bit sRGB and matches the canvas exactly.

## M189 Status Variants

- DONE M189.1 Success, Warning, and Info variants for Alert and Badge
  - Crate and templates, SSR tests, docs, site examples, contrast in every preset.
  - Done: RFC 0058. White text measured 3.08:1 on light success, so three status foreground tokens were added; solid status badges, tinted status alerts (`bg-card` moved from the base class into the Default and Destructive variants). The site audits the new examples in both themes and all 33 presets.

## M190 Display Components

- DONE M190.1 Stat
  - Done: RFC 0059 for all of M190; `dl`-based StatGroup with horizontal and vertical orientations, template, registry, docs, and a site example.
- DONE M190.2 Timeline
  - Done: `ol` timeline with `time` and hidden markers, vertical and horizontal through a named group instead of context; checked in a screenshot.
- DONE M190.3 Steps
  - Done: counter-numbered `ol` with status-colored connectors, `aria-current="step"`, and hidden completed text; checked in a screenshot.
- DONE M190.4 Indicator and Status
  - Done: logical-placement Indicator and a labelled-or-hidden Status dot. `docs/components/status.md` clashed with the generated status page, which moved to `component-status.md`.
- DONE M190.5 Radial Progress
  - Done: SVG progressbar ring with clamped value, default percentage label replaced by children; checked in a screenshot.
- DONE M190.6 Countdown
  - Done: language-neutral `D:HH:MM:SS` timer with `countdown_parts` for labelled layouts; the site example's clock was checked ticking in a browser.
- DONE M190.7 Diff
  - Done: range-input driven comparison with a runtime fixture (arrow keys, clip, click), reverse-verified with a handler that does not fire.

## M191 Input Components

- DONE M191.1 Rating
  - Done: RFC 0060 for all of M191; native radio stars with a runtime check for arrow keys, clicks, and fill.
- DONE M191.2 Number Input
  - Done: text spinbutton instead of `type="number"` (RFC amended: its empty value for incomplete text breaks control). The runtime check found the wrapper dimming when a button hit a bound; the dimming now keys off the input only.
- DONE M191.3 Tags Input
  - Done: pure add, commit, and remove functions with unit tests and a runtime check for Enter, comma paste, Backspace, and remove buttons.
- DONE M191.4 File Input
  - Done: styled native file input forwarding the change event; runtime check sets two files and reads their names.
- DONE M191.5 Swap
  - Done: `on`/`off` element props instead of child parts (RFC amended), `aria-pressed`, hidden inactive layer, and a runtime check.

## M192 Mobile Navigation

- DONE M192.1 Dock
  - Done: RFC 0061 for M192; safe-area `nav` with link or button items and `aria-current`, a phone-frame site example.
- DONE M192.2 FAB and Speed Dial
  - Done: plain button or speed dial by whether `on_open_change` is set; runtime check for open, Escape focus return, and actions. The `aria-controls` gate required the hidden container to stay rendered.

## M193 Completing Existing Components

- DONE M193.1 Multi-select for Select and Combobox
  - Done: RFC 0062; `multiple` withholds the listbox close handler, so the listbox code is unchanged. Selected options show check marks (a visible change for single selection too, noted in the CHANGELOG). Web runtime fixtures for both; the Desktop self-test still passes.
- DONE M193.2 Navigation Menu submenus
  - Done: RFC 0063; ownership-scoped script and a vertical orientation whose contents pair by value (absolute panels would overflow the outer popover). Runtime fixture covers click, hover, arrows, and Escape; the original fixture and the Desktop scenario still pass.
- DONE M193.3 Typed date input for Date Picker
  - Done: RFC 0064; `DatePickerInput` with `parse_date`/`format_date`, unit tests for orders and invalid dates, and a runtime check. The Calendar template's `CalendarDate::new` does not validate, so the template checks month lengths itself; `DatePickerTrigger` gained attribute passthrough for an icon-only trigger.
- DONE M193.4 Pie and donut charts
  - Done: RFC 0065; `chart_pie_arcs` with exact-path unit tests and a donut site example. The screenshot showed palette slices uncolored: Tailwind never saw classes returned from the primitives crate (also true of Success and Warning since 0.1.0); `CHART_COLOR_CLASSES` lists them, with a test that keeps it in sync.

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
