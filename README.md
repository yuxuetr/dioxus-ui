# dioxus-ui

`dioxus-ui` aims to be a shadcn/ui-style component system for Dioxus:

- headless primitives for behavior, accessibility, state, and composition
- Tailwind CSS styled components as the default visual layer
- a CLI that copies component source into user projects
- an optional packaged crate for users who prefer dependency-based usage

The project is intentionally documentation-first. The initial goal is to define
the architecture and development sequence before implementing components.

## Product Direction

The library should not be a pure Tailwind component package. The target shape is:

```text
Dioxus headless/primitive logic layer
+ Tailwind default style layer
+ CLI component source generator
```

This gives users two workflows:

1. Add source code into an app and customize it freely.
2. Depend on a crate and enable only the component features they need.

The source-copy workflow is the priority for early versions because Dioxus and
the component APIs are expected to evolve quickly.

## Planned Repository Layout

```text
dioxus-ui/
├─ crates/
│  ├─ dioxus-ui-core/         # shared types, class merging, theme tokens
│  ├─ dioxus-ui-primitives/   # unstyled logic components
│  ├─ dioxus-ui/              # styled public components
│  └─ dioxus-ui-cli/          # dxui init / dxui add
├─ registry/                  # component metadata used by the CLI
├─ templates/                 # source templates copied by the CLI
├─ examples/
│  ├─ web-demo/
│  └─ desktop-demo/
└─ docs/
   └─ rfcs/
```

Example run commands are documented in [examples/README.md](examples/README.md).

## Planned Usage

### Source-Copy Mode

```bash
cargo install dioxus-ui-cli

dxui init
dxui add button
dxui add dialog
dxui add input
```

`dxui add` keeps existing component files by default. Use
`dxui add button --overwrite` when you intentionally want to replace a
previously generated component file.

Expected output in a Dioxus app:

```text
src/components/ui/button.rs
src/components/ui/dialog.rs
src/components/ui/input.rs
src/components/ui/utils.rs
assets/dioxus-ui.css
```

For Tailwind CSS v4, `assets/dioxus-ui.css` should be an input stylesheet, not
a precompiled full Tailwind output:

```css
@import "tailwindcss";

@theme {
  --color-background: var(--dxui-background);
  --color-foreground: var(--dxui-foreground);
}
```

The user's Dioxus app build should produce the final CSS after scanning the app
source and generated component files.

### Crate Mode

```toml
[dependencies]
dioxus-ui = { version = "0.1", default-features = false, features = ["button", "input", "dialog"] }
```

```rust
use dioxus_ui::{Button, Dialog, Input};
```

Crate mode comes after the copied source API stabilizes.

## Component Scope

Early components:

- Button
- Input
- Textarea
- Label
- Checkbox
- Switch
- Badge
- Card
- Alert
- Avatar
- Separator
- Tabs
- Accordion

Later primitive-first components:

- Dialog
- Dropdown
- Popover
- Tooltip
- Toast
- Select
- Command

The later group needs stronger accessibility and interaction design, including
focus management, keyboard navigation, ARIA attributes, portal behavior,
outside-click handling, and positioning.

## Tailwind Rule

Tailwind class names must appear as complete source tokens. Runtime selection is
allowed, but runtime class construction is not.

Use this:

```rust
match variant {
  ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
  ButtonVariant::Secondary => "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
}
```

Avoid this:

```rust
format!("bg-{}-500", color)
```

## Documentation

- [Design Overview](docs/design.md)
- [Roadmap](docs/roadmap.md)
- [Workspace Specification](docs/workspace.md)
- [Component API Specification](docs/component-api.md)
- [Release and Package Strategy](docs/release.md)
- [Quality Gates](docs/quality-gates.md)
- [CI Browser Smoke Guide](docs/ci-browser-smoke.md)
- [CI Browser Workflow Template](docs/ci-browser-workflow-template.md)
- [Component Catalog](docs/components/README.md)
- [Documentation Site Plan](docs/site.md)
- [TODO Plan](TODOs.md)
- [RFC 0001: Project Architecture](docs/rfcs/0001-project-architecture.md)
- [RFC 0002: CLI Registry and Code Generation](docs/rfcs/0002-cli-registry-and-code-generation.md)
- [RFC 0003: Tailwind Styling Contract](docs/rfcs/0003-tailwind-styling-contract.md)
- [RFC 0009: CI Browser Workflow Activation](docs/rfcs/0009-ci-browser-workflow-activation.md)

## Verification Shortcuts

The repository includes `package.json` metadata for Node-based verification
aliases and future browser smoke tests.

Install JavaScript dependencies:

```bash
npm install
```

Run the default deterministic local gate:

```bash
npm run verify
```

This wraps preview/example smoke plus docs metadata checks. It does not run full
Rust workspace tests, browser automation that needs an installed browser,
screenshots, or release-only gates.

Run the local release gate before publishing:

```bash
npm run verify:release
```

This runs Rust workspace checks, CLI registry/list smoke, the default local
gate, feature checks, generated source-copy fixture smoke, release docs
consistency checks, package script wiring checks, and CI browser docs checks. It
also checks CI Plan documentation while keeping browser installation and
screenshots opt-in, then checks repository hygiene for forbidden generated
artifacts and inactive workflow files.

Run deterministic preview and example gates only:

```bash
npm run verify:smoke
```

This wraps existing local gates for Web preview, Mobile Web profile, Desktop
preview, and example smoke output. It does not require Playwright browser
binaries.

Verify the docs-site catalog metadata contract only:

```bash
npm run verify:docs
```

For focused debugging, the aggregate command expands to:

```bash
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-status
npm run verify:docs-structure
npm run verify:docs-routes
npm run verify:docs-source-preview
npm run verify:docs-index
npm run verify:docs-links
npm run verify:docs-anchors
```

This builds the catalog in memory from registry entries, templates, component
docs, crate features, and crate modules. The page check also verifies
`docs/components/catalog.md` matches the shared catalog builder output. The
status check verifies `docs/components/status.md` matches the local component
implementation surface. The structure check verifies public component docs keep
the required title, install, API, and accessibility sections. The route check
verifies future docs runtime route metadata. The source preview check verifies
template metadata for future source preview routes. The index check verifies
README and docs/README keep the required project entry points. The link target
check verifies tracked Markdown files do not reference missing local files. The
anchor check verifies local Markdown fragments match headings or explicit
anchors.

Verify component implementation status only:

```bash
npm run verify:docs-status
```

This checks that the generated component status snapshot still matches local
registry, template, docs, crate feature, crate module, source preview, and
source-copy target metadata. The coverage matrix is local wiring status, not
runtime visual parity.

Verify public component docs structure only:

```bash
npm run verify:docs-structure
```

This checks required component docs sections and generated command/feature
snippets. It does not score prose quality or assert runtime visual parity.

Verify documentation index consistency only:

```bash
npm run verify:docs-index
```

This checks that the root README and docs README link the required quality,
release, CI, site, RFC, and TODO entry points. It also checks that every
`package.json` `verify` and `verify:*` alias is documented in
`docs/quality-gates.md`.

Verify local Markdown link targets only:

```bash
npm run verify:docs-links
```

This checks local relative links in tracked Markdown files. It does not validate
external URLs or heading fragments.

Verify local Markdown anchors only:

```bash
npm run verify:docs-anchors
```

This checks same-file and relative Markdown fragments in tracked Markdown files.
It does not validate external URL anchors or generated docs runtime routes.

Verify release documentation consistency only:

```bash
npm run verify:release-docs
```

This checks that release docs still distinguish the local aggregate command,
required Rust/source-copy/feature release gates, and opt-in browser smoke.

Verify registry metadata only:

```bash
npm run verify:registry
```

This checks registry entry names, descriptions, source mappings, source-copy
targets, dependency references, and asset mappings. It does not execute CLI
commands or replace the Rust registry tests.

Verify Tailwind static token safety only:

```bash
npm run verify:tailwind-static
```

This checks shipped Rust source and templates for dynamic Tailwind utility
tokens such as `bg-{...}`. It does not compile Tailwind CSS or assert visual
parity.

Verify npm verification alias wiring only:

```bash
npm run verify:package-scripts
```

This checks `package.json` script relationships and local script target
existence without executing Cargo, browser automation, generated fixture smoke,
or release commands.

Verify repository hygiene only:

```bash
npm run verify:repo-hygiene
```

This checks that inactive browser workflow files and known generated artifacts
are not committed. It reports drift but does not remove files.

Verify CI browser smoke documentation only:

```bash
npm run verify:ci-docs
```

This checks that the opt-in CI browser smoke guide and workflow template still
reference the current local gates without creating active workflow files.

Verify CI Plan documentation only:

```bash
npm run verify:ci-plan
```

This checks that the quality gate CI Plan still references the current PR,
release, and opt-in browser verification boundaries.

Future browser-rendered Playwright smoke will require an explicit browser
install:

```bash
npx playwright install chromium
```

After Chromium is installed, run the opt-in mobile browser smoke:

```bash
npm run verify:mobile-browser
```

If Playwright-managed Chromium is unavailable but local Chrome is installed,
use an explicit executable path:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
```

To save an ignored local screenshot after the assertions pass, add:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

For local Chrome plus screenshot capture:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

Screenshots use the ignored `dioxus-ui-mobile-browser-preview-*.png` pattern.
When screenshot capture is enabled, the smoke also validates the generated PNG
signature, byte size, and dimensions against the mobile viewport lower bound.

That browser smoke starts the Web preview, checks a mobile browser viewport, and
cleans up the server. It is not part of default release gates.

For CI setup options and non-blocking workflow policy, see the
[CI Browser Smoke Guide](docs/ci-browser-smoke.md) and
[CI Browser Workflow Template](docs/ci-browser-workflow-template.md).

## References

- Dioxus RSX and UI documentation: <https://dioxuslabs.com/learn/0.7/essentials/ui/rsx/>
- Dioxus components direction: <https://github.com/DioxusLabs/dioxus-components>
- Tailwind class detection: <https://tailwindcss.com/docs/detecting-classes-in-source-files>
- Tailwind v4 installation: <https://tailwindcss.com/docs/installation>
- Dioxus component macro docs: <https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html>
