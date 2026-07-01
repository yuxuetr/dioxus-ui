# Desktop WebView Preview Follow-through

This document records the M37.5 follow-through plan after the rendered Web
preview and Web screenshot gate landed.

Status: Desktop preview implemented in M38. Desktop WebView screenshot capture
was probed in M39 and remains unsupported until native window launch is
repeatable.

## Goal

Add a Desktop WebView preview path only after the Web preview remains stable.
The Desktop path should prove that representative component panels can render in
a WebView without replacing the command-line Desktop smoke example.

## Minimum Desktop Path

Keep the current command-line Desktop demo:

```bash
cargo run -p dioxus-ui-desktop-demo
```

The future rendered Desktop preview should be a separate binary or explicit
serve target:

```bash
dx serve --package dioxus-ui-desktop-demo --bin preview --platform desktop
```

If Dioxus requires platform-specific features in the example crate, add them
only to the preview binary path and keep command-line smoke output deterministic.

## Required Desktop Panels

Reuse `examples/preview-states` and mirror the Web preview panels:

- form panel with Input Group and Input OTP states
- message panel with user, assistant, attachment, marker, and scroller states
- chart panel with SVG plot, legend labels, and fallback table
- open overlay panel rendered through the Desktop-supported portal strategy or
  an explicitly documented inline fallback

The Desktop preview should use the same stable `data-preview-*` markers as the
Web preview so screenshot tooling can share selectors.

## Desktop Screenshot Gate

The supported Desktop preview gate is currently structural:

```bash
node scripts/desktop-preview-verify.mjs
```

It asserts:

- preview starts without replacing `cargo run -p dioxus-ui-desktop-demo`
- form, message, chart, and overlay-open panel selectors stay present in source
- unsupported WebView behavior is visible instead of silently omitted

M39 did not add `scripts/desktop-webview-screenshot-smoke.sh` because the
native Desktop preview window is not repeatable in the current local
environment. Do not add the Desktop screenshot command to default release gates
until the preview can launch consistently and be selected by a stable title,
process name, or window id.

## Mobile Follow-through

Mobile remains checklist-based. The next Mobile step is not a component fork; it
is choosing one repeatable target command.

Keep these gaps visible:

- no selected device or emulator command
- no automated safe-area assertion
- no visual viewport assertion for browser chrome or software keyboard changes
- no native-scroll gesture assertion
- no mobile assistive technology assertion

Mobile component behavior should remain controlled and app-owned until those
checks can run repeatably.

## Quality Gate Updates

After a Desktop preview exists, update:

- `examples/README.md` with the explicit Desktop preview command
- `docs/quality-gates.md` with the Desktop preview gate
- `docs/release.md` with Desktop screenshot review notes
- `docs/components/runtime-desktop-mobile-verification.md` with the selected
  command and remaining target limitations
