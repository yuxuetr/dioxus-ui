# Tooltip

Tooltip provides a root, a trigger, primitive configuration, and a styled
content part for short supplemental descriptions.

## Source Copy

```bash
dxui add tooltip
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["tooltip"] }
```

## API Surface

- `Tooltip`
- `TooltipTrigger`
- `TooltipContent`
- `TooltipPrimitiveConfig`
- `TooltipDismissBehavior`, `TooltipSide`, `TooltipAlign`
- `tooltip_content_class`

## Behavior

`open` stays controlled by the app. Keep it, pass it to `TooltipContent`, and
handle `Tooltip` `on_open_change`:

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

- Hovering the trigger requests open after `delay_ms` (default `700`). Keyboard
  focus on the trigger requests open at once.
- The tooltip stays open while the pointer moves from the trigger onto the
  content. Leaving both requests close after a 100 ms grace period.
- Blur requests close unless the pointer is over the trigger or content; then
  pointer leave closes it. A press on the trigger requests close, and hover
  does not reopen the tooltip until the pointer leaves the trigger.
- Touch pointers do not open the tooltip.
- While open, the trigger has `aria-describedby` pointing to the content.
- Inside `Tooltip`, the content anchors to `TooltipTrigger` and uses the root's
  `on_open_change` for Escape. `anchor_id` and `on_open_change` on
  `TooltipContent` override them.
- Content is placed next to its anchor using fixed positioning, flips to the
  opposite side when the preferred side lacks room, shifts to stay inside the
  viewport, and follows resize and scroll. `side_offset` defaults to `4`
  pixels. Without an anchor, content renders in place.
- `side` defaults to `Top` and `align` to `Center`.
- Escape requests close per `dismiss` (default
  `DismissBehavior::tooltip_default()`); outside pointer presses do not.
- Without `Tooltip`, the app wires its own trigger and passes `anchor_id`.
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
