# Popover

Popover provides primitive overlay configuration with styled content, header,
title, and description parts.

## Source Copy

```bash
dxui add popover
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["popover"] }
```

## API Surface

- `Popover`
- `PopoverTrigger`
- `PopoverContent`
- `PopoverHeader`
- `PopoverTitle`
- `PopoverDescription`
- `PopoverPrimitiveConfig`
- `PopoverDismissBehavior`, `OverlaySide`, `OverlayAlign`
- `popover_content_class`

## Behavior

`Popover` owns whether the popover is open and its parts read it, so they
must sit inside it (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). `PopoverTrigger` toggles
it and anchors the content; it renders an unstyled `button`, so style it with
`class`:

```rust
rsx! {
  Popover {
    PopoverTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
      "Share"
    }
    PopoverContent { PopoverTitle { "Share link" } }
  }
}
```

Pass `open` and `on_open_change` to control it, or `default_open` to start it
open; `on_open_change` hears every change in both modes. The trigger sets
`aria-expanded` and points `aria-controls` at the content.

- Content is placed next to the trigger on `side` (default `Bottom`) with `align`
  (default `Center`) and `side_offset` (default `4`) pixels, using fixed
  positioning. It flips to the opposite side when the preferred side lacks room
  and shifts along the cross axis to stay 8 pixels inside the viewport. It
  follows window resize and scroll while open.
- Escape, a pointer press outside the content and trigger, and focus moving
  outside close it per `dismiss` (default
  `DismissBehavior::popover_default()`). Presses on the trigger are inside, so it
  keeps toggling.
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
