# Public API Surface Inventory

This inventory records the current public API surfaces that need maintainer
review before the API stability blocker can be resolved. It is descriptive and
does not approve API stability.

Use it with:

- [API Stability Surface Audit Plan](api-stability-surface-audit-plan.md)
- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md)
- [API Stability Decision Record Template](api-stability-decision-record-template.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)

## Summary

| Surface | Count | User-facing Mode | Notes |
| --- | ---: | --- | --- |
| Styled component crate modules | 64 | Crate mode | Feature-gated modules under `crates/dioxus-ui/src`. |
| Styled component features | 64 | Crate mode | Public Cargo feature names in `crates/dioxus-ui/Cargo.toml`. |
| Source-copy templates | 65 | Source-copy mode | 64 component templates plus shared `utils.rs`. |
| Registry entries | 65 | CLI/source-copy mode | 64 component entries plus `utils.json`. |
| Registry schema | 1 | Tooling metadata | `registry/schema.json` validates registry shape and is not a component. |
| Primitive modules | 20 | Crate mode/internal behavior | Public helpers and config types in `dioxus-ui-primitives`. |
| Core crate exports | 4 groups | Crate/tooling mode | `classes`, `UiDensity`, and registry data structs. |

## Styled Component Features

Current `dioxus-ui` feature names:

```text
accordion
alert
alert-dialog
aspect-ratio
attachment
avatar
badge
breadcrumb
bubble
button
button-group
calendar
card
carousel
chart
checkbox
collapsible
combobox
command
context-menu
data-table
date-picker
dialog
direction
drawer
dropdown
empty
field
hover-card
input
input-group
input-otp
item
kbd
label
marker
menubar
message
message-scroller
native-select
navigation-menu
pagination
popover
progress
radio-group
resizable
scroll-area
select
separator
sheet
sidebar
skeleton
slider
sonner
spinner
switch
table
tabs
textarea
toast
toggle
toggle-group
tooltip
typography
```

Feature names are user-facing dependency API. Renaming any feature after first
publish should be treated as a breaking change for crate-mode users.

## Source-copy Surface

Current source-copy templates:

```text
accordion
alert
alert_dialog
aspect_ratio
attachment
avatar
badge
breadcrumb
bubble
button
button_group
calendar
card
carousel
chart
checkbox
collapsible
combobox
command
context_menu
data_table
date_picker
dialog
direction
drawer
dropdown
empty
field
hover_card
input
input_group
input_otp
item
kbd
label
marker
menubar
message
message_scroller
native_select
navigation_menu
pagination
popover
progress
radio_group
resizable
scroll_area
select
separator
sheet
sidebar
skeleton
slider
sonner
spinner
switch
table
tabs
textarea
toast
toggle
toggle_group
tooltip
typography
utils
```

Source-copy users can edit generated files, but registry slugs, generated file
paths, and shared `utils` behavior still form a CLI compatibility surface.

## Primitive Surface

Current primitive module groups:

```text
active_descendant
calendar
chart
data_table
dialog
dismissal
dropdown
feedback
input_otp
layout
message_scroller
overlay
placement
popover
roving_focus
runtime
select
slider
tooltip
typeahead
```

High-risk primitive surfaces:

- overlay, placement, portal, focus, and dismissal config types
- runtime adapter traits and request/result types
- roving focus and active descendant state helpers
- select, dropdown, dialog, popover, and tooltip primitive config types
- data table, message scroller, input OTP, calendar, carousel, slider, and
  resizable state helpers

These APIs are likely to be reused directly by application code. Before first
publish, maintainers should decide whether they are provisional `0.1.x` APIs
or need a stabilization pass.

## Core Surface

Current core exports:

- `classes`
- `UiDensity`
- `RegistryComponent`
- `RegistryFile`
- `RegistryAsset`

The registry structs are tooling-facing API. Changes affect CLI, registry, and
external tooling integrations more than visual component usage.

## Risk Notes

| Surface | Risk |
| --- | --- |
| Visual-only composition components | `low` unless prop naming is inconsistent. |
| Feature flags and registry slugs | `medium` because downstream imports and CLI commands depend on them. |
| Class helper functions and constants | `medium` because crate-mode users may import them directly. |
| Overlay, focus, placement, runtime, and selection primitives | `high` because they shape app behavior contracts. |
| Source-copy template internals | `medium` for generated file layout, `low` for editable styling details. |

## Validation

Use these commands after updating this inventory:

```bash
npm run verify:api-stability-readiness
npm run verify:docs
npm run verify:release-docs
npm run verify:publish-readiness-blockers
npm run verify:package-scripts
git diff --check
```

This inventory intentionally leaves the API stability blocker unresolved.
