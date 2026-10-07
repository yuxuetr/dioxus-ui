# Number Input

Number Input is a field for a number, such as a quantity, with buttons and
arrow keys that step it within optional bounds.

## Source Copy

```bash
dxui add number-input
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["number-input"] }
```

## API Surface

- `NumberInput`
- `number_input_class`

```rust
let mut quantity = use_signal(|| 1.0);

rsx! {
  NumberInput {
    "aria-label": "Quantity",
    min: 1.0,
    max: 99.0,
    value: quantity(),
    on_value_change: move |value| quantity.set(value),
  }
}
```

`value` and `step` (default 1) are `f64`; `min` and `max` are optional. The
input keeps the text being typed, so "-" or "1." survive a render, and calls
`on_value_change` with each complete number. On blur it clamps to the bounds
and rounds to the step's decimals, so 0.1 + 0.2 reads 0.3. The buttons
disable at a bound. `decrement_label` and `increment_label` name the buttons,
"Decrease" and "Increase" by default. Other attributes, such as `id` and
`name`, go to the input.

## Accessibility Notes

The input is a text field with `role="spinbutton"`, `inputmode="decimal"`,
and `aria-valuenow`, `aria-valuemin`, and `aria-valuemax`. ArrowUp and
ArrowDown step, and Home and End go to the bounds. The buttons are
`tabindex="-1"`, since the keys do the same; they still have names for touch
screen readers. A native `type="number"` input is not used: it reports an
empty value while its text is incomplete (see
[RFC 0060](../rfcs/0060-input-components.md)).
