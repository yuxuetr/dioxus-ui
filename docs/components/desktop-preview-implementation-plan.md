# Desktop Preview Implementation Plan

This document defines the M38 implementation plan for the Desktop WebView
preview path.

Status: Planned in M38.1. Implemented through M38.5.

## Goal

Add a rendered Desktop preview path that mirrors the Web preview panels while
preserving the existing command-line Desktop smoke example.

## Implementation Shape

Use an explicit preview binary:

```bash
dx serve --package dioxus-ui-desktop-demo --bin preview --platform desktop
```

Keep the current smoke command stable:

```bash
cargo run -p dioxus-ui-desktop-demo
```

The Desktop preview should reuse the same preview state inventory and visual
panel structure as the Web preview. If possible, extract shared Dioxus preview
panel rendering so Web and Desktop do not drift.

## Required Panels

The Desktop preview must expose these selectors:

```text
[data-preview-root="desktop"]
[data-preview-panel="form"]
[data-preview-panel="message"]
[data-preview-panel="chart"]
[data-preview-panel="overlay-open"]
[data-preview-panel="inventory"]
```

The panels should cover:

- Input Group and Input OTP states
- user and assistant message states
- attachment, marker, and scroller states
- SVG chart and fallback table
- an open overlay state or explicit inline fallback

## Verification

The first Desktop gate should be structural and compile-oriented:

```bash
cargo check -q -p dioxus-ui-desktop-demo --bin preview
node scripts/desktop-preview-verify.mjs
scripts/example-smoke.sh
```

`desktop-preview-verify.mjs` should assert that:

- the Desktop preview source contains the required `data-preview-*` selectors
- the Desktop preview binary compiles
- command-line Desktop smoke output still contains representative states

Actual Desktop WebView screenshot automation should remain separate until the
window launch and screenshot capture are repeatable in local and CI contexts.

## Non-goals

- do not replace the command-line Desktop smoke example
- do not add Desktop screenshots to default release gates yet
- do not claim Mobile support from Desktop verification
- do not duplicate preview state data by hand if shared helpers can be reused

## M38 Result

M38 added:

- shared rendered preview panels in `examples/preview-states`
- Web preview reuse of the shared `PreviewSurface`
- Desktop preview binary at `examples/desktop-demo/src/bin/preview.rs`
- Desktop Tailwind CSS v4 source input at `examples/desktop-demo/assets/preview.css`
- structural Desktop gate at `scripts/desktop-preview-verify.mjs`

The default command-line Desktop smoke path remains:

```bash
cargo run -p dioxus-ui-desktop-demo
```

The rendered Desktop preview path is:

```bash
dx serve --package dioxus-ui-desktop-demo --bin preview --platform desktop
```

## Remaining Gaps

Desktop WebView screenshot capture is still planned. The current Desktop gate
proves source selectors, Tailwind source input, preview binary compilation, and
command-line smoke output. It does not open the Desktop WebView and capture
screenshots.

Mobile automation remains checklist-based until a repeatable device or emulator
command is selected.
