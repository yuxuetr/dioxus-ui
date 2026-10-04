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
- `dioxus-ui-core`
- `dioxus-ui-primitives`
- `dioxus-ui`
- `dioxus-ui-cli`
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

- feature-gated `dioxus-ui` exports
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
([RFC 0041](rfcs/0041-checkbox-indeterminate-state.md)).
