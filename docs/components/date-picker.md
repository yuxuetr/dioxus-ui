# Date Picker

Date Picker provides controlled trigger, value, and popover content parts for
composing a Calendar inside an overlay. It does not parse text input, format
dates, or own selected state in the first implementation.

## Source Copy

```bash
dxui add date-picker
```

This creates Date Picker and its Calendar dependency:

```text
src/components/ui/date_picker.rs
src/components/ui/calendar.rs
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["date-picker", "calendar"] }
```

```rust
use dioxus_ui::{DatePickerContent, DatePickerTrigger, DatePickerValue};
```

## API

- `DatePickerTrigger { open, invalid, disabled, class, children }`
- `DatePickerValue { placeholder, class, children }`
- `DatePickerContent { open, side, align, class, children }`
- `DatePickerPrimitiveConfig`

Class helpers:

- `date_picker_trigger_class(invalid, class)`
- `date_picker_value_class(class)`
- `date_picker_content_class(class)`

## Accessibility

The trigger uses button semantics with `aria-haspopup="dialog"` and controlled
expanded/invalid state. Content uses dialog semantics and popover placement data
attributes. Calendar keyboard movement, focus entry, and date parsing remain
app-owned until runtime focus adapters are implemented.

Use native date inputs when platform-native mobile behavior is the priority.
Use Calendar directly for always-visible date grids. Use Date Picker when a
visual calendar should open from a compact form control.
