# TODOs

## Progress

- Overall: 100%
- Current milestone: M22 Focus And Portal Adapter Contracts
- Current task: M23.1 Design timer and live-region contract implementation

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

- DONE M12.3 Implement Sheet
  - Add side-based sheet content classes and dialog-backed state configuration.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Verify generated fixture smoke includes Sheet.

- DONE M12.4 Implement Drawer
  - Decide whether Drawer is an alias/composition of Sheet or a separate mobile-oriented component.
  - Add source-copy and crate-mode APIs only after the distinction is documented.
  - Add docs page and demo usage if implemented as a public registry component.

- DONE M12.5 Implement Hover Card
  - Reuse popover/tooltip positioning and dismissal primitives where possible.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Document hover/focus behavior and mobile fallback constraints.

- DONE M12.6 Complete overlay variant documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M13 Menu Systems

- DONE M13.1 Plan menu system APIs
  - Define Context Menu, Menubar, and Navigation Menu API boundaries.
  - Decide which parts reuse roving focus, typeahead, dismissal, and placement primitives.
  - Document nested menu, Web/Desktop/Mobile, and source-copy constraints before implementation.

- DONE M13.2 Implement Context Menu
  - Add primitive-backed styled parts for root content, group, item, label, separator, checkbox item, radio item, and submenu placeholders.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Verify keyboard and accessibility notes cover menu roles, roving focus, and typeahead limits.

- DONE M13.3 Implement Menubar
  - Add horizontal menu root and trigger/content/item parts.
  - Reuse roving focus and typeahead primitives where possible.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M13.4 Implement Navigation Menu
  - Add navigation root, list, item, trigger, content, link, and viewport-style parts.
  - Document when to use navigation semantics versus menu semantics.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M13.5 Complete menu system documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M14 Command and Choice

- DONE M14.1 Plan command and choice APIs
  - Define Command, Combobox, and Native Select API boundaries.
  - Decide which parts reuse active descendant, typeahead, filtering, selection, and popover primitives.
  - Document Web/Desktop/Mobile, form integration, and source-copy constraints before implementation.

- DONE M14.2 Implement Command
  - Add controlled command root, input, list, group, item, empty, separator, and shortcut parts.
  - Reuse active descendant and typeahead primitives where possible.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M14.3 Implement Combobox
  - Add controlled trigger/input/content/list/item/value APIs for searchable selection.
  - Reuse popover, active descendant, and typeahead primitives where possible.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M14.4 Implement Native Select
  - Add styled native select wrapper, trigger-like select element, option group notes, and invalid/disabled states.
  - Document when to prefer native select over custom Select or Combobox.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M14.5 Complete command and choice documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M15 Calendar and Date Picker

- DONE M15.1 Plan calendar and date picker APIs
  - Define Calendar and Date Picker API boundaries.
  - Decide date math, locale, grid navigation, range selection, and popover composition strategy.
  - Document Web/Desktop/Mobile, source-copy, and form integration constraints before implementation.

- DONE M15.2 Implement calendar date primitives
  - Add pure month grid, date comparison, selection, range, and keyboard movement helpers.
  - Avoid runtime DOM dependencies in primitive state.
  - Add unit tests for month boundaries, leap years, disabled dates, and range selection behavior.

- DONE M15.3 Implement Calendar
  - Add controlled calendar root, header, navigation, grid, row, day, and caption APIs.
  - Reuse calendar date primitives and roving/grid navigation helpers where appropriate.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M15.4 Implement Date Picker
  - Add input/trigger, popover content, selected value display, and calendar composition APIs.
  - Document when to use native inputs, Calendar, or Date Picker.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M15.5 Complete calendar and date picker documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M16 Data and Visualization

- DONE M16.1 Plan data table and chart strategy
  - Define Data Table composition, sorting, filtering, selection, pagination, and empty-state boundaries.
  - Decide whether Chart ships as a component, strategy document, or deferred adapter surface.
  - Document Web/Desktop/Mobile, source-copy, accessibility, and dependency constraints before implementation.

- DONE M16.2 Implement data table state primitives
  - Add pure sorting, pagination window, row selection, and column visibility helpers.
  - Avoid runtime DOM dependencies and data-source ownership.
  - Add unit tests for stable sorting, page boundaries, selection toggles, and hidden columns.

- DONE M16.3 Implement Data Table
  - Add controlled Data Table toolbar, container, header cell, row, cell, pagination, empty state, and selected-count APIs.
  - Compose existing Table, Checkbox, Command, and Pagination patterns where practical.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M16.4 Document Chart strategy
  - Decide the initial chart backend policy and whether charts are first-party, adapter-based, or deferred.
  - Document accessibility, SSR/Web/Desktop constraints, and source-copy expectations.
  - Update parity and roadmap without introducing a chart component before the strategy is clear.

- DONE M16.5 Complete data and visualization documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M17 Layout Shells and Media

- DONE M17.1 Plan layout shell and media APIs
  - Define Sidebar, Scroll Area, Resizable, and Carousel API boundaries.
  - Decide which parts need layout, measurement, persistence, pointer, and gesture primitives.
  - Document Web/Desktop/Mobile, source-copy, accessibility, and dependency constraints before implementation.

- DONE M17.2 Implement layout state primitives
  - Add pure sidebar collapse state, resizable panel math, scroll orientation metadata, and carousel index helpers.
  - Avoid runtime DOM measurement and pointer event ownership in primitives.
  - Add unit tests for collapse toggles, panel clamping, carousel wrapping, and scroll orientation behavior.

- DONE M17.3 Implement Scroll Area
  - Add styled viewport, content, scrollbar, thumb, and corner APIs.
  - Document native scrolling behavior and custom scrollbar limits across Web/Desktop/Mobile.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M17.4 Implement Resizable
  - Add controlled panel group, panel, handle, and helper APIs using layout primitives.
  - Keep pointer dragging and DOM measurement app-owned in the first implementation.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M17.5 Implement Sidebar
  - Add controlled sidebar provider-style shell parts, rail, header, content, footer, group, item, and trigger APIs.
  - Reuse collapse primitives and document persistence as app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M17.6 Implement Carousel
  - Add controlled carousel root, viewport, content, item, previous, next, and indicator APIs.
  - Reuse carousel index primitives and document gesture/autoplay as deferred.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M17.7 Complete layout shell and media documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M18 Feedback Notifications

- DONE M18.1 Plan feedback notification APIs
  - Define Toast and Sonner API boundaries.
  - Decide queue, placement, dismiss, action, live-region, and timing ownership.
  - Document Web/Desktop/Mobile, source-copy, accessibility, and dependency constraints before implementation.

- DONE M18.2 Implement feedback state primitives
  - Add pure toast item, queue, placement, and timeout helper types.
  - Avoid timer ownership, DOM focus ownership, and portal mounting in primitives.
  - Add unit tests for queue add, dismiss, limit, placement, and timeout behavior.

- DONE M18.3 Implement Toast
  - Add controlled viewport, root, title, description, action, close, and provider-style helper APIs.
  - Reuse feedback primitives and document live announcements as app-owned integration.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M18.4 Implement Sonner
  - Add opinionated toast list composition parts and variants for success, info, warning, error, and loading.
  - Reuse feedback primitives and keep timers, promises, and async orchestration app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M18.5 Complete feedback notification documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M19 Chart Follow-through

- DONE M19.1 Plan chart follow-through APIs
  - Define chart data model, scale helper, color token, summary, and recipe boundaries.
  - Decide which chart pieces remain docs-only versus primitive helpers.
  - Document Web/Desktop/Mobile, source-copy, accessibility, and dependency constraints before implementation.

- DONE M19.2 Implement chart data primitives
  - Add pure chart series, point, domain, and scale helper types.
  - Avoid rendering backend, DOM measurement, canvas, SVG generation, and pointer event ownership in primitives.
  - Add unit tests for domain calculation, scale mapping, empty data, and stacked/ranged values.

- DONE M19.3 Implement chart accessibility helpers
  - Add pure helpers for summary text, series labels, value labels, and tabular fallback metadata.
  - Keep localization, formatting, and announcement timing app-owned.
  - Add unit tests for summary output, missing values, color-independent labels, and fallback rows.

- DONE M19.4 Document chart recipes without public component
  - Add docs-only recipes for line, bar, and area chart composition using app-owned rendering.
  - Document why `dxui add chart` and a `chart` feature remain deferred.
  - Update chart strategy with backend evaluation criteria and dependency policy.

- DONE M19.5 Complete chart follow-through documentation
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M20 Static Composition Gaps

- DONE M20.1 Plan static composition APIs
  - Define Aspect Ratio, Kbd, Typography, Breadcrumb, Empty, Field, and Item API boundaries.
  - Decide which components are pure styled wrappers versus small semantic composition parts.
  - Document Web/Desktop/Mobile, source-copy, accessibility, and dependency constraints before implementation.

- DONE M20.2 Implement Aspect Ratio, Kbd, and Typography
  - Add low-risk styled parts for fixed-ratio media slots, keyboard hints, and prose text.
  - Keep ratio math deterministic and class tokens static.
  - Add CLI templates, registry entries, docs pages, and demo usage.

- DONE M20.3 Implement Breadcrumb and Empty
  - Add semantic navigation breadcrumb parts and empty-state composition parts.
  - Keep routing, icons, and actions app-owned.
  - Add CLI templates, registry entries, docs pages, and demo usage.

- DONE M20.4 Implement Field and Item
  - Add form field composition parts and generic list/item composition parts.
  - Keep validation state, descriptions, controls, and data rendering app-owned.
  - Add CLI templates, registry entries, docs pages, and demo usage.

- DONE M20.5 Complete static composition documentation and examples
  - Update parity, accessibility, complex batch, and component catalog docs.
  - Run release quality gates before marking the batch complete.

## M21 Runtime Adapter Planning

- DONE M21.1 Plan runtime adapter boundaries
  - Define which runtime behavior belongs in adapters versus primitives, styled components, and consuming apps.
  - Cover focus commands, portal mounting, timer scheduling, live-region announcements, pointer gestures, and DOM measurement.
  - Document Web/Desktop/Mobile constraints before implementation.

- DONE M21.2 Plan focus and portal adapters
  - Define focus trap, focus return, initial focus, outside focus, and portal target contracts.
  - Map Dialog, Alert Dialog, Sheet, Drawer, Popover, Tooltip, Select, Combobox, and menu components to adapter needs.
  - Identify what can be pure state, what needs DOM/WebView commands, and what remains app-owned.

- DONE M21.3 Plan timer and live-region adapters
  - Define timer scheduling and dismissal ownership for Toast and Sonner.
  - Define live-region announcement wording, queueing, urgency, and duplicate suppression boundaries.
  - Keep async promise orchestration and persisted history app-owned unless explicitly justified.

- DONE M21.4 Plan measurement, pointer, and gesture adapters
  - Define anchor measurement, viewport collision updates, resizable pointer dragging, carousel gestures, and scroll restoration boundaries.
  - Document Web/Desktop/Mobile risks and adapter test strategy.
  - Keep physics, virtualization, and backend-specific rendering out of the first adapter pass.

- DONE M21.5 Complete runtime adapter planning docs
  - Update parity, accessibility, complex batch, roadmap, and component docs links.
  - Decide the next implementable milestone from the adapter plan.
  - Run documentation and quality checks before marking the batch complete.

## M22 Focus And Portal Adapter Contracts

- DONE M22.1 Design focus and portal contract implementation
  - Define the exact module location, feature flags, public exports, and source-copy policy for adapter contracts.
  - Decide whether contracts live in `dioxus-ui-primitives`, `dioxus-ui-core`, or a new runtime module.
  - Document Web/Desktop/Mobile fallback behavior before code changes.

- DONE M22.2 Implement focus adapter contract types
  - Add narrow focus runtime traits or structs with explicit unsupported results.
  - Add tests for focus strategy mapping, unsupported fallback, and modal versus non-modal policy.
  - Avoid DOM or WebView commands in the first contract layer.

- DONE M22.3 Implement portal adapter contract types
  - Add narrow portal runtime traits or structs with explicit target availability.
  - Add tests for inline, body, selector/named target, and unsupported fallback behavior.
  - Keep app shell z-index and target provisioning app-owned.

- DONE M22.4 Add Dialog and Popover adapter contract examples
  - Show how Dialog or Alert Dialog would request modal focus behavior.
  - Show how Popover would request non-modal portal behavior.
  - Keep examples controlled and avoid renderer-specific commands until verified.

- DONE M22.5 Complete focus and portal adapter contract documentation
  - Update runtime adapter docs, accessibility checklist, and complex batch plan.
  - Run workspace tests and documentation checks before marking the batch complete.

## M23 Timer And Live Region Adapter Contracts

- TODO M23.1 Design timer and live-region contract implementation
  - Define the exact module location, feature flags, public exports, and source-copy policy for feedback runtime contracts.
  - Decide whether contracts extend `dioxus-ui-primitives/runtime` or use separate modules.
  - Document Web/Desktop/Mobile fallback behavior before code changes.

- TODO M23.2 Implement timer adapter contract types
  - Add narrow timer runtime request/result types with explicit unsupported results.
  - Add tests for schedule, cancel, missing timer, and unsupported fallback behavior.
  - Avoid owning Toast or Sonner queue mutation in the timer layer.

- TODO M23.3 Implement live-region adapter contract types
  - Add announcement priority and duplicate policy request/result types.
  - Add tests for polite/assertive requests, duplicate policy, and unsupported fallback behavior.
  - Keep announcement wording and localization app-owned.

- TODO M23.4 Add Toast and Sonner adapter contract examples
  - Show how Toast and Sonner would request timer scheduling and live-region announcements.
  - Keep examples controlled and avoid renderer-specific timer or DOM commands until verified.

- TODO M23.5 Complete timer and live-region adapter contract documentation
  - Update runtime adapter docs, accessibility checklist, and complex batch plan.
  - Run workspace tests and documentation checks before marking the batch complete.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- If this directory is not a git repository, record completion in notes but do
  not mark implementation tasks as `DONE`.
