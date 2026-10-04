# Date Picker

Date Picker provides controlled trigger, value, and anchored dialog content
parts for composing a Calendar inside an overlay. It does not parse text input,
format dates, or own selected state.

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

## API Surface

- `DatePickerTrigger { id, open, invalid, disabled, on_open_change, class, children }`
- `DatePickerValue { placeholder, class, children }`
- `DatePickerContent { open, side, align, anchor_id, side_offset, on_open_change, dismiss, class, children }`
- `DatePickerPrimitiveConfig`
- `DatePickerDismissBehavior`, `DatePickerSide`, `DatePickerAlign`

Class helpers:

- `date_picker_trigger_class(invalid, class)`
- `date_picker_value_class(class)`
- `date_picker_content_class(class)`

## Behavior

`open`, the selected date, the focused date, and the visible month stay
controlled by the app. Give the trigger an `id`, pass it as `anchor_id`, and
make the Calendar keyboard-managed (see [Calendar](calendar.md#keyboard-behavior)):

```rust
let mut open = use_signal(|| false);

rsx! {
  DatePickerTrigger {
    id: "due-date-trigger",
    open: open(),
    on_open_change: move |next| open.set(next),
    DatePickerValue { "{label}" }
  }
  DatePickerContent {
    open: open(),
    anchor_id: "due-date-trigger",
    on_open_change: move |next| open.set(next),
    Calendar {
      // CalendarDay { focused, on_key_move, on_select: move |date| {
      //   selected.set(Some(date));
      //   open.set(false);
      // }, ... }
    }
  }
}
```

- Clicking the trigger requests `!open`.
- With `anchor_id`, content is placed on `side` (default `Bottom`) with `align`
  (default `Center`) and `side_offset` (default `4`), flipping and shifting like
  Popover.
- Opening moves focus to the Calendar's focused day (marked
  `data-dxui-autofocus`), or to the first focusable element. Tab and Shift+Tab
  wrap inside the content.
- Escape and outside interactions request close per `dismiss` (default
  `DismissBehavior::popover_default()`). Closing returns focus to the trigger
  unless an outside click already moved focus to another control.
- Choosing a date is app code: store it in `on_select` and request close.

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`; Mobile is not
covered by an automated check.

## Accessibility Notes

The trigger uses button semantics with `aria-haspopup="dialog"` and controlled
expanded/invalid state. Content uses dialog semantics with focus entry, Tab
containment, and focus return. Typed date parsing remains app-owned.

Use native date inputs when platform-native mobile behavior is the priority.
Use Calendar directly for always-visible date grids. Use Date Picker when a
visual calendar should open from a compact form control.
