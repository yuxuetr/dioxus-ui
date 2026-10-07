# Tooltip

Tooltip provides a root, a trigger, primitive configuration, and a styled
content part for short supplemental descriptions.

## Source Copy

```bash
dxui add tooltip
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["tooltip"] }
```

## API Surface

- `Tooltip`
- `TooltipTrigger`
- `TooltipContent`
- `TooltipPrimitiveConfig`
- `TooltipDismissBehavior`, `TooltipSide`, `TooltipAlign`
- `tooltip_content_class`

## Behavior

`Tooltip` owns whether the tooltip is open and its parts read it, so they
must sit inside it (see
[RFC 0077](../rfcs/0077-component-owned-state.md)):

```rust
rsx! {
  Tooltip {
    TooltipTrigger { "Save" }
    TooltipContent { "Saved 2 minutes ago" }
  }
}
```

Pass `open` and `on_open_change` to control it, or `default_open` to start it
open; `on_open_change` hears every change in both modes.

- Hovering the trigger opens it after `delay_ms` (default `700`). Keyboard
  focus on the trigger opens it at once.
- The tooltip stays open while the pointer moves from the trigger onto the
  content. Leaving both closes it after a 100 ms grace period.
- Blur closes it unless the pointer is over the trigger or content; then
  pointer leave closes it. A press on the trigger closes it, and hover
  does not reopen the tooltip until the pointer leaves the trigger.
- Touch pointers do not open the tooltip.
- While open, the trigger has `aria-describedby` pointing to the content.
- The content anchors to `TooltipTrigger`.
- Content is placed next to its anchor using fixed positioning, flips to the
  opposite side when the preferred side lacks room, shifts to stay inside the
  viewport, and follows resize and scroll. `side_offset` defaults to `4`
  pixels.
- `side` defaults to `Top` and `align` to `Center`.
- Escape closes it per `dismiss` (default
  `DismissBehavior::tooltip_default()`); outside pointer presses do not.
- `TooltipTrigger` takes `onclick` for its action and passes through other
  button attributes, such as `aria-label` for an icon-only trigger.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Tooltips open on hover and keyboard focus. Do not put essential interactive
content inside a tooltip. Skipping the delay between adjacent tooltips, touch
long press, and triggers other than buttons are not implemented (see
[RFC 0022](../rfcs/0022-tooltip-hover-and-focus-opening.md)). Tooltip and
Hover Card share one hover-open script
([RFC 0023](../rfcs/0023-hover-card-hover-and-focus-opening.md)).
