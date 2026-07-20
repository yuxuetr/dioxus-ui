# Runtime Interaction Verification

This document defines the M111 plan for adding the first browser-backed
interaction checks on top of the rendered Web preview.

Status: Planned in M111.1.

## Problem

M110 proves that every public component preview target exists in a browser DOM,
is visible, contains text, and has a non-empty layout box. That still does not
prove representative component interactions work.

The highest-risk components are runtime-sensitive overlays, disclosure
patterns, selection controls, keyboard-visible states, and components that rely
on focus or ARIA state. Those should get a small, deterministic interaction
fixture before any screenshot or parity claim.

## Decision

M111 should add a focused Web preview interaction layer:

1. Add lightweight interaction fixture targets to `examples/preview-states`.
2. Preserve all existing `data-component-preview`, `data-preview-panel`, and
   rendered DOM verification targets.
3. Add an opt-in Playwright verifier that starts the Web preview and exercises
   the fixture targets.
4. Keep visual diffing, screenshot artifacts, runtime adapter graduation, and
   component API changes out of scope.

This is not the same as the Web runtime adapter harness. The runtime harness
verifies primitive runtime families such as focus, portal, timer, live-region,
measurement, pointer, gesture, and scroll command contracts. M111 verifies that
the rendered component preview can expose representative user-level
interactions in the browser.

## First Interaction Set

The first fixture should cover a small, stable set:

| Interaction | Representative Risk | Expected Browser Assertion |
| --- | --- | --- |
| Disclosure | Accordion, Collapsible, Sidebar-style state | click toggles `aria-expanded`, `data-state`, and visible content |
| Overlay | Dialog, Popover, Dropdown, Select-style open state | trigger opens content, Escape closes it, focus remains deterministic |
| Selection | Checkbox, Switch, Radio Group, Toggle, Tabs | click or keyboard changes checked/selected/pressed attributes |
| Keyboard | Command, menus, roving-focus components | Arrow key changes active descendant or active option marker |
| Scroll status | Message Scroller and Scroll Area | jump button or status target exposes deterministic state without mutating app data |

The first implementation can use generic fixture controls rather than every
public component. The goal is to prove interaction verification infrastructure
and representative browser behavior, not to create a full prop matrix.

## Verification Contract

`npm run verify:runtime-interactions` should:

- start the Web preview with `dx serve`
- use Playwright-managed Chromium by default
- support `DIOXUS_UI_BROWSER_EXECUTABLE`
- wait for stable fixture root selectors
- click and keyboard-drive each fixture target
- assert state through DOM attributes, visible text, active element, and
  deterministic status regions
- clean up the preview server before exit

The command should be opt-in and outside `npm run verify` and
`npm run verify:release` until browser availability and fixture behavior are
stable.

## Non-goals

- no screenshots by default
- no visual diffing or shadcn/ui visual parity claims
- no full accessibility certification
- no complete overlay focus trap certification
- no native Desktop WebView automation
- no native Mobile automation
- no component API changes
- no generated source-copy template rewrites
- no provider/domain behavior such as uploads, charts backed by external
  libraries, markdown parsing, async data, or network state

## Documentation Alignment

M111 should keep these files aligned:

- `package.json`
- `README.md`
- `docs/README.md`
- `docs/release.md`
- `docs/quality-gates.md`
- `docs/site.md`
- `docs/components/README.md`
- `docs/components/runtime-interaction-verification.md`
- `docs/components/browser-dom-component-verification.md`
- `docs/components/runtime-renderer-verification.md`
- `docs/browser-artifact-policy-metadata.md`

The docs should say clearly that M111 is stronger than DOM existence checks but
weaker than screenshot parity, full accessibility certification, or runtime
adapter graduation.

## Follow-up Milestones

After M111, useful follow-up work is:

1. Expand interaction coverage across more runtime-sensitive components.
2. Add targeted screenshot smoke for a small set of stable panels.
3. Decide whether any browser command should move into CI after local
   reliability is proven.
