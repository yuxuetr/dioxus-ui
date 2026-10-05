# TODOs

## Progress

- Overall: 10%
- Current milestone: M176 Semantic Color Tokens
- Current task: M176.3 Complete the semantic color token foundation

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29) and `docs/archive/TODOs.completed-20261005.md` (M30 to M175)

## Goals

- Components use shadcn/ui semantic color tokens (`bg-primary`, `text-muted-foreground`, `border-input`, and so on) instead of fixed Tailwind palette colors. Themes, the dark theme included, redefine the tokens and leave app classes alone.
- A component site in this repository, built with Dioxus on the published components, lets a visitor browse the catalog and see each component's live examples, source, and install commands in the light and dark themes.

## M176 Semantic Color Tokens

- DONE M176.1 Design the semantic color tokens
  - Inventory the palette utilities in the crate and the templates (about 630 crate and 540 template uses of zinc, blue, red, white, green, amber, emerald, and black) and how each one is used: surfaces, text, borders, checked and selected fills, focus rings, destructive actions, and status variants.
  - Adopt the shadcn/ui token set and its v4 CSS layout: `:root` and `.dark` define `--background`, `--foreground`, `--card`, `--popover`, `--primary`, `--secondary`, `--muted`, `--accent`, `--destructive`, `--border`, `--input`, `--ring`, `--chart-1` to `--chart-5`, the `--sidebar-*` group, and `--radius`, and `@theme inline` maps them to Tailwind colors.
  - Write the palette-to-token mapping table, including where the current blue checked states and focus rings become `primary` and `ring` as in shadcn/ui, and whether toast, sonner, and attachment status variants keep palette colors or get extra tokens.
  - Decide how crate-mode apps get the token stylesheet, how RFC 0047's palette remap is retired, and record the breaking change and the reevaluation conditions in an RFC.

- DONE M176.2 Add the token stylesheet
  - Put the token `:root`, `.dark`, and `@theme inline` blocks in the CLI `DEFAULT_CSS`, replacing the `--dxui-*` variables, and carry them in the Web and Desktop preview inputs.
  - Keep the RFC 0047 palette remap until the components are migrated, so the dark theme keeps working in between.
  - Extend `npm run verify:css-inputs` to fail when the preview token blocks drift from the CLI blocks, and reverse-verify it.

- TODO M176.3 Complete the semantic color token foundation
  - Update the theming docs with the token list and the crate-mode setup.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the browser interaction smoke.
  - Push local commits to `origin/main`; crates.io publish remains a release-owner action.

## M177 Component Token Migration

- TODO M177.1 Migrate actions and forms
  - Button, Button Group, Toggle, Toggle Group, Input, Input Group, Textarea, Input OTP, Label, Field, Checkbox, Radio Group, Switch, Slider, Native Select, Select, and Combobox, in the crate and the templates.
  - Update unit tests, the docs class snippets, the browser checks that compare utility colors, and the compiled preview stylesheet.

- TODO M177.2 Migrate overlays and navigation
  - Dialog, Alert Dialog, Sheet, Drawer, Popover, Hover Card, Tooltip, Dropdown, Context Menu, Menubar, Navigation Menu, Command, Date Picker, Calendar, Tabs, Breadcrumb, Pagination, and Sidebar, in the crate and the templates.
  - Update the same tests, docs, checks, and compiled stylesheet.

- TODO M177.3 Migrate layout, data display, feedback, and messaging
  - The remaining components, including Card, Alert, Badge, Avatar, Table, Data Table, Accordion, Collapsible, Chart, Progress, Skeleton, Spinner, Toast, Sonner, Attachment, Bubble, Message, Message Scroller, and Marker, in the crate and the templates.
  - Update the same tests, docs, checks, and compiled stylesheet.

- TODO M177.4 Complete the component token migration
  - Update CHANGELOG Unreleased notes with the breaking change and a migration note for crate-mode apps.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop self-test.
  - Push local commits to `origin/main`.

## M178 Token-only Dark Theme And Palette Gate

- TODO M178.1 Move the dark theme onto the tokens
  - Remove the RFC 0047 palette remap from the CLI and preview stylesheets; `.dark` redefines only the tokens, so app palette classes keep their colors.
  - Keep the light and dark contrast checks passing, and replace the `bg-white` dark-surface probe with a token surface probe.
  - Mark RFC 0047 as superseded where its palette remap is replaced.

- TODO M178.2 Gate palette colors in component classes
  - Add a verifier that fails when crate or template class strings use palette color utilities outside the exceptions the RFC allows, and add it to the release gate.
  - Reverse-verify that a reintroduced palette color in a component fails it.

- TODO M178.3 Complete the token-only dark theme
  - Update CHANGELOG, README, quality gate, release, roadmap, and site docs.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release`, the browser interaction smoke, and the Desktop self-test.
  - Push local commits to `origin/main`.

## M179 Component Site Shell

- TODO M179.1 Design the component site
  - Define a `site/` workspace crate (`publish = false`) built with Dioxus Web and its router, depending on `dioxus-ui` by path.
  - Define the routes (home, getting started, theming, one page per component), the sidebar catalog from the existing catalog categories and registry metadata, and the light and dark theme toggle.
  - Define how examples and their source are shown (one Rust file per example, rendered live and shown with `include_str!`), how install commands come from the registry, how the site gets compiled Tailwind, and what stays out of scope (hosting, search, a theme editor) in an RFC.

- TODO M179.2 Build the site shell
  - Add the crate, the layout, the sidebar catalog, routing, the home, getting started, and theming pages, and the theme toggle.
  - Add the site's compiled stylesheet with a drift gate like `npm run verify:preview-css`.

- TODO M179.3 Verify the site in the browser
  - Add a browser check that visits every route and fails on a console error, a missing page, low text contrast in either theme, or a sideways scroll at 375px.
  - Reverse-verify it with a broken route and a low-contrast class.

- TODO M179.4 Complete the component site shell
  - Update README, docs index, quality gate, release, and roadmap docs, including how to run the site locally.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the site browser check.
  - Push local commits to `origin/main`.

## M180 Component Pages

- TODO M180.1 Build the component page template
  - Show the description, the `dxui add` command and crate feature, live examples with a preview and source tab, and links to the API and accessibility notes.
  - Fail the site check when a catalog component has no page or an example has no source.

- TODO M180.2 Add actions and forms examples
  - Add examples for every component in the Actions and Forms categories, covering their main variants and states.

- TODO M180.3 Add overlays and navigation examples
  - Add examples for every component in the Overlays and Navigation categories.

- TODO M180.4 Add layout, data display, feedback, and messaging examples
  - Add examples for every remaining component.

- TODO M180.5 Complete the component pages
  - Update CHANGELOG, README, and site docs.
  - Run `CARGO_NET_OFFLINE=true npm run verify:release` and the site browser check.
  - Push local commits to `origin/main`.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
