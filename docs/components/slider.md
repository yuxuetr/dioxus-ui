# Slider

Slider provides a controlled numeric value with range and thumb styling, in a
horizontal or vertical orientation, and `RangeSlider` a controlled pair of
values with two thumbs.

## Source Copy

```bash
dxui add slider
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["slider"] }
```

## API Surface

- `Slider`
- `SliderOrientation`
- `SliderState`
- `SliderKeyMove`
- `slider_key_move`
- `RangeSlider`

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
`aria-valuetext`, are passed to the root. A horizontal slider fills left to
right, with no right-to-left mirroring; see Vertical Sliders for the
bottom-to-top orientation.

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

## Range Slider

`RangeSlider` holds a low and a high value (see
[RFC 0070](../rfcs/0070-range-slider.md)):

```rust
let mut price = use_signal(|| (20.0, 80.0));

rsx! {
  RangeSlider {
    "aria-label": "Price",
    value: price(),
    step: 5.0,
    min_steps_between: 2,
    on_value_change: move |next| price.set(next),
  }
}
```

- Each thumb is a `slider` with its own focus, named by `start_label` and
  `end_label` ("Minimum" and "Maximum" by default), and moved by the same
  keys as `Slider`. Its `aria-valuemin` or `aria-valuemax` is the other
  thumb's value, `min_steps_between` steps away.
- A press moves the nearer thumb and focuses it; dragging keeps moving it
  and stops at the other thumb.
- The root is a `group`; name it with `aria-label` or `aria-labelledby`.
- Right-to-left horizontal range sliders are not included.

## Accessibility Notes

Slider renders `role="slider"` with its orientation and ARIA value
attributes derived from the clamped, stepped value. Give it a name with
`aria-labelledby` or `aria-label`; a `Label` with `for` does not name a
`div`. Pass `aria-valuetext` when the number alone does not say what the
value means, such as "40 percent".
