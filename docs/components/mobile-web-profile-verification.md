# Mobile Web Profile Verification

This document defines the M40 plan for adding a repeatable Mobile Web profile
verification path without claiming native Dioxus Mobile support.

Status: Planned in M40.1. Structural gate added in M40.2.

## Goal

Add a deterministic verification layer for the rendered Web preview when it is
viewed as a mobile-width web surface. This should cover the mobile checklist
items that can be checked from source, static preview markup, or browser-sized
Web rendering before this project has a selected device, simulator, or native
Mobile command.

## Boundary

Mobile Web profile verification is not the same as native Mobile support:

- it runs against the existing Web preview surface
- it can validate mobile viewport intent, responsive layout markers, visible
  status text, reduced-motion policy markers, and hover alternatives
- it cannot verify native safe-area insets, software keyboard behavior,
  assistive technology behavior, or platform WebView gesture arbitration
- it must not add separate `Mobile` components or fork component APIs

The result should be a stronger local gate than a checklist, but weaker than an
emulator-backed or device-backed Mobile runtime gate.

## First Repeatable Checks

M40 should start with checks that are stable in the repository:

| Area | Repeatable M40 Check | Still Deferred |
| --- | --- | --- |
| Viewport sizing | Web preview has a documented mobile viewport target of `390x844`. | Browser chrome and orientation changes. |
| Touch targets | Preview source exposes touch-profile markers or classes for representative interactive controls. | Physical tap accuracy and platform accessibility target metrics. |
| Hover absence | Hover-sensitive components expose click, focus, or inline fallback states in docs or preview inventory. | Real hover/touch hybrid hardware behavior. |
| Safe areas | Overlay and notification docs mention app shell padding or inline fallback ownership. | Native safe-area inset measurement. |
| Reduced motion | Runtime and preview docs keep animation policy app-owned and visible. | Device-level reduced-motion setting integration. |
| Visible status | Preview inventory includes visible text for runtime-sensitive states. | Screen-reader announcement behavior. |

The first script should be structural. It should prove that the repository
continues to expose the selected Mobile Web profile markers and documented
viewport target, without opening a native Mobile target.

M40.2 added the shared preview panel:

```text
[data-preview-panel="mobile-profile"]
[data-mobile-profile="touch-targets"]
[data-mobile-profile="hover-alternative"]
[data-mobile-profile="safe-area-owned"]
[data-mobile-profile="reduced-motion"]
[data-mobile-profile="visible-status"]
```

## Candidate Gate

Add a local deterministic script:

```bash
node scripts/mobile-web-profile-verify.mjs
```

The script should read repository files and assert:

- the Web screenshot verification document keeps the `390x844` viewport target
- the rendered Web preview uses shared `PreviewSurface` panels rather than a
  forked Mobile tree
- the shared preview state source contains mobile-profile markers for touch
  targets, hover alternatives, safe-area ownership, reduced motion, and visible
  status
- the Mobile checklist still describes emulator/device-only items as deferred

This gate should complement `node scripts/web-preview-verify.mjs`; it should
not replace browser screenshot review or native Mobile testing.

## Out Of Scope

- no native iOS or Android automation
- no Dioxus Mobile workspace target
- no emulator/device farm integration
- no screen-reader support claim from source checks
- no component API fork such as `ButtonMobile`
- no default runtime adapter promotion

## Graduation Criteria

Mobile verification can move beyond the M40 Mobile Web profile when:

- a repeatable device, simulator, emulator, or browser automation command is
  selected
- touch, safe-area, visual viewport, native scroll, reduced motion, and visible
  status checks emit deterministic output
- unsupported and cancel fallback states are asserted
- source-copy components remain runtime-free by default

Until then, Mobile remains a target profile and integration policy, not a
separate component implementation track.
