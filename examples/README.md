# Examples

The examples are workspace members so they stay checked by `cargo check
--workspace`.

The demo crates run as command-line smoke applications by default, which verify
local crate wiring and representative crate-mode states, and as rendered
preview shells through their `preview` binaries.

Representative preview states live in `examples/preview-states`
(`dioxus-ui-preview-states`) so the command-line smoke output and the Web,
Desktop, and Mobile previews share the same inventory.

## Web Demo

```bash
cargo run -p dioxus-ui-web-demo
```

Rendered preview shell:

```bash
dx serve --package dioxus-ui-web-demo --bin preview
```

`examples/web-demo/assets/preview.css` is the Tailwind CSS v4 input. The
previews link the committed compiled output,
`examples/preview-states/assets/preview.generated.css`, which
`npm run css:preview` rebuilds (RFC 0049).

Structural preview gate:

```bash
node scripts/web-preview-verify.mjs
```

## Desktop Demo

```bash
cargo run -p dioxus-ui-desktop-demo
```

Rendered Desktop preview shell:

```bash
dx serve --package dioxus-ui-desktop-demo --bin preview --platform desktop
```

The Desktop rendered preview links the same committed compiled stylesheet as
the Web preview, `examples/preview-states/assets/preview.generated.css`.
`cargo run -p dioxus-ui-desktop-demo` remains the Desktop smoke command.

Structural Desktop preview gate:

```bash
node scripts/desktop-preview-verify.mjs
```

Desktop interaction self-test (RFC 0017), which opens a window:

```bash
npm run verify:desktop-interactions
```

## Mobile Demo

`examples/mobile-demo` (`dioxus-ui-mobile-demo`) renders the shared preview
with `PreviewTarget::Mobile` for the iOS Simulator and Android:

```bash
dx serve --ios --package dioxus-ui-mobile-demo
```

The interaction self-test (RFC 0018) builds for the simulator, boots an iPhone
simulator on iOS 26 or older when none is running, and reads the result from
the app's console. It needs Xcode:

```bash
npm run verify:mobile-interactions
```

Set `DIOXUS_UI_IOS_SIMULATOR` to a device name or UDID to choose the device.

The Android self-test (RFC 0020) builds an arm64 APK, boots the first AVD
headless when no emulator is running, and reads the result from logcat. It
needs the Android SDK, NDK, and an AVD:

```bash
npm run verify:android-interactions
```

Set `DIOXUS_UI_ANDROID_SERIAL` or `DIOXUS_UI_ANDROID_AVD` to choose the device.
Dioxus 0.7 apps stop at launch on iOS 27 because they do not adopt the UIScene
lifecycle.

## Preview State Metadata

```bash
npm run verify:preview-state-metadata
```

This checks the shared preview state inventory, Web/Desktop preview binaries,
structural preview verifier state lists, Tailwind CSS v4 preview inputs, and
release wiring. It validates preview metadata wiring, not browser-rendered
visual correctness or screenshot pixels.

## CSS Input Metadata

```bash
npm run verify:css-inputs
```

This checks the Web and Desktop preview CSS input files plus the CLI default
generated CSS. It validates Tailwind CSS v4 input metadata, not compiled CSS
output or visual styling.

## Runtime Web Verification

```bash
cargo run -p dioxus-ui-runtime-web-verification
```

This fixture is a compile-checked runtime contract scaffold. Browser
automation of the rendered preview runs in
`npm run verify:runtime-interactions`.

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
rows, and reduced-motion behavior.

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

## Class Merge Gate

```bash
node scripts/class-merge-gate.mjs table tw-merge
```

`dioxus-ui-class-merge-gate` runs a class merge candidate over the RFC 0076
corpus for `scripts/class-merge-gate.mjs`: `table`, the `merge_classes` that
`dioxus-shadcn-core` ships with the table `node scripts/class-merge-table.mjs`
generates from Tailwind, or `tw-merge`, the `tw_merge` crate. The gate compares each with Tailwind's
compiled output (see
[RFC 0076](../docs/rfcs/0076-user-class-overrides.md#validation)).

## CLI Init Smoke

```bash
cargo run -p dioxus-shadcn-cli -- init --root /tmp/dxui-demo
```

## CLI Add Smoke

```bash
cargo run -p dioxus-shadcn-cli -- add button --root /tmp/dxui-demo
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
components. Web screenshots are covered by the Web preview gate; Desktop has a
structural preview gate, and Desktop WebView screenshot capture is
unsupported.

## Examples Metadata

```bash
npm run verify:examples-metadata
```

This checks example workspace membership, package script wiring, smoke script
references, preview verifier references, and this README. It does not run
examples, compile generated fixtures, launch previews, install browser
dependencies, or rewrite documentation.
