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
- `CalendarNavButton { direction, disabled, class, children }`
- `CalendarGrid { class, children }`
- `CalendarHead { class, children }`
- `CalendarHeadCell { class, children }`
- `CalendarBody { class, children }`
- `CalendarRow { class, children }`
- `CalendarDay { date, selected, today, outside_month, disabled, range_state, class, children }`

Primitive helpers:

- `calendar_month_grid(...)`
- `calendar_move_date(...)`
- `calendar_range_state(...)`
- `days_in_month(year, month)`
- `is_leap_year(year)`

## Accessibility Notes

Calendar exposes grid, row, columnheader, and gridcell roles. Selection,
disabled, outside-month, today, and range state are mapped to ARIA and data
attributes. Keyboard event wiring and DOM focus movement remain app-owned in
this first implementation.
