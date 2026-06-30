# Examples

The examples are workspace members so they stay checked by `cargo check
--workspace`.

Current examples are command-line smoke applications. They verify local crate
wiring and representative crate-mode states before a rendered Dioxus preview app
lands.

Representative preview states live in `examples/preview-states` so command-line
smoke output, future Web previews, and future Desktop previews can share the
same inventory.

## Web Demo

```bash
cargo run -p dioxus-ui-web-demo
```

Planned future command after the Dioxus web runtime is added:

```bash
dx serve --package dioxus-ui-web-demo
```

## Desktop Demo

```bash
cargo run -p dioxus-ui-desktop-demo
```

Planned future command after the Dioxus desktop runtime is added:

```bash
dx serve --package dioxus-ui-desktop-demo --platform desktop
```

## Runtime Web Verification

```bash
cargo run -p dioxus-ui-runtime-web-verification
```

This fixture starts as a compile-checked runtime contract scaffold. Browser
automation should be added separately after the panels can render under a Web
runtime.

For the current expensive runtime check, run:

```bash
cargo test -p dioxus-ui-runtime-web-verification
cargo run -p dioxus-ui-runtime-web-verification
node scripts/runtime-web-verify.mjs
```

The Node command is intentionally separate from default checks. It validates
the browser-assertion prerequisites exposed by the runtime fixture before real
Web runtime automation is added. It also validates the example-only SVG Chart
fixture prerequisites for sizing, responsive viewBox output, fallback table
rows, and reduced-motion behavior before a public Chart component is added.

## Runtime Desktop Verification

```bash
cargo run -p dioxus-ui-runtime-desktop-verification
```

This fixture starts as a compile-checked Desktop WebView smoke scaffold.
Renderer-backed Desktop automation should be added separately after the smoke
states are stable.

For the current expensive runtime check, run:

```bash
cargo test -p dioxus-ui-runtime-desktop-verification
cargo run -p dioxus-ui-runtime-desktop-verification
```

## CLI Init Smoke

```bash
cargo run -p dioxus-ui-cli -- init --root /tmp/dxui-demo
```

## CLI Add Smoke

```bash
cargo run -p dioxus-ui-cli -- add button --root /tmp/dxui-demo
```

## Generated Fixture Smoke

```bash
scripts/generated-fixture-smoke.sh
```

## Example Smoke

```bash
scripts/example-smoke.sh
```

This runs the Web and Desktop demo crates and verifies representative states for
composition, form-specific, message, scroller, direction, collapsible, and chart
components. The examples are not screenshot or visual parity proof yet.
