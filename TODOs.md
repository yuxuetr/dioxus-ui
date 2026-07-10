# TODOs

## Progress

- Overall: 100%
- Current milestone: M76 Quality Gate Alias Coverage Gate
- Current task: M76.1 Plan quality gate alias coverage

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

- TODO M76.1 Plan quality gate alias coverage
  - Define a deterministic read-only check that every `package.json` `verify:*` alias is documented in `docs/quality-gates.md`.
  - Keep command execution, prose quality scoring, and release aggregation behavior out of scope.
  - Decide how to handle opt-in browser aliases and aggregate aliases.

- TODO M76.2 Add quality gate alias coverage check
  - Extend or add a verifier that compares `package.json` verification aliases against `docs/quality-gates.md`.
  - Fail when a `verify:*` alias is missing from the quality gate docs.
  - Include the check in the existing docs or release verification flow without adding browser runtime requirements.

- TODO M76.3 Update quality gate alias documentation
  - Document focused preview and example aliases that are currently only described indirectly.
  - Clarify that opt-in browser smoke remains documented but outside default release gates.
  - Update README, release, or docs-site planning docs only where needed.

- TODO M76.4 Complete quality gate alias coverage milestone
  - Run quality gate alias checks, docs checks, package script checks, release aggregate, and diff checks.
  - Verify no workflow files or generated artifacts are committed.
  - Update TODO status only after commits and validation.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
