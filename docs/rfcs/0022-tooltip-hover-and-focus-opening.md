# RFC 0022: Tooltip Hover And Focus Opening

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Tooltip a root and a trigger part, plus a page script that:

- opens the tooltip after a hover delay;
- opens it at once on keyboard focus;
- keeps it open while the pointer is over the trigger or the content;
- closes it on pointer leave, blur, and trigger presses.

The trigger points to the content with `aria-describedby` while the tooltip
is open.

## Current State

As of M146:

- Tooltip has only `TooltipContent`, which handles anchored placement and
  Escape (RFC 0010).
- The app puts `onmouseenter`, `onmouseleave`, `onfocus`, and `onblur` on its
  own trigger. The Web preview fixture does exactly this.
- There is no open delay, so a pointer crossing the trigger flashes the
  tooltip.
- Moving the pointer from the trigger onto the content fires the trigger's
  `mouseleave` and closes the tooltip. WCAG 1.4.13 asks for hover content that
  the pointer can move onto.
- A click focuses the trigger, so a focus handler reopens the tooltip right
  after the press.
- Nothing links the trigger to the content; each app adds
  `aria-describedby` itself.
- `docs/components/accessibility.md` lists "Hover and focus opening remain
  app-owned" as Planned.

## Decision

### State

State stays controlled. The app keeps `open`, passes it to `TooltipContent`,
and handles `Tooltip` `on_open_change(bool)`:

```rust
let mut open = use_signal(|| false);

rsx! {
  Tooltip {
    on_open_change: move |next| open.set(next),
    TooltipTrigger { "Save" }
    TooltipContent { open: open(), "Saved 2 minutes ago" }
  }
}
```

The root does not take `open`; the script reads the content's
`data-state` from the DOM. Inside `Tooltip`, `TooltipContent` uses the root's
handler for Escape when it has no `on_open_change` of its own, and it uses
the trigger's id when it has no `anchor_id`.

### Components

- `Tooltip` renders a `div` with `display: contents`, so it adds no box
  around the trigger. It provides a base id and `on_open_change` through
  context, and runs the tooltip script. `delay_ms` sets the hover open delay
  and defaults to 700 ms, the Radix default.
- `TooltipTrigger` renders a `button` with an id built from the base id. It
  renders `aria-describedby` pointing to the content only while the content is
  open. It does not read the open state; the script sets and removes the
  attribute when it sees the content's `data-state` change.
- `TooltipContent` renders an id built from the base id when inside
  `Tooltip`, so the trigger can refer to it.
- Without `Tooltip`, the parts work as today: the app wires the trigger.

### Tooltip Script

The script runs for the root's lifetime and finds the trigger
(`data-dxui-tooltip-trigger`) and content (`role="tooltip"`) inside the root
on every event. It sends `"open"` or `"close"`, and only when the request
changes the current state.

| Event | Behavior |
| --- | --- |
| Pointer enters the trigger | Open after `delay_ms` |
| Pointer enters the content | Cancel a pending close |
| Pointer leaves the trigger or content | Cancel a pending open; close after 100 ms unless the pointer enters the other one |
| Keyboard focus on the trigger | Open at once |
| Focus leaves the trigger | Close at once |
| Pointer press on the trigger | Close at once; hover does not reopen until the pointer leaves the trigger, and the press's focus does not open |
| Escape | Close, through `TooltipContent` dismissal (unchanged) |

- The 100 ms grace period covers the `side_offset` gap between the trigger
  and the content.
- Touch pointers are ignored for hover, so a tap does not open the tooltip
  after the delay.
- The script tracks pointer presses instead of reading `:focus-visible`.
  Programmatic focus after a mouse interaction is not `:focus-visible` in
  Chromium, but it should still open the tooltip.

The script lives in the Tooltip module and in the Tooltip template. The CLI
parity test keeps the two copies identical. Tooltip is the only component
that uses it.

## Scope

In scope:

- the `Tooltip` root, `TooltipTrigger`, and open reporting
- hover delay, content hover, focus opening, and press closing
- `aria-describedby` while open
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Hover Card hover and focus timing (done in M148, see RFC 0023) | Hover Card needs a longer close delay and different focus rules; sharing one script now would design for a second consumer that is not built | Hover Card opening is designed; the script then moves to a shared module |
| Skipping the delay between adjacent tooltips | Needs a provider shared across tooltips | A consumer reports slow toolbar tooltips |
| Touch long press | Touch pointers are ignored; mobile apps show the text another way | Mobile tooltip behavior is designed |
| A trigger other than a `button` | Dioxus has no `asChild`; the app can still wire its own element without `Tooltip` | A consumer needs a tooltip on a link or input |
| Desktop and Mobile self-test scenarios | Hover is not available on Mobile, and the script uses only pointer, focus, and timers | The script relies on behavior that differs between WebViews |

## Verification

- The CLI parity test keeps the template script identical to the crate
  script.
- The Web preview tooltip uses the new parts, and
  `npm run verify:runtime-interactions` asserts:
  - the tooltip stays closed during the hover delay and then opens;
  - `aria-describedby` names the content only while it is open;
  - the tooltip stays open when the pointer moves onto the content and closes
    when it leaves;
  - keyboard focus opens it before the hover delay would;
  - blur and Escape close it, and outside presses do not;
  - a press on the trigger closes it, and it stays closed while the pointer
    rests on the trigger.
