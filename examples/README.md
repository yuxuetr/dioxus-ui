# Examples

The examples are workspace members so they stay checked by `cargo check
--workspace`.

Current examples are skeleton applications. They verify local crate wiring before
the first Dioxus runtime and component implementations land.

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
