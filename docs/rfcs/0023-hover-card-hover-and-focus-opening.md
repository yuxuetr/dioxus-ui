# RFC 0023: Hover Card Hover And Focus Opening

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Hover Card a root and a link trigger. The tooltip script (RFC 0022)
moves into a shared hover-open module, and Hover Card uses it with:

- a longer close delay;
- trigger presses that keep the card open;
- no `aria-describedby`.

Focus may move from the trigger into the card without closing it.

## Current State

As of M147:

- Hover Card has only `HoverCardContent` and its header, title, and
  description parts. Content handles anchored placement and popover dismissal
  (Escape, outside press, focus outside).
- The app puts hover and focus handlers on its own trigger and sets the
  content's `anchor_id` itself.
- There is no open delay, so a pointer crossing the trigger flashes the card.
- There is no close delay, so moving the pointer from the trigger onto the
  card closes it. The card can hold links, so the pointer needs to reach it.
- The Tooltip script already handles hover delay, content hover, and focus,
  but it lives in the Tooltip module. It closes on trigger presses and on any
  blur of the trigger, and it sets `aria-describedby`, all of which suit a
  tooltip but not a card.
- `docs/components/accessibility.md` lists "Needs hover/focus timing and
  mobile fallback verification" as Planned for Hover Card.
- RFC 0022 deferred Hover Card timing until Hover Card opening was designed,
  and said the script would then move to a shared module.

## Decision

### State

State stays controlled, as with Tooltip. The app keeps `open`, passes it to
`HoverCardContent`, and handles `HoverCard` `on_open_change(bool)`:

```rust
let mut open = use_signal(|| false);

rsx! {
  HoverCard {
    on_open_change: move |next| open.set(next),
    HoverCardTrigger { href: "/users/dioxus", "@dioxus" }
    HoverCardContent { open: open(), "Fullstack app framework for Rust." }
  }
}
```

Inside `HoverCard`, `HoverCardContent` uses the trigger's id when it has no
`anchor_id`, and the root's handler for dismissal when it has no
`on_open_change` of its own.

### Components

- `HoverCard` renders a `div` with `display: contents` and runs the shared
  hover-open script. `open_delay_ms` defaults to 700 ms and `close_delay_ms`
  to 300 ms, the Radix defaults.
- `HoverCardTrigger` renders an `a` with a required `href`, as the Radix
  trigger does. A link is focusable and keeps its own click behavior.
- `HoverCardContent` is unchanged outside `HoverCard`.
- `Tooltip` keeps its props and behavior. It uses the shared script with a
  100 ms close delay, press closing, and `aria-describedby`.

### Shared Hover-Open Script

The script moves from the Tooltip module and template to the shared
`hover_open` module and the template `utils.rs`. It receives the open delay,
the close delay, whether a trigger press closes, and whether the trigger is
described by the content. Parts are marked with generic attributes:

- `data-dxui-hover-open` on the root;
- `data-dxui-hover-trigger` on the trigger;
- `data-dxui-hover-content` on the content.

Each root only reads the parts whose closest root it is, so a tooltip inside
a hover card belongs to the tooltip.

| Event | Tooltip | Hover Card |
| --- | --- | --- |
| Pointer enters the trigger | Open after the open delay | Same |
| Pointer enters the content | Cancel a pending close | Same |
| Pointer leaves the trigger or content | Close after 100 ms unless the pointer enters the other one | Close after `close_delay_ms` |
| Keyboard focus on the trigger | Open at once | Same |
| Focus leaves the trigger and content | Close at once, unless the pointer is over them | Same |
| Focus moves between the trigger and content | Stay open | Same |
| Pointer press on the trigger | Close; stay closed while the pointer rests | Stay open |
| `aria-describedby` on the trigger | While open | Never |
| Escape | Close (unchanged) | Close (unchanged) |
| Outside press | Ignored (unchanged) | Close (unchanged) |

- Moving focus between the trigger and content is new for both. Tooltip
  content has nothing focusable, so Tooltip behavior does not change.
- While the pointer is over the trigger or content, blur leaves closing to
  pointer leave. Without this, pressing the card's text after focusing the
  trigger moves focus to the body and closes the card under the pointer. For
  Tooltip, this changes only Tab away while the pointer rests on the trigger.
- The card is not named in the trigger's description. Its content is rich
  and often long, and Radix leaves the trigger without ARIA links.
- Touch pointers are ignored for hover in both, as before.

The CLI parity test compares the shared script in the crate and template
`utils.rs` instead of the Tooltip copies.

## Scope

In scope:

- the shared hover-open module, with Tooltip moved onto it
- the `HoverCard` root, link `HoverCardTrigger`, and open reporting
- open and close delays, content hover and focus, and press behavior
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Skipping the delay between adjacent cards or tooltips | Needs a provider shared across roots | A consumer reports slow lists of cards or tooltips |
| Touch opening | Touch pointers are ignored; shadcn/ui recommends Popover or Sheet on mobile | Mobile hover card behavior is designed |
| A trigger other than a link | Dioxus has no `asChild`; the app can still wire its own element without `HoverCard` | A consumer needs a card on a button or other element |
| A name or role change for the content | `role="dialog"` predates this RFC and is not part of the timing | An accessibility review of Hover Card content |
| Desktop and Mobile self-test scenarios | The script uses only pointer, focus, and timers, as RFC 0022 found for Tooltip | The script relies on behavior that differs between WebViews |

## Verification

- The CLI parity test keeps the template script identical to the crate
  script.
- The Web preview renders a real Hover Card, and
  `npm run verify:runtime-interactions` asserts:
  - the card stays closed during the open delay and then opens;
  - the card stays open while the pointer crosses onto it, stays open for
    part of the close delay after the pointer leaves, and then closes;
  - a press on the trigger keeps it open, and a press on the card's text
    after that keeps it open;
  - keyboard focus opens it, and Tab into the card keeps it open;
  - Escape and outside presses close it;
  - the trigger never has `aria-describedby`;
  - the tooltip assertions from RFC 0022 still pass.
