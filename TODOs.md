# TODOs

## Progress

- Overall: 100%
- Current milestone: M41 Mobile Browser Smoke Feasibility
- Current task: M41 complete; next milestone pending

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

- DONE M31.2 Implement Input Group
  - Add input group root, addon, control, and action composition parts.
  - Preserve native input semantics and label/description ownership.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M31.3 Implement Collapsible
  - Add controlled Collapsible root, trigger, and content APIs.
  - Reuse disclosure state helpers where practical and expose ARIA-expanded semantics.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M31.4 Implement Direction
  - Add direction/provider-style composition helper if it can stay source-copy friendly.
  - Document RTL/LTR class and attribute ownership.
  - Add CLI template, registry entry, docs page, and demo usage if accepted as a public component.

- DONE M31.5 Complete low-risk gap documentation and quality gates
  - Update parity, component catalog, accessibility, and complex batch docs.
  - Run generated fixture smoke, workspace tests, and feature checks.
  - Mark M31 complete only after all new components are committed.

## M32 Form-specific Gap

- DONE M32.1 Implement Input OTP primitives or helpers
  - Add pure helpers for slot index, paste distribution, deletion behavior, and completion state if needed.
  - Add unit tests for boundaries, invalid characters, paste overflow, and disabled slots.
  - Keep actual value storage controlled by the consuming app.

- DONE M32.2 Implement Input OTP component
  - Add controlled root, group, slot, separator, and hidden/native input strategy where appropriate.
  - Add keyboard and accessibility notes for screen readers and mobile keyboards.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M32.3 Complete Input OTP documentation and quality gates
  - Update parity, component catalog, accessibility, and form docs.
  - Verify generated fixture smoke and feature checks include Input OTP.
  - Keep validation and submission app-owned.

## M33 Message and Attachment Components

- DONE M33.1 Implement Attachment
  - Add attachment root, preview, metadata, action, and remove/download slots.
  - Keep file upload, object URLs, drag-drop, and network state app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M33.2 Implement Bubble
  - Add message bubble composition parts with sender/receiver variants and density options.
  - Keep markdown, syntax highlighting, and rich content parsing app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M33.3 Implement Message
  - Add message root, avatar slot, header, content, footer/actions, and status parts.
  - Support user/assistant/system-style variants without coupling to a provider.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M33.4 Implement Marker
  - Add inline marker/highlight part for cited, selected, or annotated content.
  - Keep search indexing, citation resolution, and popover details app-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M33.5 Complete message component docs and examples
  - Update parity, component catalog, accessibility, and complex batch docs.
  - Add web/desktop examples for static message states.
  - Run generated fixture smoke, workspace tests, and feature checks.

## M34 Message Scroller and Runtime Follow-through

- DONE M34.1 Plan Message Scroller runtime boundaries
  - Define scroll-to-bottom, sticky-at-bottom, unread marker, and streaming update behavior.
  - Decide what can be pure state and what needs measurement/scroll runtime adapters.
  - Keep virtualization and async stream ownership app-owned.

- DONE M34.2 Implement Message Scroller state helpers
  - Add pure helpers for bottom threshold, unread marker visibility, and scroll intent.
  - Add unit tests for streaming append, user-scrolled-away, and reset behavior.
  - Avoid DOM measurement inside primitive state.

- DONE M34.3 Implement Message Scroller component
  - Add controlled scroller root, viewport, content, bottom anchor, unread marker, and jump button parts.
  - Wire only visible status attributes; leave actual scroll commands app/runtime-owned.
  - Add CLI template, registry entry, docs page, and demo usage.

- DONE M34.4 Add browser assertions for Web runtime verification
  - Add an expensive browser-level command for runtime Web fixture checks.
  - Cover focus, portal, timers, live-region, measurement, pointer, gesture, and message scroller prerequisites.
  - Keep the check separate from default release gates until stable.

- DONE M34.5 Complete message scroller docs and quality gates
  - Update parity, accessibility, runtime docs, and quality gate docs.
  - Run workspace tests, feature checks, generated fixture smoke, and runtime fixture commands.
  - Document any deferred Desktop/Mobile scroller behavior.

## M35 Chart Public Component Preparation

- DONE M35.1 Plan first-party SVG Chart API
  - Define ChartRoot, ChartSvg, ChartTitle, ChartDescription, ChartLegend, ChartFallbackTable, and ChartTooltipSlot APIs.
  - Limit first public chart types to line, bar, and area.
  - Keep Plotters and Charming adapters out of default generated source.

- DONE M35.2 Build chart example fixture before public component
  - Add example-only SVG chart rendering using existing chart primitives.
  - Verify measurement, responsive sizing, fallback table, and reduced-motion behavior.
  - Avoid adding `registry/chart.json` until the fixture is validated.

- DONE M35.3 Implement public Chart component if gates pass
  - Add crate-mode and source-copy Chart composition parts.
  - Add registry entry, template, docs page, and demo usage.
  - Clearly document backend ownership and fallback-table requirements.

- DONE M35.4 Complete chart public component quality gates
  - Run workspace tests, feature checks, generated fixture smoke, and runtime verification commands.
  - Update parity, accessibility, chart strategy, chart recipes, and component catalog docs.
  - Keep external backend adapters deferred unless explicitly approved.

## M36 Release Hardening for Expanded Parity

- DONE M36.1 Update registry/docs/template consistency checks
  - Ensure new public components have registry entries, templates, docs pages, and catalog links.
  - Extend generated fixture smoke coverage to every new component.
  - Keep `utils` and docs-only chart recipes excluded where appropriate.

- DONE M36.2 Update examples and screenshots strategy
  - Add representative web and desktop demo states for new composition, form, message, and chart components.
  - Document any visual verification gaps before claiming parity.
  - Avoid adding a marketing landing page instead of usable component previews.

- DONE M36.3 Final parity audit against shadcn current catalog
  - Recheck upstream shadcn/ui docs after M31-M35.
  - Mark implemented, deferred, and app-owned components with explicit reasons.
  - Create the next TODO plan if upstream adds new public components.

## M37 Rendered Preview and Screenshot Verification

- DONE M37.1 Plan rendered preview architecture
  - Define the Web and Desktop preview surfaces without removing command-line smoke examples.
  - Decide how preview state inventory, Tailwind CSS v4 input, and screenshot gates should fit together.
  - Document non-goals such as landing pages, committed full Tailwind output, and app-owned runtime features.

- DONE M37.2 Add shared preview state inventory
  - Add reusable representative state data for composition, form, message, chart, and runtime-sensitive components.
  - Keep command-line smoke output, future Web previews, and future Desktop previews aligned through the same inventory.
  - Avoid network data, upload transport, markdown parsing, provider integration, and external chart adapters.

- DONE M37.3 Build rendered Web preview shell
  - Add a usable Dioxus Web preview surface for representative component states.
  - Preserve existing command-line smoke commands or split them into explicit smoke binaries.
  - Use Tailwind CSS v4 source input syntax and avoid committing full generated Tailwind output.

- DONE M37.4 Add Web screenshot verification gate
  - Add desktop-width and mobile-width screenshot checks for the rendered Web preview.
  - Cover chart, message, form, and at least one overlay/open-state panel.
  - Keep the gate separate from default checks until runtime browser automation is stable.

- DONE M37.5 Plan Desktop WebView preview follow-through
  - Define the smallest Desktop WebView smoke path after the Web preview stabilizes.
  - Document remaining Mobile verification gaps and any platform-specific constraints.
  - Update quality gates and release docs with the new preview and screenshot commands.

## M38 Desktop WebView Preview

- DONE M38.1 Plan Desktop preview implementation
  - Define the explicit Desktop preview binary path without replacing command-line smoke output.
  - Specify required `data-preview-*` selectors and representative panels.
  - Keep Desktop screenshots and Mobile support out of scope until repeatable tooling exists.

- DONE M38.2 Extract shared rendered preview panels
  - Move reusable Web preview panel rendering into a shared example crate or helper module.
  - Keep Web and Desktop preview selectors aligned through the same rendering path.
  - Preserve Tailwind CSS v4 source scanning and command-line smoke output.

- DONE M38.3 Add Desktop preview binary
  - Add `dioxus-ui-desktop-demo --bin preview` using the shared rendered panels.
  - Enable only the Desktop runtime features required by the preview binary.
  - Ensure `cargo run -p dioxus-ui-desktop-demo` remains the command-line smoke path.

- DONE M38.4 Add Desktop preview structural gate
  - Add `scripts/desktop-preview-verify.mjs` for source selectors, binary compile, and smoke output.
  - Update examples, quality gates, and release docs with the explicit Desktop preview command.
  - Do not add Desktop screenshots to default release gates yet.

- DONE M38.5 Complete Desktop preview milestone
  - Run workspace checks, Web preview gate, Desktop preview gate, and example smoke.
  - Document remaining WebView screenshot and Mobile automation gaps.
  - Mark M38 complete only after all changes are committed.

## M39 Desktop WebView Screenshot Feasibility

- DONE M39.1 Plan Desktop WebView screenshot capture
  - Define why Desktop WebView screenshot capture differs from Web Playwright screenshots.
  - Specify candidate local commands, cleanup expectations, and unsupported behavior.
  - Keep Desktop screenshots out of default release gates until repeatable.

- DONE M39.2 Probe local Desktop screenshot tooling
  - Check whether local macOS window capture tooling can find and capture the Desktop preview window.
  - Document required permissions, platform constraints, and failure modes.
  - Avoid adding a flaky release gate if the window cannot be selected reliably.

- DONE M39.3 Add local-only Desktop screenshot smoke script if feasible
  - Add `scripts/desktop-webview-screenshot-smoke.sh` only if local window capture is repeatable.
  - Ensure the script starts and cleans up the Desktop preview process.
  - Keep generated screenshots ignored by Git.

- DONE M39.4 Complete Desktop screenshot feasibility milestone
  - Run Desktop structural gate, Web preview gate, example smoke, and any feasible screenshot smoke.
  - Update preview docs, quality gates, and release notes with the outcome.
  - Mark unsupported behavior explicitly if local screenshot capture is not reliable.

## M40 Mobile Web Profile Verification

- DONE M40.1 Plan Mobile Web profile verification path
  - Define the difference between Web mobile viewport checks and native Dioxus Mobile support.
  - Select the first repeatable checks for touch target metadata, hover alternatives, safe-area placeholders, reduced motion, and viewport sizing.
  - Keep emulator/device automation and native Mobile claims out of scope.

- DONE M40.2 Add Mobile Web profile structural gate
  - Add a script that validates the Web preview exposes mobile-profile selectors or source markers.
  - Reuse the rendered Web preview and shared preview inventory instead of creating a separate component tree.
  - Keep the gate local and deterministic without requiring a native device.

- DONE M40.3 Update Mobile verification docs and quality gates
  - Link the Mobile Web profile gate from runtime Mobile checklist, renderer verification, quality gates, and release docs.
  - Document which checklist items remain manual, emulator-backed later, or unsupported.
  - Avoid promoting Mobile runtime adapters or native Mobile support.

- DONE M40.4 Complete Mobile Web profile milestone
  - Run Web preview gate, Mobile Web profile gate, example smoke, and workspace checks as appropriate.
  - Update TODO status only after commits and validation.
  - Leave a clear next milestone seed for emulator/device follow-through if needed.

## M41 Mobile Browser Smoke Feasibility

- DONE M41.1 Plan Mobile browser smoke path
  - Define a Playwright-style mobile browser profile attached to the rendered Web preview.
  - Decide what can be verified without adding npm dependencies to the repository.
  - Keep native iOS/Android simulator, software keyboard, and assistive technology out of scope.

- DONE M41.2 Probe local browser automation availability
  - Check whether the local configured browser automation can open the Web preview at a mobile viewport.
  - Verify the mobile-profile panel, form, message, chart, and overlay-open selectors.
  - Document any tooling, dependency, or server lifecycle constraints.

- DONE M41.3 Add repeatable smoke command if feasible
  - Add a repository script only if it can run deterministically without unplanned dependency churn.
  - Ensure the command starts and cleans up the Web preview server or clearly documents the external server requirement.
  - Keep screenshots or browser artifacts ignored by Git.

- DONE M41.4 Complete Mobile browser smoke feasibility milestone
  - Run Mobile Web profile gate, Web preview gate, example smoke, and any feasible browser smoke.
  - Update docs and release gates with the supported claim and remaining native Mobile gaps.
  - Leave a clear next milestone seed for emulator/device follow-through if browser smoke is not enough.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
