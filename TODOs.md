# TODOs

## Progress

- Overall: 0%
- Current milestone: M31 Low-risk Composition Gaps
- Current task: M31.2 Implement Input Group

## Backup

- Previous completed plan: `TODOs.completed-20260628192208.md`

## M30 shadcn Current Gap Planning

- DONE M30.1 Verify current shadcn parity gaps
  - Recheck current shadcn/ui component catalog against local `registry/`, `templates/`, crate features, and docs.
  - Classify missing components into low-risk composition, form-specific, chat/message, chart, and runtime-dependent groups.
  - Update parity, component catalog, and complex batch docs before implementation.

- DONE M30.2 Plan low-risk composition gap APIs
  - Define Button Group, Input Group, Collapsible, and Direction API boundaries.
  - Decide which components are pure styled composition versus primitive-backed state.
  - Document source-copy, accessibility, Web/Desktop/Mobile, and Tailwind token constraints.

- DONE M30.3 Plan form-specific gap APIs
  - Define Input OTP API boundaries, state ownership, keyboard behavior, paste handling, and accessibility semantics.
  - Decide whether Input OTP needs primitive helpers or can remain a controlled styled part.
  - Document generated template expectations and test strategy.

- DONE M30.4 Plan message and AI-style component APIs
  - Define Attachment, Bubble, Message, Message Scroller, and Marker API boundaries.
  - Decide which parts are generic chat/message composition versus AI-specific convenience parts.
  - Keep uploads, streaming, markdown parsing, virtualized scrolling, and model/provider behavior app-owned.

## M31 Low-risk Composition Gaps

- DONE M31.1 Implement Button Group
  - Add crate-mode styled Button Group root/item APIs.
  - Add CLI template, registry entry, docs page, and demo usage.
  - Verify source-copy output compiles without internal crate imports.

- TODO M31.2 Implement Input Group
  - Add input group root, addon, control, and action composition parts.
  - Preserve native input semantics and label/description ownership.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M31.3 Implement Collapsible
  - Add controlled Collapsible root, trigger, and content APIs.
  - Reuse disclosure state helpers where practical and expose ARIA-expanded semantics.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M31.4 Implement Direction
  - Add direction/provider-style composition helper if it can stay source-copy friendly.
  - Document RTL/LTR class and attribute ownership.
  - Add CLI template, registry entry, docs page, and demo usage if accepted as a public component.

- TODO M31.5 Complete low-risk gap documentation and quality gates
  - Update parity, component catalog, accessibility, and complex batch docs.
  - Run generated fixture smoke, workspace tests, and feature checks.
  - Mark M31 complete only after all new components are committed.

## M32 Form-specific Gap

- TODO M32.1 Implement Input OTP primitives or helpers
  - Add pure helpers for slot index, paste distribution, deletion behavior, and completion state if needed.
  - Add unit tests for boundaries, invalid characters, paste overflow, and disabled slots.
  - Keep actual value storage controlled by the consuming app.

- TODO M32.2 Implement Input OTP component
  - Add controlled root, group, slot, separator, and hidden/native input strategy where appropriate.
  - Add keyboard and accessibility notes for screen readers and mobile keyboards.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M32.3 Complete Input OTP documentation and quality gates
  - Update parity, component catalog, accessibility, and form docs.
  - Verify generated fixture smoke and feature checks include Input OTP.
  - Keep validation and submission app-owned.

## M33 Message and Attachment Components

- TODO M33.1 Implement Attachment
  - Add attachment root, preview, metadata, action, and remove/download slots.
  - Keep file upload, object URLs, drag-drop, and network state app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M33.2 Implement Bubble
  - Add message bubble composition parts with sender/receiver variants and density options.
  - Keep markdown, syntax highlighting, and rich content parsing app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M33.3 Implement Message
  - Add message root, avatar slot, header, content, footer/actions, and status parts.
  - Support user/assistant/system-style variants without coupling to a provider.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M33.4 Implement Marker
  - Add inline marker/highlight part for cited, selected, or annotated content.
  - Keep search indexing, citation resolution, and popover details app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M33.5 Complete message component docs and examples
  - Update parity, component catalog, accessibility, and complex batch docs.
  - Add web/desktop examples for static message states.
  - Run generated fixture smoke, workspace tests, and feature checks.

## M34 Message Scroller and Runtime Follow-through

- TODO M34.1 Plan Message Scroller runtime boundaries
  - Define scroll-to-bottom, sticky-at-bottom, unread marker, and streaming update behavior.
  - Decide what can be pure state and what needs measurement/scroll runtime adapters.
  - Keep virtualization and async stream ownership app-owned.

- TODO M34.2 Implement Message Scroller state helpers
  - Add pure helpers for bottom threshold, unread marker visibility, and scroll intent.
  - Add unit tests for streaming append, user-scrolled-away, and reset behavior.
  - Avoid DOM measurement inside primitive state.

- TODO M34.3 Implement Message Scroller component
  - Add controlled scroller root, viewport, content, bottom anchor, unread marker, and jump button parts.
  - Wire only visible status attributes; leave actual scroll commands app/runtime-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- TODO M34.4 Add browser assertions for Web runtime verification
  - Add an expensive browser-level command for runtime Web fixture checks.
  - Cover focus, portal, timers, live-region, measurement, pointer, gesture, and message scroller prerequisites.
  - Keep the check separate from default release gates until stable.

- TODO M34.5 Complete message scroller docs and quality gates
  - Update parity, accessibility, runtime docs, and quality gate docs.
  - Run workspace tests, feature checks, generated fixture smoke, and runtime fixture commands.
  - Document any deferred Desktop/Mobile scroller behavior.

## M35 Chart Public Component Preparation

- TODO M35.1 Plan first-party SVG Chart API
  - Define ChartRoot, ChartSvg, ChartTitle, ChartDescription, ChartLegend, ChartFallbackTable, and ChartTooltipSlot APIs.
  - Limit first public chart types to line, bar, and area.
  - Keep Plotters and Charming adapters out of default generated source.

- TODO M35.2 Build chart example fixture before public component
  - Add example-only SVG chart rendering using existing chart primitives.
  - Verify measurement, responsive sizing, fallback table, and reduced-motion behavior.
  - Avoid adding `registry/chart.json` until the fixture is validated.

- TODO M35.3 Implement public Chart component if gates pass
  - Add crate-mode and source-copy Chart composition parts.
  - Add registry entry, template, docs page, and demo usage.
  - Clearly document backend ownership and fallback-table requirements.

- TODO M35.4 Complete chart public component quality gates
  - Run workspace tests, feature checks, generated fixture smoke, and runtime verification commands.
  - Update parity, accessibility, chart strategy, chart recipes, and component catalog docs.
  - Keep external backend adapters deferred unless explicitly approved.

## M36 Release Hardening for Expanded Parity

- TODO M36.1 Update registry/docs/template consistency checks
  - Ensure new public components have registry entries, templates, docs pages, and catalog links.
  - Extend generated fixture smoke coverage to every new component.
  - Keep `utils` and docs-only chart recipes excluded where appropriate.

- TODO M36.2 Update examples and screenshots strategy
  - Add representative web and desktop demo states for new composition, form, message, and chart components.
  - Document any visual verification gaps before claiming parity.
  - Avoid adding a marketing landing page instead of usable component previews.

- TODO M36.3 Final parity audit against shadcn current catalog
  - Recheck upstream shadcn/ui docs after M31-M35.
  - Mark implemented, deferred, and app-owned components with explicit reasons.
  - Create the next TODO plan if upstream adds new public components.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
