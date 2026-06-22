# TODOs

## Progress

- Overall: 69%
- Current milestone: M7 Static Component Parity
- Current task: M7.1 Implement Badge, Separator, and Skeleton

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
- DONE M4.4 Implement Select and Dropdown primitives

## M5 Distribution

- DONE M5.1 Implement `dxui init`
- DONE M5.2 Implement `dxui add`
- DONE M5.3 Publish crate-mode package strategy
- DONE M5.4 Build documentation site and component previews

## M6 Generated Source Quality

- DONE M6.1 Make generated templates self-contained
  - Remove `dioxus-ui-core` imports from generated templates.
  - Remove `dioxus-ui-primitives` imports from generated templates.
  - Keep crate-mode implementations using shared crates.

- DONE M6.2 Add generated fixture compile smoke
  - Generate a temporary Dioxus app fixture with `dxui init`.
  - Add representative components with `dxui add`.
  - Verify generated source compiles with minimal dependencies.

- DONE M6.3 Improve CLI add safeguards
  - Add `dxui add --overwrite` or documented conflict behavior.
  - Improve error messages for unknown components.
  - Add tests for repeated `dxui add`.

- DONE M6.4 Add component docs pages
  - Add one docs page per implemented component.
  - Include CLI usage, crate feature usage, and accessibility notes.

## M7 Static Component Parity

- TODO M7.1 Implement Badge, Separator, and Skeleton
  - Add crate-mode styled components.
  - Add CLI templates and registry entries.
  - Add component docs pages and catalog links.
  - Verify generated fixture compile smoke includes the new components.

- TODO M7.2 Implement Alert and Card
  - Add composable styled parts for title, description, header, content, and footer where applicable.
  - Add CLI templates and registry entries.
  - Add class utility tests for variants and user class extension.
  - Add component docs pages and examples.

- TODO M7.3 Implement Avatar and Progress
  - Add styled component APIs for image/fallback avatar usage and progress value rendering.
  - Add CLI templates and registry entries.
  - Document accessibility expectations for fallback labels and progress semantics.
  - Verify generated fixture compile smoke includes the new components.

- TODO M7.4 Implement Table and Pagination
  - Add styled table parts for header, body, row, cell, caption, and footer.
  - Add pagination parts for previous, next, item, ellipsis, and link states.
  - Add CLI templates and registry entries.
  - Add docs pages with usage, crate feature usage, and accessibility notes.

- TODO M7.5 Add static component examples
  - Update web and desktop demos to render the new static component set.
  - Include density, variant, disabled, and empty-state examples where applicable.
  - Keep examples source-copy-compatible with generated template APIs.

- TODO M7.6 Update parity tracking documentation
  - Add a shadcn parity matrix for implemented, planned, and deferred components.
  - Group missing components by static, light interaction, overlay/menu, form/date, data display, and layout.
  - Use the matrix to seed the next milestone instead of mixing complex components into M7.

## M8 Complex Interaction Preparation

- TODO M8.1 Design shared keyboard navigation primitives
  - Define roving focus, typeahead, active descendant, and escape-key behavior.
  - Specify reusable APIs for menu, combobox, command, radio group, and navigation components.
  - Document Web/Desktop/Mobile constraints before implementation.

- TODO M8.2 Plan overlay positioning and portal behavior
  - Define placement, collision handling, anchor measurement, and portal defaults.
  - Decide what belongs in primitives versus styled components.
  - Document limitations for native desktop and mobile targets.

- TODO M8.3 Prioritize complex component batches
  - Rank Calendar, Date Picker, Combobox, Command, Context Menu, Menubar, Navigation Menu, Sheet, Toast, Scroll Area, Slider, and Radio Group.
  - Split them into implementable milestones with explicit dependency order.
  - Identify which components need third-party logic crates versus first-party primitives.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- If this directory is not a git repository, record completion in notes but do
  not mark implementation tasks as `DONE`.
