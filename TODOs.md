# TODOs

## Progress

- Overall: 75%
- Current milestone: M4 Primitive-First Components
- Current task: M4.4 Implement Select and Dropdown primitives

## M0 Documentation

- DONE M0.1 Project documentation foundation
  - Create README with product direction, planned usage, and references.
  - Create docs index, design overview, roadmap, and initial RFCs.
  - Define the task sequence for implementation.

- DONE M0.2 Workspace architecture specification
  - Specify the Cargo workspace layout.
  - Define crate responsibilities and dependency direction.
  - Document feature flag strategy.
  - Document component module boundaries and platform profile strategy.

- DONE M0.3 Component API specification
  - Specify common props, class extension behavior, and children handling.
  - Define naming conventions for variants, sizes, and events.
  - Document accessibility requirements per component category.

## M1 Project Skeleton

- DONE M1.1 Convert crate into a Cargo workspace
  - Add `crates/dioxus-ui-core`.
  - Add `crates/dioxus-ui-primitives`.
  - Add `crates/dioxus-ui`.
  - Add `crates/dioxus-ui-cli`.

- DONE M1.2 Add registry and template directories
  - Add initial component registry schema.
  - Add placeholder templates for Button and Input.
  - Add validation tests for registry metadata.

- DONE M1.3 Add examples
  - Add `examples/web-demo`.
  - Add `examples/desktop-demo`.
  - Document how to run each example.

## M2 Foundation Components

- DONE M2.1 Implement core class utilities
  - Provide deterministic class composition.
  - Support user class overrides without dynamic Tailwind token construction.

- DONE M2.2 Implement Button
  - Add styled component.
  - Add CLI template.
  - Add example usage.

- DONE M2.3 Implement Input, Textarea, and Label
  - Add styled components.
  - Add CLI templates.
  - Add example usage.

## M3 Stateful Components

- DONE M3.1 Implement Checkbox and Switch
- DONE M3.2 Implement Tabs
- DONE M3.3 Implement Accordion

## M4 Primitive-First Components

- DONE M4.1 Design focus and portal primitives
- DONE M4.2 Implement Dialog primitive and styled Dialog
- DONE M4.3 Implement Popover and Tooltip primitives
- TODO M4.4 Implement Select and Dropdown primitives

## M5 Distribution

- TODO M5.1 Implement `dxui init`
- TODO M5.2 Implement `dxui add`
- TODO M5.3 Publish crate-mode package strategy
- TODO M5.4 Build documentation site and component previews

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- If this directory is not a git repository, record completion in notes but do
  not mark implementation tasks as `DONE`.
