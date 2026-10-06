# RFC 0074: Helper Templates

- Status: Accepted
- Created: 2026-10-06

## Summary

Split the source-copy `utils.rs` template into one helper template per
crate module, so `dxui add` copies only the helpers a component uses.

## Current State

Every component depends on the `utils` registry entry, whose template holds
every shared helper: class composition, the overlay types, and the anchored
overlay, listbox, roving group, modal focus, dismiss timer, hover open,
dialog label, menu, and media query helpers with their page scripts, 1435
lines in all. `dxui add button` copies all of it next to a 102-line button,
and a new app that adds Button prints 66 warnings, 63 of them for `utils.rs`
items Button never uses. A copy user reading or diffing their copies sees
code that has nothing to do with the component they added.

The crate already keeps these helpers in separate modules
(`anchored_overlay.rs`, `listbox.rs`, and so on), and the template parity
test (RFC 0066) compares each template with the crate module of the same
file name.

## Decision

### Templates

Each helper template is named after the crate module it copies, so the
parity test compares the two item by item in both directions, as it does
for components:

| Template | Holds | Uses |
| --- | --- | --- |
| `utils.rs` | `classes`, `UiDensity` | |
| `default_attribute.rs` | `default_attribute` | |
| `overlay.rs` | the overlay types and primitive configs from the primitives crate | |
| `anchored_overlay.rs` | `use_anchored_overlay` and its script | `overlay` |
| `modal_focus.rs` | `use_modal_focus_scope` and its script | |
| `dismiss_timer.rs` | `use_dismiss_timer` and its script | |
| `listbox.rs` | `use_listbox`, `ListboxMode`, and the script | |
| `roving_group.rs` | `use_roving_group`, `group_part_id`, and the script | |
| `hover_open.rs` | `use_hover_open` and its script | |
| `dialog_labels.rs` | `use_dialog_labels` and its parts | |
| `menu_marks.rs` | the checkbox, radio, and sub trigger mark classes | |
| `menu_sub.rs` | the submenu hooks | `anchored_overlay`, `listbox`, `overlay` |
| `media_query.rs` | `use_media_query` and its script | |

`overlay.rs` has no crate module of its own name. Its items come from the
primitives crate's `overlay`, `dialog`, `popover`, `tooltip`, `select`, and
`dropdown` modules; the parity test already compares template items that are
missing from the crate module with the primitives crate. The primitive
configs stay in copy mode, since component docs list them as exports.

Components import each helper from its own module, such as
`use super::listbox::{ListboxMode, use_listbox};`.

### Registry

A helper's registry entry lives in `crates/dioxus-shadcn-cli/helpers/`,
named after it with hyphens, such as `anchored-overlay.json`. Like blocks
(RFC 0073), helpers get their own directory: `registry/` then holds exactly
the public components, and the tools that read it (`dxui list`, the
catalog, the docs and feature checks) drop their `utils` exceptions. `utils`
moves there too.

A component's or helper's `dependencies` name exactly the modules its
template imports through `super::`, and `dxui add` already copies
dependencies transitively. A registry test reads each template's imports
and fails on a missing or extra dependency.

### Apps that copied 0.3 or earlier

Their `utils.rs` holds every helper. `dxui add` keeps it, since it keeps
existing files, and components added later bring the new helper files, so
the app still builds. Components copied before and after then use separate
copies of the same helper, and each copy numbers its element ids from zero,
so two overlays of different ages mounted together can pick the same id.
When `utils.rs` still defines a helper hook, `dxui add` prints a note that
suggests re-copying the older components with `--overwrite`. The 0.4.0
migration notes say the same.

## Alternatives

- **Keep `utils.rs` and silence its warnings.** The warnings go, but a copy
  of Button still carries 1300 lines of overlay code, and every helper change
  shows up as a difference in every app.
- **Ids that cannot collide across copies.** The helpers would need a source
  of ids shared by every copy, or new id formats in the crate as well, to
  serve apps that mix copies from two releases; re-copying fixes those apps.
- **A `helper` field in the registry type.** Every reader of the registry
  would need to check it, and `RegistryComponent` would gain a field.

## Verification

- Template parity: each helper template matches its crate module.
- Registry tests: each template's `super::` imports equal its registry
  dependencies; each dependency names a component or a helper.
- CLI tests: `dxui add button` writes `button.rs` and `utils.rs` only, and
  `dxui list` prints no helper; adding into an app with an old `utils.rs`
  prints the note.
- The generated fixture app with every component and block builds, and so
  does a 0.3.0 copy-mode app after adding a component with the new CLI.
