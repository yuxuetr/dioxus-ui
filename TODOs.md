# TODOs

## Progress

- Overall: 75%
- Current milestone: M148 Hover Card Hover And Focus Opening
- Current task: M148.4

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

## M42 Browser Automation Dependency Strategy

- DONE M42.1 Plan browser automation dependency strategy
  - Decide whether this repository should add Node package metadata for browser smoke tests.
  - Compare Playwright-managed browsers, external Chrome contracts, and local/manual MCP-only verification.
  - Keep default Rust and source-copy gates independent from browser downloads.

- DONE M42.2 Add package metadata if accepted
  - Add `package.json` only if it keeps scripts explicit and avoids changing Rust build behavior.
  - Define browser smoke scripts as opt-in commands, not default release gates.
  - Avoid committing generated browser binaries or large artifacts.

- DONE M42.3 Add dependency documentation and install policy
  - Document how contributors install Node dependencies and browsers.
  - Explain which commands require network access and which remain offline.
  - Keep unsupported Mobile native behavior explicit.

- DONE M42.4 Complete browser automation dependency milestone
  - Run existing Rust, Web preview, Mobile Web profile, and example gates.
  - Run npm/package checks only if package metadata is added and dependencies are available.
  - Update TODO status after commits and leave the next browser smoke implementation seed.

## M43 Playwright Dependency Preparation

- DONE M43.1 Plan Playwright dependency and lockfile policy
  - Decide whether to use npm with `package-lock.json` for browser automation dependencies.
  - Define the Playwright package boundary and browser install command.
  - Keep browser-rendered smoke opt-in and outside default release gates.

- DONE M43.2 Add Playwright package dependency if feasible
  - Add a reviewed dev dependency and lockfile only if installation succeeds cleanly.
  - Avoid browser downloads during package install.
  - Keep generated browser binaries and screenshots ignored by Git.

- DONE M43.3 Document Playwright install and offline behavior
  - Document `npm install` and browser install commands.
  - Explain which commands work without browser binaries and which require downloads.
  - Keep native Mobile support explicitly out of scope.

- DONE M43.4 Complete Playwright dependency milestone
  - Run npm verification aliases, workspace tests, and any package lock checks.
  - Do not add browser-rendered smoke unless browser binaries are installed and stable.
  - Leave a clear next seed for the first opt-in browser smoke script.

## M44 Mobile Browser Smoke Script

- DONE M44.1 Plan opt-in mobile browser smoke script
  - Define the script contract for server startup, mobile viewport assertions, screenshots, and cleanup.
  - Decide how the script fails when Playwright Chromium is not installed.
  - Keep the command outside default release gates.

- DONE M44.2 Implement opt-in mobile browser smoke script
  - Add `scripts/mobile-browser-smoke.mjs` and an npm alias.
  - Start `dx serve`, wait for the Web preview URL, run Playwright mobile assertions, and stop the server.
  - Keep screenshots optional and ignored by Git.

- DONE M44.3 Document browser smoke usage and install requirements
  - Document `npm run verify:mobile-browser` and `npx playwright install chromium`.
  - Explain that the command verifies mobile browser rendering, not native Mobile behavior.
  - Keep release gates unchanged until the command is stable.

- DONE M44.4 Complete mobile browser smoke milestone
  - Run deterministic gates and run the browser smoke if Chromium is available.
  - If Chromium is missing, verify the command fails with actionable install guidance.
  - Update TODO status after commits and leave next seed for CI or screenshot follow-through.

## M45 Mobile Browser Smoke End-to-end

- DONE M45.1 Plan Chromium install and end-to-end browser smoke
  - Define the local opt-in Chromium install path and cache/artifact expectations.
  - Keep browser downloads out of default release gates.
  - Document expected success and failure modes before installing.

- DONE M45.2 Install Playwright Chromium if feasible
  - Run `npx playwright install chromium` only as an opt-in local setup step.
  - Verify downloaded browser artifacts remain outside Git.
  - Document network or platform failures without committing generated binaries.

- DONE M45.3 Run mobile browser smoke end to end
  - Run `npm run verify:mobile-browser` with Chromium available.
  - Verify server startup, selector assertions, nonblank rendering, chart bounding box, and cleanup.
  - Keep screenshots optional and ignored.

- DONE M45.4 Complete mobile browser smoke end-to-end milestone
  - Run deterministic gates and the end-to-end browser smoke if installation succeeded.
  - Update docs with the supported local claim and remaining non-release status.
  - Leave next seed for optional screenshots or CI integration.

## M46 External Chrome Browser Smoke Path

- DONE M46.1 Plan external Chrome executable support
  - Define an opt-in environment variable for using a local Chrome/Chromium executable.
  - Document how this differs from Playwright-managed Chromium.
  - Keep browser smoke outside default release gates.

- DONE M46.2 Add external browser executable support
  - Update `scripts/mobile-browser-smoke.mjs` to use the external executable when configured.
  - Preserve the existing missing Playwright Chromium install guidance when no external path is set.
  - Fail clearly when the configured executable path is invalid.

- DONE M46.3 Probe external Chrome path locally
  - Check whether a local Chrome executable exists and can run the smoke command.
  - Document platform launch failures without claiming browser smoke success.
  - Verify server cleanup and ignored artifacts.

- DONE M46.4 Complete external Chrome browser smoke milestone
  - Run deterministic gates and any feasible external-browser smoke.
  - Update docs with the supported local command and remaining limitations.
  - Leave next seed for CI or screenshot follow-through.

## M47 Mobile Browser Screenshot Artifacts

- DONE M47.1 Plan opt-in screenshot artifact contract
  - Define when the mobile browser smoke should save screenshots.
  - Specify artifact naming, ignored paths, and failure behavior.
  - Keep screenshots outside default release gates and committed source artifacts.

- DONE M47.2 Add opt-in screenshot capture support
  - Update `scripts/mobile-browser-smoke.mjs` to save a mobile screenshot only when configured.
  - Preserve existing assertion-only behavior by default.
  - Print the saved screenshot path when capture succeeds.

- DONE M47.3 Document screenshot usage and cleanup expectations
  - Update README, quality gates, release docs, and browser smoke docs with the opt-in command.
  - Explain that screenshots verify rendered Web mobile viewport output, not native Mobile.
  - Keep generated screenshot files ignored by Git.

- DONE M47.4 Complete mobile browser screenshot artifact milestone
  - Run deterministic gates and the external Chrome screenshot smoke if feasible.
  - Verify screenshot artifact creation and cleanup behavior.
  - Update TODO status only after commits and validation.

## M48 Mobile Screenshot Metadata Checks

- DONE M48.1 Plan screenshot metadata validation contract
  - Define the minimum metadata checks for generated mobile browser screenshots.
  - Decide whether checks should run only when screenshot capture is enabled.
  - Document failure behavior, dimensions, and file size expectations.

- DONE M48.2 Add screenshot metadata validation
  - Parse the generated PNG dimensions without adding a new dependency.
  - Fail the screenshot-enabled smoke when the artifact is empty or below the mobile viewport size.
  - Print screenshot dimensions and byte size after validation succeeds.

- DONE M48.3 Document screenshot metadata checks
  - Update browser smoke docs, quality gates, release docs, and README with the validation claim.
  - Keep the command opt-in and outside default release gates.
  - Avoid claiming visual diffing or native Mobile verification.

- DONE M48.4 Complete mobile screenshot metadata milestone
  - Run deterministic gates and the external Chrome screenshot metadata smoke if feasible.
  - Verify server cleanup and ignored artifact behavior.
  - Update TODO status only after commits and validation.

## M49 CI Browser Smoke Documentation

- DONE M49.1 Plan CI browser smoke setup documentation
  - Define CI prerequisites for Node dependencies, Playwright Chromium, and Dioxus Web preview serving.
  - Decide whether browser smoke should be blocking, optional, scheduled, or manual.
  - Document artifact upload and cache expectations without adding a workflow yet.

- DONE M49.2 Add CI browser smoke setup guide
  - Create a CI-focused guide with install commands, verification commands, and failure modes.
  - Include separate paths for Playwright-managed Chromium and external Chrome executables.
  - Keep native Mobile and pixel visual regression claims out of scope.

- DONE M49.3 Link CI browser smoke guide from release and quality docs
  - Update README, quality gates, release docs, and component docs index.
  - Explain that CI browser smoke remains opt-in until the repository has a reviewed workflow.
  - Keep default local gates unchanged.

- DONE M49.4 Complete CI browser smoke documentation milestone
  - Run deterministic gates and documentation checks.
  - Verify no workflow or browser artifacts are accidentally committed.
  - Update TODO status only after commits and validation.

## M50 CI Browser Workflow Template

- DONE M50.1 Plan non-blocking CI browser workflow template
  - Define the workflow shape without adding `.github/workflows`.
  - Decide which browser strategy the template should prefer first.
  - Document permissions, triggers, cache, and artifact behavior.

- DONE M50.2 Add reviewed workflow template document
  - Add a copyable non-blocking workflow template under docs.
  - Include Playwright-managed Chromium and external Chrome notes.
  - Keep the template manual or scheduled by default, not a required merge gate.

- DONE M50.3 Link workflow template from CI browser smoke docs
  - Update README, CI browser guide, quality gates, and release docs as needed.
  - Clarify that the template is documentation until copied into `.github/workflows`.
  - Keep default local gates unchanged.

- DONE M50.4 Complete CI browser workflow template milestone
  - Run deterministic gates and documentation checks.
  - Verify no `.github/workflows` file is committed.
  - Update TODO status only after commits and validation.

## M51 CI Browser Workflow Activation RFC

- DONE M51.1 Plan workflow activation RFC scope
  - Define the decision points for turning the documented template into an active workflow.
  - Cover required-run policy, artifact retention, cache keys, runner image choice, and browser strategy.
  - Keep actual workflow activation out of scope.

- DONE M51.2 Add workflow activation RFC
  - Add a new RFC under `docs/rfcs`.
  - Document the recommended phased rollout from manual non-blocking workflow to possible required gate.
  - Include rollback criteria and failure ownership.

- DONE M51.3 Link workflow activation RFC
  - Update CI browser docs, workflow template docs, release docs, and README/RFC index references.
  - Clarify that the RFC is a decision document, not an active workflow.
  - Keep default local and release gates unchanged.

- DONE M51.4 Complete workflow activation RFC milestone
  - Run deterministic gates and documentation checks.
  - Verify no `.github/workflows` file is committed.
  - Update TODO status only after commits and validation.

## M52 Product Surface Audit

- DONE M52.1 Plan product surface audit scope
  - Define local audit dimensions for registry entries, templates, crate modules, docs pages, and feature exports.
  - Keep upstream shadcn parity refresh out of scope unless explicitly scheduled as a separate milestone.
  - Decide what evidence should drive the next implementation milestone.

- DONE M52.2 Run local catalog consistency audit
  - Compare registry entries, template files, crate modules, docs pages, and public feature/export surfaces.
  - Identify missing, extra, or intentionally docs-only/source-only items.
  - Record exact counts and drift categories.

- DONE M52.3 Document next product-surface recommendation
  - Add an audit document with findings and recommended next milestone.
  - Classify next work as component parity, theme tokens, docs-site rendering, or release hardening.
  - Avoid implementing new components in this audit milestone.

- DONE M52.4 Complete product surface audit milestone
  - Run deterministic gates and documentation checks.
  - Verify no unintended generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M53 Docs Site Catalog Data Contract

- DONE M53.1 Plan docs-site catalog data contract
  - Define catalog fields derived from registry, docs pages, crate features, and template files.
  - Decide which fields are required, derived, or intentionally deferred to a visual docs runtime.
  - Keep component API and visual docs-site implementation out of scope.

- DONE M53.2 Add catalog metadata verification script
  - Add a deterministic script that builds an in-memory catalog from local sources.
  - Verify every public component has registry, docs, template, crate feature, module, and source file coverage.
  - Print summary counts and fail on drift without writing generated output.

- DONE M53.3 Document catalog consumption path
  - Update docs-site plan and product surface audit docs with the catalog contract.
  - Add npm verification alias if appropriate.
  - Explain how a future Dioxus docs site should consume the catalog without duplicating metadata.

- DONE M53.4 Complete docs-site catalog data contract milestone
  - Run deterministic gates, workspace tests, and catalog verification.
  - Verify no generated catalog artifact is committed.
  - Update TODO status only after commits and validation.

## M54 Shared Docs Catalog Builder

- DONE M54.1 Plan shared docs catalog builder extraction
  - Define which catalog-building functions should move out of the verification script.
  - Keep generated catalog artifacts and visual docs runtime out of scope.
  - Preserve the existing `verify:docs-catalog` command behavior.

- DONE M54.2 Extract reusable catalog builder module
  - Move source-reading and catalog construction into a shared script module.
  - Update `scripts/docs-catalog-verify.mjs` to consume the shared builder.
  - Preserve validation output and failure behavior.

- DONE M54.3 Document shared catalog builder usage
  - Update docs-site plan with the shared builder boundary.
  - Explain how future docs runtime code can reuse it without duplicating metadata.
  - Keep generated artifacts deferred.

- DONE M54.4 Complete shared docs catalog builder milestone
  - Run deterministic gates, workspace tests, and catalog verification.
  - Verify no generated catalog artifact is committed.
  - Update TODO status only after commits and validation.

## M55 Static Docs Catalog View

- DONE M55.1 Plan static docs catalog view
  - Define a Markdown catalog view generated from the shared builder.
  - Decide which fields should appear in the static page and which remain deferred to a visual docs runtime.
  - Keep route rendering, screenshots, and preview images out of scope.

- DONE M55.2 Add catalog Markdown renderer and static page
  - Add a script that renders a deterministic component catalog Markdown page from `buildDocsCatalog`.
  - Add the generated static catalog page under docs without duplicating source parsing logic.
  - Preserve the existing `verify:docs-catalog` metadata check.

- DONE M55.3 Add catalog Markdown drift verification and docs links
  - Add a verification command that fails when the static catalog page drifts from builder output.
  - Link the static catalog page from docs index and quality gate docs.
  - Document how future docs runtime work should reuse the same data path.

- DONE M55.4 Complete static docs catalog view milestone
  - Run deterministic gates, workspace tests, and catalog verification.
  - Verify the static catalog page is the only committed catalog artifact.
  - Update TODO status only after commits and validation.

## M56 Static Catalog Grouping

- DONE M56.1 Plan static catalog grouping
  - Define a small category taxonomy for the static catalog page.
  - Decide whether grouping metadata should live in registry entries or renderer-owned data.
  - Keep visual docs navigation and component API changes out of scope.

- DONE M56.2 Add catalog grouping metadata
  - Add deterministic grouping data for every public component.
  - Extend the shared catalog builder output with category fields.
  - Fail if any public component is missing grouping metadata.

- DONE M56.3 Render grouped catalog Markdown
  - Update the static catalog page renderer to include grouped sections.
  - Preserve the full flat component table for scanning.
  - Keep the catalog page drift verification passing.

- DONE M56.4 Document grouping usage and complete milestone
  - Update docs-site planning and quality gate docs with grouping ownership.
  - Run deterministic gates, workspace tests, and catalog verification.
  - Update TODO status only after commits and validation.

## M57 Docs Route Manifest

- DONE M57.1 Plan docs route manifest
  - Define route metadata needed by a future Dioxus docs runtime.
  - Decide which routes can be represented statically before rendered navigation exists.
  - Keep visual route rendering and screenshot verification out of scope.

- DONE M57.2 Add route metadata to catalog builder
  - Extend catalog records with deterministic docs route and anchor fields.
  - Verify route uniqueness and stable category anchors.
  - Preserve existing catalog metadata and static page verification behavior.

- DONE M57.3 Add static route manifest page and drift gate
  - Render a deterministic Markdown route manifest from the shared builder.
  - Add a verification command that fails when the route manifest drifts.
  - Link the route manifest from docs index and quality gates.

- DONE M57.4 Complete docs route manifest milestone
  - Update docs-site planning with final route manifest ownership.
  - Run deterministic gates, workspace tests, catalog verification, and route manifest verification.
  - Update TODO status only after commits and validation.

## M58 Source Preview Manifest

- DONE M58.1 Plan source preview manifest
  - Define source preview metadata for future `/components/{slug}/source` routes.
  - Decide which template fields should be represented statically without embedding full source.
  - Keep rendered source previews, syntax highlighting, and browser route assertions out of scope.

- DONE M58.2 Add source preview metadata to catalog builder
  - Extend catalog records with deterministic source preview fields.
  - Verify template files are readable and source preview metadata is complete.
  - Preserve existing catalog, grouped catalog, and route manifest checks.

- DONE M58.3 Add static source preview manifest and drift gate
  - Render a deterministic Markdown source preview manifest from the shared builder.
  - Add a verification command that fails when the source preview manifest drifts.
  - Link the source preview manifest from docs index, route manifest, and quality gates.

- DONE M58.4 Complete source preview manifest milestone
  - Update docs-site planning with final source preview ownership.
  - Run deterministic gates, workspace tests, catalog verification, route verification, and source manifest verification.
  - Update TODO status only after commits and validation.

## M59 Docs Metadata Gate Aggregator

- DONE M59.1 Plan docs metadata aggregate gate
  - Define a single local command for all docs metadata and Markdown drift checks.
  - Decide which existing docs checks belong in the aggregate gate.
  - Keep browser, screenshot, and rendered runtime checks out of scope.

- DONE M59.2 Add aggregate docs verification command
  - Add an npm alias that runs catalog, catalog page, route manifest, and source preview checks.
  - Preserve existing individual commands for focused debugging.
  - Verify the aggregate command fails through normal npm command chaining.

- DONE M59.3 Document aggregate docs verification
  - Update README, quality gates, and docs-site plan with the aggregate command.
  - Keep release and browser smoke gates separate.
  - Explain when to use individual commands versus the aggregate command.

- DONE M59.4 Complete docs metadata gate aggregator milestone
  - Run deterministic gates, workspace tests, docs aggregate verification, and diff checks.
  - Verify no generated JSON artifacts are committed.
  - Update TODO status only after commits and validation.

## M60 Local Verification Aggregator

- DONE M60.1 Plan local verification aggregate gate
  - Define a single local npm command for default deterministic verification.
  - Decide whether the command should include smoke and docs metadata checks only.
  - Keep full Rust workspace tests, browser automation, screenshots, and release gates out of scope.

- DONE M60.2 Add local aggregate verification command
  - Add an npm alias that runs `verify:smoke` and `verify:docs`.
  - Preserve existing focused commands for targeted debugging.
  - Verify the aggregate command succeeds through normal npm command chaining.

- DONE M60.3 Document local aggregate verification
  - Update README, quality gates, and docs-site plan with the local aggregate command.
  - Explain which checks remain separate and why.
  - Keep CI/browser workflow docs unchanged unless required.

- DONE M60.4 Complete local verification aggregator milestone
  - Run deterministic gates, workspace tests, local aggregate verification, and diff checks.
  - Verify no generated JSON artifacts are committed.
  - Update TODO status only after commits and validation.

## M61 Release Verification Alignment

- DONE M61.1 Plan release verification alignment
  - Define how `npm run verify`, Rust workspace tests, source-copy gates, feature checks, and opt-in browser smoke relate.
  - Keep browser install, CI workflow activation, and new release automation out of scope.
  - Document which release commands are aggregate aliases versus required explicit gates.

- DONE M61.2 Add release documentation consistency check
  - Add a deterministic script that verifies release docs mention the local aggregate gate and release-only gates.
  - Add an npm alias for the release docs consistency check.
  - Keep the check read-only and avoid generating release artifacts.

- DONE M61.3 Update release verification documentation
  - Update release, quality gate, and docs-site planning docs with the release verification alignment.
  - Clarify that `npm run verify` is a local deterministic alias and not a replacement for Rust workspace or release-only gates.
  - Preserve focused commands for debugging and opt-in browser smoke.

- DONE M61.4 Complete release verification alignment milestone
  - Run deterministic gates, workspace tests, release docs consistency checks, and diff checks.
  - Verify no generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M62 Release Gate Aggregator

- DONE M62.1 Plan release gate aggregation
  - Define a single explicit release verification alias for the existing required local release gates.
  - Decide command ordering so cheap deterministic checks fail before expensive feature and fixture checks where practical.
  - Keep browser installation, screenshots, CI workflow activation, and native Mobile/Desktop runtime automation out of scope.

- DONE M62.2 Add release aggregate verification command
  - Add an npm alias that runs the required release gate commands without replacing focused aliases.
  - Preserve opt-in browser smoke as a separate command.
  - Verify the aggregate command succeeds through normal npm command chaining.

- DONE M62.3 Update release aggregate documentation
  - Update README, release docs, quality gates, and docs-site planning docs with the release aggregate command.
  - Extend release docs consistency checks to require the aggregate alias.
  - Clarify that focused commands remain useful for isolating failures.

- DONE M62.4 Complete release gate aggregator milestone
  - Run the release aggregate command, release docs consistency checks, workspace tests as needed, and diff checks.
  - Verify no generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M63 Package Script Consistency Gate

- DONE M63.1 Plan package script consistency gate
  - Define which npm verification aliases are required for local, docs, release, and opt-in browser gates.
  - Decide which aggregate command relationships should be checked for drift.
  - Keep command execution, browser installation, and generated artifacts out of the consistency check.

- DONE M63.2 Add package script consistency check
  - Add a deterministic read-only script that validates required `package.json` verification aliases.
  - Verify aggregate aliases reference the expected focused commands.
  - Add an npm alias for the package script consistency check.

- DONE M63.3 Update package script gate documentation
  - Update README, quality gates, release docs, and docs-site planning docs with the new consistency check.
  - Include the package script check in the release aggregate command if it remains deterministic and read-only.
  - Clarify that the check validates command wiring but does not execute the release gate.

- DONE M63.4 Complete package script consistency milestone
  - Run release aggregate verification, package script consistency checks, release docs consistency checks, and diff checks.
  - Verify no generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M64 CI Browser Docs Alignment

- DONE M64.1 Plan CI browser docs alignment
  - Define how CI browser smoke docs should reference `npm run verify`, `npm run verify:release`, and opt-in browser smoke.
  - Keep workflow activation, required merge gates, browser installation changes, and native Mobile/Desktop claims out of scope.
  - Decide which CI browser documentation invariants should be checked by a read-only script.

- DONE M64.2 Add CI browser docs consistency check
  - Add a deterministic script that verifies CI browser smoke docs and workflow template mention the expected local gates and opt-in boundary.
  - Add an npm alias for the CI docs consistency check.
  - Keep the check read-only and avoid creating `.github/workflows`.

- DONE M64.3 Update CI browser docs and verification wiring
  - Update CI browser smoke docs and workflow template to use the current local verification aliases.
  - Document the CI docs consistency check in README, quality gates, release docs, and docs-site planning docs.
  - Include the CI docs check in package script consistency and release aggregate checks if it remains deterministic and read-only.

- DONE M64.4 Complete CI browser docs alignment milestone
  - Run release aggregate verification, CI docs consistency checks, package script consistency checks, release docs checks, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M65 CI Plan Documentation Gate

- DONE M65.1 Plan CI plan documentation gate
  - Define how the CI Plan section should reference default PR, release, scheduled, and opt-in browser verification.
  - Keep actual workflow files, required gate promotion, browser installation changes, and CI provider configuration out of scope.
  - Decide which CI Plan invariants should be covered by a read-only documentation check.

- DONE M65.2 Add CI plan consistency check
  - Add a deterministic script that verifies the CI Plan documents the current local and release verification aliases.
  - Add an npm alias for the CI Plan documentation check.
  - Keep the check read-only and avoid executing CI commands or creating workflow files.

- DONE M65.3 Update CI Plan documentation and wiring
  - Update quality gates, README, release docs, and docs-site planning docs with the CI Plan consistency check.
  - Include the CI Plan check in package script consistency and release aggregate checks if it remains deterministic and read-only.
  - Clarify that CI Plan checks validate documentation only, not active workflow configuration.

- DONE M65.4 Complete CI Plan documentation milestone
  - Run release aggregate verification, CI Plan docs checks, package script checks, CI browser docs checks, release docs checks, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M66 Documentation Index Consistency Gate

- DONE M66.1 Plan documentation index consistency gate
  - Define which top-level docs and verification docs must be discoverable from README and docs/README.
  - Keep generated docs site, navigation runtime, link crawling, and CI workflow creation out of scope.
  - Decide which index invariants should be checked by a read-only script.

- DONE M66.2 Add documentation index consistency check
  - Add a deterministic script that verifies required README and docs/README links.
  - Add an npm alias for the documentation index consistency check.
  - Keep the check read-only and avoid generated artifacts.

- DONE M66.3 Update documentation indexes and verification wiring
  - Update README and docs/README with missing quality, release, CI, site, RFC, and TODO links.
  - Include the docs index check in deterministic docs/package/release verification if appropriate.
  - Document the docs index check in quality gates, release docs, and docs-site planning docs.

- DONE M66.4 Complete documentation index consistency milestone
  - Run docs index checks, docs aggregate, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M67 Local Markdown Link Target Gate

- DONE M67.1 Plan local Markdown link target gate
  - Define which Markdown files and relative links should be checked for existing local targets.
  - Keep external URL validation, anchor validation, generated docs runtime checks, and link crawling out of scope.
  - Decide how the check should handle ignored build outputs and non-Markdown assets.

- DONE M67.2 Add local Markdown link target check
  - Add a deterministic read-only script that scans repository Markdown files for local relative links.
  - Fail when a relative file target does not exist in the repository.
  - Add an npm alias for the local Markdown link target check.

- DONE M67.3 Update link target verification wiring
  - Include the link target check in deterministic docs/package/release verification if appropriate.
  - Document the check in README, quality gates, release docs, and docs-site planning docs.
  - Keep external URLs and anchor correctness documented as future scope.

- DONE M67.4 Complete local Markdown link target milestone
  - Run link target checks, docs aggregate, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M68 Local Markdown Anchor Gate

- DONE M68.1 Plan local Markdown anchor gate
  - Define which local Markdown fragments should be validated against headings or explicit anchors.
  - Keep external URL anchors, generated docs runtime routes, rendered HTML checks, and network access out of scope.
  - Decide how the check should handle GitHub-style heading slugs and duplicate headings.

- DONE M68.2 Add local Markdown anchor check
  - Add a deterministic read-only script that scans tracked Markdown files for local fragment links.
  - Validate same-file and relative Markdown fragments against target headings or explicit anchors.
  - Add an npm alias for the local Markdown anchor check.

- DONE M68.3 Update anchor verification wiring
  - Include the anchor check in deterministic docs/package/release verification if appropriate.
  - Document the check in README, quality gates, release docs, and docs-site planning docs.
  - Keep external anchors and generated route anchors documented as future scope.

- DONE M68.4 Complete local Markdown anchor milestone
  - Run anchor checks, docs aggregate, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M69 Repository Hygiene Gate

- DONE M69.1 Plan repository hygiene gate
  - Define which generated artifacts and inactive workflow files must not be committed.
  - Keep cleanup, destructive deletion, dependency pruning, and formatting changes out of scope.
  - Decide which hygiene invariants should be checked by a read-only script.

- DONE M69.2 Add repository hygiene check
  - Add a deterministic read-only script that checks for forbidden tracked files and inactive workflow files.
  - Verify browser screenshot artifacts and the inactive browser smoke workflow are not committed.
  - Add an npm alias for the repository hygiene check.

- DONE M69.3 Update repository hygiene verification wiring
  - Include the hygiene check in deterministic package/release verification if appropriate.
  - Document the check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check reports hygiene drift but does not remove files.

- DONE M69.4 Complete repository hygiene milestone
  - Run hygiene checks, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M70 Component Status Snapshot

- DONE M70.1 Plan component status snapshot
  - Define a generated Markdown status page that summarizes implemented public components by category.
  - Reuse the docs catalog builder as the source of truth for registry, docs, template, source-copy, and crate feature coverage.
  - Keep upstream shadcn refresh, visual parity claims, and new component implementation out of scope.

- DONE M70.2 Add component status generator and check
  - Add a deterministic renderer for `docs/components/status.md`.
  - Add a verification script that fails when the status page drifts from local catalog metadata.
  - Add an npm alias for the component status check.

- DONE M70.3 Update component status documentation wiring
  - Link the status page from component docs indexes and user-facing docs.
  - Include the status check in deterministic docs/package/release verification if appropriate.
  - Document that the page reflects local implementation status, not live upstream shadcn changes.

- DONE M70.4 Complete component status milestone
  - Run component status checks, docs aggregate, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M71 Component Coverage Matrix

- DONE M71.1 Plan component coverage matrix
  - Define additional status columns for docs, template, generated target, crate feature, crate module, and source preview coverage.
  - Reuse catalog metadata and keep upstream parity refresh, new components, and visual preview claims out of scope.
  - Decide how the generated Markdown should summarize complete versus incomplete coverage.

- DONE M71.2 Add coverage matrix rendering
  - Extend the component status generator with per-component coverage columns.
  - Add summary counts for fully covered components and incomplete components.
  - Keep the existing `verify:docs-status` check deterministic and read-only.

- DONE M71.3 Update coverage matrix documentation
  - Document the expanded coverage matrix in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the matrix reflects local wiring completeness, not runtime visual parity.
  - Keep the status page linked from component docs indexes.

- DONE M71.4 Complete component coverage matrix milestone
  - Run component status checks, docs aggregate, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M72 Component Docs Structure Gate

- DONE M72.1 Plan component docs structure gate
  - Define the required Markdown structure for public component docs pages.
  - Reuse docs catalog metadata and keep content quality scoring, rendered docs checks, and live upstream shadcn parity out of scope.
  - Decide how the check should validate title, Source Copy, Crate Feature, API Surface, Accessibility Notes, and source preview references.

- DONE M72.2 Add component docs structure check
  - Add a deterministic read-only script that scans public component docs pages from the docs catalog.
  - Fail when a public component doc is missing required sections or mismatched generated command/feature snippets.
  - Add an npm alias for the component docs structure check.

- DONE M72.3 Update docs structure verification wiring
  - Include the docs structure check in deterministic docs/package/release verification if appropriate.
  - Document the check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check verifies documentation shape, not prose quality or runtime visual parity.

- DONE M72.4 Complete component docs structure milestone
  - Run docs structure checks, docs aggregate, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M73 Registry Metadata Gate

- DONE M73.1 Plan registry metadata gate
  - Define a deterministic read-only npm check for `registry/*.json` metadata.
  - Keep Rust compile checks, CLI behavior tests, generated fixture smoke, and JSON Schema validation out of scope.
  - Decide which registry invariants should be verified from source paths, targets, dependencies, and assets.

- DONE M73.2 Add registry metadata check
  - Add a script that validates registry entry names, descriptions, file mappings, dependency references, and asset mappings.
  - Verify source paths exist and source-copy targets remain inside `src/components/ui`.
  - Add an npm alias for the registry metadata check.

- DONE M73.3 Update registry metadata verification wiring
  - Include the registry metadata check in deterministic package/release verification if appropriate.
  - Document the check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check verifies registry metadata shape, not runtime CLI behavior.

- DONE M73.4 Complete registry metadata milestone
  - Run registry metadata checks, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M74 Tailwind Static Token Gate

- DONE M74.1 Plan Tailwind static token gate
  - Define a deterministic read-only check for dynamic Tailwind class token patterns in component source and templates.
  - Keep full Tailwind compilation, CSS generation, visual regression, and user-provided class validation out of scope.
  - Decide which source roots and class-prefix patterns should be scanned.

- DONE M74.2 Add Tailwind static token check
  - Add a script that scans styled crate source, source-copy templates, and class helpers for dynamic Tailwind token interpolation.
  - Fail on patterns like `bg-{...}`, `text-{...}`, `border-{...}`, spacing interpolation, and common variant color interpolation.
  - Add an npm alias for the Tailwind static token check.

- DONE M74.3 Update Tailwind static token verification wiring
  - Include the Tailwind static token check in deterministic package/release verification if appropriate.
  - Document the check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check verifies static token shape, not compiled CSS output or visual parity.

- DONE M74.4 Complete Tailwind static token milestone
  - Run Tailwind token checks, package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M75 Package Script Target Gate

- DONE M75.1 Plan package script target gate
  - Define a deterministic read-only check that every local script target referenced from `package.json` exists.
  - Keep executing npm scripts, shell parsing beyond simple command segments, and external command validation out of scope.
  - Decide how to handle `node scripts/*.mjs` and direct `scripts/*.sh` references in aggregate commands.

- DONE M75.2 Add package script target check
  - Extend the package script verifier to validate local `scripts/` targets referenced by npm aliases.
  - Fail when a `node scripts/*.mjs` or direct `scripts/*` command points to a missing file.
  - Keep the check read-only and deterministic.

- DONE M75.3 Update package script target documentation
  - Document the target existence check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates package script wiring, not command behavior.
  - Keep release aggregation behavior unchanged.

- DONE M75.4 Complete package script target milestone
  - Run package script checks, release docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M76 Quality Gate Alias Coverage Gate

- DONE M76.1 Plan quality gate alias coverage
  - Define a deterministic read-only check that every `package.json` `verify:*` alias is documented in `docs/quality-gates.md`.
  - Keep command execution, prose quality scoring, and release aggregation behavior out of scope.
  - Decide how to handle opt-in browser aliases and aggregate aliases.

- DONE M76.2 Add quality gate alias coverage check
  - Extend or add a verifier that compares `package.json` verification aliases against `docs/quality-gates.md`.
  - Fail when a `verify:*` alias is missing from the quality gate docs.
  - Include the check in the existing docs or release verification flow without adding browser runtime requirements.

- DONE M76.3 Update quality gate alias documentation
  - Document focused preview and example aliases that are currently only described indirectly.
  - Clarify that opt-in browser smoke remains documented but outside default release gates.
  - Update README, release, or docs-site planning docs only where needed.

- DONE M76.4 Complete quality gate alias coverage milestone
  - Run quality gate alias checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M77 Release Aggregate Docs Coverage Gate

- DONE M77.1 Plan release aggregate docs coverage
  - Define a deterministic read-only check that `docs/release.md` covers every command segment in `package.json` `verify:release`.
  - Keep command execution, shell interpretation beyond simple `&&` segments, and release aggregate rewrites out of scope.
  - Decide whether release docs should list aggregate aliases directly instead of expanding their nested commands.

- DONE M77.2 Add release aggregate docs coverage check
  - Extend the release documentation verifier to parse `verify:release` command segments from `package.json`.
  - Fail when any release aggregate command segment is missing from `docs/release.md`.
  - Keep existing opt-in browser smoke boundary checks.

- DONE M77.3 Update release aggregate documentation
  - Align the expanded release gate list with the actual `verify:release` command chain.
  - Clarify which entries are direct aggregate segments and which are covered by nested aliases.
  - Update README, quality gates, or docs-site planning docs only where needed.

- DONE M77.4 Complete release aggregate docs coverage milestone
  - Run release docs checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M78 Release Gate Order Verification

- DONE M78.1 Plan release gate order verification
  - Define a deterministic read-only check that the `docs/release.md` expanded gate block exactly matches `package.json` `verify:release` order.
  - Keep command execution, nested alias expansion, and arbitrary shell parsing out of scope.
  - Decide how to identify the intended release gate code block without relying on brittle prose offsets.

- DONE M78.2 Add release gate order check
  - Extend the release documentation verifier to parse the expanded release gate code block.
  - Fail when commands are missing, extra, or out of order compared with the `verify:release` command chain.
  - Preserve existing release snippet and opt-in browser boundary checks.

- DONE M78.3 Update release gate order documentation
  - Document that `verify:release-docs` checks direct segment order, not nested alias behavior.
  - Clarify the expanded release gate block is generated manually but verified against `package.json`.
  - Update README, quality gates, or docs-site planning docs only where needed.

- DONE M78.4 Complete release gate order milestone
  - Run release docs checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M79 Package Lock Metadata Gate

- DONE M79.1 Plan package lock metadata gate
  - Define a deterministic read-only check that `package-lock.json` root metadata matches `package.json`.
  - Keep dependency resolution, network installs, lockfile regeneration, and package manager migration out of scope.
  - Decide which root fields are required for the current npm lockfile contract.

- DONE M79.2 Add package lock metadata check
  - Add or extend a verifier to compare package name, version, and root devDependencies between `package.json` and `package-lock.json`.
  - Fail when `package-lock.json` is missing, has an unsupported lockfile version, or drifts from package metadata.
  - Include the check in release verification without requiring browser binaries or network access.

- DONE M79.3 Update package lock metadata documentation
  - Document the package lock metadata check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates committed lockfile metadata, not dependency freshness or registry availability.
  - Keep package script and release aggregate behavior unchanged except for the new read-only gate.

- DONE M79.4 Complete package lock metadata milestone
  - Run package lock checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M80 Generated Directory Hygiene Gate

- DONE M80.1 Plan generated directory hygiene gate
  - Define a deterministic read-only check that generated dependency and build directories are never tracked.
  - Keep deleting local directories, cleaning workspaces, and validating global git ignore configuration out of scope.
  - Decide which repository-local generated directories must be explicitly ignored and forbidden when tracked.

- DONE M80.2 Add generated directory hygiene check
  - Extend repository hygiene verification to fail on tracked `node_modules/`, `target/`, and other generated output directories.
  - Add missing project `.gitignore` entries for local Rust and Node generated directories.
  - Keep the check read-only and part of the existing release hygiene gate.

- DONE M80.3 Update generated directory hygiene documentation
  - Document the generated directory hygiene rule in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check reports tracked artifacts but does not delete local build outputs.
  - Keep browser screenshot and workflow hygiene behavior unchanged.

- DONE M80.4 Complete generated directory hygiene milestone
  - Run repo hygiene checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M81 Cargo Workspace Metadata Gate

- DONE M81.1 Plan cargo workspace metadata gate
  - Define a deterministic read-only check that workspace package metadata remains consistent across Rust crates.
  - Keep dependency freshness, publishing, cargo package output, and registry availability out of scope.
  - Decide which workspace package fields and member manifests should be validated.

- DONE M81.2 Add cargo workspace metadata check
  - Add a verifier that compares workspace package metadata with member crate manifests.
  - Fail when crate manifests stop inheriting required workspace metadata or when Cargo metadata disagrees with the root workspace contract.
  - Include the check in release verification without adding network or browser requirements.

- DONE M81.3 Update cargo workspace metadata documentation
  - Document the workspace metadata check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates committed manifest metadata, not crate publishing or dependency freshness.
  - Keep package script and release aggregate behavior aligned with the new read-only gate.

- DONE M81.4 Complete cargo workspace metadata milestone
  - Run workspace metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M82 Quality Gate Release Block Sync

- DONE M82.1 Plan quality gate release block sync
  - Define a deterministic read-only check that the release gate block in `docs/quality-gates.md` matches `package.json` `verify:release`.
  - Keep command execution, nested alias expansion, and release documentation checks out of scope.
  - Decide how to identify and compare the intended quality gate release block.

- DONE M82.2 Add quality gate release block check
  - Extend or add a verifier that parses the quality gate release block and compares it with direct `verify:release` command segments.
  - Fail when commands are missing, extra, or out of order.
  - Include the check in the docs verification flow without adding browser, network, or full release requirements.

- DONE M82.3 Update quality gate release documentation
  - Align `docs/quality-gates.md` release block with the actual release aggregate command chain.
  - Document that the quality gate check validates direct release segments, not nested alias expansion.
  - Update README, release docs, or docs-site planning docs only where needed.

- DONE M82.4 Complete quality gate release block milestone
  - Run quality gate docs checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M83 Cargo Lock Metadata Gate

- DONE M83.1 Plan cargo lock metadata gate
  - Define a deterministic read-only check that `Cargo.lock` workspace package entries match Cargo metadata.
  - Keep dependency freshness, lockfile regeneration, dependency resolution, and registry availability out of scope.
  - Decide which workspace package names, versions, and lockfile fields should be validated.

- DONE M83.2 Add cargo lock metadata check
  - Add a verifier that compares committed `Cargo.lock` package entries with `cargo metadata --no-deps` workspace members.
  - Fail when the lockfile is missing, has an unsupported format, misses a workspace package, or records a mismatched workspace package version.
  - Include the check in release verification without adding network or browser requirements.

- DONE M83.3 Update cargo lock metadata documentation
  - Document the cargo lock metadata check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates committed lockfile metadata, not dependency freshness or lockfile regeneration.
  - Keep package script and release aggregate behavior aligned with the new read-only gate.

- DONE M83.4 Complete cargo lock metadata milestone
  - Run cargo lock checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M84 Pre-commit Config Metadata Gate

- DONE M84.1 Plan pre-commit config metadata gate
  - Define a deterministic read-only check that `.pre-commit-config.yaml` local hook metadata remains valid.
  - Keep hook execution, dependency installation, remote repo freshness, and network checks out of scope.
  - Decide which local hook ids, entries, and supporting config files should be validated.

- DONE M84.2 Add pre-commit config metadata check
  - Add a verifier that checks required local hook ids, command snippets, and referenced config files.
  - Fail when `.pre-commit-config.yaml` is missing required local hooks or when local hook command targets drift.
  - Include the check in release verification without adding network, browser, or full pre-commit requirements.

- DONE M84.3 Update pre-commit config metadata documentation
  - Document the pre-commit metadata check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates committed hook metadata, not hook execution or remote hook freshness.
  - Keep package script and release aggregate behavior aligned with the new read-only gate.

- DONE M84.4 Complete pre-commit config metadata milestone
  - Run pre-commit metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M85 Script Metadata Gate

- DONE M85.1 Plan script metadata gate
  - Define a deterministic read-only check that repository script metadata remains executable where required.
  - Keep script execution, shell linting, dependency installation, and generated output checks out of scope.
  - Decide which `scripts/*.sh` and `scripts/*.mjs` files should have shebangs and executable bits.

- DONE M85.2 Add script metadata check
  - Add a verifier that checks shell script shebangs, executable bits, and package-referenced script targets.
  - Fail when direct shell script targets are not executable or when runnable Node scripts lack the expected shebang.
  - Include the check in release verification without executing the scripts being inspected.

- DONE M85.3 Update script metadata documentation
  - Document the script metadata check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates committed file metadata, not script behavior or shell syntax.
  - Keep package script and release aggregate behavior aligned with the new read-only gate.

- DONE M85.4 Complete script metadata milestone
  - Run script metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M86 README Verification Summary Gate

- DONE M86.1 Plan README verification summary gate
  - Define a deterministic read-only check that README verification shortcut prose stays aligned with package scripts and quality gates.
  - Keep command execution, external link crawling, generated docs rendering, and prose rewriting out of scope.
  - Decide which default, docs, smoke, release, and focused verification aliases must remain discoverable from README.

- DONE M86.2 Add README verification summary check
  - Add a verifier that checks README verification shortcut blocks and required command snippets against package metadata.
  - Fail when README omits required focused aliases or stale release/default gate descriptions.
  - Include the check in docs verification without running the referenced commands.

- DONE M86.3 Update README verification documentation
  - Document the README verification summary gate in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates command discoverability and summary coverage, not command behavior.
  - Keep package script, docs aggregate, and release aggregate behavior aligned with the new read-only gate.

- DONE M86.4 Complete README verification milestone
  - Run README verification checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M87 Examples Metadata Gate

- DONE M87.1 Plan examples metadata gate
  - Define a deterministic read-only check that example workspace members, package aliases, smoke scripts, and examples documentation stay aligned.
  - Keep example execution, rendered previews, browser automation, and generated fixture compilation out of scope.
  - Decide which Web, Desktop, preview-state, runtime Web, and runtime Desktop example entry points must remain documented.

- DONE M87.2 Add examples metadata check
  - Add a verifier that checks example Cargo manifests, workspace membership, examples README commands, and package script wiring.
  - Fail when an example crate is missing from the workspace or examples README omits required run and verification commands.
  - Include the check in release verification without running the examples being inspected.

- DONE M87.3 Update examples metadata documentation
  - Document the examples metadata check in README, examples README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates example metadata and documentation wiring, not runtime behavior or visual output.
  - Keep package script and release aggregate behavior aligned with the new read-only gate.

- DONE M87.4 Complete examples metadata milestone
  - Run examples metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M88 RFC Metadata Gate

- DONE M88.1 Plan RFC metadata gate
  - Define a deterministic read-only check that RFC numbering, filenames, titles, and README indexes stay aligned.
  - Keep RFC content review, external link crawling, generated docs rendering, and prose rewriting out of scope.
  - Decide which root README, docs README, and release/quality references must keep RFC entry points discoverable.

- DONE M88.2 Add RFC metadata check
  - Add a verifier that checks RFC filename numbering, H1 title numbering, contiguous sequence, and README index coverage.
  - Fail when an RFC file is missing from docs/README, missing from the root README RFC section, or has mismatched numbering/title metadata.
  - Include the check in docs verification without interpreting RFC decisions.

- DONE M88.3 Update RFC metadata documentation
  - Document the RFC metadata check in README, docs README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates RFC discoverability and metadata shape, not RFC acceptance or implementation status.
  - Keep package script, docs aggregate, and release aggregate behavior aligned with the new read-only gate.

- DONE M88.4 Complete RFC metadata milestone
  - Run RFC metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M89 CSS Input Metadata Gate

- DONE M89.1 Plan CSS input metadata gate
  - Define a deterministic read-only check that Tailwind CSS v4 input files and CLI default CSS stay aligned.
  - Keep Tailwind compilation, rendered preview checks, class token scanning, and generated output inspection out of scope.
  - Decide which preview CSS files, CLI generated CSS content, and documentation snippets must keep Tailwind v4 syntax.

- DONE M89.2 Add CSS input metadata check
  - Add a verifier that checks `@import "tailwindcss";`, preview `@source` paths, absence of Tailwind v3 directives, and CLI default CSS.
  - Fail when preview CSS inputs omit required source roots or when generated CSS defaults drift from documented Tailwind v4 input syntax.
  - Include the check in release verification without compiling Tailwind or launching previews.

- DONE M89.3 Update CSS input metadata documentation
  - Document the CSS input metadata check in README, examples README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates source input metadata, not final CSS output or visual styling.
  - Keep package script and release aggregate behavior aligned with the new read-only gate.

- DONE M89.4 Complete CSS input metadata milestone
  - Run CSS input metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M90 Gitignore Metadata Gate

- DONE M90.1 Plan gitignore metadata gate
  - Define a deterministic read-only check that `.gitignore`, repository hygiene policy, and documented artifact patterns stay aligned.
  - Keep deleting local artifacts, checking global git excludes, and validating untracked file contents out of scope.
  - Decide which dependency directories, build outputs, browser profiles, screenshots, and generated fixture patterns must remain ignored.

- DONE M90.2 Add gitignore metadata check
  - Add a verifier that checks `.gitignore` contains required local artifact patterns and that repository hygiene rejects matching tracked files.
  - Fail when `.gitignore` omits required generated directories, screenshot patterns, or browser automation cache directories.
  - Include the check in release verification without removing local files.

- DONE M90.3 Update gitignore metadata documentation
  - Document the gitignore metadata check in README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates ignore policy metadata, not local cleanup or artifact contents.
  - Keep package script, repository hygiene, and release aggregate behavior aligned with the new read-only gate.

- DONE M90.4 Complete gitignore metadata milestone
  - Run gitignore metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M91 Preview State Inventory Metadata Gate

- DONE M91.1 Plan preview state inventory metadata gate
  - Define a deterministic read-only check that shared preview state inventory, Web/Desktop preview gates, package aliases, and documentation stay aligned.
  - Keep browser-rendered screenshot assertions, Tailwind compilation, `dx serve`, Desktop WebView automation, and visual parity scoring out of scope.
  - Decide which representative state labels, preview panels, preview targets, and CSS source inputs must remain discoverable.

- DONE M91.2 Add preview state inventory metadata check
  - Add a verifier that checks `examples/preview-states`, Web/Desktop preview binaries, structural preview verifiers, and package script wiring.
  - Fail when required preview state labels, `data-preview-panel` markers, preview targets, or Tailwind v4 source inputs drift.
  - Include the check in release verification without launching browsers or compiling Tailwind output.

- DONE M91.3 Update preview state metadata documentation
  - Document the preview state metadata check in README, examples README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates preview metadata wiring, not rendered visual correctness or screenshot pixels.
  - Keep preview, smoke, package script, and release aggregate behavior aligned with the new read-only gate.

- DONE M91.4 Complete preview state inventory metadata milestone
  - Run preview state metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M92 Mobile Browser Smoke Metadata Gate

- DONE M92.1 Plan mobile browser smoke metadata gate
  - Define a deterministic read-only check that the opt-in mobile browser smoke script, docs, package alias, and CI browser guidance stay aligned.
  - Keep launching Playwright, starting `dx serve`, installing browsers, writing screenshots, and validating screenshot pixels out of scope.
  - Decide which localhost settings, viewport dimensions, selector assertions, screenshot artifact pattern, and environment variables must remain discoverable.

- DONE M92.2 Add mobile browser smoke metadata check
  - Add a verifier that checks `scripts/mobile-browser-smoke.mjs`, `package.json`, README, release docs, quality gates, CI browser docs, and workflow template references.
  - Fail when required selectors, `DIOXUS_UI_BROWSER_EXECUTABLE`, `DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT`, `npx playwright install chromium`, or screenshot artifact patterns drift.
  - Include the check in release verification without launching browser automation or writing artifacts.

- DONE M92.3 Update mobile browser smoke metadata documentation
  - Document the mobile browser smoke metadata check in README, quality gates, release docs, CI browser docs, and docs-site planning docs.
  - Clarify that the check validates opt-in browser smoke wiring, not browser availability, rendered output, or screenshot content.
  - Keep package script, release aggregate, and browser opt-in behavior aligned with the new read-only gate.

- DONE M92.4 Complete mobile browser smoke metadata milestone
  - Run mobile browser smoke metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no workflow files, browser profiles, or screenshots are committed.
  - Update TODO status only after commits and validation.

## M93 CI Browser Workflow Template Metadata Gate

- DONE M93.1 Plan CI browser workflow template metadata gate
  - Define a deterministic read-only check that the documented CI browser workflow template, CI browser guide, RFC 0009 rollout policy, package alias, and active workflow absence stay aligned.
  - Keep creating `.github/workflows/`, running GitHub Actions, installing browsers, uploading artifacts, and changing workflow activation policy out of scope.
  - Decide which trigger, non-blocking policy, permission, timeout, artifact, browser strategy, and required-gate boundaries must remain discoverable.

- DONE M93.2 Add CI browser workflow template metadata check
  - Add a verifier that checks `docs/ci-browser-workflow-template.md`, `docs/ci-browser-smoke.md`, RFC 0009, package script wiring, and absence of `.github/workflows/browser-smoke.yml`.
  - Fail when `workflow_dispatch`, `continue-on-error: true`, `contents: read`, screenshot artifact upload, Playwright install, external Chrome fallback, or required-gate boundary docs drift.
  - Include the check in release verification without activating or running CI browser automation.

- DONE M93.3 Update CI workflow template metadata documentation
  - Document the CI workflow template metadata check in README, docs README, quality gates, release docs, CI browser docs, and docs-site planning docs.
  - Clarify that the check validates committed workflow documentation only, not CI execution or browser availability.
  - Keep package script, release aggregate, CI docs, CI plan, and repository hygiene behavior aligned with the new read-only gate.

- DONE M93.4 Complete CI browser workflow template metadata milestone
  - Run CI workflow template metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no active workflow files, browser profiles, or screenshots are committed.
  - Update TODO status only after commits and validation.

## M94 Browser Artifact Policy Metadata Gate

- DONE M94.1 Plan browser artifact policy metadata gate
  - Define a deterministic read-only check that CI browser artifact policy, workflow template upload behavior, RFC 0009, `.gitignore`, and repository hygiene stay aligned.
  - Keep running browser smoke, uploading artifacts, changing retention defaults, deleting local files, and validating screenshot pixels out of scope.
  - Decide which screenshot-only upload pattern, forbidden browser profile/cache artifacts, temporary preview outputs, and retention guidance must remain discoverable.

- DONE M94.2 Add browser artifact policy metadata check
  - Add a verifier that checks artifact policy documentation, workflow template upload fields, `.gitignore` screenshot patterns, and repository hygiene boundaries.
  - Fail when CI docs allow browser profiles, Playwright caches, target directories, temporary preview output, or non-PNG browser artifacts into the normal upload path.
  - Include the check in release verification without launching browser automation or touching local artifacts.

- DONE M94.3 Update browser artifact policy metadata documentation
  - Document the artifact policy metadata check in README, docs README, quality gates, release docs, CI browser docs, and docs-site planning docs.
  - Clarify that the check validates committed artifact policy metadata, not artifact upload execution or retention enforcement.
  - Keep package script, release aggregate, gitignore metadata, CI workflow template metadata, and repository hygiene behavior aligned with the new read-only gate.

- DONE M94.4 Complete browser artifact policy metadata milestone
  - Run browser artifact policy checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no active workflow files, browser profiles, screenshots, caches, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M95 Release Warning Inventory Metadata Gate

- DONE M95.1 Plan release warning inventory metadata gate
  - Define a deterministic read-only check that known release warnings, Cargo lock evidence, release docs, quality gates, and docs-site results stay aligned.
  - Keep dependency upgrades, Cargo warning parsing, `cargo report` execution, warning suppression, and network lookups out of scope.
  - Decide which current `block v0.1.6` future-incompatibility warning details must remain discoverable until resolved.

- DONE M95.2 Add release warning inventory metadata check
  - Add a verifier that checks `Cargo.lock`, package script wiring, release warning inventory docs, release docs, quality gates, and docs-site notes.
  - Fail when the known warning inventory omits the package name, version, warning type, observation command, or non-goal boundaries.
  - Include the check in release verification without running Cargo or changing dependency state.

- DONE M95.3 Update release warning inventory documentation
  - Document the release warning inventory metadata check in README, docs README, quality gates, release docs, and docs-site planning docs.
  - Clarify that the check validates warning inventory metadata, not whether Cargo currently emits the warning.
  - Keep package script, release aggregate, Cargo lock metadata, and release documentation behavior aligned with the new read-only gate.

- DONE M95.4 Complete release warning inventory metadata milestone
  - Run release warning inventory checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no dependency upgrades, lockfile rewrites, warning suppressions, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M96 Cargo Publish Metadata Gate

- DONE M96.1 Plan Cargo publish metadata gate
  - Define a deterministic read-only check that publishable crate manifests, shared workspace package metadata, release docs, and quality gates stay aligned.
  - Keep `cargo publish`, `cargo package`, crates.io lookups, dependency freshness checks, repository URL replacement, and changelog generation out of scope.
  - Decide which crate descriptions, shared README/keywords/categories metadata, and example `publish = false` boundaries must remain discoverable.

- DONE M96.2 Add Cargo publish metadata check
  - Add crate-specific descriptions and shared publish metadata fields for README, keywords, and categories.
  - Add a verifier that checks publishable crate metadata, workspace inheritance, example non-publishable boundaries, package script wiring, and release wiring.
  - Include the check in release verification without packaging or publishing crates.

- DONE M96.3 Update Cargo publish metadata documentation
  - Document the Cargo publish metadata check in README, docs README, workspace docs, release docs, quality gates, and docs-site planning docs.
  - Clarify that the check validates committed manifest metadata, not publish readiness or crates.io availability.
  - Keep package script, release aggregate, Cargo workspace metadata, and release documentation behavior aligned with the new read-only gate.

- DONE M96.4 Complete Cargo publish metadata milestone
  - Run Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no package archives, dependency updates, crates.io lookups, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M97 Publish Readiness Blocker Metadata Gate

- DONE M97.1 Plan publish readiness blocker metadata gate
  - Define a deterministic read-only check that known publish blockers, release docs, Cargo publish metadata docs, workspace docs, and docs-site notes stay aligned.
  - Keep replacing repository URLs, checking crates.io availability, running `cargo package`, running `cargo publish`, changelog generation, and API stabilization out of scope.
  - Decide which repository placeholder, pre-1.0 API, changelog, CLI template packaging, and crates.io review blockers must remain discoverable.

- DONE M97.2 Add publish readiness blocker metadata check
  - Add a verifier that checks the blocker inventory, placeholder repository evidence, package script wiring, release wiring, and publish metadata boundary docs.
  - Fail when docs imply metadata gates are enough to publish or omit required blocker categories.
  - Include the check in release verification without modifying manifests or contacting registries.

- DONE M97.3 Update publish readiness blocker documentation
  - Document the blocker inventory check in README, docs README, workspace docs, release docs, quality gates, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check preserves known blockers and does not resolve them.
  - Keep package script, release aggregate, Cargo publish metadata, and release documentation behavior aligned with the new read-only gate.

- DONE M97.4 Complete publish readiness blocker metadata milestone
  - Run publish readiness blocker checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no repository URL replacement, package archives, registry lookups, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M98 Changelog Metadata Gate

- DONE M98.1 Plan changelog metadata gate
  - Define a deterministic read-only check that `CHANGELOG.md`, release docs, publish blocker docs, quality gates, and docs-site notes stay aligned.
  - Keep generating release notes, running git-cliff, rewriting commit history, deriving changes from Git, and publishing releases out of scope.
  - Decide which project-owned changelog heading, Unreleased section, Keep a Changelog style, conventional commit reference, and stale template-link bans must remain discoverable.

- DONE M98.2 Add changelog metadata check
  - Replace stale template changelog content with a project-owned changelog scaffold.
  - Add a verifier that checks changelog structure, absence of `yuxuetr/rust-template` links, package script wiring, release wiring, and publish blocker alignment.
  - Include the check in release verification without generating release notes or contacting external services.

- DONE M98.3 Update changelog metadata documentation
  - Document the changelog metadata check in README, docs README, release docs, quality gates, publish blocker docs, and docs-site planning docs.
  - Clarify that the check validates changelog ownership and structure, not completeness of generated release notes.
  - Keep package script, release aggregate, publish readiness blockers, and release documentation behavior aligned with the new read-only gate.

- DONE M98.4 Complete changelog metadata milestone
  - Run changelog metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no generated release notes, git history rewrites, tags, package archives, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M99 Release Notes Readiness Metadata Gate

- DONE M99.1 Plan release notes readiness metadata gate
  - Define the distinction between project-owned changelog structure and publish-ready release notes.
  - Keep release note generation, git-cliff, Git history derivation, tags, publishing, and release content decisions out of scope.
  - Decide which publish blocker, release docs, quality gate, changelog metadata, and docs-site fragments must stay aligned.

- DONE M99.2 Add release notes readiness metadata check
  - Update publish blocker wording from changelog ownership to release notes readiness.
  - Add or extend a verifier that rejects stale "changelog not project-owned" wording and requires release notes readiness wording.
  - Keep the check read-only and include it in release verification without generating notes or contacting external services.

- DONE M99.3 Update release notes readiness documentation
  - Document the release notes readiness distinction in README, docs README, release docs, quality gates, publish blocker docs, changelog metadata docs, and docs-site planning docs.
  - Clarify that M98 validates changelog structure while M99 preserves the unresolved release notes readiness blocker.
  - Keep package script, release aggregate, publish readiness blockers, changelog metadata, and release documentation behavior aligned.

- DONE M99.4 Complete release notes readiness metadata milestone
  - Run release notes readiness checks, changelog checks, publish blocker checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no generated release notes, git history rewrites, tags, package archives, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M100 License Readiness Metadata Gate

- DONE M100.1 Plan license readiness metadata gate
  - Define the distinction between Cargo license metadata and committed root license text files.
  - Keep license term changes, generated legal text, copyright decisions, packaging, publishing, and registry checks out of scope.
  - Decide which workspace metadata, publish blockers, release docs, quality gates, and docs-site fragments must stay aligned.

- DONE M100.2 Add license readiness metadata check
  - Add a verifier that checks `MIT OR Apache-2.0` workspace metadata, inherited crate license metadata, missing root license file blocker wording, package script wiring, and release wiring.
  - Fail if docs imply license file readiness is resolved before `LICENSE-MIT` and `LICENSE-APACHE` are committed.
  - Keep the check read-only and include it in release verification without generating license files or contacting external services.

- DONE M100.3 Update license readiness documentation
  - Document the license readiness check in README, docs README, release docs, quality gates, publish blocker docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check preserves the missing root license file blocker and does not choose license terms.
  - Keep package script, release aggregate, publish readiness blockers, Cargo publish metadata, and release documentation behavior aligned.

- DONE M100.4 Complete license readiness metadata milestone
  - Run license readiness checks, publish blocker checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no generated license text, package archives, registry lookups, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M101 Repository Identity Readiness Metadata Gate

- DONE M101.1 Plan repository identity readiness metadata gate
  - Define the distinction between existing Cargo repository metadata and final publish-ready repository identity.
  - Keep repository URL replacement, remote repository checks, crates.io checks, packaging, and publishing out of scope.
  - Decide which workspace metadata, publish blockers, Cargo publish metadata, release docs, quality gates, and docs-site fragments must stay aligned.

- DONE M101.2 Add repository identity readiness metadata check
  - Add a verifier that checks the placeholder repository URL, publish blocker wording, workspace docs, Cargo publish metadata docs, package script wiring, and release wiring.
  - Fail if docs imply repository identity readiness is resolved while `https://github.com/your-org/dioxus-ui` remains in workspace metadata.
  - Keep the check read-only and include it in release verification without replacing URLs or contacting external services.

- DONE M101.3 Update repository identity readiness documentation
  - Document the repository identity readiness check in README, docs README, workspace docs, release docs, quality gates, publish blocker docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check preserves the placeholder repository blocker and does not choose the final repository URL.
  - Keep package script, release aggregate, publish readiness blockers, Cargo publish metadata, and release documentation behavior aligned.

- DONE M101.4 Complete repository identity readiness metadata milestone
  - Run repository identity readiness checks, publish blocker checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no repository URL replacement, package archives, registry lookups, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M102 API Stability Readiness Metadata Gate

- DONE M102.1 Plan API stability readiness metadata gate
  - Define the distinction between pre-`1.0` crate metadata and publish-ready API stability.
  - Keep API freezing, version changes, semantic versioning decisions, migration guide generation, packaging, and publishing out of scope.
  - Decide which workspace metadata, publish blockers, Cargo publish metadata, release docs, quality gates, and docs-site fragments must stay aligned.

- DONE M102.2 Add API stability readiness metadata check
  - Add a verifier that checks workspace version metadata, pre-`1.0` blocker wording, release docs, Cargo publish metadata docs, package script wiring, and release wiring.
  - Fail if docs imply API stability readiness is resolved while workspace version remains `0.1.0`.
  - Keep the check read-only and include it in release verification without changing versions or stabilizing APIs.

- DONE M102.3 Update API stability readiness documentation
  - Document the API stability readiness check in README, docs README, release docs, quality gates, publish blocker docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check preserves the pre-`1.0` stability blocker and does not freeze component APIs.
  - Keep package script, release aggregate, publish readiness blockers, Cargo publish metadata, and release documentation behavior aligned.

- DONE M102.4 Complete API stability readiness metadata milestone
  - Run API stability readiness checks, publish blocker checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no version changes, API freeze claims, package archives, registry lookups, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M103 CLI Template Packaging Readiness Metadata Gate

- DONE M103.1 Plan CLI template packaging readiness metadata gate
  - Define the distinction between source-tree template loading and publish-ready CLI template packaging.
  - Keep embedding templates, changing CLI runtime path lookup, packaging templates, running `cargo package`, running `cargo publish`, and installing the CLI out of scope.
  - Decide which CLI source, publish blockers, release docs, quality gates, Cargo publish metadata, and docs-site fragments must stay aligned.

- DONE M103.2 Add CLI template packaging readiness metadata check
  - Add a verifier that checks CLI source still reads registry/templates from repository layout, publish blocker wording, release docs, package script wiring, and release wiring.
  - Fail if docs imply CLI template packaging readiness is resolved before templates are embedded or packaged in a stable install location.
  - Keep the check read-only and include it in release verification without changing CLI packaging behavior.

- DONE M103.3 Update CLI template packaging readiness documentation
  - Document the CLI template packaging readiness check in README, docs README, release docs, quality gates, publish blocker docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check preserves the unresolved CLI template packaging blocker and does not package or embed templates.
  - Keep package script, release aggregate, publish readiness blockers, Cargo publish metadata, and release documentation behavior aligned.

- DONE M103.4 Complete CLI template packaging readiness metadata milestone
  - Run CLI template packaging readiness checks, publish blocker checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no CLI template embedding, package archives, install-location changes, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M104 Registry Availability Readiness Metadata Gate

- DONE M104.1 Plan registry availability readiness metadata gate
  - Define the distinction between local publish metadata completeness and external crates.io name/ownership availability.
  - Keep crates.io lookups, registry ownership checks, token checks, package archives, `cargo package`, and `cargo publish` out of scope.
  - Decide which workspace metadata, publish blockers, Cargo publish metadata, release docs, quality gates, and docs-site fragments must stay aligned.

- DONE M104.2 Add registry availability readiness metadata check
  - Add a verifier that checks planned publishable crate names, registry availability blocker wording, release docs, Cargo publish metadata docs, package script wiring, and release wiring.
  - Fail if docs imply crates.io registry availability has been checked while the blocker remains unresolved.
  - Keep the check read-only and include it in release verification without contacting crates.io or requiring credentials.

- DONE M104.3 Update registry availability readiness documentation
  - Document the registry availability readiness check in README, docs README, release docs, quality gates, publish blocker docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check preserves the unresolved crates.io name/ownership review blocker and does not query registries.
  - Keep package script, release aggregate, publish readiness blockers, Cargo publish metadata, and release documentation behavior aligned.

- DONE M104.4 Complete registry availability readiness metadata milestone
  - Run registry availability readiness checks, publish blocker checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no crates.io lookups, credentials, package archives, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M105 Publish Readiness Coverage Metadata Gate

- DONE M105.1 Plan publish readiness coverage metadata gate
  - Define the coverage contract between publish readiness blockers and focused readiness metadata gates.
  - Keep blocker resolution, repository URL changes, API stabilization, release note generation, license text generation, template packaging, registry lookups, package archives, and publishing out of scope.
  - Decide which blocker docs, focused readiness docs, package scripts, release docs, quality gates, and docs-site fragments must stay aligned.

- DONE M105.2 Add publish readiness coverage metadata check
  - Add a verifier that checks every current publish blocker has a corresponding focused readiness gate, docs page, README mention, package script, and release wiring.
  - Fail if blocker names, focused gate names, or release aggregate wiring drift out of sync.
  - Keep the check read-only and include it in release verification without resolving blockers or contacting external services.

- DONE M105.3 Update publish readiness coverage documentation
  - Document the coverage check in README, docs README, release docs, quality gates, publish blocker docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check validates coverage only and does not make crates publish-ready.
  - Keep package script, release aggregate, publish readiness blockers, Cargo publish metadata, and release documentation behavior aligned.

- DONE M105.4 Complete publish readiness coverage metadata milestone
  - Run publish readiness coverage checks, all focused readiness checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no blocker resolutions, credentials, package archives, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M106 Publish Readiness Resolution Runbook Metadata Gate

- DONE M106.1 Plan publish readiness resolution runbook metadata gate
  - Define the manual resolution runbook for each current publish readiness blocker.
  - Keep repository URL replacement, API stabilization, release note generation, license text generation, CLI template packaging, registry lookups, package archives, and publishing out of scope.
  - Decide which blocker docs, coverage docs, release docs, quality gates, Cargo publish metadata, README, and docs-site fragments must stay aligned.

- DONE M106.2 Add publish readiness resolution runbook metadata check
  - Add a verifier that checks the runbook covers each blocker, owner evidence, required follow-up updates, package script wiring, and release wiring.
  - Fail if docs imply the runbook resolves blockers or authorizes publish commands.
  - Keep the check read-only and include it in release verification without contacting external services or changing release state.

- DONE M106.3 Update publish readiness resolution runbook documentation
  - Document the runbook check in README, docs README, release docs, quality gates, publish blocker docs, coverage docs, Cargo publish metadata docs, and docs-site planning docs.
  - Clarify that the check validates manual resolution guidance only and does not make crates publish-ready.
  - Keep package script, release aggregate, publish readiness blockers, coverage metadata, Cargo publish metadata, and release documentation behavior aligned.

- DONE M106.4 Complete publish readiness resolution runbook metadata milestone
  - Run runbook checks, coverage checks, all focused readiness checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no blocker resolutions, credentials, package archives, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M107 Publish Order Metadata Gate

- DONE M107.1 Plan publish order metadata gate
  - Define the planned crate publish order and why dependency crates must be published first.
  - Keep package archives, `cargo package`, `cargo publish`, registry ownership checks, token checks, and dependency version changes out of scope.
  - Decide which release docs, Cargo publish metadata, registry availability metadata, runbook docs, README, quality gates, and docs-site fragments must stay aligned.

- DONE M107.2 Add publish order metadata check
  - Add a verifier that checks planned publishable crates and documented publish order across release docs, Cargo publish metadata, registry availability metadata, runbook docs, package script wiring, and release wiring.
  - Fail if the documented publish order drifts or if docs imply publish commands are authorized.
  - Keep the check read-only and include it in release verification without creating packages or contacting crates.io.

- DONE M107.3 Update publish order documentation
  - Document the publish order check in README, docs README, release docs, quality gates, Cargo publish metadata docs, registry availability docs, runbook docs, and docs-site planning docs.
  - Clarify that the check validates planned order only and does not publish crates.
  - Keep package script, release aggregate, Cargo publish metadata, registry availability, runbook, and release documentation behavior aligned.

- DONE M107.4 Complete publish order metadata milestone
  - Run publish order checks, runbook checks, registry availability checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no package archives, registry lookups, credentials, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M108 Workspace Dependency Publish Readiness Metadata Gate

- DONE M108.1 Plan workspace dependency publish readiness metadata gate
  - Define the unresolved publish-readiness risk for path-only internal workspace dependencies between publishable crates.
  - Keep dependency version changes, `cargo package`, `cargo publish`, crates.io lookups, ownership checks, credentials, package archives, and release authorization out of scope.
  - Decide which blocker docs, coverage docs, runbook docs, Cargo publish metadata, publish order metadata, README, quality gates, release docs, package scripts, and docs-site fragments must stay aligned.

- DONE M108.2 Add workspace dependency publish readiness metadata check
  - Add a verifier that checks internal publishable crate workspace dependencies, blocker inventory, coverage mapping, runbook evidence, package script wiring, and release wiring.
  - Fail if path-only internal crate dependencies remain but the readiness blocker is missing or docs imply dependency versions are publish-ready.
  - Keep the check read-only and include it in release verification without changing dependency versions, creating packages, or contacting crates.io.

- DONE M108.3 Update workspace dependency publish readiness documentation
  - Document the workspace dependency readiness check in README, docs README, release docs, quality gates, Cargo publish metadata docs, publish order docs, blocker docs, coverage docs, runbook docs, and docs-site planning docs.
  - Clarify that the check validates unresolved dependency publish readiness only and does not make crates publish-ready.
  - Keep package script, release aggregate, Cargo publish metadata, publish order, blocker, coverage, runbook, and release documentation behavior aligned.

- DONE M108.4 Complete workspace dependency publish readiness metadata milestone
  - Run workspace dependency readiness checks, coverage checks, runbook checks, publish order checks, Cargo publish metadata checks, package script checks, docs checks, release aggregate, and diff checks.
  - Verify no dependency version changes, package archives, registry lookups, credentials, publish commands, or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## M109 Rendered Component Verification

- DONE M109.1 Plan rendered component coverage verification
  - Define the rendered coverage contract for all public shadcn/ui-aligned components.
  - Keep browser launch, screenshot artifacts, visual diffing, runtime adapter stabilization, component API changes, and template edits out of scope.
  - Decide which preview-state docs, README, quality gates, release docs, docs-site notes, package scripts, and catalog metadata must stay aligned.

- DONE M109.2 Add rendered component coverage metadata check
  - Add a deterministic verifier that compares a rendered coverage manifest against the docs catalog public component list.
  - Fail when a public component lacks a stable rendered preview target, category, panel, coverage level, or runtime/app-owned note.
  - Keep the check read-only and include it in release verification without starting servers, launching browsers, writing screenshots, or claiming visual parity.

- DONE M109.3 Add preview coverage targets
  - Add rendered coverage records and stable `data-component-preview` target ids for all public components.
  - Reuse the shared preview-state crate and preserve existing Web/Desktop preview panels and state markers.
  - Keep representative previews lightweight and avoid adding full prop matrices or runtime adapter behavior.

- DONE M109.4 Update rendered coverage documentation
  - Document the rendered coverage check in README, docs README, release docs, quality gates, rendered verification docs, and docs-site planning docs.
  - Clarify that the check validates deterministic rendered target coverage, not visual parity or browser screenshot pixels.
  - Keep package script, release aggregate, preview-state metadata, docs catalog, and release documentation behavior aligned.

- DONE M109.5 Complete rendered component verification milestone
  - Run rendered coverage checks, preview metadata checks, Web/Desktop preview checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no browser artifacts, screenshots, generated docs, component API changes, or template rewrites are committed.
  - Update TODO status only after commits and validation.

## M110 Browser DOM Component Verification

- DONE M110.1 Plan browser DOM component verification
  - Define the Playwright-backed DOM verification contract for all public rendered component targets.
  - Keep screenshot artifacts, visual diffing, runtime interaction assertions, component API changes, template rewrites, and Desktop native WebView checks out of scope.
  - Decide which package scripts, README, quality gates, release docs, docs-site notes, browser artifact policy, and rendered verification docs must stay aligned.

- DONE M110.2 Add rendered component DOM verifier
  - Add an opt-in browser verifier that starts the Web preview, visits the local preview route, and checks every `data-component-preview` target from the rendered coverage manifest.
  - Fail when a target is missing, detached, hidden, empty, or has an invalid bounding box.
  - Keep the command deterministic, clean up the server process, avoid screenshots by default, and do not claim visual parity.

- DONE M110.3 Wire and document DOM verification
  - Add package script wiring for the focused DOM verification command without making it part of default release gates unless explicitly scoped.
  - Document setup, browser executable behavior, non-goals, and troubleshooting in README, quality gates, release docs, docs-site notes, and rendered verification docs.
  - Keep browser artifact policy and repository hygiene aligned so generated screenshots or traces remain ignored if later enabled.

- DONE M110.4 Complete browser DOM verification milestone
  - Run rendered coverage checks, DOM verification, Web preview checks, docs checks, package script checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, or template rewrites are committed.
  - Update TODO status only after commits and validation.

## M111 Runtime Interaction Verification

- DONE M111.1 Plan runtime interaction verification
  - Define the first browser-backed interaction contract for runtime-sensitive components.
  - Keep visual diffing, screenshot artifacts, full accessibility certification, native Desktop WebView automation, native Mobile automation, component API changes, and template rewrites out of scope.
  - Decide which preview fixtures, package scripts, README, quality gates, release docs, docs-site notes, and runtime verification docs must stay aligned.

- DONE M111.2 Add interaction fixture targets
  - Add lightweight Web preview targets for representative runtime-sensitive interactions such as disclosure, overlay open state, selection state, and keyboard-visible state.
  - Preserve existing `data-component-preview`, `data-preview-panel`, and DOM verification targets.
  - Avoid full prop matrices, provider/domain logic, network state, screenshots, and template changes.

- DONE M111.3 Add browser interaction verifier
  - Add an opt-in Playwright verifier that starts the Web preview and exercises the interaction fixture targets.
  - Fail when expected click, keyboard, focus, ARIA, or data-state transitions are missing.
  - Keep the command deterministic, clean up the server process, avoid screenshots by default, and do not claim full accessibility or visual parity.

- DONE M111.4 Wire and document interaction verification
  - Add package script wiring for the focused interaction verification command without making it part of default release gates unless explicitly scoped.
  - Document setup, browser executable behavior, non-goals, and troubleshooting in README, quality gates, release docs, docs-site notes, and runtime verification docs.
  - Keep browser artifact policy and repository hygiene aligned so generated screenshots or traces remain ignored if later enabled.

- DONE M111.5 Complete runtime interaction verification milestone
  - Run interaction verification, rendered DOM verification, rendered coverage checks, Web preview checks, docs checks, package script checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, or template rewrites are committed.
  - Update TODO status only after commits and validation.

## M112 Web Preview Screenshot Smoke

- DONE M112.1 Plan Web preview screenshot smoke
  - Define the first opt-in Web screenshot smoke contract for desktop and mobile preview viewports.
  - Keep pixel diffing, shadcn/ui visual parity claims, CI workflow activation, Desktop WebView screenshots, native Mobile screenshots, component API changes, and template rewrites out of scope.
  - Decide which screenshot artifact names, environment variables, package scripts, README, quality gates, release docs, docs-site notes, browser artifact policy, and screenshot docs must stay aligned.

- DONE M112.2 Add Web screenshot smoke verifier
  - Add an opt-in Playwright verifier that starts the Web preview, checks stable preview panels, captures desktop and mobile screenshots only when explicitly enabled, and validates PNG metadata.
  - Fail when required panels, chart SVG, fallback rows, overlay dialog, interaction fixtures, viewport dimensions, or screenshot metadata are missing.
  - Keep the command deterministic, clean up the server process, keep screenshots ignored, and avoid visual diff or pixel comparison.

- DONE M112.3 Wire and document Web screenshot smoke
  - Add package script wiring for the focused Web screenshot smoke command without making it part of default or release gates unless explicitly scoped.
  - Document setup, browser executable behavior, screenshot opt-in variables, artifact paths, non-goals, and troubleshooting in README, quality gates, release docs, docs-site notes, screenshot docs, and browser artifact policy docs.
  - Keep repository hygiene aligned so generated Web screenshot artifacts remain ignored and are not committed.

- DONE M112.4 Complete Web screenshot smoke milestone
  - Run Web screenshot smoke with screenshot capture, rendered DOM verification, runtime interaction verification, rendered coverage checks, Web preview checks, docs checks, package script checks, browser artifact policy checks, repo hygiene checks, and diff checks.
  - Verify no committed screenshots, traces, generated docs, component API changes, or template rewrites remain after validation.
  - Update TODO status only after commits and validation.

## M113 Browser Smoke Aggregate

- DONE M113.1 Plan browser smoke aggregate
  - Define a local opt-in aggregate for browser-backed Web preview verification commands that must run serially.
  - Keep CI workflow activation, screenshot capture by default, release gate promotion, Desktop WebView screenshots, native Mobile automation, component API changes, and template rewrites out of scope.
  - Decide which package scripts, README, quality gates, release docs, docs-site notes, browser artifact policy, and browser verification docs must stay aligned.

- DONE M113.2 Add serial browser smoke aggregate
  - Add a focused npm alias that runs rendered component DOM verification, Web screenshot smoke, runtime interaction verification, and mobile browser smoke sequentially.
  - Preserve each command's existing browser executable, screenshot opt-in, server cleanup, and artifact behavior.
  - Avoid parallel execution, generated wrapper artifacts, screenshots by default, CI workflow changes, and release aggregate changes.

- DONE M113.3 Wire and document browser smoke aggregate
  - Document the aggregate command, serial execution requirement, browser executable behavior, screenshot opt-in variables, and non-goals in README, quality gates, release docs, docs-site notes, and browser verification docs.
  - Extend package script verification so the aggregate stays wired without being promoted into default or release gates.
  - Keep browser artifact policy and repository hygiene aligned with existing screenshot and trace boundaries.

- DONE M113.4 Complete browser smoke aggregate milestone
  - Run the serial browser smoke aggregate, package script checks, docs checks, browser artifact policy checks, repo hygiene checks, and diff checks.
  - Verify no committed screenshots, traces, generated docs, component API changes, template rewrites, or CI workflow files remain after validation.
  - Update TODO status only after commits and validation.

## M114 Release Screenshot Review Checklist

- DONE M114.1 Plan release screenshot review checklist
  - Define a manual release-candidate screenshot review workflow built on the existing opt-in browser smoke commands.
  - Keep pixel diffing, automated visual baselines, CI workflow activation, release gate promotion, Desktop WebView screenshots, native Mobile automation, component API changes, and template rewrites out of scope.
  - Decide which review checklist docs, README, quality gates, release docs, docs-site notes, browser artifact policy, and package-script metadata must stay aligned.

- DONE M114.2 Add release screenshot review checklist
  - Add a docs checklist for capturing Web and Mobile browser screenshots, recording metadata, reviewing core panels, and cleaning generated artifacts.
  - Cover form, message, chart, overlay, interaction, mobile profile, component inventory, typography, spacing, overflow, and responsive layout review points.
  - Keep the checklist manual, artifact-light, and independent from default or release gates.

- DONE M114.3 Wire and document screenshot review checklist
  - Link the checklist from README, quality gates, release docs, docs-site notes, component docs index, screenshot smoke docs, and browser smoke aggregate docs.
  - Extend docs verification if needed so the checklist remains discoverable without adding runtime automation.
  - Keep browser artifact policy aligned with review artifact capture and cleanup expectations.

- DONE M114.4 Complete release screenshot review checklist milestone
  - Run docs checks, package script checks, browser artifact policy checks, repo hygiene checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M115 Screenshot Artifact Retention Decision

- DONE M115.1 Plan screenshot artifact retention decision
  - Define the decision boundary for release-candidate screenshot artifacts after manual review.
  - Compare attaching screenshots to GitHub releases, storing internal review notes, keeping local ignored artifacts only, and deleting artifacts after review.
  - Keep artifact uploads, GitHub release automation, CI workflow activation, pixel diffing, visual baselines, component API changes, and template rewrites out of scope.

- DONE M115.2 Add screenshot artifact retention decision
  - Document the chosen initial policy for screenshot artifact retention, naming, cleanup, and review note references.
  - Include when screenshots may be attached to release notes, when they should stay local only, and what metadata should be copied into review notes.
  - Keep generated PNG files ignored and uncommitted.

- DONE M115.3 Wire and document retention policy
  - Link the retention decision from the release screenshot review checklist, release docs, quality gates, docs-site notes, browser artifact policy docs, and CI browser docs.
  - Extend metadata checks if needed so the retention policy remains discoverable without uploading artifacts.
  - Keep repository hygiene aligned with screenshot artifact naming and cleanup expectations.

- DONE M115.4 Complete screenshot artifact retention milestone
  - Run docs checks, browser artifact policy checks, repo hygiene checks, package script checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M116 Release Screenshot Review Notes Template

- DONE M116.1 Plan release screenshot review notes template
  - Define the smallest reusable Markdown review notes template for screenshot capture metadata, reviewer decisions, observed issues, and retention outcomes.
  - Keep the template local-first and repository-safe: no committed PNG files, no uploads, no CI workflow activation, no generated docs, and no visual baseline claims.
  - Decide which existing release, quality, CI, site, and screenshot retention docs should link to the template.

- DONE M116.2 Add release screenshot review notes template
  - Add a copyable Markdown template under component docs for release-candidate screenshot review notes.
  - Include Web desktop, Web mobile, mobile browser, command output, PNG metadata, environment, issue list, decision, retention outcome, cleanup evidence, and follow-up fields.
  - Keep screenshot files ignored and uncommitted.

- DONE M116.3 Wire and verify review notes template discoverability
  - Link the template from the release screenshot review checklist, screenshot artifact retention policy, release docs, quality gates, docs-site notes, CI browser docs, and docs indexes.
  - Extend docs index or artifact policy metadata checks if needed so the template remains discoverable.
  - Keep browser commands opt-in and avoid adding runtime screenshot capture.

- DONE M116.4 Complete release screenshot review notes template milestone
  - Run docs checks, browser artifact policy checks, repo hygiene checks, package script checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M117 Release Candidate Browser Review Runbook

- DONE M117.1 Plan release candidate browser review runbook
  - Define the smallest manual release-candidate browser review sequence that combines deterministic release gates, opt-in browser smoke, optional screenshots, review notes, retention policy, and cleanup evidence.
  - Keep the runbook local-first and repository-safe: no committed screenshots, no uploads, no CI workflow activation, no generated docs, no component API changes, and no source-copy template rewrites.
  - Decide which existing README, release, quality, CI, site, checklist, notes template, and retention docs should link to the runbook.

- DONE M117.2 Add release candidate browser review runbook
  - Add a Markdown runbook under component docs with prerequisites, command order, browser executable options, screenshot capture options, review notes handoff, retention cleanup, failure triage, and non-goals.
  - Keep browser commands opt-in and outside default/release gates.
  - Reference existing smoke, checklist, notes template, and retention docs instead of duplicating detailed review criteria.

- DONE M117.3 Wire and verify runbook discoverability
  - Link the runbook from README, docs index, components README, release docs, quality gates, CI browser docs, screenshot review checklist, notes template, retention policy, and docs-site notes.
  - Extend docs index or browser artifact policy checks if needed so the runbook remains discoverable.
  - Keep artifact hygiene and screenshot naming expectations aligned.

- DONE M117.4 Complete release candidate browser review runbook milestone
  - Run docs checks, browser artifact policy checks, repo hygiene checks, package script checks, release docs checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M118 Release Candidate Handoff Checklist

- DONE M118.1 Plan release candidate handoff checklist
  - Define the smallest final handoff checklist for a release candidate after deterministic gates and optional browser review.
  - Include release gate evidence, browser review evidence, publish readiness blockers, known warning inventory, artifact hygiene, and unresolved follow-ups.
  - Keep the checklist repository-safe: no committed screenshots, no release artifacts, no generated docs, no CI workflow activation, no component API changes, and no template rewrites.

- DONE M118.2 Add release candidate handoff checklist
  - Add a Markdown checklist under release docs for final maintainer handoff.
  - Reference release gates, browser review runbook, screenshot notes template, retention policy, publish readiness blockers, release warning inventory, and repository hygiene.
  - Keep it manual and local-first without publishing, tagging, attaching artifacts, or activating workflows.

- DONE M118.3 Wire and verify handoff checklist discoverability
  - Link the handoff checklist from README, docs index, release docs, quality gates, docs-site notes, browser review runbook, and publish readiness runbook where appropriate.
  - Extend docs index or release documentation checks if needed so the handoff checklist remains discoverable.
  - Keep browser review and artifact upload boundaries aligned.

- DONE M118.4 Complete release candidate handoff checklist milestone
  - Run docs checks, release docs checks, package script checks, repo hygiene checks, browser artifact policy checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, tags, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M119 Release Candidate Handoff Metadata Gate

- DONE M119.1 Plan release candidate handoff metadata gate
  - Define a read-only verification gate for the release candidate handoff checklist.
  - Check required checklist sections, command references, publish readiness links, warning inventory links, browser review links, artifact hygiene boundaries, and non-goals.
  - Keep the gate deterministic and offline with no publishing, tagging, artifact creation, screenshot capture, CI activation, or browser launch.

- DONE M119.2 Add release candidate handoff metadata verifier
  - Add `scripts/release-candidate-handoff-verify.mjs`.
  - Add `npm run verify:release-candidate-handoff`.
  - Verify the checklist, release docs, quality gates, README, docs index, docs-site notes, browser review runbook, and publish readiness runbook remain aligned.

- DONE M119.3 Wire handoff metadata gate into release checks
  - Add the focused gate to `npm run verify:release` in the appropriate documentation metadata section.
  - Update release docs, quality gates, README verification summary, package script expectations, and docs-site notes.
  - Keep browser review optional and outside default/release browser execution.

- DONE M119.4 Complete release candidate handoff metadata gate milestone
  - Run docs checks, release docs checks, package script checks, handoff metadata checks, repo hygiene checks, browser artifact policy checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, tags, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M120 Release Gate Failure Triage Runbook

- DONE M120.1 Plan release gate failure triage runbook
  - Define the smallest manual triage flow for failures inside `npm run verify:release`.
  - Group failures by Rust workspace, CLI registry/list smoke, metadata gates, docs gates, source-copy fixture, feature checks, browser artifact policy, release warning inventory, handoff metadata, and repository hygiene.
  - Keep the runbook read-only by default and avoid destructive cleanup, generated artifact commits, browser workflow activation, publishing, tagging, component API changes, or template rewrites.

- DONE M120.2 Add release gate failure triage runbook
  - Add a Markdown runbook under release docs with failure categories, first commands to rerun, evidence to collect, owner handoff notes, and explicit non-goals.
  - Reference release docs, quality gates, release candidate handoff checklist, handoff metadata gate, browser review runbook, and publish readiness runbook.
  - Keep fixes manual and scoped instead of adding automatic repair behavior.

- DONE M120.3 Wire and verify triage runbook discoverability
  - Link the triage runbook from README, docs index, release docs, quality gates, docs-site notes, release candidate handoff checklist, and handoff metadata docs.
  - Extend release docs or handoff metadata checks if needed so the triage runbook remains discoverable.
  - Keep release failure triage outside default browser execution and outside publish automation.

- DONE M120.4 Complete release gate failure triage runbook milestone
  - Run docs checks, release docs checks, package script checks, handoff metadata checks, repo hygiene checks, browser artifact policy checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, tags, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M121 Full Release Gate Audit

- DONE M121.1 Plan full release gate audit
  - Define the audit order for `npm run verify:release`, first-failure triage, optional browser-local verification, and final handoff evidence.
  - Keep the audit repository-safe: no publishing, no tagging, no artifact upload, no screenshot capture by default, no CI workflow activation, no generated docs commits, no component API changes, and no source-copy template rewrites unless a focused failure requires them.
  - Record which documents and commands provide the source of truth for failures and handoff.

- DONE M121.2 Run full release gate
  - Run `npm run verify:release`.
  - If it fails, isolate the first failed command with `docs/release-gate-failure-triage-runbook.md`, apply the minimal fix, rerun the focused command, and rerun the release gate as needed.
  - Record known warnings, failures, fixes, and final result.

- DONE M121.3 Run optional browser-local verification
  - Run the browser-local aggregate with `DIOXUS_UI_BROWSER_EXECUTABLE` when local Chrome is available.
  - Keep screenshots disabled unless a focused failure requires manual screenshot review.
  - Record browser verification result and any skipped reason.

- DONE M121.4 Complete full release gate audit
  - Run final docs checks, release docs checks, package script checks, handoff metadata checks, repo hygiene checks, browser artifact policy checks, package lock checks, and diff checks.
  - Verify no screenshots, traces, generated docs, component API changes, template rewrites, release artifacts, tags, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M122 Publish Readiness Decision Handoff

- DONE M122.1 Plan publish readiness decision handoff
  - Define which publish readiness blockers require maintainer decisions versus local implementation.
  - Map each blocker to concrete evidence, owner input, local follow-up files, and safe validation commands.
  - Keep the plan repository-safe: no publishing, no packaging, no crates.io lookup, no repository URL replacement, no license text generation, no API stabilization decision, and no release notes generation.

- DONE M122.2 Add publish readiness decision matrix
  - Add a documentation page that classifies blockers by decision owner, local action, validation command, and exit criteria.
  - Link it from the publish readiness resolution runbook, release candidate handoff checklist, docs index, quality gates, README, and docs-site notes as needed.
  - Preserve existing blocker metadata until a maintainer intentionally resolves a blocker.

- DONE M122.3 Add first-publish maintainer handoff template
  - Add a copyable checklist for repository identity, license file approval, API stability decision, release notes readiness, CLI template packaging, crates.io ownership, and workspace dependency publish readiness.
  - Keep it as a planning artifact, not a publish authorization.
  - Include final verification commands without adding new release automation.

- DONE M122.4 Complete publish readiness handoff milestone
  - Run docs checks, release docs checks, publish readiness checks, package script checks, handoff metadata checks, repo hygiene checks, browser artifact policy checks, package lock checks, and diff checks.
  - Verify no package archives, publish commands, repository URL changes, license text files, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M123 API Stability Surface Audit

- DONE M123.1 Plan API stability surface audit
  - Define a repository-safe audit for public crate exports, component props, primitive helper functions, features, registry entries, and source-copy templates.
  - Classify API surfaces by stability risk without deciding semantic versioning policy or freezing APIs.
  - Keep the plan read-only: no version changes, no API rewrites, no migration guide generation, no packaging, and no publishing.

- DONE M123.2 Add public API surface inventory
  - Document current public modules, feature flags, component names, source-copy targets, and primitive helper groups.
  - Mark which surfaces are source-copy user-facing, crate-mode user-facing, internal implementation detail, or publish-blocker follow-up.
  - Link the inventory from API stability metadata, publish readiness matrix, release docs, quality gates, README, and docs index as needed.

- DONE M123.3 Add API stability review checklist
  - Add a maintainer checklist for reviewing component naming, prop naming, feature names, primitive helpers, registry slugs, and source-copy compatibility before first publish.
  - Include safe validation commands and explicit non-goals.
  - Keep the checklist separate from API stabilization approval.

- DONE M123.4 Complete API stability surface audit milestone
  - Run docs checks, API stability readiness checks, publish readiness checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no version changes, component API rewrites, template rewrites, package archives, publish commands, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M124 CLI Template Embedding Readiness

- DONE M124.1 Plan CLI template embedding
  - Define the compile-time registry/template embedding strategy for `dioxus-ui-cli`.
  - Identify code, tests, metadata gates, docs, generated fixture smoke, and release gate updates required to resolve the CLI template packaging blocker.
  - Keep planning repository-safe: no package archives, no `cargo package`, no `cargo publish`, no install test, no registry contact, and no template rewrites unless required by embedding.

- DONE M124.2 Implement embedded CLI asset catalog
  - Add a compile-time asset catalog for registry JSON files and template/source-copy assets.
  - Update `dxui list` and `dxui add` to read embedded registry and asset contents instead of repository paths.
  - Preserve recursive dependencies, overwrite behavior, `utils` handling, generated target paths, and list ordering.

- DONE M124.3 Update CLI template packaging readiness metadata
  - Change the focused readiness gate from "blocker remains unresolved" to "embedded template delivery is active".
  - Update publish blockers, Cargo publish metadata, publish readiness coverage, publish readiness runbook, release docs, quality gates, README, docs-site notes, and decision handoff docs.
  - Keep other publish blockers unresolved unless separately approved.

- DONE M124.4 Complete CLI template embedding milestone
  - Run CLI tests, generated fixture smoke, registry checks, docs checks, release docs checks, publish readiness checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no package archives, publish commands, repository URL changes, license files, version changes, API rewrites, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M125 First Publish Readiness Planning

- DONE M125.1 Plan first publish readiness resolution cycle
  - Define a repository-safe plan for resolving the remaining publish blockers in maintainer-approved order.
  - Split maintainer decisions from local implementation follow-up for repository identity, license files, API stability, release notes, registry availability, and workspace dependency publish readiness.
  - Keep the plan read-only: no repository URL changes, no license text generation, no crates.io contact, no package archives, no publish commands, no version changes, no tags, and no CI workflow activation.

- DONE M125.2 Add blocker resolution evidence checklist
  - Add a maintainer-facing checklist that records required evidence before each blocker can move from current to resolved.
  - Include safe local validation commands for each blocker without approving the decision.
  - Link the checklist from publish readiness blockers, resolution runbook, decision matrix, release docs, quality gates, README, and docs-site notes as needed.

- DONE M125.3 Add first-publish local implementation map
  - Map each approved maintainer decision to exact local files that would change after approval.
  - Document expected validation gates and rollback considerations for each blocker.
  - Keep local changes hypothetical unless maintainer input is already committed.

- DONE M125.4 Complete first publish readiness planning milestone
  - Run docs checks, release docs checks, publish readiness checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no package archives, publish commands, repository URL changes, license files, version changes, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M126 API Stability Decision Preparation

- DONE M126.1 Plan API stability decision preparation
  - Define a repository-safe preparation pass for deciding whether current `0.1.x` crate-mode APIs are acceptable for first publish.
  - Identify review artifacts for public component props, feature names, registry slugs, template paths, primitive helpers, and core exports.
  - Keep the plan read-only: no API rewrites, no version changes, no migration guide generation, no package archives, no publish commands, and no stability approval.

- DONE M126.2 Add API stability decision record template
  - Add a maintainer-facing decision record that can capture accepted, deferred, and required-change API surfaces.
  - Include explicit evidence fields for breaking-change policy, migration note expectations, and first-publish acceptance.
  - Link the template from API stability metadata, public API inventory, review checklist, first-publish planning docs, release docs, quality gates, README, and docs-site notes.

- DONE M126.3 Add API stability local follow-up map
  - Map each possible maintainer decision to local files and gates that would change after approval.
  - Separate source-copy compatibility follow-up from crate-mode stability follow-up.
  - Keep all changes hypothetical unless a maintainer decision is already committed.

- DONE M126.4 Complete API stability decision preparation milestone
  - Run docs checks, API stability readiness checks, publish readiness checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no API rewrites, version changes, migration guides, package archives, publish commands, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M127 Workspace Dependency Publish Readiness Preparation

- DONE M127.1 Plan workspace dependency publish readiness preparation
  - Define a repository-safe preparation pass for deciding the internal crate dependency strategy before first publish.
  - Identify review artifacts for workspace dependencies, crate manifests, publish order, version policy, and Cargo-supported publish behavior.
  - Keep the plan read-only: no dependency version changes, no manifest rewrites, no `cargo package`, no `cargo publish`, no crates.io contact, and no publish authorization.

- DONE M127.2 Add workspace dependency evidence checklist
  - Add a maintainer-facing checklist for internal dependency version policy, publish order, package ownership, and crates.io-resolvable dependency evidence.
  - Include safe validation commands without approving or applying dependency metadata changes.
  - Link the checklist from workspace dependency readiness metadata, publish order metadata, first-publish planning docs, release docs, quality gates, README, and docs-site notes.

- DONE M127.3 Add workspace dependency local follow-up map
  - Map approved dependency strategies to exact local files and gates that would change after approval.
  - Separate local development path dependencies from publishable dependency version metadata.
  - Keep all changes hypothetical unless maintainer input is already committed.

- DONE M127.4 Complete workspace dependency publish readiness preparation milestone
  - Run docs checks, workspace dependency readiness checks, publish order checks, Cargo workspace checks, Cargo publish metadata checks, publish readiness checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no dependency version changes, manifest rewrites, package archives, publish commands, crates.io contact, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M128 Release Notes Readiness Preparation

- DONE M128.1 Plan release notes readiness preparation
  - Define a repository-safe preparation pass for deciding first-publish release note scope and known warning text.
  - Identify review artifacts for `CHANGELOG.md`, release notes readiness metadata, changelog metadata, release docs, and publish blocker updates.
  - Keep the plan read-only: no generated release notes, no Git history derivation, no git-cliff run, no tags, no GitHub release, no package archives, and no publish commands.

- DONE M128.2 Add release notes evidence checklist
  - Add a maintainer-facing checklist for included changes, excluded changes, known warnings, release owner, and changelog owner.
  - Include safe validation commands without approving release contents or generating notes.
  - Link the checklist from release notes readiness metadata, changelog metadata, first-publish planning docs, release docs, quality gates, README, and docs-site notes.

- DONE M128.3 Add release notes local follow-up map
  - Map approved release note decisions to local files and gates that would change after approval.
  - Separate changelog structure ownership from publish-ready release note completeness.
  - Keep all changes hypothetical unless maintainer input is already committed.

- DONE M128.4 Complete release notes readiness preparation milestone
  - Run docs checks, release notes readiness checks, changelog checks, publish readiness checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no generated release notes, Git history derivation, git-cliff output, tags, GitHub releases, package archives, publish commands, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M129 Repository Identity Decision Preparation

- DONE M129.1 Plan repository identity decision preparation
  - Define a repository-safe preparation pass for deciding the canonical repository owner and URL before first publish.
  - Identify review artifacts for workspace metadata, repository identity metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and docs-site notes.
  - Keep the plan read-only: no repository URL replacement, no remote repository lookup, no crates.io contact, no package archives, no publish commands, no version changes, and no CI workflow activation.

- DONE M129.2 Add repository identity decision record template
  - Add a maintainer-facing decision record that can capture approved, blocked, and deferred repository identity outcomes.
  - Include explicit evidence fields for canonical URL, repository owner, remote availability confirmation, metadata update approval, and rollback expectations.
  - Link the template from repository identity metadata, first-publish planning docs, release docs, quality gates, README, and docs-site notes.

- DONE M129.3 Add repository identity local follow-up map
  - Map each possible maintainer decision to local files and gates that would change after approval.
  - Separate placeholder metadata follow-up from Cargo workspace metadata follow-up.
  - Keep all changes hypothetical unless a maintainer decision is already committed.

- DONE M129.4 Complete repository identity decision preparation milestone
  - Run docs checks, repository identity readiness checks, publish readiness checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no repository URL changes, remote lookups, crates.io contact, package archives, publish commands, version changes, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M130 License Decision Preparation

- DONE M130.1 Plan license decision preparation
  - Define a repository-safe preparation pass for deciding root license files and copyright holder text before first publish.
  - Identify review artifacts for workspace license metadata, license readiness metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, and docs-site notes.
  - Keep the plan read-only: no license text generation, no root license file commits, no license expression changes, no copyright holder changes, no package archives, no publish commands, and no crates.io contact.

- DONE M130.2 Add license decision record template
  - Add a maintainer-facing decision record that can capture approved, blocked, and deferred license file outcomes.
  - Include explicit evidence fields for `LICENSE-MIT`, `LICENSE-APACHE`, copyright holder, license expression confirmation, file commit approval, and rollback expectations.
  - Link the template from license readiness metadata, first-publish planning docs, release docs, quality gates, README, and docs-site notes.

- DONE M130.3 Add license local follow-up map
  - Map each possible maintainer decision to local files and gates that would change after approval.
  - Separate workspace license expression follow-up from root license file follow-up.
  - Keep all changes hypothetical unless a maintainer decision is already committed.

- DONE M130.4 Complete license decision preparation milestone
  - Run docs checks, license readiness checks, publish readiness checks, Cargo publish metadata checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no license files, license text generation, license expression changes, copyright holder changes, package archives, publish commands, crates.io contact, generated release notes, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M131 Publish Blocker Resolution Tracking

- DONE M131.1 Plan coordinated publish blocker resolution tracking
  - Create a single tracker for the six current publish blockers: placeholder repository URL, root license files, pre-1.0 API stability, release notes readiness, registry availability, and workspace dependency publish readiness.
  - Map each blocker to its existing decision-preparation artifacts, required maintainer evidence, local follow-up files, focused gates, and unresolved status.
  - Keep the tracker repository-safe: no repository URL replacement, no root license file commits, no API stabilization approval, no release-note generation, no crates.io contact, no dependency version changes, no package archives, and no publish commands.

- DONE M131.2 Prepare placeholder repository URL blocker handoff
  - Consolidate repository identity decision inputs, evidence requirements, focused gates, rollback expectations, and local follow-up files.
  - Confirm docs continue to show the placeholder repository URL blocker as unresolved until maintainer approval is committed.
  - Run repository identity, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M131.3 Prepare root license files blocker handoff
  - Consolidate license decision inputs, evidence requirements, focused gates, rollback expectations, and local follow-up files.
  - Confirm docs continue to show root `LICENSE-MIT` and `LICENSE-APACHE` files as unresolved until reviewed license files are committed.
  - Run license readiness, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M131.4 Prepare pre-1.0 API stability blocker handoff
  - Consolidate API stability decision inputs, review inventory, evidence requirements, focused gates, rollback expectations, and local follow-up files.
  - Confirm docs continue to show crate-mode API stability as unresolved until maintainer approval or a stabilization worklist is committed.
  - Run API stability readiness, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M131.5 Prepare release notes readiness blocker handoff
  - Consolidate release note scope inputs, evidence requirements, focused gates, rollback expectations, and local follow-up files.
  - Confirm docs continue to show release notes as not publish-ready until an approved first-publish release note scope is committed.
  - Run release notes readiness, changelog, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M131.6 Prepare registry availability blocker handoff
  - Consolidate planned crate names, owner and credential evidence requirements, publish-order evidence, focused gates, rollback expectations, and local follow-up files.
  - Confirm docs continue to show crates.io registry availability as unresolved until the release owner checks availability and ownership.
  - Run registry availability, publish order, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M131.7 Prepare workspace dependency publish readiness blocker handoff
  - Consolidate internal dependency version policy inputs, publish-order dependencies, evidence requirements, focused gates, rollback expectations, and local follow-up files.
  - Confirm docs continue to show workspace dependency publish readiness as unresolved until publish-ready internal dependency metadata is approved.
  - Run workspace dependency publish readiness, publish order, Cargo workspace, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M131.8 Complete publish blocker resolution tracking milestone
  - Run docs checks, all six focused blocker readiness checks, publish readiness coverage, runbook, release docs, package script, repo hygiene, package lock, and diff checks.
  - Verify no repository URL changes, root license files, API rewrites, generated release notes, crates.io contact, dependency version changes, package archives, publish commands, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M132 First Publish Decision Packet

- DONE M132.1 Plan first publish decision packet
  - Define a repository-safe maintainer review packet that groups the six current publish blockers and their handoff documents into one copyable decision surface.
  - Link each blocker to required evidence, decision owner, possible decision states, local follow-up boundaries, rollback expectations, and focused gates.
  - Keep the packet read-only: no repository URL changes, no root license files, no API rewrites, no generated release notes, no crates.io contact, no dependency version changes, no package archives, and no publish commands.

- DONE M132.2 Add first publish decision packet
  - Add a copyable document for release owners to record all six blocker decisions in one review pass.
  - Include explicit placeholders for blocker state, evidence location, local follow-up owner, rollback owner, and verification commands.
  - Link the packet from the decision matrix, publish blocker tracker, release candidate handoff checklist, maintainer handoff template, README, and docs index.

- DONE M132.3 Complete first publish decision packet milestone
  - Run docs checks, publish readiness blocker checks, coverage checks, runbook checks, release docs checks, package script checks, repo hygiene checks, package lock checks, and diff checks.
  - Verify no repository URL changes, root license files, API rewrites, generated release notes, crates.io contact, dependency version changes, package archives, publish commands, tags, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M133 Approved Publish Blocker Resolution

- DONE M133.1 Plan approved publish blocker resolution
  - Record the new maintainer decisions: canonical repository `https://github.com/yuxuetr/dioxus-ui`, MIT license, current `0.1.x` API acceptable for first publish, and first publish allowed.
  - Split locally resolvable blockers from external crates.io readiness: repository identity, license, API stability, release notes, and workspace dependency metadata can be updated locally; registry availability remains deferred until crates.io evidence exists.
  - Keep the milestone safe: no crates.io contact, no credential inspection, no `cargo package`, no `cargo publish`, no package archives, no tags, and no GitHub releases.

- DONE M133.2 Resolve repository identity blocker
  - Replace placeholder repository metadata with `https://github.com/yuxuetr/dioxus-ui`.
  - Update repository identity readiness docs, publish blockers, Cargo publish metadata, release docs, quality gates, README, docs-site notes, decision packet, and focused verifier expectations.
  - Run repository identity, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M133.3 Resolve MIT license blocker
  - Change workspace license metadata to MIT and commit reviewed MIT root license text.
  - Update license readiness docs, publish blockers, Cargo publish metadata, release docs, quality gates, README, docs-site notes, decision packet, and focused verifier expectations.
  - Run license readiness, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M133.4 Resolve API stability and release notes blockers
  - Record that current `0.1.x` APIs are accepted for first publish with pre-1.0 breaking-change policy.
  - Record first publish release note scope without generating tags, releases, or git-derived notes.
  - Update API stability, release notes, changelog, publish blockers, release docs, quality gates, README, docs-site notes, decision packet, and focused verifier expectations.

- DONE M133.5 Resolve workspace dependency publish readiness blocker
  - Add approved crates.io-resolvable internal dependency version metadata while preserving local path development.
  - Update workspace dependency metadata, publish order, Cargo publish metadata, publish blockers, release docs, quality gates, README, docs-site notes, decision packet, and focused verifier expectations.
  - Run workspace dependency publish readiness, publish order, Cargo workspace, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M133.6 Defer crates.io registry availability blocker with required evidence
  - Keep registry availability unresolved unless crates.io names, ownership, credentials, and publish order are confirmed by a release owner.
  - Document the exact required crates.io evidence and clarify that deferral blocks crates.io publishing but not local release readiness.
  - Run registry availability, publish order, Cargo publish metadata, publish readiness, release docs, docs, package script, repo hygiene, package lock, and diff checks.

- DONE M133.7 Complete approved publish blocker resolution milestone
  - Run docs checks, all focused blocker readiness checks, publish readiness coverage, runbook, Cargo publish metadata, publish order, release docs, package script, repo hygiene, package lock, and diff checks.
  - Verify no crates.io contact, credential inspection, package archives, publish commands, tags, GitHub releases, screenshots, traces, or CI workflow files are committed.
  - Update TODO status only after commits and validation.

## M134 Packaging Verification Before crates.io

- DONE M134.1 Move CLI registry and templates into the CLI crate
  - `cargo package -p dioxus-ui-cli --list` shows `registry/` and `templates/` are not packaged because `build.rs` reads them from the workspace root, so the published CLI would fail to build.
  - Move `registry/` and `templates/` to `crates/dioxus-ui-cli/`, read them from `CARGO_MANIFEST_DIR`, and update tests, scripts, generated catalog docs, and path references.
  - Run CLI tests, registry, docs catalog, generated fixture smoke, tailwind static, docs, and diff checks.

- DONE M134.2 Add package contents gate
  - Add `npm run verify:package-contents` that runs `cargo package --list` per publishable crate and asserts the CLI package contains every embedded registry and template asset.
  - Wire it into `npm run verify:release` and update package scripts, release docs, quality gates, README, and docs-site notes.
  - Reverse-verify that the gate fails when an embedded asset is outside the package.

- DONE M134.3 Run workspace publish dry run
  - Run `cargo publish --workspace --dry-run` without uploading to crates.io and fix any packaging or verification failures it reports.
  - Record the dry-run evidence and the exact release-owner publish commands in release readiness docs.
  - Do not run `cargo publish` without `--dry-run`, create tags, or create GitHub releases.

- DONE M134.4 Complete packaging verification milestone
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and confirm no package archives are committed.
  - Push local commits to `origin/main`.
  - Leave crates.io registry availability deferred; actual publish remains a release-owner action.

## M135 Overlay Interaction Behavior

- DONE M135.1 Design overlay interaction behavior
  - Record that styled overlay parts currently render `open` state only: no Escape, outside-click, focus, trap, restore, or anchored positioning, and the runtime traits only have record-keeping example adapters.
  - Define the additive API (`on_open_change`, `dismiss`, anchor id, side, align, offset), the `document::eval` focus-scope and measurement approach shared by Web, Desktop, and Mobile renderers, and how source-copy templates stay self-contained.
  - List excluded overlays (Select, Combobox, Date Picker, Navigation Menu, Context Menu, Menubar, DOM portal) with executable reevaluation conditions.

- DONE M135.2 Implement Dialog modal behavior
  - Dialog content closes on Escape, overlay closes on pointer when `dismiss.outside_pointer`, and Dialog Close requests close through `on_open_change`.
  - Opening focuses the first focusable element (or the content), Tab and Shift+Tab wrap inside the content, and closing restores focus to the previously focused element.
  - Mirror the behavior in the source-copy template and shared utils, update the docs page, and keep existing `open`-only usage working.

- DONE M135.3 Apply modal behavior to Alert Dialog, Sheet, and Drawer
  - Reuse the Dialog focus scope and dismissal wiring; Alert Dialog keeps outside pointer dismissal disabled.
  - Update templates, docs pages, and tests for each component.

- DONE M135.4 Implement Popover anchored positioning and dismissal
  - Measure the anchor, content, and viewport when open, place content with `compute_overlay_placement` (flip and shift), and render it with fixed positioning.
  - Close on Escape, outside pointer, and focus outside according to `DismissBehavior::popover_default()`.
  - Mirror placement in the source-copy template without importing internal crates.

- DONE M135.5 Apply anchored positioning to Dropdown, Hover Card, and Tooltip
  - Reuse Popover measurement and placement; Dropdown dismisses on Escape and outside pointer, Tooltip on Escape only.
  - Update templates, docs pages, and tests for each component.

- DONE M135.6 Verify overlay behavior in a real browser
  - Render the real Dialog, Alert Dialog, and Popover components in the Web preview interaction panel.
  - Extend `npm run verify:runtime-interactions` to assert Escape, overlay click, Tab wrap, focus restore, and in-viewport placement with flip near the viewport edge.
  - Reverse-verify that the script fails when the behavior is removed.

- DONE M135.7 Complete overlay interaction milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, roadmap Stage 7 status, and component docs to match what is now verified.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M136 Toast and Sonner Runtime Behavior

- DONE M136.1 Design toast timer and live region behavior
  - Record that Toast and Sonner render open state only: no auto-dismiss timer, no hover or focus pause, close and action buttons without handlers, and per-toast live regions inserted together with their content.
  - Define the additive API (`duration_ms`, `on_dismiss` with `ToastDismissReason`), the page-side countdown that pauses while the pointer is over or focus is inside the toast, and a persistent viewport live region.
  - Record Stage 7 Toast and Sonner exit criterion scope and what stays app-owned (queue state, swipe, stacking animation).

- DONE M136.2 Implement Toast auto-dismiss and dismiss callbacks
  - `ToastRoot` counts down `duration_ms` while open, pauses on hover and focus within, and calls `on_dismiss(Timeout)`; `0` disables the timer.
  - `ToastClose` calls `on_dismiss(Close)` and `ToastAction` runs `onclick` then `on_dismiss(Action)`.
  - Mirror the timer helper into the utils template with a script parity test, and update the docs page.

- DONE M136.3 Apply timer and dismiss callbacks to Sonner
  - Reuse the Toast timer for `SonnerToast`, `SonnerClose`, and `SonnerAction` with the same reasons and defaults.
  - Update the template and docs page.

- DONE M136.4 Verify toast behavior in a real browser
  - Make Toast and Sonner viewports persistent `role="region"` live regions labelled for notifications so additions are announced.
  - Render real Toast and Sonner components in the Web preview and extend `npm run verify:runtime-interactions` to assert timeout dismissal, hover pause, close and action reasons, and live region attributes.
  - Reverse-verify that the script fails when the pause or timer is removed.

- DONE M136.5 Complete toast runtime milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, roadmap Stage 7 status, and accessibility docs.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M137 Select and Combobox Listbox Behavior

- DONE M137.1 Design listbox overlay behavior
  - Record that Select and Combobox render open state only: no trigger toggle, no anchored placement, no keyboard navigation, no typeahead, no option selection callback, and a hardcoded `aria-expanded` on the Combobox input.
  - Define the additive API (`on_open_change`, `on_value_change`, trigger and input `id`, content anchoring props), the page-side listbox script that tracks the highlighted option with `aria-activedescendant`, and the Select-only typeahead.
  - Record what stays out of scope (multi-select, async loading, the input-inside-content Combobox layout) with reevaluation conditions.

- DONE M137.2 Implement Select listbox behavior
  - `SelectTrigger` toggles through `on_open_change` and opens on ArrowDown or ArrowUp; `SelectContent` anchors to the trigger, highlights the selected or first enabled option, moves with arrows, Home, End, and typeahead, selects on Enter, Space, or click, and closes through `on_open_change`.
  - Mirror the listbox helper into the utils template with a script parity test, and update the docs page.

- DONE M137.3 Implement Combobox listbox behavior
  - `ComboboxInput` gains `id`, `open`, `placeholder`, `oninput`, and `on_open_change`; `ComboboxContent` anchors to the input and reuses the listbox helper without typeahead so typing stays in the input.
  - Highlight resets when filtering removes the highlighted option; update the template and docs page.

- DONE M137.4 Verify listbox behavior in a real browser
  - Render real Select and Combobox components in the Web preview and extend `npm run verify:runtime-interactions` to assert placement, arrow navigation that skips disabled options, typeahead, Enter and click selection, focus staying on the trigger or input, filtering, and Escape.
  - Reverse-verify that the script fails when navigation or typeahead is removed.

- DONE M137.5 Complete listbox milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and RFC 0010 out-of-scope status.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M138 Date Picker and Calendar Keyboard Behavior

- DONE M138.1 Design date picker and calendar keyboard behavior
  - Record that Calendar parts have no click, keyboard, or focus handling, that Date Picker content has no anchor, focus entry, or dismissal, and that RFC 0010 deferred Date Picker until Popover placement was verified.
  - Define the additive API (`CalendarDay` `focused`, `on_select`, `on_key_move`, `CalendarNavButton` `onclick`, `calendar_key_move`, Date Picker trigger `id` and `on_open_change`, content anchoring and dismissal props), roving tabindex with DOM focus following `focused`, and focus entry on the focused day.
  - Record what stays app-owned (visible month, date math in source-copy mode, text parsing, range selection) with reevaluation conditions.

- DONE M138.2 Implement Calendar keyboard and selection hooks
  - `CalendarDay` renders roving tabindex when keyboard-managed, maps arrows, Page Up, Page Down (Shift for years), Home, and End to `CalendarKeyMove` through `on_key_move`, calls `on_select` on click, and moves DOM focus when `focused` turns true.
  - `CalendarNavButton` gains `onclick`; mirror `CalendarKeyMove` and `calendar_key_move` into the template, and update the docs page.

- DONE M138.3 Implement Date Picker anchored dialog behavior
  - `DatePickerTrigger` gains `id` and toggles through `on_open_change`; `DatePickerContent` anchors to the trigger, moves focus to the focused day on open, wraps Tab, restores focus on close, and closes on Escape or outside interaction.
  - The focus scope prefers a `data-dxui-autofocus` element, skips `tabindex="-1"` elements when wrapping, and restores focus only when focus was not moved elsewhere; update templates and docs.

- DONE M138.4 Verify date picker behavior in a real browser
  - Render a real Date Picker with a Calendar in the Web preview and extend `npm run verify:runtime-interactions` to assert placement, focus entry on the selected day, arrow, Page, Home, and End movement across months, Tab wrap, Enter and click selection with focus return, Escape, and outside dismissal.
  - Reverse-verify that the script fails when key mapping, focus following, or focus entry is removed.

- DONE M138.5 Complete date picker milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and RFC 0010 out-of-scope status.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M139 Dropdown and Context Menu Keyboard Behavior

- DONE M139.1 Design menu keyboard behavior
  - Record that Dropdown items have no focus, keyboard, or activation handling, that Context Menu content has no placement or dismissal, and that RFC 0010 deferred Context Menu until a point anchor existed.
  - Define the menu mode of the listbox script (DOM focus on items, wrapping arrows, Home, End, typeahead, Enter and Space activation, close and focus return), item `onclick` props, and a point anchor for Context Menu.
  - Record what stays out of scope (submenus, Menubar cross-menu navigation, checkbox and radio state) with reevaluation conditions.

- DONE M139.2 Implement Dropdown menu navigation
  - Opening focuses the first enabled item; arrows wrap, Home, End, and typeahead move focus; Enter, Space, or click activates the item through its `onclick`, closes the menu, and returns focus to the element focused before opening.
  - Extend the listbox script and template with the menu mode, keep the parity test, and update the docs page.

- DONE M139.3 Implement Context Menu point anchoring and navigation
  - `ContextMenuContent` gains `anchor_point`, `on_open_change`, and `dismiss`; the anchored overlay script places content at a viewport point with flip and shift.
  - Context Menu items gain `onclick` and reuse the menu mode; update templates and docs.

- DONE M139.4 Verify menu behavior in a real browser
  - Render real Dropdown and Context Menu components in the Web preview and extend `npm run verify:runtime-interactions` to assert focus entry, wrapping arrows that skip disabled items, typeahead, activation with close and focus return, Escape, Tab, and point placement.
  - Reverse-verify that the script fails when wrapping, activation, or focus return is removed.

- DONE M139.5 Complete menu milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and RFC 0010 out-of-scope status.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M140 Menubar Keyboard Behavior

- DONE M140.1 Design menubar keyboard behavior
  - Record that Menubar triggers and content render `open` state only, with no placement, dismissal, focus handling, or cross-menu movement, and that RFC 0010 deferred Menubar until cross-menu roving focus existed.
  - Define trigger roving focus (one Tab stop, Left, Right, Home, End), opening with ArrowDown, Enter, Space, or click, Left and Right moving to the adjacent menu while one is open, pointer hover switching, and focus return to the open menu's trigger.
  - Record what stays out of scope (submenus, ArrowUp opening on the last item, right-to-left mirroring) with reevaluation conditions.

- DONE M140.2 Implement Menubar navigation
  - Add a menubar script for trigger roving focus, adjacent-menu switching, and hover switching, reporting the next menu's value to Rust.
  - `MenubarTrigger` gains `id` and `on_open_change`; `MenubarContent` reuses the anchored overlay and the listbox menu mode; items gain `onclick`; mirror the template and keep the parity test.

- DONE M140.3 Verify menubar behavior in a real browser
  - Render a real Menubar in the Web preview and extend `npm run verify:runtime-interactions` to assert roving focus that skips disabled triggers, opening, adjacent-menu switching, hover switching, activation with close and focus return, Escape, and Tab.
  - Reverse-verify that the script fails when roving, switching, or focus return is removed.

- DONE M140.4 Complete menubar milestone
  - Update component docs, CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and RFC 0010 out-of-scope status.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M141 Navigation Menu Interaction Behavior

- DONE M141.1 Design navigation menu interaction behavior
  - Record that Navigation Menu triggers and content render `open` state only, with no click, keyboard, hover, or dismissal handling, and that RFC 0010 deferred it because its content uses CSS layout rather than anchored placement.
  - Define the disclosure navigation pattern: click and Enter or Space toggle, hover opens after a delay and closes after leaving, Left, Right, Home, and End move between top-level items, ArrowDown enters content, Up and Down move between content links, and Escape, outside presses, and focus leaving the menu close it.
  - Record what stays out of scope (viewport size measurement, submenus, motion, right-to-left mirroring) with reevaluation conditions.

- DONE M141.2 Implement Navigation Menu interaction
  - Add a navigation menu script that handles trigger clicks, keyboard movement, hover timing, and dismissal, and reports the item value to open (empty to close) through `NavigationMenu` `on_value_change`.
  - `NavigationMenuItem` gains `value`; mirror the template, keep the parity test, and update the docs page.

- DONE M141.3 Verify navigation menu behavior in a real browser
  - Render a real Navigation Menu in the Web preview and extend `npm run verify:runtime-interactions` to assert click toggling, arrow movement, content entry and link movement, Escape with focus return, hover open and close timing, and outside and focus-out dismissal.
  - Reverse-verify that the script fails when arrow movement, content entry, hover opening, or dismissal is removed.

- DONE M141.4 Complete navigation menu milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and RFC 0010 out-of-scope status.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M142 Desktop Interaction Verification

- DONE M142.1 Design desktop interaction verification
  - Record that Desktop shares every interaction script with Web through `document::eval`, that the Desktop preview renders the same interaction fixtures, and that no automated check runs them in the Desktop WebView because WKWebView has no WebDriver.
  - Define an in-app self-test: the Desktop preview runs a scenario script in its own WebView when an environment variable is set, reports results through `document::eval`, and exits with a status code; synthetic events cover script and Rust handler behavior but not native default actions.
  - Record what stays out of scope (Mobile, native key defaults such as Tab movement, CI activation) with reevaluation conditions.

- DONE M142.2 Implement the Desktop self-test
  - Add the self-test component to the Desktop preview and a scenario script covering each interaction path: modal focus scope, anchored overlay, listbox, menu mode, dismiss timer, calendar focus following, Menubar, and Navigation Menu.
  - Add `npm run verify:desktop-interactions`, which builds and runs the self-test with a timeout and reports the result.

- DONE M142.3 Reverse-verify the Desktop self-test
  - Confirm the self-test fails when an interaction script path is broken (focus return, listbox selection, menu switching, calendar focus) and when the scenario times out.
  - Keep the command out of `npm run verify:release`, like the browser smoke, because it opens a window and needs a GUI session.

- DONE M142.4 Complete desktop verification milestone
  - Update the Desktop and Mobile verification strategy, runtime interaction docs, quality gates, Known Pre-1.0 Limitations, and CHANGELOG Unreleased notes.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop self-test.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M143 Mobile Interaction Verification

- DONE M143.1 Design mobile interaction verification
  - Record that Mobile shares the interaction scripts with Web and Desktop, that no Mobile app exists in the workspace, and that a probe ran the RFC 0017 scenarios in an iOS Simulator build with every scenario passing.
  - Define a Mobile preview app, a scenario script and self-test component shared with Desktop, and a command that builds for the iOS Simulator, installs, launches with the self-test variable, and reads the result from the console.
  - Record what stays out of scope (Android, physical devices, touch gestures, CI activation) with reevaluation conditions.

- DONE M143.2 Implement the Mobile self-test
  - Move the scenario script and self-test component into `preview-states` so Desktop and Mobile run the same scenarios, and add `PreviewTarget::Mobile`.
  - Add `examples/mobile-demo` and `npm run verify:mobile-interactions`, which selects an iPhone simulator, boots it when needed, and reports the self-test result.

- DONE M143.3 Reverse-verify the Mobile self-test
  - Confirm the command fails when an interaction path is broken and when the app does not report a result.
  - Keep the command out of `npm run verify:release` because it needs Xcode and a simulator.

- DONE M143.4 Complete mobile verification milestone
  - Update the Desktop and Mobile verification strategy, the Mobile checklist, component docs, quality gates, Known Pre-1.0 Limitations, and CHANGELOG Unreleased notes.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop and Mobile self-tests.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M144 Roving Group Interaction

- DONE M144.1 Design roving group interaction
  - Record that Tabs, Radio Group, and Toggle Group render state only: Tabs triggers have no click handler, no item handles arrow keys, Radio Group and Toggle Group leave every item out of the Tab order when nothing is selected, and Tabs triggers and panels are not linked by ids.
  - Define one shared roving group script: one Tab stop per group, arrow keys by orientation with optional wrapping, Home and End, disabled items skipped, selection following focus for Tabs and Radio Group, focus only for Toggle Group, and clicks reported to Rust.
  - Define the Rust API (`Tabs` root with `on_value_change` and trigger and panel ids, `RadioGroup` `on_value_change`, `ToggleGroup` `on_toggle`) and record what stays out of scope (manual tab activation, vertical tabs, right-to-left mirroring, Desktop and Mobile self-test scenarios) with reevaluation conditions.

- DONE M144.2 Implement roving group interaction
  - Add the roving group script and hook to the crate and the template `utils.rs`, keep the parity test, and wire Tabs, Radio Group, and Toggle Group in the crate and the templates.
  - Update the component docs pages.

- DONE M144.3 Verify roving group behavior in a real browser
  - Render real Tabs, Radio Group, and Toggle Group in the Web preview and extend `npm run verify:runtime-interactions` to assert Tab stops, arrow movement that skips disabled items and wraps, Home and End, selection following focus, clicks, Toggle Group focus without pressing, and Tabs id links with Tab moving into the panel.
  - Reverse-verify that the script fails when the Tab stop, arrow movement, or selection reporting is removed.

- DONE M144.4 Complete roving group milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and preview coverage metadata.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M145 Android Interaction Verification

- DONE M145.1 Design Android interaction verification
  - Record that the Mobile self-test covers only the iOS Simulator, that the Rust Android target is now installed, and that a probe ran the eight scenarios in an Android emulator (API 36.1) with every scenario passing after building with an NDK copy whose flattened symlinks were restored.
  - Define the request channel (`am start` passes no environment, so the command sets the `debug.dioxus_ui.self_test` system property and the app reads it with `getprop`), the result channel (Rust stdout in logcat under `RustStdoutStderr`), and emulator selection, boot, and shutdown.
  - Record what stays out of scope (physical devices, x86_64 emulators, touch gestures, CI activation, repairing a broken NDK install) with reevaluation conditions.

- DONE M145.2 Implement the Android self-test
  - Read the self-test request from the system property in `examples/mobile-demo` on Android.
  - Add `npm run verify:android-interactions`, which builds the APK with `dx build --android`, selects or boots an emulator, installs, sets the property, launches, reads the result from logcat, and clears the property; it reports a broken NDK install before building.

- DONE M145.3 Reverse-verify the Android self-test
  - Confirm the command fails when an interaction path is broken, when the app reports no result, and when the NDK has flattened symlinks.
  - Keep the command out of `npm run verify:release` because it needs the Android SDK, NDK, and an emulator.

- DONE M145.4 Complete Android verification milestone
  - Update the Desktop and Mobile verification strategy, the Mobile checklist, component docs, quality gates, Known Pre-1.0 Limitations, and CHANGELOG Unreleased notes.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop, iOS, and Android self-tests.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M146 Accordion Interaction

- DONE M146.1 Design accordion interaction
  - Record that Accordion renders state only: `AccordionItem` has no value, triggers have no click reporting, no trigger handles arrow keys, triggers have no heading wrapper, and triggers and content are not linked by ids.
  - Define an `Accordion` root that reports toggled items, `AccordionItem` `value`, heading-wrapped triggers with `aria-controls`, region content with `aria-labelledby`, and Up, Down, Home, and End movement between enabled triggers that keeps every trigger in the Tab order, reusing the roving group script.
  - Record what stays out of scope (an expanded item that cannot collapse, horizontal accordions, animation, Desktop and Mobile self-test scenarios) with reevaluation conditions.

- DONE M146.2 Implement accordion interaction
  - Add an every-item Tab stop mode to the roving group script in the crate and the template `utils.rs`, share the part id builder with Tabs, and wire Accordion in the crate and the template.
  - Add single and multiple open-value helpers and update the Accordion docs page.

- DONE M146.3 Verify accordion behavior in a real browser
  - Render a real Accordion in the Web preview and extend `npm run verify:runtime-interactions` to assert id links and region names, clicks and Enter toggling, collapsing an open item, arrow movement that skips disabled triggers and wraps, Home and End, focus without toggling, and Tab reaching every enabled trigger.
  - Reverse-verify that the script fails when click reporting, arrow movement, the every-item Tab stop mode, or the id links are removed.

- DONE M146.4 Complete accordion milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and preview coverage metadata.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop self-test.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M147 Tooltip Hover And Focus Opening

- DONE M147.1 Design tooltip hover and focus opening
  - Record that Tooltip has only a content part: the app wires hover and focus handlers on its own trigger, there is no open delay, moving the pointer onto the content closes it, and the trigger is not linked to the content with `aria-describedby`.
  - Define a `Tooltip` root that reports open requests, a `TooltipTrigger` that links to the content while it is open, and a page script that opens after a hover delay, opens at once on keyboard focus, stays open while the pointer is over the trigger or content, and closes on pointer leave, blur, and trigger presses.
  - Record what stays out of scope (Hover Card timing, skipping the delay between adjacent tooltips, touch long press, Desktop and Mobile self-test scenarios) with reevaluation conditions.

- DONE M147.2 Implement tooltip hover and focus opening
  - Add the tooltip script and wire `Tooltip`, `TooltipTrigger`, and `TooltipContent` in the crate and the template, with a CLI parity test for the script.
  - Update the Tooltip docs page.

- DONE M147.3 Verify tooltip behavior in a real browser
  - Render the Web preview tooltip with the new parts and extend `npm run verify:runtime-interactions` to assert the hover delay, `aria-describedby`, staying open over the content, closing on pointer leave, immediate keyboard focus opening, closing on blur and Escape, and a trigger press closing it without reopening.
  - Reverse-verify that the script fails when the delay, content hover, focus opening, press closing, or the description link is removed.

- DONE M147.4 Complete tooltip milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and preview coverage metadata.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop self-test.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M148 Hover Card Hover And Focus Opening

- DONE M148.1 Design hover card hover and focus opening
  - Record that Hover Card has only a content part: the app wires hover and focus handlers on its own trigger, there is no open or close delay, and moving the pointer onto the card closes it.
  - Define a `HoverCard` root that reports open requests, a link `HoverCardTrigger`, and a shared hover-open script, moved out of Tooltip, that opens after a hover delay, opens at once on keyboard focus, stays open while the pointer or focus is on the trigger or the card, and closes after a close delay; Hover Card keeps trigger presses open and adds no `aria-describedby`.
  - Record what stays out of scope (skipping delays between adjacent cards, touch, non-link triggers, Desktop and Mobile self-test scenarios) with reevaluation conditions.

- DONE M148.2 Implement hover card hover and focus opening
  - Move the tooltip script into a shared hover-open module in the crate and the template `utils.rs`, keep Tooltip behavior unchanged, and wire `HoverCard`, `HoverCardTrigger`, and `HoverCardContent` in the crate and the template.
  - Update the Hover Card and Tooltip docs pages.

- DONE M148.3 Verify hover card behavior in a real browser
  - Render a real Hover Card in the Web preview and extend `npm run verify:runtime-interactions` to assert the open delay, the close delay, staying open over the card, trigger presses keeping it open, immediate keyboard focus opening, Tab into the card keeping it open, closing on Escape and outside presses, and no `aria-describedby`, while the tooltip assertions still pass.
  - Reverse-verify that the script fails when the open delay, the close delay, the card focus exemption, the press exemption, or the description exemption is removed.

- TODO M148.4 Complete hover card milestone
  - Update CHANGELOG Unreleased notes, Known Pre-1.0 Limitations, accessibility docs, and preview coverage metadata.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop self-test.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
