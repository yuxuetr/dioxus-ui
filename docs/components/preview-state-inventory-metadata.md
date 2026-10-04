# Preview State Inventory Metadata Gate

This document defines the M91 plan for keeping the shared rendered-preview state
inventory aligned with Web and Desktop preview verification.

Status: Planned in M91.1.

## Problem

The repository already has rendered preview shells for Web and Desktop plus a
shared `examples/preview-states` crate. Existing preview checks prove that
representative source fragments, CSS inputs, and command-line smoke output are
present, but the state inventory contract is duplicated across preview source,
verification scripts, package aliases, and documentation.

M91 should add a deterministic metadata gate before expanding browser-rendered
screenshot assertions. The gate should make preview drift obvious without
launching browsers, compiling Tailwind output, or scoring visual parity.

## Contract

The preview state inventory metadata gate should verify:

- `examples/preview-states/src/lib.rs` exposes `PreviewTarget::Web` and
  `PreviewTarget::Desktop`. M143 adds `PreviewTarget::Mobile` for the iOS
  Simulator preview in `examples/mobile-demo`.
- `PreviewSurface` remains the shared rendered preview entry point.
- Web and Desktop preview binaries call `PreviewSurface` with the matching
  target.
- Web and Desktop preview CSS inputs keep Tailwind CSS v4 source syntax.
- The structural preview verifiers check the same representative state labels.
- Package scripts keep focused `verify:web-preview`, `verify:desktop-preview`,
  aggregate `verify:preview`, and release wiring discoverable.

Required representative state labels:

```text
button group class
input group class
input otp helper
attachment class
bubble class
message class
message scroller helper
marker class
chart helper
direction class/attr
collapsible class
```

Required rendered preview panels:

```text
overview
actions
mobile-profile
form
message
chart
overlay-open
inventory
```

## Non-goals

- no Playwright or browser launch
- no `dx serve`
- no Desktop WebView window automation
- no Tailwind CSS compilation or output artifact checks
- no screenshot pixel comparison
- no visual parity scoring against shadcn/ui, HeroUI, or Tailwind Plus

## Planned Command

M91 should add:

```bash
npm run verify:preview-state-metadata
```

The command should be read-only and should fail with actionable missing-fragment
messages when preview inventory metadata drifts.

After it is stable, it should be included in:

```bash
npm run verify:release
```

It may also be included in `npm run verify:preview` if doing so does not
duplicate expensive Rust compilation or browser work.
