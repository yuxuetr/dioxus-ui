# Public API Surface Inventory

This inventory records the current public API surfaces that need maintainer
review before the API stability blocker can be resolved. It is descriptive and
does not approve API stability.

Use it with:

- [API Stability Surface Audit Plan](archive/first-publish/api-stability-surface-audit-plan.md)
- [API Stability Readiness Metadata](archive/first-publish/api-stability-readiness-metadata.md)
- [API Stability Decision Preparation Plan](archive/first-publish/api-stability-decision-preparation-plan.md)
- [API Stability Decision Record Template](archive/first-publish/api-stability-decision-record-template.md)
- [API Stability Local Follow-up Map](archive/first-publish/api-stability-local-follow-up-map.md)
- [API Stability Review Checklist](archive/first-publish/api-stability-review-checklist.md)
- [Publish Readiness Decision Matrix](archive/first-publish/publish-readiness-decision-matrix.md)

## Summary

| Surface | Count | User-facing Mode | Notes |
| --- | ---: | --- | --- |
| Styled component crate modules | 82 | Crate mode | Feature-gated modules under `crates/dioxus-shadcn/src`. |
| Styled component features | 82 | Crate mode | Public Cargo feature names in `crates/dioxus-shadcn/Cargo.toml`. |
| Source-copy templates | 95 | Source-copy mode | 82 component templates plus 13 helper templates (RFC 0074). |
| Registry entries | 95 | CLI/source-copy mode | 82 component entries in `registry/` plus 13 helper entries in `helpers/`. |
| Registry schema | 1 | Tooling metadata | `crates/dioxus-shadcn-cli/registry/schema.json` validates registry shape and is not a component. |
| Primitive modules | 18 | Crate mode | Public helpers, config types, and runtime adapter traits in `dioxus-shadcn-primitives`, under its own semver promise (RFC 0079). |
| Core crate exports | 4 groups | Crate/tooling mode | `classes`, `merge_classes`, `UiDensity`, and registry data structs. |

## Styled Component Features

Current `dioxus-shadcn` feature names:

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
countdown
data-table
date-picker
dialog
diff
direction
dock
drawer
dropdown
empty
fab
field
file-input
hover-card
indicator
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
number-input
pagination
popover
progress
radial-progress
radio-group
rating
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
stat
status
steps
swap
switch
table
tabs
tags-input
textarea
timeline
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
anchored_overlay
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
countdown
data_table
date_picker
default_attribute
dialog
dialog_labels
diff
direction
dismiss_timer
dock
drawer
dropdown
empty
fab
field
file_input
hover_card
hover_open
indicator
input
input_group
input_otp
item
kbd
label
listbox
marker
media_query
menu
menu_marks
menu_sub
menubar
message
message_scroller
mockup
modal_focus
native_select
navigation_menu
number_input
overlay
pagination
popover
progress
radial_progress
radio_group
rating
resizable
roving_group
scroll_area
select
separator
sheet
sidebar
skeleton
slider
sonner
spinner
stat
status
steps
swap
switch
table
tabs
tags_input
textarea
theme_controller
timeline
toast
toggle
toggle_group
tooltip
typography
utils
```

Source-copy users can edit generated files, but registry slugs, generated file
paths, and the helper templates' behavior still form a CLI compatibility
surface.

## Primitive Surface

Current primitive module groups:

```text
active_descendant
calendar
chart
data_table
dialog
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
```

High-risk primitive surfaces:

- overlay, placement, portal, focus, and dismissal config types
- runtime adapter traits and request/result types
- roving focus and active descendant state helpers
- select, dropdown, dialog, popover, and tooltip primitive config types
- data table, message scroller, input OTP, calendar, carousel, slider, and
  resizable state helpers

The primitives crate keeps a semver promise of its own and documents every
public item (RFC 0079): an app may depend on it directly, for example to
implement the runtime adapter traits. 0.6.0 removed what nothing used: the
`dismissal` and `typeahead` modules, the toast and Sonner runtime request
helpers, and `slider_snap` and `slider_percent`.
`node scripts/public-surface.mjs` lists the current items and where each is
used.

## Core Surface

Current core exports:

- `classes`
- `merge_classes`
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
| Class helper functions | `medium` because crate-mode users may import the ones component pages list; class constants are private (RFC 0079). |
| Overlay, focus, placement, runtime, and selection primitives | `high` because they shape app behavior contracts. |
| Source-copy template internals | `medium` for generated file layout, `low` for editable styling details. |

## Validation

Use these commands after updating this inventory:

```bash
npm run verify:docs
npm run verify:release-docs
npm run verify:package-scripts
git diff --check
```

The API stability blocker this inventory supported was resolved for the first
publish; see the [First Publish Archive](archive/first-publish/README.md).
