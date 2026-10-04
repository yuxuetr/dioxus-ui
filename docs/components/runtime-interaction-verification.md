# Runtime Interaction Verification

This document defines the M111 plan for adding the first browser-backed
interaction checks on top of the rendered Web preview.

Status: Fixture targets implemented in M111.2; browser verifier implemented in
M111.3; package alias and documentation wiring implemented in M111.4.

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
| Overlay | Dialog, Popover, Dropdown, Select-style open state | trigger opens content, close control hides it, dialog role remains present |
| Selection | Checkbox, Switch, Radio Group, Toggle, Tabs | click or keyboard changes checked/selected/pressed attributes |
| Keyboard | Command, menus, roving-focus components | keyboard activation changes active descendant or active option marker |
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

The M111 implementation covers five generic fixture targets:

- `disclosure`
- `overlay`
- `selection`
- `keyboard`
- `scroll-status`

It intentionally verifies representative state transitions instead of every
public component prop combination.

For a serial local run of all browser-backed preview checks, use
`npm run verify:browser-local`. Do not parallelize browser preview commands;
each command starts its own `dx serve` process.

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

## M135 Component Overlay Fixtures

M135 adds fixtures that render real `dioxus-ui` components instead of generic
controls, so the verifier exercises the shipped overlay behavior from
[RFC 0010](../rfcs/0010-overlay-interaction-behavior.md):

| Target | Component | Browser Assertion |
| --- | --- | --- |
| `dialog` | Dialog | opening focuses the first input; Tab and Shift+Tab wrap; Escape, overlay click, and Close each close it and restore focus to the trigger |
| `alert-dialog` | Alert Dialog | opening focuses Cancel; Tab wraps; Escape closes without confirming; Action runs its handler, closes, and restores focus |
| `popover` | Popover | anchored content sits below the trigger inside the viewport; Escape and an outside click close it; near the viewport bottom it flips above the trigger; the trigger still toggles it |
| `tooltip` | Tooltip | anchored content sits above the trigger; an outside press does not close it; Escape does |
| `toast` | Toast | the viewport is a polite `Notifications` region; the toast closes with reason `timeout` after its countdown, stays open while hovered past its duration, and reports `action` and `close` reasons |
| `sonner` | Sonner | the viewport is a polite region; a mounted toast unmounts with reason `timeout` and reports `close` |
| `select` | Select | the listbox sits below the trigger while focus stays on it; the selected option starts highlighted; arrows skip the disabled option without wrapping; Home, End, and typeahead move the highlight; Enter, Space, and click choose and close; a disabled option cannot be chosen; ArrowDown reopens; Escape and an outside click close it |
| `combobox` | Combobox | typing opens the listbox below the input with no highlight; arrows highlight options; filtering keeps a still-matching highlight and clears a removed one; Enter and click choose, fill the input, and keep focus in it; ArrowDown reopens and Escape closes |

The Web preview serves the Tailwind input stylesheet without compiling it, so
the Dialog overlay has no `fixed inset-0` box. The verifier dispatches the
overlay click on the element instead of clicking a screen position. Placement
uses inline fixed coordinates and does not depend on Tailwind.

Reverse checks run during M135: removing the Tab wrap listener, ignoring
Escape in Dialog content, disabling the flip, and giving Tooltip the popover
dismissal default each make the verifier fail. M136 adds two more: a countdown
that ignores hover and a countdown that never starts.

M136 adds the Toast and Sonner rows from
[RFC 0011](../rfcs/0011-toast-timer-and-live-region.md).

M137 adds the Select and Combobox rows from
[RFC 0012](../rfcs/0012-listbox-overlay-behavior.md). Visibility comes from the
Rust render while the page scripts attach a frame later, so the verifier waits
for anchored content to become `position: fixed` before pressing keys or
clicking outside. Removing arrow navigation, typeahead, disabled option
skipping, or the filter highlight reset each make the verifier fail.

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

1. Expand interaction coverage across more runtime-sensitive components. M135
   covers Dialog, Alert Dialog, Popover, and Tooltip; M136 adds Toast and
   Sonner; M137 adds Select and Combobox.
2. Add targeted screenshot smoke for a small set of stable panels.
3. Decide whether any browser command should move into CI after local
   reliability is proven.
