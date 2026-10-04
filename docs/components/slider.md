# Slider

Slider provides a controlled horizontal numeric value with range and thumb
styling.

## Source Copy

```bash
dxui add slider
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["slider"] }
```

## API Surface

- `Slider`
- `SliderState`
- `SliderKeyMove`
- `slider_state`
- `slider_key_move`
- `slider_percent`
- `slider_range_style`
- `slider_thumb_style`
- `slider_aria_attributes`

## Keyboard And Pointer Input

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

`Slider` is controlled. It calls `on_value_change` with the new snapped value
when it differs from `value`, and the app passes it back as `value`.

| Input | Change |
| --- | --- |
| ArrowRight, ArrowUp | One step up |
| ArrowLeft, ArrowDown | One step down |
| PageUp, PageDown | Ten steps up or down |
| Home, End | Minimum, maximum |
| Press or drag | The value under the pointer |

Handled keys do not scroll the page. A page script captures the pointer on
press, so a drag keeps working outside the slider. A disabled slider ignores
keys and the pointer. `slider_key_move` exposes the key mapping.

Other attributes, such as `aria-label`, `aria-labelledby`, and
`aria-valuetext`, are passed to the root. The slider always fills left to
right and is horizontal.

## Vertical Sliders

```rust
rsx! {
  div { class: "h-44",
    Slider {
      "aria-label": "Volume",
      orientation: SliderOrientation::Vertical,
      value: volume(),
      on_value_change: move |value| volume.set(value),
    }
  }
}
```

`orientation: SliderOrientation::Vertical` fills the slider from the bottom,
reads pointer positions along its height, and renders
`aria-orientation="vertical"`. Give its container a height. ArrowUp and
ArrowRight still increase the value. The thumb is centered on the value with
an inline absolute position in both orientations (see
[RFC 0042](../rfcs/0042-slider-thumb-position-and-vertical-orientation.md)).

## Accessibility Notes

Slider renders `role="slider"` with its orientation and ARIA value
attributes derived from the clamped, stepped value. Give it a name with
`aria-labelledby` or `aria-label`; a `Label` with `for` does not name a
`div`. Pass `aria-valuetext` when the number alone does not say what the
value means, such as "40 percent".
