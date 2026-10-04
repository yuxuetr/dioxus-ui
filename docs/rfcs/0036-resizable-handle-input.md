# RFC 0036: Resizable Handle Input

- Status: Accepted
- Created: 2026-10-05

## Summary

Make the Resizable handle a working window splitter. `ResizableHandle` takes
the size of the panel before it as `value`, can take focus, reports arrow,
Home, and End keys and pointer drags as a size delta through `on_resize`,
and exposes `aria-valuenow` with the separator orientation ARIA expects. The
group, panels, and handle pass through attributes.

## Current State

As of M160:

- `ResizableHandle` renders `role="separator"` with no `tabindex`, no
  `aria-valuenow`, and no event handlers. Keyboard users cannot reach it, and
  neither keys nor drags change any size.
- `aria-orientation` repeats the group orientation. ARIA defines a
  separator's orientation as the direction of the line, so the handle between
  side-by-side panels must be `vertical`; it renders `horizontal`.
- `resizable_resize_pair(first, second, delta)` already moves a delta between
  two adjacent panels and respects both panels' limits, but nothing produces
  the delta.
- None of the parts accepts attributes, so a panel cannot take the `id` that
  `aria-controls` on the handle needs.

## Decision

### API

```rust
let mut panels = use_signal(|| {
  (ResizablePanelState::new(30.0, 20.0, 80.0), ResizablePanelState::new(70.0, 20.0, 80.0))
});

rsx! {
  ResizablePanelGroup {
    ResizablePanel { id: "files", size: panels().0.size, "Files" }
    ResizableHandle {
      "aria-controls": "files",
      "aria-label": "Resize files",
      value: panels().0.size,
      min: 20.0,
      max: 80.0,
      on_resize: move |delta| {
        let (first, second) = panels();
        panels.set(resizable_resize_pair(first, second, delta));
      },
    }
    ResizablePanel { size: panels().1.size, "Editor" }
  }
}
```

- `ResizableHandle` gains `value: f64` (default `50.0`), `min: f64`
  (default `0.0`), `max: f64` (default `100.0`), `step: f64` (default
  `10.0`), and `on_resize: Option<EventHandler<f64>>`. The delta is in
  percent of the group and is what `resizable_resize_pair` takes.
- It renders `tabindex="0"` unless disabled, `aria-valuenow`,
  `aria-valuemin`, and `aria-valuemax`. `aria-orientation` is `vertical` in
  a horizontal group and `horizontal` in a vertical group. `data-orientation`
  keeps the group orientation for styling.
- Keys: in a horizontal group ArrowRight reports `+step` and ArrowLeft
  `-step`; in a vertical group ArrowDown and ArrowUp do. Home reports
  `min - value` and End `max - value`. A handled key calls `prevent_default`.
  A delta of zero is not reported.
- Pointer drags run in a page script for the handle's lifetime, like the
  Slider. A primary press captures the pointer and focuses the handle. Each
  move sends the target size `start value + pointer offset / group size *
  100`, measured on the handle's parent along the group orientation, and Rust
  reports `target - value` against the latest props. Sending a target instead
  of a per-move delta keeps the panel edge under the pointer after a limit
  stops the resize.
- A disabled handle reports nothing.
- `ResizablePanelGroup` and `ResizablePanel` extend `GlobalAttributes` and
  `div`, and `ResizableHandle` extends them too. The spread comes after the
  explicit attributes.
- The handle must be a direct child of the group.

## Scope

In scope:

- the handle props, keys, pointer script, ARIA values and orientation, and
  attribute spreading in the crate and the template
- the Resizable docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Enter to collapse and restore a panel | APG lists it as optional; `ResizablePanel` `collapsed` already hides a panel the app chooses | A consumer asks for keyboard collapse |
| Right-to-left horizontal groups | The pointer offset and arrow keys would need to flip with the computed direction | A consumer ships a right-to-left Resizable |
| Pixel-based limits and persisted layouts | Sizes stay in percent, and the app owns storage | A consumer needs a fixed-width panel |
| Desktop and Mobile self-test scenarios | The script uses the same pointer capture as the Slider, which already runs in the WebViews | A WebView reports different pointer behavior |

## Verification

- Unit tests cover the key mapping for both orientations, the Home and End
  deltas, and the separator orientation.
- The CLI parity test keeps the template pointer script identical to the
  crate script, and the generated fixture smoke keeps the template compiling.
- The Web preview renders a two-panel horizontal group, and
  `npm run verify:runtime-interactions` asserts:
  - the handle takes focus, has `aria-valuenow`, and is a vertical
    separator;
  - ArrowRight, ArrowLeft, Home, and End resize the panels within their
    limits;
  - a drag moves the panel edge with the pointer and stops at the limit;
  - passed attributes such as `aria-controls` and a panel `id` render.
- Reverse checks: removing the key handler, not starting the pointer script,
  sending per-move deltas instead of targets, keeping the group orientation
  in `aria-orientation`, or not spreading the attributes each make the
  verifier fail.
