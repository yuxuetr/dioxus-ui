# RFC 0030: Slider Keyboard And Pointer Input

- Status: Accepted
- Created: 2026-10-04

## Summary

Make `Slider` respond to the keyboard and the pointer. Arrow, Page Up,
Page Down, Home, and End keys move the value. A press or drag on the slider
sets it. Either way the new value goes to `on_value_change`.

## Current State

As of M154:

- `Slider` renders a focusable `div role="slider"` with `aria-valuemin`,
  `aria-valuemax`, and `aria-valuenow`, but handles no events and exposes no
  callback. Keys and the mouse do nothing.
- The primitives crate has `SliderState::moved(SliderKeyMove)`, which already
  steps, pages, clamps, and snaps, but nothing calls it.
- The source-copy template has its own `SliderState` without `page_step`,
  `moved`, or `SliderKeyMove`.
- The docs page tells apps to wire keys themselves.
- WAI-ARIA APG: Right and Up increase by one step, Left and Down decrease,
  Page Up and Page Down move by a larger step, and Home and End jump to the
  minimum and maximum.

## Decision

### API

```rust
let mut volume = use_signal(|| 40.0);

rsx! {
  Label { id: "volume-label", "Volume" }
  Slider {
    "aria-labelledby": "volume-label",
    value: volume(),
    on_value_change: move |value| volume.set(value),
  }
}
```

- `on_value_change: Option<EventHandler<f64>>` receives the new value. It is
  called only when the value changes, so a key at either end calls nothing.
- `Slider` stays controlled: the app passes the value back as `value`.
- `attributes` extends `GlobalAttributes` and `div` and is spread on the
  root, so apps can pass `aria-label`, `aria-labelledby`, and
  `aria-valuetext`.
- A disabled slider ignores keys and the pointer.

### Keyboard

| Key | Move |
| --- | --- |
| ArrowRight, ArrowUp | `Increment` |
| ArrowLeft, ArrowDown | `Decrement` |
| PageUp | `PageIncrement` (ten steps) |
| PageDown | `PageDecrement` |
| Home | `Home` (minimum) |
| End | `End` (maximum) |

`slider_key_move(key) -> Option<SliderKeyMove>` maps the key name.
`onkeydown` runs in Rust with the current props, calls `SliderState::moved`,
and prevents the default action for handled keys so the page does not scroll.
The template gains the same mapping and `moved`.

### Pointer

A page script, started for the slider's lifetime like the roving group
script, listens on the root:

- A primary-button `pointerdown` captures the pointer, focuses the slider,
  and sets the value at the pointer.
- `pointermove` while captured keeps setting it, and `pointerup` or
  `pointercancel` ends the drag.
- The value is `min + ratio * (max - min)`, snapped to `step` and clamped,
  where `ratio` comes from the pointer's x position in the root's box. The
  script reads `aria-valuemin`, `aria-valuemax`, and `data-step` at event
  time, so prop changes apply without restarting it.
- It sends the value only when it differs from `aria-valuenow`, and nothing
  when `aria-disabled="true"`.

Rust passes the value through `SliderState` before calling the handler, so
script rounding cannot leave the step grid.

## Scope

In scope:

- the key mapping, the callback, the pointer script, and attribute spreading
  in the crate and the templates
- the Slider docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Right-to-left sliders | The range and thumb are placed with `left`, so the slider fills left to right in every direction; keys and pointer match that | A consumer renders a Slider right to left |
| Vertical sliders | `aria-orientation` is always horizontal and the styles have no vertical layout | A consumer needs a vertical Slider |
| Multiple thumbs | A range slider needs a value pair and per-thumb focus | A consumer needs a range |
| A configurable page step | Ten steps matches the primitive's default | A consumer needs a different page step |
| Desktop and Mobile self-test scenarios | Pointer capture and key events behave the same in the WebViews, and other scripts already run there | A WebView reports different pointer behavior |

## Verification

- Unit tests cover the key mapping in the crate.
- The CLI parity test keeps the template pointer script identical to the
  crate script, and the generated fixture smoke keeps the template
  compiling.
- The Web preview renders a labelled Slider and a disabled one, and
  `npm run verify:runtime-interactions` asserts:
  - each key's movement and clamping at both ends;
  - the keys do not scroll the page;
  - a press and a drag set snapped values;
  - the disabled slider does not change.
- Reverse checks: swapping the key directions, paging by a single step, not
  starting the pointer script, or removing the disabled guard each make the
  verifier fail.
