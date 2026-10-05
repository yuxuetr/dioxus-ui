# Examples

The examples are workspace members so they stay checked by `cargo check
--workspace`.

Current examples are command-line smoke applications. They verify local crate
wiring and representative crate-mode states before a rendered Dioxus preview app
lands.

Representative preview states live in `examples/preview-states`
(`dioxus-ui-preview-states`) so command-line smoke output, future Web previews,
and future Desktop previews can share the same inventory.

## Web Demo

```bash
cargo run -p dioxus-ui-web-demo
```

Rendered preview shell:

```bash
dx serve --package dioxus-ui-web-demo --bin preview
```

The preview shell uses `examples/web-demo/assets/preview.css` as a Tailwind CSS
v4 source input. It is not a committed complete Tailwind output file.

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

The Desktop rendered preview uses `examples/desktop-demo/assets/preview.css` as
a Tailwind CSS v4 source input. It is not a committed complete Tailwind output
file. `cargo run -p dioxus-ui-desktop-demo` remains the Desktop smoke command.

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
structural preview gate and screenshot capture remains planned.

## Examples Metadata

```bash
npm run verify:examples-metadata
```

This checks example workspace membership, package script wiring, smoke script
references, preview verifier references, and this README. It does not run
examples, compile generated fixtures, launch previews, install browser
dependencies, or rewrite documentation.
