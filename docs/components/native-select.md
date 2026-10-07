# Native Select

Native Select is a styled wrapper around the platform `select`, `optgroup`, and
`option` elements. Use it for simple forms, mobile-friendly pickers, and cases
where native form submission behavior matters more than custom popover
composition.

## Source Copy

```bash
dxui add native-select
```

This creates:

```text
src/components/ui/native_select.rs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["native-select"] }
```

```rust
use dioxus_shadcn::{NativeSelect, NativeSelectGroup, NativeSelectOption};
```

## API Surface

- `NativeSelect { invalid, disabled, class, on_value_change, children }`
- `NativeSelectGroup { label, class, children }`
- `NativeSelectOption { value, disabled, selected, class, children }`

Class helpers:

- `native_select_class(invalid, class)`
- `native_select_group_class(class)`
- `native_select_option_class(class)`

## Change Events

```rust
let mut size = use_signal(|| "md".to_string());

rsx! {
  Label { r#for: "size", "Size" }
  NativeSelect {
    id: "size",
    name: "size",
    on_value_change: move |value| size.set(value),
    NativeSelectOption { value: "sm", selected: size() == "sm", "Small" }
    NativeSelectOption { value: "md", selected: size() == "md", "Medium" }
  }
}
```

A change calls `on_value_change` with the chosen option's `value`. Mark that
option `selected` to keep the select controlled. Other attributes, such as
`id`, `name`, `required`, and `aria-describedby`, are passed to the select.

## Accessibility Notes

Native Select keeps browser/platform keyboard, focus, and form behavior. Pair it
with a visible `Label` where possible, use `invalid` for `aria-invalid`, and use
`disabled` on the select or individual options when choices are unavailable.

Prefer Native Select over custom Select or Combobox for small option sets,
simple settings, and mobile forms. Prefer Combobox when the user needs search or
filtering before choosing.
