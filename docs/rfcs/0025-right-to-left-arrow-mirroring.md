# RFC 0025: Right-To-Left Arrow Mirroring

- Status: Accepted
- Created: 2026-10-04

## Summary

In a right-to-left layout, ArrowLeft moves to the next item and ArrowRight to
the previous one. This applies to the page scripts that move focus with Left
and Right:

- the roving group script used by Tabs, Radio Group, and Toggle Group;
- the Menubar script;
- the Navigation Menu script.

Each script reads the direction from the root's computed style on every key
press, so no component gains a prop.

## Current State

As of M149:

- The roving group script (RFC 0019) maps ArrowRight to the next item and
  ArrowLeft to the previous one for horizontal and two-axis groups.
- The Menubar script (RFC 0015) moves between triggers with ArrowRight and
  ArrowLeft, both on a trigger and inside an open menu.
- The Navigation Menu script (RFC 0016) moves between top-level items with
  ArrowRight and ArrowLeft.
- None of them reads the text direction. Inside `Direction` with
  `TextDirection::Rtl`, items are laid out right to left, so ArrowLeft moves
  focus to the right, away from the key pressed.
- `docs/components/accessibility.md` lists right-to-left arrow mirroring as
  Planned for Tabs, Radio Group, Toggle Group, and Menubar.
- WAI-ARIA APG and Radix both swap Left and Right in right-to-left layouts
  and leave Up, Down, Home, and End alone.

## Decision

### Reading The Direction

Each script calls `getComputedStyle(root).direction` when a key is pressed.
The computed value follows the nearest `dir` attribute, including the one
`Direction` renders, the document's `dir`, and CSS `direction`. Reading it on
each key press means a direction change after mount applies at once.

A prop or context was considered. It would duplicate what the browser
already resolves, and the app would have to keep it in step with `dir`.

### Keys

| Key | Left to right | Right to left |
| --- | --- | --- |
| ArrowRight | Next item | Previous item |
| ArrowLeft | Previous item | Next item |
| ArrowDown, ArrowUp | Unchanged | Unchanged |
| Home, End | First, last item | First, last item |

- In the roving group, the swap applies before the orientation picks the
  keys. A vertical group still ignores Left and Right. A two-axis group, such
  as Radio Group, keeps Down as next and Up as previous.
- Looping and stopping at the ends do not change.
- In Menubar, the swap applies both on a trigger and inside an open menu, so
  ArrowLeft opens the menu to the visual left.
- In Navigation Menu, the swap applies to top-level items. Up and Down inside
  content do not change.

## Scope

In scope:

- mirroring in the roving group, Menubar, and Navigation Menu scripts, in the
  crate and the templates
- docs for the affected components and Direction
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Calendar grid keys | `calendar_key_move` is a Rust helper the app calls; mirroring needs the direction in Rust, which is a separate API change | A consumer renders Calendar right to left |
| A direction prop or context | The computed style already carries the direction | A renderer reports a computed `direction` that differs from `dir` |
| Vertical orientation for Tabs | A separate feature, listed in the Tabs Planned row | Vertical tabs are designed |
| Desktop and Mobile self-test scenarios | `getComputedStyle` and `dir` behave the same in the WebViews, and the scripts already run there | A WebView reports a different computed direction |

## Verification

- The CLI parity test keeps each template script identical to the crate
  script.
- `npm run verify:runtime-interactions` sets `dir="rtl"` on the Tabs, Radio
  Group, Toggle Group, Menubar, and Navigation Menu fixtures and asserts:
  - ArrowLeft moves to the next item and ArrowRight to the previous one;
  - Radio Group's Down still moves to the next item;
  - Menubar's ArrowLeft inside an open menu opens the next menu;
  - after removing `dir`, ArrowRight moves to the next item again.
- Reverse checks: removing the swap from each script, or also swapping Up and
  Down, makes the verifier fail.
