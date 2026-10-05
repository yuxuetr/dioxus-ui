# Command and Choice API Plan

This document defines the M14 command and choice APIs. The goal is to reuse
active descendant, typeahead, selection, and popover primitives while keeping
the first implementation controlled and source-copy friendly.

Status: Implemented in M14.

## Scope

M14 covers:

- Command
- Combobox
- Native Select

These components should ship in both crate mode and source-copy mode. Crate mode
can reuse `dioxus-shadcn-core` and `dioxus-shadcn-primitives`; generated templates must
remain self-contained and must not import internal crates.

## Shared Rules

Command and choice components use controlled state first:

- explicit `value`, `selected`, `active`, `disabled`, `invalid`, and `open`
  props where applicable
- `class: String` on every styled part
- `children: Element` for composition slots

All Tailwind classes must be complete static tokens in source. Runtime selection
between predefined strings is allowed; runtime construction of class names is
not allowed.

Primitive reuse:

- active descendant for input/list relationships
- typeahead for enabled item matching
- popover placement for floating choice panels
- existing select configuration for custom select-like behavior

Deferred runtime work:

- DOM focus commands
- input event filtering adapters
- async result loading
- virtualization
- portal mounting
- full keyboard event wiring

## Command

Command is a composable command palette and searchable action list. It is not a
form control by itself.

Implemented crate API:

```rust
Command { class, children }
CommandInput { value, active_id, disabled, class }
CommandList { active_id, class, children }
CommandEmpty { class, children }
CommandGroup { class, children }
CommandLabel { class, children }
CommandItem { id, active, selected, disabled, class, children }
CommandSeparator { class }
CommandShortcut { class, children }
```

Behavior defaults:

- input uses `role="combobox"`
- list uses `role="listbox"`
- item uses `role="option"`
- active item state maps to `aria-activedescendant` and `data-active`
- selected item state maps to `aria-selected` and `data-selected`

The first implementation exposes pure helpers and styled parts. Filtering,
keyboard events, and command execution are owned by the consuming app.

## Combobox

Combobox is a searchable selection component. It combines trigger/input,
popover-like content, listbox semantics, and selected value display.

Implemented crate API:

```rust
ComboboxTrigger { open, invalid, disabled, class, children }
ComboboxInput { value, active_id, disabled, class }
ComboboxContent { open, class, children }
ComboboxList { active_id, class, children }
ComboboxEmpty { class, children }
ComboboxGroup { class, children }
ComboboxItem { value, active, selected, disabled, class, children }
ComboboxValue { class, children }
```

Behavior defaults:

- trigger uses `role="combobox"`
- content/list use listbox semantics
- item uses `role="option"`
- active descendant is optional and controlled
- popover-like dismissal and placement are reused through primitive config

Use Combobox when users need search or filtering before selecting a value. Use
Native Select for browser-native forms and simple option lists.

## Native Select

Native Select is a styled wrapper around the browser/platform native select
element. It should be the low-risk default for simple forms.

Implemented crate API:

```rust
NativeSelect { invalid, disabled, class, children }
NativeSelectGroup { label, class, children }
NativeSelectOption { value, disabled, selected, class, children }
```

Behavior defaults:

- use native `select`, `optgroup`, and `option` elements
- preserve form submission behavior
- map invalid state to `aria-invalid`
- do not replace native keyboard behavior

Native Select is documented separately from custom Select and Combobox.

## Platform Defaults

| Target | Choice components |
| --- | --- |
| Web | Prefer native inputs/selects where possible; use active descendant for custom lists. |
| Desktop | Match web semantics, but avoid assuming direct DOM focus commands. |
| Mobile | Prefer Native Select or Sheet-style flows for long lists; avoid hover-only behavior. |

## Implementation Order

1. Command
2. Combobox
3. Native Select
4. documentation, examples, and parity updates

This order started with the lowest-risk active descendant surface, then applied
the same model to searchable selection, then added a native form-friendly
option.

## Quality Gates

Each M14 component should include:

- crate-mode class composition tests
- primitive helper tests when active descendant or typeahead helpers are exposed
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking each task done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
