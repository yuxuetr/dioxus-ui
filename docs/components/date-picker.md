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
dioxus-shadcn = { version = "0.1", default-features = false, features = ["date-picker", "calendar"] }
```

```rust
use dioxus_shadcn::{DatePickerContent, DatePickerTrigger, DatePickerValue};
```

## API Surface

- `DatePickerTrigger { id, open, invalid, disabled, on_open_change, class, children }`
- `DatePickerValue { placeholder, class, children }`
- `DatePickerContent { open, side, align, anchor_id, side_offset, on_open_change, dismiss, class, children }`
- `DatePickerInput { value, order, placeholder, invalid, disabled, on_value_change, class }`
- `DateOrder`, `parse_date(text, order)`, `format_date(date, order)`
- `DatePickerPrimitiveConfig`
- `DatePickerDismissBehavior`, `DatePickerSide`, `DatePickerAlign`

Class helpers:

- `date_picker_trigger_class(invalid, class)`
- `date_picker_value_class(class)`
- `date_picker_content_class(class)`
- `date_picker_input_class(invalid, class)`

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
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

### Typed dates

`DatePickerInput` is a text field for typing a date, controlled by
`value: Option<CalendarDate>`. Share the value with the calendar, and put the
trigger beside it as an icon button with an `aria-label`:

```rust
DatePickerInput {
  "aria-label": "Start date",
  order: DateOrder::MonthDayYear,
  value: selected(),
  on_value_change: move |date| selected.set(date),
}
```

`parse_date(text, order)` reads three numbers separated by `-`, `/`, `.`, or
spaces: a four-digit first number is ISO year-month-day in any order, and
otherwise `DateOrder` (`YearMonthDay`, `MonthDayYear`, or `DayMonthYear`)
decides. Years need four digits, and impossible dates such as February 30
are rejected. The input keeps the text being typed, calls `on_value_change`
with a date once the text parses or with `None` when cleared, marks text that
does not parse with `aria-invalid` without clearing it, and on blur rewrites
a valid date with `format_date`. The placeholder defaults to the order's
pattern (see [RFC 0064](../rfcs/0064-typed-date-input.md)).

## Accessibility Notes

The trigger uses button semantics with `aria-haspopup="dialog"` and controlled
expanded/invalid state. Content uses dialog semantics with focus entry, Tab
containment, and focus return, and with an `anchor_id` takes the trigger's
name through `aria-labelledby` (see [RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)).
Typed date parsing remains app-owned.

Use native date inputs when platform-native mobile behavior is the priority.
Use Calendar directly for always-visible date grids. Use Date Picker when a
visual calendar should open from a compact form control.
