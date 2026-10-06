# Roadmap

## Stage 0: Documentation

Goal: align the product direction before implementation.

Deliverables:

- README
- design overview
- RFCs
- TODO plan

Exit criteria:

- repository shape is specified
- component sequence is defined
- styling and CLI direction are documented

## Stage 1: Workspace Skeleton

Goal: create the Rust project structure without shipping components yet.

Deliverables:

- Cargo workspace
- `dioxus-shadcn-core`
- `dioxus-shadcn-primitives`
- `dioxus-shadcn`
- `dioxus-shadcn-cli`
- empty registry and template directories
- web and desktop example skeletons

Exit criteria:

- `cargo check` succeeds
- crate dependency direction is enforced
- project docs match the actual layout

## Stage 2: Static Components and CLI Prototype

Goal: prove the copied-source workflow with low-risk components.

Deliverables:

- registry schema
- `dxui list`
- `dxui add button`
- Button template
- Input, Textarea, Label templates
- web demo

Exit criteria:

- generated files compile in a Dioxus example app
- Tailwind class strings are statically discoverable
- component API is documented

## Stage 3: Stateful Components

Goal: add components that need Dioxus state but not overlay primitives.

Deliverables:

- Checkbox
- Switch
- Tabs
- Accordion
- component interaction tests where practical

Exit criteria:

- keyboard behavior is documented
- controlled and uncontrolled APIs are clear
- examples cover common usage

## Stage 4: Primitive Layer

Goal: solve the hard accessibility and interaction problems once.

Deliverables:

- focus management utilities
- Dialog primitive
- Popover primitive
- Tooltip primitive
- Select primitive design

Exit criteria:

- primitives are unstyled
- styled wrappers and templates reuse primitive behavior
- accessibility behavior is tested or manually verified

## Stage 5: Packaged Components

Goal: make crate-mode usage viable after APIs settle.

Deliverables:

- feature-gated `dioxus-shadcn` exports
- crate-mode examples
- versioning policy
- release checklist

Exit criteria:

- users can choose copied-source or dependency mode
- feature flags avoid unnecessary component code
- docs clearly explain both installation paths

## Stage 6: Documentation Site and Previews

Goal: make component evaluation and adoption easy.

Deliverables:

- component docs site
- live examples
- theme documentation
- registry browsing

Exit criteria:

- each component has API docs, examples, and accessibility notes
- generated source and crate-mode examples stay in sync

Status: M179 adds the component site shell
([RFC 0052](rfcs/0052-component-site.md)): routes for every catalog component,
the catalog sidebar, the installation and theming guides, and the theme
toggle, checked in a browser by `npm run verify:site`. M180 adds live
examples with their source for all 64 components and links each page to the
API and accessibility notes, so the stage's exit criteria hold for the site.

## Stage 7: Runtime Adapters

Goal: add optional renderer-aware behavior without making controlled styled
components or source-copy templates depend on runtime commands by default.

Deliverables:

- focus and portal adapter contracts
- timer and live-region adapter contracts
- measurement, pointer, and gesture adapter contracts
- renderer verification matrix
- Web/Desktop verification examples
- documented fallback behavior for unsupported targets

Exit criteria:

- components still work without adapters
- adapters can report unsupported behavior without panics
- Web, Desktop, and Mobile checks are planned before defaults change
- Dialog or Alert Dialog verifies modal focus behavior
- Popover verifies non-modal portal and measurement behavior
- Toast or Sonner verifies timer and live-region behavior

Status: M135 meets the Dialog and Alert Dialog modal focus criterion and the
Popover measurement criterion in the styled components, verified in the Web
browser smoke. Popover uses fixed positioning instead of a DOM portal by
decision in [RFC 0010](rfcs/0010-overlay-interaction-behavior.md). M136 meets
the Toast and Sonner criterion with a paused countdown and persistent viewport
live regions ([RFC 0011](rfcs/0011-toast-timer-and-live-region.md)), verified
in the same browser smoke. M137 extends the same approach to Select and Combobox
listbox keyboard navigation and selection
([RFC 0012](rfcs/0012-listbox-overlay-behavior.md)), and M138 to Date Picker
and Calendar keyboard navigation
([RFC 0013](rfcs/0013-date-picker-calendar-keyboard.md)), and M139 to Dropdown
and Context Menu keyboard navigation
([RFC 0014](rfcs/0014-menu-keyboard-behavior.md)), and M140 to Menubar
roving triggers and menu switching
([RFC 0015](rfcs/0015-menubar-keyboard-behavior.md)), and M141 to Navigation
Menu disclosure interaction
([RFC 0016](rfcs/0016-navigation-menu-interaction.md)), and M144 to Tabs,
Radio Group, and Toggle Group roving focus
([RFC 0019](rfcs/0019-roving-group-interaction.md)), and M146 to Accordion
toggle reporting and trigger movement
([RFC 0021](rfcs/0021-accordion-interaction.md)), and M147 to Tooltip hover
and focus opening
([RFC 0022](rfcs/0022-tooltip-hover-and-focus-opening.md)), and M148 to Hover
Card hover and focus opening
([RFC 0023](rfcs/0023-hover-card-hover-and-focus-opening.md)), and M149 to
Command keyboard and filtering
([RFC 0024](rfcs/0024-command-keyboard-and-filtering.md)), and M150 to
right-to-left arrow mirroring
([RFC 0025](rfcs/0025-right-to-left-arrow-mirroring.md)), and M151 to Tabs
manual activation and vertical orientation
([RFC 0026](rfcs/0026-tabs-manual-activation-and-vertical-orientation.md)), and
M152 to Combobox and Command result announcements
([RFC 0027](rfcs/0027-combobox-and-command-result-announcements.md)), and M153
to Switch and Checkbox change events
([RFC 0028](rfcs/0028-switch-and-checkbox-change-events.md)), and M154 to
Button, Toggle, Input, and Textarea events
([RFC 0029](rfcs/0029-button-toggle-input-and-textarea-events.md)), and M155
to Slider keyboard and pointer input
([RFC 0030](rfcs/0030-slider-keyboard-and-pointer-input.md)), and M156 to
Collapsible and Native Select events
([RFC 0031](rfcs/0031-collapsible-and-native-select-events.md)), and M157 to
Input OTP value changes
([RFC 0032](rfcs/0032-input-otp-value-changes.md)), and M158 to Pagination
page changes ([RFC 0033](rfcs/0033-pagination-page-changes.md)), and M159 to
Carousel slide changes ([RFC 0034](rfcs/0034-carousel-slide-changes.md)), and
M160 to control label overrides
([RFC 0035](rfcs/0035-control-label-overrides.md)), and M161 to Resizable
handle input ([RFC 0036](rfcs/0036-resizable-handle-input.md)), and M162 to
Sidebar toggle and items
([RFC 0037](rfcs/0037-sidebar-toggle-and-items.md)), and M163 to form
control naming ([RFC 0038](rfcs/0038-form-control-naming.md)), and M164 to
dialog names ([RFC 0039](rfcs/0039-dialog-names.md)), and M165 to composite
widget names ([RFC 0040](rfcs/0040-composite-widget-names.md)), and M166 to
the Checkbox indeterminate state
([RFC 0041](rfcs/0041-checkbox-indeterminate-state.md)), and M167 to Slider
thumb position and vertical orientation
([RFC 0042](rfcs/0042-slider-thumb-position-and-vertical-orientation.md)),
and M168 to compiled Tailwind browser checks
([RFC 0043](rfcs/0043-compiled-tailwind-browser-checks.md)), and M169 to
Tailwind utility conflicts
([RFC 0044](rfcs/0044-tailwind-utility-conflicts.md)), and M170 to the drawn
Checkbox ([RFC 0045](rfcs/0045-drawn-checkbox.md)), and M171 to list widths
that follow the trigger
([RFC 0046](rfcs/0046-listbox-width-follows-trigger.md)), and M172 to an
opt-in dark theme ([RFC 0047](rfcs/0047-opt-in-dark-theme.md)), and M173
to the phone-width preview layout
([RFC 0048](rfcs/0048-phone-width-preview-layout.md)), and M174 to the
compiled preview stylesheet
([RFC 0049](rfcs/0049-compiled-preview-stylesheet.md)), and M175 to the
preview theme toggle ([RFC 0050](rfcs/0050-preview-theme-toggle.md)), and
M176 to M178 to the semantic color tokens, the component migration, and the
token-only dark theme ([RFC 0051](rfcs/0051-semantic-color-tokens.md)).

## Stage 8: 0.2.0 Themes and Components

Goal: close the gaps against shadcn/ui and daisyUI after the 0.1.0 publish.

Deliverables:

- theme presets switched at runtime with `data-theme`, added by `dxui theme`
- Success, Warning, and Info variants for Alert and Badge
- the daisyUI components with no counterpart here: Stat, Timeline, Steps,
  Indicator, Status, Radial Progress, Countdown, Diff, Rating, Number Input,
  Tags Input, File Input, Swap, Dock, and FAB
- multi-select, Navigation Menu submenus, typed date input, and pie charts
  from the 0.1.0 excluded scope

Exit criteria:

- every preset passes the contrast gate and the site audit
- every new component has a template, registry entry, docs page, site
  example, and tests
- 0.2.0 release notes list each breaking enum addition with a migration note

Status: M187 deploys the component site to
<https://yuxuetr.github.io/dioxus-ui/>. M188 ships 33 theme presets
([RFC 0057](rfcs/0057-theme-presets.md)), M189 the status variants
([RFC 0058](rfcs/0058-status-variants.md)), and M190 Stat, Timeline, Steps,
Indicator, Status, Radial Progress, Countdown, and Diff
([RFC 0059](rfcs/0059-display-components.md)), and M191 Rating, Number
Input, Tags Input, File Input, and Swap
([RFC 0060](rfcs/0060-input-components.md)), and M192 Dock and Fab
([RFC 0061](rfcs/0061-mobile-navigation.md)), and M193 multi-select,
Navigation Menu submenus, typed dates, and pie charts
([RFC 0062](rfcs/0062-multi-select.md) to
[RFC 0065](rfcs/0065-pie-and-donut-charts.md)). M194 publishes 0.2.0 on 2026-10-05
(tag `v0.2.0`).

## Stage 9: 0.3.0 Screens and Maintenance

Goal: make the library complete enough to build an app shell, show the code
in the docs, and cut the cost of each change.

Deliverables:

- the first-publish readiness docs and gates archived, and one source of
  truth checked between crate modules and templates
- Dropdown checkbox, radio, and shortcut items, and submenus for Dropdown,
  Context Menu, and Menubar
- scroll lock for modal overlays, Sidebar off-canvas and shortcut, a range
  slider, and a pagination range helper
- Theme Controller, Menu, and Mockup from daisyUI
- example source and API reference on the component site
- blocks: copyable screens added by `dxui add`

Exit criteria:

- every new component and block has a template, registry entry, docs page,
  site example, and tests
- a generated fixture app builds every block
- 0.3.0 release notes list each breaking change with a migration note

Status: M195 retires the first-publish gates and adds the template parity
test ([RFC 0066](rfcs/0066-template-parity.md)), M196 menu checkable items
and submenus ([RFC 0067](rfcs/0067-menu-submenus.md)), M197 scroll lock, the
off-canvas Sidebar, the range slider, and the pagination range
([RFC 0068](rfcs/0068-modal-scroll-lock.md) to
[RFC 0070](rfcs/0070-range-slider.md)), M198 Theme Controller, Menu, and
Mockup ([RFC 0071](rfcs/0071-theme-controller.md) and
[RFC 0072](rfcs/0072-menu-and-mockup.md)), M199 copyable code and the rendered
reference on the site, and M200 blocks ([RFC 0073](rfcs/0073-blocks.md)).
M201 publishes 0.3.0 on 2026-10-06 (tag `v0.3.0`).

## Stage 10: 0.4.0 Copy Mode

Goal: a copied component brings only the code it needs and builds without
warnings, and the CLI says what it did and what changed since the app copied
a component.

Deliverables:

- the shared utils template split along crate module boundaries, copied as
  registry dependencies
- copied components that build with warnings denied
- several names per `dxui add`, `dxui --version`, honest add output, and
  `dxui diff`
- `cargo-semver-checks` as a release step

Exit criteria:

- `dxui add button` copies only Button and the base helper, and an app using
  it builds with `-D warnings`
- apps with 0.3.0 templates keep building, or the migration notes say what to
  change
- 0.4.0 release notes list each breaking change with a migration note

Status: planned as M202 to M204 in `TODOs.md`.
