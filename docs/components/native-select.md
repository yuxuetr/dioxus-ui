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
dioxus-ui = { version = "0.1", default-features = false, features = ["native-select"] }
```

```rust
use dioxus_ui::{NativeSelect, NativeSelectGroup, NativeSelectOption};
```

## API Surface

- `NativeSelect { invalid, disabled, class, children }`
- `NativeSelectGroup { label, class, children }`
- `NativeSelectOption { value, disabled, selected, class, children }`

Class helpers:

- `native_select_class(invalid, class)`
- `native_select_group_class(class)`
- `native_select_option_class(class)`

## Accessibility Notes

Native Select keeps browser/platform keyboard, focus, and form behavior. Pair it
with a visible `Label` where possible, use `invalid` for `aria-invalid`, and use
`disabled` on the select or individual options when choices are unavailable.

Prefer Native Select over custom Select or Combobox for small option sets,
simple settings, and mobile forms. Prefer Combobox when the user needs search or
filtering before choosing.
