# TODOs

## Progress

- Overall: 33%
- Current milestone: M12 Overlay Variants
- Current task: M12.3 Implement Sheet

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

- DONE M7.1 Implement Badge, Separator, and Skeleton
  - Add crate-mode styled components.
  - Add CLI templates and registry entries.
  - Add component docs pages and catalog links.
  - Verify generated fixture compile smoke includes the new components.

- DONE M7.2 Implement Alert and Card
  - Add composable styled parts for title, description, header, content, and footer where applicable.
  - Add CLI templates and registry entries.
  - Add class utility tests for variants and user class extension.
  - Add component docs pages and examples.

- DONE M7.3 Implement Avatar and Progress
  - Add styled component APIs for image/fallback avatar usage and progress value rendering.
  - Add CLI templates and registry entries.
  - Document accessibility expectations for fallback labels and progress semantics.
  - Verify generated fixture compile smoke includes the new components.

- DONE M7.4 Implement Table and Pagination
  - Add styled table parts for header, body, row, cell, caption, and footer.
  - Add pagination parts for previous, next, item, ellipsis, and link states.
  - Add CLI templates and registry entries.
  - Add docs pages with usage, crate feature usage, and accessibility notes.

- DONE M7.5 Add static component examples
  - Update web and desktop demos to render the new static component set.
  - Include density, variant, disabled, and empty-state examples where applicable.
  - Keep examples source-copy-compatible with generated template APIs.

- DONE M7.6 Update parity tracking documentation
  - Add a shadcn parity matrix for implemented, planned, and deferred components.
  - Group missing components by static, light interaction, overlay/menu, form/date, data display, and layout.
  - Use the matrix to seed the next milestone instead of mixing complex components into M7.

## M8 Complex Interaction Preparation

- DONE M8.1 Design shared keyboard navigation primitives
  - Define roving focus, typeahead, active descendant, and escape-key behavior.
  - Specify reusable APIs for menu, combobox, command, radio group, and navigation components.
  - Document Web/Desktop/Mobile constraints before implementation.

- DONE M8.2 Plan overlay positioning and portal behavior
  - Define placement, collision handling, anchor measurement, and portal defaults.
  - Decide what belongs in primitives versus styled components.
  - Document limitations for native desktop and mobile targets.

- DONE M8.3 Prioritize complex component batches
  - Rank Calendar, Date Picker, Combobox, Command, Context Menu, Menubar, Navigation Menu, Sheet, Toast, Scroll Area, Slider, and Radio Group.
  - Split them into implementable milestones with explicit dependency order.
  - Identify which components need third-party logic crates versus first-party primitives.

## M9 Quality Baseline

- DONE M9.1 Add registry, docs, and template consistency checks
  - Verify every public registry component has a template.
  - Verify every public registry component has a docs page.
  - Verify docs catalog links match registry component names.
  - Keep `utils` excluded from public catalog checks.

- DONE M9.2 Add per-feature crate compile checks
  - Check each `dioxus-ui` feature compiles independently.
  - Check representative feature combinations compile.
  - Add a script or test harness that can run locally and in CI.
  - Document expected runtime cost and when to run it.

- DONE M9.3 Harden generated fixture smoke
  - Ensure generated fixture smoke validates all public components.
  - Add assertions for generated `mod.rs` content.
  - Add checks that generated templates do not import `dioxus-ui-core` or `dioxus-ui-primitives`.
  - Document the smoke command as a release gate.

- DONE M9.4 Add accessibility contract checklist
  - Document expected roles, ARIA attributes, keyboard behavior, and labeling per component group.
  - Mark which contracts are currently implemented, planned, or intentionally deferred.
  - Link the checklist from component docs and complex component batch planning.

- DONE M9.5 Add CI quality gate plan
  - Define the exact command set for local and CI verification.
  - Include `cargo check`, `cargo test`, generated fixture smoke, and feature compile checks.
  - Document known expensive checks separately from default checks.

## M10 Interaction Primitives

- DONE M10.1 Implement roving focus state primitives
  - Add orientation, looping, disabled-item skipping, and active item state.
  - Add pure unit tests for next, previous, first, last, and boundary behavior.
  - Keep primitive state independent from DOM handles.

- DONE M10.2 Implement typeahead primitives
  - Add buffer state and timeout handling.
  - Add matching over enabled item labels.
  - Add tests for repeated characters, multi-character search, and timeout reset.

- DONE M10.3 Implement active descendant primitives
  - Add state types for active item IDs.
  - Specify container and item attribute helpers.
  - Add tests for active item transitions and empty collections.

- DONE M10.4 Implement dismissal primitives
  - Add reusable escape-key and outside-interaction state helpers.
  - Integrate with existing dismiss behavior types where possible.
  - Add tests for enabled and disabled dismissal paths.

- DONE M10.5 Implement overlay placement math primitives
  - Add placement, collision padding, and collision strategy types.
  - Add pure placement tests for flip, shift, and no-collision behavior.
  - Keep runtime measurement adapters out of the public state model.

- DONE M10.6 Prepare first interaction component batch
  - Use M10 primitives to define implementation specs for Radio Group, Toggle, Toggle Group, Slider, and Spinner.
  - Decide which components require primitive crate APIs versus styled-only APIs.
  - Create the next TODO milestone from the specs.

## M11 Interaction Component Batch 1

- DONE M11.1 Implement Spinner
  - Add crate-mode styled component with size variants and accessible status semantics.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Verify generated fixture smoke includes Spinner.

- DONE M11.2 Implement Toggle
  - Add controlled styled component with pressed, disabled, variant, and size states.
  - Add `aria-pressed` semantics in crate-mode and generated template APIs.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M11.3 Implement Radio Group
  - Add primitive-backed styled Radio Group and Radio Group Item APIs.
  - Reuse roving focus primitives for orientation, looping, and disabled-item skipping.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M11.4 Implement Toggle Group
  - Add single and multiple selection modes.
  - Reuse roving focus primitives for grouped keyboard navigation.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M11.5 Implement Slider state primitive
  - Add pure value helpers for clamp, step, percentage, and keyboard delta behavior.
  - Add unit tests for boundaries, step rounding, and min/max ranges.
  - Keep runtime pointer measurement outside the primitive state model.

- DONE M11.6 Implement Slider styled component
  - Add styled root, track, range, and thumb APIs for horizontal sliders.
  - Map controlled values to ARIA attributes and range percentage classes/styles.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M11.7 Complete batch documentation and examples
  - Update parity, accessibility, and component catalog docs for M11 components.
  - Ensure web and desktop demos cover disabled, focused, selected, and boundary states.
  - Run release quality gates before marking the batch complete.

## M12 Overlay Variants

- DONE M12.1 Plan overlay variant APIs
  - Define Alert Dialog, Sheet, Drawer, and Hover Card API boundaries.
  - Decide which variants reuse Dialog, Popover, Tooltip, and overlay placement primitives.
  - Document portal, dismissal, focus, Web/Desktop/Mobile constraints before implementation.

- DONE M12.2 Implement Alert Dialog
  - Add primitive-backed styled parts for alert dialog overlay, content, title, description, action, and cancel.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Verify accessibility notes cover modal role, focus return, and destructive action labeling.

- TODO M12.3 Implement Sheet
  - Add side-based sheet content classes and dialog-backed state configuration.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Verify generated fixture smoke includes Sheet.

- TODO M12.4 Implement Drawer
  - Decide whether Drawer is an alias/composition of Sheet or a separate mobile-oriented component.
  - Add source-copy and crate-mode APIs only after the distinction is documented.
  - Add docs page and demo usage if implemented as a public registry component.

- TODO M12.5 Implement Hover Card
  - Reuse popover/tooltip positioning and dismissal primitives where possible.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Document hover/focus behavior and mobile fallback constraints.

- TODO M12.6 Complete overlay variant documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- If this directory is not a git repository, record completion in notes but do
  not mark implementation tasks as `DONE`.
