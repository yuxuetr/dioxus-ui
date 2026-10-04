# Calendar

Calendar provides controlled styled parts for date grids. Crate mode reexports
pure date primitives for month grid generation, date movement, and range
classification. Source-copy mode keeps the styled parts self-contained and lets
the app own date-grid state.

## Source Copy

```bash
dxui add calendar
```

This creates:

```text
src/components/ui/calendar.rs
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["calendar"] }
```

```rust
use dioxus_ui::{Calendar, CalendarDay, CalendarDate, CalendarRangeState};
```

## API Surface

- `Calendar { class, children }`
- `CalendarHeader { class, children }`
- `CalendarCaption { class, children }`
- `CalendarNav { class, children }`
- `CalendarNavButton { direction, disabled, onclick, class, children }`
- `CalendarGrid { class, children }`
- `CalendarHead { class, children }`
- `CalendarHeadCell { class, children }`
- `CalendarBody { class, children }`
- `CalendarRow { class, children }`
- `CalendarDay { date, selected, today, outside_month, disabled, range_state, focused, on_key_move, on_select, class, children }`

Primitive helpers:

- `calendar_month_grid(...)`
- `calendar_move_date(...)`
- `calendar_key_move(key, shift)`
- `calendar_range_state(...)`
- `days_in_month(year, month)`
- `is_leap_year(year)`

## Keyboard Behavior

The visible month, focused date, and selected date stay controlled by the app.
Pass `on_key_move` to every day to make the grid keyboard-managed:

```rust
let mut month = use_signal(|| CalendarMonth::unchecked(2026, 10));
let mut focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
let mut selected = use_signal(|| None::<CalendarDate>);
let mut move_focus = move |date: CalendarDate| {
  focused.set(date);
  month.set(CalendarMonth::unchecked(date.year, date.month));
};

// Inside the week loop over `calendar_month_grid(month(), ...)`:
rsx! {
  CalendarDay {
    date: day.date,
    selected: day.selected,
    outside_month: day.outside_month,
    focused: day.date == focused(),
    on_key_move: move |key_move| {
      move_focus(calendar_move_date(focused(), key_move, CalendarWeekday::Sunday));
    },
    on_select: move |date| {
      selected.set(Some(date));
      move_focus(date);
    },
    "{day.date.day}"
  }
}
```

- Only the `focused` day has `tabindex="0"`, so the grid is one Tab stop.
- ArrowLeft and ArrowRight move by a day, ArrowUp and ArrowDown by a week,
  Page Up and Page Down by a month (a year with Shift), and Home and End to the
  start and end of the week. `calendar_key_move` exposes the mapping.
- After a navigation key, the day that becomes `focused` takes DOM focus,
  whether Dioxus reuses its element or mounts a new one (days keyed by date).
  Changing `focused` from app code without a key press does not move focus,
  so a calendar never steals focus on render.
- Click, Enter, and Space call `on_select` with the day's date.
- `CalendarNavButton` accepts `onclick` for month navigation.
- Source-copy templates include `CalendarKeyMove` and `calendar_key_move` but
  not date arithmetic; apps that build the grid themselves apply the move with
  their own date code.
- Moving onto a disabled date leaves focus on a disabled button, which cannot
  take focus. Skip disabled dates in the app when that matters.

## Accessibility Notes

Calendar exposes grid, row, columnheader, and gridcell roles. Selection,
disabled, outside-month, today, and range state are mapped to ARIA and data
attributes. Keyboard-managed days use roving tabindex and move DOM focus with
the focused date. Arrow keys follow visual left and right; right-to-left
mirroring is not implemented.
