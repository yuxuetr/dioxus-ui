# Quality Gates

This document defines local and CI verification commands for `dioxus-ui`.

## Default Local Gate

Run before committing implementation changes:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
```

These commands should stay fast enough for regular development.

## Source-Copy Gate

Run after changes to CLI, registry, templates, or component dependencies:

```bash
cargo test -p dioxus-ui-cli --test registry
scripts/generated-fixture-smoke.sh
```

This verifies:

- public registry component names match `dioxus-ui` crate feature names.
- public registry components have docs pages and catalog entries.
- registry template source and target paths match generated module names.
- every template file is registered exactly once, including `utils`.
- `scripts/feature-check.sh` covers every public component feature.
- `dxui list` returns public components.
- `dxui init` creates the generated project structure.
- `dxui add` can add every public component.
- generated `mod.rs` includes every public component and `utils`.
- generated code does not import `dioxus-ui-core` or `dioxus-ui-primitives`.
- generated source compiles with only `dioxus = "0.7"`.

## Feature Gate

Run after changes to `crates/dioxus-ui/Cargo.toml`, crate exports, or feature
gating:

```bash
scripts/feature-check.sh
```

This verifies:

- every public `dioxus-ui` feature compiles independently.
- static component feature combinations compile.
- primitive-backed feature combinations compile.
- all `dioxus-ui` features compile together.

This command invokes Cargo many times and is intentionally separated from the
default local gate.

## Example Smoke Gate

Run after changes to Web or Desktop examples, component demo output, or expanded
parity coverage:

```bash
scripts/example-smoke.sh
```

This verifies that both command-line demo crates expose representative states
for low-risk composition, form-specific, message, scroller, direction,
collapsible, and chart components. It is not a screenshot or visual parity gate.

## Web Preview Gate

Run after changes to the rendered Web preview shell:

```bash
node scripts/web-preview-verify.mjs
```

This verifies the Dioxus Web preview binary, Tailwind CSS v4 source input, stable
`data-preview-*` screenshot targets, and representative shared inventory output.
It is the structural prerequisite for browser screenshots.

For the Playwright screenshot procedure, see
`docs/components/web-preview-screenshot-verification.md`.

## Release Gate

Run before publishing:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
cargo test -p dioxus-ui-cli --test registry
cargo run -p dioxus-ui-cli -- list
scripts/example-smoke.sh
node scripts/web-preview-verify.mjs
node scripts/desktop-preview-verify.mjs
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```

Manual release review:

- Tailwind CSS v4 input stylesheet remains `@import "tailwindcss";`.
- generated templates remain self-contained.
- component docs and registry entries remain in sync.
- accessibility contract changes are reflected in component docs.

## CI Plan

Default pull request CI should run:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
cargo test -p dioxus-ui-cli --test registry
scripts/generated-fixture-smoke.sh
```

Scheduled or release CI should additionally run:

```bash
scripts/feature-check.sh
```

The feature gate is separated because it recompiles the same crate many times.
If CI time remains acceptable, it can be promoted into default pull request CI.

## Runtime Web Verification Gate

Run after changes to runtime verification fixtures or experimental Web runtime
adapters:

```bash
cargo test -p dioxus-ui-runtime-web-verification
cargo run -p dioxus-ui-runtime-web-verification
node scripts/runtime-web-verify.mjs
```

This currently verifies compile-checked runtime panel metadata and visible
fallback status output for focus, portal, timer, live-region, measurement,
scroll command, pointer, and gesture contracts. The Node command is the
separate expensive browser-assertion prerequisite check; it asserts stable
fixture output for focus, portal, timers, live-region, measurement, pointer,
gesture, and Message Scroller prerequisites before a real Web runtime browser
driver is promoted.

Do not promote runtime Web verification into the default release gate until the
fixture renders under a Web runtime and the browser assertions are stable.

## Runtime Desktop Verification Gate

Run after changes to Desktop runtime verification fixtures or Desktop-specific
runtime adapter plans:

```bash
cargo test -p dioxus-ui-runtime-desktop-verification
cargo run -p dioxus-ui-runtime-desktop-verification
```

This currently verifies compile-checked Desktop WebView smoke metadata and
visible fallback status output for focus, portal stacking, timers, live status,
measurement, pointer capture, and conservative gesture checks.

Do not promote runtime Desktop verification into the default release gate until
the fixture starts a real Desktop WebView and the smoke checks are stable.

## Desktop Preview Gate

Run after changes to the rendered Desktop preview shell:

```bash
node scripts/desktop-preview-verify.mjs
```

This verifies the Desktop preview binary, Tailwind CSS v4 source input, stable
`data-preview-*` selectors, and representative command-line Desktop smoke
output. Desktop WebView screenshot capture remains separate until repeatable.

## Runtime Mobile Verification Gate

Mobile verification is documentation-only until a repeatable device or emulator
command exists.

After changes to Mobile runtime plans, review:

```text
docs/components/runtime-mobile-verification-checklist.md
docs/components/runtime-desktop-mobile-verification.md
docs/components/runtime-renderer-verification.md
```

Do not add Mobile runtime adapters or default component behavior until touch,
safe area, visual viewport, native scroll, reduced motion, and visible status
checks can be repeated and fallback states are visible.
