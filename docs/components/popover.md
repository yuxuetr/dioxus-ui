# Popover

Popover provides primitive overlay configuration with styled content, header,
title, and description parts.

## Source Copy

```bash
dxui add popover
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["popover"] }
```

## API Surface

- `PopoverContent`
- `PopoverHeader`
- `PopoverTitle`
- `PopoverDescription`
- `PopoverPrimitiveConfig`
- `PopoverDismissBehavior`, `OverlaySide`, `OverlayAlign`
- `popover_content_class`

## Behavior

`open` stays controlled by the app. Give the trigger an `id` and pass it as
`anchor_id`:

```rust
let mut open = use_signal(|| false);

rsx! {
  button { id: "share-trigger", onclick: move |_| open.toggle(), "Share" }
  PopoverContent {
    open: open(),
    anchor_id: "share-trigger",
    on_open_change: move |next| open.set(next),
    PopoverTitle { "Share link" }
  }
}
```

- With `anchor_id`, content is placed on `side` (default `Bottom`) with `align`
  (default `Center`) and `side_offset` (default `4`) pixels, using fixed
  positioning. It flips to the opposite side when the preferred side lacks room
  and shifts along the cross axis to stay 8 pixels inside the viewport. It
  follows window resize and scroll while open.
- Without `anchor_id`, content renders in place as before.
- Escape, a pointer press outside the content and anchor, and focus moving
  outside request close per `dismiss` (default
  `DismissBehavior::popover_default()`). Presses on the anchor are inside, so a
  toggling trigger keeps working.
- Focus is not moved into the popover.

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

Use popovers for supplemental interactive content. Configure dismissal behavior
carefully so keyboard and pointer users can close the surface predictably.

`PopoverContent` takes its name from a `PopoverTitle` inside it and its description from a
`PopoverDescription`: the parts get generated ids, and the content points
`aria-labelledby` and `aria-describedby` at the ones that are mounted. A
passed `aria-label`, `aria-labelledby`, or `aria-describedby` on `PopoverContent`
replaces the generated value, so give a title-less one an `aria-label` (see
[RFC 0039](../rfcs/0039-dialog-names.md)).
