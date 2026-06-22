# RFC 0007: Keyboard Navigation Primitives

- Status: Draft
- Created: 2026-06-22

## Summary

Define shared keyboard navigation primitives before implementing complex
components such as Radio Group, Toggle Group, Slider, Command, Combobox, Menubar,
Context Menu, and Navigation Menu.

## Goals

- Centralize roving focus behavior.
- Provide typeahead matching for menu-like collections.
- Support active descendant state for composite widgets.
- Keep primitives independent from Tailwind styling.
- Work across Dioxus Web, Desktop, and Mobile where platform support allows.

## Non-Goals

- Implement concrete complex components in this RFC.
- Define visual styling.
- Replace native controls when native semantics are sufficient.

## Primitive Concepts

### Roving Focus

Roving focus keeps one item in a composite widget tabbable while arrow keys move
focus between items.

State:

- `items`: ordered item IDs
- `active_id`: currently focusable item
- `orientation`: horizontal, vertical, or both
- `looping`: whether navigation wraps at boundaries
- `disabled_items`: items skipped during navigation

Expected keys:

- `ArrowRight` / `ArrowDown`: next enabled item
- `ArrowLeft` / `ArrowUp`: previous enabled item
- `Home`: first enabled item
- `End`: last enabled item

### Typeahead

Typeahead matches typed characters against item text.

State:

- `buffer`: recent printable characters
- `last_input_at`: timestamp used to reset the buffer
- `timeout_ms`: default 700ms

Rules:

- Ignore modifier shortcuts.
- Match enabled items only.
- Search from the item after the active item.
- Reset the buffer when the timeout expires.

### Active Descendant

Active descendant keeps DOM focus on a container while `aria-activedescendant`
points at the active item.

Use this for:

- Command
- Combobox
- Select-like listboxes
- virtualized collections

Avoid this for:

- simple radio groups where direct item focus is clearer
- native inputs that already provide expected keyboard behavior

## Proposed Rust Shapes

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavigationOrientation {
  Horizontal,
  Vertical,
  Both,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RovingFocusState {
  pub active_id: Option<String>,
  pub orientation: NavigationOrientation,
  pub looping: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeaheadState {
  pub buffer: String,
  pub timeout_ms: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveDescendantState {
  pub active_id: Option<String>,
}
```

The final implementation should avoid storing DOM handles in public state. Any
platform-specific focus command should live behind small internal helpers.

## Component Mapping

| Primitive | Components |
| --- | --- |
| Roving focus | Radio Group, Toggle Group, Menubar, Context Menu, Navigation Menu |
| Typeahead | Select, Dropdown, Command, Combobox, Menubar |
| Active descendant | Command, Combobox, virtualized Select |
| Escape dismissal | Dialog, Popover, Tooltip, Dropdown, Menubar, Context Menu |

## Platform Notes

Web:

- Prefer standard keyboard events.
- Use `tabindex` and `aria-activedescendant` where appropriate.
- Let native inputs keep native behavior unless a composite widget requires
  interception.

Desktop:

- Keyboard behavior should match web where possible.
- Focus APIs may differ by renderer; primitive state must not assume direct DOM
  access is always available.

Mobile:

- Touch interaction may be primary.
- Keyboard primitives still matter for hardware keyboards and accessibility
  tooling.
- Avoid hover-only requirements.

## Validation

Before using these primitives in components:

- Add unit tests for next/previous item selection.
- Add tests for disabled-item skipping.
- Add tests for looping and non-looping behavior.
- Add tests for typeahead buffer reset and matching.
- Add docs examples for each supported interaction pattern.
